use std::{path::Path, fs::{self, File}, io::Write};
use serde_repr::*;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

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



impl List {
    pub fn read() -> Self {
        let mut list: ListData = vec![];

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

        return deafult_data_file.to_owned();
    }
}