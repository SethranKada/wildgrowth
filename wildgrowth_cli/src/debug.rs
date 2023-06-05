use tokio::signal;
use wildgrowth_api::{user::Config, Instance};

pub async fn generate_random_id() {
    println!("Here you go: {}", uuid::Uuid::new_v4());
}

pub async fn start(verbose: &bool, quiet: &bool) {
    // load config from file, or use a default if none is found.
    let config = Config::get().await;
    // create a new instance
    let instance = Instance::start(config).await;

    // pause this thread and do nothing, waiting until ctrl_c is pressed
    tokio::select! {
        _ = signal::ctrl_c() => {
            // stop the instance and then execute it
            instance.stop().await;
            if(!quiet) {println!("Done!");}
            // exit program with the all clear that nothing went wrong
            std::process::exit(exitcode::OK);
        },
    }
}
