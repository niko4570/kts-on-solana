pub const MAX_PROCESS_NAME_LENGTH: usize = 32;
pub const PROCESS_ARRAY_SIZE: usize = 5;

pub const DEVICE_ACCOUNT_SIZE: usize = 8 + 32 + 32 + 8 + 1; // 81 bytes

pub const DAILY_USAGE_ACCOUNT_SIZE: usize = 8    // discriminator
    + 32                                         // device: Pubkey
    + 8                                          // timestamp: i64
    + 4                                          // avg_cpu_usage: f32
    + 4                                          // avg_memory_usage: f32
    + (MAX_PROCESS_NAME_LENGTH * PROCESS_ARRAY_SIZE) // 160 bytes
    + 32                                         // data_hash
    + 8;                                         // created_at: i64
    // total 256 bytes