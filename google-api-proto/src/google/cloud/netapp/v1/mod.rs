/// ListSnapshotsRequest lists snapshots.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// Required. The volume for which to retrieve snapshot information,
    /// in the format
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListSnapshotsResponse is the result of ListSnapshotsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    /// A list of snapshots in the project for the specified volume.
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetSnapshotRequest gets the state of a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    /// Required. The snapshot resource name, in the format
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateSnapshotRequest creates a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnapshotRequest {
    /// Required. The NetApp volume to create the snapshots of, in the format
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A snapshot resource
    #[prost(message, optional, tag = "2")]
    pub snapshot: ::core::option::Option<Snapshot>,
    /// Required. ID of the snapshot to create.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "3")]
    pub snapshot_id: ::prost::alloc::string::String,
}
/// DeleteSnapshotRequest deletes a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotRequest {
    /// Required. The snapshot resource name, in the format
    /// `projects/*/locations/*/volumes/*/snapshots/{snapshot_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateSnapshotRequest updates description and/or labels for a snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnapshotRequest {
    /// Required. Mask of fields to update.  At least one path must be supplied in
    /// this field.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. A snapshot resource
    #[prost(message, optional, tag = "2")]
    pub snapshot: ::core::option::Option<Snapshot>,
}
/// Snapshot is a point-in-time version of a Volume's content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// Identifier. The resource name of the snapshot.
    /// Format:
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The snapshot state.
    #[prost(enumeration = "snapshot::State", tag = "2")]
    pub state: i32,
    /// Output only. State details of the storage pool
    #[prost(string, tag = "3")]
    pub state_details: ::prost::alloc::string::String,
    /// A description of the snapshot with 2048 characters or less.
    /// Requests with longer descriptions will be rejected.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Current storage usage for the snapshot in bytes.
    #[prost(double, tag = "5")]
    pub used_bytes: f64,
    /// Output only. The time when the snapshot was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(btree_map = "string, string", tag = "7")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Snapshot`.
