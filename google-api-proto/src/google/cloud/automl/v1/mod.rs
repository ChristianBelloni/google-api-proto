/// Contains annotation details specific to classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationAnnotation {
    /// Output only. A confidence estimate between 0.0 and 1.0. A higher value
    /// means greater confidence that the annotation is positive. If a user
    /// approves an annotation as negative or positive, the score value remains
    /// unchanged. If a user creates an annotation, the score is 0 for negative or
    /// 1 for positive.
    #[prost(float, tag = "1")]
    pub score: f32,
}
/// Model evaluation metrics for classification problems.
/// Note: For Video Classification this metrics only describe quality of the
/// Video Classification predictions of "segment_classification" type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationEvaluationMetrics {
    /// Output only. The Area Under Precision-Recall Curve metric. Micro-averaged
    /// for the overall evaluation.
    #[prost(float, tag = "1")]
    pub au_prc: f32,
    /// Output only. The Area Under Receiver Operating Characteristic curve metric.
    /// Micro-averaged for the overall evaluation.
    #[prost(float, tag = "6")]
    pub au_roc: f32,
    /// Output only. The Log Loss metric.
    #[prost(float, tag = "7")]
    pub log_loss: f32,
    /// Output only. Metrics for each confidence_threshold in
    /// 0.00,0.05,0.10,...,0.95,0.96,0.97,0.98,0.99 and
    /// position_threshold = INT32_MAX_VALUE.
    /// ROC and precision-recall curves, and other aggregated metrics are derived
    /// from them. The confidence metrics entries may also be supplied for
    /// additional values of position_threshold, but from these no aggregated
    /// metrics are computed.
    #[prost(message, repeated, tag = "3")]
    pub confidence_metrics_entry: ::prost::alloc::vec::Vec<
        classification_evaluation_metrics::ConfidenceMetricsEntry,
    >,
    /// Output only. Confusion matrix of the evaluation.
    /// Only set for MULTICLASS classification problems where number
    /// of labels is no more than 10.
    /// Only set for model level evaluation, not for evaluation per label.
    #[prost(message, optional, tag = "4")]
    pub confusion_matrix: ::core::option::Option<
        classification_evaluation_metrics::ConfusionMatrix,
    >,
    /// Output only. The annotation spec ids used for this evaluation.
    #[prost(string, repeated, tag = "5")]
    pub annotation_spec_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ClassificationEvaluationMetrics`.
pub mod classification_evaluation_metrics {
    /// Metrics for a single confidence threshold.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfidenceMetricsEntry {
        /// Output only. Metrics are computed with an assumption that the model
        /// never returns predictions with score lower than this value.
        #[prost(float, tag = "1")]
        pub confidence_threshold: f32,
        /// Output only. Metrics are computed with an assumption that the model
        /// always returns at most this many predictions (ordered by their score,
        /// descendingly), but they all still need to meet the confidence_threshold.
        #[prost(int32, tag = "14")]
        pub position_threshold: i32,
        /// Output only. Recall (True Positive Rate) for the given confidence
        /// threshold.
        #[prost(float, tag = "2")]
        pub recall: f32,
        /// Output only. Precision for the given confidence threshold.
        #[prost(float, tag = "3")]
        pub precision: f32,
        /// Output only. False Positive Rate for the given confidence threshold.
        #[prost(float, tag = "8")]
        pub false_positive_rate: f32,
        /// Output only. The harmonic mean of recall and precision.
        #[prost(float, tag = "4")]
        pub f1_score: f32,
        /// Output only. The Recall (True Positive Rate) when only considering the
        /// label that has the highest prediction score and not below the confidence
        /// threshold for each example.
        #[prost(float, tag = "5")]
        pub recall_at1: f32,
        /// Output only. The precision when only considering the label that has the
        /// highest prediction score and not below the confidence threshold for each
        /// example.
        #[prost(float, tag = "6")]
        pub precision_at1: f32,
        /// Output only. The False Positive Rate when only considering the label that
        /// has the highest prediction score and not below the confidence threshold
        /// for each example.
        #[prost(float, tag = "9")]
        pub false_positive_rate_at1: f32,
        /// Output only. The harmonic mean of \[recall_at1][google.cloud.automl.v1.ClassificationEvaluationMetrics.ConfidenceMetricsEntry.recall_at1\] and \[precision_at1][google.cloud.automl.v1.ClassificationEvaluationMetrics.ConfidenceMetricsEntry.precision_at1\].
        #[prost(float, tag = "7")]
        pub f1_score_at1: f32,
        /// Output only. The number of model created labels that match a ground truth
        /// label.
        #[prost(int64, tag = "10")]
        pub true_positive_count: i64,
        /// Output only. The number of model created labels that do not match a
        /// ground truth label.
        #[prost(int64, tag = "11")]
        pub false_positive_count: i64,
        /// Output only. The number of ground truth labels that are not matched
        /// by a model created label.
        #[prost(int64, tag = "12")]
        pub false_negative_count: i64,
        /// Output only. The number of labels that were not created by the model,
        /// but if they would, they would not match a ground truth label.
        #[prost(int64, tag = "13")]
        pub true_negative_count: i64,
    }
    /// Confusion matrix of the model running the classification.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfusionMatrix {
        /// Output only. IDs of the annotation specs used in the confusion matrix.
        /// For Tables CLASSIFICATION
        /// \[prediction_type][google.cloud.automl.v1p1beta.TablesModelMetadata.prediction_type\]
        /// only list of \[annotation_spec_display_name-s][\] is populated.
        #[prost(string, repeated, tag = "1")]
        pub annotation_spec_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. Display name of the annotation specs used in the confusion
        /// matrix, as they were at the moment of the evaluation. For Tables
        /// CLASSIFICATION
        /// \[prediction_type-s][google.cloud.automl.v1p1beta.TablesModelMetadata.prediction_type\],
        /// distinct values of the target column at the moment of the model
        /// evaluation are populated here.
        #[prost(string, repeated, tag = "3")]
        pub display_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. Rows in the confusion matrix. The number of rows is equal to
        /// the size of `annotation_spec_id`.
        /// `row\[i].example_count[j\]` is the number of examples that have ground
        /// truth of the `annotation_spec_id\[i\]` and are predicted as
        /// `annotation_spec_id\[j\]` by the model being evaluated.
        #[prost(message, repeated, tag = "2")]
        pub row: ::prost::alloc::vec::Vec<confusion_matrix::Row>,
    }
    /// Nested message and enum types in `ConfusionMatrix`.
    pub mod confusion_matrix {
        /// Output only. A row in the confusion matrix.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Row {
            /// Output only. Value of the specific cell in the confusion matrix.
            /// The number of values each row has (i.e. the length of the row) is equal
            /// to the length of the `annotation_spec_id` field or, if that one is not
            /// populated, length of the \[display_name][google.cloud.automl.v1.ClassificationEvaluationMetrics.ConfusionMatrix.display_name\] field.
            #[prost(int32, repeated, tag = "1")]
            pub example_count: ::prost::alloc::vec::Vec<i32>,
        }
    }
}
/// Type of the classification problem.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClassificationType {
    /// An un-set value of this enum.
    Unspecified = 0,
    /// At most one label is allowed per example.
    Multiclass = 1,
    /// Multiple labels are allowed for one example.
    Multilabel = 2,
}
impl ClassificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClassificationType::Unspecified => "CLASSIFICATION_TYPE_UNSPECIFIED",
            ClassificationType::Multiclass => "MULTICLASS",
            ClassificationType::Multilabel => "MULTILABEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLASSIFICATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MULTICLASS" => Some(Self::Multiclass),
            "MULTILABEL" => Some(Self::Multilabel),
            _ => None,
        }
    }
}
/// Dataset metadata that is specific to image classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationDatasetMetadata {
    /// Required. Type of the classification problem.
    #[prost(enumeration = "ClassificationType", tag = "1")]
    pub classification_type: i32,
}
/// Dataset metadata specific to image object detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionDatasetMetadata {}
/// Model metadata for image classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationModelMetadata {
    /// Optional. The ID of the `base` model. If it is specified, the new model
    /// will be created based on the `base` model. Otherwise, the new model will be
    /// created from scratch. The `base` model must be in the same
    /// `project` and `location` as the new model to create, and have the same
    /// `model_type`.
    #[prost(string, tag = "1")]
    pub base_model_id: ::prost::alloc::string::String,
    /// Optional. The train budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour. The actual
    /// `train_cost` will be equal or less than this value. If further model
    /// training ceases to provide any improvements, it will stop without using
    /// full budget and the stop_reason will be `MODEL_CONVERGED`.
    /// Note, node_hour  = actual_hour * number_of_nodes_invovled.
    /// For model type `cloud`(default), the train budget must be between 8,000
    /// and 800,000 milli node hours, inclusive. The default value is 192, 000
    /// which represents one day in wall time. For model type
    /// `mobile-low-latency-1`, `mobile-versatile-1`, `mobile-high-accuracy-1`,
    /// `mobile-core-ml-low-latency-1`, `mobile-core-ml-versatile-1`,
    /// `mobile-core-ml-high-accuracy-1`, the train budget must be between 1,000
    /// and 100,000 milli node hours, inclusive. The default value is 24, 000 which
    /// represents one day in wall time.
    #[prost(int64, tag = "16")]
    pub train_budget_milli_node_hours: i64,
    /// Output only. The actual train cost of creating this model, expressed in
    /// milli node hours, i.e. 1,000 value in this field means 1 node hour.
    /// Guaranteed to not exceed the train budget.
    #[prost(int64, tag = "17")]
    pub train_cost_milli_node_hours: i64,
    /// Output only. The reason that this create model operation stopped,
    /// e.g. `BUDGET_REACHED`, `MODEL_CONVERGED`.
    #[prost(string, tag = "5")]
    pub stop_reason: ::prost::alloc::string::String,
    /// Optional. Type of the model. The available values are:
    /// *   `cloud` - Model to be used via prediction calls to AutoML API.
    ///                This is the default value.
    /// *   `mobile-low-latency-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile or edge device
    ///                with TensorFlow afterwards. Expected to have low latency, but
    ///                may have lower prediction quality than other models.
    /// *   `mobile-versatile-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile or edge device
    ///                with TensorFlow afterwards.
    /// *   `mobile-high-accuracy-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile or edge device
    ///                with TensorFlow afterwards.  Expected to have a higher
    ///                latency, but should also have a higher prediction quality
    ///                than other models.
    /// *   `mobile-core-ml-low-latency-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile device with Core
    ///                ML afterwards. Expected to have low latency, but may have
    ///                lower prediction quality than other models.
    /// *   `mobile-core-ml-versatile-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile device with Core
    ///                ML afterwards.
    /// *   `mobile-core-ml-high-accuracy-1` - A model that, in addition to
    ///                providing prediction via AutoML API, can also be exported
    ///                (see \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile device with
    ///                Core ML afterwards.  Expected to have a higher latency, but
    ///                should also have a higher prediction quality than other
    ///                models.
    #[prost(string, tag = "7")]
    pub model_type: ::prost::alloc::string::String,
    /// Output only. An approximate number of online prediction QPS that can
    /// be supported by this model per each node on which it is deployed.
    #[prost(double, tag = "13")]
    pub node_qps: f64,
    /// Output only. The number of nodes this model is deployed on. A node is an
    /// abstraction of a machine resource, which can handle online prediction QPS
    /// as given in the node_qps field.
    #[prost(int64, tag = "14")]
    pub node_count: i64,
}
/// Model metadata specific to image object detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionModelMetadata {
    /// Optional. Type of the model. The available values are:
    /// *   `cloud-high-accuracy-1` - (default) A model to be used via prediction
    ///                calls to AutoML API. Expected to have a higher latency, but
    ///                should also have a higher prediction quality than other
    ///                models.
    /// *   `cloud-low-latency-1` -  A model to be used via prediction
    ///                calls to AutoML API. Expected to have low latency, but may
    ///                have lower prediction quality than other models.
    /// *   `mobile-low-latency-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile or edge device
    ///                with TensorFlow afterwards. Expected to have low latency, but
    ///                may have lower prediction quality than other models.
    /// *   `mobile-versatile-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile or edge device
    ///                with TensorFlow afterwards.
    /// *   `mobile-high-accuracy-1` - A model that, in addition to providing
    ///                prediction via AutoML API, can also be exported (see
    ///                \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\]) and used on a mobile or edge device
    ///                with TensorFlow afterwards.  Expected to have a higher
    ///                latency, but should also have a higher prediction quality
    ///                than other models.
    #[prost(string, tag = "1")]
    pub model_type: ::prost::alloc::string::String,
    /// Output only. The number of nodes this model is deployed on. A node is an
    /// abstraction of a machine resource, which can handle online prediction QPS
    /// as given in the qps_per_node field.
    #[prost(int64, tag = "3")]
    pub node_count: i64,
    /// Output only. An approximate number of online prediction QPS that can
    /// be supported by this model per each node on which it is deployed.
    #[prost(double, tag = "4")]
    pub node_qps: f64,
    /// Output only. The reason that this create model operation stopped,
    /// e.g. `BUDGET_REACHED`, `MODEL_CONVERGED`.
    #[prost(string, tag = "5")]
    pub stop_reason: ::prost::alloc::string::String,
    /// Optional. The train budget of creating this model, expressed in milli node
    /// hours i.e. 1,000 value in this field means 1 node hour. The actual
    /// `train_cost` will be equal or less than this value. If further model
    /// training ceases to provide any improvements, it will stop without using
    /// full budget and the stop_reason will be `MODEL_CONVERGED`.
    /// Note, node_hour  = actual_hour * number_of_nodes_invovled.
    /// For model type `cloud-high-accuracy-1`(default) and `cloud-low-latency-1`,
    /// the train budget must be between 20,000 and 900,000 milli node hours,
    /// inclusive. The default value is 216, 000 which represents one day in
    /// wall time.
    /// For model type `mobile-low-latency-1`, `mobile-versatile-1`,
    /// `mobile-high-accuracy-1`, `mobile-core-ml-low-latency-1`,
    /// `mobile-core-ml-versatile-1`, `mobile-core-ml-high-accuracy-1`, the train
    /// budget must be between 1,000 and 100,000 milli node hours, inclusive.
    /// The default value is 24, 000 which represents one day in wall time.
    #[prost(int64, tag = "6")]
    pub train_budget_milli_node_hours: i64,
    /// Output only. The actual train cost of creating this model, expressed in
    /// milli node hours, i.e. 1,000 value in this field means 1 node hour.
    /// Guaranteed to not exceed the train budget.
    #[prost(int64, tag = "7")]
    pub train_cost_milli_node_hours: i64,
}
/// Model deployment metadata specific to Image Classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationModelDeploymentMetadata {
    /// Input only. The number of nodes to deploy the model on. A node is an
    /// abstraction of a machine resource, which can handle online prediction QPS
    /// as given in the model's
    /// \[node_qps][google.cloud.automl.v1.ImageClassificationModelMetadata.node_qps\].
    /// Must be between 1 and 100, inclusive on both ends.
    #[prost(int64, tag = "1")]
    pub node_count: i64,
}
/// Model deployment metadata specific to Image Object Detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionModelDeploymentMetadata {
    /// Input only. The number of nodes to deploy the model on. A node is an
    /// abstraction of a machine resource, which can handle online prediction QPS
    /// as given in the model's
    /// \[qps_per_node][google.cloud.automl.v1.ImageObjectDetectionModelMetadata.qps_per_node\].
    /// Must be between 1 and 100, inclusive on both ends.
    #[prost(int64, tag = "1")]
    pub node_count: i64,
}
/// Dataset metadata for classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationDatasetMetadata {
    /// Required. Type of the classification problem.
    #[prost(enumeration = "ClassificationType", tag = "1")]
    pub classification_type: i32,
}
/// Model metadata that is specific to text classification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationModelMetadata {
    /// Output only. Classification type of the dataset used to train this model.
    #[prost(enumeration = "ClassificationType", tag = "3")]
    pub classification_type: i32,
}
/// Dataset metadata that is specific to text extraction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionDatasetMetadata {}
/// Model metadata that is specific to text extraction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionModelMetadata {}
/// Dataset metadata for text sentiment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentDatasetMetadata {
    /// Required. A sentiment is expressed as an integer ordinal, where higher value
    /// means a more positive sentiment. The range of sentiments that will be used
    /// is between 0 and sentiment_max (inclusive on both ends), and all the values
    /// in the range must be represented in the dataset before a model can be
    /// created.
    /// sentiment_max value must be between 1 and 10 (inclusive).
    #[prost(int32, tag = "1")]
    pub sentiment_max: i32,
}
/// Model metadata that is specific to text sentiment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentModelMetadata {}
/// A vertex represents a 2D point in the image.
/// The normalized vertex coordinates are between 0 to 1 fractions relative to
/// the original plane (image, video). E.g. if the plane (e.g. whole image) would
/// have size 10 x 20 then a point with normalized coordinates (0.1, 0.3) would
/// be at the position (1, 6) on that plane.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// Required. Horizontal coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Required. Vertical coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon of a detected object on a plane.
/// On output both vertices and normalized_vertices are provided.
/// The polygon is formed by connecting vertices in the order they are listed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// Output only . The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Input configuration for \[AutoMl.ImportData][google.cloud.automl.v1.AutoMl.ImportData\] action.
///
/// The format of input depends on dataset_metadata the Dataset into which
/// the import is happening has. As input source the
/// \[gcs_source][google.cloud.automl.v1.InputConfig.gcs_source\]
/// is expected, unless specified otherwise. Additionally any input .CSV file
/// by itself must be 100MB or smaller, unless specified otherwise.
/// If an "example" file (that is, image, video etc.) with identical content
/// (even if it had different `GCS_FILE_PATH`) is mentioned multiple times, then
/// its label, bounding boxes etc. are appended. The same file should be always
/// provided with the same `ML_USE` and `GCS_FILE_PATH`, if it is not, then
/// these values are nondeterministically selected from the given ones.
///
/// The formats are represented in EBNF with commas being literal and with
/// non-terminal symbols defined near the end of this comment. The formats are:
///
/// <h4>AutoML Vision</h4>
///
///
/// <div class="ds-selector-tabs"><section><h5>Classification</h5>
///
/// See [Preparing your training
/// data](<https://cloud.google.com/vision/automl/docs/prepare>) for more
/// information.
///
/// CSV file(s) with each line in format:
///
///      ML_USE,GCS_FILE_PATH,LABEL,LABEL,...
///
/// *   `ML_USE` - Identifies the data set that the current row (file) applies
/// to.
///      This value can be one of the following:
///      * `TRAIN` - Rows in this file are used to train the model.
///      * `TEST` - Rows in this file are used to test the model during training.
///      * `UNASSIGNED` - Rows in this file are not categorized. They are
///         Automatically divided into train and test data. 80% for training and
///         20% for testing.
///
/// *   `GCS_FILE_PATH` - The Google Cloud Storage location of an image of up to
///       30MB in size. Supported extensions: .JPEG, .GIF, .PNG, .WEBP, .BMP,
///       .TIFF, .ICO.
///
/// *   `LABEL` - A label that identifies the object in the image.
///
/// For the `MULTICLASS` classification type, at most one `LABEL` is allowed
/// per image. If an image has not yet been labeled, then it should be
/// mentioned just once with no `LABEL`.
///
/// Some sample rows:
///
///      TRAIN,gs://folder/image1.jpg,daisy
///      TEST,gs://folder/image2.jpg,dandelion,tulip,rose
///      UNASSIGNED,gs://folder/image3.jpg,daisy
///      UNASSIGNED,gs://folder/image4.jpg
///
///
/// </section><section><h5>Object Detection</h5>
/// See [Preparing your training
/// data](<https://cloud.google.com/vision/automl/object-detection/docs/prepare>)
/// for more information.
///
/// A CSV file(s) with each line in format:
///
///      ML_USE,GCS_FILE_PATH,\[LABEL\],(BOUNDING_BOX | ,,,,,,,)
///
/// *   `ML_USE` - Identifies the data set that the current row (file) applies
/// to.
///      This value can be one of the following:
///      * `TRAIN` - Rows in this file are used to train the model.
///      * `TEST` - Rows in this file are used to test the model during training.
///      * `UNASSIGNED` - Rows in this file are not categorized. They are
///         Automatically divided into train and test data. 80% for training and
///         20% for testing.
///
/// *  `GCS_FILE_PATH` - The Google Cloud Storage location of an image of up to
///      30MB in size. Supported extensions: .JPEG, .GIF, .PNG. Each image
///      is assumed to be exhaustively labeled.
///
/// *  `LABEL` - A label that identifies the object in the image specified by the
///     `BOUNDING_BOX`.
///
/// *  `BOUNDING BOX` - The vertices of an object in the example image.
///     The minimum allowed `BOUNDING_BOX` edge length is 0.01, and no more than
///     500 `BOUNDING_BOX` instances per image are allowed (one `BOUNDING_BOX`
///     per line). If an image has no looked for objects then it should be
///     mentioned just once with no LABEL and the ",,,,,,," in place of the
///    `BOUNDING_BOX`.
///
/// **Four sample rows:**
///
///      TRAIN,gs://folder/image1.png,car,0.1,0.1,,,0.3,0.3,,
///      TRAIN,gs://folder/image1.png,bike,.7,.6,,,.8,.9,,
///      UNASSIGNED,gs://folder/im2.png,car,0.1,0.1,0.2,0.1,0.2,0.3,0.1,0.3
///      TEST,gs://folder/im3.png,,,,,,,,,
///    </section>
/// </div>
///
///
/// <h4>AutoML Video Intelligence</h4>
///
///
/// <div class="ds-selector-tabs"><section><h5>Classification</h5>
///
/// See [Preparing your training
/// data](<https://cloud.google.com/video-intelligence/automl/docs/prepare>) for
/// more information.
///
/// CSV file(s) with each line in format:
///
///      ML_USE,GCS_FILE_PATH
///
/// For `ML_USE`, do not use `VALIDATE`.
///
/// `GCS_FILE_PATH` is the path to another .csv file that describes training
/// example for a given `ML_USE`, using the following row format:
///
///      GCS_FILE_PATH,(LABEL,TIME_SEGMENT_START,TIME_SEGMENT_END | ,,)
///
/// Here `GCS_FILE_PATH` leads to a video of up to 50GB in size and up
/// to 3h duration. Supported extensions: .MOV, .MPEG4, .MP4, .AVI.
///
/// `TIME_SEGMENT_START` and `TIME_SEGMENT_END` must be within the
/// length of the video, and the end time must be after the start time. Any
/// segment of a video which has one or more labels on it, is considered a
/// hard negative for all other labels. Any segment with no labels on
/// it is considered to be unknown. If a whole video is unknown, then
/// it should be mentioned just once with ",," in place of `LABEL,
/// TIME_SEGMENT_START,TIME_SEGMENT_END`.
///
/// Sample top level CSV file:
///
///      TRAIN,gs://folder/train_videos.csv
///      TEST,gs://folder/test_videos.csv
///      UNASSIGNED,gs://folder/other_videos.csv
///
/// Sample rows of a CSV file for a particular ML_USE:
///
///      gs://folder/video1.avi,car,120,180.000021
///      gs://folder/video1.avi,bike,150,180.000021
///      gs://folder/vid2.avi,car,0,60.5
///      gs://folder/vid3.avi,,,
///
///
///
/// </section><section><h5>Object Tracking</h5>
///
/// See [Preparing your training
/// data](/video-intelligence/automl/object-tracking/docs/prepare) for more
/// information.
///
/// CSV file(s) with each line in format:
///
///      ML_USE,GCS_FILE_PATH
///
/// For `ML_USE`, do not use `VALIDATE`.
///
/// `GCS_FILE_PATH` is the path to another .csv file that describes training
/// example for a given `ML_USE`, using the following row format:
///
///      GCS_FILE_PATH,LABEL,\[INSTANCE_ID\],TIMESTAMP,BOUNDING_BOX
///
/// or
///
///      GCS_FILE_PATH,,,,,,,,,,
///
/// Here `GCS_FILE_PATH` leads to a video of up to 50GB in size and up
/// to 3h duration. Supported extensions: .MOV, .MPEG4, .MP4, .AVI.
/// Providing `INSTANCE_ID`s can help to obtain a better model. When
/// a specific labeled entity leaves the video frame, and shows up
/// afterwards it is not required, albeit preferable, that the same
/// `INSTANCE_ID` is given to it.
///
/// `TIMESTAMP` must be within the length of the video, the
/// `BOUNDING_BOX` is assumed to be drawn on the closest video's frame
/// to the `TIMESTAMP`. Any mentioned by the `TIMESTAMP` frame is expected
/// to be exhaustively labeled and no more than 500 `BOUNDING_BOX`-es per
/// frame are allowed. If a whole video is unknown, then it should be
/// mentioned just once with ",,,,,,,,,," in place of `LABEL,
/// \[INSTANCE_ID\],TIMESTAMP,BOUNDING_BOX`.
///
/// Sample top level CSV file:
///
///       TRAIN,gs://folder/train_videos.csv
///       TEST,gs://folder/test_videos.csv
///       UNASSIGNED,gs://folder/other_videos.csv
///
/// Seven sample rows of a CSV file for a particular ML_USE:
///
///       gs://folder/video1.avi,car,1,12.10,0.8,0.8,0.9,0.8,0.9,0.9,0.8,0.9
///       gs://folder/video1.avi,car,1,12.90,0.4,0.8,0.5,0.8,0.5,0.9,0.4,0.9
///       gs://folder/video1.avi,car,2,12.10,.4,.2,.5,.2,.5,.3,.4,.3
///       gs://folder/video1.avi,car,2,12.90,.8,.2,,,.9,.3,,
///       gs://folder/video1.avi,bike,,12.50,.45,.45,,,.55,.55,,
///       gs://folder/video2.avi,car,1,0,.1,.9,,,.9,.1,,
///       gs://folder/video2.avi,,,,,,,,,,,
///    </section>
/// </div>
///
///
/// <h4>AutoML Natural Language</h4>
///
///
/// <div class="ds-selector-tabs"><section><h5>Entity Extraction</h5>
///
/// See [Preparing your training
/// data](/natural-language/automl/entity-analysis/docs/prepare) for more
/// information.
///
/// One or more CSV file(s) with each line in the following format:
///
///      ML_USE,GCS_FILE_PATH
///
/// *   `ML_USE` - Identifies the data set that the current row (file) applies
/// to.
///      This value can be one of the following:
///      * `TRAIN` - Rows in this file are used to train the model.
///      * `TEST` - Rows in this file are used to test the model during training.
///      * `UNASSIGNED` - Rows in this file are not categorized. They are
///         Automatically divided into train and test data. 80% for training and
///         20% for testing..
///
/// *   `GCS_FILE_PATH` - a Identifies JSON Lines (.JSONL) file stored in
///       Google Cloud Storage that contains in-line text in-line as documents
///       for model training.
///
/// After the training data set has been determined from the `TRAIN` and
/// `UNASSIGNED` CSV files, the training data is divided into train and
/// validation data sets. 70% for training and 30% for validation.
///
/// For example:
///
///      TRAIN,gs://folder/file1.jsonl
///      VALIDATE,gs://folder/file2.jsonl
///      TEST,gs://folder/file3.jsonl
///
/// **In-line JSONL files**
///
/// In-line .JSONL files contain, per line, a JSON document that wraps a
/// \[`text_snippet`][google.cloud.automl.v1.TextSnippet\] field followed by
/// one or more \[`annotations`][google.cloud.automl.v1.AnnotationPayload\]
/// fields, which have `display_name` and `text_extraction` fields to describe
/// the entity from the text snippet. Multiple JSON documents can be separated
/// using line breaks (\n).
///
/// The supplied text must be annotated exhaustively. For example, if you
/// include the text "horse", but do not label it as "animal",
/// then "horse" is assumed to not be an "animal".
///
/// Any given text snippet content must have 30,000 characters or
/// less, and also be UTF-8 NFC encoded. ASCII is accepted as it is
/// UTF-8 NFC encoded.
///
/// For example:
///
///      {
///        "text_snippet": {
///          "content": "dog car cat"
///        },
///        "annotations": [
///           {
///             "display_name": "animal",
///             "text_extraction": {
///               "text_segment": {"start_offset": 0, "end_offset": 2}
///            }
///           },
///           {
///            "display_name": "vehicle",
///             "text_extraction": {
///               "text_segment": {"start_offset": 4, "end_offset": 6}
///             }
///           },
///           {
///             "display_name": "animal",
///             "text_extraction": {
///               "text_segment": {"start_offset": 8, "end_offset": 10}
///             }
///           }
///       ]
///      }\n
///      {
///         "text_snippet": {
///           "content": "This dog is good."
///         },
///         "annotations": [
///            {
///              "display_name": "animal",
///              "text_extraction": {
///                "text_segment": {"start_offset": 5, "end_offset": 7}
///              }
///            }
///         ]
///      }
///
/// **JSONL files that reference documents**
///
/// .JSONL files contain, per line, a JSON document that wraps a
/// `input_config` that contains the path to a source document.
/// Multiple JSON documents can be separated using line breaks (\n).
///
/// Supported document extensions: .PDF, .TIF, .TIFF
///
/// For example:
///
///      {
///        "document": {
///          "input_config": {
///            "gcs_source": { "input_uris": [ "gs://folder/document1.pdf" ]
///            }
///          }
///        }
///      }\n
///      {
///        "document": {
///          "input_config": {
///            "gcs_source": { "input_uris": [ "gs://folder/document2.tif" ]
///            }
///          }
///        }
///      }
///
/// **In-line JSONL files with document layout information**
///
/// **Note:** You can only annotate documents using the UI. The format described
/// below applies to annotated documents exported using the UI or `exportData`.
///
/// In-line .JSONL files for documents contain, per line, a JSON document
/// that wraps a `document` field that provides the textual content of the
/// document and the layout information.
///
/// For example:
///
///      {
///        "document": {
///                "document_text": {
///                  "content": "dog car cat"
///                }
///                "layout": [
///                  {
///                    "text_segment": {
///                      "start_offset": 0,
///                      "end_offset": 11,
///                     },
///                     "page_number": 1,
///                     "bounding_poly": {
///                        "normalized_vertices": [
///                          {"x": 0.1, "y": 0.1},
///                          {"x": 0.1, "y": 0.3},
///                          {"x": 0.3, "y": 0.3},
///                          {"x": 0.3, "y": 0.1},
///                        ],
///                      },
///                      "text_segment_type": TOKEN,
///                  }
///                ],
///                "document_dimensions": {
///                  "width": 8.27,
///                  "height": 11.69,
///                  "unit": INCH,
///                }
///                "page_count": 3,
///              },
///              "annotations": [
///                {
///                  "display_name": "animal",
///                  "text_extraction": {
///                    "text_segment": {"start_offset": 0, "end_offset": 3}
///                  }
///                },
///                {
///                  "display_name": "vehicle",
///                  "text_extraction": {
///                    "text_segment": {"start_offset": 4, "end_offset": 7}
///                  }
///                },
///                {
///                  "display_name": "animal",
///                  "text_extraction": {
///                    "text_segment": {"start_offset": 8, "end_offset": 11}
///                  }
///                },
///              ],
///
///
///
///
/// </section><section><h5>Classification</h5>
///
/// See [Preparing your training
/// data](<https://cloud.google.com/natural-language/automl/docs/prepare>) for more
/// information.
///
/// One or more CSV file(s) with each line in the following format:
///
///      ML_USE,(TEXT_SNIPPET | GCS_FILE_PATH),LABEL,LABEL,...
///
/// *   `ML_USE` - Identifies the data set that the current row (file) applies
/// to.
///      This value can be one of the following:
///      * `TRAIN` - Rows in this file are used to train the model.
///      * `TEST` - Rows in this file are used to test the model during training.
///      * `UNASSIGNED` - Rows in this file are not categorized. They are
///         Automatically divided into train and test data. 80% for training and
///         20% for testing.
///
/// *   `TEXT_SNIPPET` and `GCS_FILE_PATH` are distinguished by a pattern. If
///      the column content is a valid Google Cloud Storage file path, that is,
///      prefixed by "gs://", it is treated as a `GCS_FILE_PATH`. Otherwise, if
///      the content is enclosed in double quotes (""), it is treated as a
///      `TEXT_SNIPPET`. For `GCS_FILE_PATH`, the path must lead to a
///      file with supported extension and UTF-8 encoding, for example,
///      "gs://folder/content.txt" AutoML imports the file content
///      as a text snippet. For `TEXT_SNIPPET`, AutoML imports the column content
///      excluding quotes. In both cases, size of the content must be 10MB or
///      less in size. For zip files, the size of each file inside the zip must be
///      10MB or less in size.
///
///      For the `MULTICLASS` classification type, at most one `LABEL` is allowed.
///
///      The `ML_USE` and `LABEL` columns are optional.
///      Supported file extensions: .TXT, .PDF, .TIF, .TIFF, .ZIP
///
/// A maximum of 100 unique labels are allowed per CSV row.
///
/// Sample rows:
///
///      TRAIN,"They have bad food and very rude",RudeService,BadFood
///      gs://folder/content.txt,SlowService
///      TEST,gs://folder/document.pdf
///      VALIDATE,gs://folder/text_files.zip,BadFood
///
///
///
/// </section><section><h5>Sentiment Analysis</h5>
///
/// See [Preparing your training
/// data](<https://cloud.google.com/natural-language/automl/docs/prepare>) for more
/// information.
///
/// CSV file(s) with each line in format:
///
///      ML_USE,(TEXT_SNIPPET | GCS_FILE_PATH),SENTIMENT
///
/// *   `ML_USE` - Identifies the data set that the current row (file) applies
/// to.
///      This value can be one of the following:
///      * `TRAIN` - Rows in this file are used to train the model.
///      * `TEST` - Rows in this file are used to test the model during training.
///      * `UNASSIGNED` - Rows in this file are not categorized. They are
///         Automatically divided into train and test data. 80% for training and
///         20% for testing.
///
/// *   `TEXT_SNIPPET` and `GCS_FILE_PATH` are distinguished by a pattern. If
///      the column content is a valid  Google Cloud Storage file path, that is,
///      prefixed by "gs://", it is treated as a `GCS_FILE_PATH`. Otherwise, if
///      the content is enclosed in double quotes (""), it is treated as a
///      `TEXT_SNIPPET`. For `GCS_FILE_PATH`, the path must lead to a
///      file with supported extension and UTF-8 encoding, for example,
///      "gs://folder/content.txt" AutoML imports the file content
///      as a text snippet. For `TEXT_SNIPPET`, AutoML imports the column content
///      excluding quotes. In both cases, size of the content must be 128kB or
///      less in size. For zip files, the size of each file inside the zip must be
///      128kB or less in size.
///
///      The `ML_USE` and `SENTIMENT` columns are optional.
///      Supported file extensions: .TXT, .PDF, .TIF, .TIFF, .ZIP
///
/// *  `SENTIMENT` - An integer between 0 and
///      Dataset.text_sentiment_dataset_metadata.sentiment_max
///      (inclusive). Describes the ordinal of the sentiment - higher
///      value means a more positive sentiment. All the values are
///      completely relative, i.e. neither 0 needs to mean a negative or
///      neutral sentiment nor sentiment_max needs to mean a positive one -
///      it is just required that 0 is the least positive sentiment
///      in the data, and sentiment_max is the  most positive one.
///      The SENTIMENT shouldn't be confused with "score" or "magnitude"
///      from the previous Natural Language Sentiment Analysis API.
///      All SENTIMENT values between 0 and sentiment_max must be
///      represented in the imported data. On prediction the same 0 to
///      sentiment_max range will be used. The difference between
///      neighboring sentiment values needs not to be uniform, e.g. 1 and
///      2 may be similar whereas the difference between 2 and 3 may be
///      large.
///
/// Sample rows:
///
///      TRAIN,"@freewrytin this is way too good for your product",2
///      gs://folder/content.txt,3
///      TEST,gs://folder/document.pdf
///      VALIDATE,gs://folder/text_files.zip,2
///    </section>
/// </div>
///
///
///
/// <h4>AutoML Tables</h4><div class="ui-datasection-main"><section
/// class="selected">
///
/// See [Preparing your training
/// data](<https://cloud.google.com/automl-tables/docs/prepare>) for more
/// information.
///
/// You can use either
/// \[gcs_source][google.cloud.automl.v1.InputConfig.gcs_source\] or
/// \[bigquery_source][google.cloud.automl.v1.InputConfig.bigquery_source\].
/// All input is concatenated into a
/// single
/// \[primary_table_spec_id][google.cloud.automl.v1.TablesDatasetMetadata.primary_table_spec_id\]
///
/// **For gcs_source:**
///
/// CSV file(s), where the first row of the first file is the header,
/// containing unique column names. If the first row of a subsequent
/// file is the same as the header, then it is also treated as a
/// header. All other rows contain values for the corresponding
/// columns.
///
/// Each .CSV file by itself must be 10GB or smaller, and their total
/// size must be 100GB or smaller.
///
/// First three sample rows of a CSV file:
/// <pre>
/// "Id","First Name","Last Name","Dob","Addresses"
/// "1","John","Doe","1968-01-22","\[{"status":"current","address":"123_First_Avenue","city":"Seattle","state":"WA","zip":"11111","numberOfYears":"1"},{"status":"previous","address":"456_Main_Street","city":"Portland","state":"OR","zip":"22222","numberOfYears":"5"}\]"
/// "2","Jane","Doe","1980-10-16","\[{"status":"current","address":"789_Any_Avenue","city":"Albany","state":"NY","zip":"33333","numberOfYears":"2"},{"status":"previous","address":"321_Main_Street","city":"Hoboken","state":"NJ","zip":"44444","numberOfYears":"3"}\]}
/// </pre>
/// **For bigquery_source:**
///
/// An URI of a BigQuery table. The user data size of the BigQuery
/// table must be 100GB or smaller.
///
/// An imported table must have between 2 and 1,000 columns, inclusive,
/// and between 1000 and 100,000,000 rows, inclusive. There are at most 5
/// import data running in parallel.
///
///    </section>
/// </div>
///
///
/// **Input field definitions:**
///
/// `ML_USE`
/// : ("TRAIN" | "VALIDATE" | "TEST" | "UNASSIGNED")
///    Describes how the given example (file) should be used for model
///    training. "UNASSIGNED" can be used when user has no preference.
///
/// `GCS_FILE_PATH`
/// : The path to a file on Google Cloud Storage. For example,
///    "gs://folder/image1.png".
///
/// `LABEL`
/// : A display name of an object on an image, video etc., e.g. "dog".
///    Must be up to 32 characters long and can consist only of ASCII
///    Latin letters A-Z and a-z, underscores(_), and ASCII digits 0-9.
///    For each label an AnnotationSpec is created which display_name
///    becomes the label; AnnotationSpecs are given back in predictions.
///
/// `INSTANCE_ID`
/// : A positive integer that identifies a specific instance of a
///    labeled entity on an example. Used e.g. to track two cars on
///    a video while being able to tell apart which one is which.
///
/// `BOUNDING_BOX`
/// : (`VERTEX,VERTEX,VERTEX,VERTEX` | `VERTEX,,,VERTEX,,`)
///    A rectangle parallel to the frame of the example (image,
///    video). If 4 vertices are given they are connected by edges
///    in the order provided, if 2 are given they are recognized
///    as diagonally opposite vertices of the rectangle.
///
/// `VERTEX`
/// : (`COORDINATE,COORDINATE`)
///    First coordinate is horizontal (x), the second is vertical (y).
///
/// `COORDINATE`
/// : A float in 0 to 1 range, relative to total length of
///    image or video in given dimension. For fractions the
///    leading non-decimal 0 can be omitted (i.e. 0.3 = .3).
///    Point 0,0 is in top left.
///
/// `TIME_SEGMENT_START`
/// : (`TIME_OFFSET`)
///    Expresses a beginning, inclusive, of a time segment
///    within an example that has a time dimension
///    (e.g. video).
///
/// `TIME_SEGMENT_END`
/// : (`TIME_OFFSET`)
///    Expresses an end, exclusive, of a time segment within
///    n example that has a time dimension (e.g. video).
///
/// `TIME_OFFSET`
/// : A number of seconds as measured from the start of an
///    example (e.g. video). Fractions are allowed, up to a
///    microsecond precision. "inf" is allowed, and it means the end
///    of the example.
///
/// `TEXT_SNIPPET`
/// : The content of a text snippet, UTF-8 encoded, enclosed within
///    double quotes ("").
///
/// `DOCUMENT`
/// : A field that provides the textual content with document and the layout
///    information.
///
///
///   **Errors:**
///
///   If any of the provided CSV files can't be parsed or if more than certain
///   percent of CSV rows cannot be processed then the operation fails and
///   nothing is imported. Regardless of overall success or failure the per-row
///   failures, up to a certain count cap, is listed in
///   Operation.metadata.partial_failures.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Additional domain-specific parameters describing the semantic of the
    /// imported data, any string must be up to 25000
    /// characters long.
    ///
    /// <h4>AutoML Tables</h4>
    ///
    /// `schema_inference_version`
    /// : (integer) This value must be supplied.
    ///    The version of the
    ///    algorithm to use for the initial inference of the
    ///    column data types of the imported table. Allowed values: "1".
    #[prost(btree_map = "string, string", tag = "2")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The source of the input.
    #[prost(oneof = "input_config::Source", tags = "1")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// The source of the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location for the input content.
        /// For \[AutoMl.ImportData][google.cloud.automl.v1.AutoMl.ImportData\], `gcs_source` points to a CSV file with
        /// a structure described in \[InputConfig][google.cloud.automl.v1.InputConfig\].
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
    }
}
/// Input configuration for BatchPredict Action.
///
/// The format of input depends on the ML problem of the model used for
/// prediction. As input source the
/// \[gcs_source][google.cloud.automl.v1.InputConfig.gcs_source\]
/// is expected, unless specified otherwise.
///
/// The formats are represented in EBNF with commas being literal and with
/// non-terminal symbols defined near the end of this comment. The formats
/// are:
///
/// <h4>AutoML Vision</h4>
/// <div class="ds-selector-tabs"><section><h5>Classification</h5>
///
/// One or more CSV files where each line is a single column:
///
///      GCS_FILE_PATH
///
/// The Google Cloud Storage location of an image of up to
/// 30MB in size. Supported extensions: .JPEG, .GIF, .PNG.
/// This path is treated as the ID in the batch predict output.
///
/// Sample rows:
///
///      gs://folder/image1.jpeg
///      gs://folder/image2.gif
///      gs://folder/image3.png
///
/// </section><section><h5>Object Detection</h5>
///
/// One or more CSV files where each line is a single column:
///
///      GCS_FILE_PATH
///
/// The Google Cloud Storage location of an image of up to
/// 30MB in size. Supported extensions: .JPEG, .GIF, .PNG.
/// This path is treated as the ID in the batch predict output.
///
/// Sample rows:
///
///      gs://folder/image1.jpeg
///      gs://folder/image2.gif
///      gs://folder/image3.png
///    </section>
/// </div>
///
/// <h4>AutoML Video Intelligence</h4>
/// <div class="ds-selector-tabs"><section><h5>Classification</h5>
///
/// One or more CSV files where each line is a single column:
///
///      GCS_FILE_PATH,TIME_SEGMENT_START,TIME_SEGMENT_END
///
/// `GCS_FILE_PATH` is the Google Cloud Storage location of video up to 50GB in
/// size and up to 3h in duration duration.
/// Supported extensions: .MOV, .MPEG4, .MP4, .AVI.
///
/// `TIME_SEGMENT_START` and `TIME_SEGMENT_END` must be within the
/// length of the video, and the end time must be after the start time.
///
/// Sample rows:
///
///      gs://folder/video1.mp4,10,40
///      gs://folder/video1.mp4,20,60
///      gs://folder/vid2.mov,0,inf
///
/// </section><section><h5>Object Tracking</h5>
///
/// One or more CSV files where each line is a single column:
///
///      GCS_FILE_PATH,TIME_SEGMENT_START,TIME_SEGMENT_END
///
/// `GCS_FILE_PATH` is the Google Cloud Storage location of video up to 50GB in
/// size and up to 3h in duration duration.
/// Supported extensions: .MOV, .MPEG4, .MP4, .AVI.
///
/// `TIME_SEGMENT_START` and `TIME_SEGMENT_END` must be within the
/// length of the video, and the end time must be after the start time.
///
/// Sample rows:
///
///      gs://folder/video1.mp4,10,40
///      gs://folder/video1.mp4,20,60
///      gs://folder/vid2.mov,0,inf
///    </section>
/// </div>
///
/// <h4>AutoML Natural Language</h4>
/// <div class="ds-selector-tabs"><section><h5>Classification</h5>
///
/// One or more CSV files where each line is a single column:
///
///      GCS_FILE_PATH
///
/// `GCS_FILE_PATH` is the Google Cloud Storage location of a text file.
/// Supported file extensions: .TXT, .PDF, .TIF, .TIFF
///
/// Text files can be no larger than 10MB in size.
///
/// Sample rows:
///
///      gs://folder/text1.txt
///      gs://folder/text2.pdf
///      gs://folder/text3.tif
///
/// </section><section><h5>Sentiment Analysis</h5>
/// One or more CSV files where each line is a single column:
///
///      GCS_FILE_PATH
///
/// `GCS_FILE_PATH` is the Google Cloud Storage location of a text file.
/// Supported file extensions: .TXT, .PDF, .TIF, .TIFF
///
/// Text files can be no larger than 128kB in size.
///
/// Sample rows:
///
///      gs://folder/text1.txt
///      gs://folder/text2.pdf
///      gs://folder/text3.tif
///
/// </section><section><h5>Entity Extraction</h5>
///
/// One or more JSONL (JSON Lines) files that either provide inline text or
/// documents. You can only use one format, either inline text or documents,
/// for a single call to \[AutoMl.BatchPredict\].
///
/// Each JSONL file contains a per line a proto that
/// wraps a temporary user-assigned TextSnippet ID (string up to 2000
/// characters long) called "id", a TextSnippet proto (in
/// JSON representation) and zero or more TextFeature protos. Any given
/// text snippet content must have 30,000 characters or less, and also
/// be UTF-8 NFC encoded (ASCII already is). The IDs provided should be
/// unique.
///
/// Each document JSONL file contains, per line, a proto that wraps a Document
/// proto with `input_config` set. Each document cannot exceed 2MB in size.
///
/// Supported document extensions: .PDF, .TIF, .TIFF
///
/// Each JSONL file must not exceed 100MB in size, and no more than 20
/// JSONL files may be passed.
///
/// Sample inline JSONL file (Shown with artificial line
/// breaks. Actual line breaks are denoted by "\n".):
///
///      {
///         "id": "my_first_id",
///         "text_snippet": { "content": "dog car cat"},
///         "text_features": [
///           {
///             "text_segment": {"start_offset": 4, "end_offset": 6},
///             "structural_type": PARAGRAPH,
///             "bounding_poly": {
///               "normalized_vertices": [
///                 {"x": 0.1, "y": 0.1},
///                 {"x": 0.1, "y": 0.3},
///                 {"x": 0.3, "y": 0.3},
///                 {"x": 0.3, "y": 0.1},
///               ]
///             },
///           }
///         ],
///       }\n
///       {
///         "id": "2",
///         "text_snippet": {
///           "content": "Extended sample content",
///           "mime_type": "text/plain"
///         }
///       }
///
/// Sample document JSONL file (Shown with artificial line
/// breaks. Actual line breaks are denoted by "\n".):
///
///       {
///         "document": {
///           "input_config": {
///             "gcs_source": { "input_uris": [ "gs://folder/document1.pdf" ]
///             }
///           }
///         }
///       }\n
///       {
///         "document": {
///           "input_config": {
///             "gcs_source": { "input_uris": [ "gs://folder/document2.tif" ]
///             }
///           }
///         }
///       }
///    </section>
/// </div>
///
/// <h4>AutoML Tables</h4><div class="ui-datasection-main"><section
/// class="selected">
///
/// See [Preparing your training
/// data](<https://cloud.google.com/automl-tables/docs/predict-batch>) for more
/// information.
///
/// You can use either
/// \[gcs_source][google.cloud.automl.v1.BatchPredictInputConfig.gcs_source\]
/// or
/// \[bigquery_source][BatchPredictInputConfig.bigquery_source\].
///
/// **For gcs_source:**
///
/// CSV file(s), each by itself 10GB or smaller and total size must be
/// 100GB or smaller, where first file must have a header containing
/// column names. If the first row of a subsequent file is the same as
/// the header, then it is also treated as a header. All other rows
/// contain values for the corresponding columns.
///
/// The column names must contain the model's
/// \[input_feature_column_specs'][google.cloud.automl.v1.TablesModelMetadata.input_feature_column_specs\]
/// \[display_name-s][google.cloud.automl.v1.ColumnSpec.display_name\]
/// (order doesn't matter). The columns corresponding to the model's
/// input feature column specs must contain values compatible with the
/// column spec's data types. Prediction on all the rows, i.e. the CSV
/// lines, will be attempted.
///
///
/// Sample rows from a CSV file:
/// <pre>
/// "First Name","Last Name","Dob","Addresses"
/// "John","Doe","1968-01-22","\[{"status":"current","address":"123_First_Avenue","city":"Seattle","state":"WA","zip":"11111","numberOfYears":"1"},{"status":"previous","address":"456_Main_Street","city":"Portland","state":"OR","zip":"22222","numberOfYears":"5"}\]"
/// "Jane","Doe","1980-10-16","\[{"status":"current","address":"789_Any_Avenue","city":"Albany","state":"NY","zip":"33333","numberOfYears":"2"},{"status":"previous","address":"321_Main_Street","city":"Hoboken","state":"NJ","zip":"44444","numberOfYears":"3"}\]}
/// </pre>
/// **For bigquery_source:**
///
/// The URI of a BigQuery table. The user data size of the BigQuery
/// table must be 100GB or smaller.
///
/// The column names must contain the model's
/// \[input_feature_column_specs'][google.cloud.automl.v1.TablesModelMetadata.input_feature_column_specs\]
/// \[display_name-s][google.cloud.automl.v1.ColumnSpec.display_name\]
/// (order doesn't matter). The columns corresponding to the model's
/// input feature column specs must contain values compatible with the
/// column spec's data types. Prediction on all the rows of the table
/// will be attempted.
///    </section>
/// </div>
///
/// **Input field definitions:**
///
/// `GCS_FILE_PATH`
/// : The path to a file on Google Cloud Storage. For example,
///    "gs://folder/video.avi".
///
/// `TIME_SEGMENT_START`
/// : (`TIME_OFFSET`)
///    Expresses a beginning, inclusive, of a time segment
///    within an example that has a time dimension
///    (e.g. video).
///
/// `TIME_SEGMENT_END`
/// : (`TIME_OFFSET`)
///    Expresses an end, exclusive, of a time segment within
///    n example that has a time dimension (e.g. video).
///
/// `TIME_OFFSET`
/// : A number of seconds as measured from the start of an
///    example (e.g. video). Fractions are allowed, up to a
///    microsecond precision. "inf" is allowed, and it means the end
///    of the example.
///
///   **Errors:**
///
///   If any of the provided CSV files can't be parsed or if more than certain
///   percent of CSV rows cannot be processed then the operation fails and
///   prediction does not happen. Regardless of overall success or failure the
///   per-row failures, up to a certain count cap, will be listed in
///   Operation.metadata.partial_failures.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPredictInputConfig {
    /// The source of the input.
    #[prost(oneof = "batch_predict_input_config::Source", tags = "1")]
    pub source: ::core::option::Option<batch_predict_input_config::Source>,
}
/// Nested message and enum types in `BatchPredictInputConfig`.
pub mod batch_predict_input_config {
    /// The source of the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Required. The Google Cloud Storage location for the input content.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
    }
}
/// Input configuration of a \[Document][google.cloud.automl.v1.Document\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentInputConfig {
    /// The Google Cloud Storage location of the document file. Only a single path
    /// should be given.
    ///
    /// Max supported size: 512MB.
    ///
    /// Supported extensions: .PDF.
    #[prost(message, optional, tag = "1")]
    pub gcs_source: ::core::option::Option<GcsSource>,
}
/// *  For Translation:
///          CSV file `translation.csv`, with each line in format:
///          ML_USE,GCS_FILE_PATH
///          GCS_FILE_PATH leads to a .TSV file which describes examples that have
///          given ML_USE, using the following row format per line:
///          TEXT_SNIPPET (in source language) \t TEXT_SNIPPET (in target
///          language)
///
///    *  For Tables:
///          Output depends on whether the dataset was imported from Google Cloud
///          Storage or BigQuery.
///          Google Cloud Storage case:
///            \[gcs_destination][google.cloud.automl.v1p1beta.OutputConfig.gcs_destination\]
///            must be set. Exported are CSV file(s) `tables_1.csv`,
///            `tables_2.csv`,...,`tables_N.csv` with each having as header line
///            the table's column names, and all other lines contain values for
///            the header columns.
///          BigQuery case:
///            \[bigquery_destination][google.cloud.automl.v1p1beta.OutputConfig.bigquery_destination\]
///            pointing to a BigQuery project must be set. In the given project a
///            new dataset will be created with name
///            `export_data_<automl-dataset-display-name>_<timestamp-of-export-call>`
///            where <automl-dataset-display-name> will be made
///            BigQuery-dataset-name compatible (e.g. most special characters will
///            become underscores), and timestamp will be in
///            YYYY_MM_DDThh_mm_ss_sssZ "based on ISO-8601" format. In that
///            dataset a new table called `primary_table` will be created, and
///            filled with precisely the same data as this obtained on import.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The destination of the output.
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// The destination of the output.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Required. The Google Cloud Storage location where the output is to be written to.
        /// For Image Object Detection, Text Extraction, Video Classification and
        /// Tables, in the given directory a new directory will be created with name:
        /// export_data-<dataset-display-name>-<timestamp-of-export-call> where
        /// timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format. All export
        /// output will be written into that directory.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// Output configuration for BatchPredict Action.
