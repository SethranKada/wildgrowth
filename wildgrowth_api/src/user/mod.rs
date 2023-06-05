use directories_next::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{read_to_string, File};
use tokio::io::{AsyncWriteExt, BufWriter, ErrorKind};
use uuid::{uuid, Uuid};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub id: Uuid,
    pub peers: Option<Vec<Uuid>>,
}

impl Config {
    pub async fn get() -> Self {
        let Some(path) = ProjectDirs::from("net", "Caden Lee", "WildGrowth") else {
            panic!("Problem finding config dir!");
        };
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
            Err(error) => Self::new().await,
        }
    }

    pub async fn new() -> Self {
        let mut config = Self {
            id: Uuid::new_v4(),
            peers: None,
        };
        config.save().await;
        config
    }

    pub async fn save(&self) {
        let Some(path) = ProjectDirs::from("net", "Caden Lee", "WildGrowth") else {
            panic!("failed to find project dir!");
        };

        let mut config_path: PathBuf = path.config_dir().to_path_buf();
        config_path.push("config.toml");

        let Ok(file) = File::create(config_path).await else {
            panic!("failed to create file!");
        };

        let Ok(string) = toml::to_string_pretty(&self) else {
            panic!("failed to parse config into file");
        };

        let mut buffer = BufWriter::new(file);
        buffer.write_all(string.as_bytes()).await;
        buffer.flush().await;
    }

    pub async fn clean(&mut self) {
        let Some(mut peers) = self.peers.clone() else {
            self.peers = None;
            return;
        };
        if peers.is_empty() {
            self.peers = None;
            return;
        }
        peers.dedup();
        self.peers = Some(peers);
    }
}
