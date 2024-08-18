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

#derive(Accounts)
pub struct create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Calculator{
    pub greeting: String, 
    pub result: i64,
    pub remainder: i64
}