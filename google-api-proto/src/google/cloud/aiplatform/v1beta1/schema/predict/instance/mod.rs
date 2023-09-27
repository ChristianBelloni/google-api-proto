/// Prediction input format for Text Extraction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionPredictionInstance {
    /// The text snippet to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the text snippet. The supported MIME types are listed
    /// below.
    /// - text/plain
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// This field is only used for batch prediction. If a key is provided, the
    /// batch prediction result will by mapped to this key. If omitted, then the
    /// batch prediction result will contain the entire input instance. Vertex AI
    /// will not check if keys in the request are duplicates, so it is up to the
    /// caller to ensure the keys are unique.
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
/// Prediction input format for Text Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationPredictionInstance {
    /// The text snippet to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the text snippet. The supported MIME types are listed
    /// below.
    /// - text/plain
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Image Object Detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionInstance {
    /// The image bytes or Cloud Storage URI to make the prediction on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the image. Only the images in below listed
    /// MIME types are supported.
    /// - image/jpeg
    /// - image/gif
    /// - image/png
    /// - image/webp
    /// - image/bmp
    /// - image/tiff
    /// - image/vnd.microsoft.icon
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Video Object Tracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionInstance {
    /// The Google Cloud Storage location of the video on which to perform the
    /// prediction.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the video. Only the following are
    /// supported: video/mp4 video/avi video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision.
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    /// The end, exclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision, and "inf" or "Infinity" is allowed, which
    /// means the end of the video.
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
/// Prediction input format for Video Action Recognition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionInstance {
    /// The Google Cloud Storage location of the video on which to perform the
    /// prediction.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the video. Only the following are
    /// supported: video/mp4 video/avi video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision.
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    /// The end, exclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision, and "inf" or "Infinity" is allowed, which
    /// means the end of the video.
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
/// Prediction input format for Image Segmentation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionInstance {
    /// The image bytes to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the image. Only the images in below listed
    /// MIME types are supported.
    /// - image/jpeg
    /// - image/png
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Text Sentiment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentPredictionInstance {
    /// The text snippet to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the text snippet. The supported MIME types are listed
    /// below.
    /// - text/plain
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Image Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationPredictionInstance {
    /// The image bytes or Cloud Storage URI to make the prediction on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the image. Only the images in below listed
    /// MIME types are supported.
    /// - image/jpeg
    /// - image/gif
    /// - image/png
    /// - image/webp
    /// - image/bmp
    /// - image/tiff
    /// - image/vnd.microsoft.icon
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Video Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionInstance {
    /// The Google Cloud Storage location of the video on which to perform the
    /// prediction.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the video. Only the following are
    /// supported: video/mp4 video/avi video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision.
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    /// The end, exclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision, and "inf" or "Infinity" is allowed, which
    /// means the end of the video.
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
