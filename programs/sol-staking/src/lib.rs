use anchor_lang::prelude::*;

declare_id!("CgmvUGyVzKhKmMEDM5Hwd3T9ucUXAKTePZYYscYcpjsQ");

#[program]
pub mod sol_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
