use clap::Parser;
use commands::base_commands::BaseCli;

mod persistance;
mod commands;

#[tokio::main]
async fn main() {
    let cli = BaseCli::parse();

    BaseCli::execute(&cli).await;
}
