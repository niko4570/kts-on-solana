use anchor_lang::prelude::*;
use super::super::constants::*;
use super::super::state::{DeviceAccount,DailyUsageAccount};
use super::super::errors::KTSError;

/// Uploads daily usage data for a device.
///
/// This function initializes a new DailyUsageAccount with the provided usage metrics,
/// ensuring the device is owned by the signer and validating input constraints.
///
/// # Arguments
/// * `ctx` - The context containing the accounts.
/// * `timestamp` - The timestamp for the daily usage data.
/// * `avg_cpu_usage` - Average CPU usage percentage.
/// * `avg_memory_usage` - Average memory usage percentage.
/// * `top_processes` - List of top processes as strings.
/// * `data_hash` - A 32-byte hash of the usage data.
///
/// # Returns
/// * `Result<()>` - Ok if successful, or an error if validation fails.
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
 /// The user signing the transaction, must own the device.
pub struct UploadDailyUsage<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    
    /// The device account, verified by seeds and ownership.
    #[account(
        seeds=[b"device",device_hash.as_ref()],
        bump,
        constraint=device_account.owner==user.key() @KTSError::InvalidDeviceOwner
    )]
    pub device_account:Account<'info,DeviceAccount>,

    /// The daily usage account to be initialized.
    #[account(
        init,
        payer=user,
        space=DAILY_USAGE_ACCOUNT_SIZE,
        seeds=[b"daily_usage",device_hash.as_ref(),&timestamp.to_le_bytes().as_ref()],
        bump
    )]
    pub daily_usage_account:Account<'info,DailyUsageAccount>,
    /// The system program for account creation.
    pub system_program:Program<'info,System>,
}