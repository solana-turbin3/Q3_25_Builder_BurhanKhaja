use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolConfig {
    pub pool_id: u16,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}
