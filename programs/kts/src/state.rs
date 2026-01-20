use anchor_lang::prelude::*;
use super::constants::*;

/// Represents a registered device account on the blockchain.
///
/// This account stores information about a device that has been registered
/// in the KTS system, including ownership, unique hash, and NFT minting status.
#[account]
pub struct DeviceAccount{
    /// The public key of the user who owns this device.
    pub owner:Pubkey,
    /// A unique 32-byte SHA256 hash identifying the device.
    pub device_hash:[u8;32], //sha256 hash of device info
    /// Unix timestamp when the device was registered.
    pub registered_at: i64, //timestamp
    /// Flag indicating whether an NFT has been minted for this device.
    pub nft_minted: bool,
}

/// Represents daily usage data for a device.
///
/// This account stores aggregated usage metrics collected from a device
/// for a specific day, including CPU/memory usage and top processes.
#[account]
pub struct DailyUsageAccount{
    /// The public key of the device account this usage data belongs to.
    pub device:Pubkey,
    /// Unix timestamp representing the day this usage data is for.
    pub timestamp:i64,
    /// Average CPU usage percentage for the day.
    pub avg_cpu_usage:f32,
    /// Average memory usage percentage for the day.
    pub avg_memory_usage:f32,
    /// Array of top process names, each padded to MAX_PROCESS_NAME_LENGTH.
    pub top_processes:[[u8;MAX_PROCESS_NAME_LENGTH];PROCESS_ARRAY_SIZE], //array of process names
    /// SHA256 hash of the raw usage data for verification.
    pub data_hash:[u8;32], //sha256 hash of the usage data
    /// Unix timestamp when this account was created.
    pub created_at:i64,//timestamp
}