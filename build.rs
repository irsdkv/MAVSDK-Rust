const PROTO_INCLUDE_PATH: &str = "proto/protos";

const PLUGINS: &[&str] = &[
    "action",
    "calibration",
    "camera",
    "core",
    "geofence",
    "gimbal",
    "info",
    "mission",
    "mocap",
    "offboard",
    "param",
    "shell",
    "telemetry",
];

fn main() -> std::io::Result<()> {
    for plugin in PLUGINS {
        let proto_path = format!("{PROTO_INCLUDE_PATH}/{plugin}/{plugin}.proto");

        tonic_build::configure()
            .build_server(false)
            .out_dir("src/grpc/")
            .compile(&[proto_path.as_str()], &[PROTO_INCLUDE_PATH])?;
    }
    Ok(())
}
