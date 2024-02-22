#![allow(missing_docs)] // `construct_macro` doesn't allow doc comments for the runtime type.

use std::time::SystemTime;

use frame_support::{sp_runtime::traits::Header, traits::Hooks};
pub struct BlockBuilder<T>(std::marker::PhantomData<T>);

impl<
        T: pallet_balances::Config + pallet_timestamp::Config<Moment = u64> + pallet_contracts::Config,
    > BlockBuilder<T>
{
    pub fn initialize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<T>,
        parent_hash: <T as frame_system::Config>::Hash,
    ) {
        frame_system::Pallet::<T>::reset_events();
        frame_system::Pallet::<T>::initialize(&height, &parent_hash, &Default::default());
        pallet_balances::Pallet::<T>::on_initialize(height);
        pallet_timestamp::Pallet::<T>::set_timestamp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        );
        pallet_timestamp::Pallet::<T>::on_initialize(height);
        pallet_contracts::Pallet::<T>::on_initialize(height);
        frame_system::Pallet::<T>::note_finished_initialize();
    }

    pub fn finalize_block(
        height: frame_system::pallet_prelude::BlockNumberFor<T>,
    ) -> Result<<T as frame_system::Config>::Hash, String> {
        pallet_contracts::Pallet::<T>::on_finalize(height);
        pallet_timestamp::Pallet::<T>::on_finalize(height);
        pallet_balances::Pallet::<T>::on_finalize(height);
        Ok(frame_system::Pallet::<T>::finalize().hash())
    }
}

/// The macro will generate an implementation of `drink::SandboxConfig` for the given runtime type.
#[macro_export]
macro_rules! impl_sandbox_config {
    ($name:ident, $runtime:tt, $default_balance:expr, $default_actor:expr) => {
        $crate::paste::paste! {
            $crate::impl_sandbox_config!([<EXT_ $name:upper>], $name, $runtime, $default_balance, $default_actor);
        }
    };

    ($ext:ident, $name:ident, $runtime:tt, $default_balance:expr, $default_actor:expr) => {
        thread_local! {
            static $ext: ::std::cell::RefCell<$crate::TestExternalities> = ::std::cell::RefCell::new({
                use $crate::frame_support::sp_runtime::BuildStorage;
                let mut storage = $crate::frame_system::GenesisConfig::<$runtime>::default()
                    .build_storage().unwrap();

                $crate::pallet_balances::GenesisConfig::<$runtime> {
                    balances: vec![($default_actor, $default_balance)],
                }
                .assimilate_storage(&mut storage)
                .unwrap();

                let mut ext = $crate::TestExternalities::new(storage);
                ext.execute_with(|| {
                    <$name as $crate::SandboxConfig>::initialize_block(1, Default::default())
                });

                ext
            });
        }

        impl $crate::SandboxConfig for $name {
            type Runtime = $runtime;

            fn execute_with<T>(execute: impl FnOnce() -> T) -> T {
                $ext.with(|v| v.borrow_mut().execute_with(execute))
            }

            fn dry_run<T>(action: impl FnOnce() -> T) -> T {
                $ext.with(|v| {
                    // Make a backup of the backend.
                    let backend_backup = v.borrow_mut().as_backend();

                    // Run the action, potentially modifying storage. Ensure, that there are no pending changes
                    // that would affect the reverted backend.
                    let result = action();

                    let mut v = v.borrow_mut();
                    v.commit_all().expect("Failed to commit changes");

                    // Restore the backend.
                    v.backend = backend_backup;
                    result
                })
            }

            fn register_extension<E: ::core::any::Any + $crate::Extension>(ext: E) {
                $ext.with(|v| v.borrow_mut().register_extension(ext));
            }

            fn initialize_block(
                height: $crate::frame_system::pallet_prelude::BlockNumberFor<$runtime>,
                parent_hash: <$runtime as $crate::frame_system::Config>::Hash,
            ) {
                $crate::minimal::BlockBuilder::<$runtime>::initialize_block(height, parent_hash)
            }

            fn finalize_block(
                height: $crate::frame_system::pallet_prelude::BlockNumberFor<$runtime>,
            ) -> Result<<$runtime as $crate::frame_system::Config>::Hash, String> {
                $crate::minimal::BlockBuilder::<$runtime>::finalize_block(height)
            }

            fn default_actor() -> $crate::runtime::AccountIdFor<$runtime> {
                $default_actor
            }

            fn get_metadata() -> $crate::runtime::RuntimeMetadataPrefixed {
                $runtime::metadata()
            }

            fn convert_account_to_origin(
                account: $crate::runtime::AccountIdFor<$runtime>,
            ) -> <<$runtime as $crate::frame_system::Config>::RuntimeCall as $crate::frame_support::sp_runtime::traits::Dispatchable>::RuntimeOrigin {
                Some(account).into()
            }
        }
    };
}

