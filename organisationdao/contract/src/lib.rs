use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::ext_contract;
use near_sdk::{env, near_bindgen, Gas, AccountId};
use serde::{Serialize,Deserialize};

//external contracts
#[ext_contract(ext_ft)]
pub trait PlatformDao {
    fn create_proposal(&mut self,proposal_type:u16 ,proposal_name: String ,description: String ,dao_name:String ,dao_purpose:String ,duration_days:u64 ,duration_hours:u64 ,duration_min:u64);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    address : String,
    data : String,
    time_of_generation : String,
}

// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
// pub struct certificate {
//     address : String,
//     privacy : String,
// }

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

// Council Members Proposal
// Proposal structor
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct MemberProposal{
    pub proposal_name: String,
    pub description: String,
    pub proposal_creator: String,
    pub beneficiary:String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub time_of_creation:u64,
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64,
    pub list_voters:Vec<String>,
    pub votes:Vec<Vote>,

}

impl MemberProposal{
    pub fn new() -> Self{
        Self{
        proposal_name: String::new(),
        description: String::new(),
        proposal_creator: String::new(),
        beneficiary:String::new(),
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
            proposal_name: self.proposal_name.clone(), 
            description: self.description.clone(),
            proposal_creator: self.proposal_creator.clone(),
            beneficiary:self.beneficiary.clone(),
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
        self.time_of_creation+(self.duration_days*86400000000+self.duration_hours*3600000000+self.duration_min*60000000)
    }

    // Check if the time of a proposal is end or not 
    pub fn check_proposal(&self)->bool{
        if (env::block_timestamp() > self.end_time()) && (self.votes_for > self.votes_against){
            return true;
        }
        return false;
    } 
}

// PROPOSAL
// Proposal structor 
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
pub struct Proposals{
    pub proposal_type: u8,
    pub proposal_name: String,
    pub description: String,
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

// Proposal implementation
impl Proposals {
    // Initialise a new proposal
    pub fn new() -> Self{
        Self {
            proposal_type:0,
            proposal_name: String::new(),
            description: String::new(),
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
        self.time_of_creation+(self.duration_days*86400000000+self.duration_hours*3600000000+self.duration_min*60000000)
    }

    // Check if the time of a proposal is end or not 
    pub fn check_proposal(&self)->bool{
        if (env::block_timestamp() > self.end_time()) && (self.votes_for > self.votes_against){
            return true;
        }
        return false;
    }
}


// DAO
// Dao structor
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize)]
pub struct Dao {
    pub council_members : Vec<String>,
    pub community_members : Vec<String>,
    // pub devices_list : UnorderedMap<AccountId, Vec<Device>>,
    // certificates_list : UnorderedMap<AccountId, certificate>,
    pub dao_name: String,
    pub dao_purpose: String,
    pub founder: String,
    pub numb_council_members: u64,
    pub numb_community_members:u64,
    //proposal
    pub number_of_proposals:u16,
    pub proposals : Vec<Proposals>,
    pub member_proposals: Vec<MemberProposal>,
    //Voting
    pub duration_days:u64,
    pub duration_hours:u64,
    pub duration_min:u64
}

// Dao implementation
impl Dao {
    // Initialise a new dao
    pub fn new() -> Self{
        Self {
            council_members : Vec::new(),
            community_members: Vec::new(),
            //devices_list : UnorderedMap::new(b"m"),
            dao_name:String::new(),
            dao_purpose:String::new(),
            founder:"".to_string().try_into().unwrap(),
            numb_council_members:0,
            numb_community_members:0,
            number_of_proposals:0,
            proposals:Vec::new(),
            member_proposals:Vec::new(),
            duration_days:0,
            duration_hours:0,
            duration_min:0,
        }
    }
    pub fn create_member_proposal(&mut self,proposal_name: String,beneficiary:String,description: String){
        let proposal=MemberProposal{
            proposal_name: proposal_name,
            description: description,
            beneficiary: beneficiary,
            proposal_creator: env::signer_account_id().to_string(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:env::block_timestamp(),
            duration_days:self.duration_days,
            duration_hours:self.duration_hours,
            duration_min:self.duration_min,
            list_voters:Vec::new(),
            votes:Vec::new()

        };
        self.member_proposals.push(proposal);
    }

    // Create a new proposal in a dao 
    pub fn create_proposal (
        &mut self,
        proposal_type:u8,
        proposal_name: String,
        description: String,
    ){
        let proposal=Proposals{
            proposal_type:proposal_type,
            proposal_name: proposal_name,
            description: description,
            proposal_creator: env::signer_account_id().to_string(),
            votes_for: 0,
            votes_against: 0,
            time_of_creation:env::block_timestamp(),
            duration_days:self.duration_days,
            duration_hours:self.duration_hours,
            duration_min:self.duration_min,
            list_voters:Vec::new(),
            votes:Vec::new()
        };
        self.proposals.push(proposal);
    }

    // Replace a proposal whith a new one 
    pub fn replace_proposal(&mut self, proposal: Proposals){
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
    pub fn get_proposals(&self) -> Vec<Proposals>{
        self.proposals.clone()
    }

    // Get a spsific proposal 
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
    // Replace a member proposal whith a new one 
    pub fn replace_member_proposal(&mut self, proposal: MemberProposal){
        let mut index =0;
        for i in 0..self.member_proposals.len(){
            match self.member_proposals.get(i){
                Some(p) => if p.proposal_name==proposal.proposal_name {
                    index=i;
                },
                None => panic!("There is no member proposals"),
            }
        }
        self.member_proposals.swap_remove(index);
        self.member_proposals.insert(index, proposal);
    }

    // Get all proposals 
    pub fn get_member_proposals(&self) -> Vec<MemberProposal>{
        self.member_proposals.clone()
    }

    // Get a spsific proposal 
    pub fn get_specific_member_proposal(&self, proposal_name: String) -> MemberProposal{
        let mut proposal= MemberProposal::new();
        for i in 0..self.member_proposals.len() {
            match self.member_proposals.get(i){
                Some(p) => if p.proposal_name==proposal_name {
                    proposal=p.clone();
                },
                None => panic!("There is no DAOs"),
            }
        }
        proposal
    }
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RegistrationDao {
    daos: Vector<Dao>,
}

// Define the default, which automatically initializes the contract
impl Default for RegistrationDao {
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

// Make sure that the caller of the function is the platform account
fn assert_platform() {
    assert_eq!(
        env::signer_account_id(),
        "platformdao.testnet".to_string().try_into().unwrap(),
        "Can only be called by platform account"
    );
}

// Make sure that the caller of the function is the issuer account
fn assert_issuer() {
    assert_eq!(
        env::signer_account_id(),
        "issuer.testnet".to_string().try_into().unwrap(),
        "Can only be called by issuer account"
    );
}


// Implement the contract structure
// To be implemented in the front end
#[near_bindgen]
impl RegistrationDao {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            daos : Vector::new(b"a"),
        }
    }

    // delete all daos
    pub fn delete_all (&mut self){
        assert_self();
        for _i in 0..self.daos.len(){
            self.daos.pop();
        }
    }

    // delete a specific dao
    pub fn delete (&mut self, dao_name: String){
        assert_platform();
        for i in 0..self.daos.len(){
            match self.daos.get(i) {
                Some(d) => if d.dao_name == dao_name {
                    self.daos.swap_remove(i);
                },
                None => panic!("There is no DAOs"),
            }
        }
    }

    // METHODS

    /*** DAOS ***/

    // Create dao
    pub fn add_dao(
        &mut self,
        dao_name: String,
        dao_purpose: String,
        duration_days:u64,
        duration_hours:u64,
        duration_min:u64,
    ) {
        assert_platform();
        let mut dao = Dao {
            council_members:Vec::new(),
            community_members:Vec::new(),
            //devices_list : UnorderedMap::new(b"m"),
            dao_name: dao_name,
            dao_purpose: dao_purpose,
            founder:env::signer_account_id().to_string(),
            numb_council_members: 1,
            numb_community_members:0,
            number_of_proposals:0,
            proposals : Vec::new(),
            member_proposals: Vec::new(),
            duration_days:duration_days,
            duration_hours:duration_hours,
            duration_min:duration_min,
        };
        dao.council_members.push(dao.founder.clone());
        self.daos.push(&dao);
    }

    // get all daos
    pub fn get_all_daos(&self) -> Vec<Dao>{
        let mut vec= Vec::new();
        for i in 0..self.daos.len() {
            match self.daos.get(i) {
                Some(d) => vec.push(d),
                None => panic!("There is no DAOs"),
            }
        }
        vec
    }

    // get a specific dao
    pub fn get_dao(&self, dao_name: String) -> Dao {
        let mut dao= Dao::new();
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(d) => if d.dao_name==dao_name {
                    dao=d;
                },
                None => panic!("There is no DAOs"),
            }
        }
        dao
    }

