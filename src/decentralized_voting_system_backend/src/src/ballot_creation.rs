
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{StableBTreeMap, Storable, DefaultMemoryImpl, MemoryId};
use std::collections::HashMap;
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static BALLOTS: RefCell<StableBTreeMap<u64, Ballot, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Ballot {
    id: u64,
    election_id: u64,
    options: Vec<String>,
    start_time: u64,
    end_time: u64,
}

impl Storable for Ballot {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Ballot {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Function to create a new ballot
pub fn create_ballot(election_id: u64, options: Vec<String>, start_time: u64, end_time: u64) -> Option<Ballot> {
    let ballot_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let ballot = Ballot {
        id: ballot_id,
        election_id,
        options,
        start_time,
        end_time,
    };

    // Validate start and end times
    if end_time > start_time {
        do_insert_ballot(&ballot);
        Some(ballot)
    } else {
        
        None
    }
}

// Function to get a ballot by ID
pub fn get_ballot(ballot_id: u64) -> Option<Ballot> {
    BALLOTS.with(|service| service.borrow().get(&ballot_id))
}

// Function to get all ballots for a specific election
pub fn get_ballots_for_election(election_id: u64) -> Vec<Ballot> {
    BALLOTS
        .with(|service| {
            service
                .borrow()
                .values()
                .filter(|ballot| ballot.election_id == election_id)
                .cloned()
                .collect::<Vec<_>>()
        })
}

// Helper method to perform insert for ballots
fn do_insert_ballot(ballot: &Ballot) {
    BALLOTS.with(|service| service.borrow_mut().insert(ballot.id, ballot.clone()));
}



