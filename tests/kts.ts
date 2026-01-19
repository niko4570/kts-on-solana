import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kts } from "../target/types/kts";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
  TransactionSignature,
} from "@solana/web3.js";
import { expect } from "chai";
import { describe, it, before } from "mocha";

interface TestConfig {
  deviceHash: number[];
  deviceName: string;
}

const TEST_DATA: TestConfig = {
  deviceHash: Array.from({ length: 32 }, () => 0x22), // number[] for IDL u8[32]
  deviceName: "AdvancedTestDevice",
};

// Global variables accessible in all test blocks
let program: Program<Kts>;
let provider: anchor.AnchorProvider;
let authorityPublicKey: PublicKey;
let secondaryUser: Keypair;
let devicePda: PublicKey;

// =============================================
// Test Suite: register_device
// =============================================
describe("KTS Program Tests", () => {
  // =============================================
  // Environment Configuration (switchable)
  // =============================================
  const cluster = (process.env.CLUSTER as "localnet" | "devnet") || "localnet";

  before(async () => {
    provider =
      cluster == "devnet"
        ? anchor.AnchorProvider.env()
        : anchor.AnchorProvider.local("http://127.0.0.1:8899", {
            commitment: "confirmed",
            preflightCommitment: "confirmed", // `preflightCommitment` : simulation transaction confirmation level
          });
    anchor.setProvider(provider);
    program = anchor.workspace.Kts as Program<Kts>;
    authorityPublicKey = provider.wallet.payer.publicKey;

    [devicePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("device"), Buffer.from(TEST_DATA.deviceHash)],
      program.programId,
    );

    secondaryUser = Keypair.generate();

    // Fund secondary user (localnet only, devnet needs manual airdrop)
    if (cluster == "localnet") {
      const signature = await provider.connection.requestAirdrop(
        secondaryUser.publicKey,
        2 * LAMPORTS_PER_SOL, // 2 sol
      );
      const latestBlockhash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction({
        signature,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      });
    }
  });

  describe("register_device Instruction", () => {
    it("successfully registers a new device", async () => {
      const txSig = await program.methods
        .registerDevice(TEST_DATA.deviceHash, TEST_DATA.deviceName)
        .accountsPartial({
          user: authorityPublicKey,
          deviceAccount: devicePda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Transaction signature:", txSig);

      //Assert： Verify state
      const deviceAccount = await program.account.deviceAccount.fetch(
        devicePda,
      );
      expect(deviceAccount.owner.toBase58).to.equal(
        authorityPublicKey.toBase58,
      );
      expect(deviceAccount.nftMinted).to.be.false;
      expect(Buffer.from(deviceAccount.deviceHash)).to.deep.equal(
        Buffer.from(TEST_DATA.deviceHash),
      );
      expect(deviceAccount.registeredAt.toNumber()).to.be.greaterThan(0);

      it("fails on duplicate registraion(DeviceAlreadyRegistered)", async () => {
        try {
          await program.methods
            .registerDevice(TEST_DATA.deviceHash, TEST_DATA.deviceName)
            .accountsPartial({
              user: authorityPublicKey,
              deviceAccount: devicePda,
              systemProgram: SystemProgram.programId,
            })
            .rpc();
          expect.fail("Expected DeviceAlreadyRegistered error");
        } catch (error) {
          const anchorErr = error as anchor.AnchorError;
          expect(anchorErr.error.errorCode.code).to.equal(
            "DeviceAlreadyRegistered",
          );
          expect(anchorErr.error.errorMessage).to.equal(
            "This device is already registered",
          );
        }
      });
    });
  });
});
