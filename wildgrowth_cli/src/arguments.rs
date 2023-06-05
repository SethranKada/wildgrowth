use clap::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Extra info for debugging.
    #[arg(short, long)]
    pub verbose: bool,

    /// Silence non-error messages.
    #[arg(short, long)]
    pub quiet: bool,

    /// Specify the command to execute.
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Config {
        #[command(subcommand)]
        command: Config,
    },

    /// Manage peers and their interactions.
    Peers {
        #[command(subcommand)]
        command: Peers,
    },

    /// Allows you to manage your various content,
    /// such as publishing or downloading.
    Content {
        #[command(subcommand)]
        command: Content,
    },

    Events {
        #[command(subcommand)]
        command: Events,
    },

    /// Exchange messages with other users or systems
    Message {
        #[command(subcommand)]
        command: Message,
    },

    /// Debugging utilities for advanced users.
    Debug {
        #[command(subcommand)]
        command: Debug,
    },
}

#[derive(Subcommand)]
pub enum Config {
    Reset {
        #[command(subcommand)]
        command: Reset,
    },
}

#[derive(Subcommand)]
pub enum Reset {
    All {},

    Settings {},
}

#[derive(Subcommand)]
pub enum Peers {
    Add { new_peers: Vec<uuid::Uuid> },

    Remove { old_peers: Vec<uuid::Uuid> },

    List {},
}

#[derive(Subcommand)]
pub enum Content {
    Publish {},

    Upload {},

    Get {},

    Pin {},
}

#[derive(Subcommand)]
pub enum Events {}

#[derive(Subcommand)]
pub enum Message {
    Send {},
}

#[derive(Subcommand)]
pub enum Debug {
    /// Start a detached debugging session.
    StartDetachedSession {
        #[arg(short, long)]
        verbose: bool, // Enable verbose output for the debugging session.
        #[arg(short, long)]
        quiet: bool, // Silence non-error messages during the session.
    },

    GenerateRandomID {},
}
