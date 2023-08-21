/// A detailed representation of a Yum artifact.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumArtifact {
    /// Output only. The Artifact Registry resource name of the artifact.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The yum package name of the artifact.
    #[prost(string, tag = "2")]
    pub package_name: ::prost::alloc::string::String,
    /// Output only. An artifact is a binary or source package.
    #[prost(enumeration = "yum_artifact::PackageType", tag = "3")]
    pub package_type: i32,
    /// Output only. Operating system architecture of the artifact.
    #[prost(string, tag = "4")]
    pub architecture: ::prost::alloc::string::String,
}
/// Nested message and enum types in `YumArtifact`.
pub mod yum_artifact {
    /// Package type is either binary or source.
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
    pub enum PackageType {
        /// Package type is not specified.
        Unspecified = 0,
        /// Binary package (.rpm).
        Binary = 1,
        /// Source package (.srpm).
        Source = 2,
    }
    impl PackageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PackageType::Unspecified => "PACKAGE_TYPE_UNSPECIFIED",
                PackageType::Binary => "BINARY",
                PackageType::Source => "SOURCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PACKAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BINARY" => Some(Self::Binary),
                "SOURCE" => Some(Self::Source),
                _ => None,
            }
        }
    }
}
/// Google Cloud Storage location where the artifacts currently reside.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportYumArtifactsGcsSource {
    /// Cloud Storage paths URI (e.g., gs://my_bucket//my_object).
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Supports URI wildcards for matching multiple objects from a single URI.
    #[prost(bool, tag = "2")]
    pub use_wildcards: bool,
}
/// The request to import new yum artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportYumArtifactsRequest {
    /// The name of the parent resource where the artifacts will be imported.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The source location of the package binaries.
    #[prost(oneof = "import_yum_artifacts_request::Source", tags = "2")]
    pub source: ::core::option::Option<import_yum_artifacts_request::Source>,
}
/// Nested message and enum types in `ImportYumArtifactsRequest`.
pub mod import_yum_artifacts_request {
    /// The source location of the package binaries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Google Cloud Storage location where input content is located.
        #[prost(message, tag = "2")]
        GcsSource(super::ImportYumArtifactsGcsSource),
    }
}
/// Error information explaining why a package was not imported.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportYumArtifactsErrorInfo {
    /// The detailed error status.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The source that was not imported.
    #[prost(oneof = "import_yum_artifacts_error_info::Source", tags = "1")]
    pub source: ::core::option::Option<import_yum_artifacts_error_info::Source>,
}
/// Nested message and enum types in `ImportYumArtifactsErrorInfo`.
pub mod import_yum_artifacts_error_info {
    /// The source that was not imported.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Google Cloud Storage location requested.
        #[prost(message, tag = "1")]
        GcsSource(super::ImportYumArtifactsGcsSource),
    }
}
/// The response message from importing YUM artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportYumArtifactsResponse {
    /// The yum artifacts imported.
    #[prost(message, repeated, tag = "1")]
    pub yum_artifacts: ::prost::alloc::vec::Vec<YumArtifact>,
    /// Detailed error info for packages that were not imported.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<ImportYumArtifactsErrorInfo>,
}
/// The operation metadata for importing artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportYumArtifactsMetadata {}
/// The Artifact Registry VPC SC config that apply to a Project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcscConfig {
    /// The name of the project's VPC SC Config.
    ///
    /// Always of the form:
    /// projects/{projectID}/locations/{location}/vpcscConfig
    ///
    /// In update request: never set
    /// In response: always set
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The project per location VPC SC policy that defines the VPC SC behavior for
    /// the Remote Repository (Allow/Deny).
    #[prost(enumeration = "vpcsc_config::VpcscPolicy", tag = "2")]
    pub vpcsc_policy: i32,
}
/// Nested message and enum types in `VPCSCConfig`.
pub mod vpcsc_config {
    /// VPCSCPolicy is the VPC SC policy for project and location.
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
    pub enum VpcscPolicy {
        /// VPCSC_POLICY_UNSPECIFIED - the VPS SC policy is not defined.
        /// When VPS SC policy is not defined - the Service will use the default
        /// behavior (VPCSC_DENY).
        Unspecified = 0,
        /// VPCSC_DENY - repository will block the requests to the Upstreams for the
        /// Remote Repositories if the resource is in the perimeter.
        Deny = 1,
        /// VPCSC_ALLOW - repository will allow the requests to the Upstreams for the
        /// Remote Repositories if the resource is in the perimeter.
        Allow = 2,
    }
    impl VpcscPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VpcscPolicy::Unspecified => "VPCSC_POLICY_UNSPECIFIED",
                VpcscPolicy::Deny => "DENY",
                VpcscPolicy::Allow => "ALLOW",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VPCSC_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                "DENY" => Some(Self::Deny),
                "ALLOW" => Some(Self::Allow),
                _ => None,
            }
        }
    }
}
/// Gets the VPC SC config for a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVpcscConfigRequest {
    /// Required. The name of the VPCSCConfig resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Sets the VPCSC config of the project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVpcscConfigRequest {
    /// The project config.
    #[prost(message, optional, tag = "1")]
    pub vpcsc_config: ::core::option::Option<VpcscConfig>,
    /// Field mask to support partial updates.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A detailed representation of an Apt artifact. Information in the record
