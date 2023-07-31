mod p2p;
use colored::*;
use p2p::P2P as p_2_p;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {

    print!("{}[2J", 27 as char); //Clears the terminal
    print!("{}[2J", 27 as char); //Clears the terminal
    println!("{}", print_ascii());
    println!("{}\n", "ChainShare v1.0.0".white().bold());
    //Create a new p2p object to start the p2p network
    let mut p2p: p_2_p = p_2_p::new();
    let _ = p2p.run_task().await;
    //NOTHING UNDER HERE WILL RUN

    Ok(())
}

fn print_ascii() -> ColoredString {
    let ascii = r#"  
######################################################################   
    _____  _             _          _____  _                          
   / ____|| |           (_)        / ____|| |                         
  | |     | |__    __ _  _  _ __  | (___  | |__    __ _  _ __   ___   
  | |     | '_ \  / _` || || '_ \  \___ \ | '_ \  / _` || '__| / _ \  
  | |____ | | | || (_| || || | | | ____) || | | || (_| || |   |  __/  
   \_____||_| |_| \__,_||_||_| |_||_____/ |_| |_| \__,_||_|    \___|

######################################################################
 "#.truecolor(36, 200, 255).bold();
 ascii
}

// fn print_guide() -> ColoredString {

// }