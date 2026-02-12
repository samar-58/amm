use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::states::Config;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
init,
payer = signer,
space = 8 + Config::INIT_SPACE,
seeds = [b"config", seed.to_le_bytes().as_ref()],
bump
)]
    pub config: Account<'info, Config>,
    #[account(
init,
payer = signer,
associated_token::mint = mint_x,
associated_token::authority = config,
associated_token::token_program = token_program
)]
    pub mint_x_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
init,
payer = signer,
associated_token::mint = mint_y,
associated_token::authority = config,
associated_token::token_program = token_program
)]
    pub mint_y_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
init,
payer = signer,
seeds = [b"lp",config.key().as_ref()],
bump,
mint::decimals = 6,
mint::authority = config,
mint::token_program = token_program
)]
    pub lp_token: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
        bump: &InitializeBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            bump: bump.config,
            lp_bump: bump.lp_token,
            locked: false,
        });
        Ok(())
    }
}