/// is derived from the archive's control file.
/// See <https://www.debian.org/doc/debian-policy/ch-controlfields.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptArtifact {
    /// Output only. The Artifact Registry resource name of the artifact.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The Apt package name of the artifact.
    #[prost(string, tag = "2")]
    pub package_name: ::prost::alloc::string::String,
    /// Output only. An artifact is a binary or source package.
    #[prost(enumeration = "apt_artifact::PackageType", tag = "3")]
    pub package_type: i32,
    /// Output only. Operating system architecture of the artifact.
    #[prost(string, tag = "4")]
    pub architecture: ::prost::alloc::string::String,
    /// Output only. Repository component of the artifact.
    #[prost(string, tag = "5")]
    pub component: ::prost::alloc::string::String,
    /// Output only. Contents of the artifact's control metadata file.
    #[prost(bytes = "bytes", tag = "6")]
    pub control_file: ::prost::bytes::Bytes,
}
/// Nested message and enum types in `AptArtifact`.
pub mod apt_artifact {
    /// Package type is either binary or source.
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
    pub enum PackageType {
        /// Package type is not specified.
        Unspecified = 0,
        /// Binary package.
        Binary = 1,
        /// Source package.
        Source = 2,
    }
    impl PackageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PackageType::Unspecified => "PACKAGE_TYPE_UNSPECIFIED",
                PackageType::Binary => "BINARY",
                PackageType::Source => "SOURCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PACKAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BINARY" => Some(Self::Binary),
                "SOURCE" => Some(Self::Source),
                _ => None,
            }
        }
    }
}
/// Google Cloud Storage location where the artifacts currently reside.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAptArtifactsGcsSource {
    /// Cloud Storage paths URI (e.g., gs://my_bucket//my_object).
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Supports URI wildcards for matching multiple objects from a single URI.
    #[prost(bool, tag = "2")]
    pub use_wildcards: bool,
}
/// The request to import new apt artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAptArtifactsRequest {
    /// The name of the parent resource where the artifacts will be imported.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The source location of the package binaries.
    #[prost(oneof = "import_apt_artifacts_request::Source", tags = "2")]
    pub source: ::core::option::Option<import_apt_artifacts_request::Source>,
}
/// Nested message and enum types in `ImportAptArtifactsRequest`.
pub mod import_apt_artifacts_request {
    /// The source location of the package binaries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Google Cloud Storage location where input content is located.
        #[prost(message, tag = "2")]
        GcsSource(super::ImportAptArtifactsGcsSource),
    }
}
/// Error information explaining why a package was not imported.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAptArtifactsErrorInfo {
    /// The detailed error status.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The source that was not imported.
    #[prost(oneof = "import_apt_artifacts_error_info::Source", tags = "1")]
    pub source: ::core::option::Option<import_apt_artifacts_error_info::Source>,
}
/// Nested message and enum types in `ImportAptArtifactsErrorInfo`.
pub mod import_apt_artifacts_error_info {
    /// The source that was not imported.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Google Cloud Storage location requested.
        #[prost(message, tag = "1")]
        GcsSource(super::ImportAptArtifactsGcsSource),
    }
}
/// The response message from importing APT artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAptArtifactsResponse {
    /// The Apt artifacts imported.
    #[prost(message, repeated, tag = "1")]
    pub apt_artifacts: ::prost::alloc::vec::Vec<AptArtifact>,
    /// Detailed error info for packages that were not imported.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<ImportAptArtifactsErrorInfo>,
}
/// The operation metadata for importing artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAptArtifactsMetadata {}
/// DockerImage represents a docker artifact.
/// The following fields are returned as untyped metadata in the Version
/// resource, using camelcase keys (i.e. metadata.imageSizeBytes):
/// * imageSizeBytes
/// * mediaType
/// * buildTime
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerImage {
    /// Required. registry_location, project_id, repository_name and image id forms
    /// a unique image
    /// name:`projects/<project_id>/locations/<location>/repository/<repository_name>/dockerImages/<docker_image>`.
    /// For example,
    /// "projects/test-project/locations/us-west4/repositories/test-repo/dockerImages/
    /// nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf",
    /// where "us-west4" is the registry_location, "test-project" is the
    /// project_id, "test-repo" is the repository_name and
    /// "nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf"
    /// is the image's digest.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. URL to access the image.
    /// Example:
    /// us-west4-docker.pkg.dev/test-project/test-repo/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Tags attached to this image.
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Calculated size of the image.
    /// This field is returned as the 'metadata.imageSizeBytes' field in the
    /// Version resource.
    #[prost(int64, tag = "4")]
    pub image_size_bytes: i64,
    /// Time the image was uploaded.
    #[prost(message, optional, tag = "5")]
    pub upload_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Media type of this image, e.g.
    /// "application/vnd.docker.distribution.manifest.v2+json".
    /// This field is returned as the 'metadata.mediaType' field in the
    /// Version resource.
    #[prost(string, tag = "6")]
    pub media_type: ::prost::alloc::string::String,
    /// The time this image was built.
    /// This field is returned as the 'metadata.buildTime' field in the
    /// Version resource.
    /// The build time is returned to the client as an RFC 3339 string, which can
    /// be easily used with the JavaScript Date constructor.
    #[prost(message, optional, tag = "7")]
    pub build_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the docker image was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list docker images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDockerImagesRequest {
    /// Required. The name of the parent resource whose docker images will be
    /// listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of artifacts to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The field to order the results by.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response from listing docker images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDockerImagesResponse {
    /// The docker images returned.
    #[prost(message, repeated, tag = "1")]
    pub docker_images: ::prost::alloc::vec::Vec<DockerImage>,
    /// The token to retrieve the next page of artifacts, or empty if there are no
    /// more artifacts to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to get docker images.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDockerImageRequest {
    /// Required. The name of the docker images.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// MavenArtifact represents a maven artifact.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MavenArtifact {
    /// Required. registry_location, project_id, repository_name and maven_artifact
    /// forms a unique artifact For example,
    /// "projects/test-project/locations/us-west4/repositories/test-repo/mavenArtifacts/
    /// com.google.guava:guava:31.0-jre",
    /// where "us-west4" is the registry_location, "test-project" is the
    /// project_id, "test-repo" is the repository_name and
    /// "com.google.guava:guava:31.0-jre"
    /// is the maven artifact.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. URL to access the pom file of the artifact.
    /// Example:
    /// us-west4-maven.pkg.dev/test-project/test-repo/com/google/guava/guava/31.0/guava-31.0.pom
    #[prost(string, tag = "2")]
    pub pom_uri: ::prost::alloc::string::String,
    /// Group ID for the artifact.
    /// Example:
    /// com.google.guava
    #[prost(string, tag = "3")]
    pub group_id: ::prost::alloc::string::String,
    /// Artifact ID for the artifact.
    #[prost(string, tag = "4")]
    pub artifact_id: ::prost::alloc::string::String,
    /// Version of this artifact.
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    /// Output only. Time the artifact was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the artifact was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list maven artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMavenArtifactsRequest {
    /// Required. The name of the parent resource whose maven artifacts will be
    /// listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of artifacts to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing maven artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMavenArtifactsResponse {
    /// The maven artifacts returned.
    #[prost(message, repeated, tag = "1")]
    pub maven_artifacts: ::prost::alloc::vec::Vec<MavenArtifact>,
    /// The token to retrieve the next page of artifacts, or empty if there are no
    /// more artifacts to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to get maven artifacts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMavenArtifactRequest {
    /// Required. The name of the maven artifact.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// NpmPackage represents an npm artifact.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NpmPackage {
    /// Required. registry_location, project_id, repository_name and npm_package
    /// forms a unique package For example,
    /// "projects/test-project/locations/us-west4/repositories/test-repo/npmPackages/
    /// npm_test:1.0.0",
    /// where "us-west4" is the registry_location, "test-project" is the
    /// project_id, "test-repo" is the repository_name and
    /// npm_test:1.0.0" is the npm package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Package for the artifact.
    #[prost(string, tag = "3")]
    pub package_name: ::prost::alloc::string::String,
    /// Version of this package.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Tags attached to this package.
    #[prost(string, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Time the package was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the package was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list npm packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNpmPackagesRequest {
    /// Required. The name of the parent resource whose npm packages will be
    /// listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of artifacts to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing npm packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNpmPackagesResponse {
    /// The npm packages returned.
    #[prost(message, repeated, tag = "1")]
    pub npm_packages: ::prost::alloc::vec::Vec<NpmPackage>,
    /// The token to retrieve the next page of artifacts, or empty if there are no
    /// more artifacts to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to get npm packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNpmPackageRequest {
    /// Required. The name of the npm package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// PythonPackage represents a python artifact.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PythonPackage {
    /// Required. registry_location, project_id, repository_name and python_package
    /// forms a unique package
    /// name:`projects/<project_id>/locations/<location>/repository/<repository_name>/pythonPackages/<python_package>`.
    /// For example,
    /// "projects/test-project/locations/us-west4/repositories/test-repo/pythonPackages/
    /// python_package:1.0.0",
    /// where "us-west4" is the registry_location, "test-project" is the
    /// project_id, "test-repo" is the repository_name and
    /// python_package:1.0.0" is the python package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. URL to access the package.
    /// Example:
    /// us-west4-python.pkg.dev/test-project/test-repo/python_package/file-name-1.0.0.tar.gz
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Package for the artifact.
    #[prost(string, tag = "3")]
    pub package_name: ::prost::alloc::string::String,
    /// Version of this package.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Output only. Time the package was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the package was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list python packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPythonPackagesRequest {
    /// Required. The name of the parent resource whose python packages will be
    /// listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of artifacts to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing python packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPythonPackagesResponse {
    /// The python packages returned.
    #[prost(message, repeated, tag = "1")]
    pub python_packages: ::prost::alloc::vec::Vec<PythonPackage>,
    /// The token to retrieve the next page of artifacts, or empty if there are no
    /// more artifacts to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to get python packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPythonPackageRequest {
    /// Required. The name of the python package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A hash of file content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    /// The algorithm used to compute the hash value.
    #[prost(enumeration = "hash::HashType", tag = "1")]
    pub r#type: i32,
    /// The hash value.
    #[prost(bytes = "bytes", tag = "2")]
    pub value: ::prost::bytes::Bytes,
}
/// Nested message and enum types in `Hash`.
pub mod hash {
    /// The algorithm used to compute the hash.
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
    pub enum HashType {
        /// Unspecified.
        Unspecified = 0,
        /// SHA256 hash.
        Sha256 = 1,
        /// MD5 hash.
        Md5 = 2,
    }
    impl HashType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HashType::Unspecified => "HASH_TYPE_UNSPECIFIED",
                HashType::Sha256 => "SHA256",
                HashType::Md5 => "MD5",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HASH_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SHA256" => Some(Self::Sha256),
                "MD5" => Some(Self::Md5),
                _ => None,
            }
        }
    }
}
/// Files store content that is potentially associated with Packages or Versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// The name of the file, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1/files/a%2Fb%2Fc.txt".
    /// If the file ID part contains slashes, they are escaped.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The size of the File in bytes.
    #[prost(int64, tag = "3")]
    pub size_bytes: i64,
    /// The hashes of the file content.
    #[prost(message, repeated, tag = "4")]
    pub hashes: ::prost::alloc::vec::Vec<Hash>,
    /// Output only. The time when the File was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the File was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The name of the Package or Version that owns this file, if any.
    #[prost(string, tag = "7")]
    pub owner: ::prost::alloc::string::String,
    /// Output only. The time when the last attempt to refresh the file's data was
    /// made. Only set when the repository is remote.
    #[prost(message, optional, tag = "8")]
    pub fetch_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list files.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFilesRequest {
    /// Required. The name of the repository whose files will be listed. For
    /// example: "projects/p1/locations/us-central1/repositories/repo1
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. Filter rules are
    /// case insensitive. The fields eligible for filtering are:
    ///
    ///    * `name`
    ///    * `owner`
    ///
    ///   An example of using a filter:
    ///
    ///    * `name="projects/p1/locations/us-central1/repositories/repo1/files/a/b/*"` --> Files with an
    ///    ID starting with "a/b/".
    ///    * `owner="projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/1.0"` -->
    ///    Files owned by the version `1.0` in package `pkg1`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of files to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The field to order the results by.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response from listing files.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFilesResponse {
    /// The files returned.
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<File>,
    /// The token to retrieve the next page of files, or empty if there are no
    /// more files to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to retrieve a file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileRequest {
    /// Required. The name of the file to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Packages are named collections of versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// The name of the package, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1".
    /// If the package ID part contains slashes, the slashes are escaped.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The display name of the package.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The time when the package was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the package was last updated. This includes publishing a new
    /// version of the package.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPackagesRequest {
    /// Required. The name of the parent resource whose packages will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of packages to return. Maximum page size is 1,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPackagesResponse {
    /// The packages returned.
    #[prost(message, repeated, tag = "1")]
    pub packages: ::prost::alloc::vec::Vec<Package>,
    /// The token to retrieve the next page of packages, or empty if there are no
    /// more packages to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to retrieve a package.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageRequest {
    /// Required. The name of the package to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to delete a package.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePackageRequest {
    /// Required. The name of the package to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A Repository for storing artifacts with a specific format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repository {
    /// The name of the repository, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The format of packages that are stored in the repository.
    #[prost(enumeration = "repository::Format", tag = "2")]
    pub format: i32,
    /// The user-provided description of the repository.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Labels with user-defined metadata.
    /// This field may contain up to 64 entries. Label keys and values may be no
    /// longer than 63 characters. Label keys must begin with a lowercase letter
    /// and may only contain lowercase letters, numeric characters, underscores,
    /// and dashes.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The time when the repository was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the repository was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The Cloud KMS resource name of the customer managed encryption key that's
    /// used to encrypt the contents of the Repository. Has the form:
    /// `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`.
    /// This value may not be changed after the Repository has been created.
    #[prost(string, tag = "8")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Repository-specific configurations.
    #[prost(oneof = "repository::FormatConfig", tags = "9")]
    pub format_config: ::core::option::Option<repository::FormatConfig>,
}
/// Nested message and enum types in `Repository`.
pub mod repository {
    /// MavenRepositoryConfig is maven related repository details.
    /// Provides additional configuration details for repositories of the maven
    /// format type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MavenRepositoryConfig {
        /// The repository with this flag will allow publishing
        /// the same snapshot versions.
        #[prost(bool, tag = "1")]
        pub allow_snapshot_overwrites: bool,
        /// Version policy defines the versions that the registry will accept.
        #[prost(enumeration = "maven_repository_config::VersionPolicy", tag = "2")]
        pub version_policy: i32,
    }
    /// Nested message and enum types in `MavenRepositoryConfig`.
    pub mod maven_repository_config {
        /// VersionPolicy is the version policy for the repository.
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
        pub enum VersionPolicy {
            /// VERSION_POLICY_UNSPECIFIED - the version policy is not defined.
            /// When the version policy is not defined, no validation is performed
            /// for the versions.
            Unspecified = 0,
            /// RELEASE - repository will accept only Release versions.
            Release = 1,
            /// SNAPSHOT - repository will accept only Snapshot versions.
            Snapshot = 2,
        }
        impl VersionPolicy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    VersionPolicy::Unspecified => "VERSION_POLICY_UNSPECIFIED",
                    VersionPolicy::Release => "RELEASE",
                    VersionPolicy::Snapshot => "SNAPSHOT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "VERSION_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                    "RELEASE" => Some(Self::Release),
                    "SNAPSHOT" => Some(Self::Snapshot),
                    _ => None,
                }
            }
        }
    }
    /// A package format.
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
    pub enum Format {
        /// Unspecified package format.
        Unspecified = 0,
        /// Docker package format.
        Docker = 1,
        /// Maven package format.
        Maven = 2,
        /// NPM package format.
        Npm = 3,
        /// APT package format.
        Apt = 5,
        /// YUM package format.
        Yum = 6,
        /// Python package format.
        Python = 8,
    }
    impl Format {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Format::Unspecified => "FORMAT_UNSPECIFIED",
                Format::Docker => "DOCKER",
                Format::Maven => "MAVEN",
                Format::Npm => "NPM",
                Format::Apt => "APT",
                Format::Yum => "YUM",
                Format::Python => "PYTHON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "DOCKER" => Some(Self::Docker),
                "MAVEN" => Some(Self::Maven),
                "NPM" => Some(Self::Npm),
                "APT" => Some(Self::Apt),
                "YUM" => Some(Self::Yum),
                "PYTHON" => Some(Self::Python),
                _ => None,
            }
        }
    }
    /// Repository-specific configurations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FormatConfig {
        /// Maven repository config contains repository level configuration
        /// for the repositories of maven type.
        #[prost(message, tag = "9")]
        MavenConfig(MavenRepositoryConfig),
    }
}
/// The request to list repositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesRequest {
    /// Required. The name of the parent resource whose repositories will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of repositories to return. Maximum page size is 1,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing repositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesResponse {
    /// The repositories returned.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// The token to retrieve the next page of repositories, or empty if there are
    /// no more repositories to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to retrieve a repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepositoryRequest {
    /// Required. The name of the repository to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to create a new repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepositoryRequest {
    /// Required. The name of the parent resource where the repository will be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The repository id to use for this repository.
    #[prost(string, tag = "2")]
    pub repository_id: ::prost::alloc::string::String,
    /// The repository to be created.
    #[prost(message, optional, tag = "3")]
    pub repository: ::core::option::Option<Repository>,
}
/// The request to update a repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRepositoryRequest {
    /// The repository that replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub repository: ::core::option::Option<Repository>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request to delete a repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepositoryRequest {
    /// Required. The name of the repository to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The Artifact Registry settings that apply to a Project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectSettings {
    /// The name of the project's settings.
    ///
    /// Always of the form:
    /// projects/{project-id}/projectSettings
    ///
    /// In update request: never set
    /// In response: always set
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The redirection state of the legacy repositories in this project.
    #[prost(enumeration = "project_settings::RedirectionState", tag = "2")]
    pub legacy_redirection_state: i32,
}
/// Nested message and enum types in `ProjectSettings`.
pub mod project_settings {
    /// The possible redirection states for legacy repositories.
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
    pub enum RedirectionState {
        /// No redirection status has been set.
        Unspecified = 0,
        /// Redirection is disabled.
        RedirectionFromGcrIoDisabled = 1,
        /// Redirection is enabled.
        RedirectionFromGcrIoEnabled = 2,
        /// Redirection is enabled, and has been finalized so cannot be reverted.
        RedirectionFromGcrIoFinalized = 3,
    }
    impl RedirectionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RedirectionState::Unspecified => "REDIRECTION_STATE_UNSPECIFIED",
                RedirectionState::RedirectionFromGcrIoDisabled => {
                    "REDIRECTION_FROM_GCR_IO_DISABLED"
                }
                RedirectionState::RedirectionFromGcrIoEnabled => {
                    "REDIRECTION_FROM_GCR_IO_ENABLED"
                }
                RedirectionState::RedirectionFromGcrIoFinalized => {
                    "REDIRECTION_FROM_GCR_IO_FINALIZED"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REDIRECTION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "REDIRECTION_FROM_GCR_IO_DISABLED" => {
                    Some(Self::RedirectionFromGcrIoDisabled)
                }
                "REDIRECTION_FROM_GCR_IO_ENABLED" => {
                    Some(Self::RedirectionFromGcrIoEnabled)
                }
                "REDIRECTION_FROM_GCR_IO_FINALIZED" => {
                    Some(Self::RedirectionFromGcrIoFinalized)
                }
                _ => None,
            }
        }
    }
}
/// Gets the redirection status for a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectSettingsRequest {
    /// Required. The name of the projectSettings resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Sets the settings of the project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectSettingsRequest {
    /// The project settings.
    #[prost(message, optional, tag = "2")]
    pub project_settings: ::core::option::Option<ProjectSettings>,
    /// Field mask to support partial updates.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Tags point to a version and represent an alternative name that can be used
