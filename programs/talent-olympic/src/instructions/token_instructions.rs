use anchor_lang::prelude::*;
use crate::state::TokenAccount;

pub fn add_token(ctx: Context<AddToken>, ticker: String, price: u64) -> Result<()> {
    let new_coin = &mut ctx.accounts.new_coin;
    new_coin.ticker = ticker;
    new_coin.price = price;

    Ok(())
}

pub fn update_token_price(ctx: Context<UpdateTokenPrice>, new_price: u64) -> Result<()> {
    let coin_account = &mut ctx.accounts.coin_account;
    coin_account.price = new_price;

    Ok(())
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct AddToken<'info> {
    #[account(init,payer = signer,space = 8 + 16 + 8, seeds=[b"token",ticker.as_bytes()],bump)]
    pub new_coin: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTokenPrice<'info> {
    #[account(mut)]
    pub coin_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
