use uuid::Uuid;
use wildgrowth_api::user::Config;

pub async fn list_peers() {
    let config = Config::get().await;
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
    let mut config = Config::get().await;

    println!("Adding Peers:");
    for peer in &new_peers {
        println!("{}", peer);
    }

    if let Some(mut peers) = config.peers {
        peers.append(&mut new_peers);
        config.peers = Some(peers);
        config.clean().await;
        config.save().await;
    } else {
        config.peers = Some(new_peers);
        config.clean().await;
        config.save().await;
    }
}

pub async fn remove_peers(mut old_peers: Vec<uuid::Uuid>) {
    let mut config = Config::get().await;
    let Some(mut peers) = config.peers else {
        return;
    };
    println!("Removing Peers:");
    peers.retain(|x| !old_peers.contains(x));
    for peer in old_peers {
        println!("{}", peer);
    }
    config.peers = Some(peers);
    config.clean().await;
    config.save().await;
}
