use crate::Router;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{env, AccountId};

impl Router {
    /// Set the owner of this contract
    /// # Params
    /// * **owner_id** is a string of type [`ValidAccountId`] contains valid account.
    pub fn set_owner(&mut self, owner_id: ValidAccountId) {
        self.assert_owner();
        self.owner_id = owner_id.into();
    }

    /// Get the owner of this contract
    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// Check a caller account id is the owner of this contract
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by owner"
        );
    }
}
