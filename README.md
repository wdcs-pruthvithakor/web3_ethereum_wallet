# Ethereum Wallet CLI

A simple Ethereum wallet command-line interface (CLI) that allows you to generate a new Ethereum wallet, check the balance of an Ethereum address, send Ethereum transactions, and retrieve transaction receipts.

## Features

- **Generate a new Ethereum wallet**: Create a new wallet with a private key and address.
- **Check Ethereum balance**: Fetch the balance of an Ethereum address.
- **Send Ethereum transactions**: Send ETH from one address to another with an estimated gas fee.
- **Get transaction receipt**: Fetch the receipt of a transaction using its hash.
- **Interactive mode**: Enter commands interactively with prompts.

## Installation

To use this Ethereum Wallet CLI, ensure you have Rust and Cargo installed. If not, follow the instructions on [rust-lang.org](https://www.rust-lang.org/learn/get-started).

### Clone the Repository

```bash
git clone https://github.com/wdcs-pruthvithakor/web3_ethereum_wallet.git
cd web3_ethereum_wallet
```

### Build the Project

To build the project, run:

```bash
cargo build --release
```

This will compile the project and create an executable in the `target/release/` directory.

## Running a Local Ethereum Node with Hardhat

You can use Hardhat to set up a local Ethereum node for development and testing. To do this, follow the instructions below:

### Install Hardhat

First, ensure you have Node.js and npm installed. If not, you can download them from [nodejs.org](https://nodejs.org/).

Next, create a new Hardhat project:

```bash
mkdir hardhat-node
cd hardhat-node
npm init -y
npm install --save-dev hardhat
```

### Create Hardhat Project

Initialize a Hardhat project by running:

```bash
npx hardhat
```

This will guide you through setting up your project.

### Start Hardhat Node

Once your project is set up, you can start the Hardhat local Ethereum node with:

```bash
npx hardhat node
```

This will start a local Ethereum network on `http://localhost:8545` by default. You will see some accounts with pre-funded ETH printed to the console.

### Update the CLI to Use Local Node

Now, you can use the local Hardhat node URL in the CLI to interact with the local network. For example:

```bash
./target/release/web3_ethereum_wallet fetch-balance --address <ETHEREUM_ADDRESS> --node-url http://localhost:8545
```

Or:

```bash
./target/release/web3_ethereum_wallet send-transaction --private-key <SENDER_PRIVATE_KEY> --to-address <RECIPIENT_ADDRESS> --amount <ETH_AMOUNT> --node-url http://localhost:8545
```

You can also enter `http://localhost:8545` in **interactive mode** when prompted for a node URL.

## Usage

### Run the Command Line Interface

You can run the CLI as follows:

```bash
./target/release/web3_ethereum_wallet
```

### Available Commands

The Ethereum Wallet CLI supports the following commands:

#### 1. **Generate a new wallet**

To generate a new wallet, use the following command:

```bash
./target/release/web3_ethereum_wallet generate-wallet
```

This will output the Ethereum address and the private key.

#### 2. **Check Ethereum balance**

To check the balance of an Ethereum address, use the following command:

```bash
./target/release/web3_ethereum_wallet fetch-balance --address <ETHEREUM_ADDRESS> --node-url <ETHEREUM_NODE_URL>
```

- `--address`: Ethereum address you want to check the balance for.
- `--node-url`: Ethereum node URL (e.g., Infura, local Hardhat node URL).

#### 3. **Send Ethereum transaction**

To send an Ethereum transaction, use the following command:

```bash
./target/release/web3_ethereum_wallet send-transaction --private-key <SENDER_PRIVATE_KEY> --to-address <RECIPIENT_ADDRESS> --amount <ETH_AMOUNT> --node-url <ETHEREUM_NODE_URL>
```

- `--private-key`: Private key of the sender's Ethereum wallet.
- `--to-address`: Ethereum address of the recipient.
- `--amount`: Amount of ETH to send (in ETH).
- `--node-url`: Ethereum node URL (e.g., Infura, local Hardhat node URL).

#### 4. **Get transaction receipt**

To retrieve the receipt of a transaction using its hash, use the following command:

```bash
./target/release/web3_ethereum_wallet get-transaction-receipt --hash <TRANSACTION_HASH> --node-url <ETHEREUM_NODE_URL>
```

- `--hash`: The transaction hash for which you want to retrieve the receipt.
- `--node-url`: Ethereum node URL (e.g., Infura, local Hardhat node URL).

This command will return the transaction receipt and the transaction status (whether successful or failed).

#### 5. **Interactive Mode**

To enter interactive mode, use the following command:

```bash
./target/release/web3_ethereum_wallet interactive
```

In interactive mode, you can choose from the following options:

1. Generate a new Ethereum wallet.
2. Check Ethereum balance.
3. Send Ethereum transaction.
4. Get transaction receipt.
5. Exit.

Interactive mode will guide you through the required inputs with prompts.

## Example Interactive Mode

```bash
------------------------------------------------------
Select an option:
1. Create a new wallet
2. Check Ethereum balance
3. Send Ethereum transaction
4. Get transaction receipt
5. Exit
Enter your choice: 4
Enter Ethereum node URL:
Enter transaction hash:
------------------------------------------------------
Transaction Receipt:
{ ... }  # Detailed receipt
Transaction Status: Success
------------------------------------------------------
```

### Notes

- **Security**: Always keep your private key secure and never share it with anyone. It's essential to back up your private key in a secure location.
- **Ethereum Node URL**: You can use any public Ethereum node URL like [Infura](https://infura.io/) or connect to your own Ethereum node (e.g., a local Hardhat node on `http://localhost:8545`).

## Dependencies

This project uses the following dependencies:

- `web3` - A Rust library to interact with Ethereum nodes.
- `secp256k1` - Cryptography library for Ethereum's signature scheme.
- `tiny-keccak` - Keccak-256 hash function used for Ethereum address generation.
- `clap` - Command-line argument parser.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
