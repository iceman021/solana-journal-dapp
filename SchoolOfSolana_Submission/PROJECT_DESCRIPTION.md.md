# **Project Description**

**Deployed Frontend URL:** https://sol-journal-two.vercel.app/

**Solana Program ID:** EJTGjYQmVnedbzSTHGoqx67n5Pe4w9hnYa72C8DkBx3t

## **Project Overview**

### **Description**

"Solana Journal" is a decentralized content application built on the Solana blockchain. It allows users to create persistent, on-chain journal entries where they are the sole owners of the data. Unlike Web2 journaling apps, data here is censorship-resistant and stored permanently on the blockchain. This dApp demonstrates core Solana concepts including Program Derived Addresses (PDAs), space allocation, and React-to-Anchor integration.

### **Key Features**

* **Create Entry**: Users can mint a new Journal Entry account by providing a unique title and message.  
* **On-Chain Storage**: All data (Title, Message, Owner) is stored directly in Solana accounts, not in a database.  
* **Duplicate Prevention**: The smart contract logic prevents creating two entries with the same title for the same user.  
* **Wallet Integration**: Seamless connection with Phantom and Solflare wallets via the Solana Wallet Adapter.

### **How to Use the dApp**

1. **Connect Wallet**: Click "Select Wallet" in the top right to connect Phantom/Solflare (Devnet).  
2. **Write Entry**: Enter a unique "Title" and your "Message" in the form.  
3. **Mint**: Click "Mint Entry". This will prompt your wallet to sign a transaction.  
4. **View**: Click "Refresh List" to fetch and display your personal journal entries from the blockchain.

## **Program Architecture**

The Journal dApp uses a PDA-based architecture to manage data. Instead of a single large account holding all entries, every specific entry is its own independent account on the blockchain.

### **PDA Usage**

The program uses **Program Derived Addresses (PDAs)** to deterministically locate entries without needing a centralized database index.

**PDA Seeds:**

* **Journal Entry PDA**: Derived from seeds \[title\_as\_bytes, owner\_public\_key\].  
* **Purpose**: This seed combination ensures that every "Title" is unique per user. If a user tries to create an entry with a title they already used, the PDA collision detection will fail the transaction (Unhappy Path).

### **Program Instructions**

**Instructions Implemented:**

* **create\_entry**: Initializes a new PDA account, sets the owner, and saves the title/message data.  
* **update\_entry**: Allows the owner to overwrite the message of an existing entry.  
* **delete\_entry**: Closes the entry account and refunds the rent (SOL) back to the user.

### **Account Structure**

\#\[account\]  
\#\[derive(InitSpace)\]  
pub struct JournalEntry {  
    pub owner: Pubkey,      // The wallet that owns this entry  
    \#\[max\_len(50)\]  
    pub title: String,      // Unique identifier for this user  
    \#\[max\_len(1000)\]  
    pub message: String,    // The content of the journal  
}

## **Testing**

### **Test Coverage**

I implemented a comprehensive TypeScript test suite (tests/anchor.test.ts) to verify the smart contract logic before frontend integration.

**Happy Path Tests:**

* **Create Entry**: Successfully mints a new PDA with the correct title and message.  
* **Duplicate Prevention:** Smart contract logic prevents reusing the same title.
* **Update Entry**: Verifies that the message can be changed on an existing account.  
* **Delete Entry**: Verifies that the account is closed and data is removed.

**Unhappy Path Tests:**

* **Prevent Duplicate**: Intentionally tries to create an entry with a title that already exists. The test confirms that the program correctly throws an error and rejects the transaction.

### **Running Tests**

\# Run the test suite in Solana Playground or Local Anchor  
anchor test

### **Additional Notes for Evaluators**
 
This project was built using Solana Playground for the Smart Contract and React/Vite for the Frontend.
I encountered significant challenges with the Anchor Provider and Wallet Adapter causing race conditions (specifically the _bn error) during page load. I solved this by implementing a "Raw Transaction" strategy in the frontend. Instead of relying on the Anchor Provider to sign, I manually constructed the transaction instructions using the IDL and sent them via the standard wallet.sendTransaction method. This ensured reliability across different wallet states and prevented common RPC timeouts.
I encountered several challenges with the Anchor Provider and Wallet Adapter race conditions (specifically the \_bn error). I solved this by implementing a "Raw Transaction" strategy in the frontend, manually constructing the transaction and using sendTransaction to ensure reliability across different wallet states.