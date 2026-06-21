#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env};

fn setup_env_and_token(env: &Env) -> (Address, token::Client, token::AdminClient) {
    let admin = Address::generate(env);
    let token_address = env.register_stellar_asset_contract(admin.clone());
    let token_client = token::Client::new(env, &token_address);
    let token_admin = token::AdminClient::new(env, &token_address);
    (token_address, token_client, token_admin)
}

#[test]
fn test_1_happy_path_release() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let (token_id, token_client, token_admin) = setup_env_and_token(&env);

    let contract_id = env.register_contract(None, HarvestPayEscrow);
    let client = HarvestPayEscrowClient::new(&env, &contract_id);

    // Mint 1000 tokens to buyer
    token_admin.mint(&buyer, &1000);

    // MVP Transaction execution
    client.init(&buyer, &seller, &arbiter, &token_id, &500);
    client.lock_funds();
    assert_eq!(token_client.balance(&buyer), 500);
    assert_eq!(token_client.balance(&contract_id), 500);

    client.release_funds();
    
    // Verify successful end-to-end execution
    assert_eq!(token_client.balance(&contract_id), 0);
    assert_eq!(token_client.balance(&seller), 500);
}

#[test]
#[should_panic(expected = "Invalid state: funds are not locked")]
fn test_2_edge_case_unauthorized_release_before_lock() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let (token_id, _, _) = setup_env_and_token(&env);

    let contract_id = env.register_contract(None, HarvestPayEscrow);
    let client = HarvestPayEscrowClient::new(&env, &contract_id);

    client.init(&buyer, &seller, &arbiter, &token_id, &500);
    // Arbiter tries to release before buyer locks
    client.release_funds(); 
}

#[test]
fn test_3_state_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let (token_id, token_client, token_admin) = setup_env_and_token(&env);

    let contract_id = env.register_contract(None, HarvestPayEscrow);
    let client = HarvestPayEscrowClient::new(&env, &contract_id);

    token_admin.mint(&buyer, &500);
    client.init(&buyer, &seller, &arbiter, &token_id, &500);
    
    client.lock_funds();
    assert_eq!(client.get_state(), EscrowState::Locked);
    
    client.release_funds();
    // Assert contract storage reflects the correct state after MVP transaction
    assert_eq!(client.get_state(), EscrowState::Released);
}

#[test]
fn test_4_edge_case_refund_buyer() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let (token_id, token_client, token_admin) = setup_env_and_token(&env);

    let contract_id = env.register_contract(None, HarvestPayEscrow);
    let client = HarvestPayEscrowClient::new(&env, &contract_id);

    token_admin.mint(&buyer, &1000);
    client.init(&buyer, &seller, &arbiter, &token_id, &500);
    
    client.lock_funds();
    client.refund_buyer();

    assert_eq!(client.get_state(), EscrowState::Refunded);
    assert_eq!(token_client.balance(&buyer), 1000);
    assert_eq!(token_client.balance(&seller), 0);
}

#[test]
#[should_panic(expected = "Contract already initialized")]
fn test_5_edge_case_double_init() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let (token_id, _, _) = setup_env_and_token(&env);

    let contract_id = env.register_contract(None, HarvestPayEscrow);
    let client = HarvestPayEscrowClient::new(&env, &contract_id);

    client.init(&buyer, &seller, &arbiter, &token_id, &500);
    // Should panic on second initialization
    client.init(&buyer, &seller, &arbiter, &token_id, &500);
}