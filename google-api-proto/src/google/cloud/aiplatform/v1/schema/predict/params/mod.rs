/// Prediction model parameters for Video Object Tracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The model only returns up to that many top, by confidence score,
    /// predictions per frame of the video. If this number is very high, the
    /// Model may return fewer predictions per frame. Default value is 50.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
    /// Only bounding boxes with shortest edge at least that long as a relative
    /// value of video frame size are returned. Default value is 0.0.
    #[prost(float, tag = "3")]
    pub min_bounding_box_size: f32,
}
/// Prediction model parameters for Image Object Detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The Model only returns up to that many top, by confidence score,
    /// predictions per instance. Note that number of returned predictions is also
    /// limited by metadata's predictionsLimit. Default value is 10.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
/// Prediction model parameters for Image Segmentation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionParams {
    /// When the model predicts category of pixels of the image, it will only
    /// provide predictions for pixels that it is at least this much confident
    /// about. All other pixels will be classified as background. Default value is
    /// 0.5.
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
}
/// Prediction model parameters for Image Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The Model only returns up to that many top, by confidence score,
    /// predictions per instance. If this number is very high, the Model may return
    /// fewer predictions. Default value is 10.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
/// Prediction model parameters for Video Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The Model only returns up to that many top, by confidence score,
    /// predictions per instance. If this number is very high, the Model may return
    /// fewer predictions. Default value is 10,000.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
    /// Set to true to request segment-level classification. Vertex AI returns
    /// labels and their confidence scores for the entire time segment of the
    /// video that user specified in the input instance.
    /// Default value is true
    #[prost(bool, tag = "3")]
    pub segment_classification: bool,
    /// Set to true to request shot-level classification. Vertex AI determines
    /// the boundaries for each camera shot in the entire time segment of the
    /// video that user specified in the input instance. Vertex AI then
    /// returns labels and their confidence scores for each detected shot, along
    /// with the start and end time of the shot.
    /// WARNING: Model evaluation is not done for this classification type,
    /// the quality of it depends on the training data, but there are no metrics
    /// provided to describe that quality.
    /// Default value is false
    #[prost(bool, tag = "4")]
    pub shot_classification: bool,
    /// Set to true to request classification for a video at one-second intervals.
    /// Vertex AI returns labels and their confidence scores for each second of
    /// the entire time segment of the video that user specified in the input
    /// WARNING: Model evaluation is not done for this classification type, the
    /// quality of it depends on the training data, but there are no metrics
    /// provided to describe that quality. Default value is false
    #[prost(bool, tag = "5")]
    pub one_sec_interval_classification: bool,
}
/// Prediction model parameters for Video Action Recognition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The model only returns up to that many top, by confidence score,
    /// predictions per frame of the video. If this number is very high, the
    /// Model may return fewer predictions per frame. Default value is 50.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
