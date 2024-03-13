/// A message returned from a device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceMessage {
    #[prost(oneof = "device_message::Contents", tags = "1, 2, 3")]
    pub contents: ::core::option::Option<device_message::Contents>,
}
/// Nested message and enum types in `DeviceMessage`.
pub mod device_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Contents {
        /// Information about the device's state.
        #[prost(message, tag = "1")]
        StatusUpdate(super::StatusUpdate),
        /// The result of a device stream from ADB.
        #[prost(message, tag = "2")]
        StreamStatus(super::StreamStatus),
        /// Data from an open stream.
        #[prost(message, tag = "3")]
        StreamData(super::StreamData),
    }
}
/// A message to an ADB server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdbMessage {
    #[prost(oneof = "adb_message::Contents", tags = "1, 2")]
    pub contents: ::core::option::Option<adb_message::Contents>,
}
/// Nested message and enum types in `AdbMessage`.
pub mod adb_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Contents {
        /// Open a new stream.
        #[prost(message, tag = "1")]
        Open(super::Open),
        /// Send data to a stream.
        #[prost(message, tag = "2")]
        StreamData(super::StreamData),
    }
}
/// A StatusUpdate message given over the ADB protocol for the device state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdate {
    /// The device's state
    #[prost(enumeration = "status_update::DeviceState", tag = "1")]
    pub state: i32,
    /// A map of properties with information about this device.
    #[prost(btree_map = "string, string", tag = "2")]
    pub properties: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A comma-separated list of "features" that this device supports.
    #[prost(string, tag = "3")]
    pub features: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StatusUpdate`.
pub mod status_update {
    /// The state displayed with the ADB Device when running "adb devices"
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
    pub enum DeviceState {
        /// The device state is unknown.
        Unspecified = 0,
        /// The ADB device is in the "device" status.
        Device = 1,
        /// The ADB device is in the "recovery" status.
        Recovery = 2,
        /// The ADB device is in the "rescue" status.
        Rescue = 3,
        /// The ADB device is in the "sideload" status.
        Sideload = 4,
        /// The ADB device is in the "missing" status.
        Missing = 10,
        /// The ADB device is in the "offline" status.
        Offline = 11,
        /// The ADB device is in the "unauthorized" status.
        Unauthorized = 12,
        /// The ADB device is in the "authorizing" status.
        Authorizing = 13,
        /// The ADB device is in the "connecting" status.
        Connecting = 14,
    }
    impl DeviceState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeviceState::Unspecified => "DEVICE_STATE_UNSPECIFIED",
                DeviceState::Device => "DEVICE",
                DeviceState::Recovery => "RECOVERY",
                DeviceState::Rescue => "RESCUE",
                DeviceState::Sideload => "SIDELOAD",
                DeviceState::Missing => "MISSING",
                DeviceState::Offline => "OFFLINE",
                DeviceState::Unauthorized => "UNAUTHORIZED",
                DeviceState::Authorizing => "AUTHORIZING",
                DeviceState::Connecting => "CONNECTING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEVICE_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEVICE" => Some(Self::Device),
                "RECOVERY" => Some(Self::Recovery),
                "RESCUE" => Some(Self::Rescue),
                "SIDELOAD" => Some(Self::Sideload),
                "MISSING" => Some(Self::Missing),
                "OFFLINE" => Some(Self::Offline),
                "UNAUTHORIZED" => Some(Self::Unauthorized),
                "AUTHORIZING" => Some(Self::Authorizing),
                "CONNECTING" => Some(Self::Connecting),
                _ => None,
            }
        }
    }
}
/// The result of a stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStatus {
    /// The unique ID of this stream, assigned by the client.
    #[prost(int32, tag = "1")]
    pub stream_id: i32,
    /// The result of the stream. Either "Okay" for success or "Fail" for failure.
    #[prost(oneof = "stream_status::Status", tags = "2, 3")]
    pub status: ::core::option::Option<stream_status::Status>,
}
/// Nested message and enum types in `StreamStatus`.
pub mod stream_status {
    /// The result of the stream. Either "Okay" for success or "Fail" for failure.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        /// Okay for success.
        #[prost(message, tag = "2")]
        Okay(super::Okay),
        /// Fail for failure.
        #[prost(message, tag = "3")]
        Fail(super::Fail),
    }
}
/// Message for opening a new stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Open {
    /// The unique ID that will be used to talk to this stream. This should
    /// probably just be a number that increments for each new Open request.
    #[prost(int32, tag = "1")]
    pub stream_id: i32,
    /// An ADB service to use in the new stream.
    #[prost(string, tag = "2")]
    pub service: ::prost::alloc::string::String,
}
/// Data for a stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamData {
    /// The unique ID of this stream, assigned by the client.
    #[prost(int32, tag = "1")]
    pub stream_id: i32,
    /// The data of the stream, either bytes or "Close", indicating that the stream
    /// is done.
    #[prost(oneof = "stream_data::Contents", tags = "2, 3")]
    pub contents: ::core::option::Option<stream_data::Contents>,
}
/// Nested message and enum types in `StreamData`.
pub mod stream_data {
    /// The data of the stream, either bytes or "Close", indicating that the stream
    /// is done.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Contents {
        /// Data in the stream.
        #[prost(bytes, tag = "2")]
        Data(::prost::bytes::Bytes),
        /// The stream is closing. EOF.
        #[prost(message, tag = "3")]
        Close(super::Close),
    }
}
/// Message signifying that the stream is open
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Okay {}
/// Message signifying that the stream failed to open
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fail {
    /// A user-displayable failure reason.
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
}
/// Message signifying that the stream closed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Close {}
/// TestMatrix captures all details about a test. It contains the environment
/// configuration, test specification, test executions and overall state and
/// outcome.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestMatrix {
    /// Output only. Unique id set by the service.
    #[prost(string, tag = "1")]
    pub test_matrix_id: ::prost::alloc::string::String,
    /// The cloud project that owns the test matrix.
    #[prost(string, tag = "7")]
    pub project_id: ::prost::alloc::string::String,
    /// Information about the client which invoked the test.
    #[prost(message, optional, tag = "10")]
    pub client_info: ::core::option::Option<ClientInfo>,
    /// Required. How to run the test.
    #[prost(message, optional, tag = "3")]
    pub test_specification: ::core::option::Option<TestSpecification>,
    /// Required. The devices the tests are being executed on.
    #[prost(message, optional, tag = "4")]
    pub environment_matrix: ::core::option::Option<EnvironmentMatrix>,
    /// Output only. The list of test executions that the service creates for
    /// this matrix.
    #[prost(message, repeated, tag = "5")]
    pub test_executions: ::prost::alloc::vec::Vec<TestExecution>,
    /// Required. Where the results for the matrix are written.
    #[prost(message, optional, tag = "6")]
    pub result_storage: ::core::option::Option<ResultStorage>,
    /// Output only. Indicates the current progress of the test matrix.
    #[prost(enumeration = "TestState", tag = "8")]
    pub state: i32,
    /// Output only. The time this test matrix was initially created.
    #[prost(message, optional, tag = "9")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Describes why the matrix is considered invalid.
    /// Only useful for matrices in the INVALID state.
    #[prost(enumeration = "InvalidMatrixDetails", tag = "11")]
    pub invalid_matrix_details: i32,
    /// Output only. Details about why a matrix was deemed invalid.
    /// If multiple checks can be safely performed, they will be reported but no
    /// assumptions should be made about the length of this list.
    #[prost(message, repeated, tag = "22")]
    pub extended_invalid_matrix_details: ::prost::alloc::vec::Vec<MatrixErrorDetail>,
    /// The number of times a TestExecution should be re-attempted if one or more
    /// of its test cases fail for any reason.
    /// The maximum number of reruns allowed is 10.
    ///
    /// Default is 0, which implies no reruns.
    #[prost(int32, tag = "13")]
    pub flaky_test_attempts: i32,
    /// Output Only. The overall outcome of the test.
    /// Only set when the test matrix state is FINISHED.
    #[prost(enumeration = "OutcomeSummary", tag = "14")]
    pub outcome_summary: i32,
    /// If true, only a single attempt at most will be made to run each
    /// execution/shard in the matrix. Flaky test attempts are not affected.
    ///
    /// Normally, 2 or more attempts are made if a potential infrastructure issue
    /// is detected.
    ///
    /// This feature is for latency sensitive workloads. The incidence of
    /// execution failures may be significantly greater for fail-fast matrices
    /// and support is more limited because of that expectation.
    #[prost(bool, tag = "17")]
    pub fail_fast: bool,
}
/// Describes a single error or issue with a matrix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatrixErrorDetail {
    /// Output only. The reason for the error. This is a constant value in
    /// UPPER_SNAKE_CASE that identifies the cause of the error.
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
    /// Output only. A human-readable message about how the error in the
    /// TestMatrix. Expands on the `reason` field with additional details and
    /// possible options to fix the issue.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// A single test executed in a single environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestExecution {
    /// Output only. Unique id set by the service.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Id of the containing TestMatrix.
    #[prost(string, tag = "9")]
    pub matrix_id: ::prost::alloc::string::String,
    /// Output only. The cloud project that owns the test execution.
    #[prost(string, tag = "10")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. How to run the test.
    #[prost(message, optional, tag = "3")]
    pub test_specification: ::core::option::Option<TestSpecification>,
    /// Output only. Details about the shard.
    #[prost(message, optional, tag = "12")]
    pub shard: ::core::option::Option<Shard>,
    /// Output only. How the host machine(s) are configured.
    #[prost(message, optional, tag = "4")]
    pub environment: ::core::option::Option<Environment>,
    /// Output only. Indicates the current progress of the test execution
    /// (e.g., FINISHED).
    #[prost(enumeration = "TestState", tag = "5")]
    pub state: i32,
    /// Output only. Where the results for this execution are written.
    #[prost(message, optional, tag = "11")]
    pub tool_results_step: ::core::option::Option<ToolResultsStep>,
    /// Output only. The time this test execution was initially created.
    #[prost(message, optional, tag = "7")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Additional details about the running test.
    #[prost(message, optional, tag = "8")]
    pub test_details: ::core::option::Option<TestDetails>,
}
/// A description of how to run the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSpecification {
    /// Max time a test execution is allowed to run before it is
    /// automatically cancelled.
    /// The default value is 5 min.
    #[prost(message, optional, tag = "1")]
    pub test_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Disables video recording. May reduce test latency.
    #[prost(bool, tag = "10")]
    pub disable_video_recording: bool,
    /// Disables performance metrics recording. May reduce test latency.
    #[prost(bool, tag = "11")]
    pub disable_performance_metrics: bool,
    /// Test setup requirements.
    #[prost(oneof = "test_specification::Setup", tags = "6, 14")]
    pub setup: ::core::option::Option<test_specification::Setup>,
    /// Required. The type of test to run.
    #[prost(oneof = "test_specification::Test", tags = "2, 3, 9, 13, 15, 17")]
    pub test: ::core::option::Option<test_specification::Test>,
}
/// Nested message and enum types in `TestSpecification`.
pub mod test_specification {
    /// Test setup requirements.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Setup {
        /// Test setup requirements for Android e.g. files to install, bootstrap
        /// scripts.
        #[prost(message, tag = "6")]
        TestSetup(super::TestSetup),
        /// Test setup requirements for iOS.
        #[prost(message, tag = "14")]
        IosTestSetup(super::IosTestSetup),
    }
    /// Required. The type of test to run.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Test {
        /// An Android instrumentation test.
        #[prost(message, tag = "2")]
        AndroidInstrumentationTest(super::AndroidInstrumentationTest),
        /// An Android robo test.
        #[prost(message, tag = "3")]
        AndroidRoboTest(super::AndroidRoboTest),
        /// An Android Application with a Test Loop.
        #[prost(message, tag = "9")]
        AndroidTestLoop(super::AndroidTestLoop),
        /// An iOS XCTest, via an .xctestrun file.
        #[prost(message, tag = "13")]
        IosXcTest(super::IosXcTest),
        /// An iOS application with a test loop.
        #[prost(message, tag = "15")]
        IosTestLoop(super::IosTestLoop),
        /// An iOS Robo test.
        #[prost(message, tag = "17")]
        IosRoboTest(super::IosRoboTest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystraceSetup {
    /// Systrace duration in seconds.
    /// Should be between 1 and 30 seconds. 0 disables systrace.
    #[deprecated]
    #[prost(int32, tag = "1")]
    pub duration_seconds: i32,
}
/// A description of how to set up the Android device prior to running the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSetup {
    /// List of files to push to the device before starting the test.
    #[prost(message, repeated, tag = "1")]
    pub files_to_push: ::prost::alloc::vec::Vec<DeviceFile>,
    /// List of directories on the device to upload to GCS at the end of the test;
    /// they must be absolute paths under /sdcard, /storage or /data/local/tmp.
    /// Path names are restricted to characters a-z A-Z 0-9 _ - . + and /
    ///
    /// Note: The paths /sdcard and /data will be made available and treated as
    /// implicit path substitutions. E.g. if /sdcard on a particular device does
    /// not map to external storage, the system will replace it with the external
    /// storage path prefix for that device.
    #[prost(string, repeated, tag = "2")]
    pub directories_to_pull: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Initial setup APKs to install before the app under test is
    /// installed. Currently capped at 100.
    #[prost(message, repeated, tag = "29")]
    pub initial_setup_apks: ::prost::alloc::vec::Vec<Apk>,
    /// APKs to install in addition to those being directly tested. These will be
    /// installed after the app under test.
    /// Currently capped at 100.
    #[prost(message, repeated, tag = "3")]
    pub additional_apks: ::prost::alloc::vec::Vec<Apk>,
    /// The device will be logged in on this account for the duration of the test.
    #[prost(message, optional, tag = "4")]
    pub account: ::core::option::Option<Account>,
    /// The network traffic profile used for running the test.
    /// Available network profiles can be queried by using the
    /// NETWORK_CONFIGURATION environment type when calling
    /// TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[prost(string, tag = "5")]
    pub network_profile: ::prost::alloc::string::String,
    /// Environment variables to set for the test (only applicable for
    /// instrumentation tests).
    #[prost(message, repeated, tag = "6")]
    pub environment_variables: ::prost::alloc::vec::Vec<EnvironmentVariable>,
    /// Systrace configuration for the run.
    /// Deprecated: Systrace used Python 2 which was sunsetted on 2020-01-01.
    /// Systrace is no longer supported in the Cloud Testing API, and no Systrace
    /// file will be provided in the results.
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub systrace: ::core::option::Option<SystraceSetup>,
    /// Whether to prevent all runtime permissions to be granted at app install
    #[prost(bool, tag = "23")]
    pub dont_autogrant_permissions: bool,
}
/// A description of how to set up an iOS device prior to running the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosTestSetup {
    /// The network traffic profile used for running the test.
    /// Available network profiles can be queried by using the
    /// NETWORK_CONFIGURATION environment type when calling
    /// TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[prost(string, tag = "1")]
    pub network_profile: ::prost::alloc::string::String,
    /// iOS apps to install in addition to those being directly tested.
    #[prost(message, repeated, tag = "2")]
    pub additional_ipas: ::prost::alloc::vec::Vec<FileReference>,
    /// List of files to push to the device before starting the test.
    #[prost(message, repeated, tag = "3")]
    pub push_files: ::prost::alloc::vec::Vec<IosDeviceFile>,
    /// List of directories on the device to upload to Cloud Storage at the end of
    /// the test.
    ///
    /// Directories should either be in a shared directory (such as
    /// /private/var/mobile/Media) or within an accessible directory inside the
    /// app's filesystem (such as /Documents) by specifying the bundle ID.
    #[prost(message, repeated, tag = "4")]
    pub pull_directories: ::prost::alloc::vec::Vec<IosDeviceFile>,
}
/// A key-value pair passed as an environment variable to the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentVariable {
    /// Key for the environment variable.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Value for the environment variable.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Identifies an account and how to log into it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Required. The type of account, based what it's for (e.g. Google) and what
    /// its login mechanism is (e.g. username and password).
    #[prost(oneof = "account::AccountType", tags = "1")]
    pub account_type: ::core::option::Option<account::AccountType>,
}
/// Nested message and enum types in `Account`.
pub mod account {
    /// Required. The type of account, based what it's for (e.g. Google) and what
    /// its login mechanism is (e.g. username and password).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccountType {
        /// An automatic google login account.
        #[prost(message, tag = "1")]
        GoogleAuto(super::GoogleAuto),
    }
}
/// Enables automatic Google account login.
/// If set, the service automatically generates a Google test account and adds
/// it to the device, before executing the test. Note that test accounts might be
/// reused.
/// Many applications show their full set of functionalities when an account is
/// present on the device. Logging into the device with these generated accounts
/// allows testing more functionalities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAuto {}
/// An Android package file to install.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Apk {
    /// The path to an APK to be installed on the device before the test begins.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<FileReference>,
    /// The java package for the APK to be installed.
    /// Value is determined by examining the application's manifest.
    #[prost(string, tag = "2")]
    pub package_name: ::prost::alloc::string::String,
}
/// An Android App Bundle file format, containing a BundleConfig.pb file,
/// a base module directory, zero or more dynamic feature module directories.
/// <p>See <https://developer.android.com/guide/app-bundle/build> for guidance on
/// building App Bundles.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppBundle {
    /// Required. Bundle location information.
    #[prost(oneof = "app_bundle::Bundle", tags = "1")]
    pub bundle: ::core::option::Option<app_bundle::Bundle>,
}
/// Nested message and enum types in `AppBundle`.
pub mod app_bundle {
    /// Required. Bundle location information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Bundle {
        /// .aab file representing the app bundle under test.
        #[prost(message, tag = "1")]
        BundleLocation(super::FileReference),
    }
}
/// A single device file description.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceFile {
    /// Required.
    #[prost(oneof = "device_file::DeviceFile", tags = "1, 2")]
    pub device_file: ::core::option::Option<device_file::DeviceFile>,
}
/// Nested message and enum types in `DeviceFile`.
pub mod device_file {
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeviceFile {
        /// A reference to an opaque binary blob file.
        #[prost(message, tag = "1")]
        ObbFile(super::ObbFile),
        /// A reference to a regular file.
        #[prost(message, tag = "2")]
        RegularFile(super::RegularFile),
    }
}
/// An opaque binary blob file to install on the device before the test starts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObbFile {
    /// Required. OBB file name which must conform to the format as specified by
    /// Android
    /// e.g. \[main|patch\].0300110.com.example.android.obb
    /// which will be installed into
    ///    \<shared-storage\>/Android/obb/\<package-name\>/
    /// on the device.
    #[prost(string, tag = "1")]
    pub obb_file_name: ::prost::alloc::string::String,
    /// Required. Opaque Binary Blob (OBB) file(s) to install on the device.
    #[prost(message, optional, tag = "2")]
    pub obb: ::core::option::Option<FileReference>,
}
/// A file or directory to install on the device before the test starts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegularFile {
    /// Required. The source file.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<FileReference>,
    /// Required. Where to put the content on the device. Must be an absolute,
    /// allowlisted path. If the file exists, it will be replaced.
    /// The following device-side directories and any of their subdirectories are
    /// allowlisted:
    /// <p>${EXTERNAL_STORAGE}, /sdcard, or /storage</p>
    /// <p>${ANDROID_DATA}/local/tmp, or /data/local/tmp</p>
    /// <p>Specifying a path outside of these directory trees is invalid.
    ///
    /// <p> The paths /sdcard and /data will be made available and treated as
    /// implicit path substitutions. E.g. if /sdcard on a particular device does
    /// not map to external storage, the system will replace it with the external
    /// storage path prefix for that device and copy the file there.
    ///
    /// <p> It is strongly advised to use the <a href=
    /// "<http://developer.android.com/reference/android/os/Environment.html">>
    /// Environment API</a> in app and test code to access files on the device in a
    /// portable way.
    #[prost(string, tag = "2")]
    pub device_path: ::prost::alloc::string::String,
}
/// A file or directory to install on the device before the test starts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDeviceFile {
    /// The source file
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<FileReference>,
    /// The bundle id of the app where this file lives.
    ///
    /// iOS apps sandbox their own filesystem, so app files must specify which app
    /// installed on the device.
    #[prost(string, tag = "2")]
    pub bundle_id: ::prost::alloc::string::String,
    /// Location of the file on the device, inside the app's sandboxed filesystem
    #[prost(string, tag = "3")]
    pub device_path: ::prost::alloc::string::String,
}
/// A test of an Android Application with a Test Loop.
/// The intent \<intent-name\> will be implicitly added, since Games is the only
/// user of this api, for the time being.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidTestLoop {
    /// The java package for the application under test.
    /// The default is determined by examining the application's manifest.
    #[prost(string, tag = "2")]
    pub app_package_id: ::prost::alloc::string::String,
    /// The list of scenarios that should be run during the test.
    /// The default is all test loops, derived from the application's
    /// manifest.
    #[prost(int32, repeated, tag = "3")]
    pub scenarios: ::prost::alloc::vec::Vec<i32>,
    /// The list of scenario labels that should be run during the test.
    /// The scenario labels should map to labels defined in the application's
    /// manifest. For example, player_experience and
    /// com.google.test.loops.player_experience add all of the loops labeled in the
    /// manifest with the com.google.test.loops.player_experience name to the
    /// execution.
    /// Scenarios can also be specified in the scenarios field.
    #[prost(string, repeated, tag = "4")]
    pub scenario_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The Android package to test.
    #[prost(oneof = "android_test_loop::AppUnderTest", tags = "1, 5")]
    pub app_under_test: ::core::option::Option<android_test_loop::AppUnderTest>,
}
/// Nested message and enum types in `AndroidTestLoop`.
pub mod android_test_loop {
    /// Required. The Android package to test.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AppUnderTest {
        /// The APK for the application under test.
        #[prost(message, tag = "1")]
        AppApk(super::FileReference),
        /// A multi-apk app bundle for the application under test.
        #[prost(message, tag = "5")]
        AppBundle(super::AppBundle),
    }
}
/// A test of an iOS application that uses the XCTest framework.
/// Xcode supports the option to "build for testing", which generates an
/// .xctestrun file that contains a test specification (arguments, test methods,
/// etc). This test type accepts a zip file containing the .xctestrun file and
/// the corresponding contents of the Build/Products directory that contains all
/// the binaries needed to run the tests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosXcTest {
    /// Required. The .zip containing the .xctestrun file and the contents of the
    /// DerivedData/Build/Products directory.
    /// The .xctestrun file in this zip is ignored if the xctestrun field is
    /// specified.
    #[prost(message, optional, tag = "1")]
    pub tests_zip: ::core::option::Option<FileReference>,
    /// An .xctestrun file that will override the .xctestrun file in the
    /// tests zip. Because the .xctestrun file contains environment variables along
    /// with test methods to run and/or ignore, this can be useful for sharding
    /// tests. Default is taken from the tests zip.
    #[prost(message, optional, tag = "2")]
    pub xctestrun: ::core::option::Option<FileReference>,
    /// The Xcode version that should be used for the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    /// Defaults to the latest Xcode version Firebase Test Lab supports.
    #[prost(string, tag = "3")]
    pub xcode_version: ::prost::alloc::string::String,
    /// Output only. The bundle id for the application under test.
    #[prost(string, tag = "4")]
    pub app_bundle_id: ::prost::alloc::string::String,
    /// The option to test special app entitlements. Setting this would re-sign the
    /// app having special entitlements with an explicit application-identifier.
    /// Currently supports testing aps-environment entitlement.
    #[prost(bool, tag = "6")]
    pub test_special_entitlements: bool,
}
/// A test of an iOS application that implements one or more game loop scenarios.
/// This test type accepts an archived application (.ipa file) and a list of
/// integer scenarios that will be executed on the app sequentially.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosTestLoop {
    /// Required. The .ipa of the application to test.
    #[prost(message, optional, tag = "1")]
    pub app_ipa: ::core::option::Option<FileReference>,
    /// The list of scenarios that should be run during the test. Defaults to the
    /// single scenario 0 if unspecified.
    #[prost(int32, repeated, tag = "2")]
    pub scenarios: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The bundle id for the application under test.
    #[prost(string, tag = "3")]
    pub app_bundle_id: ::prost::alloc::string::String,
}
/// A test that explores an iOS application on an iOS device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosRoboTest {
    /// Required. The ipa stored at this file should be used to run the test.
    #[prost(message, optional, tag = "1")]
    pub app_ipa: ::core::option::Option<FileReference>,
    /// The bundle ID for the app-under-test.
    /// This is determined by examining the application's "Info.plist" file.
    #[prost(string, tag = "4")]
    pub app_bundle_id: ::prost::alloc::string::String,
    /// An optional Roboscript to customize the crawl. See
    /// <https://firebase.google.com/docs/test-lab/android/robo-scripts-reference>
    /// for more information about Roboscripts.
    #[prost(message, optional, tag = "5")]
    pub robo_script: ::core::option::Option<FileReference>,
}
/// A test of an Android application that can control an Android component
/// independently of its normal lifecycle.
/// Android instrumentation tests run an application APK and test APK inside the
/// same process on a virtual or physical AndroidDevice.  They also specify
/// a test runner class, such as com.google.GoogleTestRunner, which can vary
/// on the specific instrumentation framework chosen.
///
/// See <<https://developer.android.com/training/testing/fundamentals>> for
/// more information on types of Android tests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidInstrumentationTest {
    /// Required. The APK containing the test code to be executed.
    #[prost(message, optional, tag = "2")]
    pub test_apk: ::core::option::Option<FileReference>,
    /// The java package for the application under test.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "3")]
    pub app_package_id: ::prost::alloc::string::String,
    /// The java package for the test to be executed.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "4")]
    pub test_package_id: ::prost::alloc::string::String,
    /// The InstrumentationTestRunner class.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "5")]
    pub test_runner_class: ::prost::alloc::string::String,
    /// Each target must be fully qualified with the package name or class name,
    /// in one of these formats:
    ///
    ///   - "package package_name"
    ///   - "class package_name.class_name"
    ///   - "class package_name.class_name#method_name"
    ///
    /// If empty, all targets in the module will be run.
    #[prost(string, repeated, tag = "6")]
    pub test_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The option of whether running each test within its own invocation of
    /// instrumentation with Android Test Orchestrator or not.
    /// ** Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or
    /// higher! **
    /// Orchestrator offers the following benefits:
    ///
    ///   - No shared state
    ///   - Crashes are isolated
    ///   - Logs are scoped per test
    ///
    /// See
    /// <<https://developer.android.com/training/testing/junit-runner.html#using-android-test-orchestrator>>
    /// for more information about Android Test Orchestrator.
    ///
    /// If not set, the test will be run without the orchestrator.
    #[prost(enumeration = "OrchestratorOption", tag = "7")]
    pub orchestrator_option: i32,
    /// The option to run tests in multiple shards in parallel.
    #[prost(message, optional, tag = "9")]
    pub sharding_option: ::core::option::Option<ShardingOption>,
    /// Required.
    #[prost(oneof = "android_instrumentation_test::AppUnderTest", tags = "1, 8")]
    pub app_under_test: ::core::option::Option<
        android_instrumentation_test::AppUnderTest,
    >,
}
/// Nested message and enum types in `AndroidInstrumentationTest`.
pub mod android_instrumentation_test {
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AppUnderTest {
        /// The APK for the application under test.
        #[prost(message, tag = "1")]
        AppApk(super::FileReference),
        /// A multi-apk app bundle for the application under test.
        #[prost(message, tag = "8")]
        AppBundle(super::AppBundle),
    }
}
/// A test of an android application that explores the application on a virtual
/// or physical Android Device, finding culprits and crashes as it goes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidRoboTest {
    /// The java package for the application under test.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "2")]
    pub app_package_id: ::prost::alloc::string::String,
    /// The initial activity that should be used to start the app.
    #[prost(string, tag = "3")]
    pub app_initial_activity: ::prost::alloc::string::String,
    /// The max depth of the traversal stack Robo can explore. Needs to be at least
    /// 2 to make Robo explore the app beyond the first activity.
    /// Default is 50.
    #[deprecated]
    #[prost(int32, tag = "7")]
    pub max_depth: i32,
    /// The max number of steps Robo can execute.
    /// Default is no limit.
    #[deprecated]
    #[prost(int32, tag = "8")]
    pub max_steps: i32,
    /// A set of directives Robo should apply during the crawl.
    /// This allows users to customize the crawl. For example, the username and
    /// password for a test account can be provided.
    #[prost(message, repeated, tag = "11")]
    pub robo_directives: ::prost::alloc::vec::Vec<RoboDirective>,
    /// The mode in which Robo should run. Most clients should allow the server to
    /// populate this field automatically.
    #[prost(enumeration = "RoboMode", tag = "14")]
    pub robo_mode: i32,
    /// A JSON file with a sequence of actions Robo should perform as a prologue
    /// for the crawl.
    #[prost(message, optional, tag = "13")]
    pub robo_script: ::core::option::Option<FileReference>,
    /// The intents used to launch the app for the crawl.
    /// If none are provided, then the main launcher activity is launched.
    /// If some are provided, then only those provided are launched (the main
    /// launcher activity must be provided explicitly).
    #[prost(message, repeated, tag = "15")]
    pub starting_intents: ::prost::alloc::vec::Vec<RoboStartingIntent>,
    /// Required.
    #[prost(oneof = "android_robo_test::AppUnderTest", tags = "1, 16")]
    pub app_under_test: ::core::option::Option<android_robo_test::AppUnderTest>,
}
/// Nested message and enum types in `AndroidRoboTest`.
pub mod android_robo_test {
    /// Required.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AppUnderTest {
        /// The APK for the application under test.
        #[prost(message, tag = "1")]
        AppApk(super::FileReference),
        /// A multi-apk app bundle for the application under test.
        #[prost(message, tag = "16")]
        AppBundle(super::AppBundle),
    }
}
/// Directs Robo to interact with a specific UI element if it is encountered
/// during the crawl. Currently, Robo can perform text entry or element click.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoboDirective {
    /// Required. The android resource name of the target UI element.
    /// For example,
    ///     in Java: R.string.foo
    ///     in xml: @string/foo
    /// Only the "foo" part is needed.
    /// Reference doc:
    /// <https://developer.android.com/guide/topics/resources/accessing-resources.html>
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The text that Robo is directed to set. If left empty, the directive will be
    /// treated as a CLICK on the element matching the resource_name.
    #[prost(string, tag = "2")]
    pub input_text: ::prost::alloc::string::String,
    /// Required. The type of action that Robo should perform on the specified
    /// element.
    #[prost(enumeration = "RoboActionType", tag = "3")]
    pub action_type: i32,
}
/// Message for specifying the start activities to crawl.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoboStartingIntent {
    /// Timeout in seconds for each intent.
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Required. Intent details to start an activity.
    #[prost(oneof = "robo_starting_intent::StartingIntent", tags = "1, 2, 4")]
    pub starting_intent: ::core::option::Option<robo_starting_intent::StartingIntent>,
}
/// Nested message and enum types in `RoboStartingIntent`.
pub mod robo_starting_intent {
    /// Required. Intent details to start an activity.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartingIntent {
        /// An intent that starts the main launcher activity.
        #[prost(message, tag = "1")]
        LauncherActivity(super::LauncherActivityIntent),
        /// An intent that starts an activity with specific details.
        #[prost(message, tag = "2")]
        StartActivity(super::StartActivityIntent),
        /// Skips the starting activity
        #[prost(message, tag = "4")]
        NoActivity(super::NoActivityIntent),
    }
}
/// Specifies an intent that starts the main launcher activity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LauncherActivityIntent {}
/// A starting intent specified by an action, uri, and categories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartActivityIntent {
    /// Action name.
    /// Required for START_ACTIVITY.
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    /// URI for the action.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// Intent categories to set on the intent.
    #[prost(string, repeated, tag = "4")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Skips the starting activity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoActivityIntent {}
/// The matrix of environments in which the test is to be executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentMatrix {
    /// Required. The environment matrix.
    #[prost(oneof = "environment_matrix::EnvironmentMatrix", tags = "1, 2, 3")]
    pub environment_matrix: ::core::option::Option<
        environment_matrix::EnvironmentMatrix,
    >,
}
/// Nested message and enum types in `EnvironmentMatrix`.
pub mod environment_matrix {
    /// Required. The environment matrix.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnvironmentMatrix {
        /// A matrix of Android devices.
        #[prost(message, tag = "1")]
        AndroidMatrix(super::AndroidMatrix),
        /// A list of Android devices; the test will be run only on the specified
        /// devices.
        #[prost(message, tag = "2")]
        AndroidDeviceList(super::AndroidDeviceList),
        /// A list of iOS devices.
        #[prost(message, tag = "3")]
        IosDeviceList(super::IosDeviceList),
    }
}
/// A list of Android device configurations in which the test is to be executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDeviceList {
    /// Required. A list of Android devices.
    #[prost(message, repeated, tag = "1")]
    pub android_devices: ::prost::alloc::vec::Vec<AndroidDevice>,
}
/// A list of iOS device configurations in which the test is to be executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDeviceList {
    /// Required. A list of iOS devices.
    #[prost(message, repeated, tag = "1")]
    pub ios_devices: ::prost::alloc::vec::Vec<IosDevice>,
}
/// A set of Android device configuration permutations is defined by the
/// the cross-product of the given axes. Internally, the given AndroidMatrix
/// will be expanded into a set of AndroidDevices.
///
/// Only supported permutations will be instantiated.  Invalid permutations
/// (e.g., incompatible models/versions) are ignored.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidMatrix {
    /// Required. The ids of the set of Android device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "1")]
    pub android_model_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The ids of the set of Android OS version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "2")]
    pub android_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The set of locales the test device will enable for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "3")]
    pub locales: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The set of orientations to test with.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "4")]
    pub orientations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information about the client which invoked the test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    /// Required. Client name, such as gcloud.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of detailed information about client.
    #[prost(message, repeated, tag = "2")]
    pub client_info_details: ::prost::alloc::vec::Vec<ClientInfoDetail>,
}
/// Key-value pair of detailed information about the client which invoked the
/// test. Examples: {'Version', '1.0'}, {'Release Track', 'BETA'}.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfoDetail {
    /// Required. The key of detailed client information.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Required. The value of detailed client information.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Locations where the results of running the test are stored.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultStorage {
    /// Required.
    #[prost(message, optional, tag = "1")]
    pub google_cloud_storage: ::core::option::Option<GoogleCloudStorage>,
    /// The tool results history that contains the tool results execution that
    /// results are written to.
    ///
    /// If not provided, the service will choose an appropriate value.
    #[prost(message, optional, tag = "5")]
    pub tool_results_history: ::core::option::Option<ToolResultsHistory>,
    /// Output only. The tool results execution that results are written to.
    #[prost(message, optional, tag = "6")]
    pub tool_results_execution: ::core::option::Option<ToolResultsExecution>,
    /// Output only. URL to the results in the Firebase Web Console.
    #[prost(string, tag = "7")]
    pub results_url: ::prost::alloc::string::String,
}
/// Represents a tool results history resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultsHistory {
    /// Required. The cloud project that owns the tool results history.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. A tool results history ID.
    #[prost(string, tag = "2")]
    pub history_id: ::prost::alloc::string::String,
}
/// Represents a tool results execution resource.
///
/// This has the results of a TestMatrix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultsExecution {
    /// Output only. The cloud project that owns the tool results execution.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. A tool results history ID.
    #[prost(string, tag = "2")]
    pub history_id: ::prost::alloc::string::String,
    /// Output only. A tool results execution ID.
    #[prost(string, tag = "3")]
    pub execution_id: ::prost::alloc::string::String,
}
/// Represents a tool results step resource.
///
/// This has the results of a TestExecution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultsStep {
    /// Output only. The cloud project that owns the tool results step.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. A tool results history ID.
    #[prost(string, tag = "2")]
    pub history_id: ::prost::alloc::string::String,
    /// Output only. A tool results execution ID.
    #[prost(string, tag = "3")]
    pub execution_id: ::prost::alloc::string::String,
    /// Output only. A tool results step ID.
    #[prost(string, tag = "4")]
    pub step_id: ::prost::alloc::string::String,
}
/// A storage location within Google cloud storage (GCS).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleCloudStorage {
    /// Required. The path to a directory in GCS that will
    /// eventually contain the results for this test.
    /// The requesting user must have write access on the bucket in the supplied
    /// path.
    #[prost(string, tag = "1")]
    pub gcs_path: ::prost::alloc::string::String,
}
/// A reference to a file, used for user inputs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileReference {
    /// Required. The file reference.
    #[prost(oneof = "file_reference::File", tags = "1")]
    pub file: ::core::option::Option<file_reference::File>,
}
/// Nested message and enum types in `FileReference`.
pub mod file_reference {
    /// Required. The file reference.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum File {
        /// A path to a file in Google Cloud Storage.
        /// Example: gs://build-app-1414623860166/app%40debug-unaligned.apk
        /// These paths are expected to be url encoded (percent encoding)
        #[prost(string, tag = "1")]
        GcsPath(::prost::alloc::string::String),
    }
}
/// The environment in which the test is run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Required. The environment.
    #[prost(oneof = "environment::Environment", tags = "1, 2")]
    pub environment: ::core::option::Option<environment::Environment>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Required. The environment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Environment {
        /// An Android device which must be used with an Android test.
        #[prost(message, tag = "1")]
        AndroidDevice(super::AndroidDevice),
        /// An iOS device which must be used with an iOS test.
        #[prost(message, tag = "2")]
        IosDevice(super::IosDevice),
    }
}
/// A single Android device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDevice {
    /// Required. The id of the Android device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "1")]
    pub android_model_id: ::prost::alloc::string::String,
    /// Required. The id of the Android OS version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "2")]
    pub android_version_id: ::prost::alloc::string::String,
    /// Required. The locale the test device used for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "3")]
    pub locale: ::prost::alloc::string::String,
    /// Required. How the device is oriented during the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "4")]
    pub orientation: ::prost::alloc::string::String,
}
/// A single iOS device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDevice {
    /// Required. The id of the iOS device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "1")]
    pub ios_model_id: ::prost::alloc::string::String,
    /// Required. The id of the iOS major software version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "2")]
    pub ios_version_id: ::prost::alloc::string::String,
    /// Required. The locale the test device used for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "3")]
    pub locale: ::prost::alloc::string::String,
    /// Required. How the device is oriented during the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "4")]
    pub orientation: ::prost::alloc::string::String,
}
/// Additional details about the progress of the running test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestDetails {
    /// Output only. Human-readable, detailed descriptions of the test's progress.
    /// For example: "Provisioning a device", "Starting Test".
    ///
    /// During the course of execution new data may be appended
    /// to the end of progress_messages.
    #[prost(string, repeated, tag = "3")]
    pub progress_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. If the TestState is ERROR, then this string will contain
    /// human-readable details about the error.
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
/// Details behind an invalid request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidRequestDetail {
    /// The reason behind the error.
    #[prost(enumeration = "invalid_request_detail::Reason", tag = "1")]
    pub reason: i32,
}
/// Nested message and enum types in `InvalidRequestDetail`.
pub mod invalid_request_detail {
    /// Possible invalid request reasons.
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
    pub enum Reason {
        /// No reason has been specified - the default.
        Unspecified = 0,
        /// The request is not valid.
        RequestInvalid = 1,
        /// One or more of the resources specified in the request is too large.
        ResourceTooBig = 2,
        /// One or more resources specified in the request cannot be found.
        ResourceNotFound = 3,
        /// This request is not (currently) supported.
        Unsupported = 4,
        /// This request is not currently implemented.
        NotImplemented = 5,
        /// The caller has no permission for storing the test results
        ResultStoragePermissionDenied = 6,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unspecified => "REASON_UNSPECIFIED",
                Reason::RequestInvalid => "REQUEST_INVALID",
                Reason::ResourceTooBig => "RESOURCE_TOO_BIG",
                Reason::ResourceNotFound => "RESOURCE_NOT_FOUND",
                Reason::Unsupported => "UNSUPPORTED",
                Reason::NotImplemented => "NOT_IMPLEMENTED",
                Reason::ResultStoragePermissionDenied => {
                    "RESULT_STORAGE_PERMISSION_DENIED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "REQUEST_INVALID" => Some(Self::RequestInvalid),
                "RESOURCE_TOO_BIG" => Some(Self::ResourceTooBig),
                "RESOURCE_NOT_FOUND" => Some(Self::ResourceNotFound),
                "UNSUPPORTED" => Some(Self::Unsupported),
                "NOT_IMPLEMENTED" => Some(Self::NotImplemented),
                "RESULT_STORAGE_PERMISSION_DENIED" => {
                    Some(Self::ResultStoragePermissionDenied)
                }
                _ => None,
            }
        }
    }
}
/// Options for enabling sharding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardingOption {
    #[prost(oneof = "sharding_option::Option", tags = "1, 2, 3")]
    pub option: ::core::option::Option<sharding_option::Option>,
}
/// Nested message and enum types in `ShardingOption`.
pub mod sharding_option {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Option {
        /// Uniformly shards test cases given a total number of shards.
        #[prost(message, tag = "1")]
        UniformSharding(super::UniformSharding),
        /// Shards test cases into the specified groups of packages, classes, and/or
        /// methods.
        #[prost(message, tag = "2")]
        ManualSharding(super::ManualSharding),
        /// Shards test based on previous test case timing records.
        #[prost(message, tag = "3")]
        SmartSharding(super::SmartSharding),
    }
}
/// Uniformly shards test cases given a total number of shards.
///
/// For instrumentation tests, it will be translated to "-e numShard" and "-e
/// shardIndex" AndroidJUnitRunner arguments. With uniform sharding enabled,
/// specifying either of these sharding arguments via `environment_variables` is
/// invalid.
///
/// Based on the sharding mechanism AndroidJUnitRunner uses, there is no
/// guarantee that test cases will be distributed uniformly across all shards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniformSharding {
    /// Required. The total number of shards to create. This must always be a
    /// positive number that is no greater than the total number of test cases.
    /// When you select one or more physical devices, the number of shards must be
    /// <= 50. When you select one or more ARM virtual devices, it must be <= 200.
    /// When you select only x86 virtual devices, it must be <= 500.
    #[prost(int32, tag = "1")]
    pub num_shards: i32,
}
/// Shards test cases into the specified groups of packages, classes, and/or
/// methods.
///
/// With manual sharding enabled, specifying test targets via
/// environment_variables or in InstrumentationTest is invalid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualSharding {
    /// Required. Group of packages, classes, and/or test methods to be run for
    /// each manually-created shard. You must specify at least one shard if this
    /// field is present. When you select one or more physical devices, the number
    /// of repeated test_targets_for_shard must be <= 50. When you select one or
    /// more ARM virtual devices, it must be <= 200. When you select only x86
    /// virtual devices, it must be <= 500.
    #[prost(message, repeated, tag = "1")]
    pub test_targets_for_shard: ::prost::alloc::vec::Vec<TestTargetsForShard>,
}
/// Test targets for a shard.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestTargetsForShard {
    /// Group of packages, classes, and/or test methods to be run for each shard.
    /// The targets need to be specified in AndroidJUnitRunner argument format. For
    /// example, "package com.my.packages" "class com.my.package.MyClass".
    ///
    /// The number of test_targets must be greater than 0.
    #[prost(string, repeated, tag = "1")]
    pub test_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Shards test based on previous test case timing records.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartSharding {
    /// The amount of time tests within a shard should take.
    ///
    /// Default: 300 seconds (5 minutes).
    /// The minimum allowed: 120 seconds (2 minutes).
    ///
    /// The shard count is dynamically set based on time, up to the maximum shard
    /// limit (described below). To guarantee at least one test case for each
    /// shard, the number of shards will not exceed the number of test cases. Shard
    /// duration will be exceeded if:
    ///
    /// - The maximum shard limit is reached and there is more calculated test time
    /// remaining to allocate into shards.
    /// - Any individual test is estimated to be longer than the targeted shard
    /// duration.
    ///
    /// Shard duration is not guaranteed because smart sharding uses test case
    /// history and default durations which may not be accurate. The rules for
    /// finding the test case timing records are:
    ///
    /// - If the service has processed a test case in the last 30 days, the record
    ///   of the latest successful test case will be used.
    /// - For new test cases, the average duration of other known test cases will
    ///   be used.
    /// - If there are no previous test case timing records available, the default
    ///   test case duration is 15 seconds.
    ///
    /// Because the actual shard duration can exceed the targeted shard duration,
    /// we recommend that you set the targeted value at least 5 minutes less than
    /// the maximum allowed test timeout (45 minutes for physical devices and 60
    /// minutes for virtual), or that you use the custom test timeout value that
    /// you set. This approach avoids cancelling the shard before all tests can
    /// finish.
    ///
    /// Note that there is a limit for maximum number of shards. When you select
    /// one or more physical devices, the number of shards must be <= 50. When you
    /// select one or more ARM virtual devices, it must be <= 200. When you select
    /// only x86 virtual devices, it must be <= 500. To guarantee at least one test
    /// case for per shard, the number of shards will not exceed the number of test
    /// cases. Each shard created counts toward daily test quota.
    #[prost(message, optional, tag = "1")]
    pub targeted_shard_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Output only. Details about the shard.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    /// Output only. The index of the shard among all the shards.
    #[prost(int32, tag = "1")]
    pub shard_index: i32,
    /// Output only. The total number of shards.
    #[prost(int32, tag = "2")]
    pub num_shards: i32,
    /// Output only. Test targets for each shard. Only set for manual sharding.
    #[prost(message, optional, tag = "3")]
    pub test_targets_for_shard: ::core::option::Option<TestTargetsForShard>,
    /// Output only. The estimated shard duration based on previous test case
    /// timing records, if available.
    #[prost(message, optional, tag = "4")]
    pub estimated_shard_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Request to submit a matrix of tests for execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTestMatrixRequest {
    /// The GCE project under which this job will run.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The matrix of tests that the user wants to run.
    #[prost(message, optional, tag = "2")]
    pub test_matrix: ::core::option::Option<TestMatrix>,
    /// A string id used to detect duplicated requests.
    /// Ids are automatically scoped to a project, so
    /// users should ensure the ID is unique per-project.
    /// A UUID is recommended.
    ///
    /// Optional, but strongly recommended.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request to get the Test Matrix with the given id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestMatrixRequest {
    /// Cloud project that owns the test matrix.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Unique test matrix id which was assigned by the service.
    #[prost(string, tag = "2")]
    pub test_matrix_id: ::prost::alloc::string::String,
}
/// Request to stop running all of the tests in the specified matrix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTestMatrixRequest {
    /// Cloud project that owns the test.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Test matrix that will be canceled.
    #[prost(string, tag = "2")]
    pub test_matrix_id: ::prost::alloc::string::String,
}
/// Response containing the current state of the specified test matrix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTestMatrixResponse {
    /// The current rolled-up state of the test matrix.
    /// If this state is already final, then the cancelation request will
    /// have no effect.
    #[prost(enumeration = "TestState", tag = "1")]
    pub test_state: i32,
}
/// Specifies how to execute the test.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrchestratorOption {
    /// Default value: the server will choose the mode. Currently implies that
    /// the test will run without the orchestrator. In the future,
    /// all instrumentation tests will be run with the orchestrator.
    /// Using the orchestrator is highly encouraged because of all the benefits it
    /// offers.
    Unspecified = 0,
    /// Run test using orchestrator.
    /// ** Only compatible with AndroidJUnitRunner version 1.1 or higher! **
    /// Recommended.
    UseOrchestrator = 1,
    /// Run test without using orchestrator.
    DoNotUseOrchestrator = 2,
}
impl OrchestratorOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrchestratorOption::Unspecified => "ORCHESTRATOR_OPTION_UNSPECIFIED",
            OrchestratorOption::UseOrchestrator => "USE_ORCHESTRATOR",
            OrchestratorOption::DoNotUseOrchestrator => "DO_NOT_USE_ORCHESTRATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORCHESTRATOR_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "USE_ORCHESTRATOR" => Some(Self::UseOrchestrator),
            "DO_NOT_USE_ORCHESTRATOR" => Some(Self::DoNotUseOrchestrator),
            _ => None,
        }
    }
}
/// The mode in which Robo should run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoboMode {
    /// This means that the server should choose the mode.
    /// Recommended.
    Unspecified = 0,
    /// Runs Robo in UIAutomator-only mode without app resigning
    RoboVersion1 = 1,
    /// Runs Robo in standard Espresso with UIAutomator fallback
    RoboVersion2 = 2,
}
impl RoboMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoboMode::Unspecified => "ROBO_MODE_UNSPECIFIED",
            RoboMode::RoboVersion1 => "ROBO_VERSION_1",
            RoboMode::RoboVersion2 => "ROBO_VERSION_2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROBO_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "ROBO_VERSION_1" => Some(Self::RoboVersion1),
            "ROBO_VERSION_2" => Some(Self::RoboVersion2),
            _ => None,
        }
    }
}
/// Actions which Robo can perform on UI elements.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoboActionType {
    /// DO NOT USE. For proto versioning only.
    ActionTypeUnspecified = 0,
    /// Direct Robo to click on the specified element. No-op if specified element
    /// is not clickable.
    SingleClick = 1,
    /// Direct Robo to enter text on the specified element. No-op if specified
    /// element is not enabled or does not allow text entry.
    EnterText = 2,
    /// Direct Robo to ignore interactions with a specific element.
    Ignore = 3,
}
impl RoboActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoboActionType::ActionTypeUnspecified => "ACTION_TYPE_UNSPECIFIED",
            RoboActionType::SingleClick => "SINGLE_CLICK",
            RoboActionType::EnterText => "ENTER_TEXT",
            RoboActionType::Ignore => "IGNORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_TYPE_UNSPECIFIED" => Some(Self::ActionTypeUnspecified),
            "SINGLE_CLICK" => Some(Self::SingleClick),
            "ENTER_TEXT" => Some(Self::EnterText),
            "IGNORE" => Some(Self::Ignore),
            _ => None,
        }
    }
}
/// The detailed reason that a Matrix was deemed INVALID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InvalidMatrixDetails {
    /// Do not use. For proto versioning only.
    Unspecified = 0,
    /// The matrix is INVALID, but there are no further details available.
    DetailsUnavailable = 1,
    /// The input app APK could not be parsed.
    MalformedApk = 2,
    /// The input test APK could not be parsed.
    MalformedTestApk = 3,
    /// The AndroidManifest.xml could not be found.
    NoManifest = 4,
    /// The APK manifest does not declare a package name.
    NoPackageName = 5,
    /// The APK application ID (aka package name) is invalid.
    /// See also
    /// <https://developer.android.com/studio/build/application-id>
    InvalidPackageName = 31,
    /// The test package and app package are the same.
    TestSameAsApp = 6,
    /// The test apk does not declare an instrumentation.
    NoInstrumentation = 7,
    /// The input app apk does not have a signature.
    NoSignature = 20,
    /// The test runner class specified by user or in the test APK's manifest file
    /// is not compatible with Android Test Orchestrator.
    /// Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or
    /// higher.
    /// Orchestrator can be disabled by using DO_NOT_USE_ORCHESTRATOR
    /// OrchestratorOption.
    InstrumentationOrchestratorIncompatible = 18,
    /// The test APK does not contain the test runner class specified by the user
    /// or in the manifest file. This can be caused by one of the following
    /// reasons:
    ///
    /// - the user provided a runner class name that's incorrect, or
    /// - the test runner isn't built into the test APK (might be in the app APK
    /// instead).
    NoTestRunnerClass = 19,
    /// A main launcher activity could not be found.
    NoLauncherActivity = 8,
    /// The app declares one or more permissions that are not allowed.
    ForbiddenPermissions = 9,
    /// There is a conflict in the provided robo_directives.
    InvalidRoboDirectives = 10,
    /// There is at least one invalid resource name in the provided
    /// robo directives
    InvalidResourceName = 33,
    /// Invalid definition of action in the robo directives
    /// (e.g. a click or ignore action includes an input text field)
    InvalidDirectiveAction = 34,
    /// There is no test loop intent filter, or the one that is given is
    /// not formatted correctly.
    TestLoopIntentFilterNotFound = 12,
    /// The request contains a scenario label that was not declared in the
    /// manifest.
    ScenarioLabelNotDeclared = 13,
    /// There was an error when parsing a label's value.
    ScenarioLabelMalformed = 14,
    /// The request contains a scenario number that was not declared in the
    /// manifest.
    ScenarioNotDeclared = 15,
    /// Device administrator applications are not allowed.
    DeviceAdminReceiver = 17,
    /// The zipped XCTest was malformed. The zip did not contain a single
    /// .xctestrun file and the contents of the DerivedData/Build/Products
    /// directory.
    MalformedXcTestZip = 11,
    /// The zipped XCTest was built for the iOS simulator rather than for a
    /// physical device.
    BuiltForIosSimulator = 24,
    /// The .xctestrun file did not specify any test targets.
    NoTestsInXcTestZip = 25,
    /// One or more of the test targets defined in the .xctestrun file specifies
    /// "UseDestinationArtifacts", which is disallowed.
    UseDestinationArtifacts = 26,
    /// XC tests which run on physical devices must have
    /// "IsAppHostedTestBundle" == "true" in the xctestrun file.
    TestNotAppHosted = 28,
    /// An Info.plist file in the XCTest zip could not be parsed.
    PlistCannotBeParsed = 30,
    /// The APK is marked as "testOnly".
    /// Deprecated and not currently used.
    TestOnlyApk = 21,
    /// The input IPA could not be parsed.
    MalformedIpa = 22,
    /// The application doesn't register the game loop URL scheme.
    MissingUrlScheme = 35,
    /// The iOS application bundle (.app) couldn't be processed.
    MalformedAppBundle = 36,
    /// APK contains no code.
    /// See also
    /// <https://developer.android.com/guide/topics/manifest/application-element.html#code>
    NoCodeApk = 23,
    /// Either the provided input APK path was malformed,
    /// the APK file does not exist, or the user does not have permission to
    /// access the APK file.
    InvalidInputApk = 27,
    /// APK is built for a preview SDK which is unsupported
    InvalidApkPreviewSdk = 29,
    /// The matrix expanded to contain too many executions.
    MatrixTooLarge = 37,
    /// Not enough test quota to run the executions in this matrix.
    TestQuotaExceeded = 39,
    /// A required cloud service api is not activated.
    /// See:
    /// <https://firebase.google.com/docs/test-lab/android/continuous#requirements>
    ServiceNotActivated = 40,
    /// There was an unknown permission issue running this test.
    UnknownPermissionError = 41,
}
impl InvalidMatrixDetails {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InvalidMatrixDetails::Unspecified => "INVALID_MATRIX_DETAILS_UNSPECIFIED",
            InvalidMatrixDetails::DetailsUnavailable => "DETAILS_UNAVAILABLE",
            InvalidMatrixDetails::MalformedApk => "MALFORMED_APK",
            InvalidMatrixDetails::MalformedTestApk => "MALFORMED_TEST_APK",
            InvalidMatrixDetails::NoManifest => "NO_MANIFEST",
            InvalidMatrixDetails::NoPackageName => "NO_PACKAGE_NAME",
            InvalidMatrixDetails::InvalidPackageName => "INVALID_PACKAGE_NAME",
            InvalidMatrixDetails::TestSameAsApp => "TEST_SAME_AS_APP",
            InvalidMatrixDetails::NoInstrumentation => "NO_INSTRUMENTATION",
            InvalidMatrixDetails::NoSignature => "NO_SIGNATURE",
            InvalidMatrixDetails::InstrumentationOrchestratorIncompatible => {
                "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE"
            }
            InvalidMatrixDetails::NoTestRunnerClass => "NO_TEST_RUNNER_CLASS",
            InvalidMatrixDetails::NoLauncherActivity => "NO_LAUNCHER_ACTIVITY",
            InvalidMatrixDetails::ForbiddenPermissions => "FORBIDDEN_PERMISSIONS",
            InvalidMatrixDetails::InvalidRoboDirectives => "INVALID_ROBO_DIRECTIVES",
            InvalidMatrixDetails::InvalidResourceName => "INVALID_RESOURCE_NAME",
            InvalidMatrixDetails::InvalidDirectiveAction => "INVALID_DIRECTIVE_ACTION",
            InvalidMatrixDetails::TestLoopIntentFilterNotFound => {
                "TEST_LOOP_INTENT_FILTER_NOT_FOUND"
            }
            InvalidMatrixDetails::ScenarioLabelNotDeclared => {
                "SCENARIO_LABEL_NOT_DECLARED"
            }
            InvalidMatrixDetails::ScenarioLabelMalformed => "SCENARIO_LABEL_MALFORMED",
            InvalidMatrixDetails::ScenarioNotDeclared => "SCENARIO_NOT_DECLARED",
            InvalidMatrixDetails::DeviceAdminReceiver => "DEVICE_ADMIN_RECEIVER",
            InvalidMatrixDetails::MalformedXcTestZip => "MALFORMED_XC_TEST_ZIP",
            InvalidMatrixDetails::BuiltForIosSimulator => "BUILT_FOR_IOS_SIMULATOR",
            InvalidMatrixDetails::NoTestsInXcTestZip => "NO_TESTS_IN_XC_TEST_ZIP",
            InvalidMatrixDetails::UseDestinationArtifacts => "USE_DESTINATION_ARTIFACTS",
            InvalidMatrixDetails::TestNotAppHosted => "TEST_NOT_APP_HOSTED",
            InvalidMatrixDetails::PlistCannotBeParsed => "PLIST_CANNOT_BE_PARSED",
            InvalidMatrixDetails::TestOnlyApk => "TEST_ONLY_APK",
            InvalidMatrixDetails::MalformedIpa => "MALFORMED_IPA",
            InvalidMatrixDetails::MissingUrlScheme => "MISSING_URL_SCHEME",
            InvalidMatrixDetails::MalformedAppBundle => "MALFORMED_APP_BUNDLE",
            InvalidMatrixDetails::NoCodeApk => "NO_CODE_APK",
            InvalidMatrixDetails::InvalidInputApk => "INVALID_INPUT_APK",
            InvalidMatrixDetails::InvalidApkPreviewSdk => "INVALID_APK_PREVIEW_SDK",
            InvalidMatrixDetails::MatrixTooLarge => "MATRIX_TOO_LARGE",
            InvalidMatrixDetails::TestQuotaExceeded => "TEST_QUOTA_EXCEEDED",
            InvalidMatrixDetails::ServiceNotActivated => "SERVICE_NOT_ACTIVATED",
            InvalidMatrixDetails::UnknownPermissionError => "UNKNOWN_PERMISSION_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_MATRIX_DETAILS_UNSPECIFIED" => Some(Self::Unspecified),
            "DETAILS_UNAVAILABLE" => Some(Self::DetailsUnavailable),
            "MALFORMED_APK" => Some(Self::MalformedApk),
            "MALFORMED_TEST_APK" => Some(Self::MalformedTestApk),
            "NO_MANIFEST" => Some(Self::NoManifest),
            "NO_PACKAGE_NAME" => Some(Self::NoPackageName),
            "INVALID_PACKAGE_NAME" => Some(Self::InvalidPackageName),
            "TEST_SAME_AS_APP" => Some(Self::TestSameAsApp),
            "NO_INSTRUMENTATION" => Some(Self::NoInstrumentation),
            "NO_SIGNATURE" => Some(Self::NoSignature),
            "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE" => {
                Some(Self::InstrumentationOrchestratorIncompatible)
            }
            "NO_TEST_RUNNER_CLASS" => Some(Self::NoTestRunnerClass),
            "NO_LAUNCHER_ACTIVITY" => Some(Self::NoLauncherActivity),
            "FORBIDDEN_PERMISSIONS" => Some(Self::ForbiddenPermissions),
            "INVALID_ROBO_DIRECTIVES" => Some(Self::InvalidRoboDirectives),
            "INVALID_RESOURCE_NAME" => Some(Self::InvalidResourceName),
            "INVALID_DIRECTIVE_ACTION" => Some(Self::InvalidDirectiveAction),
            "TEST_LOOP_INTENT_FILTER_NOT_FOUND" => {
                Some(Self::TestLoopIntentFilterNotFound)
            }
            "SCENARIO_LABEL_NOT_DECLARED" => Some(Self::ScenarioLabelNotDeclared),
            "SCENARIO_LABEL_MALFORMED" => Some(Self::ScenarioLabelMalformed),
            "SCENARIO_NOT_DECLARED" => Some(Self::ScenarioNotDeclared),
            "DEVICE_ADMIN_RECEIVER" => Some(Self::DeviceAdminReceiver),
            "MALFORMED_XC_TEST_ZIP" => Some(Self::MalformedXcTestZip),
            "BUILT_FOR_IOS_SIMULATOR" => Some(Self::BuiltForIosSimulator),
            "NO_TESTS_IN_XC_TEST_ZIP" => Some(Self::NoTestsInXcTestZip),
            "USE_DESTINATION_ARTIFACTS" => Some(Self::UseDestinationArtifacts),
            "TEST_NOT_APP_HOSTED" => Some(Self::TestNotAppHosted),
            "PLIST_CANNOT_BE_PARSED" => Some(Self::PlistCannotBeParsed),
            "TEST_ONLY_APK" => Some(Self::TestOnlyApk),
            "MALFORMED_IPA" => Some(Self::MalformedIpa),
            "MISSING_URL_SCHEME" => Some(Self::MissingUrlScheme),
            "MALFORMED_APP_BUNDLE" => Some(Self::MalformedAppBundle),
            "NO_CODE_APK" => Some(Self::NoCodeApk),
            "INVALID_INPUT_APK" => Some(Self::InvalidInputApk),
            "INVALID_APK_PREVIEW_SDK" => Some(Self::InvalidApkPreviewSdk),
            "MATRIX_TOO_LARGE" => Some(Self::MatrixTooLarge),
            "TEST_QUOTA_EXCEEDED" => Some(Self::TestQuotaExceeded),
            "SERVICE_NOT_ACTIVATED" => Some(Self::ServiceNotActivated),
            "UNKNOWN_PERMISSION_ERROR" => Some(Self::UnknownPermissionError),
            _ => None,
        }
    }
}
/// The state (i.e., progress) of a test execution or matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TestState {
    /// Do not use.  For proto versioning only.
    Unspecified = 0,
    /// The execution or matrix is being validated.
    Validating = 8,
    /// The execution or matrix is waiting for resources to become available.
    Pending = 1,
    /// The execution is currently being processed.
    ///
    /// Can only be set on an execution.
    Running = 2,
    /// The execution or matrix has terminated normally.
    ///
    /// On a matrix this means that the matrix level processing completed normally,
    /// but individual executions may be in an ERROR state.
    Finished = 3,
    /// The execution or matrix has stopped because it encountered an
    /// infrastructure failure.
    Error = 4,
    /// The execution was not run because it corresponds to a unsupported
    /// environment.
    ///
    /// Can only be set on an execution.
    UnsupportedEnvironment = 5,
    /// The execution was not run because the provided inputs are incompatible with
    /// the requested environment.
    ///
    /// Example: requested AndroidVersion is lower than APK's minSdkVersion
    ///
    /// Can only be set on an execution.
    IncompatibleEnvironment = 9,
    /// The execution was not run because the provided inputs are incompatible with
    /// the requested architecture.
    ///
    /// Example: requested device does not support running the native code in
    /// the supplied APK
    ///
    /// Can only be set on an execution.
    IncompatibleArchitecture = 10,
    /// The user cancelled the execution.
    ///
    /// Can only be set on an execution.
    Cancelled = 6,
    /// The execution or matrix was not run because the provided inputs are not
    /// valid.
    ///
    /// Examples: input file is not of the expected type, is malformed/corrupt, or
    /// was flagged as malware
    Invalid = 7,
}
impl TestState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TestState::Unspecified => "TEST_STATE_UNSPECIFIED",
            TestState::Validating => "VALIDATING",
            TestState::Pending => "PENDING",
            TestState::Running => "RUNNING",
            TestState::Finished => "FINISHED",
            TestState::Error => "ERROR",
            TestState::UnsupportedEnvironment => "UNSUPPORTED_ENVIRONMENT",
            TestState::IncompatibleEnvironment => "INCOMPATIBLE_ENVIRONMENT",
            TestState::IncompatibleArchitecture => "INCOMPATIBLE_ARCHITECTURE",
            TestState::Cancelled => "CANCELLED",
            TestState::Invalid => "INVALID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEST_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "VALIDATING" => Some(Self::Validating),
            "PENDING" => Some(Self::Pending),
            "RUNNING" => Some(Self::Running),
            "FINISHED" => Some(Self::Finished),
            "ERROR" => Some(Self::Error),
            "UNSUPPORTED_ENVIRONMENT" => Some(Self::UnsupportedEnvironment),
            "INCOMPATIBLE_ENVIRONMENT" => Some(Self::IncompatibleEnvironment),
            "INCOMPATIBLE_ARCHITECTURE" => Some(Self::IncompatibleArchitecture),
            "CANCELLED" => Some(Self::Cancelled),
            "INVALID" => Some(Self::Invalid),
            _ => None,
        }
    }
}
/// Outcome summary for a finished test matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutcomeSummary {
    /// Do not use. For proto versioning only.
    Unspecified = 0,
    /// The test matrix run was successful, for instance:
    ///
    /// - All the test cases passed.
    /// - Robo did not detect a crash of the application under test.
    Success = 1,
    /// A run failed, for instance:
    ///
    /// - One or more test cases failed.
    /// - A test timed out.
    /// - The application under test crashed.
    Failure = 2,
    /// Something unexpected happened. The run should still be considered
    /// unsuccessful but this is likely a transient problem and re-running the
    /// test might be successful.
    Inconclusive = 3,
    /// All tests were skipped, for instance:
    ///
    /// - All device configurations were incompatible.
    Skipped = 4,
}
impl OutcomeSummary {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OutcomeSummary::Unspecified => "OUTCOME_SUMMARY_UNSPECIFIED",
            OutcomeSummary::Success => "SUCCESS",
            OutcomeSummary::Failure => "FAILURE",
            OutcomeSummary::Inconclusive => "INCONCLUSIVE",
            OutcomeSummary::Skipped => "SKIPPED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OUTCOME_SUMMARY_UNSPECIFIED" => Some(Self::Unspecified),
            "SUCCESS" => Some(Self::Success),
            "FAILURE" => Some(Self::Failure),
            "INCONCLUSIVE" => Some(Self::Inconclusive),
            "SKIPPED" => Some(Self::Skipped),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod test_execution_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service for requesting test executions and querying their status.
    ///
    /// This service is part of Firebase Test Lab. To learn about how to use the
    /// product, and how to integrate it with your system,
    /// visit https://firebase.google.com/docs/test-lab.
    ///
    /// Each test execution will wait for available capacity. It will then be
    /// invoked as described. The test may be invoked multiple times if an
    /// infrastructure failure is detected. Results and other files generated by
    /// the test will be stored in an external storage system.
    ///
    /// The TestExecutionService models this behavior using two resource types:
    ///
    /// - TestMatrix: a group of one or more TestExecutions, built by taking a
    ///   product of values over a pre-defined set of axes. In the case of Android
    ///   Tests, for example, device model and OS version are two axes of the matrix.
    ///
    /// - TestExecution: a single execution of one or more test targets on a
    ///   single device. These are created automatically when a TestMatrix is
    ///   created.
    ///
    /// This service returns any error codes from the canonical error space (i.e.
    /// google.rpc.Code). The errors which may be returned are specified on each
    /// method. In addition, any method may return UNAVAILABLE or INTERNAL.
    #[derive(Debug, Clone)]
    pub struct TestExecutionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TestExecutionServiceClient<T>
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
        ) -> TestExecutionServiceClient<InterceptedService<T, F>>
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
            TestExecutionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates and runs a matrix of tests according to the given specifications.
        /// Unsupported environments will be returned in the state UNSUPPORTED.
        /// A test matrix is limited to use at most 2000 devices in parallel.
        ///
        /// The returned matrix will not yet contain the executions that will be
        /// created for this matrix. Execution creation happens later on and will
        /// require a call to GetTestMatrix.
        ///
        /// May return any of the following canonical error codes:
        ///
        /// - PERMISSION_DENIED - if the user is not authorized to write to project
        /// - INVALID_ARGUMENT - if the request is malformed or if the matrix tries
        ///                      to use too many simultaneous devices.
        pub async fn create_test_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestMatrixRequest>,
        ) -> std::result::Result<tonic::Response<super::TestMatrix>, tonic::Status> {
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
                "/google.devtools.testing.v1.TestExecutionService/CreateTestMatrix",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.TestExecutionService",
                        "CreateTestMatrix",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Checks the status of a test matrix and the executions once they
        /// are created.
        ///
        /// The test matrix will contain the list of test executions to run if and only
        /// if the resultStorage.toolResultsExecution fields have been populated.
        ///
        /// Note: Flaky test executions may be added to the matrix at a later stage.
        ///
        /// May return any of the following canonical error codes:
        ///
        /// - PERMISSION_DENIED - if the user is not authorized to read project
        /// - INVALID_ARGUMENT - if the request is malformed
        /// - NOT_FOUND - if the Test Matrix does not exist
        pub async fn get_test_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestMatrixRequest>,
        ) -> std::result::Result<tonic::Response<super::TestMatrix>, tonic::Status> {
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
                "/google.devtools.testing.v1.TestExecutionService/GetTestMatrix",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.TestExecutionService",
                        "GetTestMatrix",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancels unfinished test executions in a test matrix.
        /// This call returns immediately and cancellation proceeds asynchronously.
        /// If the matrix is already final, this operation will have no effect.
        ///
        /// May return any of the following canonical error codes:
        ///
        /// - PERMISSION_DENIED - if the user is not authorized to read project
        /// - INVALID_ARGUMENT - if the request is malformed
        /// - NOT_FOUND - if the Test Matrix does not exist
        pub async fn cancel_test_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelTestMatrixRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelTestMatrixResponse>,
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
                "/google.devtools.testing.v1.TestExecutionService/CancelTestMatrix",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.TestExecutionService",
                        "CancelTestMatrix",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Android application details based on application manifest and archive
/// contents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApkDetail {
    #[prost(message, optional, tag = "1")]
    pub apk_manifest: ::core::option::Option<ApkManifest>,
}
/// An Android app manifest. See
/// <http://developer.android.com/guide/topics/manifest/manifest-intro.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApkManifest {
    /// Full Java-style package name for this application, e.g.
    /// "com.example.foo".
    #[prost(string, tag = "1")]
    pub package_name: ::prost::alloc::string::String,
    /// Minimum API level required for the application to run.
    #[prost(int32, tag = "2")]
    pub min_sdk_version: i32,
    /// Maximum API level on which the application is designed to run.
    #[prost(int32, tag = "3")]
    pub max_sdk_version: i32,
    /// Specifies the API Level on which the application is designed to run.
    #[prost(int32, tag = "6")]
    pub target_sdk_version: i32,
    /// User-readable name for the application.
    #[prost(string, tag = "4")]
    pub application_label: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub intent_filters: ::prost::alloc::vec::Vec<IntentFilter>,
    /// Permissions declared to be used by the application
    #[prost(string, repeated, tag = "7")]
    pub uses_permission: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Version number used internally by the app.
    #[prost(int64, tag = "8")]
    pub version_code: i64,
    /// Version number shown to users.
    #[prost(string, tag = "9")]
    pub version_name: ::prost::alloc::string::String,
    /// Meta-data tags defined in the manifest.
    #[prost(message, repeated, tag = "10")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    /// Feature usage tags defined in the manifest.
    #[prost(message, repeated, tag = "11")]
    pub uses_feature: ::prost::alloc::vec::Vec<UsesFeature>,
    /// Services contained in the <application> tag.
    #[prost(message, repeated, tag = "12")]
    pub services: ::prost::alloc::vec::Vec<Service>,
}
/// The <service> section of an <application> tag.
/// <https://developer.android.com/guide/topics/manifest/service-element>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The android:name value
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Intent filters in the service
    #[prost(message, repeated, tag = "2")]
    pub intent_filter: ::prost::alloc::vec::Vec<IntentFilter>,
}
/// The <intent-filter> section of an <activity> tag.
/// <https://developer.android.com/guide/topics/manifest/intent-filter-element.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentFilter {
    /// The android:name value of the <action> tag.
    #[prost(string, repeated, tag = "1")]
    pub action_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The android:name value of the <category> tag.
    #[prost(string, repeated, tag = "2")]
    pub category_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The android:mimeType value of the <data> tag.
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// A <meta-data> tag within a manifest.
/// <https://developer.android.com/guide/topics/manifest/meta-data-element.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// The android:name value
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The android:value value
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// A <uses-feature> tag within a manifest.
/// <https://developer.android.com/guide/topics/manifest/uses-feature-element.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsesFeature {
    /// The android:name value
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The android:required value
    #[prost(bool, tag = "2")]
    pub is_required: bool,
}
/// A request to get the details of an Android application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApkDetailsRequest {
    /// Optional. The APK to be parsed for details.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<FileReference>,
    /// Optional. The App Bundle to be parsed for details.
    #[prost(message, optional, tag = "2")]
    pub bundle_location: ::core::option::Option<FileReference>,
}
/// Response containing the details of the specified Android application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApkDetailsResponse {
    /// Details of the Android App.
    #[prost(message, optional, tag = "1")]
    pub apk_detail: ::core::option::Option<ApkDetail>,
}
/// Generated client implementations.
pub mod application_detail_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service which parses input applications and returns details that can be
    /// useful in the context of testing.
    #[derive(Debug, Clone)]
    pub struct ApplicationDetailServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ApplicationDetailServiceClient<T>
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
        ) -> ApplicationDetailServiceClient<InterceptedService<T, F>>
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
            ApplicationDetailServiceClient::new(
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
        /// Gets the details of an Android application APK.
        pub async fn get_apk_details(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApkDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApkDetailsResponse>,
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
                "/google.devtools.testing.v1.ApplicationDetailService/GetApkDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.ApplicationDetailService",
                        "GetApkDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A single device IP block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceIpBlock {
    /// An IP address block in CIDR notation eg: 34.68.194.64/29
    #[prost(string, tag = "1")]
    pub block: ::prost::alloc::string::String,
    /// Whether this block is used by physical or virtual devices
    #[prost(enumeration = "DeviceForm", tag = "2")]
    pub form: i32,
    /// The date this block was added to Firebase Test Lab
    #[prost(message, optional, tag = "3")]
    pub added_date: ::core::option::Option<super::super::super::r#type::Date>,
}
/// Request to list the currently supported values for an environment type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestEnvironmentCatalogRequest {
    /// Required. The type of environment that should be listed.
    #[prost(
        enumeration = "get_test_environment_catalog_request::EnvironmentType",
        tag = "1"
    )]
    pub environment_type: i32,
    /// For authorization, the cloud project requesting the TestEnvironmentCatalog.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GetTestEnvironmentCatalogRequest`.
