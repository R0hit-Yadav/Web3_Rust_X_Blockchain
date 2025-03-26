# 1️⃣.Ethereum Wallet & Smart Contract Interaction in Rust

This project demonstrates how to create an Ethereum wallet, check balances, send transactions, and 
interact with a smart contract using Rust and the `ethers-rs` and `web3` crates.

## 🚀 Features
- Generate a new Ethereum wallet with a private key.
- Check the wallet's balance.
- Send ETH from a sender wallet to a recipient.
- Interact with a smart contract to store and retrieve values.
- Fetch and display transaction receipts.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
ethers = "2.0"
web3 = "0.18"
dotenv = "0.15"
hex = "0.4"
tokio = { version = "1", features = ["full"] }
```

## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Rust_X_Blockchain.git
cd web3_eth_wallet
```

Create a `.env` file in the project root and add the following:

```ini
ETHEREUM_RPC_URL="https://holesky.infura.io/v3/YOUR_INFURA_PROJECT_ID"
SENDER_PRIVATE_KEY="your_sender_wallet_private_key"
CONTRACT_ADDRESS="your_smart_contract_address"
```

## ▶️ Compile and Run

```sh
cargo run
```

## 📜 How It Works

### 1️⃣ Create a Wallet
- Generates a new Ethereum wallet.
- Displays the wallet address and private key.

### 2️⃣ Connect to an Ethereum Node
- Uses `ETHEREUM_RPC_URL` from `.env` to connect to an Ethereum provider.

### 3️⃣ Check Wallet Balance
- Fetches and prints the balance of the generated wallet.

### 4️⃣ Send an ETH Transaction
- Sends ETH from `SENDER_PRIVATE_KEY` to the generated wallet.
- Uses gas estimation to ensure enough balance is available.

### 5️⃣ Get Transaction Receipt
- Waits for transaction confirmation and displays receipt details.

### 6️⃣ Smart Contract Interaction
- Calls a contract function to store a value (`store(100)`).
- Retrieves the stored value from the contract.

## 🧠 What You Will Learn
- How to generate Ethereum wallets using `ethers-rs`.
- Connecting Rust to an Ethereum node using Web3 providers.
- Fetching account balances and handling gas estimation.
- Sending transactions programmatically with `ethers-rs`.
- Interacting with smart contracts via ABI encoding.

## ⚡ Example Output
```yaml
New Ethereum Wallet Address: 0x1f858a8e0f638d19445a532c2f61b15e48cb649f
Private Key: "7fc26dee638a4963f43deb5a431067bd76b0d67094981e3ea8ded68bd2fab209"

Balance New Wallet: 0.000000000000000000 ETH
Balance Of Sender: 4.199323392839830016 ETH
Transaction sent! Hash: 0x4a428912df27e2733280d549a293594c1c31d9f7214c6d696e057ddb13423b41

Transaction Receipt Generating....
=============================
||==>Transaction Receipt:<===
=============================
||  Transaction Index: 12
||  Transaction Hash: 0x4a428912df27e2733280d549a293594c1c31d9f7214c6d696e057ddb13423b41
||  Block Number: 3388923
||  From: 0xb53d942b79ab480e029220f768cf7b1629acfcf4
||  To: 0x1f858a8e0f638d19445a532c2f61b15e48cb649f
||  Gas Used: 21000
||  Status: "Success"
=============================

Interacting with the smart contract...
Stored Value Transaction Hash: 0x4193618433cf502f1a84eb37d9b6110128df4f008de86ed4c091ce3f79d031b3
Waiting for transaction Confirmation...
Transaction confirmed!
Stored Value in Contract: 100
```

## 📌 Notes
- Always keep your private keys secure and never expose them in public repositories.
- Use a testnet (like infura) for development instead of mainnet to avoid spending real ETH.



# 2️⃣.Ethereum Wallet & Real-time Transaction Tracker in Rust

This project demonstrates how to interact with the Ethereum blockchain using Rust. It allows you to track transactions to a specific Ethereum address in real-time using WebSockets and the `web3` crate.

## 🚀 Features
- Connect to Ethereum via WebSockets.
- Monitor new blocks and fetch transactions.
- Track incoming transactions to a specific Ethereum address.
- Convert transaction values from Gwei to ETH.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
dotenv = "0.15.0"
ethers = "2.0.14"
futures = "0.3.31"
hex = "0.4.3"
serde = "1.0.217"
serde_json = "1.0.138"
tokio = {version = "1.43.0",features = ["full"]}
tokio-stream = "0.1.17"
web3 = "0.19.0"
```

## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Rust_X_Blockchain.git
cd tx_tracker
```

