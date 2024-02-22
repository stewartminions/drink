//! A sandboxed runtime.

mod sandbox_config;
pub use sandbox_config::SandboxConfig;
pub mod balance_api;
pub mod contracts_api;
pub mod runtime_api;
pub mod system_api;
pub mod timestamp_api;

use std::any::Any;

use sp_externalities::Extension;

/// A sandboxed runtime.
#[derive(frame_support::DefaultNoBound)]
pub struct Sandbox<Config>(std::marker::PhantomData<Config>);

impl<Config: SandboxConfig> Sandbox<Config> {
    /// Execute the given closure with the inner externallities.
    ///
    /// Returns the result of the given closure.
    pub fn execute_with<T>(&self, execute: impl FnOnce() -> T) -> T {
        Config::execute_with(execute)
    }

    /// Run an action without modifying the storage.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to run.
    pub fn dry_run<T>(&self, action: impl FnOnce() -> T) -> T {
        Config::dry_run(action)
    }

    /// Registers an extension.
    pub fn register_extension<E: Any + Extension>(&self, ext: E) {
        Config::register_extension(ext);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MinimalRuntime;
    #[test]
    fn dry_run_works() {
        let sandbox = Sandbox::<MinimalRuntime>::default();
        let balance = sandbox.free_balance(&MinimalRuntime::default_actor());

        let dry_run_balance = sandbox.dry_run(|| {
            sandbox
                .mint_into(&MinimalRuntime::default_actor(), 100)
                .unwrap();

            sandbox.free_balance(&MinimalRuntime::default_actor())
        });

        assert_eq!(balance + 100, dry_run_balance);

        assert_eq!(
            sandbox.free_balance(&MinimalRuntime::default_actor()),
            balance
        );
    }
}
