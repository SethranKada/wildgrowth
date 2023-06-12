use wildgrowth_api::user;

pub async fn reset_all() {
    println!("Resetting config to default");
    user::Config::new().await;
}
