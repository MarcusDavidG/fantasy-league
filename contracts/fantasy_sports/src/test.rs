#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, vec, Address, Env, String};

fn setup_token(env: &Env) -> (Address, token::StellarAssetClient<'_>, token::Client<'_>) {
    let admin = Address::generate(env);
    let contract = env.register_stellar_asset_contract_v2(admin);
    let id = contract.address();
    let asset_client = token::StellarAssetClient::new(env, &id);
    let client = token::Client::new(env, &id);
    (id, asset_client, client)
}

fn default_split(_env: &Env) -> PrizeSplit {
    PrizeSplit {
        first_pct: 100,
        second_pct: 0,
        third_pct: 0,
    }
}

fn three_way_split() -> PrizeSplit {
    PrizeSplit {
        first_pct: 60,
        second_pct: 30,
        third_pct: 10,
    }
}

#[test]
fn test_full_flow_winner_takes_all() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);

    let (token_id, asset_client, token_client) = setup_token(&env);
    asset_client.mint(&p1, &100);
    asset_client.mint(&p2, &100);

    client.create_contest(
        &1,
        &creator,
        &token_id,
        &25,
        &String::from_str(&env, "Match Week 1"),
        &0,
        &default_split(&env),
    );

    client.join_contest(&1, &p1);
    client.join_contest(&1, &p2);

    assert_eq!(token_client.balance(&contract_id), 50);

    let winners = vec![&env, p2.clone()];
    let result = client.declare_winners(&1, &winners);

    assert!(matches!(result.status, ContestStatus::Finalized));
    assert_eq!(token_client.balance(&p2), 125); // 100 - 25 (fee) + 50 (full pool)
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
fn test_three_way_prize_split() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    let p3 = Address::generate(&env);

    let (token_id, asset_client, token_client) = setup_token(&env);
    asset_client.mint(&p1, &100);
    asset_client.mint(&p2, &100);
    asset_client.mint(&p3, &100);

    client.create_contest(
        &2,
        &creator,
        &token_id,
        &100,
        &String::from_str(&env, "Three-Way Split Contest"),
        &0,
        &three_way_split(),
    );

    client.join_contest(&2, &p1);
    client.join_contest(&2, &p2);
    client.join_contest(&2, &p3);

    // pool = 300
    let winners = vec![&env, p1.clone(), p2.clone(), p3.clone()];
    client.declare_winners(&2, &winners);

    assert_eq!(token_client.balance(&p1), 180); // 0 + 60% of 300 = 180
    assert_eq!(token_client.balance(&p2), 90);  // 0 + 30% of 300 = 90
    assert_eq!(token_client.balance(&p3), 30);  // 0 + remainder (10% = 30)
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
fn test_cancel_refunds_all() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);

    let (token_id, asset_client, token_client) = setup_token(&env);
    asset_client.mint(&p1, &100);
    asset_client.mint(&p2, &100);

    client.create_contest(
        &3,
        &creator,
        &token_id,
        &50,
        &String::from_str(&env, "Cancellable Contest"),
        &0,
        &default_split(&env),
    );
    client.join_contest(&3, &p1);
    client.join_contest(&3, &p2);

    let result = client.cancel_contest(&3);
    assert!(matches!(result.status, ContestStatus::Cancelled));
    assert_eq!(token_client.balance(&p1), 100);
    assert_eq!(token_client.balance(&p2), 100);
    assert_eq!(token_client.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "contest is full")]
fn test_max_participants_enforced() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let (token_id, asset_client, _) = setup_token(&env);

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    asset_client.mint(&p1, &100);
    asset_client.mint(&p2, &100);

    client.create_contest(
        &4,
        &creator,
        &token_id,
        &10,
        &String::from_str(&env, "Max 1 Contest"),
        &1,
        &default_split(&env),
    );
    client.join_contest(&4, &p1);
    client.join_contest(&4, &p2); // should panic
}

#[test]
#[should_panic(expected = "already joined")]
fn test_duplicate_join_rejected() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let p1 = Address::generate(&env);
    let (token_id, asset_client, _) = setup_token(&env);
    asset_client.mint(&p1, &200);

    client.create_contest(
        &5,
        &creator,
        &token_id,
        &10,
        &String::from_str(&env, "No Doubles"),
        &0,
        &default_split(&env),
    );
    client.join_contest(&5, &p1);
    client.join_contest(&5, &p1); // should panic
}

#[test]
#[should_panic(expected = "contest already exists")]
fn test_duplicate_contest_id_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);
    let creator = Address::generate(&env);
    let token = Address::generate(&env);
    let details = String::from_str(&env, "x");
    let split = default_split(&env);
    client.create_contest(&6, &creator, &token, &0, &details, &0, &split);
    client.create_contest(&6, &creator, &token, &0, &details, &0, &split);
}

#[test]
#[should_panic(expected = "contest is not active")]
fn test_cannot_declare_winners_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);
    let creator = Address::generate(&env);
    let token = Address::generate(&env);
    let details = String::from_str(&env, "x");
    let split = default_split(&env);
    client.create_contest(&7, &creator, &token, &0, &details, &0, &split);
    let w = vec![&env, Address::generate(&env)];
    client.declare_winners(&7, &w.clone());
    client.declare_winners(&7, &w);
}

#[test]
#[should_panic(expected = "prize_split percentages must sum to 100")]
fn test_invalid_split_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, FantasySportsContract);
    let client = FantasySportsContractClient::new(&env, &contract_id);
    let bad_split = PrizeSplit {
        first_pct: 50,
        second_pct: 30,
        third_pct: 0,
    };
    client.create_contest(
        &8,
        &Address::generate(&env),
        &Address::generate(&env),
        &0,
        &String::from_str(&env, "x"),
        &0,
        &bad_split,
    );
}
