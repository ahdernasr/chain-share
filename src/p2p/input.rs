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
                &"guide" => {
                    println!("{}", "request:".yellow().bold());
                    println!("      {}{}", "'request blockchain'".yellow(), ": request the longest chain from peers.".cyan());
                    println!("{}", "view:".yellow().bold());
                    println!("      {}{}", "'view blocks'".yellow(), ": view all the blocks (files) in the blockchain network.".cyan());
                    println!("      {}{}", "'view peers'".yellow(), ": view all the peers in the network.".cyan());
                    println!("{}", "upload:".yellow().bold());
                    println!("      {}{}", "'upload <file name> <absolute file path>'".yellow(), ": mine a block and upload a file into the blockchain network.".cyan());
                    println!("{}", "retrieve:".yellow().bold());
                    println!("      {}{}\n", "'retrieve <block id> <absolute file path>'".yellow(), ": download a file from the blockchain network into a path on your computer.".cyan());
                },
                &"request" => match iterator.next() {
                    Some(option) => match option {
                        &"blockchain" => {
                            return Some("000".to_string());
                        }
                        _ => {
                            println!(
                                "\n{} {} {}\n",
                                "Invalid command - use".red().bold(),
                                "'guide'".yellow().bold(),
                                "to view the guide.".red().bold(),
                            );
                        }
                    },
                    _ => {
                        println!(
                            "\n{} {} {}\n",
                            "Invalid command - use".red().bold(),
                            "'guide'".yellow().bold(),
                            "to view the guide.".red().bold(),
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
                                    println!("\n{} {}","Number of peers:".truecolor(36, 200, 255), peers.len().to_string().bright_purple());
                                    println!("{} {}\n", "Peers List:".truecolor(36, 200, 255), peers.join(" ").bright_purple());
                                },
                                _ => {
                                    println!(
                                        "\n{} {} {}\n",
                                        "Invalid command - use".red().bold(),
                                        "'guide'".yellow().bold(),
                                        "to view the guide.".red().bold(),
                                    );
                                }
                            }
                        }
                        _ => {
                            println!(
                                "\n{} {} {}\n",
                                "Invalid command - use".red().bold(),
                                "'guide'".yellow().bold(),
                                "to view the guide.".red().bold(),
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
                                    "\n{} {} {}\n",
                                    "Invalid command - use".red().bold(),
                                    "'guide'".yellow().bold(),
                                    "to view the guide.".red().bold(),
                                );
                            }
                        }
                    }
                    _ => {
                        println!(
                            "\n{} {} {}\n",
                            "Invalid command - use".red().bold(),
                            "'guide'".yellow().bold(),
                            "to view the guide.".red().bold(),
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
                                    "\n{} {} {}\n",
                                    "Invalid command - use".red().bold(),
                                    "'guide'".yellow().bold(),
                                    "to view the guide.".red().bold(),
                                );
                            }
                        }
                    }
                    _ => {
                        println!(
                            "\n{} {} {}\n",
                            "Invalid command - use".red().bold(),
                            "'guide'".yellow().bold(),
                            "to view the guide.".red().bold(),
                        );
                    }
                },
                _ => {
                    println!(
                        "\n{} {} {}\n",
                        "Invalid command - use".red().bold(),
                        "'guide'".yellow().bold(),
                        "to view the guide.".red().bold(),
                    );
                }
            }
        }
        _ => {
            println!(
                "\n{} {} {}\n",
                "Invalid command - use".red().bold(),
                "'guide'".yellow().bold(),
                "to view the guide.".red().bold(),
            );
        }
    }
    return None;
}
