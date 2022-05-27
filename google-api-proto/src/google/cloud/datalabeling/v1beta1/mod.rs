/// An AnnotationSpecSet is a collection of label definitions. For example, in
/// image classification tasks, you define a set of possible labels for images as
/// an AnnotationSpecSet. An AnnotationSpecSet is immutable upon creation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSpecSet {
    /// Output only. The AnnotationSpecSet resource name in the following format:
    ///
    /// "projects/<var>{project_id}</var>/annotationSpecSets/<var>{annotation_spec_set_id}</var>"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name for AnnotationSpecSet that you define when you
    /// create it. Maximum of 64 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-provided description of the annotation specification set.
    /// The description can be up to 10,000 characters long.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Required. The array of AnnotationSpecs that you define when you create the
    /// AnnotationSpecSet. These are the possible labels for the labeling task.
    #[prost(message, repeated, tag="4")]
    pub annotation_specs: ::prost::alloc::vec::Vec<AnnotationSpec>,
    /// Output only. The names of any related resources that are blocking changes
    /// to the annotation spec set.
    #[prost(string, repeated, tag="5")]
    pub blocking_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Container of information related to one possible annotation that can be used
/// in a labeling task. For example, an image classification task where images
/// are labeled as `dog` or `cat` must reference an AnnotationSpec for `dog` and
/// an AnnotationSpec for `cat`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSpec {
    /// Required. The display name of the AnnotationSpec. Maximum of 64 characters.
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-provided description of the annotation specification.
    /// The description can be up to 10,000 characters long.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
