use directories_next::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub id: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self { id: "id".into() }
    }
}
