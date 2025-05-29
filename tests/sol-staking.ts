import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolStaking } from "../target/types/sol_staking";

describe("sol-staking", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solStaking as Program<SolStaking>;
  const user = anchor.web3.Keypair.generate();
  console.log("User: ", user.publicKey);

  it("Stake SOL!", async () => {

    // Airdrop SOL
    const sig = await provider.connection.requestAirdrop(user.publicKey, 5e9);
    await provider.connection.confirmTransaction(sig);

    // Get vault PDA + bump
    const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    // Get stake_account PDA
    const [stake_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), user.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .depositSol(vaultBump, new anchor.BN(1_000_000_000))
      .accounts({
        stakeAccount: stake_account,
        user: user.publicKey,
        vault,
        system_program: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);

    const stakeAccountData = await program.account.stakeAccount.fetch(stake_account);
    console.log("stake_account:", {
      owner: stakeAccountData.owner,
      staked: stakeAccountData.staked,
      bump: stakeAccountData.bump
    });

    const vaultBalance = await provider.connection.getBalance(vault);
    console.log("Vault balance (lamports):", vaultBalance);

  });
});