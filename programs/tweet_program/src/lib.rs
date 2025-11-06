use anchor_lang::prelude::*;

declare_id!("FEmrsw174xJ5ySpfLsXYBL3bbmSbovd6dByp3239zpMT");

#[program]
pub mod tweet_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
