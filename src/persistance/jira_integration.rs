use chrono::{ 
    DateTime,
    Local,
    TimeZone
};
use reqwest::Client;

use super::dtos::dtos::AddWorklogDto;

static TOKEN: &str = "OTU5OTA2MTIxOTQ1Ogv6E6j8H7FWbFPvbt30uqSW02oD";

pub struct JiraIntegration { }

impl JiraIntegration {
    pub fn str_to_jira_time(date: &Option<String>) -> String {
        let result: String;
        match date {
            Some(value) => {
                let date = match DateTime::parse_from_str(
                    format!("{} {}", value, Local::now().offset()).as_str(),
                    "%Y/%m/%d %H:%M %z",
                ) {
                    Ok(ok_date) => ok_date,
                    Err(error) => {
                        println!("Invalid date param. error={}", error);
                        std::process::exit(0);
                    }
                };

                let formated_date = JiraIntegration::jira_time_helper(&date);
                result = formated_date.to_string();
            }
            None => {
                let formated_date = JiraIntegration::jira_time_helper(&Local::now());
                result = formated_date.to_string();
            }
        }
        return result;
    }

    fn jira_time_helper<Tz: TimeZone>(date_time: &DateTime<Tz>) -> String where Tz::Offset: std::fmt::Display, {
        date_time.format("%Y-%m-%dT%H:%M:%S.%3f%z").to_string()
    }

    pub async fn add_work(body: &AddWorklogDto, id: &String) {
        let client = Client::new();
        let url = format!("https://jira.weg.net/rest/api/2/issue/{}/worklog", id);
        let result = client.post(url)
            .json(body)
            .header("Authorization", format!("{} {}", "Bearer", TOKEN))
            .send()
            .await
            .unwrap();

        println!("{:?}", result);
    } 
}
