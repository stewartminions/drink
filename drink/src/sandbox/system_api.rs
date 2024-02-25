//! System API for the sandbox.

use frame_support::sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};
use frame_system::pallet_prelude::BlockNumberFor;

use super::Sandbox;
use crate::{EventRecordOf, RuntimeCall};

/// System API for the sandbox.
pub trait SystemAPI<T>
where
    T: Sandbox,
    T::Runtime: frame_system::Config,
{
    /// Return the current height of the chain.
    fn block_number(&mut self) -> BlockNumberFor<T::Runtime>;

    /// Return the events of the current block so far.
    fn events(&mut self) -> Vec<EventRecordOf<T::Runtime>>;

    /// Reset the events of the current block.
    fn reset_events(&mut self);

    /// Execute a runtime call (dispatchable).
    ///
    /// # Arguments
    ///
    /// * `call` - The runtime call to execute.
    /// * `origin` - The origin of the call.
    fn runtime_call<Origin: Into<<RuntimeCall<T::Runtime> as Dispatchable>::RuntimeOrigin>>(
        &mut self,
        call: RuntimeCall<T::Runtime>,
        origin: Origin,
    ) -> DispatchResultWithInfo<<RuntimeCall<T::Runtime> as Dispatchable>::PostInfo>;
}

impl<T> SystemAPI<T> for T
where
    T: Sandbox,
    T::Runtime: frame_system::Config,
{
    fn block_number(&mut self) -> BlockNumberFor<T::Runtime> {
        self.execute_with(frame_system::Pallet::<T::Runtime>::block_number)
    }

    fn events(&mut self) -> Vec<EventRecordOf<T::Runtime>> {
        self.execute_with(frame_system::Pallet::<T::Runtime>::events)
    }

    fn reset_events(&mut self) {
        self.execute_with(frame_system::Pallet::<T::Runtime>::reset_events)
    }

    fn runtime_call<Origin: Into<<RuntimeCall<T::Runtime> as Dispatchable>::RuntimeOrigin>>(
        &mut self,
        call: RuntimeCall<T::Runtime>,
        origin: Origin,
    ) -> DispatchResultWithInfo<<RuntimeCall<T::Runtime> as Dispatchable>::PostInfo> {
        self.execute_with(|| call.dispatch(origin.into()))
    }
}

#[cfg(test)]
mod tests {
    use frame_support::sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};

    use crate::{
        minimal::{MinimalSandbox, MinimalSandboxRuntime},
        runtime::minimal::RuntimeEvent,
        sandbox::prelude::*,
        AccountId32, RuntimeCall, Sandbox,
    };

    fn make_transfer(
        sandbox: &mut MinimalSandbox,
        dest: AccountId32,
        value: u128,
    ) -> DispatchResultWithInfo<<RuntimeCall<MinimalSandboxRuntime> as Dispatchable>::PostInfo>
    {
        assert_ne!(
            MinimalSandbox::default_actor(),
            dest,
            "make_transfer should send to account different than default_actor"
        );
        sandbox.runtime_call(
            RuntimeCall::<MinimalSandboxRuntime>::Balances(pallet_balances::Call::<
                MinimalSandboxRuntime,
            >::transfer_allow_death {
                dest: dest.into(),
                value,
            }),
            Some(MinimalSandbox::default_actor()),
        )
    }

    #[test]
    fn dry_run_works() {
        let mut sandbox = MinimalSandbox::default();
        let actor = MinimalSandbox::default_actor();
        let initial_balance = sandbox.free_balance(&actor);

        sandbox.dry_run(|sandbox| {
            sandbox.mint_into(&actor, 100).unwrap();
            assert_eq!(sandbox.free_balance(&actor), initial_balance + 100);
        });

        assert_eq!(sandbox.free_balance(&actor), initial_balance);
    }

    #[test]
    fn runtime_call_works() {
        let mut sandbox = MinimalSandbox::default();

        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);
        let initial_balance = sandbox.free_balance(&RECIPIENT);

        let result = make_transfer(&mut sandbox, RECIPIENT, 100);
        assert!(result.is_ok());

        let expected_balance = initial_balance + 100;
        assert_eq!(sandbox.free_balance(&RECIPIENT), expected_balance);
    }

    #[test]
    fn current_events() {
        let mut sandbox = MinimalSandbox::default();
        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);

        let events_before = sandbox.events();
        assert!(events_before.is_empty());

        make_transfer(&mut sandbox, RECIPIENT, 1).expect("Failed to make transfer");

        let events_after = sandbox.events();
        assert!(!events_after.is_empty());
        assert!(matches!(
            events_after.last().unwrap().event,
            RuntimeEvent::Balances(_)
        ));
    }

    #[test]
    fn resetting_events() {
        let mut sandbox = MinimalSandbox::default();
        const RECIPIENT: AccountId32 = AccountId32::new([3u8; 32]);

        make_transfer(&mut sandbox, RECIPIENT.clone(), 1).expect("Failed to make transfer");

        assert!(!sandbox.events().is_empty());
        sandbox.reset_events();
        assert!(sandbox.events().is_empty());

        make_transfer(&mut sandbox, RECIPIENT, 1).expect("Failed to make transfer");
        assert!(!sandbox.events().is_empty());
    }
}
