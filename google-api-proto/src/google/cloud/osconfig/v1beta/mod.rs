/// An OS Config resource representing a guest configuration policy. These
/// policies represent the desired state for VM instance guest environments
/// including packages to install or remove, package repository configurations,
/// and software to install.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestPolicy {
    /// Required. Unique name of the resource in this project using one of the following
    /// forms:
    /// `projects/{project_number}/guestPolicies/{guest_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the guest policy. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time this guest policy was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last time this guest policy was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Specifies the VM instances that are assigned to this policy. This allows
    /// you to target sets or groups of VM instances by different parameters such
    /// as labels, names, OS, or zones.
    ///
    /// If left empty, all VM instances underneath this policy are targeted.
    ///
    /// At the same level in the resource hierarchy (that is within a project), the
    /// service prevents the creation of multiple policies that conflict with
    /// each other. For more information, see how the service [handles assignment
    /// conflicts](/compute/docs/os-config-management/create-guest-policy#handle-conflicts).
    #[prost(message, optional, tag = "6")]
    pub assignment: ::core::option::Option<Assignment>,
    /// The software packages to be managed by this policy.
    #[prost(message, repeated, tag = "7")]
    pub packages: ::prost::alloc::vec::Vec<Package>,
    /// A list of package repositories to configure on the VM instance. This is
    /// done before any other configs are applied so they can use these repos.
    /// Package repositories are only configured if the corresponding package
    /// manager(s) are available.
    #[prost(message, repeated, tag = "8")]
    pub package_repositories: ::prost::alloc::vec::Vec<PackageRepository>,
    /// A list of Recipes to install on the VM instance.
    #[prost(message, repeated, tag = "9")]
    pub recipes: ::prost::alloc::vec::Vec<SoftwareRecipe>,
    /// The etag for this guest policy.
    /// If this is provided on update, it must match the server's etag.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
}
/// An assignment represents the group or groups of VM instances that the policy
/// applies to.
///
/// If an assignment is empty, it applies to all VM instances. Otherwise, the
/// targeted VM instances must meet all the criteria specified. So if both
/// labels and zones are specified, the policy applies to VM instances with those
/// labels and in those zones.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assignment {
    /// Targets instances matching at least one of these label sets. This allows
    /// an assignment to target disparate groups, for example "env=prod or
    /// env=staging".
    #[prost(message, repeated, tag = "1")]
    pub group_labels: ::prost::alloc::vec::Vec<assignment::GroupLabel>,
    /// Targets instances in any of these zones. Leave empty to target instances
    /// in any zone.
    ///
    /// Zonal targeting is uncommon and is supported to facilitate the management
    /// of changes by zone.
    #[prost(string, repeated, tag = "2")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets any of the instances specified. Instances are specified by their
    /// URI in the form `zones/\[ZONE]/instances/[INSTANCE_NAME\]`.
    ///
    /// Instance targeting is uncommon and is supported to facilitate the
    /// management of changes by the instance or to target specific VM instances
    /// for development and testing.
    ///
    /// Only supported for project-level policies and must reference instances
    /// within this project.
    #[prost(string, repeated, tag = "3")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets VM instances whose name starts with one of these prefixes.
    ///
    /// Like labels, this is another way to group VM instances when targeting
    /// configs, for example prefix="prod-".
    ///
    /// Only supported for project-level policies.
    #[prost(string, repeated, tag = "4")]
    pub instance_name_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets VM instances matching at least one of the following OS types.
    ///
    /// VM instances must match all supplied criteria for a given OsType to be
    /// included.
    #[prost(message, repeated, tag = "5")]
    pub os_types: ::prost::alloc::vec::Vec<assignment::OsType>,
}
/// Nested message and enum types in `Assignment`.
pub mod assignment {
    /// Represents a group of VM intances that can be identified as having all
    /// these labels, for example "env=prod and app=web".
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupLabel {
        /// Google Compute Engine instance labels that must be present for an
        /// instance to be included in this assignment group.
        #[prost(btree_map = "string, string", tag = "1")]
        pub labels: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// Defines the criteria for selecting VM Instances by OS type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsType {
        /// Targets VM instances with OS Inventory enabled and having the following
        /// OS short name, for example "debian" or "windows".
        #[prost(string, tag = "1")]
        pub os_short_name: ::prost::alloc::string::String,
        /// Targets VM instances with OS Inventory enabled and having the following
        /// following OS version.
        #[prost(string, tag = "2")]
        pub os_version: ::prost::alloc::string::String,
        /// Targets VM instances with OS Inventory enabled and having the following
        /// OS architecture.
        #[prost(string, tag = "3")]
        pub os_architecture: ::prost::alloc::string::String,
    }
}
/// Package is a reference to the software package to be installed or removed.
/// The agent on the VM instance uses the system package manager to apply the
/// config.
///
///
/// These are the commands that the agent uses to install or remove
/// packages.
///
/// Apt
/// install: `apt-get update && apt-get -y install package1 package2 package3`
/// remove: `apt-get -y remove package1 package2 package3`
///
/// Yum
/// install: `yum -y install package1 package2 package3`
/// remove: `yum -y remove package1 package2 package3`
///
/// Zypper
/// install: `zypper install package1 package2 package3`
/// remove: `zypper rm package1 package2`
///
/// Googet
/// install: `googet -noconfirm install package1 package2 package3`
/// remove: `googet -noconfirm remove package1 package2 package3`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// Required. The name of the package. A package is uniquely identified for conflict
    /// validation by checking the package name and the manager(s) that the
    /// package targets.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The desired_state the agent should maintain for this package. The
    /// default is to ensure the package is installed.
    #[prost(enumeration = "DesiredState", tag = "2")]
    pub desired_state: i32,
    /// Type of package manager that can be used to install this package.
    /// If a system does not have the package manager, the package is not
    /// installed or removed no error message is returned. By default,
    /// or if you specify `ANY`,
    /// the agent attempts to install and remove this package using the default
    /// package manager. This is useful when creating a policy that applies to
    /// different types of systems.
    ///
    /// The default behavior is ANY.
    #[prost(enumeration = "package::Manager", tag = "3")]
    pub manager: i32,
}
/// Nested message and enum types in `Package`.
pub mod package {
    /// Types of package managers that may be used to manage this package.
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
    pub enum Manager {
        /// The default behavior is ANY.
        Unspecified = 0,
        /// Apply this package config using the default system package manager.
        Any = 1,
        /// Apply this package config only if Apt is available on the system.
        Apt = 2,
        /// Apply this package config only if Yum is available on the system.
        Yum = 3,
        /// Apply this package config only if Zypper is available on the system.
        Zypper = 4,
        /// Apply this package config only if GooGet is available on the system.
        Goo = 5,
    }
    impl Manager {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Manager::Unspecified => "MANAGER_UNSPECIFIED",
                Manager::Any => "ANY",
                Manager::Apt => "APT",
                Manager::Yum => "YUM",
                Manager::Zypper => "ZYPPER",
                Manager::Goo => "GOO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MANAGER_UNSPECIFIED" => Some(Self::Unspecified),
                "ANY" => Some(Self::Any),
                "APT" => Some(Self::Apt),
                "YUM" => Some(Self::Yum),
                "ZYPPER" => Some(Self::Zypper),
                "GOO" => Some(Self::Goo),
                _ => None,
            }
        }
    }
}
/// Represents a single Apt package repository. This repository is added to
/// a repo file that is stored at
/// `/etc/apt/sources.list.d/google_osconfig.list`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptRepository {
    /// Type of archive files in this repository. The default behavior is DEB.
    #[prost(enumeration = "apt_repository::ArchiveType", tag = "1")]
    pub archive_type: i32,
    /// Required. URI for this repository.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Required. Distribution of this repository.
    #[prost(string, tag = "3")]
    pub distribution: ::prost::alloc::string::String,
    /// Required. List of components for this repository. Must contain at least one item.
    #[prost(string, repeated, tag = "4")]
    pub components: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URI of the key file for this repository. The agent maintains
    /// a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg` containing
    /// all the keys in any applied guest policy.
    #[prost(string, tag = "5")]
    pub gpg_key: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AptRepository`.
pub mod apt_repository {
    /// Type of archive.
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
    pub enum ArchiveType {
        /// Unspecified.
        Unspecified = 0,
        /// DEB indicates that the archive contains binary files.
        Deb = 1,
        /// DEB_SRC indicates that the archive contains source files.
        DebSrc = 2,
    }
    impl ArchiveType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ArchiveType::Unspecified => "ARCHIVE_TYPE_UNSPECIFIED",
                ArchiveType::Deb => "DEB",
                ArchiveType::DebSrc => "DEB_SRC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ARCHIVE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEB" => Some(Self::Deb),
                "DEB_SRC" => Some(Self::DebSrc),
                _ => None,
            }
        }
    }
}
/// Represents a single Yum package repository. This repository is added to a
/// repo file that is stored at `/etc/yum.repos.d/google_osconfig.repo`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumRepository {
    /// Required. A one word, unique name for this repository. This is
    /// the `repo id` in the Yum config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for guest policy conflicts.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the repository.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The location of the repository directory.
    #[prost(string, tag = "3")]
    pub base_url: ::prost::alloc::string::String,
    /// URIs of GPG keys.
    #[prost(string, repeated, tag = "4")]
    pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a single Zypper package repository. This repository is added to a
/// repo file that is stored at `/etc/zypp/repos.d/google_osconfig.repo`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperRepository {
    /// Required. A one word, unique name for this repository. This is
    /// the `repo id` in the zypper config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for guest policy conflicts.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the repository.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The location of the repository directory.
    #[prost(string, tag = "3")]
    pub base_url: ::prost::alloc::string::String,
    /// URIs of GPG keys.
    #[prost(string, repeated, tag = "4")]
    pub gpg_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Goo package repository. These is added to a repo file
/// that is stored at C:/ProgramData/GooGet/repos/google_osconfig.repo.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooRepository {
    /// Required. The name of the repository.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The url of the repository.
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
/// A package repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageRepository {
    /// A specific type of repository.
    #[prost(oneof = "package_repository::Repository", tags = "1, 2, 3, 4")]
    pub repository: ::core::option::Option<package_repository::Repository>,
}
/// Nested message and enum types in `PackageRepository`.
pub mod package_repository {
    /// A specific type of repository.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Repository {
        /// An Apt Repository.
        #[prost(message, tag = "1")]
        Apt(super::AptRepository),
        /// A Yum Repository.
        #[prost(message, tag = "2")]
        Yum(super::YumRepository),
        /// A Zypper Repository.
        #[prost(message, tag = "3")]
        Zypper(super::ZypperRepository),
        /// A Goo Repository.
        #[prost(message, tag = "4")]
        Goo(super::GooRepository),
    }
}
/// A software recipe is a set of instructions for installing and configuring a
/// piece of software. It consists of a set of artifacts that are
/// downloaded, and a set of steps that install, configure, and/or update the
/// software.
///
/// Recipes support installing and updating software from artifacts in the
/// following formats:
/// Zip archive, Tar archive, Windows MSI, Debian package, and RPM package.
///
/// Additionally, recipes support executing a script (either defined in a file or
/// directly in this api) in bash, sh, cmd, and powershell.
///
/// Updating a software recipe
///
/// If a recipe is assigned to an instance and there is a recipe with the same
/// name but a lower version already installed and the assigned state
/// of the recipe is `UPDATED`, then the recipe is updated to
/// the new version.
///
/// Script Working Directories
///
/// Each script or execution step is run in its own temporary directory which
/// is deleted after completing the step.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareRecipe {
    /// Required. Unique identifier for the recipe. Only one recipe with a given name is
    /// installed on an instance.
    ///
    /// Names are also used to identify resources which helps to determine whether
    /// guest policies have conflicts. This means that requests to create multiple
    /// recipes with the same name and version are rejected since they
    /// could potentially have conflicting assignments.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The version of this software recipe. Version can be up to 4 period
    /// separated numbers (e.g. 12.34.56.78).
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Resources available to be used in the steps in the recipe.
    #[prost(message, repeated, tag = "3")]
    pub artifacts: ::prost::alloc::vec::Vec<software_recipe::Artifact>,
    /// Actions to be taken for installing this recipe. On failure it stops
    /// executing steps and does not attempt another installation. Any steps taken
    /// (including partially completed steps) are not rolled back.
    #[prost(message, repeated, tag = "4")]
    pub install_steps: ::prost::alloc::vec::Vec<software_recipe::Step>,
    /// Actions to be taken for updating this recipe. On failure it stops
    /// executing steps and  does not attempt another update for this recipe. Any
    /// steps taken (including partially completed steps) are not rolled back.
    #[prost(message, repeated, tag = "5")]
    pub update_steps: ::prost::alloc::vec::Vec<software_recipe::Step>,
    /// Default is INSTALLED. The desired state the agent should maintain for this
    /// recipe.
    ///
    /// INSTALLED: The software recipe is installed on the instance but
    ///             won't be updated to new versions.
    /// UPDATED: The software recipe is installed on the instance. The recipe is
    ///           updated to a higher version, if a higher version of the recipe is
    ///           assigned to this instance.
    /// REMOVE: Remove is unsupported for software recipes and attempts to
    ///          create or update a recipe to the REMOVE state is rejected.
    #[prost(enumeration = "DesiredState", tag = "6")]
    pub desired_state: i32,
}
/// Nested message and enum types in `SoftwareRecipe`.
pub mod software_recipe {
    /// Specifies a resource to be used in the recipe.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Artifact {
        /// Required. Id of the artifact, which the installation and update steps of this
        /// recipe can reference. Artifacts in a recipe cannot have the same id.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Defaults to false. When false, recipes are subject to validations
        /// based on the artifact type:
        ///
        /// Remote: A checksum must be specified, and only protocols with
        /// transport-layer security are permitted.
        /// GCS:    An object generation number must be specified.
        #[prost(bool, tag = "4")]
        pub allow_insecure: bool,
        /// A specific type of artifact.
        #[prost(oneof = "artifact::Artifact", tags = "2, 3")]
        pub artifact: ::core::option::Option<artifact::Artifact>,
    }
    /// Nested message and enum types in `Artifact`.
    pub mod artifact {
        /// Specifies an artifact available via some URI.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Remote {
            /// URI from which to fetch the object. It should contain both the protocol
            /// and path following the format {protocol}://{location}.
            #[prost(string, tag = "1")]
            pub uri: ::prost::alloc::string::String,
            /// Must be provided if `allow_insecure` is `false`.
            /// SHA256 checksum in hex format, to compare to the checksum of the
            /// artifact. If the checksum is not empty and it doesn't match the
            /// artifact then the recipe installation fails before running any of the
            /// steps.
            #[prost(string, tag = "2")]
            pub checksum: ::prost::alloc::string::String,
        }
        /// Specifies an artifact available as a Google Cloud Storage object.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Gcs {
            /// Bucket of the Google Cloud Storage object.
            /// Given an example URL:
            /// `<https://storage.googleapis.com/my-bucket/foo/bar#1234567`>
            /// this value would be `my-bucket`.
            #[prost(string, tag = "1")]
            pub bucket: ::prost::alloc::string::String,
            /// Name of the Google Cloud Storage object.
            /// As specified \[here\]
            /// (<https://cloud.google.com/storage/docs/naming#objectnames>)
            /// Given an example URL:
            /// `<https://storage.googleapis.com/my-bucket/foo/bar#1234567`>
            /// this value would be `foo/bar`.
            #[prost(string, tag = "2")]
            pub object: ::prost::alloc::string::String,
            /// Must be provided if allow_insecure is false.
            /// Generation number of the Google Cloud Storage object.
            /// `<https://storage.googleapis.com/my-bucket/foo/bar#1234567`>
            /// this value would be `1234567`.
            #[prost(int64, tag = "3")]
            pub generation: i64,
        }
        /// A specific type of artifact.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Artifact {
            /// A generic remote artifact.
            #[prost(message, tag = "2")]
            Remote(Remote),
            /// A Google Cloud Storage artifact.
            #[prost(message, tag = "3")]
            Gcs(Gcs),
        }
    }
    /// An action that can be taken as part of installing or updating a recipe.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Step {
        /// A specific type of step.
        #[prost(oneof = "step::Step", tags = "1, 2, 3, 4, 5, 6, 7")]
        pub step: ::core::option::Option<step::Step>,
    }
    /// Nested message and enum types in `Step`.
    pub mod step {
        /// Copies the artifact to the specified path on the instance.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CopyFile {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: ::prost::alloc::string::String,
            /// Required. The absolute path on the instance to put the file.
            #[prost(string, tag = "2")]
            pub destination: ::prost::alloc::string::String,
            /// Whether to allow this step to overwrite existing files. If this is
            /// false and the file already exists the file is not overwritten
            /// and the step is considered a success. Defaults to false.
            #[prost(bool, tag = "3")]
            pub overwrite: bool,
            /// Consists of three octal digits which represent, in
            /// order, the permissions of the owner, group, and other users for the
            /// file (similarly to the numeric mode used in the linux chmod utility).
            /// Each digit represents a three bit number with the 4 bit
            /// corresponding to the read permissions, the 2 bit corresponds to the
            /// write bit, and the one bit corresponds to the execute permission.
            /// Default behavior is 755.
            ///
            /// Below are some examples of permissions and their associated values:
            /// read, write, and execute: 7
            /// read and execute: 5
            /// read and write: 6
            /// read only: 4
            #[prost(string, tag = "4")]
            pub permissions: ::prost::alloc::string::String,
        }
        /// Extracts an archive of the type specified in the specified directory.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExtractArchive {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: ::prost::alloc::string::String,
            /// Directory to extract archive to.
            /// Defaults to `/` on Linux or `C:\` on Windows.
            #[prost(string, tag = "2")]
            pub destination: ::prost::alloc::string::String,
            /// Required. The type of the archive to extract.
            #[prost(enumeration = "extract_archive::ArchiveType", tag = "3")]
            pub r#type: i32,
        }
        /// Nested message and enum types in `ExtractArchive`.
        pub mod extract_archive {
            /// Specifying the type of archive.
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
            pub enum ArchiveType {
                /// Indicates that the archive type isn't specified.
                Unspecified = 0,
                /// Indicates that the archive is a tar archive with no encryption.
                Tar = 1,
                /// Indicates that the archive is a tar archive with gzip encryption.
                TarGzip = 2,
                /// Indicates that the archive is a tar archive with bzip encryption.
                TarBzip = 3,
                /// Indicates that the archive is a tar archive with lzma encryption.
                TarLzma = 4,
                /// Indicates that the archive is a tar archive with xz encryption.
                TarXz = 5,
                /// Indicates that the archive is a zip archive.
                Zip = 11,
            }
            impl ArchiveType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ArchiveType::Unspecified => "ARCHIVE_TYPE_UNSPECIFIED",
                        ArchiveType::Tar => "TAR",
                        ArchiveType::TarGzip => "TAR_GZIP",
                        ArchiveType::TarBzip => "TAR_BZIP",
                        ArchiveType::TarLzma => "TAR_LZMA",
                        ArchiveType::TarXz => "TAR_XZ",
                        ArchiveType::Zip => "ZIP",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "ARCHIVE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                        "TAR" => Some(Self::Tar),
                        "TAR_GZIP" => Some(Self::TarGzip),
                        "TAR_BZIP" => Some(Self::TarBzip),
                        "TAR_LZMA" => Some(Self::TarLzma),
                        "TAR_XZ" => Some(Self::TarXz),
                        "ZIP" => Some(Self::Zip),
                        _ => None,
                    }
                }
            }
        }
        /// Installs an MSI file.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallMsi {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: ::prost::alloc::string::String,
            /// The flags to use when installing the MSI
            /// defaults to \["/i"\] (i.e. the install flag).
            #[prost(string, repeated, tag = "2")]
            pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Return codes that indicate that the software installed or updated
            /// successfully. Behaviour defaults to \[0\]
            #[prost(int32, repeated, tag = "3")]
            pub allowed_exit_codes: ::prost::alloc::vec::Vec<i32>,
        }
        /// Installs a deb via dpkg.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallDpkg {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: ::prost::alloc::string::String,
        }
        /// Installs an rpm file via the rpm utility.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallRpm {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: ::prost::alloc::string::String,
        }
        /// Executes an artifact or local file.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExecFile {
            /// Arguments to be passed to the provided executable.
            #[prost(string, repeated, tag = "3")]
            pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Defaults to \[0\]. A list of possible return values that the program
            /// can return to indicate a success.
            #[prost(int32, repeated, tag = "4")]
            pub allowed_exit_codes: ::prost::alloc::vec::Vec<i32>,
            /// Location of the file to execute.
            #[prost(oneof = "exec_file::LocationType", tags = "1, 2")]
            pub location_type: ::core::option::Option<exec_file::LocationType>,
        }
        /// Nested message and enum types in `ExecFile`.
        pub mod exec_file {
            /// Location of the file to execute.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum LocationType {
                /// The id of the relevant artifact in the recipe.
                #[prost(string, tag = "1")]
                ArtifactId(::prost::alloc::string::String),
                /// The absolute path of the file on the local filesystem.
                #[prost(string, tag = "2")]
                LocalPath(::prost::alloc::string::String),
            }
        }
        /// Runs a script through an interpreter.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RunScript {
            /// Required. The shell script to be executed.
            #[prost(string, tag = "1")]
            pub script: ::prost::alloc::string::String,
            /// Return codes that indicate that the software installed or updated
            /// successfully. Behaviour defaults to \[0\]
            #[prost(int32, repeated, tag = "2")]
            pub allowed_exit_codes: ::prost::alloc::vec::Vec<i32>,
            /// The script interpreter to use to run the script. If no interpreter is
            /// specified the script is executed directly, which likely
            /// only succeed for scripts with
            /// [shebang lines](<https://en.wikipedia.org/wiki/Shebang_\(Unix\>)).
            #[prost(enumeration = "run_script::Interpreter", tag = "3")]
            pub interpreter: i32,
        }
        /// Nested message and enum types in `RunScript`.
        pub mod run_script {
            /// The interpreter used to execute a script.
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
            pub enum Interpreter {
                /// Default value for ScriptType.
                Unspecified = 0,
                /// Indicates that the script is run with `/bin/sh` on Linux and `cmd`
                /// on windows.
                Shell = 1,
                /// Indicates that the script is run with powershell.
                Powershell = 3,
            }
            impl Interpreter {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Interpreter::Unspecified => "INTERPRETER_UNSPECIFIED",
                        Interpreter::Shell => "SHELL",
                        Interpreter::Powershell => "POWERSHELL",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "INTERPRETER_UNSPECIFIED" => Some(Self::Unspecified),
                        "SHELL" => Some(Self::Shell),
                        "POWERSHELL" => Some(Self::Powershell),
                        _ => None,
                    }
                }
            }
        }
        /// A specific type of step.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Step {
            /// Copies a file onto the instance.
            #[prost(message, tag = "1")]
            FileCopy(CopyFile),
            /// Extracts an archive into the specified directory.
            #[prost(message, tag = "2")]
            ArchiveExtraction(ExtractArchive),
            /// Installs an MSI file.
            #[prost(message, tag = "3")]
            MsiInstallation(InstallMsi),
            /// Installs a deb file via dpkg.
            #[prost(message, tag = "4")]
            DpkgInstallation(InstallDpkg),
            /// Installs an rpm file via the rpm utility.
            #[prost(message, tag = "5")]
            RpmInstallation(InstallRpm),
            /// Executes an artifact or local file.
            #[prost(message, tag = "6")]
            FileExec(ExecFile),
            /// Runs commands in a shell.
            #[prost(message, tag = "7")]
            ScriptRun(RunScript),
        }
    }
}
/// A request message for creating a guest policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuestPolicyRequest {
    /// Required. The resource name of the parent using one of the following forms:
    /// `projects/{project_number}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The logical name of the guest policy in the project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "2")]
    pub guest_policy_id: ::prost::alloc::string::String,
    /// Required. The GuestPolicy to create.
    #[prost(message, optional, tag = "3")]
    pub guest_policy: ::core::option::Option<GuestPolicy>,
}
/// A request message for retrieving a guest policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuestPolicyRequest {
    /// Required. The resource name of the guest policy using one of the following forms:
    /// `projects/{project_number}/guestPolicies/{guest_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing guest policies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuestPoliciesRequest {
    /// Required. The resource name of the parent using one of the following forms:
    /// `projects/{project_number}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of guest policies to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to `ListGuestPolicies`
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing guest policies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuestPoliciesResponse {
    /// The list of GuestPolicies.
    #[prost(message, repeated, tag = "1")]
    pub guest_policies: ::prost::alloc::vec::Vec<GuestPolicy>,
    /// A pagination token that can be used to get the next page
    /// of guest policies.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message for updating a guest policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGuestPolicyRequest {
    /// Required. The updated GuestPolicy.
    #[prost(message, optional, tag = "1")]
    pub guest_policy: ::core::option::Option<GuestPolicy>,
    /// Field mask that controls which fields of the guest policy should be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request message for deleting a guest policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGuestPolicyRequest {
    /// Required. The resource name of the guest policy  using one of the following forms:
    /// `projects/{project_number}/guestPolicies/{guest_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for getting the effective guest policy assigned to the
/// instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEffectiveGuestPolicyRequest {
    /// Required. The VM instance whose policies are being looked up.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Short name of the OS running on the instance. The OS Config agent only
    /// provides this field for targeting if OS Inventory is enabled for that
    /// instance.
    #[prost(string, tag = "2")]
    pub os_short_name: ::prost::alloc::string::String,
    /// Version of the OS running on the instance. The OS Config agent only
    /// provides this field for targeting if OS Inventory is enabled for that
    /// VM instance.
    #[prost(string, tag = "3")]
    pub os_version: ::prost::alloc::string::String,
    /// Architecture of OS running on the instance. The OS Config agent only
    /// provides this field for targeting if OS Inventory is enabled for that
    /// instance.
    #[prost(string, tag = "4")]
    pub os_architecture: ::prost::alloc::string::String,
}
/// The effective guest policy that applies to a VM instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveGuestPolicy {
    /// List of package configurations assigned to the VM instance.
    #[prost(message, repeated, tag = "1")]
    pub packages: ::prost::alloc::vec::Vec<effective_guest_policy::SourcedPackage>,
    /// List of package repository configurations assigned to the VM instance.
    #[prost(message, repeated, tag = "2")]
    pub package_repositories: ::prost::alloc::vec::Vec<
        effective_guest_policy::SourcedPackageRepository,
    >,
    /// List of recipes assigned to the VM instance.
    #[prost(message, repeated, tag = "3")]
    pub software_recipes: ::prost::alloc::vec::Vec<
        effective_guest_policy::SourcedSoftwareRecipe,
    >,
}
/// Nested message and enum types in `EffectiveGuestPolicy`.
pub mod effective_guest_policy {
    /// A guest policy package including its source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedPackage {
        /// Name of the guest policy providing this config.
        #[prost(string, tag = "1")]
        pub source: ::prost::alloc::string::String,
        /// A software package to configure on the VM instance.
        #[prost(message, optional, tag = "2")]
        pub package: ::core::option::Option<super::Package>,
    }
    /// A guest policy package repository including its source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedPackageRepository {
        /// Name of the guest policy providing this config.
        #[prost(string, tag = "1")]
        pub source: ::prost::alloc::string::String,
        /// A software package repository to configure on the VM instance.
        #[prost(message, optional, tag = "2")]
        pub package_repository: ::core::option::Option<super::PackageRepository>,
    }
    /// A guest policy recipe including its source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedSoftwareRecipe {
        /// Name of the guest policy providing this config.
        #[prost(string, tag = "1")]
        pub source: ::prost::alloc::string::String,
        /// A software recipe to configure on the VM instance.
        #[prost(message, optional, tag = "2")]
        pub software_recipe: ::core::option::Option<super::SoftwareRecipe>,
    }
}
/// The desired state that the OS Config agent maintains on the VM instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DesiredState {
    /// The default is to ensure the package is installed.
    Unspecified = 0,
    /// The agent ensures that the package is installed.
    Installed = 1,
    /// The agent ensures that the package is installed and
    /// periodically checks for and install any updates.
    Updated = 2,
    /// The agent ensures that the package is not installed and uninstall it
    /// if detected.
    Removed = 3,
}
impl DesiredState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DesiredState::Unspecified => "DESIRED_STATE_UNSPECIFIED",
            DesiredState::Installed => "INSTALLED",
            DesiredState::Updated => "UPDATED",
            DesiredState::Removed => "REMOVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DESIRED_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTALLED" => Some(Self::Installed),
            "UPDATED" => Some(Self::Updated),
            "REMOVED" => Some(Self::Removed),
            _ => None,
        }
    }
}
/// Message encapsulating a value that can be either absolute ("fixed") or
/// relative ("percent") to a value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedOrPercent {
    /// Type of the value.
    #[prost(oneof = "fixed_or_percent::Mode", tags = "1, 2")]
    pub mode: ::core::option::Option<fixed_or_percent::Mode>,
}
/// Nested message and enum types in `FixedOrPercent`.
pub mod fixed_or_percent {
    /// Type of the value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Specifies a fixed value.
        #[prost(int32, tag = "1")]
        Fixed(i32),
        /// Specifies the relative value defined as a percentage, which will be
        /// multiplied by a reference value.
        #[prost(int32, tag = "2")]
        Percent(i32),
    }
}
/// A request message to initiate patching across Compute Engine instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutePatchJobRequest {
    /// Required. The project in which to run this patch in the form `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Instances to patch, either explicitly or filtered by some criteria such
    /// as zone or labels.
    #[prost(message, optional, tag = "7")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied. If omitted, instances are
    /// patched using the default configurations.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the patch job
    /// times out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// If this patch is a dry-run only, instances are contacted but
    /// will do nothing.
    #[prost(bool, tag = "6")]
    pub dry_run: bool,
    /// Display name for this patch job. This does not have to be unique.
    #[prost(string, tag = "8")]
    pub display_name: ::prost::alloc::string::String,
    /// Rollout strategy of the patch job.
    #[prost(message, optional, tag = "9")]
    pub rollout: ::core::option::Option<PatchRollout>,
}
/// Request to get an active or completed patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list details for all instances that are part of a patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsRequest {
    /// Required. The parent for the instances are in the form of `projects/*/patchJobs/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of instance details records to return.  Default is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters results listed in the response. This
    /// field supports filtering results by instance zone, name, state, or
    /// `failure_reason`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing the instances details for a patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsResponse {
    /// A list of instance status.
    #[prost(message, repeated, tag = "1")]
    pub patch_job_instance_details: ::prost::alloc::vec::Vec<PatchJobInstanceDetails>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Patch details for a VM instance. For more information about reviewing VM
