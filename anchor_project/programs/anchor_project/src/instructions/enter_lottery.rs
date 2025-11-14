use crate::states::*;
use anchor_lang::prelude::*;

pub fn enter_lottery(ctx: Context<EnterLottery>) -> Result<()> {
    let owner = &mut ctx.accounts.owner;
    let lottery = &mut ctx.accounts.lottery;
    let entry = &mut ctx.accounts.entry;

    // require!(lottery.total_entries < lottery.max_entries)

    entry.owner = owner.key();
    entry.lottery = lottery.key();
    entry.entry_id = lottery.total_entries;
    entry.bump = ctx.bumps.entry;

    lottery.prize += lottery.entry_price;
    lottery.total_entries += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct EnterLottery<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(
        init,
        payer = owner,
        space = 8 + Entry::INIT_SPACE,
        seeds = [b"entry", lottery.key().as_ref(), owner.key().as_ref(), lottery.total_entries.to_le_bytes().as_ref()],
        bump
    )]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}