///
/// As destination the
/// \[gcs_destination][google.cloud.automl.v1.BatchPredictOutputConfig.gcs_destination\]
/// must be set unless specified otherwise for a domain. If gcs_destination is
/// set then in the given directory a new directory is created. Its name
/// will be
/// "prediction-<model-display-name>-<timestamp-of-prediction-call>",
/// where timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format. The contents
/// of it depends on the ML problem the predictions are made for.
///
///   *  For Image Classification:
///          In the created directory files `image_classification_1.jsonl`,
///          `image_classification_2.jsonl`,...,`image_classification_N.jsonl`
///          will be created, where N may be 1, and depends on the
///          total number of the successfully predicted images and annotations.
///          A single image will be listed only once with all its annotations,
///          and its annotations will never be split across files.
///          Each .JSONL file will contain, per line, a JSON representation of a
///          proto that wraps image's "ID" : "<id_value>" followed by a list of
///          zero or more AnnotationPayload protos (called annotations), which
///          have classification detail populated.
///          If prediction for any image failed (partially or completely), then an
///          additional `errors_1.jsonl`, `errors_2.jsonl`,..., `errors_N.jsonl`
///          files will be created (N depends on total number of failed
///          predictions). These files will have a JSON representation of a proto
///          that wraps the same "ID" : "<id_value>" but here followed by
///          exactly one
///          \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///          containing only `code` and `message`fields.
///
///   *  For Image Object Detection:
///          In the created directory files `image_object_detection_1.jsonl`,
///          `image_object_detection_2.jsonl`,...,`image_object_detection_N.jsonl`
///          will be created, where N may be 1, and depends on the
///          total number of the successfully predicted images and annotations.
///          Each .JSONL file will contain, per line, a JSON representation of a
///          proto that wraps image's "ID" : "<id_value>" followed by a list of
///          zero or more AnnotationPayload protos (called annotations), which
///          have image_object_detection detail populated. A single image will
///          be listed only once with all its annotations, and its annotations
///          will never be split across files.
///          If prediction for any image failed (partially or completely), then
///          additional `errors_1.jsonl`, `errors_2.jsonl`,..., `errors_N.jsonl`
///          files will be created (N depends on total number of failed
///          predictions). These files will have a JSON representation of a proto
///          that wraps the same "ID" : "<id_value>" but here followed by
///          exactly one
///          \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///          containing only `code` and `message`fields.
///   *  For Video Classification:
///          In the created directory a video_classification.csv file, and a .JSON
///          file per each video classification requested in the input (i.e. each
///          line in given CSV(s)), will be created.
///
///          The format of video_classification.csv is:
///          GCS_FILE_PATH,TIME_SEGMENT_START,TIME_SEGMENT_END,JSON_FILE_NAME,STATUS
///          where:
///          GCS_FILE_PATH,TIME_SEGMENT_START,TIME_SEGMENT_END = matches 1 to 1
///              the prediction input lines (i.e. video_classification.csv has
///              precisely the same number of lines as the prediction input had.)
///          JSON_FILE_NAME = Name of .JSON file in the output directory, which
///              contains prediction responses for the video time segment.
///          STATUS = "OK" if prediction completed successfully, or an error code
///              with message otherwise. If STATUS is not "OK" then the .JSON file
///              for that line may not exist or be empty.
///
///          Each .JSON file, assuming STATUS is "OK", will contain a list of
///          AnnotationPayload protos in JSON format, which are the predictions
///          for the video time segment the file is assigned to in the
///          video_classification.csv. All AnnotationPayload protos will have
///          video_classification field set, and will be sorted by
///          video_classification.type field (note that the returned types are
///          governed by `classifaction_types` parameter in
///          \[PredictService.BatchPredictRequest.params][\]).
///
///   *  For Video Object Tracking:
///          In the created directory a video_object_tracking.csv file will be
///          created, and multiple files video_object_trackinng_1.json,
///          video_object_trackinng_2.json,..., video_object_trackinng_N.json,
///          where N is the number of requests in the input (i.e. the number of
///          lines in given CSV(s)).
///
///          The format of video_object_tracking.csv is:
///          GCS_FILE_PATH,TIME_SEGMENT_START,TIME_SEGMENT_END,JSON_FILE_NAME,STATUS
///          where:
///          GCS_FILE_PATH,TIME_SEGMENT_START,TIME_SEGMENT_END = matches 1 to 1
///              the prediction input lines (i.e. video_object_tracking.csv has
///              precisely the same number of lines as the prediction input had.)
///          JSON_FILE_NAME = Name of .JSON file in the output directory, which
///              contains prediction responses for the video time segment.
///          STATUS = "OK" if prediction completed successfully, or an error
///              code with message otherwise. If STATUS is not "OK" then the .JSON
///              file for that line may not exist or be empty.
///
///          Each .JSON file, assuming STATUS is "OK", will contain a list of
///          AnnotationPayload protos in JSON format, which are the predictions
///          for each frame of the video time segment the file is assigned to in
///          video_object_tracking.csv. All AnnotationPayload protos will have
///          video_object_tracking field set.
///   *  For Text Classification:
///          In the created directory files `text_classification_1.jsonl`,
///          `text_classification_2.jsonl`,...,`text_classification_N.jsonl`
///          will be created, where N may be 1, and depends on the
///          total number of inputs and annotations found.
///
///          Each .JSONL file will contain, per line, a JSON representation of a
///          proto that wraps input text file (or document) in
///          the text snippet (or document) proto and a list of
///          zero or more AnnotationPayload protos (called annotations), which
///          have classification detail populated. A single text file (or
///          document) will be listed only once with all its annotations, and its
///          annotations will never be split across files.
///
///          If prediction for any input file (or document) failed (partially or
///          completely), then additional `errors_1.jsonl`, `errors_2.jsonl`,...,
///          `errors_N.jsonl` files will be created (N depends on total number of
///          failed predictions). These files will have a JSON representation of a
///          proto that wraps input file followed by exactly one
///          \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///          containing only `code` and `message`.
///
///   *  For Text Sentiment:
///          In the created directory files `text_sentiment_1.jsonl`,
///          `text_sentiment_2.jsonl`,...,`text_sentiment_N.jsonl`
///          will be created, where N may be 1, and depends on the
///          total number of inputs and annotations found.
///
///          Each .JSONL file will contain, per line, a JSON representation of a
///          proto that wraps input text file (or document) in
///          the text snippet (or document) proto and a list of
///          zero or more AnnotationPayload protos (called annotations), which
///          have text_sentiment detail populated. A single text file (or
///          document) will be listed only once with all its annotations, and its
///          annotations will never be split across files.
///
///          If prediction for any input file (or document) failed (partially or
///          completely), then additional `errors_1.jsonl`, `errors_2.jsonl`,...,
///          `errors_N.jsonl` files will be created (N depends on total number of
///          failed predictions). These files will have a JSON representation of a
///          proto that wraps input file followed by exactly one
///          \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///          containing only `code` and `message`.
///
///    *  For Text Extraction:
///          In the created directory files `text_extraction_1.jsonl`,
///          `text_extraction_2.jsonl`,...,`text_extraction_N.jsonl`
///          will be created, where N may be 1, and depends on the
///          total number of inputs and annotations found.
///          The contents of these .JSONL file(s) depend on whether the input
///          used inline text, or documents.
///          If input was inline, then each .JSONL file will contain, per line,
///            a JSON representation of a proto that wraps given in request text
///            snippet's "id" (if specified), followed by input text snippet,
///            and a list of zero or more
///            AnnotationPayload protos (called annotations), which have
///            text_extraction detail populated. A single text snippet will be
///            listed only once with all its annotations, and its annotations will
///            never be split across files.
///          If input used documents, then each .JSONL file will contain, per
///            line, a JSON representation of a proto that wraps given in request
///            document proto, followed by its OCR-ed representation in the form
///            of a text snippet, finally followed by a list of zero or more
///            AnnotationPayload protos (called annotations), which have
///            text_extraction detail populated and refer, via their indices, to
///            the OCR-ed text snippet. A single document (and its text snippet)
///            will be listed only once with all its annotations, and its
///            annotations will never be split across files.
///          If prediction for any text snippet failed (partially or completely),
///          then additional `errors_1.jsonl`, `errors_2.jsonl`,...,
///          `errors_N.jsonl` files will be created (N depends on total number of
///          failed predictions). These files will have a JSON representation of a
///          proto that wraps either the "id" : "<id_value>" (in case of inline)
///          or the document proto (in case of document) but here followed by
///          exactly one
///          \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///          containing only `code` and `message`.
///
///   *  For Tables:
///          Output depends on whether
///          \[gcs_destination][google.cloud.automl.v1p1beta.BatchPredictOutputConfig.gcs_destination\]
///          or
///          \[bigquery_destination][google.cloud.automl.v1p1beta.BatchPredictOutputConfig.bigquery_destination\]
///          is set (either is allowed).
///          Google Cloud Storage case:
///            In the created directory files `tables_1.csv`, `tables_2.csv`,...,
///            `tables_N.csv` will be created, where N may be 1, and depends on
///            the total number of the successfully predicted rows.
///            For all CLASSIFICATION
///            \[prediction_type-s][google.cloud.automl.v1p1beta.TablesModelMetadata.prediction_type\]:
///              Each .csv file will contain a header, listing all columns'
///              \[display_name-s][google.cloud.automl.v1p1beta.ColumnSpec.display_name\]
///              given on input followed by M target column names in the format of
///              "<\[target_column_specs][google.cloud.automl.v1p1beta.TablesModelMetadata.target_column_spec\]
///              \[display_name][google.cloud.automl.v1p1beta.ColumnSpec.display_name\]>_<target
///              value>_score" where M is the number of distinct target values,
///              i.e. number of distinct values in the target column of the table
///              used to train the model. Subsequent lines will contain the
///              respective values of successfully predicted rows, with the last,
///              i.e. the target, columns having the corresponding prediction
///              \[scores][google.cloud.automl.v1p1beta.TablesAnnotation.score\].
///            For REGRESSION and FORECASTING
///            \[prediction_type-s][google.cloud.automl.v1p1beta.TablesModelMetadata.prediction_type\]:
///              Each .csv file will contain a header, listing all columns'
///              \[display_name-s][google.cloud.automl.v1p1beta.display_name\]
///              given on input followed by the predicted target column with name
///              in the format of
///              "predicted_<\[target_column_specs][google.cloud.automl.v1p1beta.TablesModelMetadata.target_column_spec\]
///              \[display_name][google.cloud.automl.v1p1beta.ColumnSpec.display_name\]>"
///              Subsequent lines will contain the respective values of
///              successfully predicted rows, with the last, i.e. the target,
///              column having the predicted target value.
///              If prediction for any rows failed, then an additional
///              `errors_1.csv`, `errors_2.csv`,..., `errors_N.csv` will be
///              created (N depends on total number of failed rows). These files
///              will have analogous format as `tables_*.csv`, but always with a
///              single target column having
///              \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///              represented as a JSON string, and containing only `code` and
///              `message`.
///          BigQuery case:
///            \[bigquery_destination][google.cloud.automl.v1p1beta.OutputConfig.bigquery_destination\]
///            pointing to a BigQuery project must be set. In the given project a
///            new dataset will be created with name
///            `prediction_<model-display-name>_<timestamp-of-prediction-call>`
///            where <model-display-name> will be made
///            BigQuery-dataset-name compatible (e.g. most special characters will
///            become underscores), and timestamp will be in
///            YYYY_MM_DDThh_mm_ss_sssZ "based on ISO-8601" format. In the dataset
///            two tables will be created, `predictions`, and `errors`.
///            The `predictions` table's column names will be the input columns'
///            \[display_name-s][google.cloud.automl.v1p1beta.ColumnSpec.display_name\]
///            followed by the target column with name in the format of
///            "predicted_<\[target_column_specs][google.cloud.automl.v1p1beta.TablesModelMetadata.target_column_spec\]
///            \[display_name][google.cloud.automl.v1p1beta.ColumnSpec.display_name\]>"
///            The input feature columns will contain the respective values of
///            successfully predicted rows, with the target column having an
///            ARRAY of
///            \[AnnotationPayloads][google.cloud.automl.v1p1beta.AnnotationPayload\],
///            represented as STRUCT-s, containing
///            \[TablesAnnotation][google.cloud.automl.v1p1beta.TablesAnnotation\].
///            The `errors` table contains rows for which the prediction has
///            failed, it has analogous input columns while the target column name
///            is in the format of
///            "errors_<\[target_column_specs][google.cloud.automl.v1p1beta.TablesModelMetadata.target_column_spec\]
///            \[display_name][google.cloud.automl.v1p1beta.ColumnSpec.display_name\]>",
///            and as a value has
///            \[`google.rpc.Status`\](<https://github.com/googleapis/googleapis/blob/master/google/rpc/status.proto>)
///            represented as a STRUCT, and containing only `code` and `message`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPredictOutputConfig {
    /// The destination of the output.
    #[prost(oneof = "batch_predict_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<batch_predict_output_config::Destination>,
}
/// Nested message and enum types in `BatchPredictOutputConfig`.
pub mod batch_predict_output_config {
    /// The destination of the output.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Required. The Google Cloud Storage location of the directory where the output is to
        /// be written to.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// Output configuration for ModelExport Action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelExportOutputConfig {
    /// The format in which the model must be exported. The available, and default,
    /// formats depend on the problem and model type (if given problem and type
    /// combination doesn't have a format listed, it means its models are not
    /// exportable):
    ///
    /// *  For Image Classification mobile-low-latency-1, mobile-versatile-1,
    ///         mobile-high-accuracy-1:
    ///       "tflite" (default), "edgetpu_tflite", "tf_saved_model", "tf_js",
    ///       "docker".
    ///
    /// *  For Image Classification mobile-core-ml-low-latency-1,
    ///         mobile-core-ml-versatile-1, mobile-core-ml-high-accuracy-1:
    ///       "core_ml" (default).
    ///
    /// *  For Image Object Detection mobile-low-latency-1, mobile-versatile-1,
    ///         mobile-high-accuracy-1:
    ///       "tflite", "tf_saved_model", "tf_js".
    /// Formats description:
    ///
    /// * tflite - Used for Android mobile devices.
    /// * edgetpu_tflite - Used for [Edge TPU](<https://cloud.google.com/edge-tpu/>)
    ///                     devices.
    /// * tf_saved_model - A tensorflow model in SavedModel format.
    /// * tf_js - A \[TensorFlow.js\](<https://www.tensorflow.org/js>) model that can
    ///            be used in the browser and in Node.js using JavaScript.
    /// * docker - Used for Docker containers. Use the params field to customize
    ///             the container. The container is verified to work correctly on
    ///             ubuntu 16.04 operating system. See more at
    ///             [containers
    ///             quickstart](<https://cloud.google.com/vision/automl/docs/containers-gcs-quickstart>)
    /// * core_ml - Used for iOS mobile devices.
    #[prost(string, tag = "4")]
    pub model_format: ::prost::alloc::string::String,
    /// Additional model-type and format specific parameters describing the
    /// requirements for the to be exported model files, any string must be up to
    /// 25000 characters long.
    ///
    ///   * For `docker` format:
    ///      `cpu_architecture` - (string) "x86_64" (default).
    ///      `gpu_architecture` - (string) "none" (default), "nvidia".
    #[prost(btree_map = "string, string", tag = "2")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The destination of the output.
    #[prost(oneof = "model_export_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<model_export_output_config::Destination>,
}
/// Nested message and enum types in `ModelExportOutputConfig`.
pub mod model_export_output_config {
    /// The destination of the output.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Required. The Google Cloud Storage location where the model is to be written to.
        /// This location may only be set for the following model formats:
        ///    "tflite", "edgetpu_tflite", "tf_saved_model", "tf_js", "core_ml".
        ///
        ///   Under the directory given as the destination a new one with name
        ///   "model-export-<model-display-name>-<timestamp-of-export-call>",
        ///   where timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format,
        ///   will be created. Inside the model and any of its supporting files
        ///   will be written.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// The Google Cloud Storage location for the input content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Google Cloud Storage URIs to input files, up to 2000
    /// characters long. Accepted forms:
    /// * Full object path, e.g. gs://bucket/directory/object.csv
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The Google Cloud Storage location where the output is to be written to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required. Google Cloud Storage URI to output directory, up to 2000
    /// characters long.
    /// Accepted forms:
    /// * Prefix path: gs://bucket/directory
    /// The requesting user must have write permission to the bucket.
    /// The directory is created if it doesn't exist.
    #[prost(string, tag = "1")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// A contiguous part of a text (string), assuming it has an UTF-8 NFC encoding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSegment {
    /// Output only. The content of the TextSegment.
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    /// Required. Zero-based character index of the first character of the text
    /// segment (counting characters from the beginning of the text).
    #[prost(int64, tag = "1")]
    pub start_offset: i64,
    /// Required. Zero-based character index of the first character past the end of
    /// the text segment (counting character from the beginning of the text).
    /// The character at the end_offset is NOT included in the text segment.
    #[prost(int64, tag = "2")]
    pub end_offset: i64,
}
/// A representation of an image.
/// Only images up to 30MB in size are supported.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Output only. HTTP URI to the thumbnail image.
    #[prost(string, tag = "4")]
    pub thumbnail_uri: ::prost::alloc::string::String,
    /// Input only. The data representing the image.
    /// For Predict calls \[image_bytes][google.cloud.automl.v1.Image.image_bytes\] must be set .
    #[prost(oneof = "image::Data", tags = "1")]
    pub data: ::core::option::Option<image::Data>,
}
/// Nested message and enum types in `Image`.
pub mod image {
    /// Input only. The data representing the image.
    /// For Predict calls \[image_bytes][google.cloud.automl.v1.Image.image_bytes\] must be set .
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// Image content represented as a stream of bytes.
        /// Note: As with all `bytes` fields, protobuffers use a pure binary
        /// representation, whereas JSON representations use base64.
        #[prost(bytes, tag = "1")]
        ImageBytes(::prost::bytes::Bytes),
    }
}
/// A representation of a text snippet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSnippet {
    /// Required. The content of the text snippet as a string. Up to 250000
    /// characters long.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// Optional. The format of \[content][google.cloud.automl.v1.TextSnippet.content\]. Currently the only two allowed
    /// values are "text/html" and "text/plain". If left blank, the format is
    /// automatically determined from the type of the uploaded \[content][google.cloud.automl.v1.TextSnippet.content\].
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// Output only. HTTP URI where you can download the content.
    #[prost(string, tag = "4")]
    pub content_uri: ::prost::alloc::string::String,
}
/// Message that describes dimension of a document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentDimensions {
    /// Unit of the dimension.
    #[prost(enumeration = "document_dimensions::DocumentDimensionUnit", tag = "1")]
    pub unit: i32,
    /// Width value of the document, works together with the unit.
    #[prost(float, tag = "2")]
    pub width: f32,
    /// Height value of the document, works together with the unit.
    #[prost(float, tag = "3")]
    pub height: f32,
}
/// Nested message and enum types in `DocumentDimensions`.
pub mod document_dimensions {
    /// Unit of the document dimension.
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
    pub enum DocumentDimensionUnit {
        /// Should not be used.
        Unspecified = 0,
        /// Document dimension is measured in inches.
        Inch = 1,
        /// Document dimension is measured in centimeters.
        Centimeter = 2,
        /// Document dimension is measured in points. 72 points = 1 inch.
        Point = 3,
    }
    impl DocumentDimensionUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DocumentDimensionUnit::Unspecified => {
                    "DOCUMENT_DIMENSION_UNIT_UNSPECIFIED"
                }
                DocumentDimensionUnit::Inch => "INCH",
                DocumentDimensionUnit::Centimeter => "CENTIMETER",
                DocumentDimensionUnit::Point => "POINT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DOCUMENT_DIMENSION_UNIT_UNSPECIFIED" => Some(Self::Unspecified),
                "INCH" => Some(Self::Inch),
                "CENTIMETER" => Some(Self::Centimeter),
                "POINT" => Some(Self::Point),
                _ => None,
            }
        }
    }
}
/// A structured text document e.g. a PDF.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// An input config specifying the content of the document.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<DocumentInputConfig>,
    /// The plain text version of this document.
    #[prost(message, optional, tag = "2")]
    pub document_text: ::core::option::Option<TextSnippet>,
    /// Describes the layout of the document.
    /// Sorted by \[page_number][\].
    #[prost(message, repeated, tag = "3")]
    pub layout: ::prost::alloc::vec::Vec<document::Layout>,
    /// The dimensions of the page in the document.
    #[prost(message, optional, tag = "4")]
    pub document_dimensions: ::core::option::Option<DocumentDimensions>,
    /// Number of pages in the document.
    #[prost(int32, tag = "5")]
    pub page_count: i32,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// Describes the layout information of a \[text_segment][google.cloud.automl.v1.Document.Layout.text_segment\] in the document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Layout {
        /// Text Segment that represents a segment in
        /// \[document_text][google.cloud.automl.v1p1beta.Document.document_text\].
        #[prost(message, optional, tag = "1")]
        pub text_segment: ::core::option::Option<super::TextSegment>,
        /// Page number of the \[text_segment][google.cloud.automl.v1.Document.Layout.text_segment\] in the original document, starts
        /// from 1.
        #[prost(int32, tag = "2")]
        pub page_number: i32,
        /// The position of the \[text_segment][google.cloud.automl.v1.Document.Layout.text_segment\] in the page.
        /// Contains exactly 4
        /// \[normalized_vertices][google.cloud.automl.v1p1beta.BoundingPoly.normalized_vertices\]
        /// and they are connected by edges in the order provided, which will
        /// represent a rectangle parallel to the frame. The
        /// \[NormalizedVertex-s][google.cloud.automl.v1p1beta.NormalizedVertex\] are
        /// relative to the page.
        /// Coordinates are based on top-left as point (0,0).
        #[prost(message, optional, tag = "3")]
        pub bounding_poly: ::core::option::Option<super::BoundingPoly>,
        /// The type of the \[text_segment][google.cloud.automl.v1.Document.Layout.text_segment\] in document.
        #[prost(enumeration = "layout::TextSegmentType", tag = "4")]
        pub text_segment_type: i32,
    }
    /// Nested message and enum types in `Layout`.
    pub mod layout {
        /// The type of TextSegment in the context of the original document.
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
        pub enum TextSegmentType {
            /// Should not be used.
            Unspecified = 0,
            /// The text segment is a token. e.g. word.
            Token = 1,
            /// The text segment is a paragraph.
            Paragraph = 2,
            /// The text segment is a form field.
            FormField = 3,
            /// The text segment is the name part of a form field. It will be treated
            /// as child of another FORM_FIELD TextSegment if its span is subspan of
            /// another TextSegment with type FORM_FIELD.
            FormFieldName = 4,
            /// The text segment is the text content part of a form field. It will be
            /// treated as child of another FORM_FIELD TextSegment if its span is
            /// subspan of another TextSegment with type FORM_FIELD.
            FormFieldContents = 5,
            /// The text segment is a whole table, including headers, and all rows.
            Table = 6,
            /// The text segment is a table's headers. It will be treated as child of
            /// another TABLE TextSegment if its span is subspan of another TextSegment
            /// with type TABLE.
            TableHeader = 7,
            /// The text segment is a row in table. It will be treated as child of
            /// another TABLE TextSegment if its span is subspan of another TextSegment
            /// with type TABLE.
            TableRow = 8,
            /// The text segment is a cell in table. It will be treated as child of
            /// another TABLE_ROW TextSegment if its span is subspan of another
            /// TextSegment with type TABLE_ROW.
            TableCell = 9,
        }
        impl TextSegmentType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TextSegmentType::Unspecified => "TEXT_SEGMENT_TYPE_UNSPECIFIED",
                    TextSegmentType::Token => "TOKEN",
                    TextSegmentType::Paragraph => "PARAGRAPH",
                    TextSegmentType::FormField => "FORM_FIELD",
                    TextSegmentType::FormFieldName => "FORM_FIELD_NAME",
                    TextSegmentType::FormFieldContents => "FORM_FIELD_CONTENTS",
                    TextSegmentType::Table => "TABLE",
                    TextSegmentType::TableHeader => "TABLE_HEADER",
                    TextSegmentType::TableRow => "TABLE_ROW",
                    TextSegmentType::TableCell => "TABLE_CELL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TEXT_SEGMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "TOKEN" => Some(Self::Token),
                    "PARAGRAPH" => Some(Self::Paragraph),
                    "FORM_FIELD" => Some(Self::FormField),
                    "FORM_FIELD_NAME" => Some(Self::FormFieldName),
                    "FORM_FIELD_CONTENTS" => Some(Self::FormFieldContents),
                    "TABLE" => Some(Self::Table),
                    "TABLE_HEADER" => Some(Self::TableHeader),
                    "TABLE_ROW" => Some(Self::TableRow),
                    "TABLE_CELL" => Some(Self::TableCell),
                    _ => None,
                }
            }
        }
    }
}
/// Example data used for training or prediction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExamplePayload {
    /// Required. The example data.
    #[prost(oneof = "example_payload::Payload", tags = "1, 2, 4")]
    pub payload: ::core::option::Option<example_payload::Payload>,
}
/// Nested message and enum types in `ExamplePayload`.
pub mod example_payload {
    /// Required. The example data.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Example image.
        #[prost(message, tag = "1")]
        Image(super::Image),
        /// Example text.
        #[prost(message, tag = "2")]
        TextSnippet(super::TextSnippet),
        /// Example document.
        #[prost(message, tag = "4")]
        Document(super::Document),
    }
}
/// Dataset metadata that is specific to translation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationDatasetMetadata {
    /// Required. The BCP-47 language code of the source language.
    #[prost(string, tag = "1")]
    pub source_language_code: ::prost::alloc::string::String,
    /// Required. The BCP-47 language code of the target language.
    #[prost(string, tag = "2")]
    pub target_language_code: ::prost::alloc::string::String,
}
/// Evaluation metrics for the dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationEvaluationMetrics {
    /// Output only. BLEU score.
    #[prost(double, tag = "1")]
    pub bleu_score: f64,
    /// Output only. BLEU score for base model.
    #[prost(double, tag = "2")]
    pub base_bleu_score: f64,
}
/// Model metadata that is specific to translation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationModelMetadata {
    /// The resource name of the model to use as a baseline to train the custom
    /// model. If unset, we use the default base model provided by Google
    /// Translate. Format:
    /// `projects/{project_id}/locations/{location_id}/models/{model_id}`
    #[prost(string, tag = "1")]
    pub base_model: ::prost::alloc::string::String,
    /// Output only. Inferred from the dataset.
    /// The source language (The BCP-47 language code) that is used for training.
    #[prost(string, tag = "2")]
    pub source_language_code: ::prost::alloc::string::String,
    /// Output only. The target language (The BCP-47 language code) that is used
    /// for training.
    #[prost(string, tag = "3")]
    pub target_language_code: ::prost::alloc::string::String,
}
/// Annotation details specific to translation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationAnnotation {
    /// Output only . The translated content.
    #[prost(message, optional, tag = "1")]
    pub translated_content: ::core::option::Option<TextSnippet>,
}
/// A workspace for solving a single, particular machine learning (ML) problem.
/// A workspace contains examples that may be annotated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Output only. The resource name of the dataset.
    /// Form: `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the dataset to show in the interface. The name can be
    /// up to 32 characters long and can consist only of ASCII Latin letters A-Z
    /// and a-z, underscores
    /// (_), and ASCII digits 0-9.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// User-provided description of the dataset. The description can be up to
    /// 25000 characters long.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The number of examples in the dataset.
    #[prost(int32, tag = "21")]
    pub example_count: i32,
    /// Output only. Timestamp when this dataset was created.
    #[prost(message, optional, tag = "14")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "17")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. The labels with user-defined metadata to organize your dataset.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// Label values are optional. Label keys must start with a letter.
    ///
    /// See <https://goo.gl/xmQnxf> for more information on and examples of labels.
    #[prost(btree_map = "string, string", tag = "39")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required.
    /// The dataset metadata that is specific to the problem type.
    #[prost(oneof = "dataset::DatasetMetadata", tags = "23, 24, 25, 26, 28, 30")]
    pub dataset_metadata: ::core::option::Option<dataset::DatasetMetadata>,
}
/// Nested message and enum types in `Dataset`.
pub mod dataset {
    /// Required.
    /// The dataset metadata that is specific to the problem type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatasetMetadata {
        /// Metadata for a dataset used for translation.
        #[prost(message, tag = "23")]
        TranslationDatasetMetadata(super::TranslationDatasetMetadata),
        /// Metadata for a dataset used for image classification.
        #[prost(message, tag = "24")]
        ImageClassificationDatasetMetadata(super::ImageClassificationDatasetMetadata),
        /// Metadata for a dataset used for text classification.
        #[prost(message, tag = "25")]
        TextClassificationDatasetMetadata(super::TextClassificationDatasetMetadata),
        /// Metadata for a dataset used for image object detection.
        #[prost(message, tag = "26")]
        ImageObjectDetectionDatasetMetadata(super::ImageObjectDetectionDatasetMetadata),
        /// Metadata for a dataset used for text extraction.
        #[prost(message, tag = "28")]
        TextExtractionDatasetMetadata(super::TextExtractionDatasetMetadata),
        /// Metadata for a dataset used for text sentiment.
        #[prost(message, tag = "30")]
        TextSentimentDatasetMetadata(super::TextSentimentDatasetMetadata),
    }
}
/// Annotation details for image object detection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionAnnotation {
    /// Output only. The rectangle representing the object location.
    #[prost(message, optional, tag = "1")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
    /// Output only. The confidence that this annotation is positive for the parent example,
    /// value in [0, 1], higher means higher positivity confidence.
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// Bounding box matching model metrics for a single intersection-over-union
/// threshold and multiple label match confidence thresholds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBoxMetricsEntry {
    /// Output only. The intersection-over-union threshold value used to compute
    /// this metrics entry.
    #[prost(float, tag = "1")]
    pub iou_threshold: f32,
    /// Output only. The mean average precision, most often close to au_prc.
    #[prost(float, tag = "2")]
    pub mean_average_precision: f32,
    /// Output only. Metrics for each label-match confidence_threshold from
    /// 0.05,0.10,...,0.95,0.96,0.97,0.98,0.99. Precision-recall curve is
    /// derived from them.
    #[prost(message, repeated, tag = "3")]
    pub confidence_metrics_entries: ::prost::alloc::vec::Vec<
        bounding_box_metrics_entry::ConfidenceMetricsEntry,
    >,
}
/// Nested message and enum types in `BoundingBoxMetricsEntry`.
pub mod bounding_box_metrics_entry {
    /// Metrics for a single confidence threshold.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfidenceMetricsEntry {
        /// Output only. The confidence threshold value used to compute the metrics.
        #[prost(float, tag = "1")]
        pub confidence_threshold: f32,
        /// Output only. Recall under the given confidence threshold.
        #[prost(float, tag = "2")]
        pub recall: f32,
        /// Output only. Precision under the given confidence threshold.
        #[prost(float, tag = "3")]
        pub precision: f32,
        /// Output only. The harmonic mean of recall and precision.
        #[prost(float, tag = "4")]
        pub f1_score: f32,
    }
}
/// Model evaluation metrics for image object detection problems.
/// Evaluates prediction quality of labeled bounding boxes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionEvaluationMetrics {
    /// Output only. The total number of bounding boxes (i.e. summed over all
    /// images) the ground truth used to create this evaluation had.
    #[prost(int32, tag = "1")]
    pub evaluated_bounding_box_count: i32,
    /// Output only. The bounding boxes match metrics for each
    /// Intersection-over-union threshold 0.05,0.10,...,0.95,0.96,0.97,0.98,0.99
    /// and each label confidence threshold 0.05,0.10,...,0.95,0.96,0.97,0.98,0.99
    /// pair.
    #[prost(message, repeated, tag = "2")]
    pub bounding_box_metrics_entries: ::prost::alloc::vec::Vec<BoundingBoxMetricsEntry>,
    /// Output only. The single metric for bounding boxes evaluation:
    /// the mean_average_precision averaged over all bounding_box_metrics_entries.
    #[prost(float, tag = "3")]
    pub bounding_box_mean_average_precision: f32,
}
/// Annotation for identifying spans of text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionAnnotation {
    /// Output only. A confidence estimate between 0.0 and 1.0. A higher value
    /// means greater confidence in correctness of the annotation.
    #[prost(float, tag = "1")]
    pub score: f32,
    /// Required. Text extraction annotations can either be a text segment or a
    /// text relation.
    #[prost(oneof = "text_extraction_annotation::Annotation", tags = "3")]
    pub annotation: ::core::option::Option<text_extraction_annotation::Annotation>,
}
/// Nested message and enum types in `TextExtractionAnnotation`.
pub mod text_extraction_annotation {
    /// Required. Text extraction annotations can either be a text segment or a
    /// text relation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Annotation {
        /// An entity annotation will set this, which is the part of the original
        /// text to which the annotation pertains.
        #[prost(message, tag = "3")]
        TextSegment(super::TextSegment),
    }
}
/// Model evaluation metrics for text extraction problems.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionEvaluationMetrics {
    /// Output only. The Area under precision recall curve metric.
    #[prost(float, tag = "1")]
    pub au_prc: f32,
    /// Output only. Metrics that have confidence thresholds.
    /// Precision-recall curve can be derived from it.
    #[prost(message, repeated, tag = "2")]
    pub confidence_metrics_entries: ::prost::alloc::vec::Vec<
        text_extraction_evaluation_metrics::ConfidenceMetricsEntry,
    >,
}
/// Nested message and enum types in `TextExtractionEvaluationMetrics`.
pub mod text_extraction_evaluation_metrics {
    /// Metrics for a single confidence threshold.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfidenceMetricsEntry {
        /// Output only. The confidence threshold value used to compute the metrics.
        /// Only annotations with score of at least this threshold are considered to
        /// be ones the model would return.
        #[prost(float, tag = "1")]
        pub confidence_threshold: f32,
        /// Output only. Recall under the given confidence threshold.
        #[prost(float, tag = "3")]
        pub recall: f32,
        /// Output only. Precision under the given confidence threshold.
        #[prost(float, tag = "4")]
        pub precision: f32,
        /// Output only. The harmonic mean of recall and precision.
        #[prost(float, tag = "5")]
        pub f1_score: f32,
    }
}
/// Contains annotation details specific to text sentiment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentAnnotation {
    /// Output only. The sentiment with the semantic, as given to the
    /// \[AutoMl.ImportData][google.cloud.automl.v1.AutoMl.ImportData\] when populating the dataset from which the model used
    /// for the prediction had been trained.
    /// The sentiment values are between 0 and
    /// Dataset.text_sentiment_dataset_metadata.sentiment_max (inclusive),
    /// with higher value meaning more positive sentiment. They are completely
    /// relative, i.e. 0 means least positive sentiment and sentiment_max means
    /// the most positive from the sentiments present in the train data. Therefore
    ///   e.g. if train data had only negative sentiment, then sentiment_max, would
    /// be still negative (although least negative).
    /// The sentiment shouldn't be confused with "score" or "magnitude"
    /// from the previous Natural Language Sentiment Analysis API.
    #[prost(int32, tag = "1")]
    pub sentiment: i32,
}
/// Model evaluation metrics for text sentiment problems.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentEvaluationMetrics {
    /// Output only. Precision.
    #[prost(float, tag = "1")]
    pub precision: f32,
    /// Output only. Recall.
    #[prost(float, tag = "2")]
    pub recall: f32,
    /// Output only. The harmonic mean of recall and precision.
    #[prost(float, tag = "3")]
    pub f1_score: f32,
    /// Output only. Mean absolute error. Only set for the overall model
    /// evaluation, not for evaluation of a single annotation spec.
    #[prost(float, tag = "4")]
    pub mean_absolute_error: f32,
    /// Output only. Mean squared error. Only set for the overall model
    /// evaluation, not for evaluation of a single annotation spec.
    #[prost(float, tag = "5")]
    pub mean_squared_error: f32,
    /// Output only. Linear weighted kappa. Only set for the overall model
    /// evaluation, not for evaluation of a single annotation spec.
    #[prost(float, tag = "6")]
    pub linear_kappa: f32,
    /// Output only. Quadratic weighted kappa. Only set for the overall model
    /// evaluation, not for evaluation of a single annotation spec.
    #[prost(float, tag = "7")]
    pub quadratic_kappa: f32,
    /// Output only. Confusion matrix of the evaluation.
    /// Only set for the overall model evaluation, not for evaluation of a single
    /// annotation spec.
    #[prost(message, optional, tag = "8")]
    pub confusion_matrix: ::core::option::Option<
        classification_evaluation_metrics::ConfusionMatrix,
    >,
}
/// Contains annotation information that is relevant to AutoML.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationPayload {
    /// Output only . The resource ID of the annotation spec that
    /// this annotation pertains to. The annotation spec comes from either an
    /// ancestor dataset, or the dataset that was used to train the model in use.
    #[prost(string, tag = "1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// Output only. The value of
    /// \[display_name][google.cloud.automl.v1.AnnotationSpec.display_name\]
    /// when the model was trained. Because this field returns a value at model
    /// training time, for different models trained using the same dataset, the
    /// returned value could be different as model owner could update the
    /// `display_name` between any two model training.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only . Additional information about the annotation
    /// specific to the AutoML domain.
    #[prost(oneof = "annotation_payload::Detail", tags = "2, 3, 4, 6, 7")]
    pub detail: ::core::option::Option<annotation_payload::Detail>,
}
/// Nested message and enum types in `AnnotationPayload`.
pub mod annotation_payload {
    /// Output only . Additional information about the annotation
    /// specific to the AutoML domain.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Detail {
        /// Annotation details for translation.
        #[prost(message, tag = "2")]
        Translation(super::TranslationAnnotation),
        /// Annotation details for content or image classification.
        #[prost(message, tag = "3")]
        Classification(super::ClassificationAnnotation),
        /// Annotation details for image object detection.
        #[prost(message, tag = "4")]
        ImageObjectDetection(super::ImageObjectDetectionAnnotation),
        /// Annotation details for text extraction.
        #[prost(message, tag = "6")]
        TextExtraction(super::TextExtractionAnnotation),
        /// Annotation details for text sentiment.
        #[prost(message, tag = "7")]
        TextSentiment(super::TextSentimentAnnotation),
    }
}
/// Request message for \[PredictionService.Predict][google.cloud.automl.v1.PredictionService.Predict\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. Name of the model requested to serve the prediction.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Payload to perform a prediction on. The payload must match the
    /// problem type that the model was trained to solve.
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<ExamplePayload>,
    /// Additional domain-specific parameters, any string must be up to 25000
    /// characters long.
    ///
    /// AutoML Vision Classification
    ///
    /// `score_threshold`
    /// : (float) A value from 0.0 to 1.0. When the model
    ///    makes predictions for an image, it will only produce results that have
    ///    at least this confidence score. The default is 0.5.
    ///
    /// AutoML Vision Object Detection
    ///
    /// `score_threshold`
    /// : (float) When Model detects objects on the image,
    ///    it will only produce bounding boxes which have at least this
    ///    confidence score. Value in 0 to 1 range, default is 0.5.
    ///
    /// `max_bounding_box_count`
    /// : (int64) The maximum number of bounding
    ///    boxes returned. The default is 100. The
    ///    number of returned bounding boxes might be limited by the server.
    ///
    /// AutoML Tables
    ///
    /// `feature_importance`
    /// : (boolean) Whether
    /// \[feature_importance][google.cloud.automl.v1.TablesModelColumnInfo.feature_importance\]
    ///    is populated in the returned list of
    ///    \[TablesAnnotation][google.cloud.automl.v1.TablesAnnotation\]
    ///    objects. The default is false.
    #[prost(btree_map = "string, string", tag = "3")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Response message for \[PredictionService.Predict][google.cloud.automl.v1.PredictionService.Predict\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// Prediction result.
    /// AutoML Translation and AutoML Natural Language Sentiment Analysis
    /// return precisely one payload.
    #[prost(message, repeated, tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<AnnotationPayload>,
    /// The preprocessed example that AutoML actually makes prediction on.
    /// Empty if AutoML does not preprocess the input example.
    ///
    /// For AutoML Natural Language (Classification, Entity Extraction, and
    /// Sentiment Analysis), if the input is a document, the recognized text is
    /// returned in the
    /// \[document_text][google.cloud.automl.v1.Document.document_text\]
    /// property.
    #[prost(message, optional, tag = "3")]
    pub preprocessed_input: ::core::option::Option<ExamplePayload>,
    /// Additional domain-specific prediction response metadata.
    ///
    /// AutoML Vision Object Detection
    ///
    /// `max_bounding_box_count`
    /// : (int64) The maximum number of bounding boxes to return per image.
    ///
    /// AutoML Natural Language Sentiment Analysis
    ///
    /// `sentiment_score`
    /// : (float, deprecated) A value between -1 and 1,
    ///    -1 maps to least positive sentiment, while 1 maps to the most positive
    ///    one and the higher the score, the more positive the sentiment in the
    ///    document is. Yet these values are relative to the training data, so
    ///    e.g. if all data was positive then -1 is also positive (though
    ///    the least).
    ///    `sentiment_score` is not the same as "score" and "magnitude"
    ///    from Sentiment Analysis in the Natural Language API.
    #[prost(btree_map = "string, string", tag = "2")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Request message for \[PredictionService.BatchPredict][google.cloud.automl.v1.PredictionService.BatchPredict\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPredictRequest {
    /// Required. Name of the model requested to serve the batch prediction.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The input configuration for batch prediction.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::core::option::Option<BatchPredictInputConfig>,
    /// Required. The Configuration specifying where output predictions should
    /// be written.
    #[prost(message, optional, tag = "4")]
    pub output_config: ::core::option::Option<BatchPredictOutputConfig>,
    /// Additional domain-specific parameters for the predictions, any string must
    /// be up to 25000 characters long.
    ///
    /// AutoML Natural Language Classification
    ///
    /// `score_threshold`
    /// : (float) A value from 0.0 to 1.0. When the model
    ///    makes predictions for a text snippet, it will only produce results
    ///    that have at least this confidence score. The default is 0.5.
    ///
    ///
    /// AutoML Vision Classification
    ///
    /// `score_threshold`
    /// : (float) A value from 0.0 to 1.0. When the model
    ///    makes predictions for an image, it will only produce results that
    ///    have at least this confidence score. The default is 0.5.
    ///
    /// AutoML Vision Object Detection
    ///
    /// `score_threshold`
    /// : (float) When Model detects objects on the image,
    ///    it will only produce bounding boxes which have at least this
    ///    confidence score. Value in 0 to 1 range, default is 0.5.
    ///
    /// `max_bounding_box_count`
    /// : (int64) The maximum number of bounding
    ///    boxes returned per image. The default is 100, the
    ///    number of bounding boxes returned might be limited by the server.
    /// AutoML Video Intelligence Classification
    ///
    /// `score_threshold`
    /// : (float) A value from 0.0 to 1.0. When the model
    ///    makes predictions for a video, it will only produce results that
    ///    have at least this confidence score. The default is 0.5.
    ///
    /// `segment_classification`
    /// : (boolean) Set to true to request
    ///    segment-level classification. AutoML Video Intelligence returns
    ///    labels and their confidence scores for the entire segment of the
    ///    video that user specified in the request configuration.
    ///    The default is true.
    ///
    /// `shot_classification`
    /// : (boolean) Set to true to request shot-level
    ///    classification. AutoML Video Intelligence determines the boundaries
    ///    for each camera shot in the entire segment of the video that user
    ///    specified in the request configuration. AutoML Video Intelligence
    ///    then returns labels and their confidence scores for each detected
    ///    shot, along with the start and end time of the shot.
    ///    The default is false.
    ///
    ///    WARNING: Model evaluation is not done for this classification type,
    ///    the quality of it depends on training data, but there are no metrics
    ///    provided to describe that quality.
    ///
    /// `1s_interval_classification`
    /// : (boolean) Set to true to request
    ///    classification for a video at one-second intervals. AutoML Video
    ///    Intelligence returns labels and their confidence scores for each
    ///    second of the entire segment of the video that user specified in the
    ///    request configuration. The default is false.
    ///
    ///    WARNING: Model evaluation is not done for this classification
    ///    type, the quality of it depends on training data, but there are no
    ///    metrics provided to describe that quality.
    ///
    /// AutoML Video Intelligence Object Tracking
    ///
    /// `score_threshold`
    /// : (float) When Model detects objects on video frames,
    ///    it will only produce bounding boxes which have at least this
    ///    confidence score. Value in 0 to 1 range, default is 0.5.
    ///
    /// `max_bounding_box_count`
    /// : (int64) The maximum number of bounding
    ///    boxes returned per image. The default is 100, the
    ///    number of bounding boxes returned might be limited by the server.
    ///
    /// `min_bounding_box_size`
    /// : (float) Only bounding boxes with shortest edge
    ///    at least that long as a relative value of video frame size are
    ///    returned. Value in 0 to 1 range. Default is 0.
    ///
    #[prost(btree_map = "string, string", tag = "5")]
    pub params: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Result of the Batch Predict. This message is returned in