/// Configuration for how human labeling task should be done.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanAnnotationConfig {
    /// Required. Instruction resource name.
    #[prost(string, tag="1")]
    pub instruction: ::prost::alloc::string::String,
    /// Required. A human-readable name for AnnotatedDataset defined by
    /// users. Maximum of 64 characters
    /// .
    #[prost(string, tag="2")]
    pub annotated_dataset_display_name: ::prost::alloc::string::String,
    /// Optional. A human-readable description for AnnotatedDataset.
    /// The description can be up to 10000 characters long.
    #[prost(string, tag="3")]
    pub annotated_dataset_description: ::prost::alloc::string::String,
    /// Optional. A human-readable label used to logically group labeling tasks.
    /// This string must match the regular expression `\[a-zA-Z\\d_-\]{0,128}`.
    #[prost(string, tag="4")]
    pub label_group: ::prost::alloc::string::String,
    /// Optional. The Language of this question, as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>).
    /// Default value is en-US.
    /// Only need to set this when task is language related. For example, French
    /// text classification.
    #[prost(string, tag="5")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Replication of questions. Each question will be sent to up to
    /// this number of contributors to label. Aggregated answers will be returned.
    /// Default is set to 1.
    /// For image related labeling, valid values are 1, 3, 5.
    #[prost(int32, tag="6")]
    pub replica_count: i32,
    /// Optional. Maximum duration for contributors to answer a question. Maximum
    /// is 3600 seconds. Default is 3600 seconds.
    #[prost(message, optional, tag="7")]
    pub question_duration: ::core::option::Option<::prost_types::Duration>,
    /// Optional. If you want your own labeling contributors to manage and work on
    /// this labeling request, you can set these contributors here. We will give
    /// them access to the question types in crowdcompute. Note that these
    /// emails must be registered in crowdcompute worker UI:
    /// <https://crowd-compute.appspot.com/>
    #[prost(string, repeated, tag="9")]
    pub contributor_emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Email of the user who started the labeling task and should be notified by
    /// email. If empty no notification will be sent.
    #[prost(string, tag="10")]
    pub user_email_address: ::prost::alloc::string::String,
}
/// Config for image classification human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationConfig {
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Optional. If allow_multi_label is true, contributors are able to choose
    /// multiple labels for one image.
    #[prost(bool, tag="2")]
    pub allow_multi_label: bool,
    /// Optional. The type of how to aggregate answers.
    #[prost(enumeration="StringAggregationType", tag="3")]
    pub answer_aggregation_type: i32,
}
/// Config for image bounding poly (and bounding box) human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPolyConfig {
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Optional. Instruction message showed on contributors UI.
    #[prost(string, tag="2")]
    pub instruction_message: ::prost::alloc::string::String,
}
/// Config for image polyline human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolylineConfig {
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Optional. Instruction message showed on contributors UI.
    #[prost(string, tag="2")]
    pub instruction_message: ::prost::alloc::string::String,
}
/// Config for image segmentation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentationConfig {
    /// Required. Annotation spec set resource name. format:
    /// projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Instruction message showed on labelers UI.
    #[prost(string, tag="2")]
    pub instruction_message: ::prost::alloc::string::String,
}
/// Config for video classification human labeling task.
/// Currently two types of video classification are supported:
/// 1. Assign labels on the entire video.
/// 2. Split the video into multiple video clips based on camera shot, and
/// assign labels on each video clip.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationConfig {
    /// Required. The list of annotation spec set configs.
    /// Since watching a video clip takes much longer time than an image, we
    /// support label with multiple AnnotationSpecSet at the same time. Labels
    /// in each AnnotationSpecSet will be shown in a group to contributors.
    /// Contributors can select one or more (depending on whether to allow multi
    /// label) from each group.
    #[prost(message, repeated, tag="1")]
    pub annotation_spec_set_configs: ::prost::alloc::vec::Vec<video_classification_config::AnnotationSpecSetConfig>,
    /// Optional. Option to apply shot detection on the video.
    #[prost(bool, tag="2")]
    pub apply_shot_detection: bool,
}
/// Nested message and enum types in `VideoClassificationConfig`.
pub mod video_classification_config {
    /// Annotation spec set with the setting of allowing multi labels or not.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnnotationSpecSetConfig {
        /// Required. Annotation spec set resource name.
        #[prost(string, tag="1")]
        pub annotation_spec_set: ::prost::alloc::string::String,
        /// Optional. If allow_multi_label is true, contributors are able to
        /// choose multiple labels from one annotation spec set.
        #[prost(bool, tag="2")]
        pub allow_multi_label: bool,
    }
}
/// Config for video object detection human labeling task.
/// Object detection will be conducted on the images extracted from the video,
/// and those objects will be labeled with bounding boxes.
/// User need to specify the number of images to be extracted per second as the
/// extraction frame rate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectDetectionConfig {
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Required. Number of frames per second to be extracted from the video.
    #[prost(double, tag="3")]
    pub extraction_frame_rate: f64,
}
/// Config for video object tracking human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectTrackingConfig {
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
}
/// Config for video event human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventConfig {
    /// Required. The list of annotation spec set resource name. Similar to video
    /// classification, we support selecting event from multiple AnnotationSpecSet
    /// at the same time.
    #[prost(string, repeated, tag="1")]
    pub annotation_spec_sets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Config for text classification human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationConfig {
    /// Optional. If allow_multi_label is true, contributors are able to choose
    /// multiple labels for one text segment.
    #[prost(bool, tag="1")]
    pub allow_multi_label: bool,
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="2")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Optional. Configs for sentiment selection.
    #[prost(message, optional, tag="3")]
    pub sentiment_config: ::core::option::Option<SentimentConfig>,
}
/// Config for setting up sentiments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentConfig {
    /// If set to true, contributors will have the option to select sentiment of
    /// the label they selected, to mark it as negative or positive label. Default
    /// is false.
    #[prost(bool, tag="1")]
    pub enable_label_sentiment_selection: bool,
}
/// Config for text entity extraction human labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextEntityExtractionConfig {
    /// Required. Annotation spec set resource name.
    #[prost(string, tag="1")]
    pub annotation_spec_set: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StringAggregationType {
    Unspecified = 0,
    /// Majority vote to aggregate answers.
    MajorityVote = 1,
    /// Unanimous answers will be adopted.
    UnanimousVote = 2,
    /// Preserve all answers by crowd compute.
    NoAggregation = 3,
}
/// Annotation for Example. Each example may have one or more annotations. For
/// example in image classification problem, each image might have one or more
/// labels. We call labels binded with this image an Annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    /// Output only. Unique name of this annotation, format is:
    ///
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset}/examples/{example_id}/annotations/{annotation_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The source of the annotation.
    #[prost(enumeration="AnnotationSource", tag="2")]
    pub annotation_source: i32,
    /// Output only. This is the actual annotation value, e.g classification,
    /// bounding box values are stored here.
    #[prost(message, optional, tag="3")]
    pub annotation_value: ::core::option::Option<AnnotationValue>,
    /// Output only. Annotation metadata, including information like votes
    /// for labels.
    #[prost(message, optional, tag="4")]
    pub annotation_metadata: ::core::option::Option<AnnotationMetadata>,
    /// Output only. Sentiment for this annotation.
    #[prost(enumeration="AnnotationSentiment", tag="6")]
    pub annotation_sentiment: i32,
}
/// Annotation value for an example.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationValue {
    #[prost(oneof="annotation_value::ValueType", tags="1, 2, 8, 9, 3, 10, 4, 5, 6")]
    pub value_type: ::core::option::Option<annotation_value::ValueType>,
}
/// Nested message and enum types in `AnnotationValue`.
pub mod annotation_value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueType {
        /// Annotation value for image classification case.
        #[prost(message, tag="1")]
        ImageClassificationAnnotation(super::ImageClassificationAnnotation),
        /// Annotation value for image bounding box, oriented bounding box
        /// and polygon cases.
        #[prost(message, tag="2")]
        ImageBoundingPolyAnnotation(super::ImageBoundingPolyAnnotation),
        /// Annotation value for image polyline cases.
        /// Polyline here is different from BoundingPoly. It is formed by
        /// line segments connected to each other but not closed form(Bounding Poly).
        /// The line segments can cross each other.
        #[prost(message, tag="8")]
        ImagePolylineAnnotation(super::ImagePolylineAnnotation),
        /// Annotation value for image segmentation.
        #[prost(message, tag="9")]
        ImageSegmentationAnnotation(super::ImageSegmentationAnnotation),
        /// Annotation value for text classification case.
        #[prost(message, tag="3")]
        TextClassificationAnnotation(super::TextClassificationAnnotation),
        /// Annotation value for text entity extraction case.
        #[prost(message, tag="10")]
        TextEntityExtractionAnnotation(super::TextEntityExtractionAnnotation),
        /// Annotation value for video classification case.
        #[prost(message, tag="4")]
        VideoClassificationAnnotation(super::VideoClassificationAnnotation),
        /// Annotation value for video object detection and tracking case.
        #[prost(message, tag="5")]
        VideoObjectTrackingAnnotation(super::VideoObjectTrackingAnnotation),
        /// Annotation value for video event case.
        #[prost(message, tag="6")]
        VideoEventAnnotation(super::VideoEventAnnotation),
    }
}
/// Image classification annotation definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationAnnotation {
    /// Label of image.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag="1")]
    pub x: i32,
    /// Y coordinate.
    #[prost(int32, tag="2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag="1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag="2")]
    pub y: f32,
}
/// A bounding polygon in the image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag="1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
}
/// Normalized bounding polygon.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedBoundingPoly {
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag="1")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Image bounding poly annotation. It represents a polygon including
/// bounding box in the image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageBoundingPolyAnnotation {
    /// Label of object in this bounding polygon.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
    /// The region of the polygon. If it is a bounding box, it is guaranteed to be
    /// four points.
    #[prost(oneof="image_bounding_poly_annotation::BoundedArea", tags="2, 3")]
    pub bounded_area: ::core::option::Option<image_bounding_poly_annotation::BoundedArea>,
}
/// Nested message and enum types in `ImageBoundingPolyAnnotation`.
pub mod image_bounding_poly_annotation {
    /// The region of the polygon. If it is a bounding box, it is guaranteed to be
    /// four points.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BoundedArea {
        #[prost(message, tag="2")]
        BoundingPoly(super::BoundingPoly),
        #[prost(message, tag="3")]
        NormalizedBoundingPoly(super::NormalizedBoundingPoly),
    }
}
/// A line with multiple line segments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polyline {
    /// The polyline vertices.
    #[prost(message, repeated, tag="1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
}
/// Normalized polyline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedPolyline {
    /// The normalized polyline vertices.
    #[prost(message, repeated, tag="1")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// A polyline for the image annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagePolylineAnnotation {
    /// Label of this polyline.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
    #[prost(oneof="image_polyline_annotation::Poly", tags="2, 3")]
    pub poly: ::core::option::Option<image_polyline_annotation::Poly>,
}
/// Nested message and enum types in `ImagePolylineAnnotation`.
pub mod image_polyline_annotation {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Poly {
        #[prost(message, tag="2")]
        Polyline(super::Polyline),
        #[prost(message, tag="3")]
        NormalizedPolyline(super::NormalizedPolyline),
    }
}
/// Image segmentation annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationAnnotation {
    /// The mapping between rgb color and annotation spec. The key is the rgb
    /// color represented in format of rgb(0, 0, 0). The value is the
    /// AnnotationSpec.
    #[prost(btree_map="string, message", tag="1")]
    pub annotation_colors: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, AnnotationSpec>,
    /// Image format.
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
    /// A byte string of a full image's color map.
    #[prost(bytes="bytes", tag="3")]
    pub image_bytes: ::prost::bytes::Bytes,
}
/// Text classification annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationAnnotation {
    /// Label of the text.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
}
/// Text entity extraction annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextEntityExtractionAnnotation {
    /// Label of the text entities.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
    /// Position of the entity.
    #[prost(message, optional, tag="2")]
    pub sequential_segment: ::core::option::Option<SequentialSegment>,
}
/// Start and end position in a sequence (e.g. text segment).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequentialSegment {
    /// Start position (inclusive).
    #[prost(int32, tag="1")]
    pub start: i32,
    /// End position (exclusive).
    #[prost(int32, tag="2")]
    pub end: i32,
}
/// A time period inside of an example that has a time dimension (e.g. video).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSegment {
    /// Start of the time segment (inclusive), represented as the duration since
    /// the example start.
    #[prost(message, optional, tag="1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// End of the time segment (exclusive), represented as the duration since the
    /// example start.
    #[prost(message, optional, tag="2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Video classification annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationAnnotation {
    /// The time segment of the video to which the annotation applies.
    #[prost(message, optional, tag="1")]
    pub time_segment: ::core::option::Option<TimeSegment>,
    /// Label of the segment specified by time_segment.
    #[prost(message, optional, tag="2")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
}
/// Video frame level annotation for object detection and tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectTrackingFrame {
    /// The time offset of this frame relative to the beginning of the video.
    #[prost(message, optional, tag="3")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// The bounding box location of this object track for the frame.
    #[prost(oneof="object_tracking_frame::BoundedArea", tags="1, 2")]
    pub bounded_area: ::core::option::Option<object_tracking_frame::BoundedArea>,
}
/// Nested message and enum types in `ObjectTrackingFrame`.
pub mod object_tracking_frame {
    /// The bounding box location of this object track for the frame.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BoundedArea {
        #[prost(message, tag="1")]
        BoundingPoly(super::BoundingPoly),
        #[prost(message, tag="2")]
        NormalizedBoundingPoly(super::NormalizedBoundingPoly),
    }
}
/// Video object tracking annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingAnnotation {
    /// Label of the object tracked in this annotation.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
    /// The time segment of the video to which object tracking applies.
    #[prost(message, optional, tag="2")]
    pub time_segment: ::core::option::Option<TimeSegment>,
    /// The list of frames where this object track appears.
    #[prost(message, repeated, tag="3")]
    pub object_tracking_frames: ::prost::alloc::vec::Vec<ObjectTrackingFrame>,
}
/// Video event annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoEventAnnotation {
    /// Label of the event in this annotation.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
    /// The time segment of the video to which the annotation applies.
    #[prost(message, optional, tag="2")]
    pub time_segment: ::core::option::Option<TimeSegment>,
}
/// Additional information associated with the annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationMetadata {
    /// Metadata related to human labeling.
    #[prost(message, optional, tag="2")]
    pub operator_metadata: ::core::option::Option<OperatorMetadata>,
}
/// General information useful for labels coming from contributors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorMetadata {
    /// Confidence score corresponding to a label. For examle, if 3 contributors
    /// have answered the question and 2 of them agree on the final label, the
    /// confidence score will be 0.67 (2/3).
    #[prost(float, tag="1")]
    pub score: f32,
    /// The total number of contributors that answer this question.
    #[prost(int32, tag="2")]
    pub total_votes: i32,
    /// The total number of contributors that choose this label.
    #[prost(int32, tag="3")]
    pub label_votes: i32,
    /// Comments from contributors.
    #[prost(string, repeated, tag="4")]
    pub comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Specifies where the annotation comes from (whether it was provided by a
/// human labeler or a different source).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationSource {
    Unspecified = 0,
    /// Answer is provided by a human contributor.
    Operator = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationSentiment {
    Unspecified = 0,
    /// This annotation describes negatively about the data.
    Negative = 1,
    /// This label describes positively about the data.
    Positive = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnotationType {
    Unspecified = 0,
    /// Classification annotations in an image. Allowed for continuous evaluation.
    ImageClassificationAnnotation = 1,
    /// Bounding box annotations in an image. A form of image object detection.
    /// Allowed for continuous evaluation.
    ImageBoundingBoxAnnotation = 2,
    /// Oriented bounding box. The box does not have to be parallel to horizontal
    /// line.
    ImageOrientedBoundingBoxAnnotation = 13,
    /// Bounding poly annotations in an image.
    ImageBoundingPolyAnnotation = 10,
    /// Polyline annotations in an image.
    ImagePolylineAnnotation = 11,
    /// Segmentation annotations in an image.
    ImageSegmentationAnnotation = 12,
    /// Classification annotations in video shots.
    VideoShotsClassificationAnnotation = 3,
    /// Video object tracking annotation.
    VideoObjectTrackingAnnotation = 4,
    /// Video object detection annotation.
    VideoObjectDetectionAnnotation = 5,
    /// Video event annotation.
    VideoEventAnnotation = 6,
    /// Classification for text. Allowed for continuous evaluation.
    TextClassificationAnnotation = 8,
    /// Entity extraction for text.
    TextEntityExtractionAnnotation = 9,
    /// General classification. Allowed for continuous evaluation.
    GeneralClassificationAnnotation = 14,
}
/// Container of information about an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagePayload {
    /// Image format.
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    /// A byte string of a thumbnail image.
    #[prost(bytes="bytes", tag="2")]
    pub image_thumbnail: ::prost::bytes::Bytes,
    /// Image uri from the user bucket.
    #[prost(string, tag="3")]
    pub image_uri: ::prost::alloc::string::String,
    /// Signed uri of the image file in the service bucket.
    #[prost(string, tag="4")]
    pub signed_uri: ::prost::alloc::string::String,
}
/// Container of information about a piece of text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextPayload {
    /// Text content.
    #[prost(string, tag="1")]
    pub text_content: ::prost::alloc::string::String,
}
/// Container of information of a video thumbnail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoThumbnail {
    /// A byte string of the video frame.
    #[prost(bytes="bytes", tag="1")]
    pub thumbnail: ::prost::bytes::Bytes,
    /// Time offset relative to the beginning of the video, corresponding to the
    /// video frame where the thumbnail has been extracted from.
    #[prost(message, optional, tag="2")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Container of information of a video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoPayload {
    /// Video format.
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Video uri from the user bucket.
    #[prost(string, tag="2")]
    pub video_uri: ::prost::alloc::string::String,
    /// The list of video thumbnails.
    #[prost(message, repeated, tag="3")]
    pub video_thumbnails: ::prost::alloc::vec::Vec<VideoThumbnail>,
    /// FPS of the video.
    #[prost(float, tag="4")]
    pub frame_rate: f32,
    /// Signed uri of the video file in the service bucket.
    #[prost(string, tag="5")]
    pub signed_uri: ::prost::alloc::string::String,
}
/// Dataset is the resource to hold your data. You can request multiple labeling
/// tasks for a dataset while each one will generate an AnnotatedDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Output only. Dataset resource name, format is:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the dataset. Maximum of 64 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-provided description of the annotation specification set.
    /// The description can be up to 10000 characters long.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time the dataset is created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. This is populated with the original input configs
    /// where ImportData is called. It is available only after the clients
    /// import data to this dataset.
    #[prost(message, repeated, tag="5")]
    pub input_configs: ::prost::alloc::vec::Vec<InputConfig>,
    /// Output only. The names of any related resources that are blocking changes
    /// to the dataset.
    #[prost(string, repeated, tag="6")]
    pub blocking_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The number of data items in the dataset.
    #[prost(int64, tag="7")]
    pub data_item_count: i64,
}
/// The configuration of input data, including data type, location, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. Data type must be specifed when user tries to import data.
    #[prost(enumeration="DataType", tag="1")]
    pub data_type: i32,
    /// Optional. The type of annotation to be performed on this data. You must
    /// specify this field if you are using this InputConfig in an
    /// \[EvaluationJob][google.cloud.datalabeling.v1beta1.EvaluationJob\].
    #[prost(enumeration="AnnotationType", tag="3")]
    pub annotation_type: i32,
    /// Optional. Metadata about annotations for the input. You must specify this
    /// field if you are using this InputConfig in an \[EvaluationJob][google.cloud.datalabeling.v1beta1.EvaluationJob\] for a
    /// model version that performs classification.
    #[prost(message, optional, tag="4")]
    pub classification_metadata: ::core::option::Option<ClassificationMetadata>,
    /// Optional. The metadata associated with each data type.
    #[prost(oneof="input_config::DataTypeMetadata", tags="6")]
    pub data_type_metadata: ::core::option::Option<input_config::DataTypeMetadata>,
    /// Required. Where the data is from.
    #[prost(oneof="input_config::Source", tags="2, 5")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// Optional. The metadata associated with each data type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataTypeMetadata {
        /// Required for text import, as language code must be specified.
        #[prost(message, tag="6")]
        TextMetadata(super::TextMetadata),
    }
    /// Required. Where the data is from.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Source located in Cloud Storage.
        #[prost(message, tag="2")]
        GcsSource(super::GcsSource),
        /// Source located in BigQuery. You must specify this field if you are using
        /// this InputConfig in an \[EvaluationJob][google.cloud.datalabeling.v1beta1.EvaluationJob\].
        #[prost(message, tag="5")]
        BigquerySource(super::BigQuerySource),
    }
}
/// Metadata for the text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextMetadata {
    /// The language of this text, as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>).
    /// Default value is en-US.
    #[prost(string, tag="1")]
    pub language_code: ::prost::alloc::string::String,
}
/// Metadata for classification annotations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationMetadata {
    /// Whether the classification task is multi-label or not.
    #[prost(bool, tag="1")]
    pub is_multi_label: bool,
}
/// Source of the Cloud Storage file to be imported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. The input URI of source file. This must be a Cloud Storage path
    /// (`gs://...`).
    #[prost(string, tag="1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Required. The format of the source file. Only "text/csv" is supported.
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// The BigQuery location for input data. If used in an \[EvaluationJob][google.cloud.datalabeling.v1beta1.EvaluationJob\], this
/// is where the service saves the prediction input and output sampled from the
/// model version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// Required. BigQuery URI to a table, up to 2,000 characters long. If you
    /// specify the URI of a table that does not exist, Data Labeling Service
    /// creates a table at the URI with the correct schema when you create your
    /// \[EvaluationJob][google.cloud.datalabeling.v1beta1.EvaluationJob\]. If you specify the URI of a table that already exists,
    /// it must have the
    /// [correct
    /// schema](/ml-engine/docs/continuous-evaluation/create-job#table-schema).
    ///
    /// Provide the table URI in the following format:
    ///
    /// "bq://<var>{your_project_id}</var>/<var>{your_dataset_name}</var>/<var>{your_table_name}</var>"
    ///
    /// [Learn
    /// more](/ml-engine/docs/continuous-evaluation/create-job#table-schema).
    #[prost(string, tag="1")]
    pub input_uri: ::prost::alloc::string::String,
}
/// The configuration of output data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Required. Location to output data to.
    #[prost(oneof="output_config::Destination", tags="1, 2")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Required. Location to output data to.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output to a file in Cloud Storage. Should be used for labeling output
        /// other than image segmentation.
        #[prost(message, tag="1")]
        GcsDestination(super::GcsDestination),
        /// Output to a folder in Cloud Storage. Should be used for image
        /// segmentation labeling output.
        #[prost(message, tag="2")]
        GcsFolderDestination(super::GcsFolderDestination),
    }
}
/// Export destination of the data.Only gcs path is allowed in
/// output_uri.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required. The output uri of destination file.
    #[prost(string, tag="1")]
    pub output_uri: ::prost::alloc::string::String,
    /// Required. The format of the gcs destination. Only "text/csv" and
    /// "application/json"
    /// are supported.
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Export folder destination of the data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFolderDestination {
    /// Required. Cloud Storage directory to export data to.
    #[prost(string, tag="1")]
    pub output_folder_uri: ::prost::alloc::string::String,
}
/// DataItem is a piece of data, without annotation. For example, an image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataItem {
    /// Output only. Name of the data item, in format of:
    /// projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only.
    #[prost(oneof="data_item::Payload", tags="2, 3, 4")]
    pub payload: ::core::option::Option<data_item::Payload>,
}
/// Nested message and enum types in `DataItem`.
pub mod data_item {
    /// Output only.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The image payload, a container of the image bytes/uri.
        #[prost(message, tag="2")]
        ImagePayload(super::ImagePayload),
        /// The text payload, a container of text content.
        #[prost(message, tag="3")]
        TextPayload(super::TextPayload),
        /// The video payload, a container of the video uri.
        #[prost(message, tag="4")]
        VideoPayload(super::VideoPayload),
    }
}
/// AnnotatedDataset is a set holding annotations for data in a Dataset. Each
/// labeling task will generate an AnnotatedDataset under the Dataset that the
/// task is requested for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedDataset {
    /// Output only. AnnotatedDataset resource name in format of:
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/
    /// {annotated_dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The display name of the AnnotatedDataset. It is specified in
    /// HumanAnnotationConfig when user starts a labeling task. Maximum of 64
    /// characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The description of the AnnotatedDataset. It is specified in
    /// HumanAnnotationConfig when user starts a labeling task. Maximum of 10000
    /// characters.
    #[prost(string, tag="9")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Source of the annotation.
    #[prost(enumeration="AnnotationSource", tag="3")]
    pub annotation_source: i32,
    /// Output only. Type of the annotation. It is specified when starting labeling
    /// task.
    #[prost(enumeration="AnnotationType", tag="8")]
    pub annotation_type: i32,
    /// Output only. Number of examples in the annotated dataset.
    #[prost(int64, tag="4")]
    pub example_count: i64,
    /// Output only. Number of examples that have annotation in the annotated
    /// dataset.
    #[prost(int64, tag="5")]
    pub completed_example_count: i64,
    /// Output only. Per label statistics.
    #[prost(message, optional, tag="6")]
    pub label_stats: ::core::option::Option<LabelStats>,
    /// Output only. Time the AnnotatedDataset was created.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Additional information about AnnotatedDataset.
    #[prost(message, optional, tag="10")]
    pub metadata: ::core::option::Option<AnnotatedDatasetMetadata>,
    /// Output only. The names of any related resources that are blocking changes
    /// to the annotated dataset.
    #[prost(string, repeated, tag="11")]
    pub blocking_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Statistics about annotation specs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelStats {
    /// Map of each annotation spec's example count. Key is the annotation spec
    /// name and value is the number of examples for that annotation spec.
    /// If the annotated dataset does not have annotation spec, the map will return
    /// a pair where the key is empty string and value is the total number of
    /// annotations.
    #[prost(btree_map="string, int64", tag="1")]
    pub example_count: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, i64>,
}
/// Metadata on AnnotatedDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotatedDatasetMetadata {
    /// HumanAnnotationConfig used when requesting the human labeling task for this
    /// AnnotatedDataset.
    #[prost(message, optional, tag="1")]
    pub human_annotation_config: ::core::option::Option<HumanAnnotationConfig>,
    /// Specific request configuration used when requesting the labeling task.
    #[prost(oneof="annotated_dataset_metadata::AnnotationRequestConfig", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub annotation_request_config: ::core::option::Option<annotated_dataset_metadata::AnnotationRequestConfig>,
}
/// Nested message and enum types in `AnnotatedDatasetMetadata`.
pub mod annotated_dataset_metadata {
    /// Specific request configuration used when requesting the labeling task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AnnotationRequestConfig {
        /// Configuration for image classification task.
        #[prost(message, tag="2")]
        ImageClassificationConfig(super::ImageClassificationConfig),
        /// Configuration for image bounding box and bounding poly task.
        #[prost(message, tag="3")]
        BoundingPolyConfig(super::BoundingPolyConfig),
        /// Configuration for image polyline task.
        #[prost(message, tag="4")]
        PolylineConfig(super::PolylineConfig),
        /// Configuration for image segmentation task.
        #[prost(message, tag="5")]
        SegmentationConfig(super::SegmentationConfig),
        /// Configuration for video classification task.
        #[prost(message, tag="6")]
        VideoClassificationConfig(super::VideoClassificationConfig),
        /// Configuration for video object detection task.
        #[prost(message, tag="7")]
        ObjectDetectionConfig(super::ObjectDetectionConfig),
        /// Configuration for video object tracking task.
        #[prost(message, tag="8")]
        ObjectTrackingConfig(super::ObjectTrackingConfig),
        /// Configuration for video event labeling task.
        #[prost(message, tag="9")]
        EventConfig(super::EventConfig),
        /// Configuration for text classification task.
        #[prost(message, tag="10")]
        TextClassificationConfig(super::TextClassificationConfig),
        /// Configuration for text entity extraction task.
        #[prost(message, tag="11")]
        TextEntityExtractionConfig(super::TextEntityExtractionConfig),
    }
}
/// An Example is a piece of data and its annotation. For example, an image with
/// label "house".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Example {
    /// Output only. Name of the example, in format of:
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/
    /// {annotated_dataset_id}/examples/{example_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Annotations for the piece of data in Example.
    /// One piece of data can have multiple annotations.
    #[prost(message, repeated, tag="5")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// Output only. The data part of Example.
    #[prost(oneof="example::Payload", tags="2, 6, 7")]
    pub payload: ::core::option::Option<example::Payload>,
}
/// Nested message and enum types in `Example`.
pub mod example {
    /// Output only. The data part of Example.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The image payload, a container of the image bytes/uri.
        #[prost(message, tag="2")]
        ImagePayload(super::ImagePayload),
        /// The text payload, a container of the text content.
        #[prost(message, tag="6")]
        TextPayload(super::TextPayload),
        /// The video payload, a container of the video uri.
        #[prost(message, tag="7")]
        VideoPayload(super::VideoPayload),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    Unspecified = 0,
    /// Allowed for continuous evaluation.
    Image = 1,
    Video = 2,
    /// Allowed for continuous evaluation.
    Text = 4,
    /// Allowed for continuous evaluation.
    GeneralData = 6,
}
/// Describes an evaluation between a machine learning model's predictions and
/// ground truth labels. Created when an \[EvaluationJob][google.cloud.datalabeling.v1beta1.EvaluationJob\] runs successfully.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evaluation {
    /// Output only. Resource name of an evaluation. The name has the following
    /// format:
    ///
    /// "projects/<var>{project_id}</var>/datasets/<var>{dataset_id}</var>/evaluations/<var>{evaluation_id</var>}'
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Options used in the evaluation job that created this
    /// evaluation.
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<EvaluationConfig>,
    /// Output only. Timestamp for when the evaluation job that created this
    /// evaluation ran.
    #[prost(message, optional, tag="3")]
    pub evaluation_job_run_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp for when this evaluation was created.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Metrics comparing predictions to ground truth labels.
    #[prost(message, optional, tag="5")]
    pub evaluation_metrics: ::core::option::Option<EvaluationMetrics>,
    /// Output only. Type of task that the model version being evaluated performs,
    /// as defined in the
    ///
    /// \[evaluationJobConfig.inputConfig.annotationType][google.cloud.datalabeling.v1beta1.EvaluationJobConfig.input_config\]
    /// field of the evaluation job that created this evaluation.
    #[prost(enumeration="AnnotationType", tag="6")]
    pub annotation_type: i32,
    /// Output only. The number of items in the ground truth dataset that were used
    /// for this evaluation. Only populated when the evaulation is for certain
    /// AnnotationTypes.
    #[prost(int64, tag="7")]
    pub evaluated_item_count: i64,
}
/// Configuration details used for calculating evaluation metrics and creating an
/// \[Evaluation][google.cloud.datalabeling.v1beta1.Evaluation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationConfig {
    /// Vertical specific options for general metrics.
    #[prost(oneof="evaluation_config::VerticalOption", tags="1")]
    pub vertical_option: ::core::option::Option<evaluation_config::VerticalOption>,
}
/// Nested message and enum types in `EvaluationConfig`.
pub mod evaluation_config {
    /// Vertical specific options for general metrics.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VerticalOption {
        /// Only specify this field if the related model performs image object
        /// detection (`IMAGE_BOUNDING_BOX_ANNOTATION`). Describes how to evaluate
        /// bounding boxes.
        #[prost(message, tag="1")]
        BoundingBoxEvaluationOptions(super::BoundingBoxEvaluationOptions),
    }
}
/// Options regarding evaluation between bounding boxes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBoxEvaluationOptions {
    /// Minimum
    /// [intersection-over-union
    ///
    /// (IOU)](/vision/automl/object-detection/docs/evaluate#intersection-over-union)
    /// required for 2 bounding boxes to be considered a match. This must be a
    /// number between 0 and 1.
    #[prost(float, tag="1")]
    pub iou_threshold: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationMetrics {
    /// Common metrics covering most general cases.
    #[prost(oneof="evaluation_metrics::Metrics", tags="1, 2")]
    pub metrics: ::core::option::Option<evaluation_metrics::Metrics>,
}
/// Nested message and enum types in `EvaluationMetrics`.
pub mod evaluation_metrics {
    /// Common metrics covering most general cases.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metrics {
        #[prost(message, tag="1")]
        ClassificationMetrics(super::ClassificationMetrics),
        #[prost(message, tag="2")]
        ObjectDetectionMetrics(super::ObjectDetectionMetrics),
    }
}
/// Metrics calculated for a classification model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationMetrics {
    /// Precision-recall curve based on ground truth labels, predicted labels, and
    /// scores for the predicted labels.
    #[prost(message, optional, tag="1")]
    pub pr_curve: ::core::option::Option<PrCurve>,
    /// Confusion matrix of predicted labels vs. ground truth labels.
    #[prost(message, optional, tag="2")]
    pub confusion_matrix: ::core::option::Option<ConfusionMatrix>,
}
/// Metrics calculated for an image object detection (bounding box) model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectDetectionMetrics {
    /// Precision-recall curve.
    #[prost(message, optional, tag="1")]
    pub pr_curve: ::core::option::Option<PrCurve>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrCurve {
    /// The annotation spec of the label for which the precision-recall curve
    /// calculated. If this field is empty, that means the precision-recall curve
    /// is an aggregate curve for all labels.
    #[prost(message, optional, tag="1")]
    pub annotation_spec: ::core::option::Option<AnnotationSpec>,
    /// Area under the precision-recall curve. Not to be confused with area under
    /// a receiver operating characteristic (ROC) curve.
    #[prost(float, tag="2")]
    pub area_under_curve: f32,
    /// Entries that make up the precision-recall graph. Each entry is a "point" on
    /// the graph drawn for a different `confidence_threshold`.
    #[prost(message, repeated, tag="3")]
    pub confidence_metrics_entries: ::prost::alloc::vec::Vec<pr_curve::ConfidenceMetricsEntry>,
    /// Mean average prcision of this curve.
    #[prost(float, tag="4")]
    pub mean_average_precision: f32,
}
/// Nested message and enum types in `PrCurve`.
pub mod pr_curve {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfidenceMetricsEntry {
        /// Threshold used for this entry.
        ///
        /// For classification tasks, this is a classification threshold: a
        /// predicted label is categorized as positive or negative (in the context of
        /// this point on the PR curve) based on whether the label's score meets this
        /// threshold.
        ///
        /// For image object detection (bounding box) tasks, this is the
        /// [intersection-over-union
        ///
        /// (IOU)](/vision/automl/object-detection/docs/evaluate#intersection-over-union)
        /// threshold for the context of this point on the PR curve.
        #[prost(float, tag="1")]
        pub confidence_threshold: f32,
        /// Recall value.
        #[prost(float, tag="2")]
        pub recall: f32,
        /// Precision value.
        #[prost(float, tag="3")]
        pub precision: f32,
        /// Harmonic mean of recall and precision.
        #[prost(float, tag="4")]
        pub f1_score: f32,
        /// Recall value for entries with label that has highest score.
        #[prost(float, tag="5")]
        pub recall_at1: f32,
        /// Precision value for entries with label that has highest score.
        #[prost(float, tag="6")]
        pub precision_at1: f32,
        /// The harmonic mean of \[recall_at1][google.cloud.datalabeling.v1beta1.PrCurve.ConfidenceMetricsEntry.recall_at1\] and \[precision_at1][google.cloud.datalabeling.v1beta1.PrCurve.ConfidenceMetricsEntry.precision_at1\].
        #[prost(float, tag="7")]
        pub f1_score_at1: f32,
        /// Recall value for entries with label that has highest 5 scores.
        #[prost(float, tag="8")]
        pub recall_at5: f32,
        /// Precision value for entries with label that has highest 5 scores.
        #[prost(float, tag="9")]
        pub precision_at5: f32,
        /// The harmonic mean of \[recall_at5][google.cloud.datalabeling.v1beta1.PrCurve.ConfidenceMetricsEntry.recall_at5\] and \[precision_at5][google.cloud.datalabeling.v1beta1.PrCurve.ConfidenceMetricsEntry.precision_at5\].
        #[prost(float, tag="10")]
        pub f1_score_at5: f32,
    }
}
/// Confusion matrix of the model running the classification. Only applicable
/// when the metrics entry aggregates multiple labels. Not applicable when the
/// entry is for a single label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfusionMatrix {
    #[prost(message, repeated, tag="1")]
    pub row: ::prost::alloc::vec::Vec<confusion_matrix::Row>,
}
/// Nested message and enum types in `ConfusionMatrix`.
pub mod confusion_matrix {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfusionMatrixEntry {
        /// The annotation spec of a predicted label.
        #[prost(message, optional, tag="1")]
        pub annotation_spec: ::core::option::Option<super::AnnotationSpec>,
        /// Number of items predicted to have this label. (The ground truth label for
        /// these items is the `Row.annotationSpec` of this entry's parent.)
        #[prost(int32, tag="2")]
        pub item_count: i32,
    }
    /// A row in the confusion matrix. Each entry in this row has the same
    /// ground truth label.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Row {
        /// The annotation spec of the ground truth label for this row.
        #[prost(message, optional, tag="1")]
        pub annotation_spec: ::core::option::Option<super::AnnotationSpec>,
        /// A list of the confusion matrix entries. One entry for each possible
        /// predicted label.
        #[prost(message, repeated, tag="2")]
        pub entries: ::prost::alloc::vec::Vec<ConfusionMatrixEntry>,
    }
}
/// Defines an evaluation job that runs periodically to generate
/// \[Evaluations][google.cloud.datalabeling.v1beta1.Evaluation\]. [Creating an evaluation
/// job](/ml-engine/docs/continuous-evaluation/create-job) is the starting point
/// for using continuous evaluation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationJob {
    /// Output only. After you create a job, Data Labeling Service assigns a name
    /// to the job with the following format:
    ///
    /// "projects/<var>{project_id}</var>/evaluationJobs/<var>{evaluation_job_id}</var>"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Description of the job. The description can be up to 25,000
    /// characters long.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Describes the current state of the job.
    #[prost(enumeration="evaluation_job::State", tag="3")]
    pub state: i32,
    /// Required. Describes the interval at which the job runs. This interval must
    /// be at least 1 day, and it is rounded to the nearest day. For example, if
    /// you specify a 50-hour interval, the job runs every 2 days.
    ///
    /// You can provide the schedule in
    /// [crontab format](/scheduler/docs/configuring/cron-job-schedules) or in an
    /// [English-like
    /// format](/appengine/docs/standard/python/config/cronref#schedule_format).
    ///
    /// Regardless of what you specify, the job will run at 10:00 AM UTC. Only the
    /// interval from this schedule is used, not the specific time of day.
    #[prost(string, tag="4")]
    pub schedule: ::prost::alloc::string::String,
    /// Required. The [AI Platform Prediction model
    /// version](/ml-engine/docs/prediction-overview) to be evaluated. Prediction
    /// input and output is sampled from this model version. When creating an
    /// evaluation job, specify the model version in the following format:
    ///
    /// "projects/<var>{project_id}</var>/models/<var>{model_name}</var>/versions/<var>{version_name}</var>"
    ///
    /// There can only be one evaluation job per model version.
    #[prost(string, tag="5")]
    pub model_version: ::prost::alloc::string::String,
    /// Required. Configuration details for the evaluation job.
    #[prost(message, optional, tag="6")]
    pub evaluation_job_config: ::core::option::Option<EvaluationJobConfig>,
    /// Required. Name of the \[AnnotationSpecSet][google.cloud.datalabeling.v1beta1.AnnotationSpecSet\] describing all the
    /// labels that your machine learning model outputs. You must create this
    /// resource before you create an evaluation job and provide its name in the
    /// following format:
    ///
    /// "projects/<var>{project_id}</var>/annotationSpecSets/<var>{annotation_spec_set_id}</var>"
    #[prost(string, tag="7")]
    pub annotation_spec_set: ::prost::alloc::string::String,
    /// Required. Whether you want Data Labeling Service to provide ground truth
    /// labels for prediction input. If you want the service to assign human
    /// labelers to annotate your data, set this to `true`. If you want to provide
    /// your own ground truth labels in the evaluation job's BigQuery table, set
    /// this to `false`.
    #[prost(bool, tag="8")]
    pub label_missing_ground_truth: bool,
    /// Output only. Every time the evaluation job runs and an error occurs, the
    /// failed attempt is appended to this array.
    #[prost(message, repeated, tag="9")]
    pub attempts: ::prost::alloc::vec::Vec<Attempt>,
    /// Output only. Timestamp of when this evaluation job was created.
    #[prost(message, optional, tag="10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `EvaluationJob`.
