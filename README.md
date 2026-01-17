# kts-on-solana

Collect your computer usage data and upload it to the blockchain. (A computer usage tracker)

Solana dApp - Record computer usage statistics with privacy (only hash on-chain)

## Features

- **Device Registration**: Register devices using a unique device hash as PDA seed, ensuring each device is registered only once.
- **Daily Usage Upload**: Upload aggregated daily usage statistics (CPU, memory, top processes) along with a SHA256 hash of the data for privacy.
- **Soulbound NFT Minting**: Automatically mint a soulbound NFT upon first device registration (planned feature).
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
- **DailyUsageAccount**: Stores daily usage data including averages, top processes, and data hash.

### Instructions

- `register_device`: Registers a new device.
- `upload_daily_usage`: Uploads daily usage data for a registered device.

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

```bash
anchor deploy
```

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

[QuickStart->](#quikstart)
