use anchor_lang::prelude::*;
use super::super::constants::*;
use super::super::errors::ktsError;
use super::super::state::DeviceAccount;

pub fn register_device(
    ctx: Context<RegisterDevice>,
    device_hash:[u8;32],
    device_name:String,
)->Result<()>{
    
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterDevice{
    
}