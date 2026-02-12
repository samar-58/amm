use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Config{
pub seed: u64,
pub authority: Option<Pubkey>,// optional autority of this pool
pub mint_x: Pubkey, // mint address of token x
pub mint_y: Pubkey, // mint address of token y
pub fee: u16,
pub locked: bool,
pub bump: u8, // config bump
pub lp_bump: u8 // lp token bump
}