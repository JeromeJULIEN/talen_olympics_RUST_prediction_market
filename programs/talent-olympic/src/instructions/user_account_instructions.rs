use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::state::UserAccount;

pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
    let new_account = &mut ctx.accounts.new_account;
    new_account.authority = ctx.accounts.signer.key();
    new_account.balance = 0;

    Ok(())
}

/**
@dev: deposit funds to user aocount
@requierement : only callable by user_account authority
 */
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.user_account.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, amount)?;

    let user_account = &mut ctx.accounts.user_account;
    user_account.balance += amount;

    msg!("Deposited {} lamports into the vault", amount);

    Ok(())
}

/**
@dev: withraw funds to user aocount
@requierement : only callable by user_account authority
 */
pub fn withdraw(ctx: Context<Withraw>, amount: u64) -> Result<()> {
    **ctx
        .accounts
        .user_account
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx
        .accounts
        .authority
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;

    msg!("Withdrew {} lamports from the vault", amount);

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8,seeds=[b"user",signer.key().as_ref()],bump)]
    pub new_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one =authority)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one =authority)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}
