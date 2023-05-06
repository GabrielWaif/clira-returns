use clap::{ Parser, Subcommand, Args};

#[derive(Parser)]
#[command(name = "Clira Returns")]
#[command(author = "Gabriel Rosa <gabrielrosargc@hotmail.com>")]
#[command(version = "0.1")]
#[command(about = "Jira cli client for any Jira project", long_about = None)]

pub struct Cli {
    #[clap(subcommand)]
    pub component: Components,
}

#[derive(Debug, Subcommand)]
pub enum Components {
    // Adds hours to the given project id
    Add {
        id: String,
        time: String,
        #[arg(long)]
        date: Option<String>,
        #[arg(short, long)]
        description: Option<String>
    },
    Origin(OriginCommand)
}


#[derive(Debug, Args)]
pub struct AddCommand {
    id: String,
    time: String,
    date: Option<String>,
    description: Option<String>
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TesteRespose {
    id: i32,
    title: String
}
