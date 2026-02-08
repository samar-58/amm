use anchor_lang::prelude::*;
pub mod states;
pub mod constants;
pub mod instructions;

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
}


