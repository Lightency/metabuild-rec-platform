use near_sdk::{ext_contract};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,Gas, near_bindgen};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_lts)]
pub trait Lts {
    fn ft_transfer (&mut self, receiver_id:String, amount:String, memo:String);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakingPoolContract {

}

impl Default for StakingPoolContract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the contract structure
// To be implemented in the front end 
#[near_bindgen]
impl StakingPoolContract {

    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
        }
    }
    
    pub fn transfer_lts (&mut self, amount:u128){
        let account_lts= "light-token.testnet".to_string().try_into().unwrap();
        // transfer lts to the singner 
        ext_lts::ext(account_lts)
        .with_static_gas(Gas(2 * TGAS))
        .with_attached_deposit(1)
        .ft_transfer("staking_contract.testnet".to_string(),(amount*100000000).to_string(),"".to_string());
    }

}