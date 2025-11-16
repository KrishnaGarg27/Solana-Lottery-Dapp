use anchor_lang::prelude::*;

use crate::errors::LotteryErrors;
use crate::states::*;

pub fn _claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let winner_entry = &mut ctx.accounts.winner_entry;
    let lottery = &mut ctx.accounts.lottery;
    let prize_vault = &mut ctx.accounts.prize_vault;

    require!(lottery.winner.is_some(), LotteryErrors::WinnerNotChosen);
    require!(
        winner_entry.key() == lottery.winner.unwrap(),
        LotteryErrors::WinnerMismatch
    );
    require!(
        winner_entry.owner == signer.key(),
        LotteryErrors::WinnerMismatch
    );
    require!(
        lottery.prize_pot <= prize_vault.to_account_info().lamports(),
        LotteryErrors::PrizeClaimed
    );

    let prize = lottery.prize_pot;
    **signer.to_account_info().try_borrow_mut_lamports()? += prize;
    **prize_vault.to_account_info().try_borrow_mut_lamports()? -= prize;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub winner_entry: Account<'info, Entry>,
    #[account()]
    pub lottery: Account<'info, Lottery>,
    #[account(mut, seeds = [b"vault", lottery.key().as_ref()],
        bump,)]
    /// CHECK: PDA account that holds the prize SOL.
    /// It's validated by the `seeds` attribute above, so we don't need a typed
    /// `Account<T>` here. The account is owned/checked via seeds and runtime.
    pub prize_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
