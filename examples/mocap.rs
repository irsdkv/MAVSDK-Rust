use libmavsdk::mocap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vision_position_estimate = mocap::VisionPositionEstimate {
        time_usec: 0,
        position_body: Some(mocap::PositionBody::default()),
        angle_body: Some(mocap::AngleBody::default()),
        pose_covariance: Some(mocap::Covariance::default()),
    };

    let mut mocap_service =
        mocap::mocap_service_client::MocapServiceClient::connect("http://0.0.0.0:50051").await?;

    for _ in 0..500 {
        let req = mocap::SetVisionPositionEstimateRequest {
            vision_position_estimate: Some(vision_position_estimate.clone()),
        };
        println!("Sending {:?}", req);
        mocap_service.set_vision_position_estimate(req).await?;

        vision_position_estimate.position_body.as_mut().unwrap().x_m += 0.1;
        vision_position_estimate.position_body.as_mut().unwrap().y_m -= 0.1;
        vision_position_estimate.position_body.as_mut().unwrap().z_m -= 0.01;

        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    println!("All sent successfully!");

    Ok(())
}