pub mod evaluation_job {
    /// State of the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Unspecified = 0,
        /// The job is scheduled to run at the [configured interval]\[google.cloud.datalabeling.v1beta1.EvaluationJob.schedule\]. You
        /// can \[pause][google.cloud.datalabeling.v1beta1.DataLabelingService.PauseEvaluationJob\] or
        /// \[delete][google.cloud.datalabeling.v1beta1.DataLabelingService.DeleteEvaluationJob\] the job.
        ///
        /// When the job is in this state, it samples prediction input and output
        /// from your model version into your BigQuery table as predictions occur.
        Scheduled = 1,
        /// The job is currently running. When the job runs, Data Labeling Service
        /// does several things:
        ///
        /// 1. If you have configured your job to use Data Labeling Service for
        ///    ground truth labeling, the service creates a
        ///    \[Dataset][google.cloud.datalabeling.v1beta1.Dataset\] and a labeling task for all data sampled
        ///    since the last time the job ran. Human labelers provide ground truth
        ///    labels for your data. Human labeling may take hours, or even days,
        ///    depending on how much data has been sampled. The job remains in the
        ///    `RUNNING` state during this time, and it can even be running multiple
        ///    times in parallel if it gets triggered again (for example 24 hours
        ///    later) before the earlier run has completed. When human labelers have
        ///    finished labeling the data, the next step occurs.
        ///    <br><br>
        ///    If you have configured your job to provide your own ground truth
        ///    labels, Data Labeling Service still creates a \[Dataset][google.cloud.datalabeling.v1beta1.Dataset\] for newly
        ///    sampled data, but it expects that you have already added ground truth
        ///    labels to the BigQuery table by this time. The next step occurs
        ///    immediately.
        ///
        /// 2. Data Labeling Service creates an \[Evaluation][google.cloud.datalabeling.v1beta1.Evaluation\] by comparing your
        ///    model version's predictions with the ground truth labels.
        ///
        /// If the job remains in this state for a long time, it continues to sample
        /// prediction data into your BigQuery table and will run again at the next
        /// interval, even if it causes the job to run multiple times in parallel.
        Running = 2,
        /// The job is not sampling prediction input and output into your BigQuery
        /// table and it will not run according to its schedule. You can
        /// \[resume][google.cloud.datalabeling.v1beta1.DataLabelingService.ResumeEvaluationJob\] the job.
        Paused = 3,
        /// The job has this state right before it is deleted.
        Stopped = 4,
    }
}
/// Configures specific details of how a continuous evaluation job works. Provide
/// this configuration when you create an EvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationJobConfig {
    /// Rquired. Details for the sampled prediction input. Within this
    /// configuration, there are requirements for several fields:
    ///
    /// * `dataType` must be one of `IMAGE`, `TEXT`, or `GENERAL_DATA`.
    /// * `annotationType` must be one of `IMAGE_CLASSIFICATION_ANNOTATION`,
    ///   `TEXT_CLASSIFICATION_ANNOTATION`, `GENERAL_CLASSIFICATION_ANNOTATION`,
    ///   or `IMAGE_BOUNDING_BOX_ANNOTATION` (image object detection).
    /// * If your machine learning model performs classification, you must specify
    ///   `classificationMetadata.isMultiLabel`.
    /// * You must specify `bigquerySource` (not `gcsSource`).
    #[prost(message, optional, tag="1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Required. Details for calculating evaluation metrics and creating
    /// \[Evaulations][google.cloud.datalabeling.v1beta1.Evaluation\]. If your model version performs image object
    /// detection, you must specify the `boundingBoxEvaluationOptions` field within
    /// this configuration. Otherwise, provide an empty object for this
    /// configuration.
    #[prost(message, optional, tag="2")]
    pub evaluation_config: ::core::option::Option<EvaluationConfig>,
    /// Optional. Details for human annotation of your data. If you set
    /// \[labelMissingGroundTruth][google.cloud.datalabeling.v1beta1.EvaluationJob.label_missing_ground_truth\] to
    /// `true` for this evaluation job, then you must specify this field. If you
    /// plan to provide your own ground truth labels, then omit this field.
    ///
    /// Note that you must create an \[Instruction][google.cloud.datalabeling.v1beta1.Instruction\] resource before you can
    /// specify this field. Provide the name of the instruction resource in the
    /// `instruction` field within this configuration.
    #[prost(message, optional, tag="3")]
    pub human_annotation_config: ::core::option::Option<HumanAnnotationConfig>,
    /// Required. Prediction keys that tell Data Labeling Service where to find the
    /// data for evaluation in your BigQuery table. When the service samples
    /// prediction input and output from your model version and saves it to
    /// BigQuery, the data gets stored as JSON strings in the BigQuery table. These
    /// keys tell Data Labeling Service how to parse the JSON.
    ///
    /// You can provide the following entries in this field:
    ///
    /// * `data_json_key`: the data key for prediction input. You must provide
    ///   either this key or `reference_json_key`.
    /// * `reference_json_key`: the data reference key for prediction input. You
    ///   must provide either this key or `data_json_key`.
    /// * `label_json_key`: the label key for prediction output. Required.
    /// * `label_score_json_key`: the score key for prediction output. Required.
    /// * `bounding_box_json_key`: the bounding box key for prediction output.
    ///   Required if your model version perform image object detection.
    ///
    /// Learn [how to configure prediction
    /// keys](/ml-engine/docs/continuous-evaluation/create-job#prediction-keys).
    #[prost(btree_map="string, string", tag="9")]
    pub bigquery_import_keys: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The maximum number of predictions to sample and save to BigQuery
    /// during each [evaluation interval]\[google.cloud.datalabeling.v1beta1.EvaluationJob.schedule\]. This limit
    /// overrides `example_sample_percentage`: even if the service has not sampled
    /// enough predictions to fulfill `example_sample_perecentage` during an
    /// interval, it stops sampling predictions when it meets this limit.
    #[prost(int32, tag="10")]
    pub example_count: i32,
    /// Required. Fraction of predictions to sample and save to BigQuery during
    /// each [evaluation interval]\[google.cloud.datalabeling.v1beta1.EvaluationJob.schedule\]. For example, 0.1 means
    /// 10% of predictions served by your model version get saved to BigQuery.
    #[prost(double, tag="11")]
    pub example_sample_percentage: f64,
    /// Optional. Configuration details for evaluation job alerts. Specify this
    /// field if you want to receive email alerts if the evaluation job finds that
    /// your predictions have low mean average precision during a run.
    #[prost(message, optional, tag="13")]
    pub evaluation_job_alert_config: ::core::option::Option<EvaluationJobAlertConfig>,
    /// Required. Details for how you want human reviewers to provide ground truth
    /// labels.
    #[prost(oneof="evaluation_job_config::HumanAnnotationRequestConfig", tags="4, 5, 8")]
    pub human_annotation_request_config: ::core::option::Option<evaluation_job_config::HumanAnnotationRequestConfig>,
}
/// Nested message and enum types in `EvaluationJobConfig`.
pub mod evaluation_job_config {
    /// Required. Details for how you want human reviewers to provide ground truth
    /// labels.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HumanAnnotationRequestConfig {
        /// Specify this field if your model version performs image classification or
        /// general classification.
        ///
        /// `annotationSpecSet` in this configuration must match
        /// \[EvaluationJob.annotationSpecSet][google.cloud.datalabeling.v1beta1.EvaluationJob.annotation_spec_set\].
        /// `allowMultiLabel` in this configuration must match
        /// `classificationMetadata.isMultiLabel` in \[input_config][google.cloud.datalabeling.v1beta1.EvaluationJobConfig.input_config\].
        #[prost(message, tag="4")]
        ImageClassificationConfig(super::ImageClassificationConfig),
        /// Specify this field if your model version performs image object detection
        /// (bounding box detection).
        ///
        /// `annotationSpecSet` in this configuration must match
        /// \[EvaluationJob.annotationSpecSet][google.cloud.datalabeling.v1beta1.EvaluationJob.annotation_spec_set\].
        #[prost(message, tag="5")]
        BoundingPolyConfig(super::BoundingPolyConfig),
        /// Specify this field if your model version performs text classification.
        ///
        /// `annotationSpecSet` in this configuration must match
        /// \[EvaluationJob.annotationSpecSet][google.cloud.datalabeling.v1beta1.EvaluationJob.annotation_spec_set\].
        /// `allowMultiLabel` in this configuration must match
        /// `classificationMetadata.isMultiLabel` in \[input_config][google.cloud.datalabeling.v1beta1.EvaluationJobConfig.input_config\].
        #[prost(message, tag="8")]
        TextClassificationConfig(super::TextClassificationConfig),
    }
}
/// Provides details for how an evaluation job sends email alerts based on the
/// results of a run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluationJobAlertConfig {
    /// Required. An email address to send alerts to.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// Required. A number between 0 and 1 that describes a minimum mean average
    /// precision threshold. When the evaluation job runs, if it calculates that
    /// your model version's predictions from the recent interval have
    /// \[meanAveragePrecision][google.cloud.datalabeling.v1beta1.PrCurve.mean_average_precision\] below this
    /// threshold, then it sends an alert to your specified email.
    #[prost(double, tag="2")]
    pub min_acceptable_mean_average_precision: f64,
}
/// Records a failed evaluation job run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attempt {
    #[prost(message, optional, tag="1")]
    pub attempt_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Details of errors that occurred.
    #[prost(message, repeated, tag="2")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Instruction of how to perform the labeling task for human operators.