pub mod snapshot {
    /// The Snapshot States
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
        /// Unspecified Snapshot State
        Unspecified = 0,
        /// Snapshot State is Ready
        Ready = 1,
        /// Snapshot State is Creating
        Creating = 2,
        /// Snapshot State is Deleting
        Deleting = 3,
        /// Snapshot State is Updating
        Updating = 4,
        /// Snapshot State is Disabled
        Disabled = 5,
        /// Snapshot State is Error
        Error = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Ready => "READY",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::Disabled => "DISABLED",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "DISABLED" => Some(Self::Disabled),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// ListActiveDirectoriesRequest for requesting multiple active directories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActiveDirectoriesRequest {
    /// Required. Parent value for ListActiveDirectoriesRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// ListActiveDirectoriesResponse contains all the active directories requested.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActiveDirectoriesResponse {
    /// The list of active directories.
    #[prost(message, repeated, tag = "1")]
    pub active_directories: ::prost::alloc::vec::Vec<ActiveDirectory>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetActiveDirectory for getting a single active directory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveDirectoryRequest {
    /// Required. Name of the active directory.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateActiveDirectoryRequest for creating an active directory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateActiveDirectoryRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Fields of the to be created active directory.
    #[prost(message, optional, tag = "2")]
    pub active_directory: ::core::option::Option<ActiveDirectory>,
    /// Required. ID of the active directory to create.
    #[prost(string, tag = "3")]
    pub active_directory_id: ::prost::alloc::string::String,
}
/// UpdateActiveDirectoryRequest for updating an active directory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActiveDirectoryRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Active Directory resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The volume being updated
    #[prost(message, optional, tag = "2")]
    pub active_directory: ::core::option::Option<ActiveDirectory>,
}
/// DeleteActiveDirectoryRequest for deleting a single active directory.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteActiveDirectoryRequest {
    /// Required. Name of the active directory.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ActiveDirectory is the public representation of the active directory config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveDirectory {
    /// Identifier. The resource name of the active directory.
    /// Format:
    /// `projects/{project_number}/locations/{location_id}/activeDirectories/{active_directory_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Create time of the active directory.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of the AD.
    #[prost(enumeration = "active_directory::State", tag = "3")]
    pub state: i32,
    /// Required. Name of the Active Directory domain
    #[prost(string, tag = "4")]
    pub domain: ::prost::alloc::string::String,
    /// The Active Directory site the service will limit Domain Controller
    /// discovery too.
    #[prost(string, tag = "5")]
    pub site: ::prost::alloc::string::String,
    /// Required. Comma separated list of DNS server IP addresses for the Active
    /// Directory domain.
    #[prost(string, tag = "6")]
    pub dns: ::prost::alloc::string::String,
    /// Required. NetBIOSPrefix is used as a prefix for SMB server name.
    #[prost(string, tag = "7")]
    pub net_bios_prefix: ::prost::alloc::string::String,
    /// The Organizational Unit (OU) within the Windows Active Directory the user
    /// belongs to.
    #[prost(string, tag = "8")]
    pub organizational_unit: ::prost::alloc::string::String,
    /// If enabled, AES encryption will be enabled for SMB communication.
    #[prost(bool, tag = "9")]
    pub aes_encryption: bool,
    /// Required. Username of the Active Directory domain administrator.
    #[prost(string, tag = "10")]
    pub username: ::prost::alloc::string::String,
    /// Required. Password of the Active Directory domain administrator.
    #[prost(string, tag = "11")]
    pub password: ::prost::alloc::string::String,
    /// Users to be added to the Built-in Backup Operator active directory group.
    #[prost(string, repeated, tag = "12")]
    pub backup_operators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Domain users to be given the SeSecurityPrivilege.
    #[prost(string, repeated, tag = "13")]
    pub security_operators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Name of the active directory machine. This optional parameter is used only
    /// while creating kerberos volume
    #[prost(string, tag = "14")]
    pub kdc_hostname: ::prost::alloc::string::String,
    /// KDC server IP address for the active directory machine.
    #[prost(string, tag = "15")]
    pub kdc_ip: ::prost::alloc::string::String,
    /// If enabled, will allow access to local users and LDAP users. If access is
    /// needed for only LDAP users, it has to be disabled.
    #[prost(bool, tag = "16")]
    pub nfs_users_with_ldap: bool,
    /// Description of the active directory.
    #[prost(string, tag = "17")]
    pub description: ::prost::alloc::string::String,
    /// Specifies whether or not the LDAP traffic needs to be signed.
    #[prost(bool, tag = "18")]
    pub ldap_signing: bool,
    /// If enabled, traffic between the SMB server to Domain Controller (DC) will
    /// be encrypted.
    #[prost(bool, tag = "19")]
    pub encrypt_dc_connections: bool,
    /// Labels for the active directory.
    #[prost(btree_map = "string, string", tag = "20")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The state details of the Active Directory.
    #[prost(string, tag = "21")]
    pub state_details: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ActiveDirectory`.
pub mod active_directory {
    /// The Active Directory States
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
        /// Unspecified Active Directory State
        Unspecified = 0,
        /// Active Directory State is Creating
        Creating = 1,
        /// Active Directory State is Ready
        Ready = 2,
        /// Active Directory State is Updating
        Updating = 3,
        /// Active Directory State is In use
        InUse = 4,
        /// Active Directory State is Deleting
        Deleting = 5,
        /// Active Directory State is Error
        Error = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Updating => "UPDATING",
                State::InUse => "IN_USE",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "IN_USE" => Some(Self::InUse),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// A NetApp Backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Identifier. The resource name of the backup.
    /// Format:
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The backup state.
    #[prost(enumeration = "backup::State", tag = "2")]
    pub state: i32,
    /// A description of the backup with 2048 characters or less.
    /// Requests with longer descriptions will be rejected.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Size of the file system when the backup was created. When
    /// creating a new volume from the backup, the volume capacity will have to be
    /// at least as big.
    #[prost(int64, tag = "4")]
    pub volume_usage_bytes: i64,
    /// Output only. Type of backup, manually created or created by a backup
    /// policy.
    #[prost(enumeration = "backup::Type", tag = "5")]
    pub backup_type: i32,
    /// Volume full name of this backup belongs to.
    /// Format:
    /// `projects/{projects_id}/locations/{location}/volumes/{volume_id}`
    #[prost(string, tag = "6")]
    pub source_volume: ::prost::alloc::string::String,
    /// If specified, backup will be created from the given snapshot.
    /// If not specified, there will be a new snapshot taken to initiate the backup
    /// creation. Format:
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}/snapshots/{snapshot_id}`
    #[prost(string, optional, tag = "7")]
    pub source_snapshot: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The time when the backup was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(btree_map = "string, string", tag = "9")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Total size of all backups in a chain in bytes = baseline
    /// backup size + sum(incremental backup size)
    #[prost(int64, tag = "10")]
    pub chain_storage_bytes: i64,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// The Backup States
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
        /// State not set.
        Unspecified = 0,
        /// Backup is being created. While in this state, the snapshot for the backup
        /// point-in-time may not have been created yet, and so the point-in-time may
        /// not have been fixed.
        Creating = 1,
        /// Backup is being uploaded. While in this state, none of the writes to the
        /// volume will be included in the backup.
        Uploading = 2,
        /// Backup is available for use.
        Ready = 3,
        /// Backup is being deleted.
        Deleting = 4,
        /// Backup is not valid and cannot be used for creating new volumes or
        /// restoring existing volumes.
        Error = 5,
        /// Backup is being updated.
        Updating = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Uploading => "UPLOADING",
                State::Ready => "READY",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
                State::Updating => "UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "UPLOADING" => Some(Self::Uploading),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                "UPDATING" => Some(Self::Updating),
                _ => None,
            }
        }
    }
    /// Backup types.
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
        /// Unspecified backup type.
        Unspecified = 0,
        /// Manual backup type.
        Manual = 1,
        /// Scheduled backup type.
        Scheduled = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Manual => "MANUAL",
                Type::Scheduled => "SCHEDULED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MANUAL" => Some(Self::Manual),
                "SCHEDULED" => Some(Self::Scheduled),
                _ => None,
            }
        }
    }
}
/// ListBackupsRequest lists backups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The backupVault for which to retrieve backup information,
    /// in the format
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`.
    /// To retrieve backup information for all locations, use "-" for the
    /// `{location}` value.
    /// To retrieve backup information for all backupVaults, use "-" for the
    /// `{backup_vault_id}` value.
    /// To retrieve backup information for a volume, use "-" for the
    /// `{backup_vault_id}` value and specify volume full name with the filter.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. The service may return fewer
    /// than this value. The maximum value
    /// is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// The standard list filter.
    /// If specified, backups will be returned based on the attribute name that
    /// matches the filter expression. If empty, then no backups are filtered out.
    /// See <https://google.aip.dev/160>
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListBackupsResponse is the result of ListBackupsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// A list of backups in the project.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetBackupRequest gets the state of a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. The backup resource name, in the format
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateBackupRequest creates a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. The NetApp backupVault to create the backups of, in the format
    /// `projects/*/locations/*/backupVaults/{backup_vault_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the backup.
    /// The ID must be unique within the specified backupVault.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    /// Values that do not match this pattern will trigger an INVALID_ARGUMENT
    /// error.
    #[prost(string, tag = "2")]
    pub backup_id: ::prost::alloc::string::String,
    /// Required. A backup resource
    #[prost(message, optional, tag = "3")]
    pub backup: ::core::option::Option<Backup>,
}
/// DeleteBackupRequest deletes a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. The backup resource name, in the format
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateBackupRequest updates description and/or labels for a backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Backup resource to be updated.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The backup being updated
    #[prost(message, optional, tag = "2")]
    pub backup: ::core::option::Option<Backup>,
}
/// Backup Policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupPolicy {
    /// Identifier. The resource name of the backup policy.
    /// Format:
    /// `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Number of daily backups to keep. Note that the minimum daily backup limit
    /// is 2.
    #[prost(int32, optional, tag = "2")]
    pub daily_backup_limit: ::core::option::Option<i32>,
    /// Number of weekly backups to keep. Note that the sum of daily, weekly and
    /// monthly backups should be greater than 1.
    #[prost(int32, optional, tag = "3")]
    pub weekly_backup_limit: ::core::option::Option<i32>,
    /// Number of monthly backups to keep. Note that the sum of daily, weekly and
    /// monthly backups should be greater than 1.
    #[prost(int32, optional, tag = "4")]
    pub monthly_backup_limit: ::core::option::Option<i32>,
    /// Description of the backup policy.
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// If enabled, make backups automatically according to the schedules.
    /// This will be applied to all volumes that have this policy attached and
    /// enforced on volume level. If not specified, default is true.
    #[prost(bool, optional, tag = "6")]
    pub enabled: ::core::option::Option<bool>,
    /// Output only. The total number of volumes assigned by this backup policy.
    #[prost(int32, optional, tag = "7")]
    pub assigned_volume_count: ::core::option::Option<i32>,
    /// Output only. The time when the backup policy was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(btree_map = "string, string", tag = "9")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The backup policy state.
    #[prost(enumeration = "backup_policy::State", tag = "10")]
    pub state: i32,
}
/// Nested message and enum types in `BackupPolicy`.
pub mod backup_policy {
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
        /// State not set.
        Unspecified = 0,
        /// BackupPolicy is being created.
        Creating = 1,
        /// BackupPolicy is available for use.
        Ready = 2,
        /// BackupPolicy is being deleted.
        Deleting = 3,
        /// BackupPolicy is not valid and cannot be used.
        Error = 4,
        /// BackupPolicy is being updated.
        Updating = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
                State::Updating => "UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                "UPDATING" => Some(Self::Updating),
                _ => None,
            }
        }
    }
}
/// CreateBackupPolicyRequest creates a backupPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupPolicyRequest {
    /// Required. The location to create the backup policies of, in the format
    /// `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A backupPolicy resource
    #[prost(message, optional, tag = "2")]
    pub backup_policy: ::core::option::Option<BackupPolicy>,
    /// Required. The ID to use for the backup policy.
    /// The ID must be unique within the specified location.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "3")]
    pub backup_policy_id: ::prost::alloc::string::String,
}
/// GetBackupPolicyRequest gets the state of a backupPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupPolicyRequest {
    /// Required. The backupPolicy resource name, in the format
    /// `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListBackupPoliciesRequest for requesting multiple backup policies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupPoliciesRequest {
    /// Required. Parent value for ListBackupPoliciesRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// ListBackupPoliciesResponse contains all the backup policies requested.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupPoliciesResponse {
    /// The list of backup policies.
    #[prost(message, repeated, tag = "1")]
    pub backup_policies: ::prost::alloc::vec::Vec<BackupPolicy>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UpdateBackupPolicyRequest for updating a backup policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupPolicyRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Backup Policy resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The backup policy being updated
    #[prost(message, optional, tag = "2")]
    pub backup_policy: ::core::option::Option<BackupPolicy>,
}
/// DeleteBackupPolicyRequest deletes a backup policy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupPolicyRequest {
    /// Required. The backup policy resource name, in the format
    /// `projects/{project_id}/locations/{location}/backupPolicies/{backup_policy_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A NetApp BackupVault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupVault {
    /// Identifier. The resource name of the backup vault.
    /// Format:
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The backup vault state.
    #[prost(enumeration = "backup_vault::State", tag = "2")]
    pub state: i32,
    /// Output only. Create time of the backup vault.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Description of the backup vault.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels to represent user provided metadata.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `BackupVault`.