/// \[response][google.longrunning.Operation.response\] of the operation returned
/// by the \[PredictionService.BatchPredict][google.cloud.automl.v1.PredictionService.BatchPredict\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPredictResult {
    /// Additional domain-specific prediction response metadata.
    ///
    /// AutoML Vision Object Detection
    ///
    /// `max_bounding_box_count`
    /// : (int64) The maximum number of bounding boxes returned per image.
    ///
    /// AutoML Video Intelligence Object Tracking
    ///
    /// `max_bounding_box_count`
    /// : (int64) The maximum number of bounding boxes returned per frame.
    #[prost(btree_map = "string, string", tag = "1")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Generated client implementations.
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// AutoML Prediction API.
    ///
    /// On any input that is documented to expect a string parameter in
    /// snake_case or dash-case, either of those cases is accepted.
    #[derive(Debug, Clone)]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PredictionServiceClient<T>
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
        ) -> PredictionServiceClient<InterceptedService<T, F>>
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
            PredictionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Perform an online prediction. The prediction result is directly
        /// returned in the response.
        /// Available for following ML scenarios, and their expected request payloads:
        ///
        /// AutoML Vision Classification
        ///
        /// * An image in .JPEG, .GIF or .PNG format, image_bytes up to 30MB.
        ///
        /// AutoML Vision Object Detection
        ///
        /// * An image in .JPEG, .GIF or .PNG format, image_bytes up to 30MB.
        ///
        /// AutoML Natural Language Classification
        ///
        /// * A TextSnippet up to 60,000 characters, UTF-8 encoded or a document in
        /// .PDF, .TIF or .TIFF format with size upto 2MB.
        ///
        /// AutoML Natural Language Entity Extraction
        ///
        /// * A TextSnippet up to 10,000 characters, UTF-8 NFC encoded or a document
        ///  in .PDF, .TIF or .TIFF format with size upto 20MB.
        ///
        /// AutoML Natural Language Sentiment Analysis
        ///
        /// * A TextSnippet up to 60,000 characters, UTF-8 encoded or a document in
        /// .PDF, .TIF or .TIFF format with size upto 2MB.
        ///
        /// AutoML Translation
        ///
        /// * A TextSnippet up to 25,000 characters, UTF-8 encoded.
        ///
        /// AutoML Tables
        ///
        /// * A row with column values matching
        ///   the columns of the model, up to 5MB. Not available for FORECASTING
        ///   `prediction_type`.
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PredictResponse>,
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
                "/google.cloud.automl.v1.PredictionService/Predict",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.automl.v1.PredictionService",
                        "Predict",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Perform a batch prediction. Unlike the online [Predict][google.cloud.automl.v1.PredictionService.Predict], batch
        /// prediction result won't be immediately available in the response. Instead,
        /// a long running operation object is returned. User can poll the operation
        /// result via [GetOperation][google.longrunning.Operations.GetOperation]
        /// method. Once the operation is done, [BatchPredictResult][google.cloud.automl.v1.BatchPredictResult] is returned in
        /// the [response][google.longrunning.Operation.response] field.
        /// Available for following ML scenarios:
        ///
        /// * AutoML Vision Classification
        /// * AutoML Vision Object Detection
        /// * AutoML Video Intelligence Classification
        /// * AutoML Video Intelligence Object Tracking * AutoML Natural Language Classification
        /// * AutoML Natural Language Entity Extraction
        /// * AutoML Natural Language Sentiment Analysis
        /// * AutoML Tables
        pub async fn batch_predict(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchPredictRequest>,
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
                "/google.cloud.automl.v1.PredictionService/BatchPredict",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.automl.v1.PredictionService",
                        "BatchPredict",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A definition of an annotation spec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSpec {
    /// Output only. Resource name of the annotation spec.
    /// Form:
    /// 'projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/annotationSpecs/{annotation_spec_id}'
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the annotation spec to show in the interface. The name can be
    /// up to 32 characters long and must match the regexp `\[a-zA-Z0-9_\]+`.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The number of examples in the parent dataset
    /// labeled by the annotation spec.
    #[prost(int32, tag = "9")]
    pub example_count: i32,
}
/// API proto representing a trained machine learning model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Output only. Resource name of the model.
    /// Format: `projects/{project_id}/locations/{location_id}/models/{model_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The name of the model to show in the interface. The name can be
    /// up to 32 characters long and can consist only of ASCII Latin letters A-Z
    /// and a-z, underscores
    /// (_), and ASCII digits 0-9. It must start with a letter.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The resource ID of the dataset used to create the model. The dataset must
    /// come from the same ancestor project and location.
    #[prost(string, tag = "3")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Output only. Timestamp when the model training finished  and can be used for prediction.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this model was last updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Deployment state of the model. A model can only serve
    /// prediction requests after it gets deployed.
    #[prost(enumeration = "model::DeploymentState", tag = "8")]
    pub deployment_state: i32,
    /// Used to perform a consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. The labels with user-defined metadata to organize your model.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// Label values are optional. Label keys must start with a letter.
    ///
    /// See <https://goo.gl/xmQnxf> for more information on and examples of labels.
    #[prost(btree_map = "string, string", tag = "34")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required.
    /// The model metadata that is specific to the problem type.
    /// Must match the metadata type of the dataset used to train the model.
    #[prost(oneof = "model::ModelMetadata", tags = "15, 13, 14, 20, 19, 22")]
    pub model_metadata: ::core::option::Option<model::ModelMetadata>,
}
/// Nested message and enum types in `Model`.
pub mod model {
    /// Deployment state of the model.
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
    pub enum DeploymentState {
        /// Should not be used, an un-set enum has this value by default.
        Unspecified = 0,
        /// Model is deployed.
        Deployed = 1,
        /// Model is not deployed.
        Undeployed = 2,
    }
    impl DeploymentState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeploymentState::Unspecified => "DEPLOYMENT_STATE_UNSPECIFIED",
                DeploymentState::Deployed => "DEPLOYED",
                DeploymentState::Undeployed => "UNDEPLOYED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPLOYMENT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEPLOYED" => Some(Self::Deployed),
                "UNDEPLOYED" => Some(Self::Undeployed),
                _ => None,
            }
        }
    }
    /// Required.
    /// The model metadata that is specific to the problem type.
    /// Must match the metadata type of the dataset used to train the model.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModelMetadata {
        /// Metadata for translation models.
        #[prost(message, tag = "15")]
        TranslationModelMetadata(super::TranslationModelMetadata),
        /// Metadata for image classification models.
        #[prost(message, tag = "13")]
        ImageClassificationModelMetadata(super::ImageClassificationModelMetadata),
        /// Metadata for text classification models.
        #[prost(message, tag = "14")]
        TextClassificationModelMetadata(super::TextClassificationModelMetadata),
        /// Metadata for image object detection models.
        #[prost(message, tag = "20")]
        ImageObjectDetectionModelMetadata(super::ImageObjectDetectionModelMetadata),
        /// Metadata for text extraction models.
        #[prost(message, tag = "19")]
        TextExtractionModelMetadata(super::TextExtractionModelMetadata),
        /// Metadata for text sentiment models.
        #[prost(message, tag = "22")]
        TextSentimentModelMetadata(super::TextSentimentModelMetadata),
    }
}
/// Evaluation results of a model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelEvaluation {
    /// Output only. Resource name of the model evaluation.
    /// Format:
    /// `projects/{project_id}/locations/{location_id}/models/{model_id}/modelEvaluations/{model_evaluation_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The ID of the annotation spec that the model evaluation applies to. The
    /// The ID is empty for the overall model evaluation.
    /// For Tables annotation specs in the dataset do not exist and this ID is
    /// always not set, but for CLASSIFICATION
    /// \[prediction_type-s][google.cloud.automl.v1.TablesModelMetadata.prediction_type\]
    /// the
    /// \[display_name][google.cloud.automl.v1.ModelEvaluation.display_name\]
    /// field is used.
    #[prost(string, tag = "2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// Output only. The value of
    /// \[display_name][google.cloud.automl.v1.AnnotationSpec.display_name\]
    /// at the moment when the model was trained. Because this field returns a
    /// value at model training time, for different models trained from the same
    /// dataset, the values may differ, since display names could had been changed
    /// between the two model's trainings. For Tables CLASSIFICATION
    /// \[prediction_type-s][google.cloud.automl.v1.TablesModelMetadata.prediction_type\]
    /// distinct values of the target column at the moment of the model evaluation
    /// are populated here.
    /// The display_name is empty for the overall model evaluation.
    #[prost(string, tag = "15")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Timestamp when this model evaluation was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The number of examples used for model evaluation, i.e. for
    /// which ground truth from time of model creation is compared against the
    /// predicted annotations created by the model.
    /// For overall ModelEvaluation (i.e. with annotation_spec_id not set) this is
    /// the total number of all examples used for evaluation.
    /// Otherwise, this is the count of examples that according to the ground
    /// truth were annotated by the
    /// \[annotation_spec_id][google.cloud.automl.v1.ModelEvaluation.annotation_spec_id\].
    #[prost(int32, tag = "6")]
    pub evaluated_example_count: i32,
    /// Output only. Problem type specific evaluation metrics.
    #[prost(oneof = "model_evaluation::Metrics", tags = "8, 9, 12, 11, 13")]
    pub metrics: ::core::option::Option<model_evaluation::Metrics>,
}
/// Nested message and enum types in `ModelEvaluation`.
pub mod model_evaluation {
    /// Output only. Problem type specific evaluation metrics.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metrics {
        /// Model evaluation metrics for image, text, video and tables
        /// classification.
        /// Tables problem is considered a classification when the target column
        /// is CATEGORY DataType.
        #[prost(message, tag = "8")]
        ClassificationEvaluationMetrics(super::ClassificationEvaluationMetrics),
        /// Model evaluation metrics for translation.
        #[prost(message, tag = "9")]
        TranslationEvaluationMetrics(super::TranslationEvaluationMetrics),
        /// Model evaluation metrics for image object detection.
        #[prost(message, tag = "12")]
        ImageObjectDetectionEvaluationMetrics(
            super::ImageObjectDetectionEvaluationMetrics,
        ),
        /// Evaluation metrics for text sentiment models.
        #[prost(message, tag = "11")]
        TextSentimentEvaluationMetrics(super::TextSentimentEvaluationMetrics),
        /// Evaluation metrics for text extraction models.
        #[prost(message, tag = "13")]
        TextExtractionEvaluationMetrics(super::TextExtractionEvaluationMetrics),
    }
}
/// Request message for \[AutoMl.CreateDataset][google.cloud.automl.v1.AutoMl.CreateDataset\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Required. The resource name of the project to create the dataset for.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The dataset to create.
    #[prost(message, optional, tag = "2")]
    pub dataset: ::core::option::Option<Dataset>,
}
/// Request message for \[AutoMl.GetDataset][google.cloud.automl.v1.AutoMl.GetDataset\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// Required. The resource name of the dataset to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.ListDatasets][google.cloud.automl.v1.AutoMl.ListDatasets\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. The resource name of the project from which to list datasets.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request.
    ///
    ///    * `dataset_metadata` - for existence of the case (e.g.
    ///              `image_classification_dataset_metadata:*`). Some examples of using the filter are:
    ///
    ///    * `translation_dataset_metadata:*` --> The dataset has
    ///                                           `translation_dataset_metadata`.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer results than requested.
    /// If unspecified, server will pick a default size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A token identifying a page of results for the server to return
    /// Typically obtained via
    /// \[ListDatasetsResponse.next_page_token][google.cloud.automl.v1.ListDatasetsResponse.next_page_token\] of the previous
    /// \[AutoMl.ListDatasets][google.cloud.automl.v1.AutoMl.ListDatasets\] call.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[AutoMl.ListDatasets][google.cloud.automl.v1.AutoMl.ListDatasets\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsResponse {
    /// The datasets read.
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// A token to retrieve next page of results.
    /// Pass to \[ListDatasetsRequest.page_token][google.cloud.automl.v1.ListDatasetsRequest.page_token\] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.UpdateDataset][google.cloud.automl.v1.AutoMl.UpdateDataset\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetRequest {
    /// Required. The dataset which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<Dataset>,
    /// Required. The update mask applies to the resource.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[AutoMl.DeleteDataset][google.cloud.automl.v1.AutoMl.DeleteDataset\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    /// Required. The resource name of the dataset to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.ImportData][google.cloud.automl.v1.AutoMl.ImportData\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataRequest {
    /// Required. Dataset name. Dataset must already exist. All imported
    /// annotations and examples will be added.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired input location and its domain specific semantics,
    /// if any.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::core::option::Option<InputConfig>,
}
/// Request message for \[AutoMl.ExportData][google.cloud.automl.v1.AutoMl.ExportData\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataRequest {
    /// Required. The resource name of the dataset.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired output location.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Request message for \[AutoMl.GetAnnotationSpec][google.cloud.automl.v1.AutoMl.GetAnnotationSpec\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationSpecRequest {
    /// Required. The resource name of the annotation spec to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.CreateModel][google.cloud.automl.v1.AutoMl.CreateModel\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModelRequest {
    /// Required. Resource name of the parent project where the model is being created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The model to create.
    #[prost(message, optional, tag = "4")]
    pub model: ::core::option::Option<Model>,
}
/// Request message for \[AutoMl.GetModel][google.cloud.automl.v1.AutoMl.GetModel\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. Resource name of the model.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.ListModels][google.cloud.automl.v1.AutoMl.ListModels\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Required. Resource name of the project, from which to list the models.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request.
    ///
    ///    * `model_metadata` - for existence of the case (e.g.
    ///              `video_classification_model_metadata:*`).
    ///    * `dataset_id` - for = or !=. Some examples of using the filter are:
    ///
    ///    * `image_classification_model_metadata:*` --> The model has
    ///                                       `image_classification_model_metadata`.
    ///    * `dataset_id=5` --> The model was created from a dataset with ID 5.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Requested page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A token identifying a page of results for the server to return
    /// Typically obtained via
    /// \[ListModelsResponse.next_page_token][google.cloud.automl.v1.ListModelsResponse.next_page_token\] of the previous
    /// \[AutoMl.ListModels][google.cloud.automl.v1.AutoMl.ListModels\] call.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[AutoMl.ListModels][google.cloud.automl.v1.AutoMl.ListModels\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// List of models in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub model: ::prost::alloc::vec::Vec<Model>,
    /// A token to retrieve next page of results.
    /// Pass to \[ListModelsRequest.page_token][google.cloud.automl.v1.ListModelsRequest.page_token\] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.DeleteModel][google.cloud.automl.v1.AutoMl.DeleteModel\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelRequest {
    /// Required. Resource name of the model being deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.UpdateModel][google.cloud.automl.v1.AutoMl.UpdateModel\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModelRequest {
    /// Required. The model which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<Model>,
    /// Required. The update mask applies to the resource.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[AutoMl.DeployModel][google.cloud.automl.v1.AutoMl.DeployModel\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployModelRequest {
    /// Required. Resource name of the model to deploy.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The per-domain specific deployment parameters.
    #[prost(oneof = "deploy_model_request::ModelDeploymentMetadata", tags = "2, 4")]
    pub model_deployment_metadata: ::core::option::Option<
        deploy_model_request::ModelDeploymentMetadata,
    >,
}
/// Nested message and enum types in `DeployModelRequest`.
pub mod deploy_model_request {
    /// The per-domain specific deployment parameters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModelDeploymentMetadata {
        /// Model deployment metadata specific to Image Object Detection.
        #[prost(message, tag = "2")]
        ImageObjectDetectionModelDeploymentMetadata(
            super::ImageObjectDetectionModelDeploymentMetadata,
        ),
        /// Model deployment metadata specific to Image Classification.
        #[prost(message, tag = "4")]
        ImageClassificationModelDeploymentMetadata(
            super::ImageClassificationModelDeploymentMetadata,
        ),
    }
}
/// Request message for \[AutoMl.UndeployModel][google.cloud.automl.v1.AutoMl.UndeployModel\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployModelRequest {
    /// Required. Resource name of the model to undeploy.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.ExportModel][google.cloud.automl.v1.AutoMl.ExportModel\].