/// instance details, see
/// [Listing all VM instance details for a specific patch
/// job](<https://cloud.google.com/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJobInstanceDetails {
    /// The instance name in the form `projects/*/zones/*/instances/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique identifier for the instance. This identifier is
    /// defined by the server.
    #[prost(string, tag = "2")]
    pub instance_system_id: ::prost::alloc::string::String,
    /// Current state of instance patch.
    #[prost(enumeration = "instance::PatchState", tag = "3")]
    pub state: i32,
    /// If the patch fails, this field provides the reason.
    #[prost(string, tag = "4")]
    pub failure_reason: ::prost::alloc::string::String,
    /// The number of times the agent that the agent attempts to apply the patch.
    #[prost(int64, tag = "5")]
    pub attempt_count: i64,
}
/// A request message for listing patch jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsRequest {
    /// Required. In the form of `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of instance status to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by patch
    /// jobs to be included in the response.
    /// Currently, filtering is only available on the patch_deployment field.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing patch jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsResponse {
    /// The list of patch jobs.
    #[prost(message, repeated, tag = "1")]
    pub patch_jobs: ::prost::alloc::vec::Vec<PatchJob>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A high level representation of a patch job that is either in progress
/// or has completed.
///
/// Instance details are not included in the job. To paginate through instance
/// details, use `ListPatchJobInstanceDetails`.
///
/// For more information about patch jobs, see
/// [Creating patch
/// jobs](<https://cloud.google.com/compute/docs/os-patch-management/create-patch-job>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJob {
    /// Unique identifier for this patch job in the form
    /// `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name for this patch job. This is not a unique identifier.
    #[prost(string, tag = "14")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Time this patch job was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Last time this patch job was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current state of the PatchJob.
    #[prost(enumeration = "patch_job::State", tag = "5")]
    pub state: i32,
    /// Instances to patch.
    #[prost(message, optional, tag = "13")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied.
    #[prost(message, optional, tag = "7")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the
    /// patch job times out.
    #[prost(message, optional, tag = "8")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Summary of instance details.
    #[prost(message, optional, tag = "9")]
    pub instance_details_summary: ::core::option::Option<
        patch_job::InstanceDetailsSummary,
    >,
    /// If this patch job is a dry run, the agent reports that it has
    /// finished without running any updates on the VM instance.
    #[prost(bool, tag = "10")]
    pub dry_run: bool,
    /// If this patch job failed, this message provides information about the
    /// failure.
    #[prost(string, tag = "11")]
    pub error_message: ::prost::alloc::string::String,
    /// Reflects the overall progress of the patch job in the range of
    /// 0.0 being no progress to 100.0 being complete.
    #[prost(double, tag = "12")]
    pub percent_complete: f64,
    /// Output only. Name of the patch deployment that created this patch job.
    #[prost(string, tag = "15")]
    pub patch_deployment: ::prost::alloc::string::String,
    /// Rollout strategy being applied.
    #[prost(message, optional, tag = "16")]
    pub rollout: ::core::option::Option<PatchRollout>,
}
/// Nested message and enum types in `PatchJob`.
pub mod patch_job {
    /// A summary of the current patch state across all instances that this patch
    /// job affects. Contains counts of instances in different states. These states
    /// map to `InstancePatchState`. List patch job instance details to see the
    /// specific states of each instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceDetailsSummary {
        /// Number of instances pending patch job.
        #[prost(int64, tag = "1")]
        pub pending_instance_count: i64,
        /// Number of instances that are inactive.
        #[prost(int64, tag = "2")]
        pub inactive_instance_count: i64,
        /// Number of instances notified about patch job.
        #[prost(int64, tag = "3")]
        pub notified_instance_count: i64,
        /// Number of instances that have started.
        #[prost(int64, tag = "4")]
        pub started_instance_count: i64,
        /// Number of instances that are downloading patches.
        #[prost(int64, tag = "5")]
        pub downloading_patches_instance_count: i64,
        /// Number of instances that are applying patches.
        #[prost(int64, tag = "6")]
        pub applying_patches_instance_count: i64,
        /// Number of instances rebooting.
        #[prost(int64, tag = "7")]
        pub rebooting_instance_count: i64,
        /// Number of instances that have completed successfully.
        #[prost(int64, tag = "8")]
        pub succeeded_instance_count: i64,
        /// Number of instances that require reboot.
        #[prost(int64, tag = "9")]
        pub succeeded_reboot_required_instance_count: i64,
        /// Number of instances that failed.
        #[prost(int64, tag = "10")]
        pub failed_instance_count: i64,
        /// Number of instances that have acked and will start shortly.
        #[prost(int64, tag = "11")]
        pub acked_instance_count: i64,
        /// Number of instances that exceeded the time out while applying the patch.
        #[prost(int64, tag = "12")]
        pub timed_out_instance_count: i64,
        /// Number of instances that are running the pre-patch step.
        #[prost(int64, tag = "13")]
        pub pre_patch_step_instance_count: i64,
        /// Number of instances that are running the post-patch step.
        #[prost(int64, tag = "14")]
        pub post_patch_step_instance_count: i64,
        /// Number of instances that do not appear to be running the agent. Check to
        /// ensure that the agent is installed, running, and able to communicate with
        /// the service.
        #[prost(int64, tag = "15")]
        pub no_agent_detected_instance_count: i64,
    }
    /// Enumeration of the various states a patch job passes through as it
    /// executes.
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
    pub enum State {
        /// State must be specified.
        Unspecified = 0,
        /// The patch job was successfully initiated.
        Started = 1,
        /// The patch job is looking up instances to run the patch on.
        InstanceLookup = 2,
        /// Instances are being patched.
        Patching = 3,
        /// Patch job completed successfully.
        Succeeded = 4,
        /// Patch job completed but there were errors.
        CompletedWithErrors = 5,
        /// The patch job was canceled.
        Canceled = 6,
        /// The patch job timed out.
        TimedOut = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Started => "STARTED",
                State::InstanceLookup => "INSTANCE_LOOKUP",
                State::Patching => "PATCHING",
                State::Succeeded => "SUCCEEDED",
                State::CompletedWithErrors => "COMPLETED_WITH_ERRORS",
                State::Canceled => "CANCELED",
                State::TimedOut => "TIMED_OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STARTED" => Some(Self::Started),
                "INSTANCE_LOOKUP" => Some(Self::InstanceLookup),
                "PATCHING" => Some(Self::Patching),
                "SUCCEEDED" => Some(Self::Succeeded),
                "COMPLETED_WITH_ERRORS" => Some(Self::CompletedWithErrors),
                "CANCELED" => Some(Self::Canceled),
                "TIMED_OUT" => Some(Self::TimedOut),
                _ => None,
            }
        }
    }
}
/// Patch configuration specifications. Contains details on how to apply the
/// patch(es) to a VM instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchConfig {
    /// Post-patch reboot settings.
    #[prost(enumeration = "patch_config::RebootConfig", tag = "1")]
    pub reboot_config: i32,
    /// Apt update settings. Use this setting to override the default `apt` patch
    /// rules.
    #[prost(message, optional, tag = "3")]
    pub apt: ::core::option::Option<AptSettings>,
    /// Yum update settings. Use this setting to override the default `yum` patch
    /// rules.
    #[prost(message, optional, tag = "4")]
    pub yum: ::core::option::Option<YumSettings>,
    /// Goo update settings. Use this setting to override the default `goo` patch
    /// rules.
    #[prost(message, optional, tag = "5")]
    pub goo: ::core::option::Option<GooSettings>,
    /// Zypper update settings. Use this setting to override the default `zypper`
    /// patch rules.
    #[prost(message, optional, tag = "6")]
    pub zypper: ::core::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag = "7")]
    pub windows_update: ::core::option::Option<WindowsUpdateSettings>,
    /// The `ExecStep` to run before the patch update.
    #[prost(message, optional, tag = "8")]
    pub pre_step: ::core::option::Option<ExecStep>,
    /// The `ExecStep` to run after the patch update.
    #[prost(message, optional, tag = "9")]
    pub post_step: ::core::option::Option<ExecStep>,
    /// Allows the patch job to run on Managed instance groups (MIGs).
    #[prost(bool, tag = "10")]
    pub mig_instances_allowed: bool,
}
/// Nested message and enum types in `PatchConfig`.
pub mod patch_config {
    /// Post-patch reboot settings.
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
    pub enum RebootConfig {
        /// The default behavior is DEFAULT.
        Unspecified = 0,
        /// The agent decides if a reboot is necessary by checking signals such as
        /// registry keys on Windows or `/var/run/reboot-required` on APT based
        /// systems. On RPM based systems, a set of core system package install times
        /// are compared with system boot time.
        Default = 1,
        /// Always reboot the machine after the update completes.
        Always = 2,
        /// Never reboot the machine after the update completes.
        Never = 3,
    }
    impl RebootConfig {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RebootConfig::Unspecified => "REBOOT_CONFIG_UNSPECIFIED",
                RebootConfig::Default => "DEFAULT",
                RebootConfig::Always => "ALWAYS",
                RebootConfig::Never => "NEVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REBOOT_CONFIG_UNSPECIFIED" => Some(Self::Unspecified),
                "DEFAULT" => Some(Self::Default),
                "ALWAYS" => Some(Self::Always),
                "NEVER" => Some(Self::Never),
                _ => None,
            }
        }
    }
}
/// Namespace for instance state enums.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Patch state of an instance.
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
    pub enum PatchState {
        /// Unspecified.
        Unspecified = 0,
        /// The instance is not yet notified.
        Pending = 1,
        /// Instance is inactive and cannot be patched.
        Inactive = 2,
        /// The instance is notified that it should be patched.
        Notified = 3,
        /// The instance has started the patching process.
        Started = 4,
        /// The instance is downloading patches.
        DownloadingPatches = 5,
        /// The instance is applying patches.
        ApplyingPatches = 6,
        /// The instance is rebooting.
        Rebooting = 7,
        /// The instance has completed applying patches.
        Succeeded = 8,
        /// The instance has completed applying patches but a reboot is required.
        SucceededRebootRequired = 9,
        /// The instance has failed to apply the patch.
        Failed = 10,
        /// The instance acked the notification and will start shortly.
        Acked = 11,
        /// The instance exceeded the time out while applying the patch.
        TimedOut = 12,
        /// The instance is running the pre-patch step.
        RunningPrePatchStep = 13,
        /// The instance is running the post-patch step.
        RunningPostPatchStep = 14,
        /// The service could not detect the presence of the agent. Check to ensure
        /// that the agent is installed, running, and able to communicate with the
        /// service.
        NoAgentDetected = 15,
    }
    impl PatchState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PatchState::Unspecified => "PATCH_STATE_UNSPECIFIED",
                PatchState::Pending => "PENDING",
                PatchState::Inactive => "INACTIVE",
                PatchState::Notified => "NOTIFIED",
                PatchState::Started => "STARTED",
                PatchState::DownloadingPatches => "DOWNLOADING_PATCHES",
                PatchState::ApplyingPatches => "APPLYING_PATCHES",
                PatchState::Rebooting => "REBOOTING",
                PatchState::Succeeded => "SUCCEEDED",
                PatchState::SucceededRebootRequired => "SUCCEEDED_REBOOT_REQUIRED",
                PatchState::Failed => "FAILED",
                PatchState::Acked => "ACKED",
                PatchState::TimedOut => "TIMED_OUT",
                PatchState::RunningPrePatchStep => "RUNNING_PRE_PATCH_STEP",
                PatchState::RunningPostPatchStep => "RUNNING_POST_PATCH_STEP",
                PatchState::NoAgentDetected => "NO_AGENT_DETECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PATCH_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "INACTIVE" => Some(Self::Inactive),
                "NOTIFIED" => Some(Self::Notified),
                "STARTED" => Some(Self::Started),
                "DOWNLOADING_PATCHES" => Some(Self::DownloadingPatches),
                "APPLYING_PATCHES" => Some(Self::ApplyingPatches),
                "REBOOTING" => Some(Self::Rebooting),
                "SUCCEEDED" => Some(Self::Succeeded),
                "SUCCEEDED_REBOOT_REQUIRED" => Some(Self::SucceededRebootRequired),
                "FAILED" => Some(Self::Failed),
                "ACKED" => Some(Self::Acked),
                "TIMED_OUT" => Some(Self::TimedOut),
                "RUNNING_PRE_PATCH_STEP" => Some(Self::RunningPrePatchStep),
                "RUNNING_POST_PATCH_STEP" => Some(Self::RunningPostPatchStep),
                "NO_AGENT_DETECTED" => Some(Self::NoAgentDetected),
                _ => None,
            }
        }
    }
}
/// Message for canceling a patch job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Apt patching is completed by executing `apt-get update && apt-get
/// upgrade`. Additional options can be set to control how this is executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptSettings {
    /// By changing the type to DIST, the patching is performed
    /// using `apt-get dist-upgrade` instead.
    #[prost(enumeration = "apt_settings::Type", tag = "1")]
    pub r#type: i32,
    /// List of packages to exclude from update. These packages will be excluded
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field cannot be specified with any other patch configuration
    /// fields.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AptSettings`.
