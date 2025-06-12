use anchor_lang::prelude::*;

declare_id!("3QdPomKPoQwsFCRohMmQfsHd1LhoTN9ZrwwnsxbeFXnP");

#[program]
pub mod cassegrain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
