#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use openbrush::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_lending_storage, LendingStorage, LendingStorageField);
