#![allow(unused)]
#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "user")]
pub mod user;

pub mod db;

use std::time::Duration;
use tokio::{signal, select, time::sleep};
use tokio::sync::mpsc::{channel, Sender};
use tokio_util::sync::CancellationToken;


pub async fn start_instance(i: u64, _sender: Sender<()>, token:CancellationToken) {
    tokio::select! {
        _ = token.cancelled() => {
            sleep(Duration::from_millis(100 * i)).await;
            println!("Task {} shutting down.", i);
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(i)) => {
            println!("Task {} took too long", i);
            token.cancel();
        }
    }
}

pub async fn status() {}

pub async fn create_account() {}

pub async fn delete_account() {}

pub async fn clear_cache() {}

pub async fn remove_data() {}