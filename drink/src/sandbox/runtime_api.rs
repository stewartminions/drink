//! Basic chain API.

use frame_support::sp_runtime::traits::Saturating;
use frame_system::pallet_prelude::BlockNumberFor;

use super::Sandbox;
use crate::{DrinkResult, Error};

impl<Config: crate::SandboxConfig> Sandbox<Config> {
    /// Build a new empty block and return the new height.
    pub fn build_block(&mut self) -> DrinkResult<BlockNumberFor<Config::Runtime>> {
        self.execute_with(|| {
            let mut current_block = frame_system::Pallet::<Config::Runtime>::block_number();
            let block_hash = Config::finalize_block(current_block).map_err(Error::BlockFinalize)?;
            current_block.saturating_inc();
            Config::initialize_block(current_block, block_hash);
            Ok(current_block)
        })
    }
    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    pub fn build_blocks(&mut self, n: u32) -> DrinkResult<BlockNumberFor<Config::Runtime>> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block()?);
        }
        Ok(last_block.unwrap_or_else(|| self.block_number()))
    }
}
