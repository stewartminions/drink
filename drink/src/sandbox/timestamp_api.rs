//! timestamp API for the sandbox.

use crate::Sandbox;

/// Generic Time type.
type MomentOf<R> = <R as pallet_timestamp::Config>::Moment;

/// Timestamp API for the sandbox.
pub trait TimestampAPI<T: Sandbox>
where
    T: Sandbox,
    T::Runtime: pallet_timestamp::Config,
{
    /// Return the timestamp of the current block.
    fn get_timestamp(&mut self) -> MomentOf<T::Runtime>;
    /// Set the timestamp of the current block.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The new timestamp to be set.
    fn set_timestamp(&mut self, timestamp: MomentOf<T::Runtime>);
}

impl<T> TimestampAPI<T> for T
where
    T: Sandbox,
    T::Runtime: pallet_timestamp::Config,
{
    fn get_timestamp(&mut self) -> MomentOf<T::Runtime> {
        self.execute_with(pallet_timestamp::Pallet::<T::Runtime>::get)
    }
    fn set_timestamp(&mut self, timestamp: MomentOf<T::Runtime>) {
        self.execute_with(|| pallet_timestamp::Pallet::<T::Runtime>::set_timestamp(timestamp))
    }
}

#[cfg(test)]
mod tests {
    use crate::{runtime::MinimalSandbox, sandbox::prelude::*};

    #[test]
    fn getting_and_setting_timestamp_works() {
        let mut sandbox = MinimalSandbox::default();
        for timestamp in 0..10 {
            assert_ne!(sandbox.get_timestamp(), timestamp);
            sandbox.set_timestamp(timestamp);
            assert_eq!(sandbox.get_timestamp(), timestamp);

            sandbox.build_block();
        }
    }
}
