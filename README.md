# MAVSDK-Rust

This is the Rust wrapper for MAVSDK.

The Rust wrapper is based on a gRPC client communicating with the gRPC server written in C++.
The wrapper is auto-generated from the message definitions ([proto files](https://github.com/mavlink/MAVSDK-Proto)).

## Requirements

- Rust >= 1.39.0 (for async/await support)
- Building mavsdk-rust requires the [protoc](https://grpc.io/docs/protoc-installation/) compiler to be installed.

## Trying the Examples

1. Initialize proto submodule and build

```bash
git submodule init
git submodule update
cargo build
```

2. Start a simulator as described [here.](https://mavsdk.mavlink.io/main/en/cpp/examples/#setting-up-a-simulator)

```bash
make px4_sitl jmavsim
```

3. Run **MAVSDK Backend** on `localhost:50051`. [More info here.](https://mavsdk.mavlink.io/main/en/cpp/guide/build_mavsdk_server.html)

```bash
cd [path_to_MAVSDK]
./build/default/src/mavsdk_server/src/mavsdk_server
```

4. Run examples

- Info

```bash
cargo run --example info

```

Expected output:

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/info`
Version received: Version { flight_sw_major: 1, flight_sw_minor: 10, flight_sw_patch: 0, flight_sw_vendor_major: 0, flight_sw_vendor_minor: 0, flight_sw_vendor_patch: 0, os_sw_major: 8, os_sw_minor: 2, os_sw_patch: 0 }
```

- Mocap

```bash
cargo run --example mocap
```

Expected output:

```bash
Finished dev [unoptimized + debuginfo] target(s) in 0.31s
 Running `target/debug/examples/mocap`
...
Sending SetVisionPositionEstimateRequest { vision_position_estimate: Some(VisionPositionEstimate { time_usec: 0, position_body: Some(PositionBody { x_m: 49.89981, y_m: -49.89981, z_m: -4.9900193 }), angle_body: Some(AngleBody { roll_rad: 0.0, pitch_rad: 0.0, yaw_rad: 0.0 }), pose_covariance: Some(Covariance { covariance_matrix: [] }) }) }
All sent successfully!
```

- Telemetry

```bash
cargo run --example telemery
```

Expected output:

```bash
   Compiling libmavsdk v0.1.0 (/home/ildar/sw/mavsdk-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 4.92s
     Running `target/debug/examples/telemetry`
...
Odometry: Odometry { time_usec: 0, frame_id: EstimNed, child_frame_id: Undef, position_body: PositionBody { x_m: 0.0, y_m: 0.0, z_m: -3.483048 }, q: Quaternion { w: 0.6384722, x: -0.004061609, y: 0.079110526, z: 0.76555747 }, speed_body: SpeedBody { velocity_x_m_s: 0.0042169667, velocity_y_m_s: -0.0015938352, velocity_z_m_s: -0.014632007 }, angular_velocity_body: AngularVelocityBody { roll_rad_s: 0.0005086092, pitch_rad_s: 0.00023366197, yaw_rad_s: -0.0002803828 }, pose_covariance: Covariance { covariance_matrix: [0.0079130605, 0.0, 0.0, 0.0, 0.0, 0.0, 0.007913225, 0.0, 0.0, 0.0, 0.0, 0.044821125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] }, velocity_covariance: Covariance { covariance_matrix: [0.0052988436, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0052990587, 0.0, 0.0, 0.0, 0.0, 0.0045366324, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] } }
...
```

- Takeoff and land

```bash
cargo run --example takeoff_and_land

```

Expected output:

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/takeoff_and_land`
Waiting for drone to be ready...
Drone ready to arm!
Start takeoff
Wait 10 seconds and land...
Start landing
Example finished
```

You should see the copter taking off, holding and landing in the sim window.
