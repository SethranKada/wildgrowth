use directories_next::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{read_to_string, File};
use tokio::io::{AsyncWriteExt, BufWriter, ErrorKind};
use uuid::{uuid, Uuid};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub id: Uuid,
    pub peers: Option<Vec<Uuid>>,
}

impl Config {
    pub async fn get() -> Self {
        if let Some(path) = ProjectDirs::from("net", "Caden Lee", "WildGrowth") {
            let mut config_path: PathBuf = path.config_dir().to_path_buf();
            config_path.push("config.toml");

            let config = match read_to_string(&config_path).await {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => return (Self::new().await),
                    other => {
                        panic!(
                            "Problem opening file: {} Error: {:?}",
                            &config_path.display(),
                            other
                        );
                    }
                },
            };
            match toml::from_str(config.as_str()) {
                Ok(config) => config,
                Err(error) => {
                    panic!("Problem parsing file!");
                }
            }
        } else {
            panic!("Problem finding config dir!");
        }
    }

    pub async fn new() -> Self {
        let config = Self {
            id: Uuid::new_v4(),
            peers: None,
        };
        Self::save(&config).await;
        config
    }

    pub async fn save(&self) {
        if let Some(path) = ProjectDirs::from("net", "Caden Lee", "WildGrowth") {
            let mut config_path: PathBuf = path.config_dir().to_path_buf();
            config_path.push("config.toml");

            let file = match File::create(config_path).await {
                Ok(f) => f,
                Err(error) => {
                    panic!("failed to create file! {:?}", error);
                }
            };

            let string = match toml::to_string_pretty(&self) {
                Ok(f) => f,
                Err(error) => {
                    panic!("failed to parse config into file");
                }
            };

            let mut buffer = BufWriter::new(file);
            buffer.write_all(string.as_bytes()).await;
            buffer.flush().await;
        }
    }
}
