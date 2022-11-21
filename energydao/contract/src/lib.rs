use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{env, near_bindgen, ext_contract,Gas};
use serde::{Serialize,Deserialize};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_lts)]
pub trait Lts {
    fn ft_transfer (&mut self, receiver_id:String, amount:String, memo:String);
}

// VOTE
// Vote structor 
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Vote{
    pub address: String,
    pub vote:u8,
    pub time_of_vote:u64,
}

// Vote implementation 
impl Vote {
    // Initialise a new vote
    pub fn new() -> Self{
        Self {
            address: String::new(),
            vote:0,
            time_of_vote:0,
        }
    }
}

// Proposal structor
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Proposal{
    pub proposal_type: u8,
    pub proposal_name: String,
    pub description: String,
    pub amount: u128,
    pub proposal_creator: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub time_of_creation:u64,
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64,
    pub list_voters:Vec<String>,
    pub votes:Vec<Vote>,
}

impl Proposal{
    pub fn new() -> Self{
        Self{
            proposal_type:0,
            proposal_name: String::new(),
            description: String::new(),
            amount:0,
            proposal_creator: String::new(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:0,
            duration_days:0,
            duration_hours:0,
            duration_min:0,
            list_voters:Vec::new(),
            votes:Vec::new(),
        }
    }

    // Create a new vote 
    // Returns a propsal contains the new vote 
    pub fn create_vote(&mut self, vote:u8) -> Self{
        for i in self.list_voters.clone(){
            assert!(
                env::signer_account_id().to_string() != i,
                "You already voted"
            );
        }
        let v = Vote{
            address: env::signer_account_id().to_string(),
            vote:vote,
            time_of_vote:env::block_timestamp(),
        };
        self.votes.push(v);
        if vote==0 {
            self.votes_against=self.votes_against+1;
        }else{
            self.votes_for=self.votes_for+1;
        }
        self.list_voters.push(env::signer_account_id().to_string());
        Self { 
            proposal_type:self.proposal_type,
            proposal_name: self.proposal_name.clone(), 
            description: self.description.clone(),
            amount: self.amount,
            proposal_creator: self.proposal_creator.clone(),
            votes_for: self.votes_for, 
            votes_against: self.votes_against, 
            time_of_creation: self.time_of_creation, 
            duration_days: self.duration_days, 
            duration_hours: self.duration_hours, 
            duration_min: self.duration_min, 
            list_voters: self.list_voters.clone(),
            votes: self.votes.clone() 
        }
    }

    // Get the end time of a proposal 
    pub fn end_time(&self) -> u64 {
        self.time_of_creation+(self.duration_days*86400000000000+self.duration_hours*3600000000000+self.duration_min*60000000000)
    }

    // Check if the time of a proposal is end or not 
    pub fn check_proposal(&self)->bool{
        if (env::block_timestamp() > self.end_time()) && (self.votes_for > self.votes_against){
            return true;
        }
        return false;
    } 

}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EnergyDao {
    members: UnorderedMap<String,u8>,
    proposals: Vec<Proposal>,
}

// Define the default, which automatically initializes the contract
impl Default for EnergyDao {
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
impl EnergyDao {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            members : UnorderedMap::new(b"m"),
            proposals : Vec::new(),
        }
    }

    pub fn init(&mut self) {
        assert_self();
        self.members.insert(&env::current_account_id().to_string(), &0);
    }

    // delete all members 
    pub fn delete_all (&mut self) {
        assert_self();
        self.members.clear();
    }

    // get all councils
    pub fn get_councils(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for i in self.members.keys() {
            if self.members.get(&i).unwrap() == 0 {
                vec.push(i);
            }
        }
        vec
    }

    // get all communities
    pub fn get_communities(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for i in self.members.keys() {
            if self.members.get(&i).unwrap() == 1 {
                vec.push(i);
            }
        }
        vec
    }

