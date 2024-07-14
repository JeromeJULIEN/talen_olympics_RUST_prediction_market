use anchor_lang::prelude::*;

#[error_code]
pub enum BetError {
    #[msg("Not enough SOL")]
    NotEnoughSol,
    #[msg("Bet not close yet")]
    BetNotClose,
}
