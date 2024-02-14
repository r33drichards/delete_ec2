use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{Client, Error, Region, model::Filter};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
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

    let instance_ids = instances.reservations().unwrap_or_default()
        .iter()
        .flat_map(|reservation| reservation.instances().unwrap_or_default())
        .map(|instance| instance.instance_id().unwrap_or_default())
        .map(|i| i.to_string())
        .collect::<Vec<_>>();
    
    let deleted = client.terminate_instances()
    .set_instance_ids(Some(instance_ids))
    .send()
    .await?;

    println!("Deleted instances: {:?}", deleted.terminating_instances().unwrap_or_default());

    Ok(())
}