pub mod apt_settings {
    /// Apt patch type.
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
    pub enum Type {
        /// By default, upgrade will be performed.
        Unspecified = 0,
        /// Runs `apt-get dist-upgrade`.
        Dist = 1,
        /// Runs `apt-get upgrade`.
        Upgrade = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Dist => "DIST",
                Type::Upgrade => "UPGRADE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIST" => Some(Self::Dist),
                "UPGRADE" => Some(Self::Upgrade),
                _ => None,
            }
        }
    }
}
/// Yum patching is performed by executing `yum update`. Additional options
/// can be set to control how this is executed.
///
/// Note that not all settings are supported on all platforms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumSettings {
    /// Adds the `--security` flag to `yum update`. Not supported on
    /// all platforms.
    #[prost(bool, tag = "1")]
    pub security: bool,
    /// Will cause patch to run `yum update-minimal` instead.
    #[prost(bool, tag = "2")]
    pub minimal: bool,
    /// List of packages to exclude from update. These packages are excluded by
    /// using the yum `--exclude` flag.
    #[prost(string, repeated, tag = "3")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field must not be specified with any other patch
    /// configuration fields.
    #[prost(string, repeated, tag = "4")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Googet patching is performed by running `googet update`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooSettings {}
/// Zypper patching is performed by running `zypper patch`.
/// See also <https://en.opensuse.org/SDB:Zypper_manual.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperSettings {
    /// Adds the `--with-optional` flag to `zypper patch`.
    #[prost(bool, tag = "1")]
    pub with_optional: bool,
    /// Adds the `--with-update` flag, to `zypper patch`.
    #[prost(bool, tag = "2")]
    pub with_update: bool,
    /// Install only patches with these categories.
    /// Common categories include security, recommended, and feature.
    #[prost(string, repeated, tag = "3")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Install only patches with these severities.
    /// Common severities include critical, important, moderate, and low.
    #[prost(string, repeated, tag = "4")]
    pub severities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of patches to exclude from update.
    #[prost(string, repeated, tag = "5")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of patches to be updated. These are the only patches
    /// that will be installed using 'zypper patch patch:<patch_name>' command.
    /// This field must not be used with any other patch configuration fields.
    #[prost(string, repeated, tag = "6")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Windows patching is performed using the Windows Update Agent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsUpdateSettings {
    /// Only apply updates of these windows update classifications. If empty, all
    /// updates are applied.
    #[prost(
        enumeration = "windows_update_settings::Classification",
        repeated,
        tag = "1"
    )]
    pub classifications: ::prost::alloc::vec::Vec<i32>,
    /// List of KBs to exclude from update.
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of kbs to be updated. These are the only patches
    /// that will be updated. This field must not be used with other
    /// patch configurations.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `WindowsUpdateSettings`.
