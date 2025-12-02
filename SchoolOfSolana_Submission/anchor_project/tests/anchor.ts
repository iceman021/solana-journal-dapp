import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { JournalDapp } from "../target/types/journal_dapp";
describe("journal_dapp", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JournalDapp as anchor.Program<JournalDapp>;
  
  // 1. Setup
  const program = program;
  const wallet = pg.wallet;

  // 2. Test Data
  const title = "Final Submission";
  const message = "Testing all requirements";

  it("1. Happy Path: Create Entry", async () => {
    // Generate PDA
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(title), wallet.publicKey.toBuffer()],
      program.programId
    );

    // Call Instruction
    await program.methods
      .createEntry(title, message)
      .accounts({
        journalEntry: pda,
        owner: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Verify
    const account = await program.account.journalEntry.fetch(pda);
    if (account.title !== title) throw new Error("Title mismatch");
    console.log("✅ Create Passed");
  });

  it("2. Unhappy Path: Prevent Duplicate", async () => {
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(title), wallet.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .createEntry(title, "Duplicate")
        .accounts({
          journalEntry: pda,
          owner: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      throw new Error("Program allowed duplicate entry!");
    } catch (e) {
      if (e.message === "Program allowed duplicate entry!") {
        throw e;
      }
      console.log("✅ Duplicate Blocked (Error Caught)");
    }
  });

  it("3. Happy Path: Update Entry", async () => {
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(title), wallet.publicKey.toBuffer()],
      program.programId
    );

    const newMessage = "Updated Content";

    await program.methods
      .updateEntry(title, newMessage)
      .accounts({
        journalEntry: pda,
        owner: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.journalEntry.fetch(pda);
    if (account.message !== newMessage) throw new Error("Update failed");
    console.log("✅ Update Passed");
  });

  it("4. Happy Path: Delete Entry", async () => {
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(title), wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .deleteEntry(title)
      .accounts({
        journalEntry: pda,
        owner: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    try {
      await program.account.journalEntry.fetch(pda);
      throw new Error("Account should be gone");
    } catch (e) {
      if (e.message === "Account should be gone") throw e;
      console.log("✅ Delete Passed");
    }
  });
});
