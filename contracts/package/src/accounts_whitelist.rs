use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, PartialEq, Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize,
)]
#[serde(crate = "near_sdk::serde")]
pub struct WhitelistAccounts {
    pub accounts: Vec<ValidAccountId>,
}
