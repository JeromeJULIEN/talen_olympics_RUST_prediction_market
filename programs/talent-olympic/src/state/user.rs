use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub balance: u64,
}
