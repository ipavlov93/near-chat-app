use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

/// # Description
/// Pagination account ids
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Accounts {
    pub total: U64,
    pub accounts: Vec<AccountId>,
}
