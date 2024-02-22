//! Basic chain API.

use frame_support::sp_runtime::traits::Saturating;
use frame_system::pallet_prelude::BlockNumberFor;

use super::Sandbox;

impl<Config: crate::SandboxConfig> Sandbox<Config> {
    /// Build a new empty block and return the new height.
    pub fn build_block(&self) -> BlockNumberFor<Config::Runtime> {
        self.execute_with(|| {
            let mut current_block = frame_system::Pallet::<Config::Runtime>::block_number();
            let block_hash = Config::finalize_block(current_block);
            current_block.saturating_inc();
            Config::initialize_block(current_block, block_hash);
            current_block
        })
    }
    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    pub fn build_blocks(&self, n: u32) -> BlockNumberFor<Config::Runtime> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block());
        }
        last_block.unwrap_or_else(|| self.block_number())
    }
}
