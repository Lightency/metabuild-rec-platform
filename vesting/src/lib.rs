use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::ext_contract;
use near_sdk::{env, near_bindgen, Gas, AccountId};
use serde::{Serialize,Deserialize};

#[ext_contract(ext_ft)]
pub trait lighttoken {
    fn mint_token(&mut self, account_id: AccountId, amount: u128);
    fn storage_deposit (&mut self, account_id: String);
}

// VESTORS
// Vestors structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Vestors {
    pub id:String,
    pub owner_id: String,
    pub amount_of_token: u128,
    pub locked_amount: u128,
    pub unlocked_amount: u128,
    pub duration: u64,
    pub timestamp: u64,
    pub nb_time_payment: u8,
}

// Vestors implementation
impl Vestors {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            owner_id: String::new(),
            amount_of_token: 0,
            locked_amount: 0,
            unlocked_amount: 0,
            duration: 4,
            timestamp: 0,
            nb_time_payment: 0,
        }
    }
}


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VestingContract {
    records: Vector<Vestors>,
}

// Define the default, which automatically initializes the contract
impl Default for VestingContract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Make sure that the caller of the function is the owner
fn assert_self() {
    assert_eq!(
        env::current_account_id(),
        env::predecessor_account_id(),
        "Can only be called by owner"
    );
}

// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl VestingContract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            records: Vector::new(b"a"),
        }
    }

    // Owner's methods. Can only be called by the owner
    pub fn delete_all(&mut self){
        assert_self();
        for _i in 0..self.records.len(){
            self.records.pop();
        }
    }

    // Function that delete a specific vestor 
    pub fn delete_a_vestor (&mut self,name:String){
        if name=="".to_string() {
            for i in 0..self.records.len() {
                if self.records.get(i).unwrap().owner_id == env::signer_account_id().to_string() {
                    self.records.swap_remove(i);
                }
            }
        }else {
            for i in 0..self.records.len() {
                if self.records.get(i).unwrap().owner_id == name {
                    self.records.swap_remove(i);
                }
            }
        }
    }

    /****** GET FUNCTIONS ******/

    // Function to get the list of all the vestors
    pub fn get_all_vestors (&self) -> Vec<Vestors> {
        let mut vec = Vec::new();
        for i in 0..self.records.len() {
            vec.push(self.records.get(i).unwrap());
        }
        vec
    } 

    // Function to get id

    // Function to get a specific vestor
    pub fn get_vestor(&self, id:&String) -> Vestors {
        let mut vestor = Vestors::new();
        for i in 0..self.records.len() {
            if &self.records.get(i).unwrap().id == id {
                vestor = self.records.get(i).unwrap();
            }
        }
        vestor
    }

    // Function to get all locked amount of all accounts
    pub fn get_total_locked_amount(&self) -> u128{
        let mut total_locked_amount = 0;
        for i in 0..self.records.len() {
            total_locked_amount += self.records.get(i).unwrap().locked_amount;
        }
        total_locked_amount
    }

    // Function to get all unlocked amount of all accounts
    pub fn get_total_unlocked_amount(&self) -> u128{
        let mut total_unlocked_amount = 0;
        for i in 0..self.records.len() {
            total_unlocked_amount += self.records.get(i).unwrap().unlocked_amount;
        }
        total_unlocked_amount
    }


    /****** SET FUNCTIONS ******/

    // Function to add a lockup
    pub fn add_lockup(
        &mut self,
        id: String,
        amount_of_token: u128,
    ) {
        let vestor = Vestors {
            id:id,
            owner_id: env::signer_account_id().to_string(),
            amount_of_token: amount_of_token,
            locked_amount: 3 * (amount_of_token/4),
            unlocked_amount: amount_of_token/4,
            duration: 4,
            timestamp: env::block_timestamp_ms(),
            nb_time_payment: 1,
        };
        self.records.push(&vestor);
        self.add_storage_deposit();
        self.mint_lts(amount_of_token/4);
    }

    pub fn refresh (&mut self,v_id: String) {
        if self.get_vestor(&v_id).nb_time_payment == 1 && env::block_timestamp_ms() > self.get_vestor(&v_id).timestamp + (2 * 60000) {
            self.change_data(&self.get_vestor(&v_id).id);
            self.mint_lts(self.get_vestor(&v_id).amount_of_token/4);
            env::log_str("second payment done");
        }
        env::log_str("hihihihihi");
        if self.get_vestor(&v_id).nb_time_payment == 2 && env::block_timestamp_ms() > self.get_vestor(&v_id).timestamp + (4 * 60000) {
            self.change_data(&self.get_vestor(&v_id).id);
            self.mint_lts(self.get_vestor(&v_id).amount_of_token/4);
            env::log_str("third payment done");
        }
        env::log_str("hihihihi2222");
        if self.get_vestor(&v_id).nb_time_payment == 3 && env::block_timestamp_ms() > self.get_vestor(&v_id).timestamp + (6 * 60000) {
            self.change_data(&self.get_vestor(&v_id).id);
            self.mint_lts(self.get_vestor(&v_id).amount_of_token/4);
            env::log_str("fourth payment done");
        }
        if self.get_vestor(&v_id).nb_time_payment == 4 { 
            env::log_str("already paid");
        }
        
    }


    /****** BACKUP FUNCTIONS ******/

    // Function to mint LTS 
    pub fn mint_lts (&mut self, amount:u128) {
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();

        ext_ft::ext(contract_account)
            .with_static_gas(Gas(5_000_000_000_000))
            .mint_token(env::signer_account_id(), amount*100000000);
    }

    // Function to add the vestor in the storage of the LTS token
    pub fn add_storage_deposit (&mut self) {
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();

        ext_ft::ext(contract_account)
            .with_attached_deposit(1000000000000000000000000)
            .with_static_gas(Gas(5_000_000_000_000))
            .storage_deposit(env::signer_account_id().to_string());
    }



    // Function to replace a vestor by the new one
    pub fn replace_vestor (&mut self, vestor:Vestors) {
        for i in 0..self.records.len() {
            if self.records.get(i).unwrap().id == vestor.id {
                self.records.replace(i, &vestor);
            }
        }
    }

    // Function to change data 
    pub fn change_data (&mut self, v_id: &String){
        let current_vestor = self.get_vestor(&v_id);
        let vestor = Vestors {
            id: current_vestor.id,
            owner_id: env::signer_account_id().to_string(),
            amount_of_token: current_vestor.amount_of_token,
            locked_amount: current_vestor.locked_amount - current_vestor.amount_of_token/4,
            unlocked_amount: current_vestor.unlocked_amount + current_vestor.amount_of_token/4,
            duration: 4,
            timestamp: current_vestor.timestamp,
            nb_time_payment: current_vestor.nb_time_payment + 1,
        };
        self.replace_vestor(vestor);
    }

}