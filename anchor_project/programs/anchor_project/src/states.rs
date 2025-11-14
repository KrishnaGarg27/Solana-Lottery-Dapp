use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Lottery {
    pub creator: Pubkey,
    pub lottery_id: u8,
    pub prize: u64,
    pub entry_price: u64,
    pub max_entries: u64,
    pub total_entries: u64,
    pub winner_chosen: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Entry {
    pub owner: Pubkey,
    pub lottery: Pubkey,
    pub entry_id: u64,
    pub bump: u8,
}
