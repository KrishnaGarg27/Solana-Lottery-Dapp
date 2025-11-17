use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("GCoAnH3cGsUAz9vFChaujX9LpBien56N7PTaDuvgHex7");

#[program]
pub mod anchor_project {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeLottery>,
        lottery_id: u8,
        entry_price: u64,
        max_entries: u64,
    ) -> Result<()> {
        _initialize_lottery(ctx, lottery_id, entry_price, max_entries)
    }

    pub fn enter_lottery(ctx: Context<EnterLottery>) -> Result<()> {
        _enter_lottery(ctx)
    }

    pub fn pick_winner(ctx: Context<PickWinner>, lottery_id: u8) -> Result<()> {
        _pick_winner(ctx, lottery_id)
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        _claim_prize(ctx)
    }
}
