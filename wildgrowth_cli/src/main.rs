#![allow(unused)]
use wildgrowth_api::*;

use clap::{Parser, Subcommand, ValueEnum};
use directories_next::{ProjectDirs, UserDirs};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Sender};
use tokio::{io::stdin, select, signal, time::sleep};
use tokio_util::sync::CancellationToken;

mod arguments;

#[tokio::main]
async fn main() {
    // parse cli arguments
    let cli = arguments::Cli::parse();

    let config = confy::load("WildGrowth", None).unwrap();
    let instance = Instance::start(config).await;

    // pause this thread and do nothing, waiting until ctrl_c is pressed
    tokio::select! {
        _ = signal::ctrl_c() => {
            // stop the instance and then execute it
            instance.stop().await;
            println!("Done!");
            // exit program with the all clear that nothing went wrong
            std::process::exit(exitcode::OK);
        },
    }
}
