use std::error::Error;
use futures::StreamExt;
use libp2p::{core::multiaddr::Multiaddr, identify, noise, swarm::SwarmEvent, tcp, yamux};
use tracing_subscriber::EnvFilter;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            identify::Behaviour::new(identify::Config::new(
                "/ipfs/id/1.0.0".to_string(),
                key.public(),
            ))
        })?
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            // Prints peer id identify info is being sent to.
            SwarmEvent::Behaviour(identify::Event::Sent { peer_id, .. }) => {
                println!("Sent identify info to {peer_id:?}")
            }
            // Prints out the info received via the identify event
            SwarmEvent::Behaviour(identify::Event::Received { info, .. }) => {
                println!("=== Received Peer Identity ===");
                println!("Public Key       : {:?}", info.public_key);
                println!("Protocol Version : {}", info.protocol_version);
                println!("Agent Version    : {}", info.agent_version);
                println!("Listen Addresses :");
                for addr in &info.listen_addrs {
                    println!("  - {}", addr);
                }
                println!("Supported Protocols:");
                for proto in &info.protocols {
                    println!("  - {}", proto);
                }
                println!("Observed Address : {}", info.observed_addr);
                println!("=================================");
            }
            _ => {}
        }
    }
}