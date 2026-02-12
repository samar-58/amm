use anchor_lang::prelude::*;
pub mod states;
pub mod constants;
pub mod instructions;
pub mod errors;

pub use states::*;
pub use instructions::*;
declare_id!("ARVjyJ2xtgrQ8JDfYJi2u7PKCdVXgdcQfstfAjEPYJa5");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
    ctx.accounts.initialize(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64)->Result<()>{
        ctx.accounts.deposit(amount, max_x, max_y)?;
        Ok(())
    }
       pub fn withdraw(ctx: Context<Withdraw>,amount: u64, min_x: u64, min_y: u64)->Result<()>{
       ctx.accounts.withdraw(amount, min_x, min_y)?;
       Ok(())
       }
       pub fn swap(ctx: Context<Swap>, is_x: bool, min: u64, amount: u64)->Result<()>{
        ctx.accounts.swap(is_x, min, amount)?;
      Ok(())
       }
}