/// Currently only PDF instruction is supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    /// Output only. Instruction resource name, format:
    /// projects/{project_id}/instructions/{instruction_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the instruction. Maximum of 64 characters.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. User-provided description of the instruction.
    /// The description can be up to 10000 characters long.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Creation time of instruction.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last update time of instruction.
    #[prost(message, optional, tag="5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The data type of this instruction.
    #[prost(enumeration="DataType", tag="6")]
    pub data_type: i32,
    /// Deprecated: this instruction format is not supported any more.
    /// Instruction from a CSV file, such as for classification task.
    /// The CSV file should have exact two columns, in the following format:
    ///
    /// * The first column is labeled data, such as an image reference, text.
    /// * The second column is comma separated labels associated with data.
    #[deprecated]
    #[prost(message, optional, tag="7")]
    pub csv_instruction: ::core::option::Option<CsvInstruction>,
    /// Instruction from a PDF document. The PDF should be in a Cloud Storage
    /// bucket.
    #[prost(message, optional, tag="9")]
    pub pdf_instruction: ::core::option::Option<PdfInstruction>,
    /// Output only. The names of any related resources that are blocking changes
    /// to the instruction.
    #[prost(string, repeated, tag="10")]
    pub blocking_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Deprecated: this instruction format is not supported any more.
