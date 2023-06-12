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

        // create and run task, then assign it's handle to 'task'
        let task = tokio::spawn(async move {
            Self::run(token_clone, config_clone).await;
        });

        // assign all relevant variables to the Instance,
        // so the task always has access to them,
        // also so the task is destroyed when the instance is dropped.
        Instance {
            config,
            task: Some(task),
            token,
        }
    }

    async fn run(token: CancellationToken, config: Arc<Mutex<user::Config>>) {
        let mut i: u8 = 0;
        loop {
            i += 1; //safety int, it goes up and kills the task when appropriate
            if (i >= u8::MAX - 1) {
                eprintln!("task exceeded max loop length");
                break;
            }

            // check if the token is cancelled, and if so then exit the loop
            if token.is_cancelled() {
                break;
            }

            // grab the config from the atomic, allowing local use.
            let config = config.lock().await;
            let id: Uuid = config.id;

            // do some stuff
            println!("Instance '{}' is Running task {}...", id, i);

            // release the config so other tasks can make use of it, if there are any.
            drop(config);
            // tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    pub async fn stop(mut self) {
        // cancel the token to stop the task gracefully.
        self.token.cancel();

        // then do some magic that makes this work for some reason.
        // this was made by ChatGPT and I don't know why it fixes the issue.
        if let Some(task) = self.task.take() {
            task.await.expect("Failed to join task");
        }
    }
}

impl Drop for Instance {
    // if the instance is ever dropped, call this function.
    fn drop(&mut self) {
        // give the task some time to shutdown gracefully by cancelling it's token.
        self.token.cancel();

        // then, if it still hasn't shutdown after that, abort the task.
        if let Some(task) = self.task.take() {
            // Cancel the task
            task.abort();
        }
    }
}
