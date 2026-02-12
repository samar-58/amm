import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { Keypair, PublicKey } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("amm", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.amm as Program<Amm>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  let seed: anchor.BN;
  let config: PublicKey;
  let lptoken: PublicKey;
  let minta: PublicKey;
  let mintb: PublicKey;
  let userx: PublicKey;
  let usery: PublicKey;
  let userlp: PublicKey;
  let vaulta: PublicKey;
  let vaultb: PublicKey;

  before(async () => {
    minta = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6
    );

    mintb = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6
    );

    // if (minta.toBuffer().compare(mintb.toBuffer()) > 0) {
    //   [minta, mintb] = [mintb, minta];
    // }

    seed = new anchor.BN(11);
    [config] = PublicKey.findProgramAddressSync(
      [Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    [lptoken] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), config.toBuffer()],
      program.programId
    );

    const userXAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      minta,
      wallet.publicKey
    );
    userx = userXAta.address;

    const userYAta = await getOrCreateAssociatedTokenAccount(
      connection,
      wallet.payer,
      mintb,
      wallet.publicKey
    );
    usery = userYAta.address;

    userlp = getAssociatedTokenAddressSync(lptoken, wallet.publicKey, false);

    vaulta = getAssociatedTokenAddressSync(minta, config, true, TOKEN_PROGRAM_ID);
    vaultb = getAssociatedTokenAddressSync(mintb, config, true, TOKEN_PROGRAM_ID);

    await mintTo(connection, wallet.payer, minta, userx, wallet.payer, 1_000_000_000);
    await mintTo(connection, wallet.payer, mintb, usery, wallet.payer, 1_000_000_000);
  });

  it("Initialize pool", async () => {
    const fee = 2;

    const tx = await program.methods
      .initialize(seed, fee, wallet.publicKey)
      .accountsStrict({
        signer: wallet.publicKey,
        mintX: minta,
        mintY: mintb,
        lpToken: lptoken,
        mintXVault: vaulta,
        mintYVault: vaultb,
        config: config,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Initialize tx:", tx);
  });

  it("Deposit liquidity", async () => {
    const amount = new anchor.BN(2);
    const max_x = new anchor.BN(100_000);
    const max_y = new anchor.BN(100_000);

    const tx = await program.methods
      .deposit(amount, max_x, max_y)
      .accountsStrict({
        signer: wallet.publicKey,
        mintX: minta,
        mintY: mintb,
        lpToken: lptoken,
        mintXVault: vaulta,
        mintYVault: vaultb,
        userX: userx,
        userY: usery,
        userAtaLp: userlp,
        config: config,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Deposit tx:", tx);
  });

  it("Swap tokens", async () => {
    const amount = new anchor.BN(1_000);
    const min_receive = new anchor.BN(1);

    const tx = await program.methods
      .swap(true, min_receive, amount)
      .accountsStrict({
        signer: wallet.publicKey,
        mintX: minta,
        mintY: mintb,
        lpToken: lptoken,
        mintXVault: vaulta,
        mintYVault: vaultb,
        userX: userx,
        userY: usery,
        config: config,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Swap tx:", tx);
  });

  it("Withdraw liquidity", async () => {
    const amount = new anchor.BN(1);
    const min_x = new anchor.BN(0);
    const min_y = new anchor.BN(0);

    const tx = await program.methods
      .withdraw(amount, min_x, min_y)
      .accountsStrict({
        signer: wallet.publicKey,
        mintX: minta,
        mintY: mintb,
        lpToken: lptoken,
        mintXVault: vaulta,
        mintYVault: vaultb,
        userX: userx,
        userY: usery,
        userAtaLp: userlp,
        config: config,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Withdraw tx:", tx);
  });
});