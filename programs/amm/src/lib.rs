use anchor_lang::prelude::*;

declare_id!("ARVjyJ2xtgrQ8JDfYJi2u7PKCdVXgdcQfstfAjEPYJa5");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
