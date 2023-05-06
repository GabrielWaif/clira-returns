use crate::command_structure::{ Cli, OriginCommands, Components, TesteRespose };
use crate::dtos::AddWorklogDto;
use clap::Parser;
use reqwest::Client;
use chrono::prelude::*;
mod command_structure;
mod dtos;

#[tokio::main]
async fn main() {
    let token = "OTU5OTA2MTIxOTQ1Ogv6E6j8H7FWbFPvbt30uqSW02oD";
    let cli = Cli::parse();

    match &cli.component {
        Components::Add { id, time, date, description } => { 
            let mut body: AddWorklogDto =  AddWorklogDto::new("".to_string(), "2023-05-06T09:00:00.000-0700".to_string(), 3600 );
            match date {
                Some( value ) => {
                    //let date = .expect("Invalid date.");
                    let date = match DateTime::parse_from_str(format!("{} -03:00", value).as_str(), "%Y-%m-%d %H:%M:%S %z") {
                        Ok( ok_date ) => ok_date,
                        Err( error ) => {
                            println!("Invalid date param. error={}", error);
                            std::process::exit(0);
                        }  
                    };
                    let formated_date = date.format("%Y-%m-%dT%H:%M:%S.%3f%z").to_string();
                    println!("{}", formated_date);
                    body.set_started(formated_date.to_string())
                }
                None => {}
            }

            match description {
                Some( value ) => body.set_comment(value.to_owned()),
                None => {}
            }

            let client = Client::new();
            let url = format!("https://jira.weg.net/rest/api/2/issue/{}/worklog", id);
            let result = client.post(url)
                .json(&body)
                .header("Authorization", format!("{} {}", "Bearer", token))
                .send()
                .await
                .unwrap();

            println!("{:?}", result);
        },
        Components::Origin( c ) => {
            match &c.command {
                OriginCommands::Add { name, url } => {
                    println!("Added new origin {:?} with url {:?}", name, url);

                    let result = reqwest::get("https://jira.weg.net/secure/CreateWorklog.jspa").await.unwrap().json::<TesteRespose>().await.unwrap();
                    print!("{:?}", result);
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
