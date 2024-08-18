use anchor_lang::prelude::*;

declare_id!("8wXnLTgDSWnFQrhQ3DRqpUbnSR5Fpp9KpyscRcf1c1kU");

#[program]
pub mod calculatordapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
}