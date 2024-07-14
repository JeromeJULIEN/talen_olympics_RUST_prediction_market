use anchor_lang::prelude::*;

#[account]
pub struct TokenAccount {
    pub ticker: String,
    pub price: u64, // should come from oracle prince feed
}
