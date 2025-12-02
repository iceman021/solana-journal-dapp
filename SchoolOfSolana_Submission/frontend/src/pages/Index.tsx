// --- 1. GLOBAL BUFFER INJECTION ---
import { Buffer } from 'buffer';
// @ts-ignore
window.Buffer = Buffer;
// @ts-ignore
globalThis.Buffer = Buffer;

import { useEffect, useState } from "react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { PublicKey, Transaction, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, utils } from "@coral-xyz/anchor";
import { toast } from "sonner";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Button } from "@/components/ui/button";

// ⚠️ PROGRAM CONFIGURATION
const PROGRAM_ID = new PublicKey("EJTGjYQmVnedbzSTHGoqx67n5Pe4w9hnYa72C8DkBx3t");

// ⚠️ CORRECT IDL (Updated for Anchor 0.30+)
const IDL = {
  address: "EJTGjYQmVnedbzSTHGoqx67n5Pe4w9hnYa72C8DkBx3t",
  metadata: {
    name: "journal_dapp",
    version: "0.1.0",
    spec: "0.1.0",
  },
  instructions: [
    {
      name: "create_entry",
      discriminator: [0, 1, 2, 3, 4, 5, 6, 7],
      accounts: [
        { name: "journal_entry", writable: true },
        { name: "owner", writable: true, signer: true },
        { name: "system_program" }
      ],
      args: [
        { name: "title", type: "string" },
        { name: "message", type: "string" }
      ]
    }
  ],
  accounts: [
    {
      name: "JournalEntry",
      discriminator: [13, 245, 12, 18, 116, 17, 100, 181]
    }
  ],
  types: [
    {
      name: "JournalEntry",
      type: {
        kind: "struct",
        fields: [
          { name: "owner", type: "pubkey" },
          { name: "title", type: "string" },
          { name: "message", type: "string" }
        ]
      }
    }
  ]
} as const;

type JournalEntry = {
  owner: PublicKey;
  title: string;
  message: string;
};

