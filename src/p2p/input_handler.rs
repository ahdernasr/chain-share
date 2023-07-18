// This function takes in the user input and handles it, if there is something to be published then that is returned 
pub fn handle_input(input: &str) -> Option<&str> {
    // Splits input into multiple parameters
    // Note: Always show selection
    let input_iterator: Vec<&str> = input.split(' ').collect();
    let mut iterator = input_iterator.iter();

    //Iterating through parameters, checking if they exist first through Some
    match iterator.next() {
        Some(option) => {
            match option {
                &"view" => {
                    match iterator.next() {
                        Some(option) => {
                            match option {
                                &"blocks" => {
                                    return Some("Blocks 50")
                                } //show number of blocks in blockchain and info of all blocks
                                &"storage" => {} //show storage info by blockchain
                                &"peers" => {} //show peers ID list and number of peers
                                _ => {
                                    //view 'view' help
                                }
                            }
                        }
                        _ => {
                            //view 'view' help
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
    return Some("test");
}
