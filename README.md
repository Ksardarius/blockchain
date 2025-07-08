# **🔗 My Custom Blockchain Project**

A hands-on exploration into the fundamentals of blockchain technology, building a custom Proof-of-Work (PoW) blockchain from the ground up, complemented by a modern web-based wallet.

## **🌟 Introduction**

This project serves as a comprehensive learning exercise, designed to deepen my understanding of distributed ledger technologies, cryptography, and full-stack decentralized application development. It encompasses the core components of a blockchain network and a user-facing wallet, all built with a focus on modularity and future extensibility.  
**Please Note:** This project is for educational purposes only, is currently under active development, and is **not production-ready**. It lacks the robust security, extensive testing, and network resilience required for real-world deployment.

## **✨ Features**

* **Custom Blockchain Implementation:** A foundational blockchain with a Proof-of-Work (PoW) consensus mechanism.  
* **UTXO-Based Transaction Model:** Implements the Unspent Transaction Output (UTXO) model for managing digital asset ownership.  
* **Pay-to-Public-Key-Hash (P2PKH) Scripting:** Basic scripting for secure transaction outputs.  
* **Block Creation & Mining:** Logic for assembling transactions into blocks and performing the PoW mining process.  
* **Local Persistence:** Blocks and UTXO set are stored locally.  
* **RESTful API Node:** An axum-based API layer for blockchain interaction.  
* **Basic P2P Networking (Incomplete):** Initial setup for peer-to-peer communication, with room for significant expansion.  
* **Web-Based Wallet:** A modern React application integrated with Rust WebAssembly (Wasm) for client-side cryptographic operations.  
* **Client-Side Key Management:** Private keys are generated and securely managed within the wallet's browser environment (using IndexedDB).

## **🏗️ Architecture Overview**

This project is structured as a pnpm monorepo, separating concerns into distinct Rust packages and a TypeScript/React frontend.  
.  
├── wallet-crypto/     \# Rust package: Shared cryptographic primitives and blockchain data structures  
├── blockchain/        \# Rust package: Core blockchain logic (chain, blocks, transactions, mining)  
├── node/              \# Rust package: Blockchain API layer (Axum) and P2P networking  
├── wallet-web/        \# Rust package: WebAssembly (Wasm) module for wallet's crypto operations  
└── wallet-ui/         \# TypeScript/React project: Frontend presentation layer for the wallet

### **wallet-crypto (Rust Crate)**

This foundational Rust package houses the shared data structures and cryptographic utilities essential for both the blockchain and the wallet. It defines:

* **BlockchainHash:** Custom hash type for block and transaction identifiers.  
* **TxIn (Transaction Input) & TxOut (Transaction Output):** Core components of the UTXO model.  
* **Signature:** Cryptographic signature types.  
* **PublicKey / PrivateKey:** Key management structures.  
* **PayToPublicKeyHash (P2PKH) Scripting:** Basic locking/unlocking script logic.  
* **Helper Functions:** Cryptography-related utilities used across the project.

This crate ensures consistency and type safety for critical data across the entire system.

### **blockchain (Rust Crate)**

This package encapsulates the core logic of the custom blockchain. It's responsible for:

* **Blockchain Initialization:** Setting up the genesis block.  
* **Block Management:** Adding, validating, and querying blocks.  
* **Mining Logic:** Implementing the Proof-of-Work algorithm to create new blocks.  
* **UTXO Set Management:** Tracking all unspent transaction outputs.  
* **Persistence Layer:** Storing the blockchain data (e.g., to disk).

### **node (Rust Crate)**

Built on the axum web framework, this crate provides the API layer for interacting with the blockchain. It includes:

* **RESTful Endpoints:** For querying blockchain data, submitting transactions, and initiating mining.  
* **Basic P2P Networking:** Initial code for peer discovery and communication is present, though currently commented out and marked for future completion. This lays the groundwork for a decentralized network.

### **wallet-web (Rust Crate \- WebAssembly)**

This Rust package compiles to WebAssembly (Wasm) and forms the cryptographic backend of the web wallet. Its responsibilities include:

