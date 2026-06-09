#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, String,
};

/// Storage keys for the smart contract's state.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Stores the details of a contest. Keyed by the unique contest ID.
    Contest(u64),
}

/// Represents the structure of a fantasy sports contest on-chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contest {
    /// The address of the creator/manager of the contest.
    pub creator: Address,
    /// The token used for entry fees and reward distribution (e.g., USDC, XLM wrapper).
    pub token: Address,
    /// The entry fee required to join the contest.
    pub entry_fee: i128,
    /// The total prize pool collected so far.
    pub prize_pool: i128,
    /// Metadata or external link (e.g., IPFS hash) containing team details, rules, etc.
    pub details: String,
    /// The address of the winner (populated once the contest is finalized).
    pub winner: Option<Address>,
    /// Status indicating whether the contest is still active and accepting entries.
    pub is_active: bool,
}

#[contract]
pub struct FantasySportsContract;

#[contractimpl]
impl FantasySportsContract {
    /// Creates a new fantasy sports contest.
    ///
    /// # Arguments
    /// * `env` - The Soroban execution environment.
    /// * `contest_id` - A unique identifier for the new contest.
    /// * `creator` - The address of the user creating the contest.
    /// * `token` - The contract address of the token used for fees/rewards.
    /// * `entry_fee` - The entry fee amount (must be positive).
    /// * `details` - Additional information or IPFS CID with contest description.
    pub fn create_contest(
        env: Env,
        contest_id: u64,
        creator: Address,
        token: Address,
        entry_fee: i128,
        details: String,
    ) -> Contest {
        // Enforce that the creator has signed this transaction
        creator.require_auth();

        if entry_fee < 0 {
            panic!("entry fee must be non-negative");
        }

        let key = DataKey::Contest(contest_id);

        // Check if the contest ID already exists to prevent accidental overwrites
        if env.storage().persistent().has(&key) {
            panic!("contest with this ID already exists");
        }

        let contest = Contest {
            creator: creator.clone(),
            token,
            entry_fee,
            prize_pool: 0,
            details,
            winner: None,
            is_active: true,
        };

        // Save the contest to persistent storage
        env.storage().persistent().set(&key, &contest);

        // Emit an event for analytics indexers and frontends
        env.events().publish(
            (symbol_short!("created"), contest_id),
            (creator, entry_fee),
        );

        contest
    }

    /// Allows a user to join an active contest by paying the entry fee.
    /// The entry fee is pulled from the participant's balance and added to the prize pool.
    ///
    /// # Arguments
    /// * `env` - The Soroban execution environment.
    /// * `contest_id` - The unique identifier of the contest.
    /// * `participant` - The address of the user joining the contest.
    pub fn join_contest(env: Env, contest_id: u64, participant: Address) -> Contest {
        participant.require_auth();

        let key = DataKey::Contest(contest_id);
        if !env.storage().persistent().has(&key) {
            panic!("contest not found");
        }

        let mut contest: Contest = env.storage().persistent().get(&key).unwrap();

        if !contest.is_active {
            panic!("contest is not active");
        }

        // If there's an entry fee, transfer it from participant to this contract
        if contest.entry_fee > 0 {
            let token_client = token::Client::new(&env, &contest.token);
            
            // Transfer tokens from the participant to the contract's address
            token_client.transfer(
                &participant,
                &env.current_contract_address(),
                &contest.entry_fee,
            );

            // Increase the prize pool balance
            contest.prize_pool += contest.entry_fee;
        }

        // Update the contest state in storage
        env.storage().persistent().set(&key, &contest);

        // Emit a join event
        env.events().publish(
            (symbol_short!("joined"), contest_id),
            participant,
        );

        contest
    }

    /// Finalizes the contest, declares the winner, and distributes the entire prize pool.
    /// Only the creator of the contest is authorized to declare the winner.
    ///
    /// # Arguments
    /// * `env` - The Soroban execution environment.
    /// * `contest_id` - The unique identifier of the contest.
    /// * `winner` - The address of the contest winner.
    pub fn declare_winner(env: Env, contest_id: u64, winner: Address) -> Contest {
        let key = DataKey::Contest(contest_id);
        if !env.storage().persistent().has(&key) {
            panic!("contest not found");
        }

        let mut contest: Contest = env.storage().persistent().get(&key).unwrap();

        if !contest.is_active {
            panic!("contest is already finalized");
        }

        // Only the creator can declare the winner and distribute funds
        contest.creator.require_auth();

        // If there is any money in the prize pool, transfer it to the winner
        if contest.prize_pool > 0 {
            let token_client = token::Client::new(&env, &contest.token);
            
            // Transfer accumulated funds from the contract to the winner
            token_client.transfer(
                &env.current_contract_address(),
                &winner,
                &contest.prize_pool,
            );
        }

        // Update contest state
        contest.winner = Some(winner.clone());
        contest.is_active = false;

        env.storage().persistent().set(&key, &contest);

        // Emit a winner and reward distribution event
        env.events().publish(
            (symbol_short!("winner"), contest_id),
            (winner, contest.prize_pool),
        );

        contest
    }

    /// Helper function to retrieve contest details.
    ///
    /// # Arguments
    /// * `env` - The Soroban execution environment.
    /// * `contest_id` - The unique identifier of the contest.
    pub fn get_contest(env: Env, contest_id: u64) -> Option<Contest> {
        let key = DataKey::Contest(contest_id);
        env.storage().persistent().get(&key)
    }
}

mod test;