/// Macro creating a minimal runtime with the given name. Optionally can take a chain extension
/// type as a second argument.
///
/// The new macro will automatically implement `drink::SandboxConfig`.
#[macro_export]
macro_rules! create_minimal_runtime {
    ($name:ident) => {
        create_minimal_runtime!($name, ());
    };
    ($name:ident, $chain_extension: ty) => {

// ------------ Put all the boilerplate into an auxiliary module -----------------------------------
mod construct_runtime {

    // ------------ Bring some common types into the scope -----------------------------------------
    use $crate::frame_support::{
        construct_runtime,
        derive_impl,
        parameter_types,
        sp_runtime::{
            testing::H256,
            traits::Convert,
            AccountId32, Perbill,
        },
        traits::{ConstBool, ConstU128, ConstU32, ConstU64, Currency, Randomness},
        weights::Weight,
    };
    use $crate::runtime::pallet_contracts_debugging::DrinkDebug;

    // ------------ Define the runtime type as a collection of pallets -----------------------------
    construct_runtime!(
        pub enum $name {
            System: $crate::frame_system,
            Balances: $crate::pallet_balances,
            Timestamp: $crate::pallet_timestamp,
            Contracts: $crate::pallet_contracts,
        }
    );

    // ------------ Configure pallet system --------------------------------------------------------
    #[derive_impl($crate::frame_system::config_preludes::SolochainDefaultConfig as $crate::frame_system::DefaultConfig)]
    impl $crate::frame_system::Config for $name {
        type Block = $crate::frame_system::mocking::MockBlockU32<$name>;
        type Version = ();
        type BlockHashCount = ConstU32<250>;
        type AccountData = $crate::pallet_balances::AccountData<<$name as $crate::pallet_balances::Config>::Balance>;
    }

    // ------------ Configure pallet balances ------------------------------------------------------
    impl $crate::pallet_balances::Config for $name {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
        type Balance = u128;
        type DustRemoval = ();
        type ExistentialDeposit = ConstU128<1>;
        type AccountStore = System;
        type ReserveIdentifier = [u8; 8];
        type FreezeIdentifier = ();
        type MaxLocks = ();
        type MaxReserves = ();
        type MaxHolds = ConstU32<1>;
        type MaxFreezes = ();
        type RuntimeHoldReason = RuntimeHoldReason;
        type RuntimeFreezeReason = RuntimeFreezeReason;
    }

    // ------------ Configure pallet timestamp -----------------------------------------------------
    impl $crate::pallet_timestamp::Config for $name {
        type Moment = u64;
        type OnTimestampSet = ();
        type MinimumPeriod = ConstU64<1>;
        type WeightInfo = ();
    }

    // ------------ Configure pallet contracts -----------------------------------------------------
    pub enum SandboxRandomness {}
    impl Randomness<H256, u32> for SandboxRandomness {
        fn random(_subject: &[u8]) -> (H256, u32) {
            unreachable!("No randomness")
        }
    }

    type BalanceOf = <Balances as Currency<AccountId32>>::Balance;
    impl Convert<Weight, BalanceOf> for $name {
        fn convert(w: Weight) -> BalanceOf {
            w.ref_time().into()
        }
    }

    parameter_types! {
        pub SandboxSchedule: $crate::pallet_contracts::Schedule<$name> = {
            <$crate::pallet_contracts::Schedule<$name>>::default()
        };
        pub DeletionWeightLimit: Weight = Weight::zero();
        pub DefaultDepositLimit: BalanceOf = 10_000_000;
        pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(0);
        pub MaxDelegateDependencies: u32 = 32;
    }

    impl $crate::pallet_contracts::Config for $name {
        type Time = Timestamp;
        type Randomness = SandboxRandomness;
        type Currency = Balances;
        type RuntimeEvent = RuntimeEvent;
        type RuntimeCall = RuntimeCall;
        type CallFilter = ();
        type WeightPrice = Self;
        type WeightInfo = ();
        type ChainExtension = $chain_extension;
        type Schedule = SandboxSchedule;
        type CallStack = [$crate::pallet_contracts::Frame<Self>; 5];
        type DepositPerByte = ConstU128<1>;
        type DepositPerItem = ConstU128<1>;
        type AddressGenerator = $crate::pallet_contracts::DefaultAddressGenerator;
        type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
        type MaxStorageKeyLen = ConstU32<128>;
        type UnsafeUnstableInterface = ConstBool<false>;
        type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
        type Migrations = ();
        type DefaultDepositLimit = DefaultDepositLimit;
        type Debug = DrinkDebug;
        type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
        type MaxDelegateDependencies = MaxDelegateDependencies;
        type RuntimeHoldReason = RuntimeHoldReason;
        type Environment = ();
        type Xcm = ();
    }

    // ------------ Implement `drink::Runtime` trait ---------------------------------------------------

    /// Default initial balance for the default account.
    pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

    // implement the `drink::SandboxConfig` for the runtime type
    $crate::impl_sandbox_config!($name, $name, INITIAL_BALANCE, AccountId32::new([1u8; 32]));
}




// ------------ Export runtime type itself, pallets and useful types from the auxiliary module -----
pub use construct_runtime::{
    $name, Balances, Contracts, PalletInfo, RuntimeCall, RuntimeEvent, RuntimeHoldReason,
    RuntimeOrigin, System, Timestamp,
};
    };
}

create_minimal_runtime!(MinimalRuntime);
