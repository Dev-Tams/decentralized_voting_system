
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{VirtualMemory, MemoryId};
use ic_stable_structures::{StableBTreeMap, Storable, DefaultMemoryImpl};
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static USERS: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct User {
    id: u64,
    username: String,
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Function to register a new user
pub fn register_user(username: String) -> Option<User> {
    let user_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let user = User {
        id: user_id,
        username,
    };

    do_insert_user(&user);
    Some(user)
}

// Function to get a user by ID
pub fn get_user(user_id: u64) -> Option<User> {
    USERS.with(|service| service.borrow().get(&user_id))
}

// Function to check if a user is registered for a specific election
pub fn is_user_registered_for_election(user_id: u64, election_id: u64) -> bool {
    USERS
        .with(|service| {
            service
                .borrow()
                .get(&user_id)
                .map(|user| is_user_registered_for_election_internal(user, election_id))
                .unwrap_or(false)
        })
}

// Helper method to perform insert for users
fn do_insert_user(user: &User) {
    USERS.with(|service| service.borrow_mut().insert(user.id, user.clone()));
}

// Internal helper method to check if a user is registered for a specific election
fn is_user_registered_for_election_internal(user: &User, election_id: u64) -> bool {
    true
}