pub mod get_test_environment_catalog_request {
    /// Types of environments the Test API supports.
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
    pub enum EnvironmentType {
        /// Do not use.  For proto versioning only.
        Unspecified = 0,
        /// A device running a version of the Android OS.
        Android = 1,
        /// A device running a version of iOS.
        Ios = 3,
        /// A network configuration to use when running a test.
        NetworkConfiguration = 4,
        /// The software environment provided by TestExecutionService.
        ProvidedSoftware = 5,
        /// The IP blocks used by devices in the test environment.
        DeviceIpBlocks = 6,
    }
    impl EnvironmentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnvironmentType::Unspecified => "ENVIRONMENT_TYPE_UNSPECIFIED",
                EnvironmentType::Android => "ANDROID",
                EnvironmentType::Ios => "IOS",
                EnvironmentType::NetworkConfiguration => "NETWORK_CONFIGURATION",
                EnvironmentType::ProvidedSoftware => "PROVIDED_SOFTWARE",
                EnvironmentType::DeviceIpBlocks => "DEVICE_IP_BLOCKS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENVIRONMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ANDROID" => Some(Self::Android),
                "IOS" => Some(Self::Ios),
                "NETWORK_CONFIGURATION" => Some(Self::NetworkConfiguration),
                "PROVIDED_SOFTWARE" => Some(Self::ProvidedSoftware),
                "DEVICE_IP_BLOCKS" => Some(Self::DeviceIpBlocks),
                _ => None,
            }
        }
    }
}
/// A description of a test environment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestEnvironmentCatalog {
    /// Output only.
    #[prost(
        oneof = "test_environment_catalog::EnvironmentCatalog",
        tags = "1, 3, 4, 5, 6"
    )]
    pub environment_catalog: ::core::option::Option<
        test_environment_catalog::EnvironmentCatalog,
    >,
}
/// Nested message and enum types in `TestEnvironmentCatalog`.
pub mod test_environment_catalog {
    /// Output only.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnvironmentCatalog {
        /// Supported Android devices.
        #[prost(message, tag = "1")]
        AndroidDeviceCatalog(super::AndroidDeviceCatalog),
        /// Supported iOS devices.
        #[prost(message, tag = "3")]
        IosDeviceCatalog(super::IosDeviceCatalog),
        /// Supported network configurations.
        #[prost(message, tag = "4")]
        NetworkConfigurationCatalog(super::NetworkConfigurationCatalog),
        /// The software test environment provided by TestExecutionService.
        #[prost(message, tag = "5")]
        SoftwareCatalog(super::ProvidedSoftwareCatalog),
        /// The IP blocks used by devices in the test environment.
        #[prost(message, tag = "6")]
        DeviceIpBlockCatalog(super::DeviceIpBlockCatalog),
    }
}
/// List of IP blocks used by the Firebase Test Lab
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceIpBlockCatalog {
    /// The device IP blocks used by Firebase Test Lab
    #[prost(message, repeated, tag = "1")]
    pub ip_blocks: ::prost::alloc::vec::Vec<DeviceIpBlock>,
}
/// The currently supported Android devices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDeviceCatalog {
    /// The set of supported Android device models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<AndroidModel>,
    /// The set of supported Android OS versions.
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<AndroidVersion>,
    /// The set of supported runtime configurations.
    #[prost(message, optional, tag = "3")]
    pub runtime_configuration: ::core::option::Option<AndroidRuntimeConfiguration>,
}
/// Android configuration that can be selected at the time a test is run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidRuntimeConfiguration {
    /// The set of available locales.
    #[prost(message, repeated, tag = "1")]
    pub locales: ::prost::alloc::vec::Vec<Locale>,
    /// The set of available orientations.
    #[prost(message, repeated, tag = "2")]
    pub orientations: ::prost::alloc::vec::Vec<Orientation>,
}
/// A description of an Android device tests may be run on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidModel {
    /// The unique opaque id for this model.
    /// Use this for invoking the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The human-readable marketing name for this device model.
    /// Examples: "Nexus 5", "Galaxy S5".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The manufacturer of this device.
    #[prost(string, tag = "3")]
    pub manufacturer: ::prost::alloc::string::String,
    /// The company that this device is branded with.
    /// Example: "Google", "Samsung".
    #[prost(string, tag = "9")]
    pub brand: ::prost::alloc::string::String,
    /// The name of the industrial design.
    /// This corresponds to android.os.Build.DEVICE.
    #[prost(string, tag = "10")]
    pub codename: ::prost::alloc::string::String,
    /// Whether this device is virtual or physical.
    #[prost(enumeration = "DeviceForm", tag = "4")]
    pub form: i32,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[prost(enumeration = "DeviceFormFactor", tag = "16")]
    pub form_factor: i32,
    /// Version-specific information of an Android model.
    #[prost(message, repeated, tag = "21")]
    pub per_version_info: ::prost::alloc::vec::Vec<PerAndroidVersionInfo>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[prost(int32, tag = "5")]
    pub screen_x: i32,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[prost(int32, tag = "6")]
    pub screen_y: i32,
    /// Screen density in DPI.
    /// This corresponds to ro.sf.lcd_density
    #[prost(int32, tag = "12")]
    pub screen_density: i32,
    /// True if and only if tests with this model are recorded by stitching
    /// together screenshots. See use_low_spec_video_recording in device config.
    #[prost(bool, tag = "17")]
    pub low_fps_video_recording: bool,
    /// The set of Android versions this device supports.
    #[prost(string, repeated, tag = "7")]
    pub supported_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of supported ABIs for this device.
    /// This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level
    /// 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2.
    /// The most preferred ABI is the first element in the list.
    ///
    /// Elements are optionally prefixed by "version_id:" (where version_id is
    /// the id of an AndroidVersion), denoting an ABI that is supported only on
    /// a particular version.
    #[prost(string, repeated, tag = "11")]
    pub supported_abis: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL of a thumbnail image (photo) of the device.
    #[prost(string, tag = "19")]
    pub thumbnail_url: ::prost::alloc::string::String,
}
/// A version of the Android OS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidVersion {
    /// An opaque id for this Android version.
    /// Use this id to invoke the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A string representing this version of the Android OS.
    /// Examples: "4.3", "4.4".
    #[prost(string, tag = "2")]
    pub version_string: ::prost::alloc::string::String,
    /// The API level for this Android version.
    /// Examples: 18, 19.
    #[prost(int32, tag = "3")]
    pub api_level: i32,
    /// The code name for this Android version.
    /// Examples: "JellyBean", "KitKat".
    #[prost(string, tag = "4")]
    pub code_name: ::prost::alloc::string::String,
    /// The date this Android version became available in the market.
    #[prost(message, optional, tag = "5")]
    pub release_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// Market share for this version.
    #[prost(message, optional, tag = "6")]
    pub distribution: ::core::option::Option<Distribution>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A version-specific information of an Android model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerAndroidVersionInfo {
    /// An Android version.
    #[prost(string, tag = "1")]
    pub version_id: ::prost::alloc::string::String,
    /// The number of online devices for an Android version.
    #[prost(enumeration = "DeviceCapacity", tag = "2")]
    pub device_capacity: i32,
    /// Output only. The estimated wait time for a single interactive device
    /// session using Direct Access.
    #[prost(message, optional, tag = "3")]
    pub interactive_device_availability_estimate: ::core::option::Option<
        ::prost_types::Duration,
    >,
    /// Output only. Identifies supported clients for DirectAccess for this Android
    /// version.
    #[prost(message, optional, tag = "4")]
    pub direct_access_version_info: ::core::option::Option<DirectAccessVersionInfo>,
}
/// Denotes whether Direct Access is supported, and by which client versions.
///
/// DirectAccessService is currently available as a preview to select developers.
/// You can register today on behalf of you and your team at
/// <https://developer.android.com/studio/preview/android-device-streaming>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectAccessVersionInfo {
    /// Whether direct access is supported at all. Clients are expected to
    /// filter down the device list to only android models and versions which
    /// support Direct Access when that is the user intent.
    #[prost(bool, tag = "1")]
    pub direct_access_supported: bool,
    /// Output only. Indicates client-device compatibility, where a device is known
    /// to work only with certain workarounds implemented in the Android Studio
    /// client. Expected format "major.minor.micro.patch", e.g.
    /// "5921.22.2211.8881706".
    #[prost(string, tag = "2")]
    pub minimum_android_studio_version: ::prost::alloc::string::String,
}
/// Data about the relative number of devices running a
/// given configuration of the Android platform.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// Output only. The time this distribution was measured.
    #[prost(message, optional, tag = "1")]
    pub measurement_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The estimated fraction (0-1) of the total market with this
    /// configuration.
    #[prost(double, tag = "2")]
    pub market_share: f64,
}
/// The currently supported iOS devices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDeviceCatalog {
    /// The set of supported iOS device models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<IosModel>,
    /// The set of supported iOS software versions.
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<IosVersion>,
    /// The set of supported Xcode versions.
    #[prost(message, repeated, tag = "4")]
    pub xcode_versions: ::prost::alloc::vec::Vec<XcodeVersion>,
    /// The set of supported runtime configurations.
    #[prost(message, optional, tag = "3")]
    pub runtime_configuration: ::core::option::Option<IosRuntimeConfiguration>,
}
/// iOS configuration that can be selected at the time a test is run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosRuntimeConfiguration {
    /// The set of available locales.
    #[prost(message, repeated, tag = "1")]
    pub locales: ::prost::alloc::vec::Vec<Locale>,
    /// The set of available orientations.
    #[prost(message, repeated, tag = "2")]
    pub orientations: ::prost::alloc::vec::Vec<Orientation>,
}
/// A description of an iOS device tests may be run on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosModel {
    /// The unique opaque id for this model.
    /// Use this for invoking the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The human-readable name for this device model.
    /// Examples: "iPhone 4s", "iPad Mini 2".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The set of iOS major software versions this device supports.
    #[prost(string, repeated, tag = "3")]
    pub supported_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Device capabilities.
    /// Copied from
    /// <https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html>
    #[prost(string, repeated, tag = "5")]
    pub device_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[prost(int32, tag = "7")]
    pub screen_x: i32,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[prost(int32, tag = "8")]
    pub screen_y: i32,
    /// Screen density in DPI.
    #[prost(int32, tag = "9")]
    pub screen_density: i32,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[prost(enumeration = "DeviceFormFactor", tag = "6")]
    pub form_factor: i32,
    /// Version-specific information of an iOS model.
    #[prost(message, repeated, tag = "14")]
    pub per_version_info: ::prost::alloc::vec::Vec<PerIosVersionInfo>,
}
/// An iOS version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosVersion {
    /// An opaque id for this iOS version.
    /// Use this id to invoke the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// An integer representing the major iOS version.
    /// Examples: "8", "9".
    #[prost(int32, tag = "2")]
    pub major_version: i32,
    /// An integer representing the minor iOS version.
    /// Examples: "1", "2".
    #[prost(int32, tag = "4")]
    pub minor_version: i32,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The available Xcode versions for this version.
    #[prost(string, repeated, tag = "5")]
    pub supported_xcode_version_ids: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// A version-specific information of an iOS model.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerIosVersionInfo {
    /// An iOS version.
    #[prost(string, tag = "1")]
    pub version_id: ::prost::alloc::string::String,
    /// The number of online devices for an iOS version.
    #[prost(enumeration = "DeviceCapacity", tag = "2")]
    pub device_capacity: i32,
}
/// A location/region designation for language.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locale {
    /// The id for this locale.
    /// Example: "en_US".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A human-friendly name for this language/locale.
    /// Example: "English".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// A human-friendly string representing the region for this
    /// locale. Example: "United States". Not present for every locale.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
    /// Tags for this dimension.
    /// Example: "default".
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Screen orientation of the device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orientation {
    /// The id for this orientation.
    /// Example: "portrait".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A human-friendly name for this orientation.
    /// Example: "portrait".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Tags for this dimension.
    /// Example: "default".
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// An Xcode version that an iOS version is compatible with.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XcodeVersion {
    /// The id for this version.
    /// Example: "9.2".
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Tags for this Xcode version.
    /// Example: "default".
    #[prost(string, repeated, tag = "2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfigurationCatalog {
    #[prost(message, repeated, tag = "1")]
    pub configurations: ::prost::alloc::vec::Vec<NetworkConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfiguration {
    /// The unique opaque id for this network traffic configuration.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The emulation rule applying to the upload traffic.
    #[prost(message, optional, tag = "2")]
    pub up_rule: ::core::option::Option<TrafficRule>,
    /// The emulation rule applying to the download traffic.
    #[prost(message, optional, tag = "3")]
    pub down_rule: ::core::option::Option<TrafficRule>,
}
/// Network emulation parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficRule {
    /// Packet delay, must be >= 0.
    #[prost(message, optional, tag = "1")]
    pub delay: ::core::option::Option<::prost_types::Duration>,
    /// Packet loss ratio (0.0 - 1.0).
    #[prost(float, tag = "2")]
    pub packet_loss_ratio: f32,
    /// Packet duplication ratio (0.0 - 1.0).
    #[prost(float, tag = "3")]
    pub packet_duplication_ratio: f32,
    /// Bandwidth in kbits/second.
    #[prost(float, tag = "4")]
    pub bandwidth: f32,
    /// Burst size in kbits.
    #[prost(float, tag = "5")]
    pub burst: f32,
}
/// The currently provided software environment on the devices under test.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvidedSoftwareCatalog {
    /// Deprecated: Use AndroidX Test Orchestrator going forward.
    ///
    /// A string representing the current version of Android Test Orchestrator
    /// that is used in the environment. The package is available at
    /// <https://maven.google.com/web/index.html#com.android.support.test:orchestrator.>
    #[deprecated]
    #[prost(string, tag = "1")]
    pub orchestrator_version: ::prost::alloc::string::String,
    /// A string representing the current version of AndroidX Test Orchestrator
    /// that is used in the environment. The package is available at
    /// <https://maven.google.com/web/index.html#androidx.test:orchestrator.>
    #[prost(string, tag = "2")]
    pub androidx_orchestrator_version: ::prost::alloc::string::String,
}
/// Whether the device is physical or virtual.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceForm {
    /// Do not use.  For proto versioning only.
    Unspecified = 0,
    /// Android virtual device using Compute Engine native virtualization. Firebase
    /// Test Lab only.
    Virtual = 1,
    /// Actual hardware.
    Physical = 2,
    /// Android virtual device using emulator in nested virtualization. Equivalent
    /// to Android Studio.
    Emulator = 3,
}
impl DeviceForm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceForm::Unspecified => "DEVICE_FORM_UNSPECIFIED",
            DeviceForm::Virtual => "VIRTUAL",
            DeviceForm::Physical => "PHYSICAL",
            DeviceForm::Emulator => "EMULATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEVICE_FORM_UNSPECIFIED" => Some(Self::Unspecified),
            "VIRTUAL" => Some(Self::Virtual),
            "PHYSICAL" => Some(Self::Physical),
            "EMULATOR" => Some(Self::Emulator),
            _ => None,
        }
    }
}
/// The form factor of a device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceFormFactor {
    /// Do not use. For proto versioning only.
    Unspecified = 0,
    /// This device has the shape of a phone.
    Phone = 1,
    /// This device has the shape of a tablet.
    Tablet = 2,
    /// This device has the shape of a watch or other wearable.
    Wearable = 3,
}
impl DeviceFormFactor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceFormFactor::Unspecified => "DEVICE_FORM_FACTOR_UNSPECIFIED",
            DeviceFormFactor::Phone => "PHONE",
            DeviceFormFactor::Tablet => "TABLET",
            DeviceFormFactor::Wearable => "WEARABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEVICE_FORM_FACTOR_UNSPECIFIED" => Some(Self::Unspecified),
            "PHONE" => Some(Self::Phone),
            "TABLET" => Some(Self::Tablet),
            "WEARABLE" => Some(Self::Wearable),
            _ => None,
        }
    }
}
/// Capacity based on the number of online devices in the lab.
///
/// Important: device capacity does not directly reflect the length of the
/// queue at a moment in time. It does not take into account current traffic or
/// the state of the devices.
///
/// For physical devices, the number is the average of online devices in the last
/// 30 days.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceCapacity {
    /// The value of device capacity is unknown or unset.
    Unspecified = 0,
    /// Devices that are high in capacity (The lab has a large number of these
    /// devices).
    ///
    /// These devices are generally suggested for running a large number of
    /// simultaneous tests (e.g. more than 100 tests).
    ///
    /// Please note that high capacity devices do not guarantee short wait times
    /// due to several factors:
    /// 1. Traffic (how heavily they are used at any given moment)
    /// 2. High capacity devices are prioritized for certain usages, which may
    /// cause user tests to be slower than selecting other similar device types.
    High = 1,
    /// Devices that are medium in capacity (The lab has a decent number of these
    /// devices, though not as many as high capacity devices).
    ///
    /// These devices are suitable for fewer test runs (e.g. fewer than 100 tests)
    /// and only for low shard counts (e.g. less than 10 shards).
    Medium = 2,
    /// Devices that are low in capacity (The lab has a small number of these
    /// devices).
    ///
    /// These devices may be used if users need to test on this specific device
    /// model and version. Please note that due to low capacity, the tests may take
    /// much longer to finish, especially if a large number of tests are invoked at
    /// once. These devices are not suitable for test sharding.
    Low = 3,
    /// Devices that are completely missing from the lab.
    ///
    /// These devices are unavailable either temporarily or permanently and should
    /// not be requested. If the device is also marked as deprecated, this state
    /// is very likely permanent.
    None = 4,
}
impl DeviceCapacity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceCapacity::Unspecified => "DEVICE_CAPACITY_UNSPECIFIED",
            DeviceCapacity::High => "DEVICE_CAPACITY_HIGH",
            DeviceCapacity::Medium => "DEVICE_CAPACITY_MEDIUM",
            DeviceCapacity::Low => "DEVICE_CAPACITY_LOW",
            DeviceCapacity::None => "DEVICE_CAPACITY_NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEVICE_CAPACITY_UNSPECIFIED" => Some(Self::Unspecified),
            "DEVICE_CAPACITY_HIGH" => Some(Self::High),
            "DEVICE_CAPACITY_MEDIUM" => Some(Self::Medium),
            "DEVICE_CAPACITY_LOW" => Some(Self::Low),
            "DEVICE_CAPACITY_NONE" => Some(Self::None),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod test_environment_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for discovering environments supported by the TestExecutionService.
    ///
    /// Over time the TestService may add or remove devices or configuration options
    /// (e.g., when new devices and APIs are released).  Clients should check here
    /// periodically to discover what options are supported.
    ///
    /// It defines the following resource model:
    ///
    /// - The API a collection of [TestEnvironmentCatalog]
    ///   [google.devtools.test.v1.TestEnvironmentCatalog] resources, named
    ///   `testEnvironmentCatalog/*`
    ///
    /// - Each TestEnvironmentCatalog resource describes a set of supported
    ///   environments.
    ///
    /// - An [AndroidDeviceCatalog][google.devtools.test.v1.AndroidDeviceCatalog]
    ///   describes supported Android devices. It contains lists of supported
    ///   [AndroidModels][google.devtools.test.v1.AndroidModel] and
    ///   [AndroidVersions][google.devtools.test.v1.AndroidVersion] along with a
    ///   [AndroidRuntimeConfiguration][google.devtools.test.v1.AndroidRuntimeConfiguration].
    ///   Each AndroidModel contains a list of Versions it supports. All
    ///   models support all locales and orientations described by the
    ///   AndroidRuntimeConfiguration
    ///
    /// - An [IosDeviceCatalog][google.devtools.test.v1.IosDeviceCatalog]
    ///   describes supported iOS devices. It contains lists of supported
    ///   [IosModels][google.devtools.test.v1.IosModel] and
    ///   [IosVersions][google.devtools.test.v1.IosVersion] along with a
    ///   [IosRuntimeConfiguration][google.devtools.test.v1.IosRuntimeConfiguration].
    ///   Each IosModel contains a list of Versions it supports. All
    ///   models support all locales and orientations described by the
    ///   IosRuntimeConfiguration.
    #[derive(Debug, Clone)]
    pub struct TestEnvironmentDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TestEnvironmentDiscoveryServiceClient<T>
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
        ) -> TestEnvironmentDiscoveryServiceClient<InterceptedService<T, F>>
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
            TestEnvironmentDiscoveryServiceClient::new(
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
        /// Gets the catalog of supported test environments.
        ///
        /// May return any of the following canonical error codes:
        ///
        /// - INVALID_ARGUMENT - if the request is malformed
        /// - NOT_FOUND - if the environment type does not exist
        /// - INTERNAL - if an internal error occurred
        pub async fn get_test_environment_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestEnvironmentCatalogRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TestEnvironmentCatalog>,
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
                "/google.devtools.testing.v1.TestEnvironmentDiscoveryService/GetTestEnvironmentCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.TestEnvironmentDiscoveryService",
                        "GetTestEnvironmentCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A Request for the device session from the session service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeviceSessionRequest {
    /// Required. The Compute Engine project under which this device will be
    /// allocated. "projects/{project_id}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A device session to create.
    #[prost(message, optional, tag = "2")]
    pub device_session: ::core::option::Option<DeviceSession>,
}
/// Request a list of device sessions in the provided parent matching the given
/// filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceSessionsRequest {
    /// Required. The name of the parent to request, e.g. "projects/{project_id}"
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of DeviceSessions to return.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A continuation token for paging.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. If specified, responses will be filtered by the given filter.
    /// Allowed fields are: session_state.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// A list of device sessions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeviceSessionsResponse {
    /// The sessions matching the specified filter in the given cloud project.
    #[prost(message, repeated, tag = "1")]
    pub device_sessions: ::prost::alloc::vec::Vec<DeviceSession>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request object for a Device Session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeviceSessionRequest {
    /// Required. Name of the DeviceSession, e.g.
    /// "projects/{project_id}/deviceSessions/{session_id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for cancelling a Device Session.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDeviceSessionRequest {
    /// Required. Name of the DeviceSession, e.g.
    /// "projects/{project_id}/deviceSessions/{session_id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for the UpdateDeviceSession RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeviceSessionRequest {
    /// Required. DeviceSession to update.
    /// The device session's `name` field is used to identify the session to update
    /// "projects/{project_id}/deviceSessions/{session_id}"
    #[prost(message, optional, tag = "1")]
    pub device_session: ::core::option::Option<DeviceSession>,
    /// Required. The list of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Protobuf message describing the device message, used from several RPCs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSession {
    /// Optional. Name of the DeviceSession, e.g.
    /// "projects/{project_id}/deviceSessions/{session_id}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The title of the DeviceSession to be presented in the UI.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Current state of the DeviceSession.
    #[prost(enumeration = "device_session::SessionState", tag = "3")]
    pub state: i32,
    /// Output only. The historical state transitions of the session_state message
    /// including the current session state.
    #[prost(message, repeated, tag = "14")]
    pub state_histories: ::prost::alloc::vec::Vec<device_session::SessionStateEvent>,
    /// Output only. The interval of time that this device must be interacted with
    /// before it transitions from ACTIVE to TIMEOUT_INACTIVITY.
    #[prost(message, optional, tag = "7")]
    pub inactivity_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The time that the Session was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp that the session first became ACTIVE.
    #[prost(message, optional, tag = "9")]
    pub active_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The requested device
    #[prost(message, optional, tag = "15")]
    pub android_device: ::core::option::Option<AndroidDevice>,
    #[prost(oneof = "device_session::Expiration", tags = "13, 5")]
    pub expiration: ::core::option::Option<device_session::Expiration>,
}
/// Nested message and enum types in `DeviceSession`.
pub mod device_session {
    /// A message encapsulating a series of Session states and the time that the
    /// DeviceSession first entered those states.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionStateEvent {
        /// Output only. The session_state tracked by this event
        #[prost(enumeration = "SessionState", tag = "1")]
        pub session_state: i32,
        /// Output only. The time that the session_state first encountered that
        /// state.
        #[prost(message, optional, tag = "2")]
        pub event_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. A human-readable message to explain the state.
        #[prost(string, tag = "3")]
        pub state_message: ::prost::alloc::string::String,
    }
    /// The state that the device session resides.
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
    pub enum SessionState {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Initial state of a session request. The session is being validated for
        /// correctness and a device is not yet requested.
        Requested = 1,
        /// The session has been validated and is in the queue for a device.
        Pending = 2,
        /// The session has been granted and the device is accepting
        /// connections.
        Active = 3,
        /// The session duration exceeded the device’s reservation time period and
        /// timed out automatically.
        Expired = 4,
        /// The user is finished with the session and it was canceled by the user
        /// while the request was still getting allocated or after allocation and
        /// during device usage period.
        Finished = 5,
        /// Unable to complete the session because the device was unavailable and
        /// it failed to allocate through the scheduler. For example, a device not
        /// in the catalog was requested or the request expired in the allocation
        /// queue.
        Unavailable = 6,
        /// Unable to complete the session for an internal reason, such as an
        /// infrastructure failure.
        Error = 7,
    }
    impl SessionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SessionState::Unspecified => "SESSION_STATE_UNSPECIFIED",
                SessionState::Requested => "REQUESTED",
                SessionState::Pending => "PENDING",
                SessionState::Active => "ACTIVE",
                SessionState::Expired => "EXPIRED",
                SessionState::Finished => "FINISHED",
                SessionState::Unavailable => "UNAVAILABLE",
                SessionState::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SESSION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "REQUESTED" => Some(Self::Requested),
                "PENDING" => Some(Self::Pending),
                "ACTIVE" => Some(Self::Active),
                "EXPIRED" => Some(Self::Expired),
                "FINISHED" => Some(Self::Finished),
                "UNAVAILABLE" => Some(Self::Unavailable),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// Optional. The amount of time that a device will be initially allocated
        /// for. This can eventually be extended with the UpdateDeviceSession RPC.
        /// Default: 30 minutes.
        #[prost(message, tag = "13")]
        Ttl(::prost_types::Duration),
        /// Optional. If the device is still in use at this time, any connections
        /// will be ended and the SessionState will transition from ACTIVE to
        /// FINISHED.
        #[prost(message, tag = "5")]
        ExpireTime(::prost_types::Timestamp),
    }
}
/// Generated client implementations.
pub mod direct_access_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service for allocating devices and interacting with the live-allocated
    /// devices.
    ///
    /// This service is part of Firebase Test Lab. To learn about how to use the
    /// product, and how to integrate it with your system,
    /// visit https://firebase.google.com/docs/test-lab.
    ///
    /// Each Session will wait for available capacity, at a higher
    /// priority over Test Execution. When allocated, the session will be exposed
    /// through a stream for integration.
    ///
    /// DirectAccessService is currently available as a preview to select developers.
    /// You can register today on behalf of you and your team at
    /// https://developer.android.com/studio/preview/android-device-streaming
    #[derive(Debug, Clone)]
    pub struct DirectAccessServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DirectAccessServiceClient<T>
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
        ) -> DirectAccessServiceClient<InterceptedService<T, F>>
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
            DirectAccessServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// POST /v1/projects/{project_id}/deviceSessions
        pub async fn create_device_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeviceSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::DeviceSession>, tonic::Status> {
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
                "/google.devtools.testing.v1.DirectAccessService/CreateDeviceSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.DirectAccessService",
                        "CreateDeviceSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GET /v1/projects/{project_id}/deviceSessions
        /// Lists device Sessions owned by the project user.
        pub async fn list_device_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeviceSessionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDeviceSessionsResponse>,
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
                "/google.devtools.testing.v1.DirectAccessService/ListDeviceSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.DirectAccessService",
                        "ListDeviceSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GET /v1/projects/{project_id}/deviceSessions/{device_session_id}
        /// Return a DeviceSession, which documents the allocation status and
        /// whether the device is allocated. Clients making requests from this API
        /// must poll GetDeviceSession.
        pub async fn get_device_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeviceSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::DeviceSession>, tonic::Status> {
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
                "/google.devtools.testing.v1.DirectAccessService/GetDeviceSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.DirectAccessService",
                        "GetDeviceSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// POST
        /// /v1/projects/{project_id}/deviceSessions/{device_session_id}:cancel
        /// Changes the DeviceSession to state FINISHED and terminates all connections.
        /// Canceled sessions are not deleted and can be retrieved or
        /// listed by the user until they expire based on the 28 day deletion policy.
        pub async fn cancel_device_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelDeviceSessionRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.devtools.testing.v1.DirectAccessService/CancelDeviceSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.DirectAccessService",
                        "CancelDeviceSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PATCH
        /// /v1/projects/{projectId}/deviceSessions/deviceSessionId}:updateDeviceSession
        /// Updates the current device session to the fields described by the
        /// update_mask.
        pub async fn update_device_session(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeviceSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::DeviceSession>, tonic::Status> {
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
                "/google.devtools.testing.v1.DirectAccessService/UpdateDeviceSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.DirectAccessService",
                        "UpdateDeviceSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exposes ADB connection for use with the Adb Device Forwarder project
        /// if the reserved device supports ADB.
        /// gRPC headers are used to authenticate the Connect RPC, as well as
        /// associate to a particular device session.
        /// In particular, the user must specify the "X-FTL-Session-Name" header.
        pub async fn adb_connect(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::AdbMessage>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DeviceMessage>>,
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
                "/google.devtools.testing.v1.DirectAccessService/AdbConnect",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.testing.v1.DirectAccessService",
                        "AdbConnect",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