/// to access the version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// The name of the tag, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1".
    /// If the package part contains slashes, the slashes are escaped.
    /// The tag part can only have characters in \[a-zA-Z0-9\-._~:@\], anything else
    /// must be URL encoded.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the version the tag refers to, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811"
    /// If the package or version ID parts contain slashes, the slashes are
    /// escaped.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// The request to list tags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// The name of the parent resource whose tags will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. Filter rules are
    /// case insensitive. The fields eligible for filtering are:
    ///
    ///    * `version`
    ///
    ///   An example of using a filter:
    ///
    ///    * `version="projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/1.0"`
    ///    --> Tags that are applied to the version `1.0` in package `pkg1`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of tags to return. Maximum page size is 10,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing tags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsResponse {
    /// The tags returned.
    #[prost(message, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    /// The token to retrieve the next page of tags, or empty if there are no
    /// more tags to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to retrieve a tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagRequest {
    /// The name of the tag to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to create a new tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    /// The name of the parent resource where the tag will be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The tag id to use for this repository.
    #[prost(string, tag = "2")]
    pub tag_id: ::prost::alloc::string::String,
    /// The tag to be created.
    #[prost(message, optional, tag = "3")]
    pub tag: ::core::option::Option<Tag>,
}
/// The request to create or update a tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagRequest {
    /// The tag that replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub tag: ::core::option::Option<Tag>,
    /// The update mask applies to the resource. For the `FieldMask` definition,
    /// see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request to delete a tag.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagRequest {
    /// The name of the tag to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The body of a version resource. A version resource represents a
