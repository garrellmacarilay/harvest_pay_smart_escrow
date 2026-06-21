#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

// Define the states of our escrow contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowState {
    Uninitialized,
    AwaitingFunds,
    Locked,
    Released,
    Refunded,
}

// Keys for contract storage
#[contracttype]
pub enum DataKey {
    Buyer,
    Seller,
    Arbiter,
    Token,
    Amount,
    State,
}

#[contract]
pub struct HarvestPayEscrow;

#[contractimpl]
impl HarvestPayEscrow {
    /// Initializes the escrow contract with the involved parties and terms.
    pub fn init(
        env: Env,
        buyer: Address,
        seller: Address,
        arbiter: Address,
        token: Address,
        amount: i128,
    ) {
        // Ensure the contract hasn't been initialized already
        if env.storage().instance().has(&DataKey::State) {
            panic!("Contract already initialized");
        }

        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::Seller, &seller);
        env.storage().instance().set(&DataKey::Arbiter, &arbiter);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage()
            .instance()
            .set(&DataKey::State, &EscrowState::AwaitingFunds);
    }

    /// Buyer locks funds into the smart contract.
    pub fn lock_funds(env: Env) {
        let state: EscrowState = env.storage().instance().get(&DataKey::State).unwrap();
        if state != EscrowState::AwaitingFunds {
            panic!("Invalid state: cannot lock funds");
        }

        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        // Require the buyer's authorization to pull funds
        buyer.require_auth();

        // Transfer tokens from buyer to the contract address
        let client = token::Client::new(&env, &token);
        client.transfer(&buyer, &env.current_contract_address(), &amount);

        // Update state to Locked
        env.storage()
            .instance()
            .set(&DataKey::State, &EscrowState::Locked);
    }

    /// Arbiter releases funds to the farmer (seller) after successful delivery.
    pub fn release_funds(env: Env) {
        let state: EscrowState = env.storage().instance().get(&DataKey::State).unwrap();
        if state != EscrowState::Locked {
            panic!("Invalid state: funds are not locked");
        }

        let arbiter: Address = env.storage().instance().get(&DataKey::Arbiter).unwrap();
        let seller: Address = env.storage().instance().get(&DataKey::Seller).unwrap();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        // Only the arbiter (e.g., the local cooperative) can authorize the release
        arbiter.require_auth();

        // Transfer tokens from the contract to the seller
        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &seller, &amount);

        // Update state to Released
        env.storage()
            .instance()
            .set(&DataKey::State, &EscrowState::Released);
    }

    /// Arbiter refunds the buyer if the delivery fails or is rejected.
    pub fn refund_buyer(env: Env) {
        let state: EscrowState = env.storage().instance().get(&DataKey::State).unwrap();
        if state != EscrowState::Locked {
            panic!("Invalid state: funds are not locked");
        }

        let arbiter: Address = env.storage().instance().get(&DataKey::Arbiter).unwrap();
        let buyer: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        // Only the arbiter can authorize a refund
        arbiter.require_auth();

        // Transfer tokens from the contract back to the buyer
        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &buyer, &amount);

        // Update state to Refunded
        env.storage()
            .instance()
            .set(&DataKey::State, &EscrowState::Refunded);
    }

    /// Helper to read the current state of the escrow.
    pub fn get_state(env: Env) -> EscrowState {
        env.storage()
            .instance()
            .get(&DataKey::State)
            .unwrap_or(EscrowState::Uninitialized)
    }
}