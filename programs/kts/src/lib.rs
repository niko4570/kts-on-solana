use anchor_lang::prelude::*;

declare_id!("GUMBiEgcTL58Sz7wKoWWQyiUoFisbZ6ELvt2ZyoyEJ5G");

#[program]
pub mod kts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
