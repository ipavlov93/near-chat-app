use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Timestamp};
use schemars::JsonSchema;

/// # Validation settings
/// The minimum message length
const MIN_MSG_LENGTH: usize = 4;
/// The maximum message length
const MAX_MSG_LENGTH: usize = 1000;

/// # Description
/// Message is supposed to store only `text` actually.
/// Sender and sending date could be easily found in final tx.
/// Dropping these fields here will help to minify storage consumption by the contract.
/// Indexing data by separated backend service will help to optimise search.
#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize, JsonSchema,
)]
#[serde(crate = "near_sdk::serde")]
pub struct Message {
    pub text: String,
    pub sender: AccountId,
    pub created_at: Timestamp,
}

/// # Description
/// Pagination messages
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Messages {
    pub total: U64,
    pub messages: Vec<Message>,
}

/// # Description
/// Checks input message length
pub fn validate_message(message: String) -> bool {
    if message.len() < MIN_MSG_LENGTH || message.len() > MAX_MSG_LENGTH {
        return false;
    }
    true
}
