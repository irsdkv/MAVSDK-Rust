// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionRequest {
    /// The mission plan
    #[prost(message, optional, tag = "1")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionUploadResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// The mission plan
    #[prost(message, optional, tag = "2")]
    pub mission_plan: ::core::option::Option<MissionPlan>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMissionDownloadResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMissionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentMissionItemRequest {
    /// Index of the mission item to be set as the next one (0-based)
    #[prost(int32, tag = "1")]
    pub index: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCurrentMissionItemResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMissionFinishedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMissionFinishedResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// True if the mission is finished and the last mission item has been reached
    #[prost(bool, tag = "2")]
    pub is_finished: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeMissionProgressRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionProgressResponse {
    /// Mission progress
    #[prost(message, optional, tag = "1")]
    pub mission_progress: ::core::option::Option<MissionProgress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAfterMissionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReturnToLaunchAfterMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
    /// If true, trigger an RTL at the end of the mission
    #[prost(bool, tag = "2")]
    pub enable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAfterMissionRequest {
    /// If true, trigger an RTL at the end of the mission
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReturnToLaunchAfterMissionResponse {
    #[prost(message, optional, tag = "1")]
    pub mission_result: ::core::option::Option<MissionResult>,
}
///
/// Type representing a mission item.
///
/// A MissionItem can contain a position and/or actions.
/// Mission items are building blocks to assemble a mission,
/// which can be sent to (or received from) a system.
/// They cannot be used independently.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionItem {
    /// Latitude in degrees (range: -90 to +90)
    #[prost(double, tag = "1")]
    pub latitude_deg: f64,
    /// Longitude in degrees (range: -180 to +180)
    #[prost(double, tag = "2")]
    pub longitude_deg: f64,
    /// Altitude relative to takeoff altitude in metres
    #[prost(float, tag = "3")]
    pub relative_altitude_m: f32,
    /// Speed to use after this mission item (in metres/second)
    #[prost(float, tag = "4")]
    pub speed_m_s: f32,
    /// True will make the drone fly through without stopping, while false will make the drone stop on the waypoint
    #[prost(bool, tag = "5")]
    pub is_fly_through: bool,
    /// Gimbal pitch (in degrees)
    #[prost(float, tag = "6")]
    pub gimbal_pitch_deg: f32,
    /// Gimbal yaw (in degrees)
    #[prost(float, tag = "7")]
    pub gimbal_yaw_deg: f32,
    /// Camera action to trigger at this mission item
    #[prost(enumeration = "mission_item::CameraAction", tag = "8")]
    pub camera_action: i32,
    /// Loiter time (in seconds)
    #[prost(float, tag = "9")]
    pub loiter_time_s: f32,
    /// Camera photo interval to use after this mission item (in seconds)
    #[prost(double, tag = "10")]
    pub camera_photo_interval_s: f64,
    /// Radius for completing a mission item (in metres)
    #[prost(float, tag = "11")]
    pub acceptance_radius_m: f32,
    /// Absolute yaw angle (in degrees)
    #[prost(float, tag = "12")]
    pub yaw_deg: f32,
}
/// Nested message and enum types in `MissionItem`.
pub mod mission_item {
    /// Possible camera actions at a mission item.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CameraAction {
        /// No action
        None = 0,
        /// Take a single photo
        TakePhoto = 1,
        /// Start capturing photos at regular intervals
        StartPhotoInterval = 2,
        /// Stop capturing photos at regular intervals
        StopPhotoInterval = 3,
        /// Start capturing video
        StartVideo = 4,
        /// Stop capturing video
        StopVideo = 5,
    }
    impl CameraAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CameraAction::None => "CAMERA_ACTION_NONE",
                CameraAction::TakePhoto => "CAMERA_ACTION_TAKE_PHOTO",
                CameraAction::StartPhotoInterval => "CAMERA_ACTION_START_PHOTO_INTERVAL",
                CameraAction::StopPhotoInterval => "CAMERA_ACTION_STOP_PHOTO_INTERVAL",
                CameraAction::StartVideo => "CAMERA_ACTION_START_VIDEO",
                CameraAction::StopVideo => "CAMERA_ACTION_STOP_VIDEO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CAMERA_ACTION_NONE" => Some(Self::None),
                "CAMERA_ACTION_TAKE_PHOTO" => Some(Self::TakePhoto),
                "CAMERA_ACTION_START_PHOTO_INTERVAL" => Some(Self::StartPhotoInterval),
                "CAMERA_ACTION_STOP_PHOTO_INTERVAL" => Some(Self::StopPhotoInterval),
                "CAMERA_ACTION_START_VIDEO" => Some(Self::StartVideo),
                "CAMERA_ACTION_STOP_VIDEO" => Some(Self::StopVideo),
                _ => None,
            }
        }
    }
}
/// Mission plan type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionPlan {
    /// The mission items
    #[prost(message, repeated, tag = "1")]
    pub mission_items: ::prost::alloc::vec::Vec<MissionItem>,
}
/// Mission progress type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionProgress {
    /// Current mission item index (0-based), if equal to total, the mission is finished
    #[prost(int32, tag = "1")]
    pub current: i32,
    /// Total number of mission items
    #[prost(int32, tag = "2")]
    pub total: i32,
}
/// Result type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionResult {
    /// Result enum value
    #[prost(enumeration = "mission_result::Result", tag = "1")]
    pub result: i32,
    /// Human-readable English string describing the result
    #[prost(string, tag = "2")]
    pub result_str: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MissionResult`.
pub mod mission_result {
    /// Possible results returned for action requests.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result
        Unknown = 0,
        /// Request succeeded
        Success = 1,
        /// Error
        Error = 2,
        /// Too many mission items in the mission
        TooManyMissionItems = 3,
        /// Vehicle is busy
        Busy = 4,
        /// Request timed out
        Timeout = 5,
        /// Invalid argument
        InvalidArgument = 6,
        /// Mission downloaded from the system is not supported
        Unsupported = 7,
        /// No mission available on the system
        NoMissionAvailable = 8,
        /// Unsupported mission command
        UnsupportedMissionCmd = 11,
        /// Mission transfer (upload or download) has been cancelled
        TransferCancelled = 12,
        /// No system connected
        NoSystem = 13,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unknown => "RESULT_UNKNOWN",
                Result::Success => "RESULT_SUCCESS",
                Result::Error => "RESULT_ERROR",
                Result::TooManyMissionItems => "RESULT_TOO_MANY_MISSION_ITEMS",
                Result::Busy => "RESULT_BUSY",
                Result::Timeout => "RESULT_TIMEOUT",
                Result::InvalidArgument => "RESULT_INVALID_ARGUMENT",
                Result::Unsupported => "RESULT_UNSUPPORTED",
                Result::NoMissionAvailable => "RESULT_NO_MISSION_AVAILABLE",
                Result::UnsupportedMissionCmd => "RESULT_UNSUPPORTED_MISSION_CMD",
                Result::TransferCancelled => "RESULT_TRANSFER_CANCELLED",
                Result::NoSystem => "RESULT_NO_SYSTEM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNKNOWN" => Some(Self::Unknown),
                "RESULT_SUCCESS" => Some(Self::Success),
                "RESULT_ERROR" => Some(Self::Error),
                "RESULT_TOO_MANY_MISSION_ITEMS" => Some(Self::TooManyMissionItems),
                "RESULT_BUSY" => Some(Self::Busy),
                "RESULT_TIMEOUT" => Some(Self::Timeout),
                "RESULT_INVALID_ARGUMENT" => Some(Self::InvalidArgument),
                "RESULT_UNSUPPORTED" => Some(Self::Unsupported),
                "RESULT_NO_MISSION_AVAILABLE" => Some(Self::NoMissionAvailable),
                "RESULT_UNSUPPORTED_MISSION_CMD" => Some(Self::UnsupportedMissionCmd),
                "RESULT_TRANSFER_CANCELLED" => Some(Self::TransferCancelled),
                "RESULT_NO_SYSTEM" => Some(Self::NoSystem),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod mission_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Enable waypoint missions.
    #[derive(Debug, Clone)]
    pub struct MissionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MissionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MissionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MissionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MissionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        /// Upload a list of mission items to the system.
        ///
        /// The mission items are uploaded to a drone. Once uploaded the mission can be started and
        /// executed even if the connection is lost.
        pub async fn upload_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UploadMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/UploadMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("mavsdk.rpc.mission.MissionService", "UploadMission"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Cancel an ongoing mission upload.
        pub async fn cancel_mission_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMissionUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelMissionUploadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/CancelMissionUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "CancelMissionUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Download a list of mission items from the system (asynchronous).
        ///
        /// Will fail if any of the downloaded mission items are not supported
        /// by the MAVSDK API.
        pub async fn download_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::DownloadMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DownloadMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/DownloadMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "DownloadMission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Cancel an ongoing mission download.
        pub async fn cancel_mission_download(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMissionDownloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelMissionDownloadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/CancelMissionDownload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "CancelMissionDownload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Start the mission.
        ///
        /// A mission must be uploaded to the vehicle before this can be called.
        pub async fn start_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/StartMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("mavsdk.rpc.mission.MissionService", "StartMission"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Pause the mission.
        ///
        /// Pausing the mission puts the vehicle into
        /// [HOLD mode](https://docs.px4.io/en/flight_modes/hold.html).
        /// A multicopter should just hover at the spot while a fixedwing vehicle should loiter
        /// around the location where it paused.
        pub async fn pause_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PauseMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/PauseMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("mavsdk.rpc.mission.MissionService", "PauseMission"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Clear the mission saved on the vehicle.
        pub async fn clear_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClearMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/ClearMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("mavsdk.rpc.mission.MissionService", "ClearMission"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Sets the mission item index to go to.
        ///
        /// By setting the current index to 0, the mission is restarted from the beginning. If it is set
        /// to a specific index of a mission item, the mission will be set to this item.
        ///
        /// Note that this is not necessarily true for general missions using MAVLink if loop counters
        /// are used.
        pub async fn set_current_mission_item(
            &mut self,
            request: impl tonic::IntoRequest<super::SetCurrentMissionItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetCurrentMissionItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SetCurrentMissionItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "SetCurrentMissionItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Check if the mission has been finished.
        pub async fn is_mission_finished(
            &mut self,
            request: impl tonic::IntoRequest<super::IsMissionFinishedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsMissionFinishedResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/IsMissionFinished",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "IsMissionFinished",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Subscribe to mission progress updates.
        pub async fn subscribe_mission_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeMissionProgressRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MissionProgressResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SubscribeMissionProgress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "SubscribeMissionProgress",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        ///
        /// Get whether to trigger Return-to-Launch (RTL) after mission is complete.
        ///
        /// Before getting this option, it needs to be set, or a mission
        /// needs to be downloaded.
        pub async fn get_return_to_launch_after_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReturnToLaunchAfterMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetReturnToLaunchAfterMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/GetReturnToLaunchAfterMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "GetReturnToLaunchAfterMission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Set whether to trigger Return-to-Launch (RTL) after the mission is complete.
        ///
        /// This will only take effect for the next mission upload, meaning that
        /// the mission may have to be uploaded again.
        pub async fn set_return_to_launch_after_mission(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReturnToLaunchAfterMissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetReturnToLaunchAfterMissionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/mavsdk.rpc.mission.MissionService/SetReturnToLaunchAfterMission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "mavsdk.rpc.mission.MissionService",
                        "SetReturnToLaunchAfterMission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
