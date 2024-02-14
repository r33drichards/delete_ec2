use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error, Region, model::Filter};
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let instances = client.describe_instances()
        .set_filters(Some(vec![
            Filter::builder().name("instance-state-name").values("running").build(),
            Filter::builder().name("tag:Name").values("nix").build(),
        ]))
        .send()
        .await?;

    for reservation in instances.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            println!("Instance ID: {}", instance.instance_id().unwrap_or_default());
            // Add more details as needed
        }
    }

    Ok(())
}
