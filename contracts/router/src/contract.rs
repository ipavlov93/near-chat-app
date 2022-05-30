//! This contract implements accounts cache that's stored in blockchain.
//!
//! The contract provides methods to [add_chat], [delete_chat], [get_chat_accounts].
//!
//! [add_chat]: struct.Router.html#method.add_chat
//! [delete_chat]: struct.Router.html#method.delete_chat
//! [get_chat_accounts]: struct.Router.html#method.get_chat_accounts

use chat_space_package::pagination::calculate_pagination_range;
use near_sdk::json_types::U64;
use near_sdk::{near_bindgen, AccountId};

use crate::account::Accounts;
use crate::Router;
use crate::*;

near_sdk::setup_alloc!();

#[near_bindgen]
impl Router {
    /// # Description
    /// Owner adds given chat account id if it hasn't already exist
    /// # Params
    /// * **account_id** is account of type [`ValidAccountId`] contains validated account id.
    pub fn add_chat(&mut self, account_id: ValidAccountId) {
        // check ownership
        self.assert_owner();
        for chat in self.chat_accounts.iter() {
            assert_ne!(
                chat.to_string(),
                account_id.to_string(),
                "Account id already exists"
            );
        }
        self.chat_accounts.push(&account_id.into());
    }

    /// # Description
    /// Owner deletes chat account by index.
    /// Be aware that last element will be stored at given index position after deleting.
    /// # Params
    /// * **account_id** is a chat account index of type [`U64`].
    pub fn delete_chat(&mut self, index: U64) -> AccountId {
        // check ownership
        self.assert_owner();

        if self.chat_accounts.len() > index.0 {
            return self.chat_accounts.swap_remove(index.0)
        }
        AccountId::new()
    }

    /// # Description
    /// Returns chat accounts batch with [`MAX_LIMIT`] maximum length
    /// # Params
    /// * **page** is a page number for pagination.
    pub fn get_chat_accounts(&self, page: u64, mut limit: u64) -> Accounts {
        let total = self.chat_accounts.len();

        if limit > MAX_LIMIT {
            limit = MAX_LIMIT
        }

        let (offset, mut last_account_index) = calculate_pagination_range(page, limit);
        if last_account_index > total {
            last_account_index = total;
        }

        let mut accounts: Vec<AccountId> = Vec::new();
        for index in offset..last_account_index {
            accounts.push(self.chat_accounts.get(index).unwrap())
        }

        Accounts {
            total: U64::from(total),
            accounts,
        }
    }
}