/// Models need to be enabled for exporting, otherwise an error code will be
/// returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportModelRequest {
    /// Required. The resource name of the model to export.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired output location and configuration.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::core::option::Option<ModelExportOutputConfig>,
}
/// Request message for \[AutoMl.GetModelEvaluation][google.cloud.automl.v1.AutoMl.GetModelEvaluation\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelEvaluationRequest {
    /// Required. Resource name for the model evaluation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[AutoMl.ListModelEvaluations][google.cloud.automl.v1.AutoMl.ListModelEvaluations\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelEvaluationsRequest {
    /// Required. Resource name of the model to list the model evaluations for.
    /// If modelId is set as "-", this will list model evaluations from across all
    /// models of the parent location.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. An expression for filtering the results of the request.
    ///
    ///    * `annotation_spec_id` - for =, !=  or existence. See example below for
    ///                           the last.
    ///
    /// Some examples of using the filter are:
    ///
    ///    * `annotation_spec_id!=4` --> The model evaluation was done for
    ///                              annotation spec with ID different than 4.
    ///    * `NOT annotation_spec_id:*` --> The model evaluation was done for
    ///                                 aggregate of all annotation specs.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// Requested page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// A token identifying a page of results for the server to return.
    /// Typically obtained via
    /// \[ListModelEvaluationsResponse.next_page_token][google.cloud.automl.v1.ListModelEvaluationsResponse.next_page_token\] of the previous
    /// \[AutoMl.ListModelEvaluations][google.cloud.automl.v1.AutoMl.ListModelEvaluations\] call.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[AutoMl.ListModelEvaluations][google.cloud.automl.v1.AutoMl.ListModelEvaluations\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelEvaluationsResponse {
    /// List of model evaluations in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub model_evaluation: ::prost::alloc::vec::Vec<ModelEvaluation>,
    /// A token to retrieve next page of results.
    /// Pass to the \[ListModelEvaluationsRequest.page_token][google.cloud.automl.v1.ListModelEvaluationsRequest.page_token\] field of a new
    /// \[AutoMl.ListModelEvaluations][google.cloud.automl.v1.AutoMl.ListModelEvaluations\] request to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod auto_ml_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// AutoML Server API.
    ///
    /// The resource names are assigned by the server.
    /// The server never reuses names that it has created after the resources with
    /// those names are deleted.
    ///
    /// An ID of a resource is the last element of the item's resource name. For
    /// `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`, then
    /// the id for the item is `{dataset_id}`.
    ///
    /// Currently the only supported `location_id` is "us-central1".
    ///
    /// On any input that is documented to expect a string parameter in
    /// snake_case or dash-case, either of those cases is accepted.
    #[derive(Debug, Clone)]
    pub struct AutoMlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AutoMlClient<T>
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
        ) -> AutoMlClient<InterceptedService<T, F>>
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
            AutoMlClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a dataset.
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
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
                "/google.cloud.automl.v1.AutoMl/CreateDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.automl.v1.AutoMl", "CreateDataset"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a dataset.
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> std::result::Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.cloud.automl.v1.AutoMl/GetDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "GetDataset"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists datasets in a project.
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDatasetsResponse>,
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
                "/google.cloud.automl.v1.AutoMl/ListDatasets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.automl.v1.AutoMl", "ListDatasets"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a dataset.
        pub async fn update_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetRequest>,
        ) -> std::result::Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.cloud.automl.v1.AutoMl/UpdateDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.automl.v1.AutoMl", "UpdateDataset"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a dataset and all of its contents.
        /// Returns empty response in the
        /// [response][google.longrunning.Operation.response] field when it completes,
        /// and `delete_details` in the
        /// [metadata][google.longrunning.Operation.metadata] field.
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
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
                "/google.cloud.automl.v1.AutoMl/DeleteDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.automl.v1.AutoMl", "DeleteDataset"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports data into a dataset.
        /// For Tables this method can only be called on an empty Dataset.
        ///
        /// For Tables:
        /// *   A
        /// [schema_inference_version][google.cloud.automl.v1.InputConfig.params]
        ///     parameter must be explicitly set.
        /// Returns an empty response in the
        /// [response][google.longrunning.Operation.response] field when it completes.
        pub async fn import_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDataRequest>,
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
                "/google.cloud.automl.v1.AutoMl/ImportData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "ImportData"));
            self.inner.unary(req, path, codec).await
        }
        /// Exports dataset's data to the provided output location.
        /// Returns an empty response in the
        /// [response][google.longrunning.Operation.response] field when it completes.
        pub async fn export_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDataRequest>,
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
                "/google.cloud.automl.v1.AutoMl/ExportData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "ExportData"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets an annotation spec.
        pub async fn get_annotation_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationSpecRequest>,
        ) -> std::result::Result<tonic::Response<super::AnnotationSpec>, tonic::Status> {
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
                "/google.cloud.automl.v1.AutoMl/GetAnnotationSpec",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.automl.v1.AutoMl", "GetAnnotationSpec"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a model.
        /// Returns a Model in the [response][google.longrunning.Operation.response]
        /// field when it completes.
        /// When you create a model, several model evaluations are created for it:
        /// a global evaluation, and one evaluation for each annotation spec.
        pub async fn create_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModelRequest>,
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
                "/google.cloud.automl.v1.AutoMl/CreateModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "CreateModel"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets a model.
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> std::result::Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/google.cloud.automl.v1.AutoMl/GetModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "GetModel"));
            self.inner.unary(req, path, codec).await
        }
        /// Lists models.
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModelsResponse>,
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
                "/google.cloud.automl.v1.AutoMl/ListModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "ListModels"));
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a model.
        /// Returns `google.protobuf.Empty` in the
        /// [response][google.longrunning.Operation.response] field when it completes,
        /// and `delete_details` in the
        /// [metadata][google.longrunning.Operation.metadata] field.
        pub async fn delete_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelRequest>,
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
                "/google.cloud.automl.v1.AutoMl/DeleteModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "DeleteModel"));
            self.inner.unary(req, path, codec).await
        }
        /// Updates a model.
        pub async fn update_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateModelRequest>,
        ) -> std::result::Result<tonic::Response<super::Model>, tonic::Status> {
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
                "/google.cloud.automl.v1.AutoMl/UpdateModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "UpdateModel"));
            self.inner.unary(req, path, codec).await
        }
        /// Deploys a model. If a model is already deployed, deploying it with the
        /// same parameters has no effect. Deploying with different parametrs
        /// (as e.g. changing
        /// [node_number][google.cloud.automl.v1p1beta.ImageObjectDetectionModelDeploymentMetadata.node_number])
        ///  will reset the deployment state without pausing the model's availability.
        ///
        /// Only applicable for Text Classification, Image Object Detection , Tables, and Image Segmentation; all other domains manage
        /// deployment automatically.
        ///
        /// Returns an empty response in the
        /// [response][google.longrunning.Operation.response] field when it completes.
        pub async fn deploy_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployModelRequest>,
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
                "/google.cloud.automl.v1.AutoMl/DeployModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "DeployModel"));
            self.inner.unary(req, path, codec).await
        }
        /// Undeploys a model. If the model is not deployed this method has no effect.
        ///
        /// Only applicable for Text Classification, Image Object Detection and Tables;
        /// all other domains manage deployment automatically.
        ///
        /// Returns an empty response in the
        /// [response][google.longrunning.Operation.response] field when it completes.
        pub async fn undeploy_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeployModelRequest>,
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
                "/google.cloud.automl.v1.AutoMl/UndeployModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.automl.v1.AutoMl", "UndeployModel"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports a trained, "export-able", model to a user specified Google Cloud
        /// Storage location. A model is considered export-able if and only if it has
        /// an export format defined for it in
        /// [ModelExportOutputConfig][google.cloud.automl.v1.ModelExportOutputConfig].
        ///
        /// Returns an empty response in the
        /// [response][google.longrunning.Operation.response] field when it completes.
        pub async fn export_model(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportModelRequest>,
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
                "/google.cloud.automl.v1.AutoMl/ExportModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.automl.v1.AutoMl", "ExportModel"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets a model evaluation.
        pub async fn get_model_evaluation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelEvaluationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ModelEvaluation>,
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
                "/google.cloud.automl.v1.AutoMl/GetModelEvaluation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.automl.v1.AutoMl",
                        "GetModelEvaluation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists model evaluations.
        pub async fn list_model_evaluations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelEvaluationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModelEvaluationsResponse>,
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
                "/google.cloud.automl.v1.AutoMl/ListModelEvaluations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.automl.v1.AutoMl",
                        "ListModelEvaluations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Metadata used across all long running operations returned by AutoML API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. Progress of operation. Range: [0, 100].
    /// Not used currently.
    #[prost(int32, tag = "13")]
    pub progress_percent: i32,
    /// Output only. Partial failures encountered.
    /// E.g. single files that couldn't be read.
    /// This field should never exceed 20 entries.
    /// Status details field will contain standard GCP error details.
    #[prost(message, repeated, tag = "2")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Output only. Time when the operation was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the operation was updated for the last time.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Ouptut only. Details of specific operation. Even if this field is empty,
    /// the presence allows to distinguish different types of operations.
    #[prost(
        oneof = "operation_metadata::Details",
        tags = "8, 24, 25, 10, 30, 15, 16, 21, 22"
    )]
    pub details: ::core::option::Option<operation_metadata::Details>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Ouptut only. Details of specific operation. Even if this field is empty,
    /// the presence allows to distinguish different types of operations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Details of a Delete operation.
        #[prost(message, tag = "8")]
        DeleteDetails(super::DeleteOperationMetadata),
        /// Details of a DeployModel operation.
        #[prost(message, tag = "24")]
        DeployModelDetails(super::DeployModelOperationMetadata),
        /// Details of an UndeployModel operation.
        #[prost(message, tag = "25")]
        UndeployModelDetails(super::UndeployModelOperationMetadata),
        /// Details of CreateModel operation.
        #[prost(message, tag = "10")]
        CreateModelDetails(super::CreateModelOperationMetadata),
        /// Details of CreateDataset operation.
        #[prost(message, tag = "30")]
        CreateDatasetDetails(super::CreateDatasetOperationMetadata),
        /// Details of ImportData operation.
        #[prost(message, tag = "15")]
        ImportDataDetails(super::ImportDataOperationMetadata),
        /// Details of BatchPredict operation.
        #[prost(message, tag = "16")]
        BatchPredictDetails(super::BatchPredictOperationMetadata),
        /// Details of ExportData operation.
        #[prost(message, tag = "21")]
        ExportDataDetails(super::ExportDataOperationMetadata),
        /// Details of ExportModel operation.
        #[prost(message, tag = "22")]
        ExportModelDetails(super::ExportModelOperationMetadata),
    }
}
/// Details of operations that perform deletes of any entities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOperationMetadata {}
/// Details of DeployModel operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployModelOperationMetadata {}
/// Details of UndeployModel operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployModelOperationMetadata {}
/// Details of CreateDataset operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetOperationMetadata {}
/// Details of CreateModel operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModelOperationMetadata {}
/// Details of ImportData operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataOperationMetadata {}
/// Details of ExportData operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataOperationMetadata {
    /// Output only. Information further describing this export data's output.
    #[prost(message, optional, tag = "1")]
    pub output_info: ::core::option::Option<
        export_data_operation_metadata::ExportDataOutputInfo,
    >,
}
/// Nested message and enum types in `ExportDataOperationMetadata`.
pub mod export_data_operation_metadata {
    /// Further describes this export data's output.
    /// Supplements
    /// \[OutputConfig][google.cloud.automl.v1.OutputConfig\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExportDataOutputInfo {
        /// The output location to which the exported data is written.
        #[prost(oneof = "export_data_output_info::OutputLocation", tags = "1")]
        pub output_location: ::core::option::Option<
            export_data_output_info::OutputLocation,
        >,
    }
    /// Nested message and enum types in `ExportDataOutputInfo`.
    pub mod export_data_output_info {
        /// The output location to which the exported data is written.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OutputLocation {
            /// The full path of the Google Cloud Storage directory created, into which
            /// the exported data is written.
            #[prost(string, tag = "1")]
            GcsOutputDirectory(::prost::alloc::string::String),
        }
    }
}
/// Details of BatchPredict operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPredictOperationMetadata {
    /// Output only. The input config that was given upon starting this
    /// batch predict operation.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<BatchPredictInputConfig>,
    /// Output only. Information further describing this batch predict's output.
    #[prost(message, optional, tag = "2")]
    pub output_info: ::core::option::Option<
        batch_predict_operation_metadata::BatchPredictOutputInfo,
    >,
}
/// Nested message and enum types in `BatchPredictOperationMetadata`.
pub mod batch_predict_operation_metadata {
    /// Further describes this batch predict's output.
    /// Supplements
    /// \[BatchPredictOutputConfig][google.cloud.automl.v1.BatchPredictOutputConfig\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BatchPredictOutputInfo {
        /// The output location into which prediction output is written.
        #[prost(oneof = "batch_predict_output_info::OutputLocation", tags = "1")]
        pub output_location: ::core::option::Option<
            batch_predict_output_info::OutputLocation,
        >,
    }
    /// Nested message and enum types in `BatchPredictOutputInfo`.
    pub mod batch_predict_output_info {
        /// The output location into which prediction output is written.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OutputLocation {
            /// The full path of the Google Cloud Storage directory created, into which
            /// the prediction output is written.
            #[prost(string, tag = "1")]
            GcsOutputDirectory(::prost::alloc::string::String),
        }
    }
}
/// Details of ExportModel operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportModelOperationMetadata {
    /// Output only. Information further describing the output of this model
    /// export.
    #[prost(message, optional, tag = "2")]
    pub output_info: ::core::option::Option<
        export_model_operation_metadata::ExportModelOutputInfo,
    >,
}
/// Nested message and enum types in `ExportModelOperationMetadata`.
pub mod export_model_operation_metadata {
    /// Further describes the output of model export.
    /// Supplements
    /// \[ModelExportOutputConfig][google.cloud.automl.v1.ModelExportOutputConfig\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExportModelOutputInfo {
        /// The full path of the Google Cloud Storage directory created, into which
        /// the model will be exported.
        #[prost(string, tag = "1")]
        pub gcs_output_directory: ::prost::alloc::string::String,
    }
}