/// Instruction from a CSV file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvInstruction {
    /// CSV file for the instruction. Only gcs path is allowed.
    #[prost(string, tag="1")]
    pub gcs_file_uri: ::prost::alloc::string::String,
}
/// Instruction from a PDF file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PdfInstruction {
    /// PDF file for the instruction. Only gcs path is allowed.
    #[prost(string, tag="1")]
    pub gcs_file_uri: ::prost::alloc::string::String,
}
/// Request message for CreateDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Required. Dataset resource parent, format:
    /// projects/{project_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The dataset to be created.
    #[prost(message, optional, tag="2")]
    pub dataset: ::core::option::Option<Dataset>,
}
/// Request message for GetDataSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// Required. Dataset resource name, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. Dataset resource parent, format:
    /// projects/{project_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filter on dataset is not supported at this moment.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by
    /// \[ListDatasetsResponse.next_page_token][google.cloud.datalabeling.v1beta1.ListDatasetsResponse.next_page_token\] of the previous
    /// \[DataLabelingService.ListDatasets\] call.
    /// Returns the first page if empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of listing datasets within a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsResponse {
    /// The list of datasets to return.
    #[prost(message, repeated, tag="1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    /// Required. Dataset resource name, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ImportData API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataRequest {
    /// Required. Dataset resource name, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Specify the input source of the data.
    #[prost(message, optional, tag="2")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Email of the user who started the import task and should be notified by
    /// email. If empty no notification will be sent.
    #[prost(string, tag="3")]
    pub user_email_address: ::prost::alloc::string::String,
}
/// Request message for ExportData API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataRequest {
    /// Required. Dataset resource name, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Annotated dataset resource name. DataItem in
    /// Dataset and their annotations in specified annotated dataset will be
    /// exported. It's in format of
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/
    /// {annotated_dataset_id}
    #[prost(string, tag="2")]
    pub annotated_dataset: ::prost::alloc::string::String,
    /// Optional. Filter is not supported at this moment.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Specify the output destination.
    #[prost(message, optional, tag="4")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Email of the user who started the export task and should be notified by
    /// email. If empty no notification will be sent.
    #[prost(string, tag="5")]
    pub user_email_address: ::prost::alloc::string::String,
}
/// Request message for GetDataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataItemRequest {
    /// Required. The name of the data item to get, format:
    /// projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListDataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataItemsRequest {
    /// Required. Name of the dataset to list data items, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filter is not supported at this moment.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by
    /// \[ListDataItemsResponse.next_page_token][google.cloud.datalabeling.v1beta1.ListDataItemsResponse.next_page_token\] of the previous
    /// \[DataLabelingService.ListDataItems\] call.
    /// Return first page if empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of listing data items in a dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataItemsResponse {
    /// The list of data items to return.
    #[prost(message, repeated, tag="1")]
    pub data_items: ::prost::alloc::vec::Vec<DataItem>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetAnnotatedDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotatedDatasetRequest {
    /// Required. Name of the annotated dataset to get, format:
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/
    /// {annotated_dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAnnotatedDatasets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotatedDatasetsRequest {
    /// Required. Name of the dataset to list annotated datasets, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filter is not supported at this moment.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by
    /// \[ListAnnotatedDatasetsResponse.next_page_token][google.cloud.datalabeling.v1beta1.ListAnnotatedDatasetsResponse.next_page_token\] of the previous
    /// \[DataLabelingService.ListAnnotatedDatasets\] call.
    /// Return first page if empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of listing annotated datasets for a dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotatedDatasetsResponse {
    /// The list of annotated datasets to return.
    #[prost(message, repeated, tag="1")]
    pub annotated_datasets: ::prost::alloc::vec::Vec<AnnotatedDataset>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteAnnotatedDataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotatedDatasetRequest {
    /// Required. Name of the annotated dataset to delete, format:
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/
    /// {annotated_dataset_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for starting an image labeling task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImageRequest {
    /// Required. Name of the dataset to request labeling task, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Basic human annotation config.
    #[prost(message, optional, tag="2")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
    /// Required. The type of image labeling task.
    #[prost(enumeration="label_image_request::Feature", tag="3")]
    pub feature: i32,
    /// Required. Config for labeling tasks. The type of request config must
    /// match the selected feature.
    #[prost(oneof="label_image_request::RequestConfig", tags="4, 5, 6, 7")]
    pub request_config: ::core::option::Option<label_image_request::RequestConfig>,
}
/// Nested message and enum types in `LabelImageRequest`.
pub mod label_image_request {
    /// Image labeling task feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Feature {
        Unspecified = 0,
        /// Label whole image with one or more of labels.
        Classification = 1,
        /// Label image with bounding boxes for labels.
        BoundingBox = 2,
        /// Label oriented bounding box. The box does not have to be parallel to
        /// horizontal line.
        OrientedBoundingBox = 6,
        /// Label images with bounding poly. A bounding poly is a plane figure that
        /// is bounded by a finite chain of straight line segments closing in a loop.
        BoundingPoly = 3,
        /// Label images with polyline. Polyline is formed by connected line segments
        /// which are not in closed form.
        Polyline = 4,
        /// Label images with segmentation. Segmentation is different from bounding
        /// poly since it is more fine-grained, pixel level annotation.
        Segmentation = 5,
    }
    /// Required. Config for labeling tasks. The type of request config must
    /// match the selected feature.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestConfig {
        /// Configuration for image classification task.
        /// One of image_classification_config, bounding_poly_config,
        /// polyline_config and segmentation_config are required.
        #[prost(message, tag="4")]
        ImageClassificationConfig(super::ImageClassificationConfig),
        /// Configuration for bounding box and bounding poly task.
        /// One of image_classification_config, bounding_poly_config,
        /// polyline_config and segmentation_config are required.
        #[prost(message, tag="5")]
        BoundingPolyConfig(super::BoundingPolyConfig),
        /// Configuration for polyline task.
        /// One of image_classification_config, bounding_poly_config,
        /// polyline_config and segmentation_config are required.
        #[prost(message, tag="6")]
        PolylineConfig(super::PolylineConfig),
        /// Configuration for segmentation task.
        /// One of image_classification_config, bounding_poly_config,
        /// polyline_config and segmentation_config are required.
        #[prost(message, tag="7")]
        SegmentationConfig(super::SegmentationConfig),
    }
}
/// Request message for LabelVideo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelVideoRequest {
    /// Required. Name of the dataset to request labeling task, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Basic human annotation config.
    #[prost(message, optional, tag="2")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
    /// Required. The type of video labeling task.
    #[prost(enumeration="label_video_request::Feature", tag="3")]
    pub feature: i32,
    /// Required. Config for labeling tasks. The type of request config must
    /// match the selected feature.
    #[prost(oneof="label_video_request::RequestConfig", tags="4, 5, 6, 7")]
    pub request_config: ::core::option::Option<label_video_request::RequestConfig>,
}
/// Nested message and enum types in `LabelVideoRequest`.
pub mod label_video_request {
    /// Video labeling task feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Feature {
        Unspecified = 0,
        /// Label whole video or video segment with one or more labels.
        Classification = 1,
        /// Label objects with bounding box on image frames extracted from the video.
        ObjectDetection = 2,
        /// Label and track objects in video.
        ObjectTracking = 3,
        /// Label the range of video for the specified events.
        Event = 4,
    }
    /// Required. Config for labeling tasks. The type of request config must
    /// match the selected feature.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestConfig {
        /// Configuration for video classification task.
        /// One of video_classification_config, object_detection_config,
        /// object_tracking_config and event_config is required.
        #[prost(message, tag="4")]
        VideoClassificationConfig(super::VideoClassificationConfig),
        /// Configuration for video object detection task.
        /// One of video_classification_config, object_detection_config,
        /// object_tracking_config and event_config is required.
        #[prost(message, tag="5")]
        ObjectDetectionConfig(super::ObjectDetectionConfig),
        /// Configuration for video object tracking task.
        /// One of video_classification_config, object_detection_config,
        /// object_tracking_config and event_config is required.
        #[prost(message, tag="6")]
        ObjectTrackingConfig(super::ObjectTrackingConfig),
        /// Configuration for video event task.
        /// One of video_classification_config, object_detection_config,
        /// object_tracking_config and event_config is required.
        #[prost(message, tag="7")]
        EventConfig(super::EventConfig),
    }
}
/// Request message for LabelText.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTextRequest {
    /// Required. Name of the data set to request labeling task, format:
    /// projects/{project_id}/datasets/{dataset_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Basic human annotation config.
    #[prost(message, optional, tag="2")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
    /// Required. The type of text labeling task.
    #[prost(enumeration="label_text_request::Feature", tag="6")]
    pub feature: i32,
    /// Required. Config for labeling tasks. The type of request config must
    /// match the selected feature.
    #[prost(oneof="label_text_request::RequestConfig", tags="4, 5")]
    pub request_config: ::core::option::Option<label_text_request::RequestConfig>,
}
/// Nested message and enum types in `LabelTextRequest`.
pub mod label_text_request {
    /// Text labeling task feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Feature {
        Unspecified = 0,
        /// Label text content to one of more labels.
        TextClassification = 1,
        /// Label entities and their span in text.
        TextEntityExtraction = 2,
    }
    /// Required. Config for labeling tasks. The type of request config must
    /// match the selected feature.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestConfig {
        /// Configuration for text classification task.
        /// One of text_classification_config and text_entity_extraction_config
        /// is required.
        #[prost(message, tag="4")]
        TextClassificationConfig(super::TextClassificationConfig),
        /// Configuration for entity extraction task.
        /// One of text_classification_config and text_entity_extraction_config
        /// is required.
        #[prost(message, tag="5")]
        TextEntityExtractionConfig(super::TextEntityExtractionConfig),
    }
}
/// Request message for GetExample
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExampleRequest {
    /// Required. Name of example, format:
    /// projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/
    /// {annotated_dataset_id}/examples/{example_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An expression for filtering Examples. Filter by
    /// annotation_spec.display_name is supported. Format
    /// "annotation_spec.display_name = {display_name}"
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
}
/// Request message for ListExamples.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExamplesRequest {
    /// Required. Example resource parent.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. An expression for filtering Examples. For annotated datasets that
    /// have annotation spec set, filter by
    /// annotation_spec.display_name is supported. Format
    /// "annotation_spec.display_name = {display_name}"
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by
    /// \[ListExamplesResponse.next_page_token][google.cloud.datalabeling.v1beta1.ListExamplesResponse.next_page_token\] of the previous
    /// \[DataLabelingService.ListExamples\] call.
    /// Return first page if empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of listing Examples in and annotated dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExamplesResponse {
    /// The list of examples to return.
    #[prost(message, repeated, tag="1")]
    pub examples: ::prost::alloc::vec::Vec<Example>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CreateAnnotationSpecSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAnnotationSpecSetRequest {
    /// Required. AnnotationSpecSet resource parent, format:
    /// projects/{project_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Annotation spec set to create. Annotation specs must be included.
    /// Only one annotation spec will be accepted for annotation specs with same
    /// display_name.
    #[prost(message, optional, tag="2")]
    pub annotation_spec_set: ::core::option::Option<AnnotationSpecSet>,
}
/// Request message for GetAnnotationSpecSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationSpecSetRequest {
    /// Required. AnnotationSpecSet resource name, format:
    /// projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListAnnotationSpecSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationSpecSetsRequest {
    /// Required. Parent of AnnotationSpecSet resource, format:
    /// projects/{project_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filter is not supported at this moment.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by
    /// \[ListAnnotationSpecSetsResponse.next_page_token][google.cloud.datalabeling.v1beta1.ListAnnotationSpecSetsResponse.next_page_token\] of the previous
    /// \[DataLabelingService.ListAnnotationSpecSets\] call.
    /// Return first page if empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of listing annotation spec set under a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationSpecSetsResponse {
    /// The list of annotation spec sets.
    #[prost(message, repeated, tag="1")]
    pub annotation_spec_sets: ::prost::alloc::vec::Vec<AnnotationSpecSet>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for DeleteAnnotationSpecSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAnnotationSpecSetRequest {
    /// Required. AnnotationSpec resource name, format:
    /// `projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateInstruction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstructionRequest {
    /// Required. Instruction resource parent, format:
    /// projects/{project_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Instruction of how to perform the labeling task.
    #[prost(message, optional, tag="2")]
    pub instruction: ::core::option::Option<Instruction>,
}
/// Request message for GetInstruction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstructionRequest {
    /// Required. Instruction resource name, format:
    /// projects/{project_id}/instructions/{instruction_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteInstruction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstructionRequest {
    /// Required. Instruction resource name, format:
    /// projects/{project_id}/instructions/{instruction_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListInstructions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstructionsRequest {
    /// Required. Instruction resource parent, format:
    /// projects/{project_id}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Filter is not supported at this moment.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by
    /// \[ListInstructionsResponse.next_page_token][google.cloud.datalabeling.v1beta1.ListInstructionsResponse.next_page_token\] of the previous
    /// \[DataLabelingService.ListInstructions\] call.
    /// Return first page if empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of listing instructions under a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstructionsResponse {
    /// The list of Instructions to return.
    #[prost(message, repeated, tag="1")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetEvaluation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEvaluationRequest {
    /// Required. Name of the evaluation. Format:
    ///
    /// "projects/<var>{project_id}</var>/datasets/<var>{dataset_id}</var>/evaluations/<var>{evaluation_id}</var>'
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for SearchEvaluation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEvaluationsRequest {
    /// Required. Evaluation search parent (project ID). Format:
    /// "projects/<var>{project_id}</var>"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. To search evaluations, you can filter by the following:
    ///
    /// * evaluation<span>_</span>job.evaluation_job_id (the last part of
    ///   \[EvaluationJob.name][google.cloud.datalabeling.v1beta1.EvaluationJob.name\])
    /// * evaluation<span>_</span>job.model_id (the <var>{model_name}</var> portion
    ///   of \[EvaluationJob.modelVersion][google.cloud.datalabeling.v1beta1.EvaluationJob.model_version\])
    /// * evaluation<span>_</span>job.evaluation_job_run_time_start (Minimum
    ///   threshold for the
    ///   \[evaluationJobRunTime][google.cloud.datalabeling.v1beta1.Evaluation.evaluation_job_run_time\] that created
    ///   the evaluation)
    /// * evaluation<span>_</span>job.evaluation_job_run_time_end (Maximum
    ///   threshold for the
    ///   \[evaluationJobRunTime][google.cloud.datalabeling.v1beta1.Evaluation.evaluation_job_run_time\] that created
    ///   the evaluation)
    /// * evaluation<span>_</span>job.job_state (\[EvaluationJob.state][google.cloud.datalabeling.v1beta1.EvaluationJob.state\])
    /// * annotation<span>_</span>spec.display_name (the Evaluation contains a
    ///   metric for the annotation spec with this
    ///   \[displayName][google.cloud.datalabeling.v1beta1.AnnotationSpec.display_name\])
    ///
    /// To filter by multiple critiera, use the `AND` operator or the `OR`
    /// operator. The following examples shows a string that filters by several
    /// critiera:
    ///
    /// "evaluation<span>_</span>job.evaluation_job_id =
    /// <var>{evaluation_job_id}</var> AND evaluation<span>_</span>job.model_id =
    /// <var>{model_name}</var> AND
    /// evaluation<span>_</span>job.evaluation_job_run_time_start =
    /// <var>{timestamp_1}</var> AND
    /// evaluation<span>_</span>job.evaluation_job_run_time_end =
    /// <var>{timestamp_2}</var> AND annotation<span>_</span>spec.display_name =
    /// <var>{display_name}</var>"
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by the
    /// \[nextPageToken][google.cloud.datalabeling.v1beta1.SearchEvaluationsResponse.next_page_token\] of the response
    /// to a previous search request.
    ///
    /// If you don't specify this field, the API call requests the first page of
    /// the search.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of searching evaluations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEvaluationsResponse {
    /// The list of evaluations matching the search.
    #[prost(message, repeated, tag="1")]
    pub evaluations: ::prost::alloc::vec::Vec<Evaluation>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message of SearchExampleComparisons.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchExampleComparisonsRequest {
    /// Required. Name of the \[Evaluation][google.cloud.datalabeling.v1beta1.Evaluation\] resource to search for example
    /// comparisons from. Format:
    ///
    /// "projects/<var>{project_id}</var>/datasets/<var>{dataset_id}</var>/evaluations/<var>{evaluation_id}</var>"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by the
    /// \[nextPageToken][SearchExampleComparisons.next_page_token\] of the response
    /// to a previous search rquest.
    ///
    /// If you don't specify this field, the API call requests the first page of
    /// the search.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results of searching example comparisons.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchExampleComparisonsResponse {
    /// A list of example comparisons matching the search criteria.
    #[prost(message, repeated, tag="1")]
    pub example_comparisons: ::prost::alloc::vec::Vec<search_example_comparisons_response::ExampleComparison>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SearchExampleComparisonsResponse`.
pub mod search_example_comparisons_response {
    /// Example comparisons comparing ground truth output and predictions for a
    /// specific input.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExampleComparison {
        /// The ground truth output for the input.
        #[prost(message, optional, tag="1")]
        pub ground_truth_example: ::core::option::Option<super::Example>,
        /// Predictions by the model for the input.
        #[prost(message, repeated, tag="2")]
        pub model_created_examples: ::prost::alloc::vec::Vec<super::Example>,
    }
}
/// Request message for CreateEvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEvaluationJobRequest {
    /// Required. Evaluation job resource parent. Format:
    /// "projects/<var>{project_id}</var>"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The evaluation job to create.
    #[prost(message, optional, tag="2")]
    pub job: ::core::option::Option<EvaluationJob>,
}
/// Request message for UpdateEvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEvaluationJobRequest {
    /// Required. Evaluation job that is going to be updated.
    #[prost(message, optional, tag="1")]
    pub evaluation_job: ::core::option::Option<EvaluationJob>,
    /// Optional. Mask for which fields to update. You can only provide the
    /// following fields:
    ///
    /// * `evaluationJobConfig.humanAnnotationConfig.instruction`
    /// * `evaluationJobConfig.exampleCount`
    /// * `evaluationJobConfig.exampleSamplePercentage`
    ///
    /// You can provide more than one of these fields by separating them with
    /// commas.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GetEvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEvaluationJobRequest {
    /// Required. Name of the evaluation job. Format:
    ///
    /// "projects/<var>{project_id}</var>/evaluationJobs/<var>{evaluation_job_id}</var>"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for PauseEvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseEvaluationJobRequest {
    /// Required. Name of the evaluation job that is going to be paused. Format:
    ///
    /// "projects/<var>{project_id}</var>/evaluationJobs/<var>{evaluation_job_id}</var>"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message ResumeEvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeEvaluationJobRequest {
    /// Required. Name of the evaluation job that is going to be resumed. Format:
    ///
    /// "projects/<var>{project_id}</var>/evaluationJobs/<var>{evaluation_job_id}</var>"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message DeleteEvaluationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEvaluationJobRequest {
    /// Required. Name of the evaluation job that is going to be deleted. Format:
    ///
    /// "projects/<var>{project_id}</var>/evaluationJobs/<var>{evaluation_job_id}</var>"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListEvaluationJobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEvaluationJobsRequest {
    /// Required. Evaluation job resource parent. Format:
    /// "projects/<var>{project_id}</var>"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. You can filter the jobs to list by model_id (also known as
    /// model_name, as described in
    /// \[EvaluationJob.modelVersion][google.cloud.datalabeling.v1beta1.EvaluationJob.model_version\]) or by
    /// evaluation job state (as described in \[EvaluationJob.state][google.cloud.datalabeling.v1beta1.EvaluationJob.state\]). To filter
    /// by both criteria, use the `AND` operator or the `OR` operator. For example,
    /// you can use the following string for your filter:
    /// "evaluation<span>_</span>job.model_id = <var>{model_name}</var> AND
    /// evaluation<span>_</span>job.state = <var>{evaluation_job_state}</var>"
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer results than
    /// requested. Default value is 100.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results for the server to return.
    /// Typically obtained by the
    /// \[nextPageToken][google.cloud.datalabeling.v1beta1.ListEvaluationJobsResponse.next_page_token\] in the response
    /// to the previous request. The request returns the first page if this is
    /// empty.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Results for listing evaluation jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEvaluationJobsResponse {
    /// The list of evaluation jobs to return.
    #[prost(message, repeated, tag="1")]
    pub evaluation_jobs: ::prost::alloc::vec::Vec<EvaluationJob>,
    /// A token to retrieve next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod data_labeling_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Service for the AI Platform Data Labeling API.
    #[derive(Debug, Clone)]
    pub struct DataLabelingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataLabelingServiceClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DataLabelingServiceClient<InterceptedService<T, F>>
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
            DataLabelingServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Creates dataset. If success return a Dataset resource.
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets dataset by resource name.
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists datasets under a project. Pagination is supported.
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> Result<tonic::Response<super::ListDatasetsResponse>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a dataset by resource name.
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Imports data into dataset based on source locations defined in request.
        /// It can be called multiple times for the same dataset. Each dataset can
        /// only have one long running operation running on it. For example, no
        /// labeling task (also long running operation) can be started while
        /// importing is still ongoing. Vice versa.
        pub async fn import_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDataRequest>,
        ) -> Result<
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ImportData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Exports data and annotations from dataset.
        pub async fn export_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDataRequest>,
        ) -> Result<
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ExportData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a data item in a dataset by resource name. This API can be
        /// called after data are imported into dataset.
        pub async fn get_data_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataItemRequest>,
        ) -> Result<tonic::Response<super::DataItem>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetDataItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists data items in a dataset. This API can be called after data
        /// are imported into dataset. Pagination is supported.
        pub async fn list_data_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataItemsRequest>,
        ) -> Result<tonic::Response<super::ListDataItemsResponse>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListDataItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an annotated dataset by resource name.
        pub async fn get_annotated_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotatedDatasetRequest>,
        ) -> Result<tonic::Response<super::AnnotatedDataset>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetAnnotatedDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists annotated datasets for a dataset. Pagination is supported.
        pub async fn list_annotated_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnnotatedDatasetsRequest>,
        ) -> Result<
            tonic::Response<super::ListAnnotatedDatasetsResponse>,
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListAnnotatedDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an annotated dataset by resource name.
        pub async fn delete_annotated_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnnotatedDatasetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/DeleteAnnotatedDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a labeling task for image. The type of image labeling task is
        /// configured by feature in the request.
        pub async fn label_image(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelImageRequest>,
        ) -> Result<
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/LabelImage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a labeling task for video. The type of video labeling task is
        /// configured by feature in the request.
        pub async fn label_video(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelVideoRequest>,
        ) -> Result<
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/LabelVideo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts a labeling task for text. The type of text labeling task is
        /// configured by feature in the request.
        pub async fn label_text(
            &mut self,
            request: impl tonic::IntoRequest<super::LabelTextRequest>,
        ) -> Result<
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/LabelText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an example by resource name, including both data and annotation.
        pub async fn get_example(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExampleRequest>,
        ) -> Result<tonic::Response<super::Example>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetExample",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists examples in an annotated dataset. Pagination is supported.
        pub async fn list_examples(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExamplesRequest>,
        ) -> Result<tonic::Response<super::ListExamplesResponse>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListExamples",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an annotation spec set by providing a set of labels.
        pub async fn create_annotation_spec_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAnnotationSpecSetRequest>,
        ) -> Result<tonic::Response<super::AnnotationSpecSet>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/CreateAnnotationSpecSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an annotation spec set by resource name.
        pub async fn get_annotation_spec_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationSpecSetRequest>,
        ) -> Result<tonic::Response<super::AnnotationSpecSet>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetAnnotationSpecSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists annotation spec sets for a project. Pagination is supported.
        pub async fn list_annotation_spec_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnnotationSpecSetsRequest>,
        ) -> Result<
            tonic::Response<super::ListAnnotationSpecSetsResponse>,
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListAnnotationSpecSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an annotation spec set by resource name.
        pub async fn delete_annotation_spec_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAnnotationSpecSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/DeleteAnnotationSpecSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an instruction for how data should be labeled.
        pub async fn create_instruction(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstructionRequest>,
        ) -> Result<
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/CreateInstruction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an instruction by resource name.
        pub async fn get_instruction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstructionRequest>,
        ) -> Result<tonic::Response<super::Instruction>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetInstruction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists instructions for a project. Pagination is supported.
        pub async fn list_instructions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstructionsRequest>,
        ) -> Result<tonic::Response<super::ListInstructionsResponse>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListInstructions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes an instruction object by resource name.
        pub async fn delete_instruction(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstructionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/DeleteInstruction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an evaluation by resource name (to search, use
        /// [projects.evaluations.search][google.cloud.datalabeling.v1beta1.DataLabelingService.SearchEvaluations]).
        pub async fn get_evaluation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEvaluationRequest>,
        ) -> Result<tonic::Response<super::Evaluation>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetEvaluation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches [evaluations][google.cloud.datalabeling.v1beta1.Evaluation] within a project.
        pub async fn search_evaluations(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchEvaluationsRequest>,
        ) -> Result<tonic::Response<super::SearchEvaluationsResponse>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/SearchEvaluations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches example comparisons from an evaluation. The return format is a
        /// list of example comparisons that show ground truth and prediction(s) for
        /// a single input. Search by providing an evaluation ID.
        pub async fn search_example_comparisons(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchExampleComparisonsRequest>,
        ) -> Result<
            tonic::Response<super::SearchExampleComparisonsResponse>,
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/SearchExampleComparisons",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an evaluation job.
        pub async fn create_evaluation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEvaluationJobRequest>,
        ) -> Result<tonic::Response<super::EvaluationJob>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/CreateEvaluationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an evaluation job. You can only update certain fields of the job's
        /// [EvaluationJobConfig][google.cloud.datalabeling.v1beta1.EvaluationJobConfig]: `humanAnnotationConfig.instruction`,
        /// `exampleCount`, and `exampleSamplePercentage`.
        ///
        /// If you want to change any other aspect of the evaluation job, you must
        /// delete the job and create a new one.
        pub async fn update_evaluation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEvaluationJobRequest>,
        ) -> Result<tonic::Response<super::EvaluationJob>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/UpdateEvaluationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets an evaluation job by resource name.
        pub async fn get_evaluation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEvaluationJobRequest>,
        ) -> Result<tonic::Response<super::EvaluationJob>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/GetEvaluationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pauses an evaluation job. Pausing an evaluation job that is already in a
        /// `PAUSED` state is a no-op.
        pub async fn pause_evaluation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseEvaluationJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/PauseEvaluationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resumes a paused evaluation job. A deleted evaluation job can't be resumed.
        /// Resuming a running or scheduled evaluation job is a no-op.
        pub async fn resume_evaluation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeEvaluationJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ResumeEvaluationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Stops and deletes an evaluation job.
        pub async fn delete_evaluation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEvaluationJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/DeleteEvaluationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all evaluation jobs within a project with possible filters.
        /// Pagination is supported.
        pub async fn list_evaluation_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEvaluationJobsRequest>,
        ) -> Result<tonic::Response<super::ListEvaluationJobsResponse>, tonic::Status> {
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
                "/google.cloud.datalabeling.v1beta1.DataLabelingService/ListEvaluationJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Response used for ImportData longrunning operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataOperationResponse {
    /// Ouptut only. The name of imported dataset.
    #[prost(string, tag="1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. Total number of examples requested to import
    #[prost(int32, tag="2")]
    pub total_count: i32,
    /// Output only. Number of examples imported successfully.
    #[prost(int32, tag="3")]
    pub import_count: i32,
}
/// Response used for ExportDataset longrunning operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataOperationResponse {
    /// Ouptut only. The name of dataset.
    /// "projects/*/datasets/*"
    #[prost(string, tag="1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. Total number of examples requested to export
    #[prost(int32, tag="2")]
    pub total_count: i32,
    /// Output only. Number of examples exported successfully.
    #[prost(int32, tag="3")]
    pub export_count: i32,
    /// Output only. Statistic infos of labels in the exported dataset.
    #[prost(message, optional, tag="4")]
    pub label_stats: ::core::option::Option<LabelStats>,
    /// Output only. output_config in the ExportData request.
    #[prost(message, optional, tag="5")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Metadata of an ImportData operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataOperationMetadata {
    /// Output only. The name of imported dataset.
    /// "projects/*/datasets/*"
    #[prost(string, tag="1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. Partial failures encountered.
    /// E.g. single files that couldn't be read.
    /// Status details field will contain standard GCP error details.
    #[prost(message, repeated, tag="2")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Output only. Timestamp when import dataset request was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata of an ExportData operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataOperationMetadata {
    /// Output only. The name of dataset to be exported.
    /// "projects/*/datasets/*"
    #[prost(string, tag="1")]
    pub dataset: ::prost::alloc::string::String,
    /// Output only. Partial failures encountered.
    /// E.g. single files that couldn't be read.
    /// Status details field will contain standard GCP error details.
    #[prost(message, repeated, tag="2")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Output only. Timestamp when export dataset request was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata of a labeling operation, such as LabelImage or LabelVideo.
/// Next tag: 20
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelOperationMetadata {
    /// Output only. Progress of label operation. Range: [0, 100].
    #[prost(int32, tag="1")]
    pub progress_percent: i32,
    /// Output only. Partial failures encountered.
    /// E.g. single files that couldn't be read.
    /// Status details field will contain standard GCP error details.
    #[prost(message, repeated, tag="2")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Output only. Timestamp when labeling request was created.
    #[prost(message, optional, tag="16")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Ouptut only. Details of specific label operation.
    #[prost(oneof="label_operation_metadata::Details", tags="3, 4, 11, 14, 12, 15, 5, 6, 7, 8, 9, 13")]
    pub details: ::core::option::Option<label_operation_metadata::Details>,
}
/// Nested message and enum types in `LabelOperationMetadata`.
pub mod label_operation_metadata {
    /// Ouptut only. Details of specific label operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Details of label image classification operation.
        #[prost(message, tag="3")]
        ImageClassificationDetails(super::LabelImageClassificationOperationMetadata),
        /// Details of label image bounding box operation.
        #[prost(message, tag="4")]
        ImageBoundingBoxDetails(super::LabelImageBoundingBoxOperationMetadata),
        /// Details of label image bounding poly operation.
        #[prost(message, tag="11")]
        ImageBoundingPolyDetails(super::LabelImageBoundingPolyOperationMetadata),
        /// Details of label image oriented bounding box operation.
        #[prost(message, tag="14")]
        ImageOrientedBoundingBoxDetails(super::LabelImageOrientedBoundingBoxOperationMetadata),
        /// Details of label image polyline operation.
        #[prost(message, tag="12")]
        ImagePolylineDetails(super::LabelImagePolylineOperationMetadata),
        /// Details of label image segmentation operation.
        #[prost(message, tag="15")]
        ImageSegmentationDetails(super::LabelImageSegmentationOperationMetadata),
        /// Details of label video classification operation.
        #[prost(message, tag="5")]
        VideoClassificationDetails(super::LabelVideoClassificationOperationMetadata),
        /// Details of label video object detection operation.
        #[prost(message, tag="6")]
        VideoObjectDetectionDetails(super::LabelVideoObjectDetectionOperationMetadata),
        /// Details of label video object tracking operation.
        #[prost(message, tag="7")]
        VideoObjectTrackingDetails(super::LabelVideoObjectTrackingOperationMetadata),
        /// Details of label video event operation.
        #[prost(message, tag="8")]
        VideoEventDetails(super::LabelVideoEventOperationMetadata),
        /// Details of label text classification operation.
        #[prost(message, tag="9")]
        TextClassificationDetails(super::LabelTextClassificationOperationMetadata),
        /// Details of label text entity extraction operation.
        #[prost(message, tag="13")]
        TextEntityExtractionDetails(super::LabelTextEntityExtractionOperationMetadata),
    }
}
/// Metadata of a LabelImageClassification operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImageClassificationOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelImageBoundingBox operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImageBoundingBoxOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelImageOrientedBoundingBox operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImageOrientedBoundingBoxOperationMetadata {
    /// Basic human annotation config.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of LabelImageBoundingPoly operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImageBoundingPolyOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of LabelImagePolyline operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImagePolylineOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelImageSegmentation operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelImageSegmentationOperationMetadata {
    /// Basic human annotation config.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelVideoClassification operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelVideoClassificationOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelVideoObjectDetection operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelVideoObjectDetectionOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelVideoObjectTracking operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelVideoObjectTrackingOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelVideoEvent operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelVideoEventOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelTextClassification operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTextClassificationOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Details of a LabelTextEntityExtraction operation metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelTextEntityExtractionOperationMetadata {
    /// Basic human annotation config used in labeling request.
    #[prost(message, optional, tag="1")]
    pub basic_config: ::core::option::Option<HumanAnnotationConfig>,
}
/// Metadata of a CreateInstruction operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstructionMetadata {
    /// The name of the created Instruction.
    /// projects/{project_id}/instructions/{instruction_id}
    #[prost(string, tag="1")]
    pub instruction: ::prost::alloc::string::String,
    /// Partial failures encountered.
    /// E.g. single files that couldn't be read.
    /// Status details field will contain standard GCP error details.
    #[prost(message, repeated, tag="2")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Timestamp when create instruction request was created.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
