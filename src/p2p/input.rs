use crate::p2p::blockchain::BlockChain;
// This function takes in the user input and handles it, if there is something to be published then that is returned
pub fn handle_input<'a>(
    input: &'a str,
    blockchain: &BlockChain,
    peers: &Vec<String>,
) -> Option<String> {
    // Splits input into multiple parameters
    // Note: Always show selection
    let input_iterator: Vec<&str> = input.split(' ').collect();
    let mut iterator = input_iterator.iter();

    //Iterating through parameters, checking if they exist first through Some
    match iterator.next() {
        Some(option) => {
            match option {
                &"request" => match iterator.next() {
                    Some(option) => match option {
                        &"blockchain" => return Some("000".to_string()),
                        _ => {
                            println!("Invalid command - use 'request blockchain to request the longest chain'");
                        }
                    },
                    _ => {
                        println!("Invalid command - use 'request blockchain to request the longest chain'");
                    }
                },
                &"view" => {
                    match iterator.next() {
                        Some(option) => {
                            match option {
                                &"blocks" => {
                                    println!("{:?}", blockchain.to_viewable());
                                } //show number of blocks in blockchain and info of all blocks
                                &"peers" => {
                                    println!("Number of peers: {}", peers.len());
                                    println!("Peer List: {:?}", peers);
                                } //show peers ID list and number of peers
                                _ => {
                                    println!("Invalid command: 'view help' for more info");
                                }
                            }
                        }
                        _ => {
                            println!("Invalid command: 'view help' for more info");
                        }
                    }
                }
                &"upload" => {
                    match iterator.next() {
                        Some(option) => {
                            let file_name = option;
                            match iterator.next() {
                                Some(option) => {
                                    let file_path = option;
                                    let string: String =
                                        format!("111%{}%{}", &file_name, &file_path);
                                    return Some(string.clone());
                                }
                                _ => {
                                    //invalid command, view 'upload' help
                                }
                            }
                        }
                        _ => {
                            //invalid command, view 'upload' help
                        }
                    }
                }
                &"retrieve" => {
                    match iterator.next() {
                        Some(option) => {
                            let hash = option;
                        }
                        _ => {
                            //invalid command, view 'upload' help
                        }
                    }
                }
                &"info" => {} //show info about app
                _ => {
                    println!("Invalid input - refer to guide.")
                }
            }
        }
        _ => {
            println!("Invalid input - refer to guide.")
        }
    }
    return None;
}
