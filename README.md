# kts-on-solana

Collect your computer usage data and upload it to the blockchain. (A computer usage tracker)

Solana dApp - Record computer usage statistics with privacy (only hash on-chain)

## Features

- **Device Registration**: Register devices using a unique device hash as PDA seed, ensuring each device is registered only once.
- **Daily Usage Upload**: Upload aggregated daily usage statistics (CPU, memory, top processes) along with a SHA256 hash of the data for privacy. Top processes are stored as fixed-size byte arrays (up to 5 processes, each up to 32 bytes).
- **Soulbound NFT Minting**: Mark soulbound NFT as minted upon device registration (integration with Metaplex for actual minting).
- **Privacy-Focused**: Only stores hashes and minimal plaintext statistics on-chain to protect user data.
- **Electron App Integration**: Scheduled data collection in the Electron main process for hourly snapshots and daily uploads.

## Tech Stack

- **Blockchain**: Solana + Anchor Framework
- **Frontend/App**: Electron + TypeScript
- **NFT**: Metaplex (for soulbound NFTs)
- **Testing**: Mocha + Chai
- **Linting**: Prettier

## Architecture

The project consists of:

- **Solana Program** (`programs/kts/`): Anchor-based smart contract handling device registration and usage uploads.
- **Electron App** (`app/`): Desktop application for data collection and blockchain interaction.
- **Tests** (`tests/`): TypeScript tests for program validation.
- **Migrations** (`migrations/`): Deployment scripts.

### Key Accounts

- **DeviceAccount**: Stores device owner, hash, registration timestamp, and NFT mint status.
- **DailyUsageAccount**: Stores daily usage data including averages, top processes, data hash, and creation timestamp.

### Program Details

The KTS program implements three main instructions with corresponding account structures:

#### State Structures

- **DeviceAccount**:

  - `owner`: Public key of the device owner
  - `device_hash`: 32-byte SHA256 hash of device info
  - `registered_at`: Timestamp when the device was registered
  - `nft_minted`: Boolean indicating if the NFT has been minted

- **DailyUsageAccount**:
  - `device`: Public key of the associated device
  - `timestamp`: Timestamp for the daily usage data
  - `avg_cpu_usage`: Average CPU usage percentage (f32)
  - `avg_memory_usage`: Average memory usage percentage (f32)
  - `top_processes`: Array of 5 process names, each up to 32 bytes
  - `data_hash`: 32-byte SHA256 hash of the usage data
  - `created_at`: Timestamp when the record was created

#### Constants

- `MAX_PROCESS_NAME_LENGTH`: 32 bytes (maximum length for process names)
- `PROCESS_ARRAY_SIZE`: 5 (fixed size for top processes array)
- `DEVICE_ACCOUNT_SIZE`: 81 bytes (discriminator + 32 bytes owner + 32 bytes device hash + 8 bytes timestamp + 1 byte boolean)
- `DAILY_USAGE_ACCOUNT_SIZE`: 256 bytes (discriminator + device pubkey + timestamps + floats + process array + data hash)

### Instructions

- `register_device`: Registers a new device using a unique device hash as PDA seed.

  - Parameters: `device_hash` ([u8;32]), `device_name` (String)
  - Creates a new DeviceAccount with PDA seeds `[b"device", device_hash.as_ref()]`
  - Validates device name length (≤64 characters)
  - Initializes account with owner, hash, timestamp, and nft_minted = false

- `upload_daily_usage`: Uploads aggregated daily usage statistics (CPU, memory, top processes) along with a SHA256 hash of the data.

  - Parameters: `device_hash` ([u8;32]), `timestamp` (i64), `avg_cpu_usage` (f32), `avg_memory_usage` (f32), `top_processes` (Vec<String>), `data_hash` ([u8;32])
  - Validates that the device belongs to the caller
  - Ensures `top_processes` array contains exactly 5 entries
  - Verifies each process name ≤ 32 characters
  - Creates a new DailyUsageAccount with PDA seeds `[b"daily_usage", device_hash.as_ref(), &timestamp.to_le_bytes().as_ref()]`

- `mark_nft_minted`: Marks the NFT as minted for a registered device (for soulbound NFT integration).
  - Validates that the caller owns the device
  - Checks that the NFT hasn't already been marked as minted
  - Updates the device account's `nft_minted` field to true

### Error Handling

The program implements comprehensive error handling through the `KTSError` enum:

- `DeviceAlreadyRegistered`: Thrown when attempting to register a device that's already registered
- `InvalidDeviceOwner`: Thrown when the caller doesn't own the device they're trying to access
- `DailyUsageAlreadyUploaded`: Thrown when daily usage for a specific day has already been uploaded
- `InvalidProcessNameLength`: Thrown when a process name exceeds the maximum length
- `InvalidTopProcessesArraySize`: Thrown when the top processes array doesn't contain exactly 5 entries
- `NftAlreadyMinted`: Thrown when trying to mint an NFT for a device that already has one marked

