use anchor_lang::prelude::*;

declare_id!("AuZ2yuo5wAr4Lzz9gRq6DJepjHuTK88HKW5NaUZBzNh4");

#[program]
pub mod anchor_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