pub mod backup_vault {
    /// The Backup Vault States
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
        /// State not set.
        Unspecified = 0,
        /// BackupVault is being created.
        Creating = 1,
        /// BackupVault is available for use.
        Ready = 2,
        /// BackupVault is being deleted.
        Deleting = 3,
        /// BackupVault is not valid and cannot be used.
        Error = 4,
        /// BackupVault is being updated.
        Updating = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
                State::Updating => "UPDATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                "UPDATING" => Some(Self::Updating),
                _ => None,
            }
        }
    }
}
/// GetBackupVaultRequest gets the state of a backupVault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupVaultRequest {
    /// Required. The backupVault resource name, in the format
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListBackupVaultsRequest lists backupVaults.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupVaultsRequest {
    /// Required. The location for which to retrieve backupVault information,
    /// in the format
    /// `projects/{project_id}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListBackupVaultsResponse is the result of ListBackupVaultsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupVaultsResponse {
    /// A list of backupVaults in the project for the specified location.
    #[prost(message, repeated, tag = "1")]
    pub backup_vaults: ::prost::alloc::vec::Vec<BackupVault>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CreateBackupVaultRequest creates a backup vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupVaultRequest {
    /// Required. The location to create the backup vaults, in the format
    /// `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for the backupVault.
    /// The ID must be unique within the specified location.
    /// The max supported length is 63 characters.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    /// Values that do not match this pattern will trigger an INVALID_ARGUMENT
    /// error.
    #[prost(string, tag = "2")]
    pub backup_vault_id: ::prost::alloc::string::String,
    /// Required. A backupVault resource
    #[prost(message, optional, tag = "3")]
    pub backup_vault: ::core::option::Option<BackupVault>,
}
/// DeleteBackupVaultRequest deletes a backupVault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupVaultRequest {
    /// Required. The backupVault resource name, in the format
    /// `projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateBackupVaultRequest updates description and/or labels for a backupVault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupVaultRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Backup resource to be updated.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The backupVault being updated
    #[prost(message, optional, tag = "2")]
    pub backup_vault: ::core::option::Option<BackupVault>,
}
/// GetKmsConfigRequest gets a KMS Config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKmsConfigRequest {
    /// Required. Name of the KmsConfig
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListKmsConfigsRequest lists KMS Configs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKmsConfigsRequest {
    /// Required. Parent value
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListKmsConfigsResponse is the response to a ListKmsConfigsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKmsConfigsResponse {
    /// The list of KmsConfigs
    #[prost(message, repeated, tag = "1")]
    pub kms_configs: ::prost::alloc::vec::Vec<KmsConfig>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CreateKmsConfigRequest creates a KMS Config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKmsConfigRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting KmsConfig
    /// If auto-generating Id server-side, remove this field and
    /// id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub kms_config_id: ::prost::alloc::string::String,
    /// Required. The required parameters to create a new KmsConfig.
    #[prost(message, optional, tag = "3")]
    pub kms_config: ::core::option::Option<KmsConfig>,
}
/// UpdateKmsConfigRequest updates a KMS Config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKmsConfigRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// KmsConfig resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The KmsConfig being updated
    #[prost(message, optional, tag = "2")]
    pub kms_config: ::core::option::Option<KmsConfig>,
}
/// DeleteKmsConfigRequest deletes a KMS Config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKmsConfigRequest {
    /// Required. Name of the KmsConfig.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// EncryptVolumesRequest specifies the KMS config to encrypt existing volumes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptVolumesRequest {
    /// Required. Name of the KmsConfig.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// VerifyKmsConfigRequest specifies the KMS config to be validated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyKmsConfigRequest {
    /// Required. Name of the KMS Config to be verified.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// VerifyKmsConfigResponse contains the information if the config is correctly
