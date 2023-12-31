mod blockchain;
mod input;
mod parser;
use async_std::io;
use blockchain::{Block, BlockChain};
use colored::*;
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
use std::collections::HashSet;
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
    // Initialise the new peer in the p2p network
    pub fn new() -> P2P {
        // Create a random PeerId
        let id_keys = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(id_keys.public());
        println!(
            "{} {}\n",
            "Local peer id:".truecolor(36, 200, 255),
            local_peer_id.to_string().bright_magenta()
        );

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

        // Create and subscribe to a Gossipsub topic
        let topic = gossipsub::IdentTopic::new("test-net");
        let _ = gossipsub.subscribe(&topic);

        // Create a Swarm to manage peers and events
        let create_swarm = {
            let mdns =
                mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id).unwrap();
            let behaviour = MyBehaviour { gossipsub, mdns };
            SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
        };

        //Initialise new blockchain (if longest chain exists this will be replaced)
        let bc: blockchain::BlockChain = blockchain::BlockChain::new();
        return P2P {
            swarm: create_swarm,
            peers: vec![],
            blockchain: bc,
        };
    }

    // Runs the event loop that handles data transfered around the p2p network
    pub async fn run_task(&mut self) -> Result<(), Box<dyn Error>> {
        let topic = gossipsub::IdentTopic::new("test-net");

        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

        // Listen on all interfaces and whatever port the OS assigns
        self.swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        println!(
            "{} {} {}",
            "Use cmd".truecolor(36, 200, 255),
            "'guide'".yellow().bold(),
            "to view the guide.".truecolor(36, 200, 255),
        );
        println!(
            "{}",
            "Any additions before requesting blockchain will possibly not be saved.\n"
                .red()
                .bold()
        );

        let mut discovered_peers: HashSet<PeerId> = HashSet::new();
        // Kick it off

        loop {
            select! {
                // Handle every line of input from the user
                line = stdin.select_next_some() => {
                    let input = line.unwrap().clone();
                    let trimmed_input = input.trim();
                        let to_publish: Option<String> =  handle_input(&trimmed_input, &self.blockchain, &self.peers);
                        match to_publish {
                            Some(command) => {
                                /*
                                Key:
                                000 - Initiate a blockchain request
                                111+data - Initiate mining new block then publishing it to peers
                                222+data - Initiate retrieving and downloading a file from the blockchain
                                */
                                match &command[0..3] {
                                    //Initiate blockchain request
                                    "000" => {
                                        println!("{}", "Updating blockchain if longer chain exists...".bright_cyan().italic());
                                        if let Err(_) = self.swarm
                                        .behaviour_mut().gossipsub
                                        .publish(topic.clone(), command.as_bytes()) {
                                        //If there is an error in request blockchain, use own blockchain instance
                                            println!("{} {}\n", "!".red().bold(), "No peers, keeping local instance.".truecolor(36, 200, 255));
                                    }
                                },
                                    //Initiate mining new block then publishing it to peers
                                    "111" => {
                                        let split_command = command.split("%").collect::<Vec<&str>>();
                                        let file_name = split_command[1];
                                        let file_path = split_command[2];
                                        //obtain the file type from the file name (.txt for example)
                                        let split_path = &file_path.split('/').collect::<Vec<&str>>();
                                        let full_file = split_path[split_path.len()-1];
                                        let split_file = &full_file.split('.').collect::<Vec<&str>>();
                                        let file_type = split_file[split_file.len()-1];
                                        //accquire the data from the given file path
                                        let data: Option<String> = parser::file_to_string(file_path);
                                        match data {
                                            Some(data) => {
                                                let mined_block: blockchain::Block = blockchain::Block::new(
                                                    self.blockchain.blocks[self.blockchain.blocks.len()-1].id+1,
                                                    file_name.to_string(),
                                                    data,
                                                    file_type.to_string(),
                                                    self.blockchain.blocks[self.blockchain.blocks.len()-1].current_hash.to_owned(),
                                                );
                                                //add error checking to avoid publishing if block is invalid
                                                self.blockchain.add_block(mined_block);
                                                if let Err(_) = self.swarm
                                                .behaviour_mut().gossipsub
                                                .publish(topic.clone(), self.blockchain.blocks[self.blockchain.blocks.len()-1].to_sendable().as_bytes()) {
                                                }
                                            }
                                            _ => {
                                                println!("\n{}\n", "Invalid file path/Unsupported file type, please try again.".red().bold())
                                            }
                                        }
                                },
                                    // Initiate retrieving and downloading a file from the blockchain
                                    "222" => {
                                        let split_command = command.split("%").collect::<Vec<&str>>();
                                        let id = split_command[1];
                                        let path = split_command[2];
                                        //Parse the id to u64
                                        let parsed_id = match id.parse::<u64>() {
                                            Ok(num) => num,
                                            Err(_) => {
                                                println!("\n{}\n", "Invalid ID, please try again.".red().bold());
                                                999999
                                            }
                                        };
                                        if parsed_id != 999999 {
                                            let block = self.blockchain.find_block(parsed_id);
                                            match block {
                                                Some(block) => {
                                                    parser::string_to_file(block.file_data.to_owned(), block.file_name.to_owned(), block.file_type.to_owned(), path.to_string());
                                                }
                                                _ => {
                                                    println!("\n{}\n", "Block does not exist - use the correct ID".red().bold());
                                                }
                                            }
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
                // Handle different events; peer join, peer leave, message recieved
                event = self.swarm.select_next_some() => match event {
                    // Peer discovered
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            if !discovered_peers.contains(&peer_id) {
                                // Peer is not yet discovered, proceed with handling
                                println!("\n{} {} {}\n", "!".red().bold(), "mDNS discovered a new peer:".truecolor(36, 200, 255), peer_id.to_string().bright_purple());
                                self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                                // Add peer to the peer list
                                self.peers.push(peer_id.to_string());
                                // Add the peer ID to the discovered_peers set
                                discovered_peers.insert(peer_id.clone());
                                // Todo: Add any additional actions you want to perform on the newly discovered peer
                            }
                        }
                    },
                    // Peer lost
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            if discovered_peers.contains(&peer_id) {
                                println!("\n{} {} {}\n", "!".red().bold(), "mDNS discover peer has expired:".truecolor(36, 200, 255), peer_id.to_string().bright_purple());
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                            discovered_peers.remove(&peer_id);
                            //Remove peer from peer list
                            let index = self.peers.iter().position(|x| *x == peer_id.to_string());
                            match index {
                                Some(i) => {
                                    self.peers.remove(i);
                                }
                                _ => {}
                            }
                        }
                        }
                    },
                    // Message recieved from a peer
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
                                if let Err(_) = self.swarm
                                .behaviour_mut().gossipsub
                                .publish(topic.clone(), self.blockchain.to_sendable().as_bytes()) {
                                println!("\n{} {}\n", "!".red().bold(), "Error sending local instance of blockchain to joined peer.".truecolor(36, 200, 255));
                            }
                            },
                            "111" => {
                                let temp_block: Block = parser::block_parser(String::from_utf8_lossy(&message.data).to_string());
                                self.blockchain.add_block(temp_block);
                            }
                            "222" => {
                                let temp_bc: BlockChain = parser::blockchain_parser(String::from_utf8_lossy(&message.data).to_string());
                                if temp_bc.blocks.len() > self.blockchain.blocks.len() {
                                    self.blockchain = temp_bc;
                                    println!("{} {}\n", "!".red().bold(), "Blockchain now up-to-date.".truecolor(36, 200, 255))
                                }
                            }
                            _ => {
                                println!(
                                    "Got message: '{}' with id: {id} from peer: {peer_id}",
                                    String::from_utf8_lossy(&message.data),
                                )},
                            }
                        }
                    // SwarmEvent::NewListenAddr { address, .. } => {
                    //     // println!("Local node is listening on {address}");
                    // }
                    _ => {}
                }
            }
        }
    }
}
