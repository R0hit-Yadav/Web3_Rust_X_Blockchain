use ethers::{prelude::*, types::Filter};
use std::{env, sync::Arc};
use dotenv::dotenv;
use tokio::time::Duration;
use ethers::types::BlockNumber;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let sepolia_rpc = env::var("SEPOLIA_RPC")?;
    let outbox_contract = env::var("OUTBOX_CONTRACT")?.parse::<Address>()?;
    
    println!("Using outbox contract: {:?}", outbox_contract);
    
    let provider = Arc::new(Provider::<Http>::try_from(sepolia_rpc)?);
    
    listen_for_messages(provider, outbox_contract).await;
    Ok(())
}

// Listen for messages received on Sepolia
async fn listen_for_messages(provider: Arc<Provider<Http>>, outbox_contract: Address) {
    println!("Listening for messages on Sepolia...");

    // Create a filter for the Dispatch event
    let filter = Filter::new()
        .address(outbox_contract)
        .event("Dispatch(uint32,bytes32,uint32,address,uint32,address,bytes)");

    println!("Created filter with event signature: Dispatch(uint32,bytes32,uint32,address,uint32,address,bytes)");

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;

        let latest_block = match provider.get_block_number().await {
            Ok(block) => {
                println!("Latest block: {}", block);
                block
            }
            Err(e) => {
                println!("Error getting latest block: {:?}", e);
                continue;
            }
        };

        let start_block = if latest_block > 10.into() {
            BlockNumber::Number((latest_block - 10).into())
        } else {
            BlockNumber::Earliest
        };

        println!("Checking blocks from {} to latest", start_block);

        match provider.get_logs(&filter.clone()
            .from_block(start_block)
            .to_block(BlockNumber::Latest))
            .await {
                Ok(logs) => {
                    println!("Found {} logs", logs.len());
                    for log in logs {
                        println!("Raw log data: {:?}", log);
                        println!("Topics: {:?}", log.topics);
                        
                        // Try to decode the message data
                        match String::from_utf8(log.data.to_vec()) {
                            Ok(data) => {
                                println!("Received cross-chain message: {}", data);
                                println!("From chain ID: {}", U256::from_big_endian(&log.topics[1].as_ref()));
                                println!("Message ID: {:?}", log.topics[2]);
                                println!("To chain ID: {}", U256::from_big_endian(&log.topics[3].as_ref()));
                                println!("Recipient: {:?}", Address::from_slice(log.topics[4].as_bytes()));
                                println!("Sender: {:?}", Address::from_slice(log.topics[5].as_bytes()));
                                println!("----------------------------------------");
                            }
                            Err(e) => {
                                println!("Error decoding message data: {:?}", e);
                                println!("Raw data: {:?}", log.data);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error getting logs: {:?}", e);
                }
            }
    }
}
