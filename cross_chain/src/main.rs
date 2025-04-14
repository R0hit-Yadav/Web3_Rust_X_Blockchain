use ethers::{
    prelude::*,
    signers::{LocalWallet, Signer},
    types::{TransactionRequest, Bytes, U256},
};
use std::{env, sync::Arc};
use tokio::time::Duration;
use dotenv::dotenv;
use hex;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let holesky_rpc = env::var("HOLESKY_RPC")?;
    let sepolia_rpc = env::var("SEPOLIA_RPC")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let inbox_contract = env::var("INBOX_CONTRACT")?.parse::<Address>()?;
    
    println!("Using inbox contract: {:?}", inbox_contract);
    
    // Load signer and provider
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(17000u64);
    let provider = Provider::<Http>::try_from(holesky_rpc)?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Send cross-chain message
    send_cross_chain_message(client, inbox_contract).await?;
    
    Ok(())
}

// Send a message from Holesky to Sepolia via Hyperlane
async fn send_cross_chain_message(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, 
    inbox_contract: Address
) -> eyre::Result<()> {
    let target_chain = 11155111; // Sepolia Chain ID
    let recipient = "0x40C34974068CBe7Ef930e4585a68740a7aee2B89".parse::<Address>()?;
    let message = "Hello Sepolia!".as_bytes();

    println!("Sending cross-chain message from Holesky to Sepolia...");
    println!("Sender address: {:?}", client.address());
    println!("Recipient address: {:?}", recipient);
    println!("Target chain ID: {}", target_chain);

    // Create the Hyperlane message payload
    let mut payload = Vec::new();
    
    // Add version (1 byte)
    payload.push(0x01);
    
    // Add nonce (32 bytes)
    let mut nonce = [0u8; 32];
    U256::from(0).to_big_endian(&mut nonce);
    payload.extend(nonce);
    
    // Add origin chain ID (32 bytes)
    let mut origin_chain_bytes = [0u8; 32];
    U256::from(17000).to_big_endian(&mut origin_chain_bytes); // Holesky chain ID
    payload.extend(origin_chain_bytes);
    
    // Add sender address (32 bytes)
    let sender = client.address();
    let mut sender_bytes = [0u8; 32];
    sender_bytes[12..].copy_from_slice(sender.as_bytes());
    payload.extend(sender_bytes);
    
    // Add destination chain ID (32 bytes)
    let mut dest_chain_bytes = [0u8; 32];
    U256::from(target_chain).to_big_endian(&mut dest_chain_bytes);
    payload.extend(dest_chain_bytes);
    
    // Add recipient address (32 bytes)
    let mut recipient_bytes = [0u8; 32];
    recipient_bytes[12..].copy_from_slice(recipient.as_bytes());
    payload.extend(recipient_bytes);
    
    // Add message body
    payload.extend(message);

    println!("Message payload length: {} bytes", payload.len());
    println!("Message payload: {:?}", hex::encode(&payload));

    let encoded_message = Bytes::from(payload);

    let tx = TransactionRequest::new()
        .to(inbox_contract)
        .data(encoded_message)
        .value(U256::from(0))
        .gas(500000); // Increased gas limit for cross-chain message

    println!("Sending transaction to inbox contract...");
    let pending_tx = client.send_transaction(tx, None).await?;
    let receipt = pending_tx.await?;
    
    if let Some(receipt) = receipt {
        println!("Transaction Hash: {:?}", receipt.transaction_hash);
        println!("Block Number: {:?}", receipt.block_number);
        println!("Gas Used: {:?}", receipt.gas_used);
        println!("Message sent successfully!");
    } else {
        println!("Transaction receipt is None");
    }
    Ok(())
}
