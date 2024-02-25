//! Module containing the core trait use to customize the sandboxed runtime.

use core::any::Any;

pub mod balance_api;
pub mod contracts_api;
pub mod runtime_api;
pub mod system_api;
pub mod timestamp_api;

/// The prelude of the sandbox module.
pub mod prelude {
    pub use super::{
        balance_api::BalanceAPI, contracts_api::ContractAPI, runtime_api::RuntimeAPI,
        system_api::SystemAPI, timestamp_api::TimestampAPI, Sandbox,
    };
}

use frame_metadata::RuntimeMetadataPrefixed;
use frame_support::sp_runtime::traits::Dispatchable;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_externalities::Extension;

/// The type of an account identifier.
pub type AccountIdFor<R> = <R as frame_system::Config>::AccountId;

/// A runtime to use.
pub trait Sandbox {
    /// The runtime associated with the sandbox.
    type Runtime: frame_system::Config;

    /// Execute the given externalities.
    fn execute_with<T>(&mut self, execute: impl FnOnce() -> T) -> T;

    /// Dry run an action without modifying the storage.
    fn dry_run<T>(&mut self, action: impl FnOnce(&mut Self) -> T) -> T;

    /// Register an extension.
    fn register_extension<E: Any + Extension>(&mut self, ext: E);

    /// Initialize a new block at particular height.
    fn initialize_block(
        _height: BlockNumberFor<Self::Runtime>,
        _parent_hash: <Self::Runtime as frame_system::Config>::Hash,
    ) {
    }

    /// Finalize a block at particular height.
    fn finalize_block(
        _height: BlockNumberFor<Self::Runtime>,
    ) -> <Self::Runtime as frame_system::Config>::Hash {
        Default::default()
    }

    /// Default actor for the sandbox.
    fn default_actor() -> AccountIdFor<Self::Runtime>;

    /// Metadata of the runtime.
    fn get_metadata() -> RuntimeMetadataPrefixed;

    /// Convert an account to an call origin.
    fn convert_account_to_origin(
        account: AccountIdFor<Self::Runtime>,
    ) -> <<Self::Runtime as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin;
}