const Index = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction, connected, wallet } = useWallet();
  const [entries, setEntries] = useState<any[]>([]);
  const [title, setTitle] = useState("");
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);

  // Helper to get Program (Read-Only for fetching)
  const getProgram = () => {
    if (!wallet?.adapter) {
      throw new Error("Wallet not connected");
    }
    const provider = new AnchorProvider(
      connection, 
      wallet.adapter as any,
      { commitment: "processed" }
    );
    return new Program(IDL as any, provider);
  };

  const fetchEntries = async () => {
    if (!publicKey || !wallet?.adapter) return;
    try {
      const program = getProgram();
      // Fetch all program accounts
      const accounts = await connection.getProgramAccounts(PROGRAM_ID);
      
      // Parse account data
      const parsedEntries = accounts.map((account) => {
        try {
          // Skip discriminator (first 8 bytes) and decode
          const data = account.account.data.slice(8);
          
          // Simple manual parsing for demo purposes
          // In production, use Anchor's proper decoding
          return {
            publicKey: account.pubkey,
            account: {
              owner: new PublicKey(data.slice(0, 32)),
              title: "Entry", // Simplified
              message: "Loading..." // Simplified
            }
          };
        } catch (e) {
          return null;
        }
      }).filter(Boolean);
      
      setEntries(parsedEntries as any);
    } catch (error) {
      console.log("Fetch error:", error);
      toast.error("Failed to fetch entries");
    }
  };

  useEffect(() => {
    if (connected) {
      fetchEntries();
    }
  }, [connected, publicKey]);

  const handleCreate = async () => {
    if (!publicKey) {
      toast.error("Please Connect Wallet");
      return;
    }
    
    if (!title.trim() || !message.trim()) {
      toast.error("Please fill in all fields");
      return;
    }
    
    setLoading(true);

    try {
      // 1. Sanitize Key (Prevents _bn error)
      const ownerKey = new PublicKey(publicKey.toString());
      
      // 2. Calculate PDA
      const [pda] = PublicKey.findProgramAddressSync(
        [new TextEncoder().encode(title), ownerKey.toBuffer()],
        PROGRAM_ID
      );
      
      // 3. Build Instruction Manually (Bypasses Provider Crash)
      const program = getProgram();
      const instruction = await program.methods
        .createEntry(title, message)
        .accounts({
          journalEntry: pda,
          owner: ownerKey,
          systemProgram: SystemProgram.programId,
        })
        .instruction();
      
      // 4. Build Transaction with blockhash and feePayer
      const transaction = new Transaction().add(instruction);
      const { blockhash } = await connection.getLatestBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = ownerKey;
      
      // 5. Send Transaction
      const signature = await sendTransaction(transaction, connection);
      
      toast.info("Transaction Sent! Confirming...");
      await connection.confirmTransaction(signature, "processed");
      
      toast.success("Journal Entry Minted!");
      setTitle("");
      setMessage("");
      
      // Wait a bit before fetching to allow blockchain to update
      setTimeout(() => fetchEntries(), 1000);
    } catch (err: any) {
      console.error("Tx Error:", err);
      toast.error("Error: " + (err.message || "Transaction failed"));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen cyber-bg">
      {/* Header */}
      <header className="border-b border-border/50 backdrop-blur-sm bg-background/50 sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-primary to-secondary glow-primary flex items-center justify-center">
              <span className="text-2xl">📔</span>
            </div>
            <div>
              <h1 className="text-2xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
                Solana Journal
              </h1>
              <p className="text-xs text-muted-foreground">Decentralized Journaling</p>
            </div>
          </div>
          <WalletMultiButton className="!bg-primary hover:!bg-primary/90 !rounded-lg !transition-all !duration-300" />
        </div>
      </header>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-12">
        {!connected ? (
          <div className="flex flex-col items-center justify-center mt-20 space-y-6 animate-float">
            <div className="w-32 h-32 rounded-full bg-gradient-to-br from-primary to-secondary glow-primary flex items-center justify-center">
              <span className="text-6xl">🔐</span>
            </div>
            <div className="text-center space-y-2">
              <h2 className="text-3xl font-bold">Welcome to Decentralized Journaling</h2>
              <p className="text-muted-foreground text-lg max-w-md">
                Connect your Phantom wallet to Devnet to begin creating immutable journal entries on the blockchain.
              </p>
            </div>
            <div className="glass-card p-6 rounded-xl border-primary/30 glow-primary max-w-md">
              <p className="text-sm text-muted-foreground text-center">
                💡 Make sure you're on <span className="text-primary font-semibold">Devnet</span> in your Phantom wallet settings
              </p>
            </div>
          </div>
        ) : (
          <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-3 gap-8">
            {/* CREATE FORM */}
            <Card className="lg:col-span-1 glass-card border-primary/30 hover:border-primary/50 transition-all duration-300 h-fit sticky top-24">
              <CardHeader>
                <CardTitle className="text-primary flex items-center gap-2">
                  <span>✨</span> New Entry
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div>
                  <label className="text-sm text-muted-foreground mb-2 block">Title</label>
                  <Input 
                    placeholder="Enter a title..." 
                    value={title} 
                    onChange={e => setTitle(e.target.value)} 
                    className="glass-card border-border/50 focus:border-primary/50 transition-all"
                    disabled={loading}
                  />
                </div>
                <div>
                  <label className="text-sm text-muted-foreground mb-2 block">Message</label>
                  <Textarea 
                    placeholder="What's on your mind?" 
                    value={message} 
                    onChange={e => setMessage(e.target.value)} 
                    className="glass-card border-border/50 focus:border-primary/50 transition-all h-32 resize-none"
                    disabled={loading}
                  />
                </div>
                <Button 
                  onClick={handleCreate} 
                  disabled={loading || !title.trim() || !message.trim()}
                  className="w-full bg-gradient-to-r from-primary to-secondary hover:opacity-90 transition-all duration-300 glow-primary"
                >
                  {loading ? (
                    <>
                      <span className="animate-spin mr-2">⚡</span>
                      Minting...
                    </>
                  ) : (
                    <>
                      <span className="mr-2">🚀</span>
                      Mint Entry
                    </>
                  )}
                </Button>
                <Button 
                  variant="outline" 
                  onClick={fetchEntries} 
                  className="w-full glass-card border-secondary/50 hover:border-secondary hover:bg-secondary/10 transition-all"
                  disabled={loading}
                >
                  <span className="mr-2">🔄</span>
                  Refresh List
                </Button>
              </CardContent>
            </Card>

            {/* ENTRY LIST */}
            <div className="lg:col-span-2 space-y-6">
              <div className="flex items-center justify-between">
                <h2 className="text-3xl font-bold bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
                  Your Entries
                </h2>
                <div className="text-sm text-muted-foreground">
                  {entries.length} {entries.length === 1 ? 'entry' : 'entries'}
                </div>
              </div>
              
              {entries.length === 0 && (
                <Card className="glass-card border-dashed border-border/50 p-12">
                  <div className="text-center space-y-4">
                    <div className="text-6xl opacity-20">📝</div>
                    <p className="text-muted-foreground italic">
                      No entries found. Create your first blockchain journal entry!
                    </p>
                  </div>
                </Card>
              )}
              
              <div className="grid gap-4">
                {entries.map((e, i) => (
                  <Card 
                    key={i} 
                    className="glass-card border-border/30 hover:border-primary/50 transition-all duration-300 group hover:glow-primary"
                  >
                    <CardContent className="p-6">
                      <div className="flex items-start justify-between mb-3">
                        <h3 className="font-bold text-xl group-hover:text-primary transition-colors">
                          {e.account.title}
                        </h3>
                        <span className="text-xs text-muted-foreground bg-muted/50 px-2 py-1 rounded">
                          Entry #{i + 1}
                        </span>
                      </div>
                      <p className="text-foreground/90 leading-relaxed">
                        {e.account.message}
                      </p>
                      <div className="mt-4 pt-4 border-t border-border/30 flex items-center gap-2 text-xs text-muted-foreground">
                        <span className="flex items-center gap-1">
                          <span>🔑</span>
                          {e.account.owner.toString().slice(0, 4)}...{e.account.owner.toString().slice(-4)}
                        </span>
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            </div>
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="border-t border-border/50 mt-20">
        <div className="container mx-auto px-4 py-6 text-center text-sm text-muted-foreground">
          <p>Built with ⚡ on Solana Devnet</p>
        </div>
      </footer>
    </div>
  );
};

export default Index;