    pub fn check_existance_dao (&self, dao_name: String) -> bool {
        let mut res = false;
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(d) => if d.dao_name==dao_name {
                    res = true;
                    break;
                },
                None => panic!("There is no DAOs"),
            }
        }
        res
    }

    /*** PROPOSALS ***/
    //create member proposal

    pub fn create_member_proposal(&mut self,dao_name: String,beneficiary:String,proposal_name: String,description: String){
        let mut dao= Dao::new();
        let mut index=0;
        for i in 0..self.daos.len(){
            match self.daos.get(i){
                Some(d) => if d.dao_name==dao_name {
                    dao=d;
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        dao.create_member_proposal(proposal_name, beneficiary, description);
        self.daos.replace(index, &dao);


    }

    // create proposal
    #[payable]
    pub fn create_proposal(
        &mut self,
        dao_name: String,
        proposal_type:u8,
        proposal_name: String,
        description: String,
    ){
        let mut dao= Dao::new();
        let mut index=0;
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(d) => if d.dao_name==dao_name {
                    dao=d;
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        dao.create_proposal(proposal_type, proposal_name, description);
        self.daos.replace(index, &dao);
    }

    // request of a dao creation
    pub fn request_dao (
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
        // account as account id
        let account = "platformdao.testnet".to_string().try_into().unwrap();

        ext_ft::ext(account)
            .with_static_gas(Gas(5*1000000000000))
            .create_proposal(proposal_type,
                proposal_name,
                description,
                dao_name,
                dao_purpose,
                duration_days,
                duration_hours,
                duration_min);
    }


    // get all proposals in a specific dao 
    pub fn get_all_proposals(&self,dao_name: String) -> Vec<Proposals> {
        let dao=self.get_dao(dao_name);
        dao.get_proposals()
    }

    //get a specific proposal in a specific dao
    pub fn get_proposal(&self, dao_name: String,proposal_name: String) -> Proposals{
        let dao=self.get_dao(dao_name);
        dao.get_specific_proposal(proposal_name)
    }

    //get the end time of a specific proposal
    pub fn get_end_time(&self, dao_name: String,proposal_name: String) -> u64 {
        let proposal=self.get_proposal(dao_name, proposal_name);
        proposal.end_time()
    }

    //get all member proposals in a specific dao
    pub fn get_all_member_proposals(&self,dao_name: String)-> Vec<MemberProposal>{
        let dao=self.get_dao(dao_name);
        dao.get_member_proposals()
    }
     //get a specific member proposal in a specific dao
     pub fn get_member_proposal(&self, dao_name: String,proposal_name: String) -> MemberProposal{
        let dao=self.get_dao(dao_name);
        dao.get_specific_member_proposal(proposal_name)
    }

    //get the end time of a specific proposal
    pub fn get_end_time_member(&self, dao_name: String,proposal_name: String) -> u64 {
        let proposal=self.get_member_proposal(dao_name, proposal_name);
        proposal.end_time()
    }

    /*** Proposal VOTES ***/

    // add a vote 
    pub fn add_vote(
        &mut self,
        dao_name: String,
        proposal_name: String,
        vote: u8
    ){
        let proposal =self.get_dao(dao_name.clone()).get_specific_proposal(proposal_name).create_vote(vote);
        let mut dao= Dao::new();
        let mut index=0;
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(mut d) => if d.dao_name==dao_name {
                    d.replace_proposal(proposal.clone());
                    dao=d;
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        self.daos.replace(index, &dao);
    }

    // get votes for 
    pub fn get_votes_for(&self, dao_name: String,proposal_name: String) -> u32 {
        let proposal= self.get_dao(dao_name.clone()).get_specific_proposal(proposal_name);
        proposal.votes_for
    }

    // get votes against 
    pub fn get_votes_against(&self, dao_name: String,proposal_name: String) -> u32 {
        let proposal= self.get_dao(dao_name.clone()).get_specific_proposal(proposal_name);
        proposal.votes_against
    }

    // get number of votes 
    pub fn get_number_votes(&self, dao_name: String,proposal_name: String) -> u32 { 
        let proposal= self.get_dao(dao_name.clone()).get_specific_proposal(proposal_name);
        proposal.votes_against + proposal.votes_for
    }

    // check the proposal and return a message
    pub fn check_the_proposal(&self, dao_name: String,proposal_name: String) -> String{
        let proposal=self.get_proposal(dao_name, proposal_name);
        let check= proposal.check_proposal();
        if check==true {
            let msg="Proposal accepted".to_string();
            msg
        }else{
            let msg="Proposal refused".to_string();
            msg
        }
    }

    /*** Member Proposal VOTES ***/

    // add a vote 
    pub fn add_member_vote(
        &mut self,
        dao_name: String,
        proposal_name: String,
        vote: u8
    ){
        let proposal =self.get_dao(dao_name.clone()).get_specific_member_proposal(proposal_name).create_vote(vote);
        let mut dao= Dao::new();
        let mut index=0;
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(mut d) => if d.dao_name==dao_name {
                    d.replace_member_proposal(proposal.clone());
                    dao=d;
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        self.daos.replace(index, &dao);
    }

    // get votes for 
    pub fn get_member_votes_for(&self, dao_name: String,proposal_name: String) -> u32 {
        let proposal= self.get_dao(dao_name.clone()).get_specific_member_proposal(proposal_name);
        proposal.votes_for
    }

    // get votes against 
    pub fn get_member_votes_against(&self, dao_name: String,proposal_name: String) -> u32 {
        let proposal= self.get_dao(dao_name.clone()).get_specific_member_proposal(proposal_name);
        proposal.votes_against
    }

    // get number of votes 
    pub fn get_number_member_votes(&self, dao_name: String,proposal_name: String) -> u32 { 
        let proposal= self.get_dao(dao_name.clone()).get_specific_member_proposal(proposal_name);
        proposal.votes_against + proposal.votes_for
    }

    // check the proposal and return a message
    pub fn check_the_member_proposal(&self, dao_name: String,proposal_name: String) -> String{
        let proposal=self.get_member_proposal(dao_name, proposal_name);
        let check= proposal.check_proposal();
        if check==true {
            let msg="Proposal accepted".to_string();
            msg
        }else{
            let msg="Proposal refused".to_string();
            msg
        }
    }


    // Add a council member to a dao 
    pub fn process_member_proposal (&mut self, dao_name: String, account:String) {
        let mut dao= Dao::new();
        let mut index=0;
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(d) => if d.dao_name==dao_name {
                    dao=d;
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        dao.council_members.push(account);
        dao.numb_council_members = dao.numb_council_members + 1;
        self.daos.replace(index, &dao);
    }

    pub fn add_community_member (&mut self, dao_name: String, account:String) {
        let mut dao= Dao::new();
        let mut index=0;
        for i in 0..self.daos.len() {
            match self.daos.get(i){
                Some(d) => if d.dao_name==dao_name {
                    dao=d;
                    index=i;
                },
                None => panic!("There is no DAOs"),
            }
        }
        dao.community_members.push(account);
        dao.numb_community_members = dao.numb_community_members + 1;
        self.daos.replace(index, &dao);
    }


    // Add a new device
    // pub fn add_device (&mut self, dao_name: String ,device_address: String ,device_data: String, device_time_of_generation: String ) {
    //     let device = Device {
    //         address:device_address,
    //         data:device_data,
    //         time_of_generation:device_time_of_generation
    //     };
    //     let mut dao= Dao::new();
    //     let mut index=0;
    //     for i in 0..self.daos.len() {
    //         match self.daos.get(i){
    //             Some(d) => if d.dao_name==dao_name {
    //                 dao=d;
    //                 index=i;
    //             },
    //             None => panic!("There is no DAOs"),
    //         }
    //     }
    //     if self.check_existance_dao(dao_name) == true {

    //     }
    // }

}
