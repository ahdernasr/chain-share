// use async_std::io;
// mod input_handler;
// use input_handler::handle_input;
// use futures::prelude::*;
use colored::*;

pub async fn cli_task() {
    //TODO, add COLOR and ASCI-ART
    //CLI begins here
    // let _args = CLIArgs::parse();
    // handle_args(args);

    println!("{}", "Welcome to ChainShare".red().bold());

    // Activate Input Stream
    // let stdin = io::stdin();
    // let mut reader = io::BufReader::new(stdin);

    // loop {
    //     let mut input = String::new();
    //     // Read user input asynchronously
    //     match reader.read_line(&mut input).await {
    //         Ok(bytes_read) => {
    //             if bytes_read == 0 {
    //                 break; // Exit loop if no more input
    //             }
    //             // Process the user input
    //             handle_input(input.trim());
    //         }
    //         Err(error) => {
    //             eprintln!("Error reading input: {}", error);
    //             break;
    //         }
    //     }
    // }
}