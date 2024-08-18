use anchor_lang::prelude::*;

declare_id!("8wXnLTgDSWnFQrhQ3DRqpUbnSR5Fpp9KpyscRcf1c1kU");

#[program]
pub mod calculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
