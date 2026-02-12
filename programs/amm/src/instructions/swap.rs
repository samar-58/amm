use crate::{Config, errors::AmmError, transfer_from_pda, transfer_tokens};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
pub signer: Signer<'info>,
pub mint_y: Box<InterfaceAccount<'info, Mint>>,
pub mint_x: Box<InterfaceAccount<'info, Mint>>,
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
pub mint_x_vault: Box<InterfaceAccount<'info, TokenAccount>>,
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
pub mint_y_vault: Box<InterfaceAccount<'info, TokenAccount>>,
#[account(
mut,
seeds = [b"lp",config.key().as_ref()],
bump = config.lp_bump,
)]
pub lp_token: Box<InterfaceAccount<'info, Mint>>,
pub token_program: Interface<'info, TokenInterface>,
pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
}

impl <'info> Swap <'info>{
pub fn swap(&self, is_x: bool, min: u64, amount: u64)->Result<()>{
require!(self.config.locked == false, AmmError::PoolLocked);
require!(amount != 0, AmmError::InvalidAmount);

let mut curve = ConstantProduct::init(self.mint_x_vault.amount, self.mint_y_vault.amount, self.lp_token.supply, self.config.fee, None).map_err(|_| AmmError::CurveError)?;

let liquidity_pair = match is_x {
    true => LiquidityPair::X,
    false => LiquidityPair::Y
};
let swap = curve.swap(liquidity_pair, amount, min).map_err(|_| AmmError::CurveError)?;

require!(swap.deposit !=0 || swap.withdraw != 0, AmmError::InvalidAmount);

// deposit token logic
if is_x{
    transfer_tokens(&self.user_x, &self.mint_x_vault, &swap.deposit, &self.mint_x, &self.signer, &self.token_program)?;
}
else {
      transfer_tokens(&self.user_y, &self.mint_y_vault, &swap.deposit, &self.mint_y, &self.signer, &self.token_program)?; 
}
//withdraw logic
let (from, to, mint) = match is_x{
true=>(&self.mint_x_vault, &self.user_x, &self.mint_x),
false=>(&self.mint_y_vault, &self.user_y, &self.mint_y),
};

transfer_from_pda(from, to, &amount, mint, &self.token_program, &self.config)?;
    Ok(())
}

}
