# NexGuard - Decentralized Insurance Platform

**NexGuard** is a decentralized insurance platform for DeFi users, leveraging AI for real-time risk assessment and automated claims processing. Built on the Solana blockchain, NexGuard combines advanced machine learning with decentralized physical infrastructure (DePIN) to ensure secure and efficient insurance services. Key features include policy purchase, staking, insurance pools, and AI-powered claims verification.

## Project Overview

- **Blockchain**: Solana
- **Smart Contract Language**: Rust
- **Backend**: Node.js - TBA
- **Frontend**: React - TBA
- **AI Models**: TensorFlow/PyTorch for risk assessment - TBA

## Features

- **Policy Purchase**: Users can select and purchase insurance policies.
- **Staking and Pooling**: Users can stake assets in insurance pools.
- **Automated Claims**: Real-time claims processing with AI-powered verification.
- **Secure Data Storage**: Decentralized physical infrastructure for data integrity.

---

## Prerequisites

Before you begin, ensure you have the following installed on your machine:

1. **Solana CLI**: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
2. **Anchor CLI**: [Install Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)
3. **Node.js** (for backend and frontend setup): [Install Node.js](https://nodejs.org/)
4. **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
5. **Git**: [Install Git](https://git-scm.com/)

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone https://github.com/your-username/nexguard.git
   cd nexguard
   ```

2. **Install Dependencies**
   ```bash
   cd programs/nexguard_insurance
   npm install
   ```

3. **Set Up Solana Configuration**
   ```bash
   solana config set --url https://rpc.devnet.soo.network/rpc
   ```

4. **Build the Project**
   ```bash
   anchor build
   ```

---

## Deploying the Contract to Solana Devnet

Follow these steps to deploy the `NexGuard` contract on Solana Devnet.

1. **Generate a New Keypair** (if needed)
   ```bash
   solana-keygen new --outfile ~/my-solana-keypair.json
   ```

2. **Fund Your Devnet Account**  
   Get SOL for the Devnet by using Solana's [faucet](https://solfaucet.com/).

3. **Build and Deploy the Contract**
   ```bash
   anchor build
   anchor deploy
   ```

4. **Confirm Deployment**  
   After deployment, confirm the program ID. Anchor will display the deployed program's public key. Save this ID to use in your frontend and backend configurations.

---

## Testing the Smart Contract

1. **Set Up Local Validator**
   ```bash
   solana-test-validator
   ```

2. **Run Anchor Tests**
   ```bash
   anchor test
   ```

