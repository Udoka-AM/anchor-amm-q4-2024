use anchor_lang::prelude::*;

mod errors;
mod state;
mod instructions;

use instructions::*;

declare_id!("7HhA6WBHpugdJGm8svXpDtUsfWR87G5Vt7XihJAjc6MT");

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
