use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;

declare_id!("DqKkMRaPxD9H6RHucpzjDgwj7FUZXKJXb7xx778TnSM8");

#[program]
pub mod anchor_project {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeLottery>,
        lottery_id: u8,
        entry_price: u64,
        max_entries: u64,
    ) -> Result<()> {
        initialize_lottery(ctx, lottery_id, entry_price, max_entries)
    }

    pub fn enter_lottery(ctx: Context<EnterLottery>) -> Result<()> {
        enter_lottery(ctx)
    }
}
