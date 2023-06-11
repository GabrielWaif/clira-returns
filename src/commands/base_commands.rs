use std::time::Duration;

use clap::{
    command,
    Parser,
    Subcommand
};

use humantime::parse_duration;

use crate::persistance::dtos::dtos::AddWorklogDto;
use crate::persistance::jira_integration::JiraIntegration;

use super::origin_commands::BaseOrigin;

#[derive(Parser)]
#[command(name = "Clira Returns")]
#[command(author = "Gabriel Rosa <gabrielrosargc@hotmail.com>")]
#[command(version = "0.1")]
#[command(about = "Jira cli client for any Jira project", long_about = None)]
pub struct BaseCli {
    #[clap(subcommand)]
    pub base_commands: BaseCommands,
}

#[derive(Debug, Subcommand)]
pub enum BaseCommands {
    Add {
        id: String,
        time: String,
        #[arg(long)]
        date: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Get {
        id: String,
    },
    Origin(BaseOrigin),
}

impl BaseCli {
    pub async fn execute(commands: &BaseCli) {
        match &commands.base_commands {
            BaseCommands::Add {
                id,
                time,
                date,
                description,
            } => {
                BaseCli::add_command(id, time, date, description).await;
            }
            BaseCommands::Get{ id } => {
                BaseCli::get_command(&id).await;
            }
            BaseCommands::Origin(c) => {
                BaseOrigin::execute(&c).await;
            }
        };
    }

    async fn get_command(id: &String) {
        let result = JiraIntegration::get_work(id).await;
    }

    async fn add_command(
        id: &String,
        time: &String,
        date: &Option<String>,
        description: &Option<String>,
    ) {

        let duration: i32 = time.parse().unwrap_or_else(|_| {
            parse_duration(time).unwrap_or_else(|_| Duration::ZERO).as_secs() as i32
        }); 

        let mut body: AddWorklogDto = AddWorklogDto::new(
            "".to_string(),
            "".to_string(),
            duration as i32,
        );

        body.set_started(JiraIntegration::str_to_jira_time(date));

        match description {
            Some(value) => body.set_comment(value.to_owned()),
            None => {}
        }

        JiraIntegration::add_work(&body, &id).await;
    }
}