/// and error message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyKmsConfigResponse {
    /// Output only. If the customer key configured correctly to the encrypt
    /// volume.
    #[prost(bool, tag = "1")]
    pub healthy: bool,
    /// Output only. Error message if config is not healthy.
    #[prost(string, tag = "2")]
    pub health_error: ::prost::alloc::string::String,
    /// Output only. Instructions for the customers to provide the access to the
    /// encryption key.
    #[prost(string, tag = "3")]
    pub instructions: ::prost::alloc::string::String,
}
/// KmsConfig is the customer managed encryption key(CMEK) configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KmsConfig {
    /// Identifier. Name of the KmsConfig.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Customer managed crypto key resource full name. Format:
    /// projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}.
    #[prost(string, tag = "2")]
    pub crypto_key_name: ::prost::alloc::string::String,
    /// Output only. State of the KmsConfig.
    #[prost(enumeration = "kms_config::State", tag = "3")]
    pub state: i32,
    /// Output only. State details of the KmsConfig.
    #[prost(string, tag = "4")]
    pub state_details: ::prost::alloc::string::String,
    /// Output only. Create time of the KmsConfig.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Description of the KmsConfig.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "7")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Instructions to provide the access to the customer provided
    /// encryption key.
    #[prost(string, tag = "8")]
    pub instructions: ::prost::alloc::string::String,
    /// Output only. The Service account which will have access to the customer
    /// provided encryption key.
    #[prost(string, tag = "9")]
    pub service_account: ::prost::alloc::string::String,
}
/// Nested message and enum types in `KmsConfig`.
pub mod kms_config {
    /// The KmsConfig States
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
        /// Unspecified KmsConfig State
        Unspecified = 0,
        /// KmsConfig State is Ready
        Ready = 1,
        /// KmsConfig State is Creating
        Creating = 2,
        /// KmsConfig State is Deleting
        Deleting = 3,
        /// KmsConfig State is Updating
        Updating = 4,
        /// KmsConfig State is In Use.
        InUse = 5,
        /// KmsConfig State is Error
        Error = 6,
        /// KmsConfig State is Pending to verify crypto key access.
        KeyCheckPending = 7,
        /// KmsConfig State is Not accessbile by the SDE service account to the
        /// crypto key.
        KeyNotReachable = 8,
        /// KmsConfig State is Disabling.
        Disabling = 9,
        /// KmsConfig State is Disabled.
        Disabled = 10,
        /// KmsConfig State is Migrating.
        /// The existing volumes are migrating from SMEK to CMEK.
        Migrating = 11,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Ready => "READY",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::InUse => "IN_USE",
                State::Error => "ERROR",
                State::KeyCheckPending => "KEY_CHECK_PENDING",
                State::KeyNotReachable => "KEY_NOT_REACHABLE",
                State::Disabling => "DISABLING",
                State::Disabled => "DISABLED",
                State::Migrating => "MIGRATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "IN_USE" => Some(Self::InUse),
                "ERROR" => Some(Self::Error),
                "KEY_CHECK_PENDING" => Some(Self::KeyCheckPending),
                "KEY_NOT_REACHABLE" => Some(Self::KeyNotReachable),
                "DISABLING" => Some(Self::Disabling),
                "DISABLED" => Some(Self::Disabled),
                "MIGRATING" => Some(Self::Migrating),
                _ => None,
            }
        }
    }
}
/// TransferStats reports all statistics related to replication transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferStats {
    /// bytes trasferred so far in current transfer.
    #[prost(int64, optional, tag = "1")]
    pub transfer_bytes: ::core::option::Option<i64>,
    /// Total time taken during transfer.
    #[prost(message, optional, tag = "2")]
    pub total_transfer_duration: ::core::option::Option<::prost_types::Duration>,
    /// Last transfer size in bytes.
    #[prost(int64, optional, tag = "3")]
    pub last_transfer_bytes: ::core::option::Option<i64>,
    /// Time taken during last transfer.
    #[prost(message, optional, tag = "4")]
    pub last_transfer_duration: ::core::option::Option<::prost_types::Duration>,
    /// Lag duration indicates the duration by which Destination region volume
    /// content lags behind the primary region volume content.
    #[prost(message, optional, tag = "5")]
    pub lag_duration: ::core::option::Option<::prost_types::Duration>,
    /// Time when progress was updated last.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when last transfer completed.
    #[prost(message, optional, tag = "7")]
    pub last_transfer_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A message describing the cause of the last transfer failure.
    #[prost(string, optional, tag = "8")]
    pub last_transfer_error: ::core::option::Option<::prost::alloc::string::String>,
}
/// Replication is a nested resource under Volume, that describes a
/// cross-region replication relationship between 2 volumes in different
/// regions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replication {
    /// Identifier. The resource name of the Replication.
    /// Format:
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. State of the replication.
    #[prost(enumeration = "replication::State", tag = "2")]
    pub state: i32,
    /// Output only. State details of the replication.
    #[prost(string, tag = "3")]
    pub state_details: ::prost::alloc::string::String,
    /// Output only. Indicates whether this points to source or destination.
    #[prost(enumeration = "replication::ReplicationRole", tag = "4")]
    pub role: i32,
    /// Required. Indicates the schedule for replication.
    #[prost(enumeration = "replication::ReplicationSchedule", tag = "5")]
    pub replication_schedule: i32,
    /// Output only. Indicates the state of mirroring.
    #[prost(enumeration = "replication::MirrorState", tag = "6")]
    pub mirror_state: i32,
    /// Output only. Condition of the relationship. Can be one of the following:
    /// - true: The replication relationship is healthy. It has not missed the most
    /// recent scheduled transfer.
    /// - false: The replication relationship is not healthy. It has missed the
    /// most recent scheduled transfer.
    #[prost(bool, optional, tag = "8")]
    pub healthy: ::core::option::Option<bool>,
    /// Output only. Replication create time.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Full name of destination volume resource.
    /// Example : "projects/{project}/locations/{location}/volumes/{volume_id}"
    #[prost(string, tag = "10")]
    pub destination_volume: ::prost::alloc::string::String,
    /// Output only. Replication transfer statistics.
    #[prost(message, optional, tag = "11")]
    pub transfer_stats: ::core::option::Option<TransferStats>,
    /// Resource labels to represent user provided metadata.
    #[prost(btree_map = "string, string", tag = "12")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A description about this replication relationship.
    #[prost(string, optional, tag = "13")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Required. Input only. Destination volume parameters
    #[prost(message, optional, tag = "14")]
    pub destination_volume_parameters: ::core::option::Option<
        DestinationVolumeParameters,
    >,
    /// Output only. Full name of source volume resource.
    /// Example : "projects/{project}/locations/{location}/volumes/{volume_id}"
    #[prost(string, tag = "15")]
    pub source_volume: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Replication`.
pub mod replication {
    /// The replication states
    /// New enum values may be added in future to indicate possible new states.
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
        /// Unspecified replication State
        Unspecified = 0,
        /// Replication is creating.
        Creating = 1,
        /// Replication is ready.
        Ready = 2,
        /// Replication is updating.
        Updating = 3,
        /// Replication is deleting.
        Deleting = 5,
        /// Replication is in error state.
        Error = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Ready => "READY",
                State::Updating => "UPDATING",
                State::Deleting => "DELETING",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
    /// New enum values may be added in future to support different replication
    /// topology.
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
    pub enum ReplicationRole {
        /// Unspecified replication role
        Unspecified = 0,
        /// Indicates Source volume.
        Source = 1,
        /// Indicates Destination volume.
        Destination = 2,
    }
    impl ReplicationRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReplicationRole::Unspecified => "REPLICATION_ROLE_UNSPECIFIED",
                ReplicationRole::Source => "SOURCE",
                ReplicationRole::Destination => "DESTINATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REPLICATION_ROLE_UNSPECIFIED" => Some(Self::Unspecified),
                "SOURCE" => Some(Self::Source),
                "DESTINATION" => Some(Self::Destination),
                _ => None,
            }
        }
    }
    /// Schedule for Replication.
    /// New enum values may be added in future to support different frequency of
    /// replication.
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
    pub enum ReplicationSchedule {
        /// Unspecified ReplicationSchedule
        Unspecified = 0,
        /// Replication happens once every 10 minutes.
        Every10Minutes = 1,
        /// Replication happens once every hour.
        Hourly = 2,
        /// Replication happens once every day.
        Daily = 3,
    }
    impl ReplicationSchedule {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReplicationSchedule::Unspecified => "REPLICATION_SCHEDULE_UNSPECIFIED",
                ReplicationSchedule::Every10Minutes => "EVERY_10_MINUTES",
                ReplicationSchedule::Hourly => "HOURLY",
                ReplicationSchedule::Daily => "DAILY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REPLICATION_SCHEDULE_UNSPECIFIED" => Some(Self::Unspecified),
                "EVERY_10_MINUTES" => Some(Self::Every10Minutes),
                "HOURLY" => Some(Self::Hourly),
                "DAILY" => Some(Self::Daily),
                _ => None,
            }
        }
    }
    /// Mirroring states.
    /// No new value is expected to be added in future.
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
    pub enum MirrorState {
        /// Unspecified MirrorState
        Unspecified = 0,
        /// Destination volume is being prepared.
        Preparing = 1,
        /// Destination volume has been initialized and is ready to receive
        /// replication transfers.
        Mirrored = 2,
        /// Destination volume is not receiving replication transfers.
        Stopped = 3,
        /// Replication is in progress.
        Transferring = 4,
    }
    impl MirrorState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MirrorState::Unspecified => "MIRROR_STATE_UNSPECIFIED",
                MirrorState::Preparing => "PREPARING",
                MirrorState::Mirrored => "MIRRORED",
                MirrorState::Stopped => "STOPPED",
                MirrorState::Transferring => "TRANSFERRING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MIRROR_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PREPARING" => Some(Self::Preparing),
                "MIRRORED" => Some(Self::Mirrored),
                "STOPPED" => Some(Self::Stopped),
                "TRANSFERRING" => Some(Self::Transferring),
                _ => None,
            }
        }
    }
}
/// ListReplications lists replications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplicationsRequest {
    /// Required. The volume for which to retrieve replication information,
    /// in the format
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListReplicationsResponse is the result of ListReplicationsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReplicationsResponse {
    /// A list of replications in the project for the specified volume.
    #[prost(message, repeated, tag = "1")]
    pub replications: ::prost::alloc::vec::Vec<Replication>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetReplicationRequest gets the state of a replication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReplicationRequest {
    /// Required. The replication resource name, in the format
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DestinationVolumeParameters specify input parameters used for creating
/// destination volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationVolumeParameters {
    /// Required. Existing destination StoragePool name.
    #[prost(string, tag = "1")]
    pub storage_pool: ::prost::alloc::string::String,
    /// Desired destination volume resource id. If not specified, source volume's
    /// resource id will be used.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "2")]
    pub volume_id: ::prost::alloc::string::String,
    /// Destination volume's share name. If not specified, source volume's share
    /// name will be used.
    #[prost(string, tag = "3")]
    pub share_name: ::prost::alloc::string::String,
    /// Description for the destination volume.
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// CreateReplicationRequest creates a replication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReplicationRequest {
    /// Required. The NetApp volume to create the replications of, in the format
    /// `projects/{project_id}/locations/{location}/volumes/{volume_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A replication resource
    #[prost(message, optional, tag = "2")]
    pub replication: ::core::option::Option<Replication>,
    /// Required. ID of the replication to create.
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "3")]
    pub replication_id: ::prost::alloc::string::String,
}
/// DeleteReplicationRequest deletes a replication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReplicationRequest {
    /// Required. The replication resource name, in the format
    /// `projects/*/locations/*/volumes/*/replications/{replication_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateReplicationRequest updates description and/or labels for a replication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReplicationRequest {
    /// Required. Mask of fields to update.  At least one path must be supplied in
    /// this field.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. A replication resource
    #[prost(message, optional, tag = "2")]
    pub replication: ::core::option::Option<Replication>,
}
/// StopReplicationRequest stops a replication until resumed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationRequest {
    /// Required. The resource name of the replication, in the format of
    /// projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates whether to stop replication forcefully while data transfer is in
    /// progress.
    /// Warning! if force is true, this will abort any current transfers
    /// and can lead to data loss due to partial transfer.
    /// If force is false, stop replication will fail while data transfer is in
    /// progress and you will need to retry later.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// ResumeReplicationRequest resumes a stopped replication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeReplicationRequest {
    /// Required. The resource name of the replication, in the format of
    /// projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ReverseReplicationDirectionRequest reverses direction of replication. Source
