use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("GUMBiEgcTL58Sz7wKoWWQyiUoFisbZ6ELvt2ZyoyEJ5G");

use instructions::*;

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

    pub fn upload_daily_usage(
        ctx:Context<UploadDailyUsage>,
        device_hash:[u8;32],
        timestamp:i64,
        avg_cpu_usage:f32,
        avg_memory_usage:f32,
        top_processes:Vec<String>,
        data_hash:[u8;32],
    )->Result<()>{
        instructions::upload_daily_usage(
            ctx, 
            device_hash,
            timestamp, 
            avg_cpu_usage, 
            avg_memory_usage, 
            top_processes, 
            data_hash)
    }

    pub fn mark_nft_minted(ctx:Context<MarkNftMinted>)->Result<()>{
        instructions::mark_nft_minted(ctx)
    }
}