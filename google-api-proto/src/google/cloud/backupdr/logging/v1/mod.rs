/// This is a BackupRecoveryJobReportLog published as part of GCBDR Reporting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRecoveryJobReportLog {
    /// The job_name field displays the name of the job being reported.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// The job_name field displays the category of the job whether it is Backup or
    /// Recovery Job.
    #[prost(string, tag = "2")]
    pub job_category: ::prost::alloc::string::String,
    /// The job_type field displays the type of the job.
    #[prost(string, tag = "3")]
    pub job_type: ::prost::alloc::string::String,
    /// The log_backup field displays whether the backup taken is only for logs, DB
    /// or both.
    #[prost(string, tag = "4")]
    pub log_backup: ::prost::alloc::string::String,
    /// The job_status field displays the status of the job.
    #[prost(string, tag = "5")]
    pub job_status: ::prost::alloc::string::String,
    /// The resource_name field displays the name of the resource.
    #[prost(string, tag = "6")]
    pub resource_name: ::prost::alloc::string::String,
    /// The resource_type field displays the type of the resource.
    #[prost(string, tag = "7")]
    pub resource_type: ::prost::alloc::string::String,
    /// The error_code field displays the error code.
    #[prost(int32, tag = "8")]
    pub error_code: i32,
    /// The error_message field displays the error message if the job is not
    /// successful.
    #[prost(string, tag = "9")]
    pub error_message: ::prost::alloc::string::String,
    /// The job_initiation_failure_reason field displays the reason for failure,
    /// if the job was not run.
    #[prost(string, tag = "10")]
    pub job_initiation_failure_reason: ::prost::alloc::string::String,
    /// The job_start_time field displays the timestamp when the job started.
    #[prost(string, tag = "11")]
    pub job_start_time: ::prost::alloc::string::String,
    /// The job_end_time field displays the timestamp when the job ended.
    #[prost(string, tag = "12")]
    pub job_end_time: ::prost::alloc::string::String,
    /// The job_queued_time field displays the timestamp when the job was
    /// queued for running.
    #[prost(string, tag = "13")]
    pub job_queued_time: ::prost::alloc::string::String,
    /// The job_duration_in_hours field displays the duration in hours which the
    /// job took to complete.
    #[prost(double, tag = "14")]
    pub job_duration_in_hours: f64,
    /// The hostname field displays the name of the host.
    #[prost(string, tag = "15")]
    pub hostname: ::prost::alloc::string::String,
    /// The appliance_name field displays the name of the backup appliance.
    #[prost(string, tag = "16")]
    pub appliance_name: ::prost::alloc::string::String,
    /// The backup_rule_policy_name field displays the policy name which is
    /// associated with this job.
    #[prost(string, tag = "17")]
    pub backup_rule_policy_name: ::prost::alloc::string::String,
    /// The backup_plan_policy_template field displays the name of the backup plan
    /// for this application.
    #[prost(string, tag = "18")]
    pub backup_plan_policy_template: ::prost::alloc::string::String,
    /// The backup_type field displays the type of backup taken: Log, Incremental
    /// or Full Copy.
    #[prost(string, tag = "19")]
    pub backup_type: ::prost::alloc::string::String,
    /// The recovery_point field displays the timestamp of recovery point.
    #[prost(string, tag = "20")]
    pub recovery_point: ::prost::alloc::string::String,
    /// The backup_consistency field displays whether the backup is crash
    /// consistent or application consistent.
    #[prost(string, tag = "21")]
    pub backup_consistency: ::prost::alloc::string::String,
    /// The target_host_name field displays the target host name.
    #[prost(string, tag = "22")]
    pub target_host_name: ::prost::alloc::string::String,
    /// The target_appliance_name field displays the target appliance name.
    #[prost(string, tag = "23")]
    pub target_appliance_name: ::prost::alloc::string::String,
    /// The target_pool_name field displays target pool name.
    #[prost(string, tag = "24")]
    pub target_pool_name: ::prost::alloc::string::String,
    /// The resource_data_size_in_gib field displays resource data size in Gib.
    #[prost(double, tag = "25")]
    pub resource_data_size_in_gib: f64,
    /// The data_copied_in_gib field displays the amount of the data copied
    /// during backup in Gib.
    #[prost(double, tag = "26")]
    pub data_copied_in_gib: f64,
    /// The onvault_pool_storage_consumed_in_gib field displays the amount of
    /// onvault pool storage consumed in Gib.
    #[prost(double, tag = "27")]
    pub onvault_pool_storage_consumed_in_gib: f64,
    /// The pre_compress_in_gib field displays the size before compression in Gib.
    #[prost(double, tag = "28")]
    pub pre_compress_in_gib: f64,
    /// The compression_ratio field displays the ratio of post compression size to
    /// pre compression size.
    #[prost(double, tag = "29")]
    pub compression_ratio: f64,
    /// The data_change_rate field displays the percentage of data copied during
    /// backup to application size.
    #[prost(double, tag = "30")]
    pub data_change_rate: f64,
    /// The snapshot_disk_size_in_gib field displays the snapshot disk size in Gib.
    #[prost(double, tag = "31")]
    pub snapshot_disk_size_in_gib: f64,
    /// The data_written_in_gib field displays the amount of
    /// remote data written in Gib.
    #[prost(double, tag = "32")]
    pub data_written_in_gib: f64,
    /// The data_sent_in_gib field displays the amount of
    /// network data sent in Gib.
    #[prost(double, tag = "33")]
    pub data_sent_in_gib: f64,
    /// The job_id field displays the id of the job being reported.
    #[prost(string, tag = "34")]
    pub job_id: ::prost::alloc::string::String,
    /// The host_id field displays the host id.
    #[prost(string, tag = "35")]
    pub host_id: ::prost::alloc::string::String,
    /// The backup_rule_policy_id field displays the policy id.
    #[prost(string, tag = "36")]
    pub backup_rule_policy_id: ::prost::alloc::string::String,
    /// The resource_id field displays the resource id.
    #[prost(string, tag = "37")]
    pub resource_id: ::prost::alloc::string::String,
    /// The target_pool_id field displays the target pool id.
    #[prost(string, tag = "38")]
    pub target_pool_id: ::prost::alloc::string::String,
    /// The target_host_id field displays the target host id.
    #[prost(string, tag = "39")]
    pub target_host_id: ::prost::alloc::string::String,
    /// The target_appliance_id field displays the target appliance id.
    #[prost(string, tag = "40")]
    pub target_appliance_id: ::prost::alloc::string::String,
}
/// This is a UnprotectedResourceLogReport published as part of GCBDR
/// Reporting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnprotectedResourceReportLog {
    /// Required. Name of the host where the application/resource resides.
    #[prost(string, tag = "1")]
    pub host_name: ::prost::alloc::string::String,
    /// Required. Name of the application/resource.
    #[prost(string, tag = "2")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Type of the application/resource.
    #[prost(string, tag = "3")]
    pub resource_type: ::prost::alloc::string::String,
    /// Optional. Name of the database instance for instance members.
    #[prost(string, tag = "4")]
    pub instance_name: ::prost::alloc::string::String,
    /// Required. Date when the Appliance was Discovered first.
    #[prost(string, tag = "5")]
    pub discovered_on: ::prost::alloc::string::String,
    /// Required. Name of the appliance on which it was discovered.
    #[prost(string, tag = "6")]
    pub discovered_by: ::prost::alloc::string::String,
    /// Required. Id of the Appliance
    #[prost(string, tag = "7")]
    pub appliance_id: ::prost::alloc::string::String,
    /// Required. Id of the application/resource
    #[prost(string, tag = "8")]
    pub resource_id: ::prost::alloc::string::String,
    /// Required. Id of the Host where the application/resource resides.
    #[prost(string, tag = "9")]
    pub host_id: ::prost::alloc::string::String,
}
/// This is a DailyScheduleComplianceReportLog published as part of GCBDR
/// Reporting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyScheduleComplianceReportLog {
    /// Required. Resource/App Name.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Resource/App Type.
    #[prost(string, tag = "2")]
    pub resource_type: ::prost::alloc::string::String,
    /// Required. Policy Name.
    #[prost(string, tag = "3")]
    pub backup_rule_policy_name: ::prost::alloc::string::String,
    /// Required. Policy Template Name.
    #[prost(string, tag = "4")]
    pub backup_plan_policy_template: ::prost::alloc::string::String,
    /// Required. Name of the host where the app/resource resides.
    #[prost(string, tag = "5")]
    pub host_name: ::prost::alloc::string::String,
    /// Required. Appliance Name.
    #[prost(string, tag = "6")]
    pub appliance_name: ::prost::alloc::string::String,
    /// Required. Date for which compliance is being reported.
    #[prost(string, tag = "7")]
    pub date: ::prost::alloc::string::String,
    /// Required. Backup Window Start time.
    #[prost(string, tag = "8")]
    pub backup_window_start_time: ::prost::alloc::string::String,
    /// Required. JobType of the policy.
    #[prost(string, tag = "9")]
    pub job_type: ::prost::alloc::string::String,
    /// Required. Compliance status for the policy.
    #[prost(string, tag = "10")]
    pub status: ::prost::alloc::string::String,
    /// Required. Description for the status reason.
    #[prost(string, tag = "11")]
    pub comment: ::prost::alloc::string::String,
    /// Required. Resource/App Id.
    #[prost(string, tag = "12")]
    pub resource_id: ::prost::alloc::string::String,
    /// Required. Host Id.
    #[prost(string, tag = "13")]
    pub host_id: ::prost::alloc::string::String,
    /// Required. Policy Template Id
    #[prost(string, tag = "14")]
    pub backup_plan_policy_template_id: ::prost::alloc::string::String,
    /// Required. Policy Id.
    #[prost(string, tag = "15")]
    pub backup_rule_policy_id: ::prost::alloc::string::String,
    /// Required. Appliance Id.
    #[prost(string, tag = "16")]
    pub appliance_id: ::prost::alloc::string::String,
}
/// This is a BackupStorageUtilizationReportLog published as part of GCBDR
/// Reporting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupStorageUtilizationReportLog {
    /// Optional. The appliance name.
    #[prost(string, tag = "1")]
    pub appliance_name: ::prost::alloc::string::String,
    /// Required. The storage resource type.
    #[prost(string, tag = "2")]
    pub storage_type: ::prost::alloc::string::String,
    /// Required. The storage pool name.
    #[prost(string, tag = "3")]
    pub pool_name: ::prost::alloc::string::String,
    /// Required. Total capacity of the pool in GiB.
    #[prost(double, tag = "4")]
    pub total_capacity_in_gib: f64,
    /// Required. Used capacity of the pool in GiB.
    #[prost(double, tag = "5")]
    pub used_capacity_in_gib: f64,
    /// Required. Utilization percentage of a storage pool.
    #[prost(double, tag = "6")]
    pub utilization_percentage: f64,
    /// Required. Appliance id.
    #[prost(string, tag = "7")]
    pub appliance_id: ::prost::alloc::string::String,
}
/// Holds information for the Protected Resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedResource {
    /// Required. Resource name.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Resource Type.
    #[prost(string, tag = "2")]
    pub resource_type: ::prost::alloc::string::String,
    /// Required. Resource ID.
    #[prost(string, tag = "3")]
    pub resource_id: ::prost::alloc::string::String,
    /// Optional. Backup Inclusion/Exclusion.
    #[prost(string, tag = "4")]
    pub backup_inclusion_or_exclusion: ::prost::alloc::string::String,
    /// Required. Host Id.
    #[prost(string, tag = "5")]
    pub host_id: ::prost::alloc::string::String,
    /// Required. Host Name.
    #[prost(string, tag = "6")]
    pub host_name: ::prost::alloc::string::String,
    /// Required. Backup Template ID.
    #[prost(string, tag = "7")]
    pub backup_plan_policy_template_id: ::prost::alloc::string::String,
    /// Required. Backup Template.
    #[prost(string, tag = "8")]
    pub backup_plan_policy_template: ::prost::alloc::string::String,
    /// Required. Sla Id.
    #[prost(string, tag = "9")]
    pub sla_id: ::prost::alloc::string::String,
    /// Required. Backup Plan restrictions.
    #[prost(string, tag = "10")]
    pub backup_plan_restrictions: ::prost::alloc::string::String,
    /// Required. Protected On.
    #[prost(string, tag = "11")]
    pub protected_on: ::prost::alloc::string::String,
    /// Optional. Policy Overrides.
    #[prost(string, tag = "12")]
    pub policy_overrides: ::prost::alloc::string::String,
    /// Optional. Source Appliance in case of streamsnap.
    #[prost(string, tag = "13")]
    pub source_appliance: ::prost::alloc::string::String,
    /// Optional. Source Appliance Id in case of streamsnap.
    #[prost(string, tag = "14")]
    pub source_appliance_id: ::prost::alloc::string::String,
    /// Required. Protected Data (GiB).
    #[prost(double, tag = "15")]
    pub protected_data_in_gib: f64,
    /// Optional. Onvault (GiB) .
    #[prost(double, tag = "16")]
    pub onvault_in_gib: f64,
    /// Optional. Originating Appliance in case of streamsnap.
    #[prost(string, tag = "17")]
    pub appliance_name: ::prost::alloc::string::String,
    /// Optional. Originating Appliance id in case of streamsnap.
    #[prost(string, tag = "18")]
    pub appliance_id: ::prost::alloc::string::String,
    /// Optional. Remote Appliance in case of streamsnap.
    #[prost(string, tag = "19")]
    pub remote_appliance: ::prost::alloc::string::String,
    /// Optional. Remote Appliance id in case of streamsnap.
    #[prost(string, tag = "20")]
    pub remote_appliance_id: ::prost::alloc::string::String,
    /// Optional. Recovery Point.
    #[prost(string, tag = "21")]
    pub recovery_point: ::prost::alloc::string::String,
}
/// This is an event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The event_time field displays the time when the event was reported
    #[prost(string, tag = "1")]
    pub event_time: ::prost::alloc::string::String,
    /// The srcid field displays unique id of the event occurred in the backup
    /// appliance
    #[prost(int64, tag = "2")]
    pub srcid: i64,
    /// The errormessage field describes the detailed error associated with the
    /// event
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
    /// The event_id field displays unique id associated with the error
    #[prost(int32, tag = "4")]
    pub event_id: i32,
    /// The component field displays the source of the event
    #[prost(string, tag = "5")]
    pub component: ::prost::alloc::string::String,
    /// The appliance_id field displays unique id of the appliance on which event
    /// occurred
    #[prost(int64, tag = "6")]
    pub appliance_name: i64,
    /// The appname field displays name of the application associated with the
    /// event
    #[prost(string, tag = "7")]
    pub app_name: ::prost::alloc::string::String,
    /// The apptype field displays type of the application associated with the
    /// event
    #[prost(string, tag = "8")]
    pub app_type: ::prost::alloc::string::String,
    /// The jobname field displays name of the job associated with the event
    #[prost(string, tag = "9")]
    pub job_name: ::prost::alloc::string::String,
}
