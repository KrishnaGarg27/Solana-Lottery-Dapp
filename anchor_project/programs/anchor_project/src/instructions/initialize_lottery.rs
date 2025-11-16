use crate::errors::LotteryErrors;
use crate::states::*;
use anchor_lang::prelude::*;

pub fn _initialize_lottery(
    ctx: Context<InitializeLottery>,
    lottery_id: u8,
    entry_price: u64,
    max_entries: u64,
) -> Result<()> {
    let creator = &mut ctx.accounts.creator;
    let lottery = &mut ctx.accounts.lottery;
    let prize_vault = &mut ctx.accounts.prize_vault;

    require!(entry_price > 0, LotteryErrors::ZeroEntryPrice);
    require!(max_entries > 0, LotteryErrors::ZeroMaxEntries);

    lottery.creator = creator.key();
    lottery.prize_vault = prize_vault.key();
    lottery.lottery_id = lottery_id;
    lottery.prize_pot = 0;
    lottery.entry_price = entry_price;
    lottery.max_entries = max_entries;
    lottery.total_entries = 0;
    lottery.winner = None;
    lottery.bump = ctx.bumps.lottery;

    Ok(())
}

#[derive(Accounts)]
#[instruction(lottery_id: u8)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Lottery::INIT_SPACE,
        seeds = [b"lottery", creator.key().as_ref(), lottery_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,
    #[account(
        init,
        payer = creator,
        space = 0,
        seeds = [b"vault", lottery.key().as_ref()],
        bump,
    )]
    /// CHECK: PDA account that will hold prize SOL. It's created with `init` and
    /// validated by the `seeds` attribute above, so we don't need a typed
    /// `Account<T>` here. The account is owned/checked via seeds and runtime.
    pub prize_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