Create a `.env` file in the project root and add the following:

```ini
ETHEREUM_WS_URL="wss://mainnet.infura.io/ws/v3/YOUR_INFURA_PROJECT_ID"
```

## ▶️ Compile and Run

```sh
cargo run
```

## 📜 How It Works

### 1️⃣ Connect to an Ethereum Node
- Uses `ETHEREUM_WS_URL` from `.env` to establish a WebSocket connection.

### 2️⃣ Monitor New Blocks
- Subscribes to new block headers and retrieves block details.

### 3️⃣ Track Transactions to a Specific Address
- Iterates through transactions in each new block.
- Filters transactions matching `TARGET_ADDRESS.`
- Converts transaction values to ETH and logs the details.

## 🧠 What You Will Learn
- Connecting Rust to Ethereum via WebSockets.
- Subscribing to blockchain events in real-time.
- Fetching and filtering transactions using `web3`.
- Handling asynchronous operations with `tokio`.

## ⚡ Example Output
```yaml
Listening for transactions to: 0x3328F7f4A1D1C57c35df56bBf0c9dCAFCA309C49

Block Number: 18834567
Incoming Tx: 0xabc123...
   From: 0xdef456...
   Amount: 0.5 ETH
```

## 📌 Notes
- Always use a WebSocket provider like Infura for real-time updates.
- Keep your .env file secure and never expose API keys.
- Use testnets for development to avoid real ETH expenses.


# 3️⃣.Ethereum Smart Contract Interaction & Event Listen using Rust

This project demonstrates how to interact with an Ethereum smart contract using Rust, `ethers-rs`, and `web3`. It covers connecting to an Ethereum node, checking balances, sending transactions, and listening for events.

## 🚀 Features
- Connect to Ethereum via WebSockets.
- Check the balance of a sender wallet.
- Send transactions to a smart contract.
- Listen Event for and process contract events.
- Retrieve stored values from the contract.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
dotenv = "0.15.0"
ethers = "2.0.14"
futures = "0.3.31"
hex = "0.4.3"
serde = "1.0.217"
serde_json = "1.0.138"
tokio = {version = "1.43.0",features = ["full"]}
tokio-stream = "0.1.17"
web3 = "0.19.0"
```


## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Rust_X_Blockchain.git
cd event_listener
```

## ▶️ Compile and Run

```sh
cargo run
```

## 📜 How It Works

### 1️⃣ Connect to an Ethereum Node
- Uses `ETHEREUM_WS_URL` from `.env` to establish a WebSocket connection.

### 2️⃣ Listens for the NumberUpdatedEvent emitted by the smart contract.

### 3️⃣ Retrieves the updated stored value from the contract.


## 🧠 What You Will Learn
- Connecting Rust to Ethereum via WebSockets.
- Subscribing to Event
- Fetching and filtering transactions of Event

## ⚡ Example Output

- Listening Event 

```yaml
Balance Of Sender: 7.739316169149609092 ETH
Interacting with the smart contract...
Waiting for transaction Confirmation...
Transaction confirmed!

Listening for NumberUpdatedEvent...
Event Received!
Log Index: 485
Transaction Hash: 0xe2e229195297d4828cc917955735dda3c790d7086f670bca9bbd7735892ae8ad
Transaction Index: 20
Sender: 0xb53d942b79ab480e029220f768cf7b1629acfcf4
Block Number: 3415923
Block Hash: 0xe58dfa18a91e12fc1d9192387d2b5f8c52211e050a89b7508b353f111e76dcd2
Event Value in Contract: 2312
```

-Listening Past Events and Values

```yaml
Balance Of Sender: 0.008359136304910111 ETH
Interacting with the smart contract...
Past Event Values
Fetching past events...
========================================
Past events and that values
Transaction Hash: 0xc4f9429dbab339c5741ff8eec21d69b0dc18506e875e73f3223df42731d59cf3
Sender: 0xb53d942b79ab480e029220f768cf7b1629acfcf4
Block Number: 7781487
Stored Value at that Time: 5
========================================
Past events and that values
Transaction Hash: 0xf5246b38e955aa556fffce933a495a393b6027c81eb325835432b4608726954d
Sender: 0xb53d942b79ab480e029220f768cf7b1629acfcf4
Block Number: 7781492
Stored Value at that Time: 10
========================================
Past events and that values
Transaction Hash: 0xe0e8f384a43e9c170952c810015f5271bcde66ef6e6f5f11c63c4bef82950671
Sender: 0xb53d942b79ab480e029220f768cf7b1629acfcf4
Block Number: 7781585
Stored Value at that Time: 5
========================================
.....
```


