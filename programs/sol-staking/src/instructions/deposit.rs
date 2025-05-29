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
