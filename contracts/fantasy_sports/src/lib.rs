#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, vec, Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Contest(u64),
    Participants(u64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContestStatus {
    Active,
    Cancelled,
    Finalized,
}

/// Prize split config: percentages must sum to 100
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrizeSplit {
    pub first_pct: u32,  // e.g. 60
    pub second_pct: u32, // e.g. 30
    pub third_pct: u32,  // e.g. 10 (0 = no third place)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contest {
    pub creator: Address,
    pub token: Address,
    pub entry_fee: i128,
    pub prize_pool: i128,
    pub details: String,
    pub max_participants: u32,
    pub prize_split: PrizeSplit,
    pub winners: Vec<Address>,
    pub status: ContestStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContestView {
    pub contest: Contest,
    pub participants: Vec<Address>,
}

#[contract]
pub struct FantasySportsContract;

#[contractimpl]
impl FantasySportsContract {
    /// Creates a new fantasy sports contest.
    /// prize_split percentages must sum to 100.
    /// max_participants = 0 means unlimited.
    pub fn create_contest(
        env: Env,
        contest_id: u64,
        creator: Address,
        token: Address,
        entry_fee: i128,
        details: String,
        max_participants: u32,
        prize_split: PrizeSplit,
    ) -> Contest {
        creator.require_auth();

        if entry_fee < 0 {
            panic!("entry_fee must be non-negative");
        }
        if prize_split.first_pct + prize_split.second_pct + prize_split.third_pct != 100 {
            panic!("prize_split percentages must sum to 100");
        }

        let key = DataKey::Contest(contest_id);
        if env.storage().persistent().has(&key) {
            panic!("contest already exists");
        }

        let contest = Contest {
            creator: creator.clone(),
            token,
            entry_fee,
            prize_pool: 0,
            details,
            max_participants,
            prize_split,
            winners: vec![&env],
            status: ContestStatus::Active,
        };

        env.storage().persistent().set(&key, &contest);
        env.storage()
            .persistent()
            .set(&DataKey::Participants(contest_id), &Vec::<Address>::new(&env));

        env.events()
            .publish((symbol_short!("created"), contest_id), (creator, entry_fee));

        contest
    }

    /// Join an active contest by paying the entry fee.
    pub fn join_contest(env: Env, contest_id: u64, participant: Address) -> Contest {
        participant.require_auth();

        let key = DataKey::Contest(contest_id);
        let mut contest: Contest = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("contest not found"));

        if !matches!(contest.status, ContestStatus::Active) {
            panic!("contest is not active");
        }

        let mut participants: Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::Participants(contest_id))
            .unwrap_or_else(|| Vec::new(&env));

        if contest.max_participants > 0
            && participants.len() as u32 >= contest.max_participants
        {
            panic!("contest is full");
        }

        // Prevent duplicate entries
        for p in participants.iter() {
            if p == participant {
                panic!("already joined");
            }
        }

        if contest.entry_fee > 0 {
            token::Client::new(&env, &contest.token).transfer(
                &participant,
                &env.current_contract_address(),
                &contest.entry_fee,
            );
            contest.prize_pool += contest.entry_fee;
        }

        participants.push_back(participant.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Participants(contest_id), &participants);
        env.storage().persistent().set(&key, &contest);

        env.events()
            .publish((symbol_short!("joined"), contest_id), participant);

        contest
    }

    /// Declare winners (1, 2, or 3 addresses) and distribute prize pool per split.
    /// winners vec must match the prize_split config (e.g. 3 entries if third_pct > 0).
    pub fn declare_winners(
        env: Env,
        contest_id: u64,
        winners: Vec<Address>,
    ) -> Contest {
        let key = DataKey::Contest(contest_id);
        let mut contest: Contest = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("contest not found"));

        if !matches!(contest.status, ContestStatus::Active) {
            panic!("contest is not active");
        }

        contest.creator.require_auth();

        let split = &contest.prize_split;
        let expected_winners: u32 = if split.third_pct > 0 {
            3
        } else if split.second_pct > 0 {
            2
        } else {
            1
        };

        if winners.len() as u32 != expected_winners {
            panic!("wrong number of winners for prize split");
        }

        if contest.prize_pool > 0 {
            let token_client = token::Client::new(&env, &contest.token);
            let pool = contest.prize_pool;

            // First place
            let first_amount = (pool * split.first_pct as i128) / 100;
            token_client.transfer(
                &env.current_contract_address(),
                &winners.get(0).unwrap(),
                &first_amount,
            );

            // Second place
            if split.second_pct > 0 {
                let second_amount = (pool * split.second_pct as i128) / 100;
                token_client.transfer(
                    &env.current_contract_address(),
                    &winners.get(1).unwrap(),
                    &second_amount,
                );
            }

            // Third place gets remainder to avoid dust from integer division
            if split.third_pct > 0 {
                let paid = (pool * split.first_pct as i128) / 100
                    + (pool * split.second_pct as i128) / 100;
                let third_amount = pool - paid;
                token_client.transfer(
                    &env.current_contract_address(),
                    &winners.get(2).unwrap(),
                    &third_amount,
                );
            }
        }

        contest.winners = winners.clone();
        contest.status = ContestStatus::Finalized;
        env.storage().persistent().set(&key, &contest);

        env.events()
            .publish((symbol_short!("winners"), contest_id), winners);

        contest
    }

    /// Cancel a contest and refund all participants. Only creator can cancel.
    pub fn cancel_contest(env: Env, contest_id: u64) -> Contest {
        let key = DataKey::Contest(contest_id);
        let mut contest: Contest = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("contest not found"));

        if !matches!(contest.status, ContestStatus::Active) {
            panic!("contest is not active");
        }

        contest.creator.require_auth();

        if contest.prize_pool > 0 {
            let token_client = token::Client::new(&env, &contest.token);
            let participants: Vec<Address> = env
                .storage()
                .persistent()
                .get(&DataKey::Participants(contest_id))
                .unwrap_or_else(|| Vec::new(&env));

            for participant in participants.iter() {
                token_client.transfer(
                    &env.current_contract_address(),
                    &participant,
                    &contest.entry_fee,
                );
            }
        }

        contest.status = ContestStatus::Cancelled;
        env.storage().persistent().set(&key, &contest);

        env.events()
            .publish((symbol_short!("cancel"), contest_id), contest_id);

        contest
    }

    /// Returns contest details plus participant list.
    pub fn get_contest(env: Env, contest_id: u64) -> Option<ContestView> {
        let key = DataKey::Contest(contest_id);
        env.storage().persistent().get(&key).map(|contest| {
            let participants: Vec<Address> = env
                .storage()
                .persistent()
                .get(&DataKey::Participants(contest_id))
                .unwrap_or_else(|| Vec::new(&env));
            ContestView {
                contest,
                participants,
            }
        })
    }

    /// Returns only the participant list for a contest.
    pub fn get_participants(env: Env, contest_id: u64) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::Participants(contest_id))
            .unwrap_or_else(|| Vec::new(&env))
    }
}

mod test;