# 4️⃣.Rust Serialization Benchmark

This project benchmarks the performance of various serialization and deserialization libraries in Rust using the criterion framework. It compares libraries like bincode, bcs, serde_json, borsh, rmp-serde, and protobuf in terms of speed and output size.

## 🚀 Features
- Benchmark serialization speed for multiple Rust libraries.
- Benchmark deserialization speed for the same libraries.
- Measure the size of serialized data for each library.
- Use a custom SampleData struct to simulate real-world data.
- Generate detailed performance reports with criterion.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
bcs = "0.1.6"
bincode = "1.3.3"
borsh = { version = "1.5.5", features = ["derive"] }
prost = { version = "0.13.5", features = ["derive"] }
prost-types = "0.13.5"
protobuf = "3.7.1"
rmp-serde = "1.3.0"
serde = { version = "1.0.218", features = ["derive"] }
serde_json = "1.0.139"

[dev-dependencies]
criterion = "0.5.1"

[build-dependencies]
prost-build = "0.13.5"
```

## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Rust_X_Blockchain.git
cd benchmark
```

## ▶️ Compile and Run

```sh
cargo bench
```

## 📜 How It Works

### 1️⃣ Define Sample Data
- A SampleData struct is defined in lib.rs with fields for id, name, active, and values.
- The sample_data() function generates an instance of this struct for testing.

### 2️⃣ Serialization Benchmarks
- Measures the time taken to serialize SampleData using bincode, bcs, serde_json, borsh, rmp-serde, and protobuf.
- Uses black_box to prevent compiler optimizations from skewing results.

### 3️⃣ Deserialization Benchmarks
- Measures the time taken to deserialize the serialized data back into SampleData for each library.

### 4️⃣ Size Benchmarks
- Calculates and compares the size (in bytes) of the serialized output for each library.

### 5️⃣ Benchmark Grouping
- Uses criterion_group to organize benchmarks into serialization, deserialization, and size categories.
- Reports throughput in bytes for serialization tasks.


## 🧠 What You Will Learn
- How to use the criterion framework for benchmarking in Rust.
- Comparing performance characteristics of popular serialization libraries.
- Measuring throughput and execution time for serialization/deserialization tasks.
- Handling protocol buffers (protobuf) and other binary formats in Rust.
- Structuring a Rust project with benchmarks and dependencies.

## ⚡ Example Output
```yaml
Serialization/bincode serialize   time:   [12.345 µs 12.567 µs 12.789 µs]
Serialization/bcs serialize       time:   [15.678 µs 15.890 µs 16.123 µs]
Serialization/serde_json serialize time: [25.432 µs 25.678 µs 25.901 µs]
...

Deserialization/bincode deserialize time: [10.234 µs 10.456 µs 10.678 µs]
Deserialization/bcs deserialize   time:   [13.567 µs 13.789 µs 14.012 µs]
...

size bincode                      time:   [1.234 ns 1.345 ns 1.456 ns]
size json                         time:   [2.345 ns 2.456 ns 2.567 ns]
...
```

## 📌 Notes
- The SampleData struct in the provided code is minimal (id: 1, name: "H", ...). For more realistic results, consider using larger or more complex data (e.g., the commented-out version in lib.rs).
- Benchmark results may vary based on system specifications and Rust compiler optimizations.
- Ensure you have sufficient memory and CPU resources when running benchmarks with large datasets.


# 5️⃣.Rust Merkle Tree Implementation

This project implements a Merkle Tree in Rust, a data structure commonly used in cryptography and blockchain applications to efficiently verify data integrity. It supports multiple hash functions (SHA-256, SHA-512, and Blake2b) and includes functionality to generate proofs and verify them.

## 🚀 Features
- Construct a Merkle Tree from a list of input leaves (byte vectors).
- Support for multiple hash functions: SHA-256, SHA-512, and Blake2b.
- Compute the Merkle Root of the tree.
- Generate inclusion proofs for specific leaves.
- Verify proofs against the Merkle Root.
- Interactive command-line interface to input leaves and select hash functions.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
blake2 = "0.10.6"
digest = "0.10.7"
hex = "0.4.3"
md-5 = "0.10.6"
ripemd160 = "0.10.0"
sha2 = "0.10.8"
```

## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Rust_X_Blockchain.git
cd merkle_tree
```

