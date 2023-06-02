#![allow(unused)]
use wildgrowth_api::start_instance;

use clap::{Parser, Subcommand, ValueEnum};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Sender};
use tokio::{io::stdin, select, signal, time::sleep};
use tokio_util::sync::CancellationToken;

mod arguments;

#[tokio::main]
async fn main() {
    let cli = arguments::Cli::parse();
    let token = CancellationToken::new();
    let (send, mut recv) = channel(1);

    println!("Creating tasks");
    for i in 0..10 {
        tokio::spawn(start_instance(i, send.clone(), token.child_token()));
    }

    match signal::ctrl_c().await {
        Ok(()) => {
            println!();
            token.cancel();
            drop(send);
            let _ = recv.recv().await;
            println!("Done!");
            std::process::exit(exitcode::OK);
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            std::process::exit(exitcode::OSERR);
        }
    }
}
