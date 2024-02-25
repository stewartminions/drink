//! Balance API for the sandbox.
use frame_support::{sp_runtime::DispatchError, traits::fungible::Mutate};

use super::Sandbox;
use crate::{runtime::AccountIdFor, BalanceOf};

/// Balance API for the sandbox.
pub trait BalanceAPI<T: Sandbox>
where
    T: Sandbox,
    T::Runtime: pallet_balances::Config,
{
    /// Mint tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    fn mint_into(
        &mut self,
        address: &AccountIdFor<T::Runtime>,
        amount: BalanceOf<T::Runtime>,
    ) -> Result<BalanceOf<T::Runtime>, DispatchError>;

    /// Return the free balance of an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    fn free_balance(&mut self, address: &AccountIdFor<T::Runtime>) -> BalanceOf<T::Runtime>;
}

impl<T> BalanceAPI<T> for T
where
    T: Sandbox,
    T::Runtime: pallet_balances::Config,
{
    fn mint_into(
        &mut self,
        address: &AccountIdFor<T::Runtime>,
        amount: BalanceOf<T::Runtime>,
    ) -> Result<BalanceOf<T::Runtime>, DispatchError> {
        self.execute_with(|| pallet_balances::Pallet::<T::Runtime>::mint_into(address, amount))
    }

    fn free_balance(&mut self, address: &AccountIdFor<T::Runtime>) -> BalanceOf<T::Runtime> {
        self.execute_with(|| pallet_balances::Pallet::<T::Runtime>::free_balance(address))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::MinimalSandbox;
    #[test]
    fn mint_works() {
        let mut sandbox = MinimalSandbox::default();
        let balance = sandbox.free_balance(&MinimalSandbox::default_actor());

        sandbox
            .mint_into(&MinimalSandbox::default_actor(), 100)
            .unwrap();

        assert_eq!(
            sandbox.free_balance(&MinimalSandbox::default_actor()),
            balance + 100
        );
    }
}