/// collection of components, such as files and other data. This may correspond
/// to a version in many package management schemes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// The name of the version, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/art1".
    /// If the package or version ID parts contain slashes, the slashes are
    /// escaped.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Description of the version, as specified in its metadata.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The time when the version was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the version was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A list of related tags. Will contain up to 100 tags that
    /// reference this version.
    #[prost(message, repeated, tag = "7")]
    pub related_tags: ::prost::alloc::vec::Vec<Tag>,
    /// Output only. Repository-specific Metadata stored against this version.
    /// The fields returned are defined by the underlying repository-specific
    /// resource. Currently, the resources could be:
    /// \[DockerImage][google.devtools.artifactregistry.v1.DockerImage\]
    /// \[MavenArtifact][google.devtools.artifactregistry.v1.MavenArtifact\]
    #[prost(message, optional, tag = "8")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
}
/// The request to list versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// The name of the parent resource whose versions will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of versions to return. Maximum page size is 1,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The view that should be returned in the response.
    #[prost(enumeration = "VersionView", tag = "4")]
    pub view: i32,
    /// Optional. The field to order the results by.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response from listing versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// The versions returned.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
    /// The token to retrieve the next page of versions, or empty if there are no
    /// more versions to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to retrieve a version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// The name of the version to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The view that should be returned in the response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
}
/// The request to delete a version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// The name of the version to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// By default, a version that is tagged may not be deleted. If force=true, the
    /// version and any tags pointing to the version are deleted.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// The metadata of an LRO from deleting multiple versions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteVersionsMetadata {
    /// The versions the operation failed to delete.
    #[prost(string, repeated, tag = "2")]
    pub failed_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The view, which determines what version information is returned in a