## ▶️ Compile and Run

```sh
cargo run
```

## 📜 How It Works

### 1️⃣ Define Hashers
- A Hasher trait is implemented for SHA-256, SHA-512, and Blake2b, allowing flexible hash function selection.

### 2️⃣ Build the Merkle Tree
- Takes a vector of byte arrays (Vec<Vec<u8>>) as leaves.
- Computes hashes for each leaf and iteratively combines them into parent nodes until the root is derived.
- Ensures smaller hashes are placed on the left for consistency.

### 3️⃣ Compute the Merkle Root
- The root hash is calculated by recursively hashing pairs of nodes from the bottom up.

### 4️⃣ Verify Proofs
- Verifies a proof by recomputing the root from a leaf and its proof, comparing it to the provided root.


## 🧠 What You Will Learn
- How to implement a Merkle Tree in Rust with generic hash functions.
- Using traits to abstract over different hash algorithms.
- Generating and verifying cryptographic proofs.
- Working with Rust’s standard library for I/O and byte manipulation.
- Structuring a Rust project with a library (lib.rs) and binary (main.rs).

## ⚡ Example Output
```yaml
Enter number of leaves:
4
Enter data for leaf 1:
apple
Enter data for leaf 2:
banana
Enter data for leaf 3:
cherry
Enter data for leaf 4:
date
Choose hash function 
1.SHA-256
2.SHA-512
3.Blake2b
1

Leaf 1 : "3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea"
Leaf 2 : "e80b5017098950fc58aad83c8c14978e696095b16b7e997b65c57d8e86f9e317"
Leaf 3 : "2b7e151628aed2a6abf7158809cf4f3c762e7160f38b4da56a784d9045190cfea"
Leaf 4 : "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d"

Level 0 (Leaves): ["3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea", "e80b5017098950fc58aad83c8c14978e696095b16b7e997b65c57d8e86f9e317", "2b7e151628aed2a6abf7158809cf4f3c762e7160f38b4da56a784d9045190cfea", "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d"]

Level 1: [("3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea", "e80b5017098950fc58aad83c8c14978e696095b16b7e997b65c57d8e86f9e317"), ("2b7e151628aed2a6abf7158809cf4f3c762e7160f38b4da56a784d9045190cfea", "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d")]

Level 2: [("6e8e228fb3e6532dffa25d7c43f09adbfbf5b621f0a695d8cf0b23bd8e819ae5", "e5c6f3e2e260e575cc4f13e5f6f399abdf5e63e6e8b8c69ebfc2c7f5e3eabf2b")]

Merkle Root: 8f8e2e6b5e2a2f5e7b8c9d4f5a6e7b8c9d4f5a6e7b8c9d4f5a6e7b8c9d4f5a6e

Proof for LEAF 1: [("3f79bb7b435b05321651daefd374cdc681dc06faa65e374e38337b88ca046dea", "e80b5017098950fc58aad83c8c14978e696095b16b7e997b65c57d8e86f9e317"), ("e5c6f3e2e260e575cc4f13e5f6f399abdf5e63e6e8b8c69ebfc2c7f5e3eabf2b")]
Hash: "8f8e2e6b5e2a2f5e7b8c9d4f5a6e7b8c9d4f5a6e7b8c9d4f5a6e7b8c9d4f5a6e"
Proof Verification: true
...
```

## 📌 Notes
- The project assumes input data as UTF-8 strings converted to bytes, but you can modify it to accept raw bytes or other formats.
- For production use, consider adding error handling for invalid inputs or hash computation failures.
- The implementation sorts sibling hashes (smaller on the left) to ensure consistency, which is a common practice in Merkle Trees.


# 6️⃣.🌐Libp2p Peer-to-Peer Networking in Rust

This project showcases a modular and extensible implementation of peer-to-peer (P2P) networking using the libp2p crate in Rust. It includes multiple features like ping, peer identification, chat, distributed key-value storage, and file sharing over a decentralized network.

## 🚀 Features
- Ping: Test connectivity with other peers using the ping protocol.
- Identify: Discover and display peer identity information (e.g., public key, supported protocols).
- Chat: Real-time messaging between peers using the gossipsub protocol.
- Distributed Key-Value Store: Store and retrieve key-value pairs across the network with Kademlia DHT.
- Distributed File Sharing: Share and retrieve files in a decentralized manner, with local persistence using sled.
- Modular design with separate modules for each functionality.

