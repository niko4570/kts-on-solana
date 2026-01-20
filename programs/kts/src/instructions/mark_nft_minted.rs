use anchor_lang::prelude::*;
use super::super::errors::KTSError;
use super::super::state::DeviceAccount;

/// Marks an NFT as minted for a registered device.
///
/// This function sets the nft_minted flag to true on the DeviceAccount.
/// It ensures the caller is the device owner and that the NFT hasn't been minted yet.
///
/// # Arguments
/// * `ctx` - The context containing the accounts.
///
/// # Returns
/// * `Result<()>` - Ok if successful, or an error if validation fails.
pub fn mark_nft_minted(ctx:Context<MarkNftMinted>)->Result<()>{
    let device=&mut ctx.accounts.device_account;
    
    require_keys_eq!(
        device.owner,
        ctx.accounts.user.key(),
        KTSError::InvalidDeviceOwner
    );
    require!(
        !device.nft_minted,
        KTSError::NftAlreadyMinted
    );
    device.nft_minted=true;
    msg!(
        "NFT marked as for device: {} by owner {}",
        device.device_hash.as_ref().to_vec().iter().map(|b| format!("{:02x}", b)).collect::<String>(),
        ctx.accounts.user.key(),
    );
    Ok(())
}

/// Accounts required for the MarkNftMinted instruction.
///
/// This struct defines the accounts needed to mark an NFT as minted,
/// including the device owner signer and the device account.
#[derive(Accounts)]
pub struct MarkNftMinted<'info>{
    /// The user signing the transaction, must be the device owner.
    #[account(mut)]
    pub user: Signer<'info>,
    /// The device account to update, verified by seeds.
    #[account(
        mut,
        seeds=[b"device",device_account.device_hash.as_ref()],
        bump,
    )]
    pub device_account:Account<'info,DeviceAccount>,
}