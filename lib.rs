#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[openbrush::contract]
mod xtransfer {
    use ink_lang::codegen::Env;
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
    use ink_storage::Mapping;
    use scale::{Decode, Encode};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct XTransfer {
        admin: AccountId,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BadOrigin,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl XTransfer {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|this: &mut Self| {
                this.admin = Self::env().caller();
            })
        }

        /// Admin renounce ownership to a specific account
        ///
        /// The caller must be the badge admin.
        #[ink(message)]
        pub fn renounce_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.esure_admin()?;
            self.admin = new_owner;
            Ok(())
        }

        /// Returns error if caller is not admin
        fn esure_admin(&self) -> Result<()> {
            let caller = self.env().caller();
            if self.admin != caller {
                return Err(Error::BadOrigin);
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use openbrush::traits::mock::{Addressable, SharedCallStack};

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<Environment>()
        }

        #[ink::test]
        fn test_renounce_ownership() {}
    }
}
