use near_sdk::{ext_contract};
use serde::{Serialize, Deserialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Gas};
use near_sdk::collections::{Vector, UnorderedMap};

pub const TGAS: u64 = 1_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize,Deserialize)]
pub struct Data {
    amount:u128,
    time:u64,
    reward:f64,
    next_reward_time:u64,
    unstaked_amount:u128,
    unstake_timestamp:u64
}
#[ext_contract(ext_lts)]
pub trait Lts {
    fn ft_transfer (&mut self, receiver_id:String, amount:String, memo:String);
    fn ft_balance_of (&mut self, account_id:String)->u128;
}

#[ext_contract(ext_treasury)]
pub trait Treasury {
    fn add_staker (&mut self, account:String);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rewardercontract {
    redeemers:Vector<String>,
    staker_data:UnorderedMap<String,Data>,
}

impl Default for Rewardercontract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

fn assert_self() {
    assert_eq!(
        env::current_account_id(),
        env::predecessor_account_id(),
        "Can only be called by owner"
    );
}

// Implement the Rewardercontract structure
#[near_bindgen]
impl Rewardercontract {

    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            redeemers: Vector::new(b"a"),
            staker_data: UnorderedMap::new(b"m"),
        }
    }

    // delete all stakers
    pub fn delete_all_stakers(&mut self) {
        assert_self();
        self.staker_data.clear();
    }

    pub fn redeem(&mut self,account:String){
        self.redeemers.push(&account);
    }

    pub fn add_staker(&mut self, account:String, amount:u128) {
        if self.staker_data.get(&account).is_none() {
            let data = Data {
                amount : amount,
                time: env::block_timestamp(),
                reward:0 as f64,
                next_reward_time:env::block_timestamp() + 120000000,
                unstaked_amount:0,
                unstake_timestamp:0
            };
            self.staker_data.insert(&account, &data);
            let account_treasury= "treasurydao.testnet".to_string().try_into().unwrap();
            ext_treasury::ext(account_treasury)
                .with_static_gas(Gas(2 * TGAS))
                .add_staker(account.clone());
        }else {
            let mut data = self.staker_data.get(&account).unwrap(); 
            data.amount+=amount;
            data.time = env::block_timestamp(); 
            self.staker_data.insert(&account, &data);
            self.staker_data.insert(&account, &data);
            let account_treasury= "treasurydao.testnet".to_string().try_into().unwrap();
            ext_treasury::ext(account_treasury)
                .with_static_gas(Gas(2 * TGAS))
                .add_staker(account.clone());
        }
    }

    pub fn check_staker(&self, account:String) -> bool {
        let mut existance = false;
        let stakers = self.staker_data.keys_as_vector();
        for i in stakers.to_vec() {
            if account == i {
                existance = true;
                break;
            }
        }
        existance
    }

    pub fn get_totalstaked(&self) -> f64 {
        let mut sum:f64= 0.0;
        for i in self.staker_data.values_as_vector().to_vec() {
                sum = sum + i.amount as f64+ i.reward ;
        }
        sum
    }

    pub fn get_data(&self, account:String) -> Data {
        self.staker_data.get(&account).unwrap()
    } 

    pub fn unstake(&mut self, account:String, amount:u128){
        if self.check_staker(account.clone()){
            if amount < self.get_data(account.clone()).amount {
                let mut data=self.get_data(account.clone());
                data.amount-=amount;
                data.unstaked_amount+=amount;
                data.unstake_timestamp=env::block_timestamp();
                self.staker_data.insert(&account.clone(), &data);
            }else if amount == self.get_data(account.clone()).amount {
                let mut data=self.get_data(account.clone());
                data.amount-=amount;
                data.unstaked_amount+=amount;
                data.unstake_timestamp=env::block_timestamp();
                self.staker_data.insert(&account.clone(), &data);
            }else{
                panic!("You don't have enough staked amount !!!");
            }
        }else {
            panic!("You are not one of the stakers");
        }
    }

    pub fn withdraw(&mut self, account:String, amount:u128){
        if self.check_staker(account.clone()){
            if env::block_timestamp() > self.get_data(account.clone()).unstake_timestamp + 180000000 {
                if amount > self.get_data(account.clone()).unstaked_amount {
                    panic!("You don't have enough unstaked amount !!!");
                }else {
                    let mut data=self.get_data(account.clone());
                    data.unstaked_amount-=amount;
                    self.staker_data.insert(&account.clone(), &data);
                }
            }else {
                panic!("You must wait 48 Hours after your last unstake");
            }
        }else {
            panic!("You are not one of the unstakers");
        }
    }

    pub fn withdraw_reward(&mut self,account:String){
        if self.check_staker(account.clone()){
            let mut data=self.get_data(account.clone());
            let account_lts= "light-token.testnet".to_string().try_into().unwrap();
            ext_lts::ext(account_lts)
                .with_static_gas(Gas(2 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer(account.clone(),((data.reward*100000000.0) as u128).to_string(),"".to_string());
            data.reward=0.0;
            self.staker_data.insert(&account.clone(), &data);
        }else {
            panic!("You are not one of the unstakers");
        }
    }

    pub fn get_total_amount_per_wallet(&self, account:String) -> f64{
        self.get_data(account.clone()).amount as f64+ self.get_data(account.clone()).reward
    }

    // pub fn get_balance(&self) -> u128 {
    //     let account_lts= "light-token.testnet".to_string().try_into().unwrap();
    //     ext_lts::ext(account_lts)
    //             .with_static_gas(Gas(2 * TGAS))
    //             .ft_balance_of(env::current_account_id().to_string());
    // }


    pub fn calculaterewards(&self,account:String)-> f64{
        //Reward to stakers= Total staked (t) X APY(t) 
        //APY(t)=Staking pool supply/total staked(t) X Yield parameter.
        let staked_per_wallet = self.get_total_amount_per_wallet(account);
        let reward_pool = 100 as f64;
        let total_reward = (reward_pool / 1095 as f64) as f64;
        let apy=(total_reward / self.get_totalstaked() as f64) as f64;
        let reward = (apy * staked_per_wallet as f64) as f64;
        return reward;
    } 

    pub fn update_reward(&mut self,account:String){
        let mut new_data= self.get_data(account.clone());
        if env::block_timestamp() > new_data.next_reward_time {
            let add_reward= self.calculaterewards(account.clone());
            new_data.reward = new_data.reward + add_reward;
            new_data.next_reward_time = new_data.next_reward_time + 120000000;
            self.staker_data.insert(&account, &new_data);
        }else {
            panic!("You have not earned reward yet");
        }
    }
}