## Installation

### Prerequisites

- Node.js (v16+)
- Yarn
- Rust (for Anchor)
- Solana CLI
- Anchor CLI

### Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/niko4570/kts-on-solana
   cd kts
   ```

2. Install dependencies:

   ```bash
   yarn install
   ```

3. Install Anchor dependencies:
   ```bash
   anchor build
   ```

## Building and Deployment

### Build the Solana Program

```bash
anchor build
```

### Deploy to Localnet

1. Start a local Solana validator in a new terminal window:

   ```bash
   solana-test-validator
   ```

   Or alternatively, you can use Anchor to start the local network:

   ```bash
   anchor localnet
   ```

2. In another terminal window, deploy the program to the local network:

   ```bash
   anchor deploy --provider.cluster localnet
   ```

   If you're using `anchor localnet` in the first terminal, you can just run:

   ```bash
   anchor deploy
   ```

3. To run tests against the deployed program:

   ```bash
   anchor test
   ```

4. Configure Solana CLI to use the local network (if needed):
   ```bash
   solana config set --url localhost
   ```

### Deploy to Devnet (Optional)

To deploy to Solana Devnet:

```bash
anchor deploy --provider.cluster devnet
```

Remember to have sufficient SOL tokens in your wallet for deployment fees.

### Deployment Examples

#### Example: Complete Localnet Deployment

1. Build the program:

   ```bash
   anchor build
   ```

2. Start the local validator in the background:

   ```bash
   solana-test-validator &
   ```

3. Deploy the program:

   ```bash
   anchor deploy
   ```

   You should see output similar to:

   ```
   Deploying program "kts"...
   Program Id: Emd9sap9brkhi4VLMUfUmf7qX2vaV9m2fwXjQSdB7XYp
   ```

4. Run tests to verify:

   ```bash
   anchor test
   ```

#### Example: Deploying to Devnet

1. Switch to devnet:

   ```bash
   solana config set --url https://api.devnet.solana.com
   ```

2. Airdrop some SOL for deployment fees:

   ```bash
   solana airdrop 2
   ```

3. Build and deploy:

   ```bash
   anchor build
   anchor deploy --provider.cluster devnet
   ```

   Note: Deployment to devnet may take longer and requires network fees.

### Build the Electron App

```bash
cd app
npm install
npm run build
```

## Usage

### Running Tests

```bash
anchor test
```

The tests include validation of device registration, usage upload, and NFT minting status.

### Running the App

1. Start a local Solana validator:

   ```bash
   solana-test-validator
   ```

2. Deploy the program:

   ```bash
   anchor deploy
   ```

3. Run the Electron app:
   ```bash
   cd app
   npm start
   ```

### API Usage

#### Register Device

```typescript
// Example: Register a device with hash and name
await program.methods
  .registerDevice(deviceHash, deviceName)
  .accounts({
    user: userPublicKey,
    deviceAccount: deviceAccountPDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### Upload Daily Usage

```typescript
// Example: Upload usage data
await program.methods
  .uploadDailyUsage(
    deviceHash,
    timestamp,
    avgCpu,
    avgMem,
    topProcesses,
    dataHash,
  )
  .accounts({
    user: userPublicKey,
    deviceAccount: deviceAccountPDA,
    dailyUsageAccount: dailyUsageAccountPDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

#### Mark NFT Minted

```typescript
// Example: Mark NFT as minted for a device
await program.methods
  .markNftMinted()
  .accounts({
    user: userPublicKey,
    deviceAccount: deviceAccountPDA,
  })
  .rpc();
```

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature/your-feature`.
3. Commit changes: `git commit -am 'Add your feature'`.
4. Push to branch: `git push origin feature/your-feature`.
5. Submit a pull request.

### Code Style

- Use Prettier for formatting: `yarn lint:fix`.
- Follow Rust and TypeScript best practices.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## QuickStart

Follow these steps to get started quickly with kts-on-solana.

### 1. Prerequisites

Ensure you have the following installed:

- Node.js (v16+)
- Yarn
- Rust and Cargo
- Solana CLI
- Anchor CLI

### 2. Clone and Setup

```bash
git clone <repository-url>
cd kts
yarn install
```

### 3. Build the Solana Program

```bash
anchor build
```

### 4. Start Local Solana Validator

In a new terminal:

```bash
solana-test-validator
```

### 5. Deploy the Program

```bash
anchor deploy
```

### 6. Run Tests

```bash
anchor test
```

### 7. Build and Run the Electron App

```bash
cd app
npm install
npm run build
npm start
```

### 8. Basic Usage

- **Register a Device**: In the app, enter device details to register.
- **Upload Usage Data**: The app will automatically collect and upload daily usage data at midnight.

For detailed API usage, see the [Usage](#usage) section.
