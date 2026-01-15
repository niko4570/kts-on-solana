use anchor_lang::prelude::*;
use super::constants::*;

#[account]
pub struct DeviceAccount{
    pub owner:Pubkey,
    pub device_hash:[u8;32], //sha256 hash of device info
    pub registered_at: i64, //timestamp
    pub nft_minted: bool,
}

#[account]
pub struct DailyUsageAccount{
    pub device:Pubkey,
    pub timestamp:i64,
    pub avg_cpu_usage:f32,
    pub avg_memory_usage:f32,
    pub top_processes:[[u8;MAX_PROCESS_NAME_LENGTH];PROCESS_ARRAY_SIZE], //array of process names
    pub data_hash:[u8;32], //sha256 hash of the usage data
    pub created_at:i64,//timestamp
}