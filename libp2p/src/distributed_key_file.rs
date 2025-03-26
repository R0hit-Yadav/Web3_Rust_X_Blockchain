use std::{error::Error, fs, path::Path};
use futures::stream::StreamExt;
use libp2p::{
    kad,
    kad::{store::MemoryStore, Mode, Quorum},
    mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId,
};
use tokio::{
    io::{self, AsyncBufReadExt},
    select,
};
use sled;
use tracing_subscriber::EnvFilter;
use std::collections::HashSet;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    #[derive(NetworkBehaviour)]
    struct Behaviour {
        kademlia: kad::Behaviour<MemoryStore>,
        mdns: mdns::tokio::Behaviour,
    }

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            Ok(Behaviour {
                kademlia: kad::Behaviour::new(
                    key.public().to_peer_id(),
                    MemoryStore::new(key.public().to_peer_id()),
                ),
                mdns: mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?,
            })
        })?
        .build();

    swarm.behaviour_mut().kademlia.set_mode(Some(Mode::Server));

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    println!("Local Peer ID: {}", swarm.local_peer_id());
    println!("PUT_FILE <key> <file_path>: Store a file in the DHT");
    println!("GET_FILE <key>: Retrieve a file from the DHT");
    println!("LIST_FILE: List all files stored in the DHT");
    println!("LIST_PEERS: List all connected peers");


    // Open Sled database
    let db_path = std::env::args().nth(1).unwrap_or_else(|| "file_store/db".to_string());
    let db = sled::open(db_path).expect("Failed to open sled database");

    // Store discovered peers
    let mut peers: HashSet<PeerId> = HashSet::new();

    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                handle_input_line(&mut swarm.behaviour_mut().kademlia, &db, &peers, line);
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address:?}");
                }
                SwarmEvent::Behaviour(BehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, multiaddr) in list {
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                        peers.insert(peer_id);
                    }
                }
                SwarmEvent::Behaviour(BehaviourEvent::Kademlia(kad::Event::OutboundQueryProgressed { result, ..})) => {
                    match result {
                        kad::QueryResult::GetRecord(Ok(kad::GetRecordOk::FoundRecord(kad::PeerRecord {
                            record: kad::Record { key, value, .. },
                            ..
                        }))) => {
                            let filename = format!("retrieved_{}", std::str::from_utf8(key.as_ref()).unwrap());
                            
                            // Save to sled
                            if let Err(e) = db.insert(key.as_ref(), value.clone()) {
                                eprintln!("Failed to save file to sled: {e}");
                            } else {
                                println!("File saved to sled: {:?}", filename);
                            }

                            // Also save as a file
                            if let Err(e) = fs::write(&filename, &value) {
                                // eprintln!("Failed to save file: {e}");
                            } else {
                                println!("File retrieved and saved as: {}", filename);
                            }
                        }
                        kad::QueryResult::GetRecord(Err(err)) => {
                            eprintln!("Failed to get record: {err:?}");
                        }
                        kad::QueryResult::PutRecord(Ok(kad::PutRecordOk { key })) => {
                            println!(
                                "Successfully stored file under key {:?}",
                                std::str::from_utf8(key.as_ref()).unwrap()
                            );
                        }
                        kad::QueryResult::PutRecord(Err(err)) => {
                            eprintln!("Failed to store file: {err:?}");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

//Handles user input commands
fn handle_input_line(kademlia: &mut kad::Behaviour<MemoryStore>,db: &sled::Db,peers: &HashSet<PeerId>,line: String) 
{
    let mut args = line.split_whitespace();

    match args.next() {
        Some("PUT_FILE") => {
            if let (Some(key), Some(file_path)) = (args.next(), args.next()) {
                let path = Path::new(file_path);
                if !path.exists() {
                    eprintln!("File does not exist: {}", file_path);
                    return;
                }

                match fs::read(path) {
                    Ok(file_bytes) => {
                        let record_key = kad::RecordKey::new(&key);
                        let filename = path.file_name().unwrap().to_string_lossy();

                        let file_data = format!("{}|{}", filename, base64::encode(&file_bytes));

                        // Store in sled first
                        if let Err(e) = db.insert(&key, file_data.as_bytes()) {
                            eprintln!("Failed to store file in sled: {e}");
                            return;
                        }

                        // Store in Kademlia network
                        let record = kad::Record {
                            key: record_key.clone(),
                            value: file_bytes,
                            publisher: None,
                            expires: None,
                        };
                        kademlia
                            .put_record(record, Quorum::One)
                            .expect("Failed to store file in DHT.");
                        println!("File stored with key: {}", key);
                    }
                    Err(e) => eprintln!("Error reading file: {}", e),
                }
            } else {
                eprintln!("Usage: PUT_FILE <key> <file_path>");
            }
        }
        Some("GET_FILE") => {
            if let Some(key) = args.next() {
                let record_key = kad::RecordKey::new(&key);

                // Check in sled first
                if let Ok(Some(file_bytes)) = db.get(&key) {
                    let filename = format!("retrieved_{}", key);
                    if let Err(e) = fs::write(&filename, &file_bytes) {
                        eprintln!("Failed to save file: {e}");
                    } else {
                        println!("File retrieved from sled and saved as: {}", filename);
                    }
                } else {
                    println!("File not found in sled, requesting from network...");
                    kademlia.get_record(record_key);
                }
            } else {
                eprintln!("Usage: GET_FILE <key>");
            }
        }
        Some("LIST_FILE") => {
            println!("Stored files:");
            for entry in db.iter() {
                match entry {
                    Ok((key, value)) => {
                        let key_str = String::from_utf8_lossy(&key);
                        
                        // Extract filename from stored value
                        let file_data = String::from_utf8_lossy(&value);
                        let file_name = file_data.split_once('|').map(|(name, _)| name).unwrap_or("Unknown File");

                        println!("Key: {} | File Name: {}", key_str, file_name);
                    }
                    Err(e) => eprintln!("Error reading from DB: {e}"),
                }
            }
        }
        Some("LIST_PEERS") => {
            println!("Connected peers:");
            if peers.is_empty() {
                println!("No peers connected.");
            } else {
                for peer in peers {
                    println!("{peer}");
                }
            }
        }
        _ => {
            eprintln!("Expected commands: PUT_FILE <key> <file_path>, GET_FILE <key>, LIST_FILE, LIST_PEERS");
        }
    }
}
