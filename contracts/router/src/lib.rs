pub mod account;
pub mod contract;
pub mod owner;
mod testing;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};

/// # Pagination settings
/// The maximum amount of messages per page. Message amount that can be read at once
pub(crate) const MAX_LIMIT: u64 = 100;

/// This is an enum that contains storage keys.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    ChatAccounts,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Router {
    /// owner of this contract
    owner_id: AccountId,
    /// Chat contract account ids
    chat_accounts: Vector<AccountId>,
}

impl Default for Router {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Router {
    /// # Description
    /// Creates the contract and inits whitelist
    /// # Params
    /// * **owner_id** is a string of type [`ValidAccountId`] contains valid account.
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Router {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner_id: owner_id.into(),
            chat_accounts: Vector::new(StorageKeys::ChatAccounts),
        }
    }
}
