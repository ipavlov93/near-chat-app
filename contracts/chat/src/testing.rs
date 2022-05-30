// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::test_env::{alice, bob, carol};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, MockedBlockchain, VMContext};

    use crate::WhitelistAccounts;
    use crate::{Chat, MAX_LIMIT};

    // part of writing unit tests is setting up a mock context
    fn get_context(
        is_view: bool,
        signer: ValidAccountId,
        current_account_id: ValidAccountId,
    ) -> VMContext {
        VMContextBuilder::new()
            .block_timestamp(100)
            .current_account_id(current_account_id)
            .signer_account_id(signer)
            .is_view(is_view)
            .build()
    }

    #[test]
    fn whitelist_account_should_send_message() {
        // test accounts
        let account = alice().clone();
        let whitelisted_account = bob().clone();
        let owner = carol().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            whitelisted_account.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        let mut whitelist = WhitelistAccounts::default();

        whitelist
            .accounts
            .push(ValidAccountId::try_from(whitelisted_account.clone().to_string()).unwrap());
        // init a contract
        let mut contract = Chat::new(whitelist);
        assert_eq!(
            contract.is_whitelisted_account(owner.try_into().unwrap()),
            false
        );
        assert_eq!(
            contract.is_whitelisted_account(account.try_into().unwrap()),
            false
        );
        assert_eq!(
            contract.is_whitelisted_account(whitelisted_account.try_into().unwrap()),
            true
        );

        let msg = "first msg".to_string();
        contract.send_message(msg);
    }

    #[test]
    #[should_panic(expected = "Invalid message")]
    fn sending_invalid_message() {
        let whitelisted_account = bob().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            bob().clone().try_into().unwrap(),
            bob().clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        let mut whitelist = WhitelistAccounts::default();

        whitelist
            .accounts
            .push(ValidAccountId::try_from(whitelisted_account.to_string()).unwrap());
        // init a contract
        let mut contract = Chat::new(whitelist);

        let msg = "msg".to_string();
        contract.send_message(msg);
    }

    #[test]
    #[should_panic(expected = "Not enough rights")]
    fn sending_restricted() {
        // test accounts
        let account = alice().clone();
        let whitelisted_account = bob().clone();
        let owner = carol().clone();

        let context = get_context(
            false,
            account.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
        );
        testing_env!(context.clone());
        let mut whitelist = WhitelistAccounts::default();
        whitelist
            .accounts
            .push(ValidAccountId::try_from(whitelisted_account.to_string()).unwrap());
        let mut contract = Chat::new(whitelist);

        let msg = "first msg".to_string();
        contract.send_message(msg);
    }

    #[test]
    fn should_get_pagination_messages() {
        let whitelisted_account = bob().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            bob().clone().try_into().unwrap(),
            bob().clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        // init a contract
        let mut whitelist = WhitelistAccounts::default();
        whitelist
            .accounts
            .push(ValidAccountId::try_from(whitelisted_account.to_string()).unwrap());
        let mut contract = Chat::new(whitelist);

        // empty chat
        let res = contract.get_messages(0, MAX_LIMIT);
        assert_eq!(res.total.0, 0);
        assert_eq!(res.messages.len(), 0);

        // sent fake messages
        let msg_number = 2;
        for i in 1..msg_number + 1 {
            let msg = format!("msg {} from {}", i, whitelisted_account);
            contract.send_message(msg);
        }

        let res = contract.get_messages(0, MAX_LIMIT);
        assert_eq!(res.total.0, msg_number);
        assert_eq!(res.messages.len(), msg_number as usize);

        let res = contract.get_messages(1, MAX_LIMIT);
        assert_eq!(res.messages.len(), msg_number as usize);

        let res = contract.get_messages(2, MAX_LIMIT);
        assert_eq!(res.messages.len(), 0);

        // sent fake messages again
        let additional_msg_number = MAX_LIMIT * 2;
        for i in 1..additional_msg_number + 1 {
            let msg = format!("msg {} from {}", i, whitelisted_account);
            contract.send_message(msg);
        }

        let res = contract.get_messages(0, MAX_LIMIT);
        assert_eq!(res.total.0, msg_number + additional_msg_number);
        assert_eq!(res.messages.len(), MAX_LIMIT as usize);

        let res = contract.get_messages(1, MAX_LIMIT);
        assert_eq!(res.messages.len(), MAX_LIMIT as usize);
        let res = contract.get_messages(2, MAX_LIMIT);
        assert_eq!(res.messages.len(), MAX_LIMIT as usize);
        let res = contract.get_messages(3, MAX_LIMIT);
        assert_eq!(res.messages.len(), msg_number as usize);
        let res = contract.get_messages(4, MAX_LIMIT);
        assert_eq!(res.messages.len(), 0);
    }
}