pub mod windows_update_settings {
    /// Microsoft Windows update classifications as defined in
    /// \[1\]
    /// <https://support.microsoft.com/en-us/help/824684/description-of-the-standard-terminology-that-is-used-to-describe-micro>
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
    pub enum Classification {
        /// Invalid. If classifications are included, they must be specified.
        Unspecified = 0,
        /// "A widely released fix for a specific problem that addresses a critical,
        /// non-security-related bug." \[1\]
        Critical = 1,
        /// "A widely released fix for a product-specific, security-related
        /// vulnerability. Security vulnerabilities are rated by their severity. The
        /// severity rating is indicated in the Microsoft security bulletin as
        /// critical, important, moderate, or low." \[1\]
        Security = 2,
        /// "A widely released and frequent software update that contains additions
        /// to a product's definition database. Definition databases are often used
        /// to detect objects that have specific attributes, such as malicious code,
        /// phishing websites, or junk mail." \[1\]
        Definition = 3,
        /// "Software that controls the input and output of a device." \[1\]
        Driver = 4,
        /// "New product functionality that is first distributed outside the context
        /// of a product release and that is typically included in the next full
        /// product release." \[1\]
        FeaturePack = 5,
        /// "A tested, cumulative set of all hotfixes, security updates, critical
        /// updates, and updates. Additionally, service packs may contain additional
        /// fixes for problems that are found internally since the release of the
        /// product. Service packs my also contain a limited number of
        /// customer-requested design changes or features." \[1\]
        ServicePack = 6,
        /// "A utility or feature that helps complete a task or set of tasks." \[1\]
        Tool = 7,
        /// "A tested, cumulative set of hotfixes, security updates, critical
        /// updates, and updates that are packaged together for easy deployment. A
        /// rollup generally targets a specific area, such as security, or a
        /// component of a product, such as Internet Information Services (IIS)." \[1\]
        UpdateRollup = 8,
        /// "A widely released fix for a specific problem. An update addresses a
        /// noncritical, non-security-related bug." \[1\]
        Update = 9,
    }
    impl Classification {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Classification::Unspecified => "CLASSIFICATION_UNSPECIFIED",
                Classification::Critical => "CRITICAL",
                Classification::Security => "SECURITY",
                Classification::Definition => "DEFINITION",
                Classification::Driver => "DRIVER",
                Classification::FeaturePack => "FEATURE_PACK",
                Classification::ServicePack => "SERVICE_PACK",
                Classification::Tool => "TOOL",
                Classification::UpdateRollup => "UPDATE_ROLLUP",
                Classification::Update => "UPDATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLASSIFICATION_UNSPECIFIED" => Some(Self::Unspecified),
                "CRITICAL" => Some(Self::Critical),
                "SECURITY" => Some(Self::Security),
                "DEFINITION" => Some(Self::Definition),
                "DRIVER" => Some(Self::Driver),
                "FEATURE_PACK" => Some(Self::FeaturePack),
                "SERVICE_PACK" => Some(Self::ServicePack),
                "TOOL" => Some(Self::Tool),
                "UPDATE_ROLLUP" => Some(Self::UpdateRollup),
                "UPDATE" => Some(Self::Update),
                _ => None,
            }
        }
    }
}
/// A step that runs an executable for a PatchJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "1")]
    pub linux_exec_step_config: ::core::option::Option<ExecStepConfig>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "2")]
    pub windows_exec_step_config: ::core::option::Option<ExecStepConfig>,
}
/// Common configurations for an ExecStep.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepConfig {
    /// Defaults to \[0\]. A list of possible return values that the
    /// execution can return to indicate a success.
    #[prost(int32, repeated, tag = "3")]
    pub allowed_success_codes: ::prost::alloc::vec::Vec<i32>,
    /// The script interpreter to use to run the script. If no interpreter is
    /// specified the script will be executed directly, which will likely
    /// only succeed for scripts with [shebang lines]
    /// (<https://en.wikipedia.org/wiki/Shebang_\(Unix\>)).
    #[prost(enumeration = "exec_step_config::Interpreter", tag = "4")]
    pub interpreter: i32,
    /// Location of the executable.
    #[prost(oneof = "exec_step_config::Executable", tags = "1, 2")]
    pub executable: ::core::option::Option<exec_step_config::Executable>,
}
/// Nested message and enum types in `ExecStepConfig`.
pub mod exec_step_config {
    /// The interpreter used to execute the a file.
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
    pub enum Interpreter {
        /// Invalid for a Windows ExecStepConfig. For a Linux ExecStepConfig, the
        /// interpreter will be parsed from the shebang line of the script if
        /// unspecified.
        Unspecified = 0,
        /// Indicates that the script is run with `/bin/sh` on Linux and `cmd`
        /// on Windows.
        Shell = 1,
        /// Indicates that the file is run with PowerShell flags
        /// `-NonInteractive`, `-NoProfile`, and `-ExecutionPolicy Bypass`.
        Powershell = 2,
    }
    impl Interpreter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Interpreter::Unspecified => "INTERPRETER_UNSPECIFIED",
                Interpreter::Shell => "SHELL",
                Interpreter::Powershell => "POWERSHELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTERPRETER_UNSPECIFIED" => Some(Self::Unspecified),
                "SHELL" => Some(Self::Shell),
                "POWERSHELL" => Some(Self::Powershell),
                _ => None,
            }
        }
    }
    /// Location of the executable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// An absolute path to the executable on the VM.
        #[prost(string, tag = "1")]
        LocalPath(::prost::alloc::string::String),
        /// A Google Cloud Storage object containing the executable.
        #[prost(message, tag = "2")]
        GcsObject(super::GcsObject),
    }
}
/// Google Cloud Storage object representation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Required. Bucket of the Google Cloud Storage object.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the Google Cloud Storage object.
    #[prost(string, tag = "2")]
    pub object: ::prost::alloc::string::String,
    /// Required. Generation number of the Google Cloud Storage object. This is used to
    /// ensure that the ExecStep specified by this PatchJob does not change.
    #[prost(int64, tag = "3")]
    pub generation_number: i64,
}
/// A filter to target VM instances for patching. The targeted
/// VMs must meet all criteria specified. So if both labels and zones are
/// specified, the patch job targets only VMs with those labels and in those
/// zones.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchInstanceFilter {
    /// Target all VM instances in the project. If true, no other criteria is
    /// permitted.
    #[prost(bool, tag = "1")]
    pub all: bool,
    /// Targets VM instances matching at least one of these label sets. This allows
    /// targeting of disparate groups, for example "env=prod or env=staging".
    #[prost(message, repeated, tag = "2")]
    pub group_labels: ::prost::alloc::vec::Vec<patch_instance_filter::GroupLabel>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM
    /// instances in any zone.
    #[prost(string, repeated, tag = "3")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets any of the VM instances specified. Instances are specified by their
    /// URI in the form `zones/\[ZONE]/instances/[INSTANCE_NAME\]`,
    /// `projects/\[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME\]`, or
    /// `<https://www.googleapis.com/compute/v1/projects/\[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME\]`>
    #[prost(string, repeated, tag = "4")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets VMs whose name starts with one of these prefixes. Similar to
    /// labels, this is another way to group VMs when targeting configs, for
    /// example prefix="prod-".
    #[prost(string, repeated, tag = "5")]
    pub instance_name_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `PatchInstanceFilter`.
