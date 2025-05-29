use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("CgmvUGyVzKhKmMEDM5Hwd3T9ucUXAKTePZYYscYcpjsQ");

#[program]
pub mod sol_staking {
    use super::*;

    pub fn deposit_sol(ctx: Context<DepositSol>, bump: u8, amount: u64) -> Result<()> {
        deposit::handler(ctx, bump, amount)?;
        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, bump: u8, amount: u64) -> Result<()> {
        withdraw::handler(ctx, bump, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
