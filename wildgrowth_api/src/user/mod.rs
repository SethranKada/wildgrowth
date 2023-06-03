use directories_next::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::{uuid, Uuid};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub id: Uuid,
    pub peers: Option<Vec<Uuid>>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            peers: None,
        }
    }
}
