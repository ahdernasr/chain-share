use crate::p2p::blockchain::BlockChain;
use colored::*;
// This function takes in the user input and handles it, if there is something to be published then that is returned
pub fn handle_input<'a>(
    input: &'a str,
    blockchain: &BlockChain,
    peers: &Vec<String>,
) -> Option<String> {
    // Splits input into multiple parameters
    let input_iterator: Vec<&str> = input.split(' ').collect();
    let mut iterator = input_iterator.iter();

    //Iterating through parameters, checking if they exist first through Option types
    match iterator.next() {
        Some(option) => {
            match option {
                &"request" => match iterator.next() {
                    Some(option) => match option {
                        &"blockchain" => {
                            return Some("000".to_string());
                        }
                        _ => {
                            println!(
                                "{} {} {}",
                                "Invalid command:".red(),
                                "'request blockchain'".yellow().bold(),
                                "to request the longest chain.".red().bold()
                            );
                        }
                    },
                    _ => {
                        println!(
                            "{} {} {}",
                            "Invalid command:".red().bold(),
                            "'request blockchain'".yellow().bold(),
                            "to request the longest chain.".red().bold()
                        );
                    }
                },
                &"view" => {
                    match iterator.next() {
                        Some(option) => {
                            match option {
                                &"blocks" => {
                                    //show number of blocks in blockchain and info of all blocks
                                    println!("{}", blockchain.to_viewable());
                                }
                                &"peers" => {
                                    //show peers ID list and number of peers
                                    println!("{} {}","Number of peers:".cyan(), peers.len().to_string().bright_purple());
                                    println!("{} {}", "Peers List:".cyan(), peers.join(" ").bright_purple());
                                }
                                _ => {
                                    println!(
                                        "{} {} {}",
                                        "Invalid command:".red().bold(),
                                        "'view help'".yellow().bold(),
                                        "for more info.".red().bold()
                                    );
                                }
                            }
                        }
                        _ => {
                            println!(
                                "{} {} {}",
                                "Invalid command:".red().bold(),
                                "'view help'".yellow().bold(),
                                "for more info.".red().bold()
                            );
                        }
                    }
                }
                &"upload" => match iterator.next() {
                    Some(option) => {
                        let file_name = option;
                        match iterator.next() {
                            Some(option) => {
                                let file_path = option;
                                let string: String = format!("111%{}%{}", &file_name, &file_path);
                                return Some(string.clone());
                            }
                            _ => {
                                println!(
                                    "{} {} {}",
                                    "Invalid command:".red().bold(),
                                    "'upload help'".yellow().bold(),
                                    "for more info.".red().bold()
                                );
                            }
                        }
                    }
                    _ => {
                        println!(
                            "{} {} {}",
                            "Invalid command:".red().bold(),
                            "'upload help'".yellow().bold(),
                            "for more info.".red().bold()
                        );
                    }
                },
                &"retrieve" => match iterator.next() {
                    Some(option) => {
                        let id = option;
                        match iterator.next() {
                            Some(option) => {
                                let string: String = format!("222%{}%{}", id, option);
                                return Some(string.clone());
                            }
                            _ => {
                                println!(
                                    "{} {} {}",
                                    "Invalid command:".red().bold(),
                                    "'retrieve help'".yellow().bold(),
                                    "for more info.".red().bold()
                                );
                            }
                        }
                    }
                    _ => {
                        println!(
                            "{} {} {}",
                            "Invalid command:".red(),
                            "'retrieve help'".yellow().bold(),
                            "for more info.".red().bold()
                        );
                    }
                },
                _ => {
                    println!("{}", "Invalid input - refer to guide:".red().bold());
                }
            }
        }
        _ => {
            println!("{}", "Invalid input - refer to guide:".red().bold());
        }
    }
    return None;
}
