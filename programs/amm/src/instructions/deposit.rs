use crate::{errors::AmmError, transfer_tokens, Config};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};
use constant_product_curve::ConstantProduct;
use num_integer::Roots;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,
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
    #[account(
init_if_needed,
payer = signer,
associated_token::mint=lp_token,
associated_token::authority = signer,
)]
    pub user_ata_lp: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        require!(!self.config.locked, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);
        require!(max_x > 0 && max_y > 0, AmmError::InvalidAmount);

        let (x_amount, y_amount, lp_to_mint) =
            self.calculate_deposit_amounts(amount, max_x, max_y)?;

        require!(
            x_amount <= max_x && y_amount <= max_y,
            AmmError::SlippageExceeded
        );
        //for depositing mint_x in the vault
        transfer_tokens(
            &self.user_x,
            &self.mint_x_vault,
            &x_amount,
            &self.mint_x,
            &self.signer,
            &self.token_program,
        )?;
        //for depositing mint_y in the vault
        transfer_tokens(
            &self.user_y,
            &self.mint_y_vault,
            &y_amount,
            &self.mint_y,
            &self.signer,
            &self.token_program,
        )?;

        //mint lp tokens to the user
        self.mint_lp_tokens(lp_to_mint)?;

        msg!(
            "Deposited {} X and {} Y, minted {} LP",
            x_amount,
            y_amount,
            lp_to_mint
        );
        Ok(())
    }
    fn calculate_deposit_amounts(
        &self,
        lp_amount: u64,
        max_x: u64,
        max_y: u64,
    ) -> Result<(u64, u64, u64)> {
        let is_first_deposit = self.lp_token.supply == 0;

        if is_first_deposit {
            // first depositor sets the initial price ratio
            // they deposit max_x and max_y, receive lp_amount tokens
            let lp = (max_x as u128)
                .checked_mul(max_y as u128)
                .ok_or(AmmError::Overflow)?
                .sqrt() as u64;
            Ok((max_x, max_y, lp))
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

            Ok((amounts.x, amounts.y, lp_amount))
        }
    }
    fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        let mint_account = MintTo {
            mint: self.lp_token.to_account_info(),
            to: self.user_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let config_seeds = self.config.seed.to_le_bytes();

        let seeds: &[&[u8]] = &[b"config", config_seeds.as_ref(), &[self.config.bump]];

        let signer_seeds = &[seeds];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_account,
            signer_seeds,
        );

        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
