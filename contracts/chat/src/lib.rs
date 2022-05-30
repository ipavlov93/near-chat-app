pub mod contract;
pub mod message;
mod testing;

use crate::message::Message;
use chat_space_package::accounts_whitelist::WhitelistAccounts;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, Vector};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};

/// # Pagination settings
/// The maximum amount of messages per page. Message amount that can be read at once
pub(crate) const MAX_LIMIT: u64 = 100;

/// This is an enum that contains storage keys
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    MessagesKey,
    WhitelistedAccountsKey,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Chat {
    /// This is a vector that contains account messages
    messages: Vector<Message>,
    /// This is a set that contains allowed account keys to make contract calls
    whitelist: LookupSet<AccountId>,
}

impl Default for Chat {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Chat {
    /// # Description
    /// Creates the contract and inits whitelist
    /// # Params
    /// * **accounts** is an object of type [`WhitelistAccounts`] contains valid accounts.
    #[init]
    pub fn new(accounts: WhitelistAccounts) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        // whitelist should store only valid accounts
        let mut whitelist = LookupSet::new(StorageKeys::WhitelistedAccountsKey);
        for account in accounts.accounts.iter() {
            whitelist.insert(account.as_ref());
        }

        Self {
            whitelist,
            messages: Vector::new(StorageKeys::MessagesKey),
        }
    }
}
