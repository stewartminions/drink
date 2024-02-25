//! Basic chain API.

use frame_support::sp_runtime::traits::Saturating;
use frame_system::pallet_prelude::BlockNumberFor;

use super::{system_api::SystemAPI, Sandbox};

/// Runtime API for the sandbox.
pub trait RuntimeAPI<T: Sandbox> {
    /// Build a new empty block and return the new height.
    fn build_block(&mut self) -> BlockNumberFor<T::Runtime>;

    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    fn build_blocks(&mut self, n: u32) -> BlockNumberFor<T::Runtime>;
}

impl<T> RuntimeAPI<T> for T
where
    T: Sandbox,
    T::Runtime: frame_system::Config,
{
    fn build_block(&mut self) -> BlockNumberFor<T::Runtime> {
        self.execute_with(|| {
            let mut current_block = frame_system::Pallet::<T::Runtime>::block_number();
            let block_hash = T::finalize_block(current_block);
            current_block.saturating_inc();
            T::initialize_block(current_block, block_hash);
            current_block
        })
    }

    fn build_blocks(&mut self, n: u32) -> BlockNumberFor<T::Runtime> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block());
        }
        last_block.unwrap_or_else(|| self.block_number())
    }
}