/// becomes destination and destination becomes source.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReverseReplicationDirectionRequest {
    /// Required. The resource name of the replication, in the format of
    /// projects/{project_id}/locations/{location}/volumes/{volume_id}/replications/{replication_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The service levels - Storage Pool, Volumes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceLevel {
    /// Unspecified service level.
    Unspecified = 0,
    /// Premium service level.
    Premium = 1,
    /// Extreme service level.
    Extreme = 2,
    /// Standard (Software offering)
    Standard = 3,
}
impl ServiceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceLevel::Unspecified => "SERVICE_LEVEL_UNSPECIFIED",
            ServiceLevel::Premium => "PREMIUM",
            ServiceLevel::Extreme => "EXTREME",
            ServiceLevel::Standard => "STANDARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVICE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "PREMIUM" => Some(Self::Premium),
            "EXTREME" => Some(Self::Extreme),
            "STANDARD" => Some(Self::Standard),
            _ => None,
        }
    }
}
/// Defined the current volume encryption key source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncryptionType {
    /// The source of encryption key is not specified.
    Unspecified = 0,
    /// Google managed encryption key.
    ServiceManaged = 1,
    /// Customer managed encryption key, which is stored in KMS.
    CloudKms = 2,
}
impl EncryptionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncryptionType::Unspecified => "ENCRYPTION_TYPE_UNSPECIFIED",
            EncryptionType::ServiceManaged => "SERVICE_MANAGED",
            EncryptionType::CloudKms => "CLOUD_KMS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENCRYPTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SERVICE_MANAGED" => Some(Self::ServiceManaged),
            "CLOUD_KMS" => Some(Self::CloudKms),
            _ => None,
        }
    }
}
/// GetStoragePoolRequest gets a Storage Pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoragePoolRequest {
    /// Required. Name of the storage pool
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListStoragePoolsRequest lists Storage Pools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoragePoolsRequest {
    /// Required. Parent value
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListStoragePoolsResponse is the response to a ListStoragePoolsRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoragePoolsResponse {
    /// The list of StoragePools
    #[prost(message, repeated, tag = "1")]
    pub storage_pools: ::prost::alloc::vec::Vec<StoragePool>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CreateStoragePoolRequest creates a Storage Pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoragePoolRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting storage pool
    /// If auto-generating Id server-side, remove this field and
    /// id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub storage_pool_id: ::prost::alloc::string::String,
    /// Required. The required parameters to create a new storage pool.
    #[prost(message, optional, tag = "3")]
    pub storage_pool: ::core::option::Option<StoragePool>,
}
/// UpdateStoragePoolRequest updates a Storage Pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoragePoolRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// StoragePool resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The pool being updated
    #[prost(message, optional, tag = "2")]
    pub storage_pool: ::core::option::Option<StoragePool>,
}
/// DeleteStoragePoolRequest deletes a Storage Pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStoragePoolRequest {
    /// Required. Name of the storage pool
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// StoragePool is a container for volumes with a service level and capacity.
/// Volumes can be created in a pool of sufficient available capacity.
/// StoragePool capacity is what you are billed for.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePool {
    /// Identifier. Name of the storage pool
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Service level of the storage pool
    #[prost(enumeration = "ServiceLevel", tag = "2")]
    pub service_level: i32,
    /// Required. Capacity in GIB of the pool
    #[prost(int64, tag = "3")]
    pub capacity_gib: i64,
    /// Output only. Allocated size of all volumes in GIB in the storage pool
    #[prost(int64, tag = "4")]
    pub volume_capacity_gib: i64,
    /// Output only. Volume count of the storage pool
    #[prost(int32, tag = "5")]
    pub volume_count: i32,
    /// Output only. State of the storage pool
    #[prost(enumeration = "storage_pool::State", tag = "6")]
    pub state: i32,
    /// Output only. State details of the storage pool
    #[prost(string, tag = "7")]
    pub state_details: ::prost::alloc::string::String,
    /// Output only. Create time of the storage pool
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Description of the storage pool
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "10")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. VPC Network name.
    /// Format: projects/{project}/global/networks/{network}
    #[prost(string, tag = "11")]
    pub network: ::prost::alloc::string::String,
    /// Specifies the Active Directory to be used for creating a SMB volume.
    #[prost(string, tag = "12")]
    pub active_directory: ::prost::alloc::string::String,
    /// Specifies the KMS config to be used for volume encryption.
    #[prost(string, tag = "13")]
    pub kms_config: ::prost::alloc::string::String,
    /// Flag indicating if the pool is NFS LDAP enabled or not.
    #[prost(bool, tag = "14")]
    pub ldap_enabled: bool,
    /// Name of the Private Service Access allocated range. If
    /// not provided, any available range will be chosen.
    #[prost(string, tag = "15")]
    pub psa_range: ::prost::alloc::string::String,
    /// Output only. Specifies the current pool encryption key source.
    #[prost(enumeration = "EncryptionType", tag = "16")]
    pub encryption_type: i32,
    /// Deprecated. Used to allow SO pool to access AD or DNS server from other
    /// regions.
    #[deprecated]
    #[prost(bool, optional, tag = "17")]
    pub global_access_allowed: ::core::option::Option<bool>,
}
/// Nested message and enum types in `StoragePool`.
pub mod storage_pool {
    /// The Storage Pool States
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
        /// Unspecified Storage Pool State
        Unspecified = 0,
        /// Storage Pool State is Ready
        Ready = 1,
        /// Storage Pool State is Creating
        Creating = 2,
        /// Storage Pool State is Deleting
        Deleting = 3,
        /// Storage Pool State is Updating
        Updating = 4,
        /// Storage Pool State is Restoring
        Restoring = 5,
        /// Storage Pool State is Disabled
        Disabled = 6,
        /// Storage Pool State is Error
        Error = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Ready => "READY",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::Restoring => "RESTORING",
                State::Disabled => "DISABLED",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "RESTORING" => Some(Self::Restoring),
                "DISABLED" => Some(Self::Disabled),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Message for requesting list of Volumes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumesRequest {
    /// Required. Parent value for ListVolumesRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing Volumes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumesResponse {
    /// The list of Volume
    #[prost(message, repeated, tag = "1")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a Volume
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeRequest {
    /// Required. Name of the volume
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a Volume
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVolumeRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Id of the requesting volume
    /// If auto-generating Id server-side, remove this field and
    /// Id from the method_signature of Create RPC
    #[prost(string, tag = "2")]
    pub volume_id: ::prost::alloc::string::String,
    /// Required. The volume being created.
    #[prost(message, optional, tag = "3")]
    pub volume: ::core::option::Option<Volume>,
}
/// Message for updating a Volume
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVolumeRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Volume resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The volume being updated
    #[prost(message, optional, tag = "2")]
    pub volume: ::core::option::Option<Volume>,
}
/// Message for deleting a Volume
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVolumeRequest {
    /// Required. Name of the volume
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If this field is set as true, CCFE will not block the volume resource
    /// deletion even if it has any snapshots resource. (Otherwise, the request
    /// will only work if the volume has no snapshots.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// RevertVolumeRequest reverts the given volume to the specified snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevertVolumeRequest {
    /// Required. The resource name of the volume, in the format of
    /// projects/{project_id}/locations/{location}/volumes/{volume_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The snapshot resource ID, in the format 'my-snapshot', where the
    /// specified ID is the {snapshot_id} of the fully qualified name like
    /// projects/{project_id}/locations/{location_id}/volumes/{volume_id}/snapshots/{snapshot_id}
    #[prost(string, tag = "2")]
    pub snapshot_id: ::prost::alloc::string::String,
}
/// Volume provides a filesystem that you can mount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Identifier. Name of the volume
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. State of the volume
    #[prost(enumeration = "volume::State", tag = "2")]
    pub state: i32,
    /// Output only. State details of the volume
    #[prost(string, tag = "3")]
    pub state_details: ::prost::alloc::string::String,
    /// Output only. Create time of the volume
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Share name of the volume
    #[prost(string, tag = "5")]
    pub share_name: ::prost::alloc::string::String,
    /// Output only. Name of the Private Service Access allocated range. This is
    /// optional. If not provided, any available range will be chosen.
    #[prost(string, tag = "6")]
    pub psa_range: ::prost::alloc::string::String,
    /// Required. StoragePool name of the volume
    #[prost(string, tag = "7")]
    pub storage_pool: ::prost::alloc::string::String,
    /// Output only. VPC Network name.
    /// Format: projects/{project}/global/networks/{network}
    #[prost(string, tag = "8")]
    pub network: ::prost::alloc::string::String,
    /// Output only. Service level of the volume
    #[prost(enumeration = "ServiceLevel", tag = "9")]
    pub service_level: i32,
    /// Required. Capacity in GIB of the volume
    #[prost(int64, tag = "10")]
    pub capacity_gib: i64,
    /// Optional. Export policy of the volume
    #[prost(message, optional, tag = "11")]
    pub export_policy: ::core::option::Option<ExportPolicy>,
    /// Required. Protocols required for the volume
    #[prost(enumeration = "Protocols", repeated, packed = "false", tag = "12")]
    pub protocols: ::prost::alloc::vec::Vec<i32>,
    /// Optional. SMB share settings for the volume.
    #[prost(enumeration = "SmbSettings", repeated, packed = "false", tag = "13")]
    pub smb_settings: ::prost::alloc::vec::Vec<i32>,
    /// Output only. Mount options of this volume
    #[prost(message, repeated, tag = "14")]
    pub mount_options: ::prost::alloc::vec::Vec<MountOption>,
    /// Optional. Default unix style permission (e.g. 777) the mount point will be
    /// created with. Applicable for NFS protocol types only.
    #[prost(string, tag = "15")]
    pub unix_permissions: ::prost::alloc::string::String,
    /// Optional. Labels as key value pairs
    #[prost(btree_map = "string, string", tag = "16")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. Description of the volume
    #[prost(string, tag = "17")]
    pub description: ::prost::alloc::string::String,
    /// Optional. SnapshotPolicy for a volume.
    #[prost(message, optional, tag = "18")]
    pub snapshot_policy: ::core::option::Option<SnapshotPolicy>,
    /// Optional. Snap_reserve specifies percentage of volume storage reserved for
    /// snapshot storage. Default is 0 percent.
    #[prost(double, tag = "19")]
    pub snap_reserve: f64,
    /// Optional. Snapshot_directory if enabled (true) the volume will contain a
    /// read-only .snapshot directory which provides access to each of the volume's
    /// snapshots.
    #[prost(bool, tag = "20")]
    pub snapshot_directory: bool,
    /// Output only. Used capacity in GIB of the volume. This is computed
    /// periodically and it does not represent the realtime usage.
    #[prost(int64, tag = "21")]
    pub used_gib: i64,
    /// Optional. Security Style of the Volume
    #[prost(enumeration = "SecurityStyle", tag = "22")]
    pub security_style: i32,
    /// Optional. Flag indicating if the volume is a kerberos volume or not, export
    /// policy rules control kerberos security modes (krb5, krb5i, krb5p).
    #[prost(bool, tag = "23")]
    pub kerberos_enabled: bool,
    /// Output only. Flag indicating if the volume is NFS LDAP enabled or not.
    #[prost(bool, tag = "24")]
    pub ldap_enabled: bool,
    /// Output only. Specifies the ActiveDirectory name of a SMB volume.
    #[prost(string, tag = "25")]
    pub active_directory: ::prost::alloc::string::String,
    /// Optional. Specifies the source of the volume to be created from.
    #[prost(message, optional, tag = "26")]
    pub restore_parameters: ::core::option::Option<RestoreParameters>,
    /// Output only. Specifies the KMS config to be used for volume encryption.
    #[prost(string, tag = "27")]
    pub kms_config: ::prost::alloc::string::String,
    /// Output only. Specified the current volume encryption key source.
    #[prost(enumeration = "EncryptionType", tag = "28")]
    pub encryption_type: i32,
    /// Output only. Indicates whether the volume is part of a replication
    /// relationship.
    #[prost(bool, tag = "29")]
    pub has_replication: bool,
    /// BackupConfig of the volume.
    #[prost(message, optional, tag = "30")]
    pub backup_config: ::core::option::Option<BackupConfig>,
    /// Optional. List of actions that are restricted on this volume.
    #[prost(enumeration = "RestrictedAction", repeated, packed = "false", tag = "31")]
    pub restricted_actions: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    /// The volume states
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
        /// Unspecified Volume State
        Unspecified = 0,
        /// Volume State is Ready
        Ready = 1,
        /// Volume State is Creating
        Creating = 2,
        /// Volume State is Deleting
        Deleting = 3,
        /// Volume State is Updating
        Updating = 4,
        /// Volume State is Restoring
        Restoring = 5,
        /// Volume State is Disabled
        Disabled = 6,
        /// Volume State is Error
        Error = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Ready => "READY",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Updating => "UPDATING",
                State::Restoring => "RESTORING",
                State::Disabled => "DISABLED",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "READY" => Some(Self::Ready),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "UPDATING" => Some(Self::Updating),
                "RESTORING" => Some(Self::Restoring),
                "DISABLED" => Some(Self::Disabled),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Defines the export policy for the volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportPolicy {
    /// Required. List of export policy rules
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<SimpleExportPolicyRule>,
}
/// An export policy rule describing various export options.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleExportPolicyRule {
    /// Comma separated list of allowed clients IP addresses
    #[prost(string, optional, tag = "1")]
    pub allowed_clients: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether Unix root access will be granted.
    #[prost(string, optional, tag = "2")]
    pub has_root_access: ::core::option::Option<::prost::alloc::string::String>,
    /// Access type (ReadWrite, ReadOnly, None)
    #[prost(enumeration = "AccessType", optional, tag = "3")]
    pub access_type: ::core::option::Option<i32>,
    /// NFS V3 protocol.
    #[prost(bool, optional, tag = "4")]
    pub nfsv3: ::core::option::Option<bool>,
    /// NFS V4 protocol.
    #[prost(bool, optional, tag = "5")]
    pub nfsv4: ::core::option::Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching
    /// the 'allowedClients' specification. It enables nfs clients to mount using
    /// 'authentication' kerberos security mode.
    #[prost(bool, optional, tag = "6")]
    pub kerberos_5_read_only: ::core::option::Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients
    /// matching the 'allowedClients' specification. It enables nfs clients to
    /// mount using 'authentication' kerberos security mode. The
    /// 'kerberos5ReadOnly' value be ignored if this is enabled.
    #[prost(bool, optional, tag = "7")]
    pub kerberos_5_read_write: ::core::option::Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching
    /// the 'allowedClients' specification. It enables nfs clients to mount using
    /// 'integrity' kerberos security mode.
    #[prost(bool, optional, tag = "8")]
    pub kerberos_5i_read_only: ::core::option::Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients
    /// matching the 'allowedClients' specification. It enables nfs clients to
    /// mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly'
    /// value be ignored if this is enabled.
    #[prost(bool, optional, tag = "9")]
    pub kerberos_5i_read_write: ::core::option::Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching
    /// the 'allowedClients' specification. It enables nfs clients to mount using
    /// 'privacy' kerberos security mode.
    #[prost(bool, optional, tag = "10")]
    pub kerberos_5p_read_only: ::core::option::Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients
    /// matching the 'allowedClients' specification. It enables nfs clients to
    /// mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly'
    /// value be ignored if this is enabled.
    #[prost(bool, optional, tag = "11")]
    pub kerberos_5p_read_write: ::core::option::Option<bool>,
}
/// Snapshot Policy for a volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotPolicy {
    /// If enabled, make snapshots automatically according to the schedules.
    /// Default is false.
    #[prost(bool, optional, tag = "1")]
    pub enabled: ::core::option::Option<bool>,
    /// Hourly schedule policy.
    #[prost(message, optional, tag = "2")]
    pub hourly_schedule: ::core::option::Option<HourlySchedule>,
    /// Daily schedule policy.
    #[prost(message, optional, tag = "3")]
    pub daily_schedule: ::core::option::Option<DailySchedule>,
    /// Weekly schedule policy.
    #[prost(message, optional, tag = "4")]
    pub weekly_schedule: ::core::option::Option<WeeklySchedule>,
    /// Monthly schedule policy.
    #[prost(message, optional, tag = "5")]
    pub monthly_schedule: ::core::option::Option<MonthlySchedule>,
}
/// Make a snapshot every hour e.g. at 04:00, 05:00, 06:00.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HourlySchedule {
    /// The maximum number of Snapshots to keep for the hourly schedule
    #[prost(double, optional, tag = "1")]
    pub snapshots_to_keep: ::core::option::Option<f64>,
    /// Set the minute of the hour to start the snapshot (0-59), defaults to the
    /// top of the hour (0).
    #[prost(double, optional, tag = "2")]
    pub minute: ::core::option::Option<f64>,
}
/// Make a snapshot every day e.g. at 04:00, 05:20, 23:50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailySchedule {
    /// The maximum number of Snapshots to keep for the hourly schedule
    #[prost(double, optional, tag = "1")]
    pub snapshots_to_keep: ::core::option::Option<f64>,
    /// Set the minute of the hour to start the snapshot (0-59), defaults to the
    /// top of the hour (0).
    #[prost(double, optional, tag = "2")]
    pub minute: ::core::option::Option<f64>,
    /// Set the hour to start the snapshot (0-23), defaults to midnight (0).
    #[prost(double, optional, tag = "3")]
    pub hour: ::core::option::Option<f64>,
}
/// Make a snapshot every week e.g. at Monday 04:00, Wednesday 05:20, Sunday
/// 23:50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeeklySchedule {
    /// The maximum number of Snapshots to keep for the hourly schedule
    #[prost(double, optional, tag = "1")]
    pub snapshots_to_keep: ::core::option::Option<f64>,
    /// Set the minute of the hour to start the snapshot (0-59), defaults to the
    /// top of the hour (0).
    #[prost(double, optional, tag = "2")]
    pub minute: ::core::option::Option<f64>,
    /// Set the hour to start the snapshot (0-23), defaults to midnight (0).
    #[prost(double, optional, tag = "3")]
    pub hour: ::core::option::Option<f64>,
    /// Set the day or days of the week to make a snapshot. Accepts a comma
    /// separated days of the week. Defaults to 'Sunday'.
    #[prost(string, optional, tag = "4")]
    pub day: ::core::option::Option<::prost::alloc::string::String>,
}
/// Make a snapshot once a month e.g. at 2nd 04:00, 7th 05:20, 24th 23:50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlySchedule {
    /// The maximum number of Snapshots to keep for the hourly schedule
    #[prost(double, optional, tag = "1")]
    pub snapshots_to_keep: ::core::option::Option<f64>,
    /// Set the minute of the hour to start the snapshot (0-59), defaults to the
    /// top of the hour (0).
    #[prost(double, optional, tag = "2")]
    pub minute: ::core::option::Option<f64>,
    /// Set the hour to start the snapshot (0-23), defaults to midnight (0).
    #[prost(double, optional, tag = "3")]
    pub hour: ::core::option::Option<f64>,
    /// Set the day or days of the month to make a snapshot (1-31). Accepts a
    /// comma separated number of days. Defaults to '1'.
    #[prost(string, optional, tag = "4")]
    pub days_of_month: ::core::option::Option<::prost::alloc::string::String>,
}
/// View only mount options for a volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountOption {
    /// Export string
    #[prost(string, tag = "1")]
    pub export: ::prost::alloc::string::String,
    /// Full export string
    #[prost(string, tag = "2")]
    pub export_full: ::prost::alloc::string::String,
    /// Protocol to mount with.
    #[prost(enumeration = "Protocols", tag = "3")]
    pub protocol: i32,
    /// Instructions for mounting
    #[prost(string, tag = "4")]
    pub instructions: ::prost::alloc::string::String,
}
/// The RestoreParameters if volume is created from a snapshot or backup.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreParameters {
    /// The source that the volume is created from.
    #[prost(oneof = "restore_parameters::Source", tags = "1, 2")]
    pub source: ::core::option::Option<restore_parameters::Source>,
}
/// Nested message and enum types in `RestoreParameters`.
pub mod restore_parameters {
    /// The source that the volume is created from.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Full name of the snapshot resource.
        /// Format:
        /// projects/{project}/locations/{location}/volumes/{volume}/snapshots/{snapshot}
        #[prost(string, tag = "1")]
        SourceSnapshot(::prost::alloc::string::String),
        /// Full name of the backup resource.
        /// Format:
        /// projects/{project}/locations/{location}/backupVaults/{backup_vault_id}/backups/{backup_id}
        #[prost(string, tag = "2")]
        SourceBackup(::prost::alloc::string::String),
    }
}
/// BackupConfig contains backup related config on a volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupConfig {
    /// Optional. When specified, schedule backups will be created based on the
    /// policy configuration.
    #[prost(string, repeated, tag = "1")]
    pub backup_policies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Name of backup vault.
    /// Format:
    /// projects/{project_id}/locations/{location}/backupVaults/{backup_vault_id}
    #[prost(string, tag = "2")]
    pub backup_vault: ::prost::alloc::string::String,
    /// Optional. When set to true, scheduled backup is enabled on the volume.
    /// This field should be nil when there's no backup policy attached.
    #[prost(bool, optional, tag = "3")]
    pub scheduled_backup_enabled: ::core::option::Option<bool>,
}
/// Protocols is an enum of all the supported network protocols for a volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Protocols {
    /// Unspecified protocol
    Unspecified = 0,
    /// NFS V3 protocol
    Nfsv3 = 1,
    /// NFS V4 protocol
    Nfsv4 = 2,
    /// SMB protocol
    Smb = 3,
}
impl Protocols {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Protocols::Unspecified => "PROTOCOLS_UNSPECIFIED",
            Protocols::Nfsv3 => "NFSV3",
            Protocols::Nfsv4 => "NFSV4",
            Protocols::Smb => "SMB",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROTOCOLS_UNSPECIFIED" => Some(Self::Unspecified),
            "NFSV3" => Some(Self::Nfsv3),
            "NFSV4" => Some(Self::Nfsv4),
            "SMB" => Some(Self::Smb),
            _ => None,
        }
    }
}
/// AccessType is an enum of all the supported access types for a volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessType {
    /// Unspecified Access Type
    Unspecified = 0,
    /// Read Only
    ReadOnly = 1,
    /// Read Write
    ReadWrite = 2,
    /// None
    ReadNone = 3,
}
impl AccessType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessType::Unspecified => "ACCESS_TYPE_UNSPECIFIED",
            AccessType::ReadOnly => "READ_ONLY",
            AccessType::ReadWrite => "READ_WRITE",
            AccessType::ReadNone => "READ_NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "READ_ONLY" => Some(Self::ReadOnly),
            "READ_WRITE" => Some(Self::ReadWrite),
            "READ_NONE" => Some(Self::ReadNone),
            _ => None,
        }
    }
}
/// SMBSettings
/// Modifies the behaviour of a SMB volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SmbSettings {
    /// Unspecified default option
    Unspecified = 0,
    /// SMB setting encrypt data
    EncryptData = 1,
    /// SMB setting browsable
    Browsable = 2,
    /// SMB setting notify change
    ChangeNotify = 3,
    /// SMB setting not to notify change
    NonBrowsable = 4,
    /// SMB setting oplocks
    Oplocks = 5,
    /// SMB setting to show snapshots
    ShowSnapshot = 6,
    /// SMB setting to show previous versions
    ShowPreviousVersions = 7,
    /// SMB setting to access volume based on enumerartion
    AccessBasedEnumeration = 8,
    /// Continuously available enumeration
    ContinuouslyAvailable = 9,
}
impl SmbSettings {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SmbSettings::Unspecified => "SMB_SETTINGS_UNSPECIFIED",
            SmbSettings::EncryptData => "ENCRYPT_DATA",
            SmbSettings::Browsable => "BROWSABLE",
            SmbSettings::ChangeNotify => "CHANGE_NOTIFY",
            SmbSettings::NonBrowsable => "NON_BROWSABLE",
            SmbSettings::Oplocks => "OPLOCKS",
            SmbSettings::ShowSnapshot => "SHOW_SNAPSHOT",
            SmbSettings::ShowPreviousVersions => "SHOW_PREVIOUS_VERSIONS",
            SmbSettings::AccessBasedEnumeration => "ACCESS_BASED_ENUMERATION",
            SmbSettings::ContinuouslyAvailable => "CONTINUOUSLY_AVAILABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SMB_SETTINGS_UNSPECIFIED" => Some(Self::Unspecified),
            "ENCRYPT_DATA" => Some(Self::EncryptData),
            "BROWSABLE" => Some(Self::Browsable),
            "CHANGE_NOTIFY" => Some(Self::ChangeNotify),
            "NON_BROWSABLE" => Some(Self::NonBrowsable),
            "OPLOCKS" => Some(Self::Oplocks),
            "SHOW_SNAPSHOT" => Some(Self::ShowSnapshot),
            "SHOW_PREVIOUS_VERSIONS" => Some(Self::ShowPreviousVersions),
            "ACCESS_BASED_ENUMERATION" => Some(Self::AccessBasedEnumeration),
            "CONTINUOUSLY_AVAILABLE" => Some(Self::ContinuouslyAvailable),
            _ => None,
        }
    }
}
/// The security style of the volume, can be either UNIX or NTFS.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityStyle {
    /// SecurityStyle is unspecified
    Unspecified = 0,
    /// SecurityStyle uses NTFS
    Ntfs = 1,
    /// SecurityStyle uses NTFS
    Unix = 2,
}
impl SecurityStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityStyle::Unspecified => "SECURITY_STYLE_UNSPECIFIED",
            SecurityStyle::Ntfs => "NTFS",
            SecurityStyle::Unix => "UNIX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECURITY_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
            "NTFS" => Some(Self::Ntfs),
            "UNIX" => Some(Self::Unix),
            _ => None,
        }
    }
}
/// Actions to be restricted for a volume.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RestrictedAction {
    /// Unspecified restricted action
    Unspecified = 0,
    /// Prevent volume from being deleted when mounted.
    Delete = 1,
}
impl RestrictedAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RestrictedAction::Unspecified => "RESTRICTED_ACTION_UNSPECIFIED",
            RestrictedAction::Delete => "DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESTRICTED_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been canceled successfully
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod net_app_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// NetApp Files Google Cloud Service
    #[derive(Debug, Clone)]
    pub struct NetAppClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NetAppClient<T>
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
        ) -> NetAppClient<InterceptedService<T, F>>
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
            NetAppClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns descriptions of all storage pools owned by the caller.
        pub async fn list_storage_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStoragePoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStoragePoolsResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListStoragePools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListStoragePools"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new storage pool.
        pub async fn create_storage_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStoragePoolRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateStoragePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateStoragePool"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the description of the specified storage pool by poolId.
        pub async fn get_storage_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStoragePoolRequest>,
        ) -> std::result::Result<tonic::Response<super::StoragePool>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetStoragePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetStoragePool"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the storage pool properties with the full spec
        pub async fn update_storage_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateStoragePoolRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateStoragePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateStoragePool"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Warning! This operation will permanently delete the storage pool.
        pub async fn delete_storage_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStoragePoolRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteStoragePool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteStoragePool"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Volumes in a given project.
        pub async fn list_volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListVolumesResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListVolumes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListVolumes"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single Volume.
        pub async fn get_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeRequest>,
        ) -> std::result::Result<tonic::Response<super::Volume>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetVolume"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new Volume in a given project and location.
        pub async fn create_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVolumeRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateVolume"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single Volume.
        pub async fn update_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVolumeRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateVolume"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single Volume.
        pub async fn delete_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVolumeRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteVolume"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Revert an existing volume to a specified snapshot.
        /// Warning! This operation will permanently revert all changes made after the
        /// snapshot was created.
        pub async fn revert_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::RevertVolumeRequest>,
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
                "/google.cloud.netapp.v1.NetApp/RevertVolume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "RevertVolume"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns descriptions of all snapshots for a volume.
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSnapshotsResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListSnapshots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListSnapshots"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Describe a snapshot for a volume.
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> std::result::Result<tonic::Response<super::Snapshot>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetSnapshot"));
            self.inner.unary(req, path, codec).await
        }
        /// Create a new snapshot for a volume.
        pub async fn create_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSnapshotRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateSnapshot"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a snapshot.
        pub async fn delete_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteSnapshot"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific snapshot.
        pub async fn update_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSnapshotRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateSnapshot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateSnapshot"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists active directories.
        pub async fn list_active_directories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListActiveDirectoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListActiveDirectoriesResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListActiveDirectories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "ListActiveDirectories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Describes a specified active directory.
        pub async fn get_active_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActiveDirectoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActiveDirectory>,
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
                "/google.cloud.netapp.v1.NetApp/GetActiveDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "GetActiveDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateActiveDirectory
        /// Creates the active directory specified in the request.
        pub async fn create_active_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateActiveDirectoryRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateActiveDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "CreateActiveDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update the parameters of an active directories.
        pub async fn update_active_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateActiveDirectoryRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateActiveDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "UpdateActiveDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the active directory specified in the request.
        pub async fn delete_active_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteActiveDirectoryRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteActiveDirectory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "DeleteActiveDirectory",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns descriptions of all KMS configs owned by the caller.
        pub async fn list_kms_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKmsConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKmsConfigsResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListKmsConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListKmsConfigs"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new KMS config.
        pub async fn create_kms_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKmsConfigRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateKmsConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateKmsConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the description of the specified KMS config by kms_config_id.
        pub async fn get_kms_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKmsConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::KmsConfig>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetKmsConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetKmsConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the Kms config properties with the full spec
        pub async fn update_kms_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateKmsConfigRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateKmsConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateKmsConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Encrypt the existing volumes without CMEK encryption with the desired the
        /// KMS config for the whole region.
        pub async fn encrypt_volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::EncryptVolumesRequest>,
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
                "/google.cloud.netapp.v1.NetApp/EncryptVolumes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "EncryptVolumes"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verifies KMS config reachability.
        pub async fn verify_kms_config(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyKmsConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyKmsConfigResponse>,
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
                "/google.cloud.netapp.v1.NetApp/VerifyKmsConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "VerifyKmsConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Warning! This operation will permanently delete the Kms config.
        pub async fn delete_kms_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKmsConfigRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteKmsConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteKmsConfig"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns descriptions of all replications for a volume.
        pub async fn list_replications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReplicationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListReplicationsResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListReplications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListReplications"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Describe a replication for a volume.
        pub async fn get_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReplicationRequest>,
        ) -> std::result::Result<tonic::Response<super::Replication>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetReplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a new replication for a volume.
        pub async fn create_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReplicationRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateReplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a replication.
        pub async fn delete_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReplicationRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteReplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific replication.
        pub async fn update_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReplicationRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateReplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stop Cross Region Replication.
        pub async fn stop_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::StopReplicationRequest>,
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
                "/google.cloud.netapp.v1.NetApp/StopReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "StopReplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resume Cross Region Replication.
        pub async fn resume_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeReplicationRequest>,
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
                "/google.cloud.netapp.v1.NetApp/ResumeReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ResumeReplication"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Reverses direction of replication. Source becomes destination and
        /// destination becomes source.
        pub async fn reverse_replication_direction(
            &mut self,
            request: impl tonic::IntoRequest<super::ReverseReplicationDirectionRequest>,
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
                "/google.cloud.netapp.v1.NetApp/ReverseReplicationDirection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "ReverseReplicationDirection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates new backup vault
        pub async fn create_backup_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupVaultRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateBackupVault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateBackupVault"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the description of the specified backup vault
        pub async fn get_backup_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupVaultRequest>,
        ) -> std::result::Result<tonic::Response<super::BackupVault>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetBackupVault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetBackupVault"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns list of all available backup vaults.
        pub async fn list_backup_vaults(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupVaultsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBackupVaultsResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListBackupVaults",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListBackupVaults"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the settings of a specific backup vault.
        pub async fn update_backup_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupVaultRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateBackupVault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateBackupVault"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Warning! This operation will permanently delete the backup vault.
        pub async fn delete_backup_vault(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupVaultRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteBackupVault",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteBackupVault"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a backup from the volume specified in the request
        /// The backup can be created from the given snapshot if specified in the
        /// request. If no snapshot specified, there'll be a new snapshot taken to
        /// initiate the backup creation.
        pub async fn create_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "CreateBackup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the description of the specified backup
        pub async fn get_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupRequest>,
        ) -> std::result::Result<tonic::Response<super::Backup>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetBackup"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns descriptions of all backups for a backupVault.
        pub async fn list_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBackupsResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListBackups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.netapp.v1.NetApp", "ListBackups"));
            self.inner.unary(req, path, codec).await
        }
        /// Warning! This operation will permanently delete the backup.
        pub async fn delete_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "DeleteBackup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update backup with full spec.
        pub async fn update_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "UpdateBackup"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates new backup policy
        pub async fn create_backup_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupPolicyRequest>,
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
                "/google.cloud.netapp.v1.NetApp/CreateBackupPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "CreateBackupPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the description of the specified backup policy by backup_policy_id.
        pub async fn get_backup_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupPolicyRequest>,
        ) -> std::result::Result<tonic::Response<super::BackupPolicy>, tonic::Status> {
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
                "/google.cloud.netapp.v1.NetApp/GetBackupPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.netapp.v1.NetApp", "GetBackupPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns list of all available backup policies.
        pub async fn list_backup_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupPoliciesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListBackupPoliciesResponse>,
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
                "/google.cloud.netapp.v1.NetApp/ListBackupPolicies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "ListBackupPolicies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates settings of a specific backup policy.
        pub async fn update_backup_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupPolicyRequest>,
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
                "/google.cloud.netapp.v1.NetApp/UpdateBackupPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "UpdateBackupPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Warning! This operation will permanently delete the backup policy.
        pub async fn delete_backup_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupPolicyRequest>,
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
                "/google.cloud.netapp.v1.NetApp/DeleteBackupPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.netapp.v1.NetApp",
                        "DeleteBackupPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
