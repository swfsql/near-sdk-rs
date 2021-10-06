use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, require, AccountId};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ownable {
    pub owner: Option<AccountId>,
}

impl Ownable {
    pub fn new() -> Self {
        Self { owner: Some(env::predecessor_account_id()) }
    }

    pub fn owner(&self) -> Option<AccountId> {
        self.owner.clone()
    }

    pub fn only_owner(&self) {
        require!(
            Some(env::predecessor_account_id()) == self.owner(),
            "Ownable: caller is not the owner"
        );
    }

    pub fn renounce_ownership(&mut self) {
        self.only_owner();
        self.owner = None;
    }

    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        self.only_owner();
        self.owner = Some(new_owner);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let ownable = Ownable::new();
        assert_eq!(ownable.owner(), Some(accounts(1)));
    }

    #[test]
    fn test_only_owner_success() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let ownable = Ownable::new();
        ownable.only_owner();
    }

    #[test]
    #[should_panic(expected = "Ownable: caller is not the owner")]
    fn test_only_owner_fail() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let ownable = Ownable::new();
        let context = get_context(accounts(2));
        testing_env!(context.build());
        ownable.only_owner();
    }

    #[test]
    fn test_renounce_ownership() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut ownable = Ownable::new();
        assert_eq!(ownable.owner(), Some(accounts(1)));
        ownable.renounce_ownership();
        assert_eq!(ownable.owner(), None);
    }

    #[test]
    fn test_transfer_ownership() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let mut ownable = Ownable::new();
        assert_eq!(ownable.owner(), Some(accounts(1)));
        ownable.transfer_ownership(accounts(2));
        assert_eq!(ownable.owner(), Some(accounts(2)));
    }
}
