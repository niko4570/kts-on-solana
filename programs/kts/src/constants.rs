/// Maximum length allowed for a process name in bytes.
pub const MAX_PROCESS_NAME_LENGTH: usize = 32;

/// Number of top processes to store in the daily usage data.
pub const PROCESS_ARRAY_SIZE: usize = 5;

/// Maximum length allowed for a device name in characters.
pub const MAX_DEVICE_NAME_LENGTH: usize = 64;

/// Size of the DeviceAccount struct in bytes (including 8-byte discriminator).
pub const DEVICE_ACCOUNT_SIZE: usize = 8 + 32 + 32 + 8 + 1; // 81 bytes

/// Size of the DailyUsageAccount struct in bytes (including 8-byte discriminator).
pub const DAILY_USAGE_ACCOUNT_SIZE: usize = 8    // discriminator
    + 32                                         // device: Pubkey
    + 8                                          // timestamp: i64
    + 4                                          // avg_cpu_usage: f32
    + 4                                          // avg_memory_usage: f32
    + (MAX_PROCESS_NAME_LENGTH * PROCESS_ARRAY_SIZE) // 160 bytes
    + 32                                         // data_hash
    + 8;                                         // created_at: i64
    // total 256 bytes