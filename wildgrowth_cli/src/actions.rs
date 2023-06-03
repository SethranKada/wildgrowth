use tokio::signal;
use wildgrowth_api::{user::Config, Instance};

pub async fn start() {
    // load config from file, or use a default if none is found.
    let config: Config = Config::default();
    // create a new instance
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

pub async fn list_peers() {
    let config: Config = Config::default();
    println!("Peers:");
    if let Some(peers) = config.peers {
        for peer in peers {
            println!("{}", peer);
        }
    } else {
        println!("...");
    }
}

pub async fn add_peers(mut new_peers: Vec<uuid::Uuid>) {
    let mut config: Config = Config::default();

    println!("Adding Peers:");
    for peer in &new_peers {
        println!("{}", peer);
    }

    if let Some(mut peers) = config.peers {
        peers.append(&mut new_peers);
        config.peers = Some(peers);
    } else {
        config.peers = Some(new_peers);
    }
}
