
use cli;
use tokio;

#[tokio::main]
async fn main() {
    cli::begin_session().await;
}