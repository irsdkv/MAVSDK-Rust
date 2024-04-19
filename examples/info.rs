use libmavsdk::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut version_service =
        info::info_service_client::InfoServiceClient::connect("http://0.0.0.0:50051").await?;

    let version = version_service
        .get_version(info::GetVersionRequest::default())
        .await?;

    println!("Version received: {version:?}");

    Ok(())
}