## 📦 Dependencies
Ensure you have Rust installed. You need the following Rust crates:

```toml
[dependencies]
async-std = "1.13.0"
async-trait = "0.1.87"
base64 = "0.22.1"
bincode = {version = "1.3"}
futures = "0.3.31"
libp2p = {version = "0.55.0",features = ["request-response","noise","kad", "ping", "tcp", "tokio", "yamux","mdns","floodsub","macros","gossipsub","identify"]}
serde = {version = "1.0.219",features = ["derive"]}
serde_json = "1.0.140"
sled = "0.34.7"
tokio = {version = "1.44.0",features = ["full"]}
tracing = "0.1.41"
tracing-subscriber = {version = "0.3.19",features = ["env-filter"]}
```

## 🛠 Setup
Clone the repository:

```sh
git clone https://github.com/R0hit-Yadav/Web3_Rust_X_Blockchain.git
cd libp2p
```

## ▶️ Compile and Run

```sh
cargo run

Enter
1. Ping
2. Identify
3. Chat
4. Distributed Key Value
5. Distributed Key File

```

## 📜 How It Works

### 1️⃣ Ping
- Establishes a connection with a peer and measures round-trip time using the ping protocol.
- Listens on all interfaces (/ip4/0.0.0.0/tcp/0) and dials a peer if an address is provided as an argument.

### 2️⃣ Identify
- Discovers peers and retrieves their identity details (e.g., public key, protocol version, listen addresses).
- Useful for debugging and understanding the network topology.

### 3️⃣ Chat
- Implements a gossip-based chat system using gossipsub.
- Discovers local peers with mdns and broadcasts messages to subscribed topics.

### 4️⃣ Distributed Key-Value Store
- Uses Kademlia DHT to store and retrieve key-value pairs across the network.
- Supports commands like PUT <key> <value>, GET <key>, and provider discovery.

### 5️⃣ Distributed File Sharing
- Extends the key-value store to handle file storage and retrieval.
- Stores files locally in a sled database and shares them over the DHT.
- Commands: PUT_FILE <key> <file_path>, GET_FILE <key>, LIST_FILE, LIST_PEERS.


## 🧠 What You Will Learn
- How to build a P2P network using libp2p in Rust.
- Implementing peer discovery with mdns and Kademlia DHT.
- Broadcasting messages with gossipsub.
- Storing and retrieving distributed data with Kademlia.
- Handling file sharing in a decentralized system with local persistence.

## ⚡ Example Output

- Ping 
```yaml
Listening on /ip4/127.0.0.1/tcp/12345
Dialed /ip4/192.168.1.10/tcp/54321
PingEvent { peer: PeerId("12D3KooW..."), result: Ok(PingSuccess { rtt: 12ms }) }
```

- Identify
```yaml
Listening on /ip4/127.0.0.1/tcp/12345
Dialed /ip4/192.168.1.10/tcp/54321
=== Received Peer Identity ===
Public Key       : Ed25519(PublicKey("..."))
Protocol Version : /ipfs/id/1.0.0
Agent Version    : libp2p/0.53.0
Listen Addresses :
  - /ip4/192.168.1.10/tcp/54321
Supported Protocols:
  - /ipfs/ping/1.0.0
  - /ipfs/id/1.0.0
Observed Address : /ip4/127.0.0.1/tcp/12345
=================================
```

- Chat
```yaml
Listening on /ip4/127.0.0.1/tcp/12345
Enter message: Hello, world!
mDNS discovered new peer: 12D3KooW...
=======================
Received Message: 'Hello, world!'
From
Id: 123456789
Source: 12D3KooW...
=======================
```

- Distributed Key-Value
```yaml
Local Peer ID: 12D3KooW...
Listening in /ip4/127.0.0.1/tcp/12345
PUT mykey myvalue
Successfully put record "mykey"
GET mykey
Got record "mykey" "myvalue"
```

- Distributed File Sharing
```yaml
Local Peer ID: 12D3KooW...
Listening on /ip4/127.0.0.1/tcp/12345
PUT_FILE myfile ./example.txt
File stored with key: myfile
GET_FILE myfile
File retrieved and saved as: retrieved_myfile
LIST_FILE
Stored files:
Key: myfile | File Name: example.txt
LIST_PEERS
Connected peers:
12D3KooW...
```

