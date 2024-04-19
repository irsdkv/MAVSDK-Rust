use futures_util::StreamExt;
use libmavsdk::telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut telemetry_service =
        telemetry::telemetry_service_client::TelemetryServiceClient::connect(
            "http://0.0.0.0:50051",
        )
        .await?;

    let mut odometry_stream = telemetry_service
        .subscribe_odometry(telemetry::SubscribeOdometryRequest::default())
        .await?
        .into_inner();

    while let Some(odometry) = odometry_stream.next().await {
        let asd = odometry.unwrap();
        println!("asd {:?}", asd)
    }

    Ok(())
}
