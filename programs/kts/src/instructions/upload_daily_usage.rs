use anchor_lang::prelude::*;
use super::super::constants::*;
use super::super::state::{DeviceAccount,DailyUsageAccount};
use super::super::errors::KTSError;

pub fn upload_daily_usage(
    ctx:Context<UploadDailyUsage>,
    timestamp:i64,
    avg_cpu_usage:f32,
    avg_memory_usage:f32,
    top_processes:Vec<String>,
    data_hash:[u8;32],
)->Result<()>{
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(device_hash:[u8;32],timestamp:i64)]
pub struct UploadDailyUsage<'info>{
    #[account(mut)]
    pub user:Signer<'info>,

    #[account(
        seeds=[b"device",device_hash.as_ref()],
        bump,
        constraint=device_account.owner==user.key() @KTSError::InvalidDeviceOwner
    )]
    pub device_account:Account<'info,DeviceAccount>,
    #[account(
        init,
        payer=user,
        space=DAILY_USAGE_ACCOUNT_SIZE,
        seeds=[b"daily_usage",device_hash.as_ref(),&timestamp.to_le_bytes().as_ref()],
        bump
    )]
    pub daily_usage_account:Account<'info,DailyUsageAccount>,
    pub system_program:Program<'info,System>,
}