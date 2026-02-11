use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, MintTo, mint_to},
};
use constant_product_curve::ConstantProduct;

use crate::{Config, errors::AmmError, transfer_tokens};

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
    pub fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        require!(!self.config.locked, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);
        require!(max_x > 0 && max_y > 0, AmmError::InvalidAmount);

        let (x_amount, y_amount) = self.calculate_deposit_amounts(amount, max_x, max_y)?;

        require!(
            x_amount <= max_x && y_amount <= max_y,
            AmmError::SlippageExceeded
        );
//for depositing mint_x in the vault
    transfer_tokens(&self.signer, &self.mint_x_vault, &x_amount, &self.mint_x, &self.signer, &self.token_program)?;
//for depositing mint_y in the vault
    transfer_tokens(&self.signer, &self.mint_y_vault, &y_amount, &self.mint_y, &self.signer, &self.token_program)?;

        self.mint_lp_tokens(amount)?;

        msg!("Deposited {} X and {} Y, minted {} LP", x_amount, y_amount, amount);
        Ok(())
    }
    fn calculate_deposit_amounts(&self, lp_amount: u64, max_x: u64, max_y: u64) -> Result<(u64, u64)> {
        let is_first_deposit = self.lp_token.supply == 0;

        if is_first_deposit {
            // first depositor sets the initial price ratio
            // they deposit max_x and max_y, receive lp_amount tokens
            Ok((max_x, max_y))
        } else {
            // later all deposits must maintain pool ratio
            let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                self.mint_x_vault.amount,
                self.mint_y_vault.amount,
                self.lp_token.supply,
                lp_amount,
                6,
            )
            .map_err(|_| AmmError::CurveError)?;

            Ok((amounts.x, amounts.y))
        }
    }
    fn mint_lp_tokens(&mut self, amount: u64)->Result<()>{
let mint_account = MintTo{
    mint: self.lp_token.to_account_info(),
    to: self.signer.to_account_info(),
    authority: self.config.to_account_info()
};

let config_seeds = self.config.seed.to_le_bytes();

let seeds:&[&[u8]] = &[
b"config",
config_seeds.as_ref(),
&[self.config.bump]
];

let signer_seeds = &[seeds];

let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), mint_account, signer_seeds);

mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
