use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;
declare_id!("GUMBiEgcTL58Sz7wKoWWQyiUoFisbZ6ELvt2ZyoyEJ5G");


#[program]

pub mod kts {
    use super::*;

}