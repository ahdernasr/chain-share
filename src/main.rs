mod args;
mod blockchain;
use async_std::io;
use args::handle_args;
use args::CLIArgs;
use clap::Parser;
use std::io::{stdout, Write};
use termion::clear;
mod p2p;
use futures::{future::Either, prelude::*, select};
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade},
    gossipsub, identity, mdns, noise,
    swarm::NetworkBehaviour,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};
use libp2p_quic as quic;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;

// We create a custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::async_io::Behaviour,
}
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //TODO, add Windows support
    // Clear the command line (Unix-based Systems)
    print!("{}", clear::All);
    stdout().flush().unwrap();

    let (_p2p_result, _cli_result) = futures::join!(p2p_task(), cli_task());

    Ok(())
}

async fn p2p_task() -> Result<(), Box<dyn Error>> {
        // Create a random PeerId
        let id_keys = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(id_keys.public());
        println!("Local peer id: {local_peer_id}");

        // Set up an encrypted DNS-enabled TCP Transport over the yamux protocol.
        let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1Lazy)
            .authenticate(
                noise::Config::new(&id_keys).expect("signing libp2p-noise static keypair"),
            )
            .multiplex(yamux::Config::default())
            .timeout(std::time::Duration::from_secs(20))
            .boxed();

        let quic_transport = quic::async_std::Transport::new(quic::Config::new(&id_keys));
        let transport = OrTransport::new(quic_transport, tcp_transport)
            .map(|either_output, _| match either_output {
                Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
                Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            })
            .boxed();

        // To content-address message, we can take the hash of message and use it as an ID.
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        // Set a custom gossipsub configuration
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
            .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
            .build()
            .expect("Valid config");

        // build a gossipsub network behaviour
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(id_keys),
            gossipsub_config,
        )
        .expect("Correct configuration");
        // Create a Gossipsub topic
        let topic = gossipsub::IdentTopic::new("test-net");
        // subscribes to our topic
        gossipsub.subscribe(&topic).unwrap();

        // Create a Swarm to manage peers and events
        let mut swarm = {
            let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();
            let behaviour = MyBehaviour { gossipsub, mdns };
            SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
        };

        // Read full lines from stdin
        // let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap()).unwrap();
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

        // Kick it off
        loop {
            select! {
                // PROPOGATES MESSAGE TO ALL PEERS
                // line = stdin.select_next_some() => {
                //     if let Err(e) = swarm
                //         .behaviour_mut().gossipsub
                //         .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()) {
                //         println!("Publish error: {e:?}");
                //     }
                // },
                event = swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}");
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {peer_id}");
                            swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    //HANDLES MESSAGE RECIEPT FROM ANOTHER NODE
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => println!(
                            "Got message: '{}' with id: {id} from peer: {peer_id}",
                            String::from_utf8_lossy(&message.data),
                        ),
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Local node is listening on {address}");
                    }
                    _ => {}
                }
            }
    };
}

async fn cli_task() {
    //TODO, add COLOR and ASCI-ART
    //CLI begins here
    // let args = CLIArgs::parse();
    // handle_args(args);

    // Activate Input Stream
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    loop {
        let mut input = String::new();
        // Read user input asynchronously
        match reader.read_line(&mut input).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break; // Exit loop if no more input
                }
                // Process the user input
                println!("You entered: {}", input.trim());
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

// let mut BC: blockchain::BlockChain = blockchain::BlockChain::new();
// let mined_block: blockchain::Block = blockchain::Block::new(2, "test file".to_string(), "none".to_string(),BC.blocks[0].current_hash.to_owned());
// println!("{:?}", mined_block);
// BC.add_block(mined_block);
// println!("{:?}", BC);
// BC.chain_is_valid(&BC.blocks);