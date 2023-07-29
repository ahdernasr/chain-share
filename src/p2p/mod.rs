mod blockchain;
mod input;
mod parser;
use async_std::io;
use blockchain::{Block, BlockChain};
use futures::{future::Either, prelude::*, select};
use input::handle_input;
use libp2p::Transport;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade},
    gossipsub, identity, mdns, noise,
    swarm::NetworkBehaviour,
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId,
};
use libp2p_quic as quic;
use rand::Rng;
use std::error::Error;
use std::time::Duration;

// Peer to peer object that controls most of the p2p network and blockchain functionality
pub struct P2P {
    pub swarm: Swarm<MyBehaviour>,
    pub peers: Vec<String>,
    pub blockchain: BlockChain,
}
// Custom network behaviour that combines Gossipsub and Mdns.
#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::async_io::Behaviour,
}

impl P2P {
    pub fn new() -> P2P {
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

        //Creates a random message id for each message (ID is no longer tied to message value to allow duplicate message)
        fn message_id_fn(_message: &gossipsub::Message) -> gossipsub::MessageId {
            let mut rng = rand::thread_rng();
            let random_id: String = (0..16).map(|_| rng.gen::<u8>().to_string()).collect();
            return gossipsub::MessageId::from(random_id);
        }

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
        let _ = gossipsub.subscribe(&topic);

        // Create a Swarm to manage peers and events
        let create_swarm = {
            let mdns =
                mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();
            let behaviour = MyBehaviour { gossipsub, mdns };
            SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
        };

        //Initialise new blockchain (if longest chain exists this will be replaced)
        let mut bc: blockchain::BlockChain = blockchain::BlockChain::new();
        return P2P {
            swarm: create_swarm,
            peers: vec![],
            blockchain: bc,
        };
    }

    pub async fn run_task(&mut self) -> Result<(), Box<dyn Error>> {
        let topic = gossipsub::IdentTopic::new("test-net");
        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

        // Listen on all interfaces and whatever port the OS assigns
        self.swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        println!("Use cmd 'request blockchain' to request longest chain from peers");

        // Kick it off
        loop {
            select! {
                line = stdin.select_next_some() => {
                    let input = line.unwrap().clone();
                    let to_publish: Option<String> =  handle_input(&input, &self.blockchain, &self.peers);
                    match to_publish {
                        Some(command) => {
                            match &command[0..3] {
                                //Initiate blockchain request
                                "000" => {
                                    if let Err(e) = self.swarm
                                    .behaviour_mut().gossipsub
                                    .publish(topic.clone(), command.as_bytes()) {
                                    //If there is an error in request blockchain, use own blockchain instance
                                    if input == "request blockchain" {
                                        println!("No peers, Creating own blockchain");
                                        println!("{:?}", self.blockchain);
                                    } else {
                                        println!("Publish error: {e:?}");
                                    }
                                }
                                //Initiate adding mining new block then publishing it
                            },
                                "111" => {
                                    let split_command = command.split("%").collect::<Vec<&str>>();
                                    let file_name = split_command[1];
                                    let file_path = split_command[2];
                                    let mined_block: blockchain::Block = blockchain::Block::new(
                                        self.blockchain.blocks[self.blockchain.blocks.len()-1].id+1,
                                        file_name.to_string(),
                                        file_path.to_string(),
                                        self.blockchain.blocks[self.blockchain.blocks.len()-1].current_hash.to_owned(),
                                    );
                                    //add error checking to avoid publishing if block is invalid
                                    self.blockchain.add_block(mined_block);
                                    if let Err(e) = self.swarm
                                    .behaviour_mut().gossipsub
                                    .publish(topic.clone(), self.blockchain.blocks[self.blockchain.blocks.len()-1].to_sendable().as_bytes()) {
                                        println!("Publish error: {e:?}");
                                    }
                            },
                            _ => {

                            }
                            //create a nested match statement here based on the command
                            // publish commands:
                            //  upload
                            //  request blockchain
                        }
                        }
                        _ =>{}
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discovered a new peer: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                            //Add peer to peer list
                            self.peers.push(peer_id.to_string())
                        }
                        //Todo - Find a better fix
                        //Requests blockchain in a loop Loop is used because the first iterations might not work due to a peer error
                        // loop {
                        //     task::sleep(Duration::from_secs(10)).await;
                        //     if let Err(e) = self.swarm
                        //     .behaviour_mut().gossipsub
                        //     .publish(topic.clone(), "Requesting Blockchain".as_bytes()) {
                        //     println!("Publish error: {e:?}");
                        //     } else {
                        //         break;
                        //     }
                        // }
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("mDNS discover peer has expired: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                            //Remove peer from peer list
                            let index = self.peers.iter().position(|x| *x == peer_id.to_string()).unwrap();
                            self.peers.remove(index);
                        }
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => {
                            /*
                            Key:
                            Prefix 000 - Blockchain request receipt
                            Prefix 111 - Block mined and added update
                            Prefix 222 - Blockchain request answered and Blockchain sent
                            */
                        match &String::from_utf8_lossy(&message.data).as_ref()[0..3] {
                            //Handle a request for longest chain from peer
                            "000" => {
                                println!("Longest chain requested...");
                                println!("Sending local instance of blockchain.");
                                if let Err(e) = self.swarm
                                .behaviour_mut().gossipsub
                                .publish(topic.clone(), self.blockchain.to_sendable().as_bytes()) {
                                println!("Publish error: {e:?}");
                            }
                            },
                            "111" => {
                                println!("Block recieved");
                                let temp_block: Block = parser::block_parser(String::from_utf8_lossy(&message.data).to_string());
                                self.blockchain.add_block(temp_block);
                                //if block is invalid, could indicate that local blockchain instance is outdated,
                                //so spam prompt user to request blockchain
                            }
                            "222" => {
                                println!("Blockchain recieved!");
                                let temp_bc: BlockChain = parser::blockchain_parser(String::from_utf8_lossy(&message.data).to_string());
                                if temp_bc.blocks.len() > self.blockchain.blocks.len() {
                                    self.blockchain = temp_bc;
                                }
                                //Parse into a blockchain object
                                //If blockchain.length is bigger than the local instance, update it
                                //Confirm blockchain update
                            }
                            _ => {
                                println!(
                                    "Got message: '{}' with id: {id} from peer: {peer_id}",
                                    String::from_utf8_lossy(&message.data),
                                )},
                            }
                        }
                        // Message types:
                        // 1. Blockchain request -> Publish blockchain
                        // 2. New block added -> Add blockchain to local instance of block chain
                    // SwarmEvent::NewListenAddr { address, .. } => {
                    //     // println!("Local node is listening on {address}");
                    // }
                    _ => {}
                }
            }
        }
    }

    // pub fn get_peers_count(&self) -> u32 {
    // self.peers
    // }
}
