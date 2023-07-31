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
                    println!("\n{}", "request:".yellow().bold());
                    println!(
                        "      {}{}",
                        "'request blockchain'".yellow(),
                        ": request the longest chain from peers.".truecolor(36, 200, 255)
                    );
                    println!("{}", "view:".yellow().bold());
                    println!(
                        "      {}{}",
                        "'view blocks'".yellow(),
                        ": view all the blocks (files) in the blockchain network.".truecolor(36, 200, 255)
                    );
                    println!(
                        "      {}{}",
                        "'view peers'".yellow(),
                        ": view all the peers in the network.".truecolor(36, 200, 255)
                    );
                    println!(
                        "      {}{}",
                        "'view types'".yellow(),
                        ": view all supported file types for data upload and download.".truecolor(36, 200, 255)
                    );
                    println!("{}", "upload:".yellow().bold());
                    println!(
                        "      {}{}",
                        "'upload <file name> <absolute file path>'".yellow(),
                        ": mine a block and upload a file into the blockchain network.".truecolor(36, 200, 255)
                    );
                    println!("{}", "retrieve:".yellow().bold());
                    println!("      {}{}\n", "'retrieve <block id> <absolute file path>'".yellow(), ": download a file from the blockchain network into a path on your computer.".truecolor(36, 200, 255));
                }
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
                                    println!(
                                        "\n{} {}",
                                        "Number of peers:".truecolor(36, 200, 255),
                                        peers.len().to_string().bright_purple()
                                    );
                                    println!(
                                        "{} {}\n",
                                        "Peers List:".truecolor(36, 200, 255),
                                        peers.join(" ").bright_purple()
                                    );
                                }
                                &"types" => {
                                    //shows supported file types
                                    println!("\n{}", "This program supports all UTF-8 encoded file types, including: ".yellow().bold());
                                    println!("{}\n      {}", "Plain Text Files: ".yellow(), "Files containing plain text such as '.txt' filesm configuration files, logs, etc.".truecolor(36, 200, 255));
                                    println!("{}\n      {}", "Source code files: ".yellow(), "Source code files for programming languages like .rs (Rust), .c (C), .cpp (C++), .py (Python), .java (Java), etc.".truecolor(36, 200, 255));
                                    println!("{}\n      {}", "Markup and Data Files: ".yellow(), "Files that use markup languages like HTML, XML, JSON, YAML, etc.".truecolor(36, 200, 255));
                                    println!("{}\n      {}", "Documentation Files: ".yellow(), "Files with documentation in various formats like Markdown (.md) or reStructuredText (.rst).".truecolor(36, 200, 255));
                                    println!("{}\n      {}", "Configuration Files: ".yellow(), "Various configuration files like .toml (Tom's Obvious, Minimal Language), .yaml (YAML Ain't Markup Language), .ini (Initialization Files), etc.".truecolor(36, 200, 255));
                                    println!("{}\n      {}", "Text Data Exports: ".yellow(), "CSV (Comma-Separated Values) files, TSV (Tab-Separated Values) files, and other data exports that contain text data.".truecolor(36, 200, 255));
                                    println!(
                                        "{}\n {}",
                                        "Text-Based Formats: ".yellow(),
                                        "Text-based serialization formats like .json, .yml, etc.

                                    "
                                        .truecolor(36, 200, 255)
                                    );
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
