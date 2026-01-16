use anchor_lang::prelude::*;

#[error_code]

pub enum KTSError{
    
    #[msg("This device is already registered")]
    DeviceAlreadyRegistered,

    #[msg("You are not the owner of this device")]
    InvalidDeviceOwner,

    #[msg("Daily usage for this day has already been uploaded")]
    DailyUsageAlreadyUploaded,

    #[msg("Process name exceeds maximum length")]
    InvalidProcessNameLength,

    #[msg("Top processes array must contain exactly 5 entries")]
    InvalidTopProcessesArraySize,
}