/// response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VersionView {
    /// The default / unset value.
    /// The API will default to the BASIC view.
    Unspecified = 0,
    /// Includes basic information about the version, but not any related tags.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl VersionView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VersionView::Unspecified => "VERSION_VIEW_UNSPECIFIED",
            VersionView::Basic => "BASIC",
            VersionView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VERSION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Metadata type for longrunning-operations, currently empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {}
/// Generated client implementations.
pub mod artifact_registry_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Artifact Registry API service.
    ///
    /// Artifact Registry is an artifact management system for storing artifacts
    /// from different package management systems.
    ///
    /// The resources managed by this API are:
    ///
    /// * Repositories, which group packages and their data.
    /// * Packages, which group versions and their tags.
    /// * Versions, which are specific forms of a package.
    /// * Tags, which represent alternative names for versions.
    /// * Files, which contain content and are optionally associated with a Package
    ///   or Version.
    #[derive(Debug, Clone)]
    pub struct ArtifactRegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ArtifactRegistryClient<T>
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
        ) -> ArtifactRegistryClient<InterceptedService<T, F>>
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
            ArtifactRegistryClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists docker images.
        pub async fn list_docker_images(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDockerImagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDockerImagesResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListDockerImages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListDockerImages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a docker image.
        pub async fn get_docker_image(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDockerImageRequest>,
        ) -> std::result::Result<tonic::Response<super::DockerImage>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetDockerImage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetDockerImage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists maven artifacts.
        pub async fn list_maven_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMavenArtifactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMavenArtifactsResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListMavenArtifacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListMavenArtifacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a maven artifact.
        pub async fn get_maven_artifact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMavenArtifactRequest>,
        ) -> std::result::Result<tonic::Response<super::MavenArtifact>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetMavenArtifact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetMavenArtifact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists npm packages.
        pub async fn list_npm_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNpmPackagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNpmPackagesResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListNpmPackages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListNpmPackages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a npm package.
        pub async fn get_npm_package(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNpmPackageRequest>,
        ) -> std::result::Result<tonic::Response<super::NpmPackage>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetNpmPackage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetNpmPackage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists python packages.
        pub async fn list_python_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPythonPackagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPythonPackagesResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListPythonPackages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListPythonPackages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a python package.
        pub async fn get_python_package(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPythonPackageRequest>,
        ) -> std::result::Result<tonic::Response<super::PythonPackage>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetPythonPackage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetPythonPackage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports Apt artifacts. The returned Operation will complete once the
        /// resources are imported. Package, Version, and File resources are created
        /// based on the imported artifacts. Imported artifacts that conflict with
        /// existing resources are ignored.
        pub async fn import_apt_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportAptArtifactsRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ImportAptArtifacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ImportAptArtifacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Imports Yum (RPM) artifacts. The returned Operation will complete once the
        /// resources are imported. Package, Version, and File resources are created
        /// based on the imported artifacts. Imported artifacts that conflict with
        /// existing resources are ignored.
        pub async fn import_yum_artifacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportYumArtifactsRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ImportYumArtifacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ImportYumArtifacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists repositories.
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRepositoriesResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListRepositories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListRepositories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a repository.
        pub async fn get_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a repository. The returned Operation will finish once the
        /// repository has been created. Its response will be the created Repository.
        pub async fn create_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepositoryRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/CreateRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "CreateRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a repository.
        pub async fn update_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/UpdateRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "UpdateRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a repository and all of its contents. The returned Operation will
        /// finish once the repository has been deleted. It will not have any Operation
        /// metadata and will return a google.protobuf.Empty response.
        pub async fn delete_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepositoryRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/DeleteRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "DeleteRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists packages.
        pub async fn list_packages(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPackagesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPackagesResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListPackages",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListPackages",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a package.
        pub async fn get_package(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPackageRequest>,
        ) -> std::result::Result<tonic::Response<super::Package>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetPackage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetPackage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a package and all of its versions and tags. The returned operation
        /// will complete once the package has been deleted.
        pub async fn delete_package(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePackageRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/DeletePackage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "DeletePackage",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists versions.
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVersionsResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListVersions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a version
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> std::result::Result<tonic::Response<super::Version>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a version and all of its content. The returned operation will
        /// complete once the version has been deleted.
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/DeleteVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "DeleteVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists files.
        pub async fn list_files(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFilesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFilesResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListFiles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListFiles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a file.
        pub async fn get_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileRequest>,
        ) -> std::result::Result<tonic::Response<super::File>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetFile",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists tags.
        pub async fn list_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTagsResponse>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListTags",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "ListTags",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a tag.
        pub async fn get_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagRequest>,
        ) -> std::result::Result<tonic::Response<super::Tag>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a tag.
        pub async fn create_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagRequest>,
        ) -> std::result::Result<tonic::Response<super::Tag>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/CreateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "CreateTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates a tag.
        pub async fn update_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagRequest>,
        ) -> std::result::Result<tonic::Response<super::Tag>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/UpdateTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "UpdateTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a tag.
        pub async fn delete_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagRequest>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/DeleteTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "DeleteTag",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the IAM policy for a given resource.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM policy for a given resource.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Tests if the caller has a list of permissions on a resource.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the Settings for the Project.
        pub async fn get_project_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProjectSettings>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetProjectSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetProjectSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the Settings for the Project.
        pub async fn update_project_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectSettingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProjectSettings>,
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/UpdateProjectSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "UpdateProjectSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves the VPCSC Config for the Project.
        pub async fn get_vpcsc_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVpcscConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::VpcscConfig>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetVPCSCConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "GetVPCSCConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the VPCSC Config for the Project.
        pub async fn update_vpcsc_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVpcscConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::VpcscConfig>, tonic::Status> {
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
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/UpdateVPCSCConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.devtools.artifactregistry.v1.ArtifactRegistry",
                        "UpdateVPCSCConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