* **Key Generation:** Securely generating public and private key pairs client-side.  
* **Transaction Signing:** Using private keys to sign transactions before broadcast.  
* **Local Persistence (IndexedDB):** Securely storing private keys and wallet data within the browser's IndexedDB.  
* **API Call Orchestration:** Facilitating communication between the React frontend and the node API.

### **wallet-ui (TypeScript/React Project)**

This is the user-facing part of the wallet, built with React and TypeScript in a pnpm monorepo. It focuses on:

* **Presentation Logic:** User interface for displaying wallet balance, transaction history, and sending/receiving funds.  
* **User Interaction:** Handling user inputs and displaying feedback.  
* **Integration with wallet-web (Wasm):** Communicating with the Wasm module for all cryptographic and data persistence operations, ensuring private keys never leave the client's browser.

## **🚀 Getting Started**

To run this project locally, you'll need:

* Rust (with wasm-pack installed for the wallet's Wasm compilation)  
* Node.js (LTS version)  
* pnpm

First, clone the repository and navigate into its root directory:  
git clone https://github.com/ksardarius/blockchain.git  
cd blockchain \# Navigate to the root of the cloned repository

Now, follow these steps to set up and run the blockchain node and the web wallet in two separate terminals.

### **Terminal 1: Run the Blockchain Node**

This terminal will run your Rust-based blockchain node, which provides the API for your wallet.  
\# 1\. Navigate to the 'node' directory  
cd node

\# 2\. Run the blockchain node  
cargo run

Leave this terminal running. It will output logs related to block creation, mining, and API requests.

### **Terminal 2: Set Up and Run the Web Wallet**

This terminal will handle the setup and execution of your React-based web wallet.  
\# 1\. Navigate to the 'wallet-ui' directory  
cd wallet-ui

\# 2\. Install JavaScript dependencies for the wallet  
pnpm install

\# 3\. Navigate back to the monorepo root to build the Rust WebAssembly module  
cd .. \# Go back to the 'blockchain' root directory

\# 4\. Build the Rust WebAssembly module for the wallet  
pnpm wasm build

\# 5\. Build the React application (wallet-ui)  
pnpm build

\# 6\. Navigate back to the 'wallet-ui' directory to start the development server  
cd wallet-ui

\# 7\. Start the wallet web application  
pnpm app dev

The wallet should now be accessible in your browser, typically at http://localhost:5173. The node API will be running on a different port, e.g., http://localhost:3000.

## **🛣️ Future Improvements**

This project lays a solid foundation, but there are many exciting avenues for future development:

* **Complete P2P Networking:** Implement full peer discovery, block propagation, transaction relay, and robust network synchronization.  
* **Advanced Consensus:** Explore and implement more sophisticated consensus mechanisms like full Proof-of-Stake (PoS) or Byzantine Fault Tolerance (BFT) variants.  
* **Smart Contract Support:** Integrate a virtual machine (e.g., EVM-compatible or a custom WASM-based VM) to enable programmable smart contracts.  
* **Improved UTXO Selection:** Implement more advanced coin selection algorithms (e.g., Branch and Bound, Random-Improve) to optimize for fees, privacy, and UTXO management.  
* **Enhanced Cryptography:** Add support for multi-signature transactions, Schnorr signatures, or other advanced cryptographic schemes.  
* **Wallet Feature Expansion:**  
  * Comprehensive transaction history with detailed views.  
  * Multi-account management.  
  * Improved UI/UX for a more intuitive user experience.  
  * Address book functionality.  
* **Testing Suite:** Implement extensive unit, integration, and end-to-end tests to ensure robustness and correctness.  
* **Security Audits & Hardening:** Conduct thorough security reviews and implement best practices for production-grade security.  
* **Performance Optimization:** Further optimize transaction processing, block validation, and network communication for higher throughput.  
* **Cross-Chain Interoperability:** Explore mechanisms for connecting this blockchain with other networks.

## **⚠️ Disclaimer**

This project is a personal learning endeavor and is not intended for production use. It has not undergone security audits, and its cryptographic implementations are simplified for clarity. Do not use it for real-world value.

## **📄 License**

This project is licensed under the MIT License. See the LICENSE file for details.

## **👤 Author**

\[Mihails Orlovs\]
\[mihails.orlovs@gmail.com\]