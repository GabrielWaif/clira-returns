use clap::{ Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "Clira Returns")]
#[command(author = "Gabriel Rosa <gabrielrosargc@hotmail.com>")]
#[command(version = "0.1")]
#[command(about = "Jira cli client for any Jira project", long_about = None)]

struct Cli {
    #[clap(subcommand)]
    pub component: Components,
}

#[derive(Debug, Subcommand)]
pub enum Components {
    // Adds hours to the given project id
    Add { id: String, time: String, date: Option<String>, description: Option<String> },
    Origin(OriginCommand)
}

#[derive(Debug, Args)]
pub struct AddCommand{
    id: String
}

#[derive(Debug, Args)]
pub struct OriginCommand {
    #[clap(subcommand)]
    pub command: OriginCommands,
} 

#[derive(Debug, Subcommand)]
pub enum OriginCommands {
    Add { name: String, url: String },
    Remove { name: String },
    Use { name: String }
} 

fn main() {
    let cli = Cli::parse();

    match &cli.component {
        Components::Add { id, time, date, description } => {
            let description = description.as_ref().unwrap();
            let date = date.as_ref().unwrap();
            println!("{} - {} {}", date, id, description);
            todo!();
        },
        Components::Origin( c ) => {
            match &c.command {
                OriginCommands::Add { name, url } => {
                    println!("Added new origin {:?} with url {:?}", name, url);
                    todo!();
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
        },
    };
}
