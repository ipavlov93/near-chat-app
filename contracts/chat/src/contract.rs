//! This contract implements immutable chat that's stored in blockchain.
//!
//! The contract provides methods to [send_message], [get_messages], [is_whitelisted_account].
//!
//! [send_message]: struct.Chat.html#method.send_message
//! [get_messages]: struct.Chat.html#method.get_messages
//! [is_whitelisted_account]: struct.Chat.html#method.is_whitelisted_account

use near_sdk::json_types::{ValidAccountId, U64};
use near_sdk::{env, near_bindgen};

use chat_space_package::pagination::calculate_pagination_range;

use crate::message::{validate_message, Message, Messages};
use crate::*;

near_sdk::setup_alloc!();

#[near_bindgen]
impl Chat {
    /// # Description
    /// Check does whitelist contain given account
    /// # Params
    /// * **account_id** is a string of type [`ValidAccountId`] contains validated account.
    pub fn is_whitelisted_account(&self, account_id: ValidAccountId) -> bool {
        self.whitelist.contains(&account_id.into())
    }

    /// # Description
    /// Publishes given message to storage if it's valid. Sender account must be whitelisted
    /// # Params
    /// * **text_message** is a string text message.
    pub fn send_message(&mut self, text_message: String) {
        let account_id = env::signer_account_id();

        // check whitelist
        assert!(self.whitelist.contains(&account_id), "Not enough rights");

        assert!(validate_message(text_message.clone()), "Invalid message");

        let message = Message {
            text: text_message,
            sender: env::signer_account_id(),
            created_at: env::block_timestamp(),
        };
        self.messages.push(&message);
    }

    /// # Description
    /// Returns message batch with [`MAX_LIMIT`] maximum length
    /// # Params
    /// * **page** is a page number for pagination.
    pub fn get_messages(&self, page: u64, mut limit: u64) -> Messages {
        let total = self.messages.len();

        if limit > MAX_LIMIT {
            limit = MAX_LIMIT
        }

        let (offset, mut last_message_index) = calculate_pagination_range(page, limit);
        if last_message_index > total {
            last_message_index = total;
        }

        let mut messages: Vec<Message> = Vec::new();
        for index in offset..last_message_index {
            messages.push(self.messages.get(index).unwrap())
        }

        Messages {
            total: U64::from(total),
            messages,
        }
    }
}
