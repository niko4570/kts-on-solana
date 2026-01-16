use anchor_lang::prelude::*;
use super::super::constants::*;
use super::super::errors::KTSError;
use super::super::state::DeviceAccount;

pub fn register_device(
    ctx: Context<RegisterDevice>,
    device_hash:[u8;32],
    device_name:String,
)->Result<()>{
    require!(device_name.len()<=64,KTSError::InvalidProcessNameLength);
    let device_account=&mut ctx.accounts.device_account;
    device_account.owner=*ctx.accounts.user.key;
    device_account.device_hash=device_hash;
    device_account.registered_at=Clock::get()?.unix_timestamp;
    device_account.nft_minted=false;
    Ok(())
}

#[derive(Accounts)]
#[instruction(device_hash:[u8;32])]
pub struct RegisterDevice<'info>{  
    #[account(mut)]
    pub user:Signer<'info>,

    #[account(
        init,
        payer=user,
        space=DEVICE_ACCOUNT_SIZE,
        seeds=[b"device",device_hash.as_ref()],
        bump,
    )]
    pub device_account:Account<'info,DeviceAccount>,
    pub system_program:Program<'info,System>,

}