use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("Emd9sap9brkhi4VLMUfUmf7qX2vaV9m2fwXjQSdB7XYp");

use instructions::*;

/// The main KTS (Key Tracking System) program module.
///
/// This program manages device registration and daily usage tracking on Solana.
/// It provides instructions for registering devices, uploading usage data, and marking NFT minting.
#[program]
pub mod kts {
    use super::*;

    /// Registers a new device in the system.
    ///
    /// Creates a new DeviceAccount with the provided device hash and name.
    /// The device must not already be registered.
    ///
    /// # Arguments
    /// * `ctx` - The execution context containing accounts.
    /// * `device_hash` - A unique 32-byte hash identifying the device.
    /// * `device_name` - A human-readable name for the device (max 64 chars).
    ///
    /// # Returns
    /// * `Result<()>` - Success or an error if registration fails.
    pub fn register_device(
        ctx:Context<RegisterDevice>,
        device_hash:[u8;32],
        device_name:String,
    ) ->Result<()>{
        instructions::register_device(ctx,device_hash,device_name)
    }

    /// Uploads daily usage data for a registered device.
    ///
    /// Creates a DailyUsageAccount with CPU/memory usage metrics and top processes.
    /// The device must be owned by the signer and usage not already uploaded for the day.
    ///
    /// # Arguments
    /// * `ctx` - The execution context containing accounts.
    /// * `device_hash` - The 32-byte hash of the device.
    /// * `timestamp` - Unix timestamp for the usage data.
    /// * `avg_cpu_usage` - Average CPU usage percentage.
    /// * `avg_memory_usage` - Average memory usage percentage.
    /// * `top_processes` - Array of top 5 process names.
    /// * `data_hash` - 32-byte hash of the usage data for verification.
    ///
    /// # Returns
    /// * `Result<()>` - Success or an error if upload fails.
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

    /// Marks that an NFT has been minted for a device.
    ///
    /// Sets the nft_minted flag to true on the DeviceAccount.
    /// Can only be called once per device.
    ///
    /// # Arguments
    /// * `ctx` - The execution context containing accounts.
    ///
    /// # Returns
    /// * `Result<()>` - Success or an error if already minted.
    pub fn mark_nft_minted(ctx:Context<MarkNftMinted>)->Result<()>{
        instructions::mark_nft_minted(ctx)
    }
}