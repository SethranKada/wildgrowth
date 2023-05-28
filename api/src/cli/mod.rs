#![allow(unused)]
mod commands;

use crate::start;
use commands as arguments;
use std::time::Duration;
use clap::{Parser, Subcommand,ValueEnum};
use tokio::{signal, select, io::stdin, time::sleep};
use tokio::sync::mpsc::{channel, Sender};
use tokio_util::sync::CancellationToken;

pub async fn begin_session() {
    let cli = arguments::Cli::parse();
    let token = CancellationToken::new();
    let (send, mut recv) = channel(1);

    println!("Creating tasks");
    for i in 0..10 {
        tokio::spawn(start(
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
