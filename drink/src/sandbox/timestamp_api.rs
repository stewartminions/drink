//! timestamp API for the sandbox.

use crate::{Sandbox, SandboxConfig};

/// Generic Time type.
type MomentOf<R> = <R as pallet_timestamp::Config>::Moment;

impl<Config: SandboxConfig> Sandbox<Config>
where
    Config::Runtime: pallet_timestamp::Config,
{
    /// Return the timestamp of the current block.
    pub fn get_timestamp(&self) -> MomentOf<Config::Runtime> {
        self.execute_with(pallet_timestamp::Pallet::<Config::Runtime>::get)
    }

    /// Set the timestamp of the current block.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The new timestamp to be set.
    pub fn set_timestamp(&self, timestamp: MomentOf<Config::Runtime>) {
        self.execute_with(|| pallet_timestamp::Pallet::<Config::Runtime>::set_timestamp(timestamp))
    }
}

#[cfg(test)]
mod tests {
    use crate::{runtime::MinimalRuntime, Sandbox};

    #[test]
    fn getting_and_setting_timestamp_works() {
        let sandbox = Sandbox::<MinimalRuntime>::default();
        for timestamp in 0..10 {
            assert_ne!(sandbox.get_timestamp(), timestamp);
            sandbox.set_timestamp(timestamp);
            assert_eq!(sandbox.get_timestamp(), timestamp);

            sandbox.build_block();
        }
    }
}
