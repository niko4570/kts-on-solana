use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("GUMBiEgcTL58Sz7wKoWWQyiUoFisbZ6ELvt2ZyoyEJ5G");

use instructions::*;
use state::*;

#[program]

pub mod kts {
    use super::*;
    pub fn register_device(
        ctx:Context<RegisterDevice>,
        device_hash:[u8;32],
        device_name:String,
    ) ->Result<()>{
        instructions::register_device(ctx,device_hash,device_name)
    }
}