use anchor_lang::prelude::*;

#[account]
pub struct BetAccount {
    pub user_account: Pubkey,
    pub token_account: Pubkey, // Added meme coin field
    pub amount: u64,
    pub prediction: bool, // true for higher, false for lower
    pub price_at_bet: u64,
    pub end_time: u64,
}
