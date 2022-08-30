#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract(env = PinkEnvironment)]
mod xtransfer {
    use ink_lang::codegen::Env;
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
    use ink_storage::Mapping;
    use pink_extension::{push_message, PinkEnvironment};
    use scale::{Decode, Encode};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct XTransfer {
        admin: AccountId,
        reserve_account: AccountId,
        pbridge_registry: Mapping<AccountId, ()>,
        local_reserved: Mapping<AccountId, ()>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BadOrigin,
        CannotDeposit,
        InsufficientBalance,
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
        /// The caller must be the admin.
        #[ink(message)]
        pub fn renounce_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            self.esure_admin()?;
            self.admin = new_owner;
            Ok(())
        }

        /// Admin mark an asset as pbridge enalbed
        ///
        /// The caller must be the admin.
        #[ink(message)]
        pub fn enable_pbridge(&mut self, asset: AccountId, reserved: bool) -> Result<()> {
            self.esure_admin()?;
            self.pbridge_registry.insert(&asset, &());
            if reserved {
                self.local_reserved.insert(&asset, &());
            }
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_fungible(
            &self,
            asset: AccountId,
            recipient: AccountId,
            amount: u128,
        ) -> Result<()> {
            // Check if asset has enabled pbridge
            if self.pbridge_registry.contains(&asset) {
                return Err(Error::CannotDeposit);
            }

            // Check if caller has sufficient balance

            // 1) Withdraw amount of asset from sender

            // 2) If asset reserved on FatContract, deposit amount of asset to reserve account

            // 3) Push message to blockchain
            // let message: Vec<u8> = PBridgeReport::FungibleTransfer(
            //     asset.into(),
            //     MultiLocation::new(
            //         0,
            //         X1(Junction::AccountId32 {
            //             network: NetworkId::Any,
            //             id: recipient.into(),
            //         }),
            //     )
            //     .encode(),
            //     amount,
            // ).encode();
            let topic: Vec<u8> = b"phala/contract/pbridge/report".to_vec();
            // push_message(message, topic)

            Ok(())
        }

        #[ink(message)]
        pub fn send_xcm_onchain(&self, message: Vec<u8>, topic: Vec<u8>) {
            push_message(message, topic)
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

        fn default_accounts() -> ink_env::test::DefaultAccounts<PinkEnvironment> {
            ink_env::test::default_accounts::<Environment>()
        }

        #[ink::test]
        fn test_renounce_ownership() {}
    }
}
