use std::{path::Path, fs::{self, File}, io::Write};
use colored::Colorize;
use reqwest::Response;
use serde_repr::*;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

use crate::config::Config;

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
        
                    // println!("{}", data_file);
        
                    list = serde_json::from_str(&data_file).unwrap_or_else(|_e| {
                        // println!("{}", e);
                        println!("Error: can't parse data_file into json");
                        vec![]
                    })
                }
            },
            false => {
                let api_link: String = format!("{}", config.remote_location);
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
                        println!("Error: unable to send request to server");
                        std::process::exit(exitcode::UNAVAILABLE);
                    });
                    
                // println!("{:?}", &res.text().await.unwrap_or("Error: can't covert body to text".to_string()));
 
                if res.status().is_success() {
                    let res_json = &res.json::<ApiRes>().await.unwrap_or_else(|_| {
                        println!("Error: unable to parse response to json");
                        std::process::exit(exitcode::DATAERR);
                    });
                    if res_json.valid {
                        list = res_json.data.to_owned();
                    }
                }
                else {
                    println!("Error: invalid response from server");
                    std::process::exit(exitcode::DATAERR);
                }

            },
        }        
        return List { data: list };
    }

    pub fn write(new_data: ListData, data_path_string: &String) {

        let data_path: &Path = Path::new(data_path_string);

        let string_data: String = serde_json::to_string_pretty(&new_data).unwrap_or_else(|_| {
            println!("Error: can't parse ListData to string_data");
            std::process::exit(exitcode::DATAERR);

        });

        fs::write(data_path, string_data).unwrap_or_else(|_|{
            println!("Error: couldn't find or write project dir");
            std::process::exit(exitcode::OSFILE);
        });   
    }
    
    fn create_data_file(data_path: &Path, file_name: &str) -> String {
        std::fs::create_dir_all(data_path).unwrap();

        println!("Creating a data file...");

        let deafult_data_file: &str = "[]";

        let mut new_data_file: File = File::create(data_path.join(file_name))
            .expect("Error: can't create data file");

        new_data_file.write_all(deafult_data_file.as_bytes()).expect("Error: can't write config file");

        println!("For more information about this tool, run '{}'", format!("clerk.exe -h").bold());

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
                println!("Error: unable to send request to server");
                std::process::exit(exitcode::UNAVAILABLE);
            });
            
        // println!("{:?}", &res.text().await.unwrap_or("Error: can't covert body to text".to_string()));

        if !res.status().is_success() {
            println!("Error: invalid response from server");
            std::process::exit(exitcode::DATAERR);
        }
    }
}