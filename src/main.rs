#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate reqwest;
extern crate chrono;
extern crate colored;

use reqwest::header;
use chrono::prelude::*;
use colored::*;
use clap::{Arg, App};


#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    id: String,
    title: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    name: String,
    message: String,
    commit: Commit
}

#[derive(Serialize, Deserialize, Debug)]
struct MergeRequestEvent {
    target_id: i32,
    target_title: String,
    author_username: String,
    created_at: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: i32,
    name: String
}

fn is_created_after_tag(mr_created_at: String, tag_timestamp: i64) -> bool {
    let mr_created_at_timestamp = DateTime::parse_from_rfc3339(
        mr_created_at.as_str()
    ).unwrap().timestamp();
    mr_created_at_timestamp > tag_timestamp
}

fn main() {
    let matches =
        App::new("Lineage: tiny changelog-generator for gitlab")
            .author("<dmitriy.sadkovoy@gmail.com>")
            .about("Does awesome things")
            .arg(Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("GITLAB_HOST")
                .required(true)
                .help("Target gitlab host")
                .takes_value(true))
            .arg(Arg::with_name("token")
                .short("a")
                .long("token")
                .value_name("PERSONAL_ACCESS_TOKEN")
                .required(true)
                .help("Your GitLab access-token \
                (https://<YOUR_GITLAB_HOST>/profile/personal_access_tokens)")
                .takes_value(true))
            .arg(Arg::with_name("project")
                .short("p")
                .long("project")
                .value_name("PROJECT_NAME")
                .required(true)
                .help("Your project name")
                .takes_value(true))
            .arg(Arg::with_name("tag")
                .short("t")
                .long("tag")
                .value_name("TAG")
                .required(true)
                .help("Last deployed tag name")
                .takes_value(true))
            .get_matches();

    let host = matches.value_of("host").unwrap();
    let token = matches.value_of("token").unwrap();
    let project = matches.value_of("project").unwrap();
    let tag = matches.value_of("tag").unwrap();

    let mut headers = header::Headers::new();
    headers.set_raw("PRIVATE-TOKEN", token.to_string());

    let client = reqwest::ClientBuilder::new()
    .danger_disable_certificate_validation_entirely()
    .build().unwrap();

    let mut projects_response = client.get(
        format!("{}/api/v4/projects?search={}", host, project).as_str()
    ).headers(headers.clone()).send().unwrap();
    let projects: Vec<Project> = projects_response.json().unwrap();
    let project_id = projects[0].id;


    let mut tag_response = client.get(
        format!("{}/api/v4/projects/{}/repository/tags/{}", host, project_id, tag).as_str()
    ).headers(headers.clone()).send().unwrap();

    let tag: Tag = tag_response.json().unwrap();
    let parsed_tag_date = DateTime::parse_from_rfc3339(
        tag.commit.created_at.as_str()
    ).unwrap();

    let mut events_response = client.get(
        format!(
            "{}/api/v4/projects/{}/events?target_type=merge_request&action=merged&after={}&per_page=100",
            host, project_id, parsed_tag_date.format("%Y-%m-%d")
        ).as_str()
    ).headers(headers.clone()).send().unwrap();

    let mut merge_requests: Vec<MergeRequestEvent> = events_response.json().unwrap();
    merge_requests.retain(
        |mr| is_created_after_tag(
            mr.created_at.clone(),
            parsed_tag_date.timestamp())
    );

    for mr in &merge_requests {
        println!(
            "{} - Author: {}",
            mr.target_title.green().bold(),
            mr.author_username
        )
    }

}
