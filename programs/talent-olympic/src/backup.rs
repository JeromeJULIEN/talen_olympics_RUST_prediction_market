use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GBf2Gk5ukKg11u741UgNZ2nCSbmp2eVPjgXCr98wHMRm");

#[program]
pub mod prediction_market {
    use super::*;
    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
        new_account.authority = ctx.accounts.signer.key();
        new_account.balance = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8,seeds=[b"user",signer.key().as_ref()],bump)]
    pub new_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,
    pub balance: u64,
}
