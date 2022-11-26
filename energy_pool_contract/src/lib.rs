use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Serialize , Deserialize};
use near_sdk::{env, near_bindgen, Promise, AccountId};


// VOTE
// Vote struct
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[derive(Serialize, Deserialize)]
 pub struct Vote{
    pub address: String,
    pub vote:u8,
    pub time_of_vote:u64,
 }

// Vote implementation
 impl Vote {
    pub fn new() -> Self{
        Self {
            address: String::new(),
            vote:0,
            time_of_vote:0,
        }
    }
 }


// PROPOSALS
// Proposals struct
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Proposals{
    pub title: String,
    pub description: String,
    pub proposal_creator: String,
    pub amount: u128,
    pub benificiary: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub time_of_creation:u64,
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64,
    pub list_voters:Vec<String>,
    pub votes:Vec<Vote>,
}

// Proposals implementation 
impl Proposals {
    pub fn new() -> Self{
        Self {
            title: String::new(),
            description: String::new(),
            proposal_creator: String::new(),
            amount: 0,
            benificiary: String::new(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:0,
            duration_days:0,
            duration_hours:0,
            duration_min:0,
            list_voters: Vec::new(),
            votes:Vec::new(),
        }
    }

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
            title: self.title.clone(), 
            description: self.description.clone(),
            proposal_creator: self.proposal_creator.clone(), 
            amount: self.amount, 
            benificiary: self.benificiary.clone(), 
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

    pub fn get_amount(&self) -> u128 {
        self.amount
    }

    pub fn get_benificiary(&self) -> String {
        self.benificiary.clone()
    }

    pub fn end_time(&self) -> u64 {
        self.time_of_creation+(self.duration_days*86400000000+self.duration_hours*3600000000+self.duration_min*60000000)
    }

    pub fn check_proposal(&self)->bool{
        if env::block_timestamp() > self.end_time() {
            return true;
        }
        return false;
    }
}


// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EnergiePoolContract {
    members: Vec<String>,
    records: Vec<Proposals>,
}

// Define the default, which automatically initializes the contract
impl Default for EnergiePoolContract {
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
impl EnergiePoolContract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            members: Vec::new(),
            records: Vec::new(),
        }
    }

    // delete all proposals
    pub fn delete_all(&mut self){
        assert_self();
        for _i in 0..self.records.len(){
            self.records.pop();
        }
    }

    // Methods.

    // Add a member
    pub fn add_member (&mut self, account:String) {
        let mut existance = false;
        for i in self.members.clone(){
            if i == account {
                existance = true;
                break;
            }
        }
        assert!(existance == false , "Account already exist");
        self.members.push(account);
    }

    // Remove a member
    pub fn remove_member (&mut self, account:String) {
        for i in 0..self.members.len(){
            if self.members[i] == account {
                self.members.swap_remove(i);
                break;
            }
        }
    }

    // create proposal
    pub fn create_proposal (
        &mut self,
        title: String,
        description: String,
        amount: u128,
        benificiary: String,
        duration_days:u64,
        duration_hours:u64,
        duration_min:u64
    ){  
        let mut existance = false;
        for i in self.members.clone(){
            if i == env::signer_account_id().to_string() {
                existance = true;
                break;
            }
        }
        assert!(existance == true , "You are not one of the councils");
        let proposal=Proposals{
            title: title,
            description: description,
            proposal_creator: env::signer_account_id().to_string(),
            amount: amount,
            benificiary: benificiary,
            votes_for: 0,
            votes_against: 0,
            time_of_creation:env::block_timestamp(),
            duration_days:duration_days,
            duration_hours:duration_hours,
            duration_min:duration_min,
            list_voters: Vec::new(),
            votes:Vec::new()
        };
        self.records.push(proposal);
    }

    // replace a proposal 
    pub fn replace_proposal(&mut self, proposal: Proposals){
        let mut index =0;
        for i in 0..self.records.len(){
            match self.records.get(i){
                Some(p) => if p.title==proposal.title {
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        self.records.swap_remove(index);
        self.records.insert(index, proposal);
    }

    // get all proposals 
    pub fn get_proposals(&self) -> Vec<Proposals>{
        self.records.clone()
    }

    //get a specific proposal 
    pub fn get_specific_proposal(&self, title: String) -> Proposals{
        let mut proposal= Proposals::new();
        for i in 0..self.records.len() {
            match self.records.get(i){
                Some(p) => if p.title==title {
                    proposal=p.clone();
                },
                None => panic!("There is no DAOs"),
            }
        }
        proposal
    }

    //get the end time of a specific proposal
    pub fn get_end_time(&self,title: String) -> u64 {
        let proposal=self.get_specific_proposal(title);
        proposal.end_time()
    }
    
    // add a vote 
    pub fn add_vote(
        &mut self,
        title: String,
        vote: u8
    ){
        let mut proposal = self.get_specific_proposal(title);
        proposal = proposal.create_vote(vote);
        self.replace_proposal(proposal);
    }

    // get votes for 
    pub fn get_votes_for(&self, title: String) -> u32 {
        let proposal = self.get_specific_proposal(title);   
        proposal.votes_for
    }

    // get votes against 
    pub fn get_votes_against(&self, title: String) -> u32 {
        let proposal = self.get_specific_proposal(title);   
        proposal.votes_against
    }

    // get number of votes 
    pub fn get_nember_votes(&self, title: String) -> u32{
        let proposal = self.get_specific_proposal(title);
        proposal.votes_against + proposal.votes_for
    }

    // funtion that pay near to an account
    pub fn pay(&self, amount: u128, to: AccountId) -> Promise {
        Promise::new(to).transfer(amount)
    }

    // check the proposal and send near to the benificiary if it's true
    pub fn check_and_send_near(&self,title: String) -> String{
        let proposal = self.get_specific_proposal(title);
        let check= proposal.check_proposal();
        if check==true {
            let benificiary= proposal.get_benificiary().try_into().unwrap();
            let amount= proposal.get_amount() * 1000000000000000000000000;
            let _payment=self.pay(amount,benificiary);
            let msg="Proposal accepted and amount was sent".to_string();
            msg
        }else{
            let msg="Proposal refused".to_string();
            msg
        }
    }
}

