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
- **DailyUsageAccount**: Stores daily usage data including averages, top processes, top processes (stored as byte arrays), data hash, and creation timestamp.

### Instructions

- `register_device`: Registers a new device using a unique device hash as PDA seed.
- `upload_daily_usage`: Uploads aggregated daily usage statistics (CPU, memory, top processes) along with a SHA256 hash of the data.
- `mark_nft_minted`: Marks the NFT as minted for a registered device (for soulbound NFT integration).

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
await program.methods.registerDevice(deviceHash, deviceName).accounts({...}).rpc();
```

#### Upload Daily Usage

```typescript
// Example: Upload usage data
await program.methods.uploadDailyUsage(deviceHash, timestamp, avgCpu, avgMem, topProcesses, dataHash).accounts({...}).rpc();
```

#### Mark NFT Minted

```typescript
// Example: Mark NFT as minted for a device
await program.methods
  .markNftMinted()
  .accounts({ deviceAccount: deviceAccountPubkey, user: userPubkey })
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
