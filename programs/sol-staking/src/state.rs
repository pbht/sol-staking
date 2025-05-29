use anchor_lang::prelude::*;

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked: u64,
    pub bump: u8,
}