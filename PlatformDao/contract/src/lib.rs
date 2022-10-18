use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::ext_contract;
use near_sdk::{env, near_bindgen, Gas};
use serde::{Serialize,Deserialize};

//Organization structure
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[derive(Serialize)]
pub struct Organisations{
    pub id:u32,
    pub name:String,
    pub number_members:u32,
    
}

impl Organisations {
    pub fn new()-> Self{
        Self {
            id:0, 
            name:String::new() , 
            number_members: 1,
             }
    }
    
}

//external contracts
#[ext_contract(ext_ft)]
pub trait OrganisationDAO {
    fn add_dao(&mut self,dao_name:String,dao_purpose:String,duration_days:u64,duration_hours:u64,duration_min:u64);
    fn delete(&mut self,dao_name:String);
    
}


//Vote structure
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[derive(Serialize,Deserialize)]
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


 // Proposal structor 
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[derive(Serialize,Deserialize)]
pub struct Proposals{
    pub proposal_name: String,
    pub description: String,
    pub proposal_creator: String,
    pub proposal_type:u16,
    pub dao_name:String,
    pub dao_purpose:String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub time_of_creation:u64,
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64,
    pub list_voters:Vec<String>,
    pub votes:Vec<Vote>,
}

// Proposal implementation
impl Proposals {
    pub fn new() -> Self{
        Self {
            proposal_name: String::new(),
            description: String::new(),
            proposal_creator: String::new(),
            dao_name:String::new(),
            dao_purpose:String::new(),
            proposal_type:0,
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
            proposal_name: self.proposal_name.clone(), 
            description: self.description.clone(),
            proposal_creator: self.proposal_creator.clone(),
            proposal_type:self.proposal_type.clone(),
            dao_name:self.dao_name.clone(),
            dao_purpose:self.dao_purpose.clone(),
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

    pub fn end_time(&self) -> u64 {
        self.time_of_creation+(self.duration_days*86400000000+self.duration_hours*3600000000+self.duration_min*60000000)
    }

    pub fn check_proposal(&self)->bool{
        if (env::block_timestamp() > self.end_time()) && (self.votes_for > self.votes_against){
            return true;
        }
        return false;
    }

}

// Define the PlatformDao PlatformDao structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlatformDao {
    pub dao_name: String,
    pub dao_purpose: String,
    pub founder: String,
    pub dao_members: Vec<String>,
    pub numb_members: u64,
    //Organisations
    pub numb_of_organisations:u32,
    pub organisations:Vec<Organisations>,
    //proposal
    pub number_of_proposals:u16,
    pub proposals : Vec<Proposals>,
    //Voting
    pub threshold:u8,
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64
}

// Define the default, which automatically initializes the PlatformDao
impl Default for PlatformDao{
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

// Implement the PlatformDao structure
#[near_bindgen]
impl PlatformDao {
    #[init]
    pub fn new() -> Self{
        Self {
            dao_name:String::new(),
            dao_purpose:String::new(),
            founder:String::new(),
            dao_members:Vec::new(),
            organisations:Vec::new(),
            numb_of_organisations:0,
            numb_members:0,
            number_of_proposals:0,
            proposals:Vec::new(),
            threshold:0,
            duration_days:0,
            duration_hours:0,
            duration_min:0,
        }
    }
    //Proposal type =0 (register organisation)
    //Proposal type =1 (delete organisation)
    pub fn create_proposal (
        &mut self,
        proposal_type:u16,
        proposal_name: String,
        description: String,
        dao_name:String,
        dao_purpose:String,
        duration_days:u64,
        duration_hours:u64,
        duration_min:u64
    ){
        let proposal=Proposals{
            proposal_type:proposal_type,
            proposal_name: proposal_name,
            description: description,
            dao_name:dao_name,
            dao_purpose:dao_purpose,
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

    pub fn replace_proposal(&mut self, proposal: Proposals){
        let mut index =0;
        for i in 0..self.proposals.len(){
            match self.proposals.get(i){
                Some(p) => if p.proposal_name==proposal.proposal_name {
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        self.proposals.swap_remove(index);
        self.proposals.insert(index, proposal);
    }

    pub fn get_proposals(&self) -> Vec<Proposals>{
        self.proposals.clone()
    }

    pub fn get_specific_proposal(&self, proposal_name: String) -> Proposals{
        let mut proposal= Proposals::new();
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

    pub fn add_vote (&mut self, proposal_name: String , vote:u8){
        let mut proposal = self.get_specific_proposal(proposal_name);
        proposal.create_vote(vote);
        self.replace_proposal(proposal);
    }

    pub fn get_organisations(&self) -> Vec<Organisations>{
        self.organisations.clone()
    }

    pub fn get_specific_organisation(&self, organisation_name: String) -> Organisations{
        let mut organisation= Organisations::new();
        for i in 0..self.organisations.len() {
            match self.organisations.get(i){
      
                Some(o) => if o.name==organisation_name{
                    organisation=o.clone();
                },
                None => panic!("There is no DAOs"),
            }
        }
        organisation
    } 

    pub fn process_proposal(&mut self, proposal_name:String){
        let proposal = &self.get_specific_proposal(proposal_name);
        let contract="organisationdao.testnet".to_string().try_into().unwrap();
        if proposal.check_proposal() == true{
            if proposal.proposal_type==0{
                let current_numb=self.numb_of_organisations;
                let new_org= Organisations{
                    id:current_numb,
                    name:proposal.dao_name.clone(), 
                    number_members: 1
                };
                self.organisations.push(new_org);
                self.numb_of_organisations=current_numb+1;
                ext_ft::ext(contract)
                .with_static_gas(Gas(5*1000000000000))
                .add_dao(proposal.dao_name.clone(),proposal.dao_purpose.clone(),proposal.duration_days,proposal.duration_hours,proposal.duration_min);
            }
            else {
                let org = self.get_specific_organisation(proposal.dao_name.clone());
                self.organisations.swap_remove(org.id.try_into().unwrap());
                ext_ft::ext(contract)
                .with_static_gas(Gas(5*1000000000000))
                .delete(proposal.dao_name.clone());
            }
        }else{
            panic!("this proposal is not validated")
        }  
}
    
}
