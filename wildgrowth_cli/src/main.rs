#![allow(unused)]
use wildgrowth_api::*;

use clap::{Parser, Subcommand, ValueEnum};
use directories_next::{ProjectDirs, UserDirs};
use tokio::sync::mpsc::{channel, Sender};
use tokio::{io::stdin, select, signal, time::sleep};

mod arguments;
use arguments::*;
mod peers;
use peers::*;
mod debug;
use debug::*;
mod config;
use config::*;

#[tokio::main]
async fn main() {
    // parse cli arguments
    let cli = arguments::Cli::parse();

    match &cli.command {
        Commands::Config { command } => match command {
            Config::Reset { command } => match command {
                Reset::All {} => config::reset_all().await,
                Reset::Settings {} => {}
            },
        },
        Commands::Peers { command } => match command {
            Peers::Add { new_peers } => add_peers(new_peers.clone()).await,
            Peers::Remove { old_peers } => remove_peers(old_peers.clone()).await,
            Peers::List {} => list_peers().await,
        },
        Commands::Content { command } => match command {
            Content::Get {} => {}
            Content::Pin {} => {}
            Content::Upload {} => {}
            Content::Publish {} => {}
        },
        Commands::Events { command } => {}
        Commands::Message { command } => {}
        Commands::Debug { command } => match command {
            Debug::StartDetachedSession { verbose, quiet } => start(verbose, quiet).await,
            Debug::GenerateRandomID {} => generate_random_id().await,
        },
    }
}
