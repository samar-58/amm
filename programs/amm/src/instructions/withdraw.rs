use crate::{Config, errors::AmmError, transfer_from_pda};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, BurnChecked,burn_checked}
};
use constant_product_curve::ConstantProduct;

#[derive(Accounts)]
pub struct Withdraw<'info> {
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
#[account(mut)]
pub user_x: InterfaceAccount<'info, TokenAccount>,
#[account(mut)]
pub user_y: InterfaceAccount<'info, TokenAccount>,
#[account(
mut,
associated_token::mint = mint_y,
associated_token::authority = config,
associated_token::token_program = token_program
)]
pub mint_y_vault: InterfaceAccount<'info, TokenAccount>,
#[account(
mut,
seeds = [b"lp",config.key().as_ref()],
bump = config.lp_bump,
)]
pub lp_token: InterfaceAccount<'info, Mint>,
#[account(
mut,
associated_token::mint=lp_token,
associated_token::authority = signer,
)]
pub user_ata_lp: InterfaceAccount<'info, TokenAccount>,
pub token_program: Interface<'info, TokenInterface>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
}

impl <'info> Withdraw <'info>{
pub fn withdraw(&mut self, amount: u64, min_x: u64, min_y: u64)->Result<()>{
     require!(!self.config.locked, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);
        require!(
            self.user_ata_lp.amount >= amount,
            AmmError::Insufficientbalance
        );
        
        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.mint_x_vault.amount,
            self.mint_y_vault.amount,
            self.lp_token.supply,
            amount,
            6
        ).map_err(|_| AmmError::CurveError)?;
        
        require!(
            amounts.x >= min_x && amounts.y >= min_y,
            AmmError::SlippageExceeded
        );
        
        self.burn(amount)?;
       //withdraw x tokens
        transfer_from_pda(&self.mint_x_vault, &self.user_x, &amounts.x, &self.mint_x, &self.token_program, &self.config)?;
       //withdraw y tokens
        transfer_from_pda(&self.mint_y_vault, &self.user_y, &amounts.y, &self.mint_y, &self.token_program, &self.config)?;
    Ok(())
}

fn burn(
&self,
amount: u64
)->Result<()>{
let accounts = BurnChecked{
mint: self.lp_token.to_account_info(),
authority: self.signer.to_account_info(),
from: self.user_ata_lp.to_account_info()
};

let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

burn_checked(ctx, amount,self.lp_token.decimals)?;
Ok(())
}
}
