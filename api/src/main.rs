use is_terminal::IsTerminal;
use api::cli;

#[tokio::main]
async fn main() {
    #[cfg(feature = "cli")]
    if std::io::stdout().is_terminal() {cli::begin_session().await;}
}