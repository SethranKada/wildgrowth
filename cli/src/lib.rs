#![allow(unused)]
mod commands;
use commands as arguments;
use api::start_instance;

use std::time::Duration;
use tokio::{signal, select, io::stdin, time::sleep};
use tokio::sync::mpsc::{channel, Sender};
use tokio_util::sync::CancellationToken;
use clap::{Parser, Subcommand,ValueEnum};

pub async fn begin_session() {
    let cli = arguments::Cli::parse();
    let token = CancellationToken::new();
    let (send, mut recv) = channel(1);

    println!("Creating tasks");
    for i in 0..10 {
        tokio::spawn(start_instance(
            i,
            send.clone(),
            token.child_token()));
    }

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("");
            token.cancel();
            drop(send);
            let _ = recv.recv().await;
            println!("Done!");
            std::process::exit(exitcode::OK);
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            std::process::exit(exitcode::OSERR);
        },
    }
}
