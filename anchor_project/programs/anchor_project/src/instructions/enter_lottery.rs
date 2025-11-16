use crate::errors::LotteryErrors;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;

pub fn _enter_lottery(ctx: Context<EnterLottery>) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let lottery = &mut ctx.accounts.lottery;
    let entry = &mut ctx.accounts.entry;
    let prize_vault = &mut ctx.accounts.prize_vault;

    require!(
        lottery.total_entries < lottery.max_entries,
        LotteryErrors::MaxEntriesReached
    );
    require!(lottery.winner.is_none(), LotteryErrors::WinnerChosen);
    require!(
        lottery.entry_price <= owner.to_account_info().lamports(),
        LotteryErrors::InsufficientBalance
    );

    let ix = transfer(&owner.key(), &prize_vault.key(), lottery.entry_price);
    invoke(
        &ix,
        &[owner.to_account_info(), prize_vault.to_account_info()],
    )?;

    entry.owner = owner.key();
    entry.lottery = lottery.key();
    entry.entry_id = lottery.total_entries;
    entry.bump = ctx.bumps.entry;

    lottery.prize_pot += lottery.entry_price;
    lottery.total_entries += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct EnterLottery<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    /// CHECK: PDA account that holds prize SOL. Validated by seeds in initialize_lottery.
    pub prize_vault: AccountInfo<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Entry::INIT_SPACE,
        seeds = [b"entry", lottery.key().as_ref(), lottery.total_entries.to_le_bytes().as_ref()],
        bump
    )]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}
