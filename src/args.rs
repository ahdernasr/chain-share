use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct CLIArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,   
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// View information about the BlockChain and it's Nodes
    View(ViewCommand),
    /// Mine a block to upload a file 
    Mine(MineCommand),
    /// Retrieve a file 
    Retrieve(RetrieveFileCommand),
}

#[derive(Debug, Args)]
pub struct ViewCommand {
    /// Peers (p), BlockChain Length (bl), BlockChain Storage (bs)
    pub option: String,
}

#[derive(Debug, Args)]
pub struct MineCommand {
    /// File Name
    pub name: String,
    // File Path
    pub path: String,
}

#[derive(Debug, Args)]
pub struct RetrieveFileCommand {
    // File to retrieve's Hash
    pub hash: String,
}




// #[derive(Debug, Subcommand)]
// pub enum ViewSubcommand {
//     Peers(ViewPeers),
// }

// #[derive(Debug, Args)]
// pub struct ViewPeers {
//     /// The name of the user
//     pub name: String, 
// }

//Function that handles the commmands from the CLI
pub fn handle_args(args: CLIArgs) {
    match args.entity_type {
        EntityType::View(view_command) => {
            let option = view_command.option;
            match option.as_str() {
                "p" => {
                    println!("View Option: Peers");
                    println!("Number of peers: 14");
                    println!("Peers Lists: TODO");
                    // Todo 
                }
                "bl" => {
                    println!("View Option: Length");
                    println!("Number of blocks: 50");
                    // Todo 
                }
                "bs" => {
                    println!("View Option: Storage");
                    println!("BlockChain storage space: 50gb");
                    // Todo
                }
                _ => {
                    println!("Invalid View option given");
                    println!("Valid options: Peers (p), BlockChain Length (bl), BlockChain Storage (bs)")
                }
            }
        }
        EntityType::Mine(mine_command) => {
            let (name, path) = (mine_command.name, mine_command.path);
            println!("File Name: {}, File Path: {}", name, path);
            // Todo
        } 
        EntityType::Retrieve(retrieve_command) => {
            let hash = retrieve_command.hash;
            println!("File hash: {}", hash);
            // Todo 
        }
    }
}