#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token, Address, Env, String,
};

#[test]
fn test_fantasy_sports_flow() {
    let env = Env::default();
    env.mock_all_auths();

    // Register our contract
    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    // Generate test user addresses
    let creator = Address::generate(&env);
    let participant_1 = Address::generate(&env);
    let participant_2 = Address::generate(&env);

    // Register a mock token to act as our entry fee and payout asset
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_contract_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_contract_id);
    let token_query = token::Client::new(&env, &token_contract_id);

    // Mint initial tokens to participants for entry fees
    token_client.mint(&participant_1, &100);
    token_client.mint(&participant_2, &100);

    // Check starting balances
    assert_eq!(token_query.balance(&participant_1), 100);
    assert_eq!(token_query.balance(&participant_2), 100);

    // --- Create a Contest ---
    let contest_id = 101;
    let entry_fee = 25;
    let details = String::from_str(&env, "Dream11 Premier League Match Week 1");

    let contest = client.create_contest(
        &contest_id,
        &creator,
        &token_contract_id,
        &entry_fee,
        &details,
    );

    assert_eq!(contest.creator, creator);
    assert_eq!(contest.token, token_contract_id);
    assert_eq!(contest.entry_fee, entry_fee);
    assert_eq!(contest.prize_pool, 0);
    assert_eq!(contest.is_active, true);
    assert_eq!(contest.winner, None);

    // Verify contest retrieval
    let retrieved = client.get_contest(&contest_id).unwrap();
    assert_eq!(retrieved.entry_fee, entry_fee);

    // --- Participants Join ---
    // Participant 1 joins
    let contest = client.join_contest(&contest_id, &participant_1);
    assert_eq!(contest.prize_pool, 25);
    assert_eq!(token_query.balance(&participant_1), 75); // 100 - 25
    assert_eq!(token_query.balance(&contract_id), 25);

    // Participant 2 joins
    let contest = client.join_contest(&contest_id, &participant_2);
    assert_eq!(contest.prize_pool, 50); // 25 + 25
    assert_eq!(token_query.balance(&participant_2), 75); // 100 - 25
    assert_eq!(token_query.balance(&contract_id), 50);

    // --- Declare Winner ---
    // Creator declares Participant 2 as the winner
    let contest = client.declare_winner(&contest_id, &participant_2);
    assert_eq!(contest.is_active, false);
    assert_eq!(contest.winner, Some(participant_2.clone()));

    // Verify rewards were distributed and contract balance is empty
    assert_eq!(token_query.balance(&participant_2), 125); // 75 (remaining) + 50 (prize pool)
    assert_eq!(token_query.balance(&participant_1), 75);  // unchanged
    assert_eq!(token_query.balance(&contract_id), 0);      // empty
}

#[test]
#[should_panic(expected = "contest with this ID already exists")]
fn test_duplicate_contest_id_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let token = Address::generate(&env);
    let details = String::from_str(&env, "Contest Details");

    client.create_contest(&1, &creator, &token, &10, &details);
    // This second call with the same ID must panic
    client.create_contest(&1, &creator, &token, &10, &details);
}

#[test]
#[should_panic(expected = "contest is already finalized")]
fn test_cannot_declare_winner_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let token = Address::generate(&env);
    let details = String::from_str(&env, "Contest Details");

    client.create_contest(&1, &creator, &token, &0, &details);
    client.declare_winner(&1, &Address::generate(&env));
    // Declaring winner again must panic
    client.declare_winner(&1, &Address::generate(&env));
}
