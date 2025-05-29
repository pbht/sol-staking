use anchor_lang::prelude::*;
use crate::state::StakeAccount;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct DepositSol<'info> {
    #[account(
        init_if_needed,
        seeds = [b"stake", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 8 + 1
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: PDA vault
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>

}

pub fn handler(ctx: Context<DepositSol>, bump: u8, amount: u64) -> Result<()> {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.vault.key(),
        amount
    );

    anchor_lang::solana_program::program::invoke(
        &ix, 
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info()
        ]
    )?;

    let stake = &mut ctx.accounts.stake_account;
    stake.owner = ctx.accounts.user.key();
    stake.staked += amount;
    stake.bump = bump;        

    Ok(())
}