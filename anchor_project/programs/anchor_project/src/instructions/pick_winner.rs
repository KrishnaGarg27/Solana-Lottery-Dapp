use anchor_lang::prelude::*;

use crate::errors::LotteryErrors;
use crate::states::*;

pub fn _pick_winner(ctx: Context<PickWinner>, lottery_id: u8) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;

    require!(lottery.total_entries > 0, LotteryErrors::NoEntries);
    require!(lottery.winner.is_none(), LotteryErrors::WinnerChosen);

    let winner_index = (Clock::get()?.unix_timestamp as u64) % lottery.total_entries;

    let (entry_pda, _bump) = Pubkey::find_program_address(
        &[
            b"entry",
            lottery.key().as_ref(),
            winner_index.to_le_bytes().as_ref(),
        ],
        &ctx.program_id,
    );

    lottery.winner = Some(entry_pda);

    Ok(())
}

#[derive(Accounts)]
#[instruction(lottery_id: u8)]
pub struct PickWinner<'info> {
    #[account()]
    pub creator: Signer<'info>,
    #[account(mut, seeds = [b"lottery", creator.key().as_ref(), lottery_id.to_le_bytes().as_ref()], bump)]
    pub lottery: Account<'info, Lottery>,
}
