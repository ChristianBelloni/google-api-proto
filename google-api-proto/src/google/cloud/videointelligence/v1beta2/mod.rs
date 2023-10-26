/// Video annotation request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoRequest {
    /// Input video location. Currently, only
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>) URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>). A video URI
    /// may include wildcards in `object-id`, and thus identify multiple videos.
    /// Supported wildcards: '*' to match 0 or more characters;
    /// '?' to match 1 character. If unset, the input video should be embedded
    /// in the request as `input_content`. If set, `input_content` should be unset.
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// The video data bytes.
    /// If unset, the input video(s) should be specified via `input_uri`.
    /// If set, `input_uri` should be unset.
    #[prost(bytes = "bytes", tag = "6")]
    pub input_content: ::prost::bytes::Bytes,
    /// Required. Requested video annotation features.
    #[prost(enumeration = "Feature", repeated, packed = "false", tag = "2")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// Additional video context and/or feature-specific parameters.
    #[prost(message, optional, tag = "3")]
    pub video_context: ::core::option::Option<VideoContext>,
    /// Optional. Location where the output (in JSON format) should be stored.
    /// Currently, only [Google Cloud Storage](<https://cloud.google.com/storage/>)
    /// URIs are supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>).
    #[prost(string, tag = "4")]
    pub output_uri: ::prost::alloc::string::String,
    /// Optional. Cloud region where annotation should take place. Supported cloud
    /// regions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region
    /// is specified, a region will be determined based on video file location.
    #[prost(string, tag = "5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Video context and/or feature-specific parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoContext {
    /// Video segments to annotate. The segments may overlap and are not required
    /// to be contiguous or span the whole video. If unspecified, each video is
    /// treated as a single segment.
    #[prost(message, repeated, tag = "1")]
    pub segments: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Config for LABEL_DETECTION.
    #[prost(message, optional, tag = "2")]
    pub label_detection_config: ::core::option::Option<LabelDetectionConfig>,
    /// Config for SHOT_CHANGE_DETECTION.
    #[prost(message, optional, tag = "3")]
    pub shot_change_detection_config: ::core::option::Option<ShotChangeDetectionConfig>,
    /// Config for EXPLICIT_CONTENT_DETECTION.
    #[prost(message, optional, tag = "4")]
    pub explicit_content_detection_config: ::core::option::Option<
        ExplicitContentDetectionConfig,
    >,
    /// Config for FACE_DETECTION.
    #[prost(message, optional, tag = "5")]
    pub face_detection_config: ::core::option::Option<FaceDetectionConfig>,
}
/// Config for LABEL_DETECTION.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelDetectionConfig {
    /// What labels should be detected with LABEL_DETECTION, in addition to
    /// video-level labels or segment-level labels.
    /// If unspecified, defaults to `SHOT_MODE`.
    #[prost(enumeration = "LabelDetectionMode", tag = "1")]
    pub label_detection_mode: i32,
    /// Whether the video has been shot from a stationary (i.e. non-moving) camera.
    /// When set to true, might improve detection accuracy for moving objects.
    /// Should be used with `SHOT_AND_FRAME_MODE` enabled.
    #[prost(bool, tag = "2")]
    pub stationary_camera: bool,
    /// Model to use for label detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
}
/// Config for SHOT_CHANGE_DETECTION.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShotChangeDetectionConfig {
    /// Model to use for shot change detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for EXPLICIT_CONTENT_DETECTION.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentDetectionConfig {
    /// Model to use for explicit content detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for FACE_DETECTION.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceDetectionConfig {
    /// Model to use for face detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
    /// Whether bounding boxes be included in the face annotation output.
    #[prost(bool, tag = "2")]
    pub include_bounding_boxes: bool,
}
/// Video segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSegment {
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the start of the segment (inclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the end of the segment (inclusive).
    #[prost(message, optional, tag = "2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Video segment level annotation results for label detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSegment {
    /// Video segment where a label was detected.
    #[prost(message, optional, tag = "1")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// Confidence that the label is accurate. Range: \[0, 1\].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Video frame level annotation results for label detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Confidence that the label is accurate. Range: \[0, 1\].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Detected entity from video analysis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](<https://developers.google.com/knowledge-graph/>).
    #[prost(string, tag = "1")]
    pub entity_id: ::prost::alloc::string::String,
    /// Textual description, e.g. `Fixed-gear bicycle`.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Language code for `description` in BCP-47 format.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Label annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelAnnotation {
    /// Detected entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::core::option::Option<Entity>,
    /// Common categories for the detected entity.
    /// E.g. when the label is `Terrier` the category is likely `dog`. And in some
    /// cases there might be more than one categories e.g. `Terrier` could also be
    /// a `pet`.
    #[prost(message, repeated, tag = "2")]
    pub category_entities: ::prost::alloc::vec::Vec<Entity>,
    /// All video segments where a label was detected.
    #[prost(message, repeated, tag = "3")]
    pub segments: ::prost::alloc::vec::Vec<LabelSegment>,
    /// All video frames where a label was detected.
    #[prost(message, repeated, tag = "4")]
    pub frames: ::prost::alloc::vec::Vec<LabelFrame>,
}
/// Video frame level annotation results for explicit content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Likelihood of the pornography content..
    #[prost(enumeration = "Likelihood", tag = "2")]
    pub pornography_likelihood: i32,
}
/// Explicit content annotation (based on per-frame visual signals only).
/// If no explicit content has been detected in a frame, no annotations are
/// present for that frame.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentAnnotation {
    /// All video frames where explicit content was detected.
    #[prost(message, repeated, tag = "1")]
    pub frames: ::prost::alloc::vec::Vec<ExplicitContentFrame>,
}
/// Normalized bounding box.
/// The normalized vertex coordinates are relative to the original image.
/// Range: \[0, 1\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedBoundingBox {
    /// Left X coordinate.
    #[prost(float, tag = "1")]
    pub left: f32,
    /// Top Y coordinate.
    #[prost(float, tag = "2")]
    pub top: f32,
    /// Right X coordinate.
    #[prost(float, tag = "3")]
    pub right: f32,
    /// Bottom Y coordinate.
    #[prost(float, tag = "4")]
    pub bottom: f32,
}
/// Video segment level annotation results for face detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceSegment {
    /// Video segment where a face was detected.
    #[prost(message, optional, tag = "1")]
    pub segment: ::core::option::Option<VideoSegment>,
}
/// Video frame level annotation results for face detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceFrame {
    /// Normalized Bounding boxes in a frame.
    /// There can be more than one boxes if the same face is detected in multiple
    /// locations within the current frame.
    #[prost(message, repeated, tag = "1")]
    pub normalized_bounding_boxes: ::prost::alloc::vec::Vec<NormalizedBoundingBox>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the video frame for this location.
    #[prost(message, optional, tag = "2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Face annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaceAnnotation {
    /// Thumbnail of a representative face view (in JPEG format).
    #[prost(bytes = "bytes", tag = "1")]
    pub thumbnail: ::prost::bytes::Bytes,
    /// All video segments where a face was detected.
    #[prost(message, repeated, tag = "2")]
    pub segments: ::prost::alloc::vec::Vec<FaceSegment>,
    /// All video frames where a face was detected.
    #[prost(message, repeated, tag = "3")]
    pub frames: ::prost::alloc::vec::Vec<FaceFrame>,
}
/// Annotation results for a single video.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationResults {
    /// Video file location in
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Label annotations on video level or user specified segment level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "2")]
    pub segment_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on shot level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "3")]
    pub shot_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on frame level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "4")]
    pub frame_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Face annotations. There is exactly one element for each unique face.
    #[prost(message, repeated, tag = "5")]
    pub face_annotations: ::prost::alloc::vec::Vec<FaceAnnotation>,
    /// Shot annotations. Each shot is represented as a video segment.
    #[prost(message, repeated, tag = "6")]
    pub shot_annotations: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Explicit content annotation.
    #[prost(message, optional, tag = "7")]
    pub explicit_annotation: ::core::option::Option<ExplicitContentAnnotation>,
    /// If set, indicates an error. Note that for a single `AnnotateVideoRequest`
    /// some videos may succeed and some may fail.
    #[prost(message, optional, tag = "9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Video annotation response. Included in the `response`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoResponse {
    /// Annotation results for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag = "1")]
    pub annotation_results: ::prost::alloc::vec::Vec<VideoAnnotationResults>,
}
/// Annotation progress for a single video.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationProgress {
    /// Video file location in
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Approximate percentage processed thus far.
    /// Guaranteed to be 100 when fully processed.
    #[prost(int32, tag = "2")]
    pub progress_percent: i32,
    /// Time when the request was received.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time of the most recent update.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Video annotation progress. Included in the `metadata`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoProgress {
    /// Progress metadata for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag = "1")]
    pub annotation_progress: ::prost::alloc::vec::Vec<VideoAnnotationProgress>,
}
/// Video annotation feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Feature {
    /// Unspecified.
    Unspecified = 0,
    /// Label detection. Detect objects, such as dog or flower.
    LabelDetection = 1,
    /// Shot change detection.
    ShotChangeDetection = 2,
    /// Explicit content detection.
    ExplicitContentDetection = 3,
    /// Human face detection and tracking.
    FaceDetection = 4,
}
impl Feature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Feature::Unspecified => "FEATURE_UNSPECIFIED",
            Feature::LabelDetection => "LABEL_DETECTION",
            Feature::ShotChangeDetection => "SHOT_CHANGE_DETECTION",
            Feature::ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION",
            Feature::FaceDetection => "FACE_DETECTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FEATURE_UNSPECIFIED" => Some(Self::Unspecified),
            "LABEL_DETECTION" => Some(Self::LabelDetection),
            "SHOT_CHANGE_DETECTION" => Some(Self::ShotChangeDetection),
            "EXPLICIT_CONTENT_DETECTION" => Some(Self::ExplicitContentDetection),
            "FACE_DETECTION" => Some(Self::FaceDetection),
            _ => None,
        }
    }
}
/// Label detection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelDetectionMode {
    /// Unspecified.
    Unspecified = 0,
    /// Detect shot-level labels.
    ShotMode = 1,
    /// Detect frame-level labels.
    FrameMode = 2,
    /// Detect both shot-level and frame-level labels.
    ShotAndFrameMode = 3,
}
impl LabelDetectionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LabelDetectionMode::Unspecified => "LABEL_DETECTION_MODE_UNSPECIFIED",
            LabelDetectionMode::ShotMode => "SHOT_MODE",
            LabelDetectionMode::FrameMode => "FRAME_MODE",
            LabelDetectionMode::ShotAndFrameMode => "SHOT_AND_FRAME_MODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LABEL_DETECTION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOT_MODE" => Some(Self::ShotMode),
            "FRAME_MODE" => Some(Self::FrameMode),
            "SHOT_AND_FRAME_MODE" => Some(Self::ShotAndFrameMode),
            _ => None,
        }
    }
}
/// Bucketized representation of likelihood.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unspecified likelihood.
    Unspecified = 0,
    /// Very unlikely.
    VeryUnlikely = 1,
    /// Unlikely.
    Unlikely = 2,
    /// Possible.
    Possible = 3,
    /// Likely.
    Likely = 4,
    /// Very likely.
    VeryLikely = 5,
}
impl Likelihood {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Likelihood::Unspecified => "LIKELIHOOD_UNSPECIFIED",
            Likelihood::VeryUnlikely => "VERY_UNLIKELY",
            Likelihood::Unlikely => "UNLIKELY",
            Likelihood::Possible => "POSSIBLE",
            Likelihood::Likely => "LIKELY",
            Likelihood::VeryLikely => "VERY_LIKELY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIKELIHOOD_UNSPECIFIED" => Some(Self::Unspecified),
            "VERY_UNLIKELY" => Some(Self::VeryUnlikely),
            "UNLIKELY" => Some(Self::Unlikely),
            "POSSIBLE" => Some(Self::Possible),
            "LIKELY" => Some(Self::Likely),
            "VERY_LIKELY" => Some(Self::VeryLikely),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod video_intelligence_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that implements Google Cloud Video Intelligence API.
    #[derive(Debug, Clone)]
    pub struct VideoIntelligenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VideoIntelligenceServiceClient<T>
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
        ) -> VideoIntelligenceServiceClient<InterceptedService<T, F>>
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
            VideoIntelligenceServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Performs asynchronous video annotation. Progress and results can be
        /// retrieved through the `google.longrunning.Operations` interface.
        /// `Operation.metadata` contains `AnnotateVideoProgress` (progress).
        /// `Operation.response` contains `AnnotateVideoResponse` (results).
        pub async fn annotate_video(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateVideoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.videointelligence.v1beta2.VideoIntelligenceService/AnnotateVideo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.videointelligence.v1beta2.VideoIntelligenceService",
                        "AnnotateVideo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
