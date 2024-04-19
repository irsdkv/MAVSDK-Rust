use std::process;
use std::thread::sleep;
use std::time::Duration;

use futures_util::StreamExt;
use libmavsdk::action::{self};
use libmavsdk::telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut telemetry_service =
        telemetry::telemetry_service_client::TelemetryServiceClient::connect(
            "http://0.0.0.0:50051",
        )
        .await?;

    let mut telemetry_stream = telemetry_service
        .subscribe_health(telemetry::SubscribeHealthRequest::default())
        .await?
        .into_inner();

    println!("Waiting for drone to be ready...");
    while let Some(odometry) = telemetry_stream.next().await {
        if odometry.is_ok() {
            break;
        }
    }
    println!("Drone ready to arm!");

    let mut action_service =
        action::action_service_client::ActionServiceClient::connect("http://0.0.0.0:50051").await?;

    let arm_request = action::ArmRequest::default();
    let arm_response =
        action::action_service_client::ActionServiceClient::arm(&mut action_service, arm_request)
            .await?
            .into_inner();

    // Checking responses example
    if let Some(value) = arm_response.action_result {
        if !value.result_str.eq("Success") {
            println!("Drone armed {:?}", value);
        }
    } else {
        process::exit(1);
    }

    println!("Start takeoff");
    let takeoff_request = action::TakeoffRequest::default();
    action::action_service_client::ActionServiceClient::takeoff(
        &mut action_service,
        takeoff_request,
    )
    .await?;

    println!("Wait 10 seconds and land...");
    sleep(Duration::from_millis(10_000));

    println!("Start landing");
    let land_request = action::LandRequest::default();
    action::action_service_client::ActionServiceClient::land(&mut action_service, land_request)
        .await?;

    println!("Example finished");
    Ok(())
}
