// mod user_management;
// mod ballot_creation;
// mod voting_process;
// mod result_tabulation;  

// pub use user_management::*;
// pub use ballot_creation::*;
// pub use voting_process::*;
// pub use result_tabulation::*;



use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Vote {
    id: u64,
    voter_id: u64,
    candidate: String,
    election_id: u64,
    timestamp: u64,
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for Vote {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for Vote {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static VOTES_STORAGE: RefCell<StableBTreeMap<u64, Vote, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static ELECTIONS: RefCell<StableBTreeMap<u64, Election, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct Election {
    id: u64,
    title: String,
    candidates: Vec<String>,
    start_time: u64,
    end_time: u64,
}

impl Storable for Election {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Election {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct VotePayload {
    voter_id: u64,
    candidate: String,
    election_id: u64,
}

#[ic_cdk::query]
fn get_vote(id: u64) -> Result<Vote, Error> {
    match _get_vote(&id) {
        Some(vote) => Ok(vote),
        None => Err(Error::NotFound {
            msg: format!("a vote with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn cast_vote(vote_payload: VotePayload) -> Option<Vote> {
    // Validate that the election is ongoing
    if is_election_ongoing(vote_payload.election_id) {
        let id = ID_COUNTER
            .with(|counter| {
                let current_value = *counter.borrow().get();
                counter.borrow_mut().set(current_value + 1)
            })
            .expect("cannot increment id counter");

        let vote = Vote {
            id,
            voter_id: vote_payload.voter_id,
            candidate: vote_payload.candidate,
            election_id: vote_payload.election_id,
            timestamp: time(),
        };
        do_insert_vote(&vote);
        Some(vote)
    } else {
        // Election not ongoing
        None
    }
}

#[ic_cdk::update]
fn create_election(title: String, candidates: Vec<String>, start_time: u64, end_time: u64) -> Option<Election> {
    let election_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let election = Election {
        id: election_id,
        title,
        candidates,
        start_time,
        end_time,
    };

    if end_time > start_time {
        do_insert_election(&election);
        Some(election)
    } else {
        // Invalid election duration
        None
    }
}

// helper method to perform insert for elections.
fn do_insert_election(election: &Election) {
    ELECTIONS.with(|service| service.borrow_mut().insert(election.id, election.clone()));
}

// helper method to perform insert for votes.
fn do_insert_vote(vote: &Vote) {
    VOTES_STORAGE.with(|service| service.borrow_mut().insert(vote.id, vote.clone()));
}

// a helper method to get a vote by id. used in get_vote/delete_vote
fn _get_vote(id: &u64) -> Option<Vote> {
    VOTES_STORAGE.with(|service| service.borrow().get(id))
}

// Check if an election is ongoing
fn is_election_ongoing(election_id: u64) -> bool {
    match ELECTIONS.with(|service| service.borrow().get(&election_id)) {
        Some(election) => {
            let current_time = time();
            current_time >= election.start_time && current_time <= election.end_time
        }
        None => false,
    }
}

#[ic_cdk::query]
fn get_election(election_id: u64) -> Result<Election, Error> {
    match _get_election(&election_id) {
        Some(election) => Ok(election),
        None => Err(Error::NotFound {
            msg: format!("an election with id={} not found", election_id),
        }),
    }
}

#[ic_cdk::query]
fn get_election_results(election_id: u64) -> Result<Vec<(String, u64)>, Error> {
    match _get_election(&election_id) {
        Some(election) => {
            if is_election_ended(&election) {
                let votes = VOTES_STORAGE.with(|service| {
                    service
                        .borrow()
                        .values()
                        .filter(|vote| vote.election_id == election_id)
                        .collect::<Vec<_>>()
                });

                let mut result_map = std::collections::HashMap::new();

                for vote in votes {
                    let candidate_count = result_map.entry(vote.candidate.clone()).or_insert(0);
                    *candidate_count += 1;
                }

                let result_vec: Vec<(String, u64)> = result_map.into_iter().collect();
                Ok(result_vec)
            } else {
                // Election is still ongoing
                Err(Error::ElectionOngoing {
                    msg: "cannot retrieve results until the election ends".to_string(),
                })
            }
        }
        None => Err(Error::NotFound {
            msg: format!("an election with id={} not found", election_id),
        }),
    }
}

// a helper method to get an election by id. used in get_election/get_election_results
fn _get_election(id: &u64) -> Option<Election> {
    ELECTIONS.with(|service| service.borrow().get(id))
}

// Check if an election has ended
fn is_election_ended(election: &Election) -> bool {
    let current_time = time();
    current_time > election.end_time
}

#[ic_cdk::query]
fn get_ongoing_elections() -> Vec<Election> {
    let current_time = time();
    ELECTIONS
        .with(|service| {
            service
                .borrow()
                .values()
                .filter(|election| current_time >= election.start_time && current_time <= election.end_time)
                .cloned()
                .collect::<Vec<_>>()
        })
}


fn get_available_elections() -> Vec<Election> {
    let current_time = time();
    ELECTIONS
        .with(|service| {
            service
                .borrow()
                .values()
                .filter(|election| current_time < election.start_time)
                .cloned()
                .collect::<Vec<_>>()
        })
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct Voter {
    id: u64,
    username: String,
    registered_elections: Vec<u64>,
}

impl Storable for Voter {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Voter {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static VOTERS: RefCell<StableBTreeMap<u64, Voter, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

#[ic_cdk::update]
fn register_voter(username: String) -> Option<Voter> {
    let voter_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let voter = Voter {
        id: voter_id,
        username,
        registered_elections: Vec::new(),
    };

    do_insert_voter(&voter);
    Some(voter)
}

#[ic_cdk::update]
fn register_voter_for_election(voter_id: u64, election_id: u64) -> Result<(), Error> {
    let voter = _get_voter(&voter_id).ok_or(Error::NotFound {
        msg: format!("a voter with id={} not found", voter_id),
    })?;

    let election = _get_election(&election_id).ok_or(Error::NotFound {
        msg: format!("an election with id={} not found", election_id),
    })?;

    if election.start_time > time() {
        // Only allow registration for elections that haven't started yet
        let mut updated_voter = voter.clone();
        updated_voter.registered_elections.push(election_id);
        do_insert_voter(&updated_voter);
        Ok(())
    } else {
        // Election has started; cannot register anymore
        Err(Error::RegistrationClosed {
            msg: "cannot register for an election that has already started".to_string(),
        })
    }
}

#[ic_cdk::update]
fn cast_vote_for_registered_voter(voter_id: u64, election_id: u64, candidate: String) -> Result<Vote, Error> {
    let voter = _get_voter(&voter_id).ok_or(Error::NotFound {
        msg: format!("a voter with id={} not found", voter_id),
    })?;

    if voter.registered_elections.contains(&election_id) {
        // Only allow votes from registered voters
        let payload = VotePayload {
            voter_id,
            candidate,
            election_id,
        };

        cast_vote(payload).ok_or(Error::VoteError {
            msg: "error casting vote".to_string(),
        })
    } else {
        // Voter is not registered for the specified election
        Err(Error::NotRegistered {
            msg: format!("voter with id={} is not registered for election with id={}", voter_id, election_id),
        })
    }
}

// helper method to perform insert for voters.
fn do_insert_voter(voter: &Voter) {
    VOTERS.with(|service| service.borrow_mut().insert(voter.id, voter.clone()));
}

// a helper method to get a voter by id.
fn _get_voter(id: &u64) -> Option<Voter> {
    VOTERS.with(|service| service.borrow().get(id))
}



#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
