#[cfg(test)]
mod tests {
    use near_sdk::{MockedBlockchain, testing_env, VMContext};
    use near_sdk::json_types::{U64, ValidAccountId};
    use near_sdk::test_utils::test_env::{alice, bob, carol};
    use near_sdk::test_utils::VMContextBuilder;

    use crate::{MAX_LIMIT, Router};

    // part of writing unit tests is setting up a mock context
    fn get_context(
        is_view: bool,
        signer: ValidAccountId,
        current_account_id: ValidAccountId,
        predecessor_account_id: ValidAccountId,
    ) -> VMContext {
        VMContextBuilder::new()
            .block_timestamp(100)
            .current_account_id(current_account_id)
            .predecessor_account_id(predecessor_account_id)
            .signer_account_id(signer)
            .is_view(is_view)
            .build()
    }

    #[test]
    fn should_add_and_delete_chat() {
        let owner = carol().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            owner.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        // init a contract
        let mut contract =
            Router::new(ValidAccountId::try_from(owner.clone().to_string()).unwrap());

        // adding account ids
        let number = 5;
        for i in 0..number {
            contract.add_chat(ValidAccountId::try_from(format!("{}.near", i)).unwrap());
        }
        let res = contract.get_chat_accounts(1, MAX_LIMIT);
        assert_eq!(res.total.0, number);
        assert_eq!(res.accounts.len(), number as usize);

        // deleting element that doesn't exist
        let element = contract.delete_chat(U64::from(number));
        assert_eq!(element, "");
        assert_eq!(res.total.0, number);

        // deleting element
        let index = 1;
        let element = contract.delete_chat(U64::from(index));
        assert_eq!(element, format!("{}.near", index));

        // length changed and last element filled into given index position
        let res = contract.get_chat_accounts(1, MAX_LIMIT);
        assert_eq!(res.total.0, number - 1);
        assert_eq!(res.accounts.len(), (number - 1) as usize);
        assert_eq!(
            res.accounts.get(index as usize).unwrap(),
            &format!("{}.near", number - 1)
        );
    }

    #[test]
    #[should_panic(expected = "Can only be called by owner")]
    fn should_panic_delete_chat() {
        let account = alice().clone();
        let owner = carol().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            account.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
            account.clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        // init a contract
        let mut contract =
            Router::new(ValidAccountId::try_from(owner.clone().to_string()).unwrap());

        // try to delete account
        contract.delete_chat(U64::from(0));
    }

    #[test]
    #[should_panic(expected = "Can only be called by owner")]
    fn should_panic_add_chat() {
        let account = alice().clone();
        let owner = carol().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            account.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
            account.clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        // init a contract
        let mut contract =
            Router::new(ValidAccountId::try_from(owner.clone().to_string()).unwrap());

        // try to add fake account
        contract.add_chat(ValidAccountId::try_from(bob()).unwrap());
    }

    #[test]
    fn should_get_pagination_accounts() {
        let owner = carol().clone();

        // set up the mock context into the testing environment
        let context = get_context(
            false,
            owner.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
            owner.clone().try_into().unwrap(),
        );
        testing_env!(context.clone());

        // init a contract
        let mut contract =
            Router::new(ValidAccountId::try_from(owner.clone().to_string()).unwrap());

        // empty router
        let res = contract.get_chat_accounts(0, MAX_LIMIT);
        assert_eq!(res.total.0, 0);
        assert_eq!(res.accounts.len(), 0);

        // add fake accounts
        let accounts_number = 2;
        for i in 1..accounts_number + 1 {
            contract.add_chat(ValidAccountId::try_from(format!("{}.near", i)).unwrap());
        }

        let res = contract.get_chat_accounts(0, MAX_LIMIT);
        assert_eq!(res.total.0, accounts_number);
        assert_eq!(res.accounts.len(), accounts_number as usize);

        let res = contract.get_chat_accounts(1, MAX_LIMIT);
        assert_eq!(res.accounts.len(), accounts_number as usize);

        let res = contract.get_chat_accounts(2, MAX_LIMIT);
        assert_eq!(res.accounts.len(), 0);

        // add fake accounts again
        let additional_accounts_number = accounts_number * 2;
        let total = additional_accounts_number + accounts_number;
        for i in accounts_number + 1..total + 1 {
            contract.add_chat(ValidAccountId::try_from(format!("{}.near", i)).unwrap());
        }

        let res = contract.get_chat_accounts(0, MAX_LIMIT);
        assert_eq!(res.total.0, accounts_number + additional_accounts_number);
        assert_eq!(res.accounts.len(), total as usize);

        let res = contract.get_chat_accounts(1, MAX_LIMIT);
        assert_eq!(res.accounts.len(), total as usize);

        let res = contract.get_chat_accounts(1, accounts_number);
        assert_eq!(res.accounts.len(), accounts_number as usize);
        let res = contract.get_chat_accounts(2, accounts_number);
        assert_eq!(res.accounts.len(), accounts_number as usize);
        let res = contract.get_chat_accounts(3, accounts_number);
        assert_eq!(res.accounts.len(), accounts_number as usize);
        let res = contract.get_chat_accounts(4, accounts_number);
        assert_eq!(res.accounts.len(), 0);
    }
}
