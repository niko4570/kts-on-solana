use anchor_lang::prelude::*;
use super::super::constants::*;
use super::super::errors::KTSError;
use super::super::state::DeviceAccount;

/// Registers a new device in the system.
/// 
/// This function creates a new DeviceAccount with the provided device hash and name.
/// It ensures the device name length is valid and initializes the account fields.
/// 
/// # Arguments
/// * `ctx` - The context containing the accounts.
/// * `device_hash` - A 32-byte hash representing the device.
/// * `device_name` - The name of the device, must be <= 64 characters.
/// 
/// # Returns
/// * `Result<()>` - Ok if successful, or an error if validation fails.
pub fn register_device(
    ctx: Context<RegisterDevice>,
    device_hash:[u8;32],
    device_name:String,
)->Result<()>{
    // Validate the device name length to prevent invalid inputs.
    require!(device_name.len()<=MAX_DEVICE_NAME_LENGTH,KTSError::InvalidDeviceNameLength);
    
    // Access the device account to initialize it.
    let device_account=&mut ctx.accounts.device_account;
    
    // Set the owner to the signer (user).
    device_account.owner=*ctx.accounts.user.key;
    
    // Set the device hash.
    device_account.device_hash=device_hash;
    
    // Record the registration timestamp.
    device_account.registered_at=Clock::get()?.unix_timestamp;
    
    // Initialize NFT minted status to false.
    device_account.nft_minted=false;
    
    Ok(())
}

/// Accounts required for the RegisterDevice instruction.
/// 
/// This struct defines the accounts needed to register a device,
/// including the user signer, the device account to be created,
/// and the system program.
#[derive(Accounts)]
#[instruction(device_hash:[u8;32])]
pub struct RegisterDevice<'info>{  
    /// The user signing the transaction, who will own the device.
    #[account(mut)]
    pub user:Signer<'info>,

    /// The device account to be initialized with PDA seeds.
    #[account(
        init,
        payer=user,
        space=DEVICE_ACCOUNT_SIZE,
        seeds=[b"device",device_hash.as_ref()],
        bump,
    )]
    pub device_account:Account<'info,DeviceAccount>,
    
    /// The system program for account creation.
    pub system_program:Program<'info,System>,

}