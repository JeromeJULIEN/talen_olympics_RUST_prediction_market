use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("9rsW6mMnXv9riwGuuZTkAHLPR7Ui3kMKvqBnPr8m1Xif");

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod prediction_market {
    use super::*;

    /// USER MANAGEMENT
    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        instructions::create_user(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    /// TOKEN MANAGEMENT
    pub fn add_token(ctx: Context<AddToken>, ticker: String, price: u64) -> Result<()> {
        instructions::add_token(ctx, ticker, price)
    }

    /// should be updated by oracle
    pub fn update_token_price(ctx: Context<UpdateTokenPrice>, new_price: u64) -> Result<()> {
        instructions::update_token_price(ctx, new_price)
    }

    /// BET MANAGEMENT
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        token_account: Pubkey,
        amount: u64,
        prediction: bool,
        end_time: u64,
    ) -> Result<()> {
        instructions::place_bet(ctx, token_account, amount, prediction, end_time)
    }

    pub fn check_bet(ctx: Context<CheckBet>) -> Result<()> {
        instructions::check_bet(ctx)
    }
}
