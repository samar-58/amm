use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Config;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
mut,
seeds = [b"config", config.seed.to_le_bytes().as_ref()],
bump = config.bump
)]
    pub config: Account<'info, Config>,
    #[account(
mut,
associated_token::mint = mint_x,
associated_token::authority = config,
associated_token::token_program = token_program
)]
    pub mint_x_vault: InterfaceAccount<'info, TokenAccount>,

    pub user_x: InterfaceAccount<'info, TokenAccount>,
    pub user_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
mut,
associated_token::mint = mint_y,
associated_token::authority = config,
associated_token::token_program = token_program
)]
    pub mint_y_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
seeds = [b"lp",config.key().as_ref()],
bump = config.lp_bump,
)]
    pub lp_token: InterfaceAccount<'info, Mint>,
    #[account(
init_if_needed,
payer = signer,
associated_token::mint=lp_token,
associated_token::authority = signer,
)]
    pub user_ata_lp: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit() -> Result<()> {
        Ok(())
    }
}
