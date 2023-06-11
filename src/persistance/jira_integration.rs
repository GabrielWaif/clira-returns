use std::fmt::Display;

use chrono::{ 
    DateTime,
    Local,
    TimeZone
};
use reqwest::Client;
use serde::Deserialize;

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

        let result  = client.post(url)
            .json(body)
            .header("Authorization", format!("{} {}", "Bearer", TOKEN))
            .send()
            .await;

        match result {
            Ok(_) => log_format_param("Success", format!("Work added to {}", id)),
            Err(_) => log_format_param("Error", format!("Failed to add work to {}", id)),
        }
        
    } 

    pub async fn get_work(id: &String) -> Result<Issue, String> {
        let client = Client::new();
        let url = format!("https://jira.weg.net/rest/api/2/issue/{}", id);

        let result  = client.get(url)
            .header("Authorization", format!("{} {}", "Bearer", TOKEN))
            .send()
            .await
            .unwrap()
            .json::<Issue>()
            .await
            .unwrap();

        log_issue(&result);
        return Ok(result);
    } 
}

fn log_issue(issue: &Issue) {
    log_format_param("Id", &issue.key);
    log_format_param("Name", &issue.fields.name);
    log_format_param("Description", &issue.fields.issue_description);
    log_format_param("Used time", secs_to_hours(issue.fields.progress.progress));
    log_format_param("Remaining time", secs_to_hours(&issue.fields.progress.total - &issue.fields.progress.progress));
}

fn log_format_param<T: Display>(key: &str, value: T) {
    println!("[{}]: {}", key, value);
}

fn secs_to_hours(secs: i32) -> String {
    return format!("{}h", (secs as f32)/3600.0);
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    key: String,
    fields: IssueFields,
}

#[derive(Debug, Deserialize)]
struct IssueFields {
    progress: IssueProgress,
    #[serde(alias = "summary")]
    name: String,
    #[serde(alias = "customfield_11108")]
    issue_description: String
//    worklog: IssueWorklog 
}

#[derive(Debug, Deserialize)]
struct IssueProgress {
    progress: i32,
    total: i32,
    percent: i32
}

#[derive(Debug, Deserialize)]
struct IssueWorklog {
}
