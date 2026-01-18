use anchor_lang::prelude::*;
use super::super::errors::KTSError;
use super::super::state::DeviceAccount;


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



#[derive(Accounts)]

pub struct MarkNftMinted<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds=[b"device",device_account.device_hash.as_ref()],
        bump,
    )]
    pub device_account:Account<'info,DeviceAccount>,
}