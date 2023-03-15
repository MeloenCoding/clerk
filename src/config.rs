use std::{path::Path, fs::File};
use std::io::Write;
use std::fs::read_to_string;

use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Config {
    pub local: bool,
    pub config_dir: String,
    pub local_location: String,
    pub remote_location: String,
    pub remote_key: String,
    pub app_id: String,
    pub app_key: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    settings: Option<TomlConfigSettings>,
    locations: Option<TomlConfigLocations>,
    api: Option<TomlConfigApi>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfigSettings {
    local: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfigLocations {
    config_dir: Option<String>,
    local_location: Option<String>,
    remote_location: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfigApi {
    remote_key: Option<String>,
    app_id: Option<String>,
    app_key: Option<String>
}

impl Config {
    pub fn read() -> Self {
        let mut config: TomlConfig = TomlConfig { settings: None, locations: None, api: None };
        if let Some(proj_dirs) = ProjectDirs::from("dev", "meloencoding", "clerk") {
            let config_path: &Path = proj_dirs.config_dir();
            let config_str: String = match read_to_string(config_path.join("config.toml")) {
                Ok(data) => data,
                Err(_error) => {
                    Self::create_config(config_path, proj_dirs.data_dir())
                }
            };

            config = toml::from_str(&config_str).unwrap_or_else(|_e| {
                // println!("{}", e);
                println!("Error: can't parse config_str into toml");
                TomlConfig { 
                    settings: None, 
                    locations: None,
                    api: None
                }
            })
        }

        let config_local: bool = match config.settings {
            Some(settings) => settings.local.unwrap_or_else(|| {
                println!("Error: missing local variable in Settings table");
                true
            }),
            None => {
                println!("Error: missing Settings table");
                true
            }
        };

        let (config_dir_setting, local_location_setting, remote_location_setting): (String, String, String) = match config.locations {
            Some(locations) => {
                let config_dir_setting_real: String = locations.config_dir.unwrap_or_else(|| {
                    println!("Error: missing config_dir_setting");
                    "".to_owned()
                });
                let local_location_setting_real: String = locations.local_location.unwrap_or_else(|| {
                    println!("Error: missing config_dir_setting");
                    "".to_owned()
                });
                let remote_location_real: String = locations.remote_location.unwrap_or_else(|| {
                    println!("Error: missing config_dir_setting");
                    "".to_owned()
                });
                (config_dir_setting_real, local_location_setting_real, remote_location_real)
            },
            None => {
                println!("Error: missing Locations table");
                ("".to_owned(), "".to_owned(), "".to_owned())
            }
        };

        let (remote_key_setting, app_id_setting, app_key_setting) = match config.api {
            Some(api_settings) => {
                let remote_key_setting_real = api_settings.remote_key.unwrap_or_else(|| {
                    println!("Error: missing remote_key");
                    "".to_owned()
                });
                let app_id_setting_real = api_settings.app_id.unwrap_or_else(|| {
                    println!("Error: missing app_id");
                    "".to_owned()
                });
                let app_key_setting_real = api_settings.app_key.unwrap_or_else(|| {
                    println!("Error: missing app_key");
                    "".to_owned()
                });
                (remote_key_setting_real, app_id_setting_real, app_key_setting_real)
            },
            None => {
                println!("Error: missing Api table");
                ("".to_owned(), "".to_owned(), "".to_owned())
            }
        };

        return Config {
            local: config_local,
            config_dir: config_dir_setting,
            local_location: local_location_setting,
            remote_location: remote_location_setting,
            remote_key: remote_key_setting,
            app_id: app_id_setting,
            app_key: app_key_setting
        };
    }

    fn create_config(config_path: &Path, local_data_path: &Path) -> String {
        std::fs::create_dir_all(config_path).unwrap();

        println!("Creating a config file...");

        let default_config: TomlConfig = TomlConfig {
            settings: Some(TomlConfigSettings {
                local: Some(true)
            }),
            locations: Some(TomlConfigLocations {
                config_dir: Some(config_path.join("config.toml").to_str().unwrap().to_owned()),
                local_location: Some(local_data_path.join("data.json").to_str().unwrap().to_owned()),
                remote_location: Some("".to_owned()),
            }),
            api: Some(TomlConfigApi { 
                remote_key: Some("".to_owned()),
                app_id: Some("".to_owned()),
                app_key: Some("".to_owned()) 
            })
        };

        let config_string: String = toml::to_string(&default_config).unwrap_or_else(|_| {
            println!("Error: unable to format Config to tomlConfigString.");
            "".to_owned()
        });

        let mut new_config_file: File = File::create(config_path.join("config.toml"))
            .expect("Error: can't create config file");
        
        new_config_file.write_all(config_string.as_bytes()).expect("can't write config file");

        return config_string;
    }


}