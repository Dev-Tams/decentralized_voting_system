use ic_cdk::api::time;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{StableBTreeMap, Storable, DefaultMemoryImpl, MemoryId};
use std::collections::HashMap;
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
struct Vote {
    id: u64,
    voter_id: u64,
    candidate: String,
    election_id: u64,
    timestamp: u64,
}


// Function to calculate election results
pub fn calculate_election_results(election_id: u64) -> HashMap<String, u64> {
    let votes = VOTES_STORAGE.with(|service| {
        service
            .borrow()
            .values()
            .filter(|vote| vote.election_id == election_id)
            .collect::<Vec<_>>()
    });

    let mut result_map = HashMap::new();

    for vote in votes {
        let candidate_count = result_map.entry(vote.candidate.clone()).or_insert(0);
        *candidate_count += 1;
    }

    result_map
}
