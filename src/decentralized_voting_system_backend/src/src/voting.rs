
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{VirtualMemory, MemoryId};
use ic_stable_structures::{StableBTreeMap, Storable, DefaultMemoryImpl};
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static VOTES_STORAGE: RefCell<StableBTreeMap<u64, Vote, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Vote {
    id: u64,
    voter_id: u64,
    candidate: String,
    election_id: u64,
    timestamp: u64,
}

impl Storable for Vote {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Vote {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Function to cast a vote
pub fn cast_vote(payload: VotePayload) -> Option<Vote> {
    // Validate if the election is still open for voting
    if is_election_open(payload.election_id) {
        // Check if the user has already cast a vote in this election
        if !has_user_cast_vote(payload.voter_id, payload.election_id) {
            let vote_id = ID_COUNTER
                .with(|counter| {
                    let current_value = *counter.borrow().get();
                    counter.borrow_mut().set(current_value + 1)
                })
                .expect("cannot increment id counter");

            let vote = Vote {
                id: vote_id,
                voter_id: payload.voter_id,
                candidate: payload.candidate,
                election_id: payload.election_id,
                timestamp: time(),
            };

            do_insert_vote(&vote);
            Some(vote)
        } else {
            // User has already cast a vote in this election
            None
        }
    } else {
        // Election is closed for voting
        None
    }
}

// Function to get a vote by ID
pub fn get_vote(vote_id: u64) -> Option<Vote> {
    VOTES_STORAGE.with(|service| service.borrow().get(&vote_id))
}

// Internal helper method to check if a user has already cast a vote in a specific election
fn has_user_cast_vote(voter_id: u64, election_id: u64) -> bool {
    VOTES_STORAGE
        .with(|service| {
            service
                .borrow()
                .values()
                .any(|vote| vote.voter_id == voter_id && vote.election_id == election_id)
        })
}

// Internal helper method to check if an election is still open for voting
fn is_election_open(election_id: u64) -> bool {
    // Placeholder logic - you might have a more sophisticated mechanism to check election status
    // For now, assume all elections are always open
    true
}

// Helper method to perform insert for votes
fn do_insert_vote(vote: &Vote) {
    VOTES_STORAGE.with(|service| service.borrow_mut().insert(vote.id, vote.clone()));
}

