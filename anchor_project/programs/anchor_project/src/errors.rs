use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryErrors {
    #[msg("Entry Price cannot be zero")]
    ZeroEntryPrice,
    #[msg("Max Entries cannot be zero")]
    ZeroMaxEntries,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
    #[msg("Max Entries has been reached")]
    MaxEntriesReached,
    #[msg("Winner has already been chosen")]
    WinnerChosen,
    #[msg("No entries to choose from")]
    NoEntries,
    #[msg("Winner has not been chosen")]
    WinnerNotChosen,
    #[msg("The instruction does not match the Lottery Winner")]
    WinnerMismatch,
    #[msg("Prize has already been claimed")]
    PrizeClaimed,
}
