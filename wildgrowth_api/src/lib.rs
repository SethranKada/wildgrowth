#![allow(unused)]
pub mod db;
pub mod net;
pub mod user;

use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};
use tokio::task;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub struct Instance {
    config: Arc<Mutex<user::Config>>,
    task: Option<JoinHandle<()>>,
    token: CancellationToken,
}

impl Instance {
    pub async fn start(config: user::Config) -> Self {
        println!("instance started!");

        let token = CancellationToken::new();
        let token_clone = token.clone();

        let config = Arc::new(Mutex::new(config));
        let config_clone = Arc::clone(&config);

        let task = tokio::spawn(async move {
            Self::run(token_clone, config_clone).await;
        });

        Instance {
            config,
            task: Some(task),
            token,
        }
    }

    async fn run(token: CancellationToken, config: Arc<Mutex<user::Config>>) {
        loop {
            if token.is_cancelled() {
                break;
            }
            let config = config.lock().await;
            let id: Uuid = config.id;
            println!("Instance '{}' is Running...", id);
            drop(config);

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    pub async fn stop(mut self) {
        self.token.cancel();
        if let Some(task) = self.task.take() {
            task.await.expect("Failed to join task");
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.token.cancel();
        if let Some(task) = self.task.take() {
            // Cancel the task
            task.abort();
        }
    }
}
