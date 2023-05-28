use clap::{Parser, Subcommand,ValueEnum};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(long)]
    debug: bool,

    /// Extra info for the beginners.
    #[arg(short, long)]
    verbose: bool,

    /// silence non-error messages
    #[arg(short, long)]
    quiet: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// initialize instance and create config file
    Init {
        /// Turn debugging information on
        #[arg(long)]
        debug: bool,

        /// Extra info for the beginners
        #[arg(short, long)]
        verbose: bool,

        /// silence non-error messages
        #[arg(short, long)]
        quiet: bool,
    },

    /// check whether your node is functioning properly
    Status {
        /// Turn debugging information on
        #[arg(long)]
        debug: bool,

        /// extra info for beginners
        #[arg(short, long)]
        verbose: bool,
    },

    /// reset instance to a fresh install
    Clean {
        /// Turn debugging information on
        #[arg(long)]
        debug: bool,

        /// Extra info for the beginners
        #[arg(short, long)]
        verbose: bool,

        /// silence non-error messages
        #[arg(short, long)]
        quiet: bool,

        /// reset everything to a fresh install
        #[arg(value_enum)]
        mode: CleanMode,
    },


}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CleanMode {
    /// only reset user settings
    Settings,
    /// only clear cache
    Cache,
    /// only remove non-essential files
    Data,
    /// reset everything
    All
}
