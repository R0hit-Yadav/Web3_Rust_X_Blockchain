use async_trait::async_trait;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    identity,
    noise,
    request_response::{
        Behaviour as RequestResponseBehaviour, Codec, Config, Event, Message,
        ProtocolSupport,
    },
    SwarmBuilder,
    swarm:: SwarmEvent,
    tcp,
    yamux, PeerId, Transport,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use std::{error::Error, io};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

/// Define the protocol name.
#[derive(Debug, Clone)]
struct MyProtocol;
impl AsRef<str> for MyProtocol {
    fn as_ref(&self) -> &str {
        "/my_protocol/1.0.0"
    }
}

/// Define request and response types.
#[derive(Debug, Serialize, Deserialize)]
struct MyRequest {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyResponse {
    pub message: String,
}

/// Implement the custom Codec trait.
#[derive(Debug, Clone, Default)]
struct MyCodec;

#[async_trait]
impl Codec for MyCodec {
    type Protocol = MyProtocol;
    type Request = MyRequest;
    type Response = MyResponse;

    async fn read_request<T: AsyncRead + AsyncReadExt + Unpin + Send>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> Result<Self::Request, io::Error> {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        serde_json::from_slice(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    async fn read_response<T: AsyncRead + AsyncReadExt + Unpin + Send>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> Result<Self::Response, io::Error> {
        let mut buf = Vec::new();
        io.read_to_end(&mut buf).await?;
        serde_json::from_slice(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    async fn write_request<T: AsyncWrite + AsyncWriteExt + Unpin + Send>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> Result<(), io::Error> {
        let data = serde_json::to_vec(&req).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        io.write_all(&data).await?;
        io.flush().await
    }

    async fn write_response<T: AsyncWrite + AsyncWriteExt + Unpin + Send>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> Result<(), io::Error> {
        let data = serde_json::to_vec(&res).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        io.write_all(&data).await?;
        io.flush().await
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("Local Peer ID: {local_peer_id}");

    // Transport with TCP, Noise encryption, and Yamux multiplexing
    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();

    // Configure the request-response behavior
    let behaviour = RequestResponseBehaviour::with_codec(
        MyCodec::default(),
        vec![(MyProtocol, ProtocolSupport::Full)],
        Config::default(),
    );

    let mut swarm = SwarmBuilder::with_new_identity().build();

    // Start listening on a random TCP port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {address:?}");
            }
            SwarmEvent::Behaviour(Event::Message { peer, message, .. }) => match message {
                Message::Request { request, channel, .. } => {
                    info!("Received request: {:?}", request);
                    let response = MyResponse {
                        message: format!("Hello, you sent: {}", request.data),
                    };
                    swarm.behaviour_mut().send_response(channel, response).unwrap();
                }
                Message::Response { response, .. } => {
                    info!("Received response: {:?}", response);
                }
            },
            SwarmEvent::Behaviour(Event::OutboundFailure { peer, error, request_id, .. }) => {
                warn!("Outbound failure with {peer:?}: {error:?}, request_id: {request_id:?}");
            }
            SwarmEvent::Behaviour(Event::InboundFailure { peer, error, request_id, .. }) => {
                warn!("Inbound failure with {peer:?}: {error:?}, request_id: {request_id:?}");
            }
            _ => {}
        }
    }
}
