use clap::{
    Args,
    Subcommand
};

#[derive(Debug, Args)]
pub struct BaseOrigin {
    #[clap(subcommand)]
    pub command: OriginCommands,
} 

impl BaseOrigin {
    pub async fn execute(origin: &BaseOrigin) {
        match &origin.command {
            OriginCommands::Add { name, url } => {
                println!("Added new origin {:?} with url {:?}", name, url);

            }
            OriginCommands::Remove { name } => {
                println!("Removed origin {:?}", name);
                todo!();
            },
            OriginCommands::Use { name } => {
                println!("Changed origin to {:?}", name);
                todo!();
            },
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum OriginCommands {
    Add { name: String, url: String },
    Remove { name: String },
    Use { name: String }

} 
