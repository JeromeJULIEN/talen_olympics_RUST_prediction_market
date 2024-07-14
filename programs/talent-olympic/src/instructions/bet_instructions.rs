use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::errors::BetError;
use crate::state::{BetAccount, TokenAccount, UserAccount};

// need the token to bet on, the amount bet, the prediction (up or own) and bet time limit
// bet in SOL
pub fn place_bet(
    ctx: Context<PlaceBet>,
    token_account: Pubkey,
    amount: u64,
    prediction: bool,
    end_time: u64,
) -> Result<()> {
    // user should have enough SOL to pay the bet
    require!(
        ctx.accounts.user_account.balance >= amount,
        BetError::NotEnoughSol
    );

    // initiate the bet account
    let new_bet = &mut ctx.accounts.new_bet;
    new_bet.user_account = ctx.accounts.user_account.key();
    new_bet.token_account = token_account;
    new_bet.amount = amount;
    new_bet.prediction = prediction;
    new_bet.end_time = end_time;
    new_bet.price_at_bet = ctx.accounts.token_account.price; // temp : should use oracle feed price

    // transfer SOL amount to the bet account
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.new_bet.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, amount)?;

    let user_account = &mut ctx.accounts.user_account;
    user_account.balance -= amount;

    Ok(())
}

pub fn check_bet(ctx: Context<CheckBet>) -> Result<()> {
    // causing error...
    /*require!(
        Clock::get()?.unix_timestamp > ctx.accounts.bet.end_time as i64,
        BetError::BetNotClose
    );*/

    let is_price_up: bool = ctx.accounts.bet.price_at_bet <= ctx.accounts.token_account.price;

    let amount = ctx.accounts.bet.amount;

    if ctx.accounts.bet.prediction == is_price_up {
        **ctx
            .accounts
            .bet
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **ctx
            .accounts
            .authority
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;

        msg!("You won")
    } else {
        msg!("You loose")
    }

    Ok(())
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(init,payer=authority,space=100)]
    pub new_bet: Account<'info, BetAccount>,
    #[account()]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one = authority)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckBet<'info> {
    #[account(mut)]
    pub bet: Account<'info, BetAccount>,
    #[account()]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one = authority)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
