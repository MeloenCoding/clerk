use std::{path::Path, fs::{self, File}, io::Write};
use colored::Colorize;
use reqwest::Response;
use serde_repr::*;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

use crate::{config::Config, create_error};

pub type ListData = Vec<MainTaskFormat>;

#[derive(Debug, Serialize)]
pub struct List {
    pub data: ListData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub data: String,
    pub state: TaskState
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MainTaskFormat {
    pub data: Vec<Todo>,
    pub title: String,
    pub state: TaskState,
    pub github_link: String
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum TaskState {
    Pending = 0,
    Doing = 1,
    Completed = 2
}

#[derive(Debug, Deserialize)]
pub struct ApiRes {
    valid: bool,
    data: Vec<MainTaskFormat>
}

impl List {
    pub async fn read(config: &Config) -> Self {
        let mut list: ListData = vec![];        
        match config.local {
            true => {
                if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "clerk") {
                    let data_path: &Path = proj_dirs.data_dir();
                    let data_file: String = match fs::read_to_string(data_path.join("data.json")) {
                        Ok(data) => data,
                        Err(_error) => { 
                            Self::create_data_file(data_path, "data.json")
                        }
                    };
        
                    list = serde_json::from_str(&data_file).unwrap_or_else(|_e| {
                        create_error("can't parse data_file into json", None);
                        vec![]
                    })
                }
            },
            false => {
                let api_link: String = format!("{}", config.remote_location);
                if api_link == "" || !api_link.starts_with("http") || !api_link.starts_with("https") {
                    create_error("the api endpoint in your config must start with either: 'http' or 'https' when you try to use a remote location", Some(exitcode::DATAERR));
                }
                let res: Response = reqwest::Client::new()
                    .post(api_link)
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({
                        "appId": config.app_id.to_string(),
                        "appKey": config.app_key.to_string(),
                        "clientKey": config.remote_key.to_string(),
                        "endpoint": "/show",
                        "data": {}
                    }))
                    .send()
                    .await.unwrap_or_else(|_| {
                        create_error("unable to send '/show' request to server. Either your api endpoint doesn't exist, or is not setup to handle post requests", Some(exitcode::UNAVAILABLE));
                        panic!("Error: unable to send '/show' request to server. Either your api endpoint doesn't exist, or is not setup to handle post requests");
                    });
                    
                if res.status().is_success() {
                    let res_json = &res.json::<ApiRes>().await.unwrap_or_else(|_| {
                        create_error("unable to parse '/show' response to valid json. Make sure your data file in your server is valid json and has the required structure", Some(exitcode::DATAERR));
                        panic!("Error: unable to parse '/show' response to json");
                    });
                    if res_json.valid {
                        list = res_json.data.to_owned();
                    }
                }
                else {
                    create_error("invalid '/show' response from server. Request status was not succesfull", Some(exitcode::DATAERR));
                }
            },
        }        
        return List { data: list };
    }

    pub fn write(new_data: ListData, data_path_string: &String) {

        let data_path: &Path = Path::new(data_path_string);

        let string_data: String = serde_json::to_string_pretty(&new_data).unwrap_or_else(|_| {
            create_error("can't parse ListData to string_data", Some(exitcode::DATAERR));
            panic!("Error: unable to parse response to json");
        });

        fs::write(data_path, string_data).unwrap_or_else(|_|{
            create_error("couldn't find or write project dir", Some(exitcode::OSFILE));
            panic!("Error: couldn't find or write project dir");
        });   
    }
    
    fn create_data_file(data_path: &Path, file_name: &str) -> String {
        std::fs::create_dir_all(data_path).unwrap();

        println!("Creating a data file...");

        let deafult_data_file: &str = "[]";

        let mut new_data_file: File = File::create(data_path.join(file_name))
            .expect("Error: can't create data file");

        new_data_file.write_all(deafult_data_file.as_bytes()).expect("Error: can't write config file");

        println!("For more information about this tool, run '{}'. \nOr take a look at the documentation here: {}", format!("clerk.exe -h").bold(), env!("CARGO_PKG_REPOSITORY").bold());

        return deafult_data_file.to_owned();
    }

    pub async fn set(config: &Config, new_data: &ListData) {
        let api_link: String = format!("{}", config.remote_location);
        let res: Response = reqwest::Client::new()
            .post(api_link)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "appId": config.app_id.to_string(),
                "appKey": config.app_key.to_string(),
                "clientKey": config.remote_key.to_string(),
                "endpoint": "/set",
                "data": {
                    "list": new_data
                }
            }))
            .send()
            .await.unwrap_or_else(|_| {
                create_error("unable to send request to server", Some(exitcode::UNAVAILABLE));
                panic!("Error: unable to send request to server");
            });
            
        if !res.status().is_success() {
            create_error("invalid response from server", Some(exitcode::DATAERR));
            panic!("Error: invalid response from server");
        }
    }
}