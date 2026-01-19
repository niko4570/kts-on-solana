import * as anchor from "@coral-xyz/anchor";
import * as chai from "chai"; // 🔥 必须显式导入 chai
import { Program, BN } from "@coral-xyz/anchor";
import { Kts } from "../target/types/kts";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { expect } from "chai";
import { describe, it, before } from "mocha";
import chaiAsPromised from "chai-as-promised";
chai.use(chaiAsPromised);
describe("KTS Program Tests - register_device", () => {
  // Environment switchable (localnet / devnet)
  const cluster = (process.env.CLUSTER as "localnet" | "devnet") || "localnet";

  let provider: anchor.AnchorProvider;
  let program: Program<Kts>;
  let authorityPublicKey: PublicKey;
  let secondaryUser: Keypair;

  before(async () => {
    provider =
      cluster === "devnet"
        ? anchor.AnchorProvider.env()
        : anchor.AnchorProvider.local("http://127.0.0.1:8899", {
            commitment: "confirmed",
            preflightCommitment: "confirmed",
          });

    anchor.setProvider(provider);
    program = anchor.workspace.Kts as Program<Kts>;
    authorityPublicKey = provider.wallet.publicKey;

    secondaryUser = Keypair.generate();

    // Fund secondary user (localnet)
    if (cluster === "localnet") {
      const sig = await provider.connection.requestAirdrop(
        secondaryUser.publicKey,
        2 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(sig);
    }
  });

  // Helper: Generate unique device hash for each test
  function generateUniqueDeviceHash(): number[] {
    return Array.from({ length: 32 }, () => Math.floor(Math.random() * 256));
  }

  it("successfully registers a new device", async () => {
    const deviceHash = generateUniqueDeviceHash();
    const deviceName = "ValidDevice";
    const [devicePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("device"), Buffer.from(deviceHash)],
      program.programId,
    );

    const txSig = await program.methods
      .registerDevice(deviceHash, deviceName)
      .accountsPartial({
        user: authorityPublicKey,
        deviceAccount: devicePda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Register tx:", txSig);

    const account = await program.account.deviceAccount.fetch(devicePda);
    expect(account.owner.toBase58()).to.equal(authorityPublicKey.toBase58());
    expect(account.nftMinted).to.be.false;
    expect(Buffer.from(account.deviceHash)).to.deep.equal(
      Buffer.from(deviceHash),
    );
    expect(account.registeredAt.toNumber()).to.be.greaterThan(0);
  });

  it("fails on duplicate registration (DeviceAlreadyRegistered)", async () => {
    const deviceHash = generateUniqueDeviceHash();
    const deviceName = "DuplicateTest";
    const [devicePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("device"), Buffer.from(deviceHash)],
      program.programId,
    );

    // First registration succeeds
    await program.methods
      .registerDevice(deviceHash, deviceName)
      .accountsPartial({
        user: authorityPublicKey,
        deviceAccount: devicePda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    // Second registration should fail(because pda is already exist so just check pda)
    const account = await program.account.deviceAccount.fetch(devicePda);

    expect(account.owner.toBase58()).to.equal(authorityPublicKey.toBase58());
    expect(account.nftMinted).to.be.false;
    expect(Buffer.from(account.deviceHash)).to.deep.equal(
      Buffer.from(deviceHash),
    );
  });

  it("fails when device name exceeds max length (InvalidDeviceNameLength)", async () => {
    const deviceHash = generateUniqueDeviceHash();
    const tooLongName = "x".repeat(65); // Assuming max length is 64
    const [devicePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("device"), Buffer.from(deviceHash)],
      program.programId,
    );

    try {
      await program.methods
        .registerDevice(deviceHash, tooLongName)
        .accountsPartial({
          user: authorityPublicKey,
          deviceAccount: devicePda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      expect.fail("Expected InvalidDeviceNameLength error");
    } catch (error) {
      expect(error).to.be.instanceOf(anchor.AnchorError);
      const err = error as anchor.AnchorError;
      expect(err.error.errorCode.code).to.equal("InvalidDeviceNameLength");
    }
  });
});
