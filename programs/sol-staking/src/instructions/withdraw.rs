use anchor_lang::prelude::*;
use crate::state::StakeAccount;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct WithdrawSol<'info> {
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

pub fn handler(ctx: Context<WithdrawSol>, bump: u8, amount: u64) -> Result<()> {
    let stake = &mut ctx.accounts.stake_account;

    require!(stake.staked >= amount, ErrorCode::InsufficientFunds);
    stake.staked -= amount;

    let ix  = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.vault.key(),
        &ctx.accounts.user.key(),
        amount
    );

    let seeds: &[&[u8]] = &[b"vault", &[bump]];
    anchor_lang::solana_program::program::invoke_signed(
        &ix, 
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.user.to_account_info()
        ], 
        &[seeds]
    )?;

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Can't withdraw more SOL than is staked.")]
    InsufficientFunds
}