    pub fn check_member(&self, account:String) -> bool {
        let mut result=false;
        for i in 0..self.members.keys_as_vector().len() {
            if self.members.keys_as_vector().get(i).unwrap() == account {
                result = true;
                break;
            } 
        }
        result
    }

    pub fn check_council (&self, account:String) -> bool {
        if self.check_member(account.clone()) == false {
            return false ;
        }else {
            if self.members.get(&account).unwrap() == 0 {
                return true;
            }else {
                return false;
            }
        }
    }

    // Create a new proposal 
    pub fn create_proposal (
        &mut self,
        proposal_type:u8,
        proposal_name: String,
        description: String,
        amount:u128,
        duration_days: u64,
        duration_hours: u64,
        duration_min: u64,
    ){
        assert_eq!(
            self.check_council(env::signer_account_id().to_string()),
            true,
            "Proposals can be created only by the councils"
        );
        let proposal=Proposal{
            proposal_type:proposal_type,
            proposal_name: proposal_name,
            description: description,
            amount:amount,
            proposal_creator: env::signer_account_id().to_string(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:env::block_timestamp(),
            duration_days:duration_days,
            duration_hours:duration_hours,
            duration_min:duration_min,
            list_voters:Vec::new(),
            votes:Vec::new()
        };
        self.proposals.push(proposal);
    }

    // Replace a proposal whith a new one 
    pub fn replace_proposal(&mut self, proposal: Proposal){
        let mut index =0;
        for i in 0..self.proposals.len(){
            match self.proposals.get(i){
                Some(p) => if p.proposal_name==proposal.proposal_name {
                    index=i;
                },
                None => panic!("There is no PROPOSALs"),
            }
        }
        self.proposals.swap_remove(index);
        self.proposals.insert(index, proposal);
    }

    // Get all proposals 
    pub fn get_proposals(&self) -> Vec<Proposal>{
        self.proposals.clone()
    }

    // Get a spsific proposal 
    pub fn get_specific_proposal(&self, proposal_name: String) -> Proposal{
        let mut proposal= Proposal::new();
        for i in 0..self.proposals.len() {
            match self.proposals.get(i){
                Some(p) => if p.proposal_name==proposal_name {
                    proposal=p.clone();
                },
                None => panic!("There is no DAOs"),
            }
        }
        proposal
    }

    // add a vote 
    pub fn add_vote(
        &mut self,
        proposal_name: String,
        vote: u8
    ){
        if env::block_timestamp() < self.get_specific_proposal(proposal_name.clone()).end_time() {
            assert_eq!(
                self.check_member(env::signer_account_id().to_string()),
                true,
                "You must be one of the dao members to vote"
            );
            let proposal =self.get_specific_proposal(proposal_name.clone()).create_vote(vote);
            self.replace_proposal(proposal);
        }else {
            panic!("Proposal has been expired");
        }
        
    }

    // add a council
    pub fn add_council(&mut self, account:String){
        assert_eq!(
            self.check_council(env::signer_account_id().to_string()),
            true,
            "To add a council you must be one of the councils"
        );
        self.members.insert(&account, &0);
    }

    // add community
    pub fn add_community (&mut self,account:String) {
            self.members.insert(&account, &1); 
    }

    // check the proposal and return a message
    pub fn check_the_proposal(&self,proposal_name: String) -> String{
        let proposal=self.get_specific_proposal(proposal_name);
        let check= proposal.check_proposal();
        if check==true {
            let msg="Proposal accepted".to_string();
            msg
        }else{
            let msg="Proposal refused".to_string();
            msg
        }
    }

    // fund function 
    pub fn fund (&mut self,account:String,amount:u128){
        let account_lts= "light-token.testnet".to_string().try_into().unwrap();
        ext_lts::ext(account_lts)
        .with_static_gas(Gas(2 * TGAS))
        .with_attached_deposit(1)
        .ft_transfer(account,(amount*100000000).to_string(),"".to_string());
    }

}