pub mod patch_instance_filter {
    /// Represents a group of VMs that can be identified as having all these
    /// labels, for example "env=prod and app=web".
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupLabel {
        /// Compute Engine instance labels that must be present for a VM instance to
        /// be targeted by this filter.
        #[prost(btree_map = "string, string", tag = "1")]
        pub labels: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// Patch rollout configuration specifications. Contains details on the
/// concurrency control when applying patch(es) to all targeted VMs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchRollout {
    /// Mode of the patch rollout.
    #[prost(enumeration = "patch_rollout::Mode", tag = "1")]
    pub mode: i32,
    /// The maximum number (or percentage) of VMs per zone to disrupt at any given
    /// moment. The number of VMs calculated from multiplying the percentage by the
    /// total number of VMs in a zone is rounded up.
    ///
    /// During patching, a VM is considered disrupted from the time the agent is
    /// notified to begin until patching has completed. This disruption time
    /// includes the time to complete reboot and any post-patch steps.
    ///
    /// A VM contributes to the disruption budget if its patching operation fails
    /// either when applying the patches, running pre or post patch steps, or if it
    /// fails to respond with a success notification before timing out. VMs that
    /// are not running or do not have an active agent do not count toward this
    /// disruption budget.
    ///
    /// For zone-by-zone rollouts, if the disruption budget in a zone is exceeded,
    /// the patch job stops, because continuing to the next zone requires
    /// completion of the patch process in the previous zone.
    ///
    /// For example, if the disruption budget has a fixed value of `10`, and 8 VMs
    /// fail to patch in the current zone, the patch job continues to patch 2 VMs
    /// at a time until the zone is completed. When that zone is completed
    /// successfully, patching begins with 10 VMs at a time in the next zone. If 10
    /// VMs in the next zone fail to patch, the patch job stops.
    #[prost(message, optional, tag = "2")]
    pub disruption_budget: ::core::option::Option<FixedOrPercent>,
}
/// Nested message and enum types in `PatchRollout`.
pub mod patch_rollout {
    /// Type of the rollout.
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
    pub enum Mode {
        /// Mode must be specified.
        Unspecified = 0,
        /// Patches are applied one zone at a time. The patch job begins in the
        /// region with the lowest number of targeted VMs. Within the region,
        /// patching begins in the zone with the lowest number of targeted VMs. If
        /// multiple regions (or zones within a region) have the same number of
        /// targeted VMs, a tie-breaker is achieved by sorting the regions or zones
        /// in alphabetical order.
        ZoneByZone = 1,
        /// Patches are applied to VMs in all zones at the same time.
        ConcurrentZones = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::ZoneByZone => "ZONE_BY_ZONE",
                Mode::ConcurrentZones => "CONCURRENT_ZONES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "ZONE_BY_ZONE" => Some(Self::ZoneByZone),
                "CONCURRENT_ZONES" => Some(Self::ConcurrentZones),
                _ => None,
            }
        }
    }
}
/// Patch deployments are configurations that individual patch jobs use to
/// complete a patch. These configurations include instance filter, package
/// repository settings, and a schedule. For more information about creating and
/// managing patch deployments, see [Scheduling patch
/// jobs](<https://cloud.google.com/compute/docs/os-patch-management/schedule-patch-jobs>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchDeployment {
    /// Unique name for the patch deployment resource in a project. The patch
    /// deployment name is in the form:
    /// `projects/{project_id}/patchDeployments/{patch_deployment_id}`.
    /// This field is ignored when you create a new patch deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Description of the patch deployment. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. VM instances to patch.
    #[prost(message, optional, tag = "3")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Optional. Patch configuration that is applied.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Optional. Duration of the patch. After the duration ends, the patch times out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Time the patch deployment was created. Timestamp is in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the patch deployment was last updated. Timestamp is in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time a patch job was started by this deployment.
    /// Timestamp is in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text
    /// format.
    #[prost(message, optional, tag = "10")]
    pub last_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Rollout strategy of the patch job.
    #[prost(message, optional, tag = "11")]
    pub rollout: ::core::option::Option<PatchRollout>,
    /// Output only. Current state of the patch deployment.
    #[prost(enumeration = "patch_deployment::State", tag = "12")]
    pub state: i32,
    /// Schedule for the patch.
    #[prost(oneof = "patch_deployment::Schedule", tags = "6, 7")]
    pub schedule: ::core::option::Option<patch_deployment::Schedule>,
}
/// Nested message and enum types in `PatchDeployment`.
pub mod patch_deployment {
    /// Represents state of patch peployment.
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
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Active value means that patch deployment generates Patch Jobs.
        Active = 1,
        /// Paused value means that patch deployment does not generate
        /// Patch jobs. Requires user action to move in and out from this state.
        Paused = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
                State::Paused => "PAUSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "PAUSED" => Some(Self::Paused),
                _ => None,
            }
        }
    }
    /// Schedule for the patch.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schedule {
        /// Required. Schedule a one-time execution.
        #[prost(message, tag = "6")]
        OneTimeSchedule(super::OneTimeSchedule),
        /// Required. Schedule recurring executions.
        #[prost(message, tag = "7")]
        RecurringSchedule(super::RecurringSchedule),
    }
}
/// Sets the time for a one time patch deployment. Timestamp is in
/// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneTimeSchedule {
    /// Required. The desired patch job execution time.
    #[prost(message, optional, tag = "1")]
    pub execute_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Sets the time for recurring patch deployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringSchedule {
    /// Required. Defines the time zone that `time_of_day` is relative to.
    /// The rules for daylight saving time are determined by the chosen time zone.
    #[prost(message, optional, tag = "1")]
    pub time_zone: ::core::option::Option<super::super::super::r#type::TimeZone>,
    /// Optional. The time that the recurring schedule becomes effective.
    /// Defaults to `create_time` of the patch deployment.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The end time at which a recurring patch deployment schedule is no longer
    /// active.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Time of the day to run a recurring deployment.
    #[prost(message, optional, tag = "4")]
    pub time_of_day: ::core::option::Option<super::super::super::r#type::TimeOfDay>,
    /// Required. The frequency unit of this recurring schedule.
    #[prost(enumeration = "recurring_schedule::Frequency", tag = "5")]
    pub frequency: i32,
    /// Output only. The time the last patch job ran successfully.
    #[prost(message, optional, tag = "9")]
    pub last_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the next patch job is scheduled to run.
    #[prost(message, optional, tag = "10")]
    pub next_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[prost(oneof = "recurring_schedule::ScheduleConfig", tags = "6, 7")]
    pub schedule_config: ::core::option::Option<recurring_schedule::ScheduleConfig>,
}
/// Nested message and enum types in `RecurringSchedule`.
pub mod recurring_schedule {
    /// Specifies the frequency of the recurring patch deployments.
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
    pub enum Frequency {
        /// Invalid. A frequency must be specified.
        Unspecified = 0,
        /// Indicates that the frequency of recurrence should be expressed in terms
        /// of weeks.
        Weekly = 1,
        /// Indicates that the frequency of recurrence should be expressed in terms
        /// of months.
        Monthly = 2,
        /// Indicates that the frequency of recurrence should be expressed in terms
        /// of days.
        Daily = 3,
    }
    impl Frequency {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Frequency::Unspecified => "FREQUENCY_UNSPECIFIED",
                Frequency::Weekly => "WEEKLY",
                Frequency::Monthly => "MONTHLY",
                Frequency::Daily => "DAILY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FREQUENCY_UNSPECIFIED" => Some(Self::Unspecified),
                "WEEKLY" => Some(Self::Weekly),
                "MONTHLY" => Some(Self::Monthly),
                "DAILY" => Some(Self::Daily),
                _ => None,
            }
        }
    }
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScheduleConfig {
        /// Required. Schedule with weekly executions.
        #[prost(message, tag = "6")]
        Weekly(super::WeeklySchedule),
        /// Required. Schedule with monthly executions.
        #[prost(message, tag = "7")]
        Monthly(super::MonthlySchedule),
    }
}
/// Represents a weekly schedule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeeklySchedule {
    /// Required. Day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "1")]
    pub day_of_week: i32,
}
/// Represents a monthly schedule. An example of a valid monthly schedule is
/// "on the third Tuesday of the month" or "on the 15th of the month".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlySchedule {
    /// One day in a month.
    #[prost(oneof = "monthly_schedule::DayOfMonth", tags = "1, 2")]
    pub day_of_month: ::core::option::Option<monthly_schedule::DayOfMonth>,
}
/// Nested message and enum types in `MonthlySchedule`.
pub mod monthly_schedule {
    /// One day in a month.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DayOfMonth {
        /// Required. Week day in a month.
        #[prost(message, tag = "1")]
        WeekDayOfMonth(super::WeekDayOfMonth),
        /// Required. One day of the month. 1-31 indicates the 1st to the 31st day. -1
        /// indicates the last day of the month.
        /// Months without the target day will be skipped. For example, a schedule to
        /// run "every month on the 31st" will not run in February, April, June, etc.
        #[prost(int32, tag = "2")]
        MonthDay(i32),
    }
}
/// Represents one week day in a month. An example is "the 4th Sunday".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeekDayOfMonth {
    /// Required. Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1
    /// indicates the last week of the month.
    #[prost(int32, tag = "1")]
    pub week_ordinal: i32,
    /// Required. A day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "2")]
    pub day_of_week: i32,
    /// Optional. Represents the number of days before or after the given week day of month
    /// that the patch deployment is scheduled for. For example if `week_ordinal`
    /// and `day_of_week` values point to the second day of the month and this
    /// `day_offset` value is set to `3`, the patch deployment takes place three
    /// days after the second Tuesday of the month. If this value is negative, for
    /// example -5, the patches  are deployed five days before before the second
    /// Tuesday of the month. Allowed values are in range `[-30, 30]`.
    #[prost(int32, tag = "3")]
    pub day_offset: i32,
}
/// A request message for creating a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePatchDeploymentRequest {
    /// Required. The project to apply this patch deployment to in the form `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A name for the patch deployment in the project. When creating a name
    /// the following rules apply:
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "2")]
    pub patch_deployment_id: ::prost::alloc::string::String,
    /// Required. The patch deployment to create.
    #[prost(message, optional, tag = "3")]
    pub patch_deployment: ::core::option::Option<PatchDeployment>,
}
/// A request message for retrieving a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing patch deployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsRequest {
    /// Required. The resource name of the parent in the form `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of patch deployments to return. Default is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to ListPatchDeployments
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing patch deployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsResponse {
    /// The list of patch deployments.
    #[prost(message, repeated, tag = "1")]
    pub patch_deployments: ::prost::alloc::vec::Vec<PatchDeployment>,
    /// A pagination token that can be used to get the next page of patch
    /// deployments.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message for deleting a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for updating a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePatchDeploymentRequest {
    /// Required. The patch deployment to Update.
    #[prost(message, optional, tag = "1")]
    pub patch_deployment: ::core::option::Option<PatchDeployment>,
    /// Optional. Field mask that controls which fields of the patch deployment should be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A request message for pausing a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PausePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for resuming a patch deployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod os_config_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// OS Config API
    ///
    /// The OS Config service is a server-side component that you can use to
    /// manage package installations and patch jobs for virtual machine instances.
    #[derive(Debug, Clone)]
    pub struct OsConfigServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OsConfigServiceClient<T>
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
        ) -> OsConfigServiceClient<InterceptedService<T, F>>
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
            OsConfigServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Patch VM instances by creating and running a patch job.
        pub async fn execute_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecutePatchJobRequest>,
        ) -> std::result::Result<tonic::Response<super::PatchJob>, tonic::Status> {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ExecutePatchJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "ExecutePatchJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the patch job. This can be used to track the progress of an
        /// ongoing patch job or review the details of completed jobs.
        pub async fn get_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPatchJobRequest>,
        ) -> std::result::Result<tonic::Response<super::PatchJob>, tonic::Status> {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetPatchJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "GetPatchJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Cancel a patch job. The patch job must be active. Canceled patch jobs
        /// cannot be restarted.
        pub async fn cancel_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelPatchJobRequest>,
        ) -> std::result::Result<tonic::Response<super::PatchJob>, tonic::Status> {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CancelPatchJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "CancelPatchJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a list of patch jobs.
        pub async fn list_patch_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPatchJobsResponse>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "ListPatchJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a list of instance details for a given patch job.
        pub async fn list_patch_job_instance_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchJobInstanceDetailsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPatchJobInstanceDetailsResponse>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchJobInstanceDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "ListPatchJobInstanceDetails",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create an OS Config patch deployment.
        pub async fn create_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CreatePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "CreatePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get an OS Config patch deployment.
        pub async fn get_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetPatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "GetPatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a page of OS Config patch deployments.
        pub async fn list_patch_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchDeploymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPatchDeploymentsResponse>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchDeployments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "ListPatchDeployments",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete an OS Config patch deployment.
        pub async fn delete_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePatchDeploymentRequest>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/DeletePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "DeletePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update an OS Config patch deployment.
        pub async fn update_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/UpdatePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "UpdatePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Change state of patch deployment to "PAUSED".
        /// Patch deployment in paused state doesn't generate patch jobs.
        pub async fn pause_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::PausePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/PausePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "PausePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Change state of patch deployment back to "ACTIVE".
        /// Patch deployment in active state continues to generate patch jobs.
        pub async fn resume_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumePatchDeploymentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PatchDeployment>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ResumePatchDeployment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "ResumePatchDeployment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create an OS Config guest policy.
        pub async fn create_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGuestPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::GuestPolicy>, tonic::Status> {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CreateGuestPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "CreateGuestPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get an OS Config guest policy.
        pub async fn get_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGuestPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::GuestPolicy>, tonic::Status> {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetGuestPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "GetGuestPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a page of OS Config guest policies.
        pub async fn list_guest_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGuestPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListGuestPoliciesResponse>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListGuestPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "ListGuestPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update an OS Config guest policy.
        pub async fn update_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGuestPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::GuestPolicy>, tonic::Status> {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/UpdateGuestPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "UpdateGuestPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete an OS Config guest policy.
        pub async fn delete_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGuestPolicyRequest>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/DeleteGuestPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "DeleteGuestPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lookup the effective guest policy that applies to a VM instance. This
        /// lookup merges all policies that are assigned to the instance ancestry.
        pub async fn lookup_effective_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEffectiveGuestPolicyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EffectiveGuestPolicy>,
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
                "/google.cloud.osconfig.v1beta.OsConfigService/LookupEffectiveGuestPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.osconfig.v1beta.OsConfigService",
                        "LookupEffectiveGuestPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
