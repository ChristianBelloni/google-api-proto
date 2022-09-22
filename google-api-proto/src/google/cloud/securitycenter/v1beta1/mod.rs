/// User specified security marks that are attached to the parent Security
/// Command Center resource. Security marks are scoped within a Security Command
/// Center organization -- they can be modified and viewed by all users who have
/// proper permissions on the organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityMarks {
    /// The relative resource name of the SecurityMarks. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Examples:
    /// "organizations/{organization_id}/assets/{asset_id}/securityMarks"
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Mutable user specified security marks belonging to the parent resource.
    /// Constraints are as follows:
    ///
    ///   * Keys and values are treated as case insensitive
    ///   * Keys must be between 1 - 256 characters (inclusive)
    ///   * Keys must be letters, numbers, underscores, or dashes
    ///   * Values have leading and trailing whitespace trimmed, remaining
    ///     characters must be between 1 - 4096 characters (inclusive)
    #[prost(btree_map="string, string", tag="2")]
    pub marks: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Security Command Center representation of a Google Cloud
/// resource.
///
/// The Asset is a Security Command Center resource that captures information
/// about a single Google Cloud resource. All modifications to an Asset are only
/// within the context of Security Command Center and don't affect the referenced
/// Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The relative resource name of this asset. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/assets/{asset_id}".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Security Command Center managed properties. These properties are managed by
    /// Security Command Center and cannot be modified by the user.
    #[prost(message, optional, tag="2")]
    pub security_center_properties: ::core::option::Option<asset::SecurityCenterProperties>,
    /// Resource managed properties. These properties are managed and defined by
    /// the Google Cloud resource and cannot be modified by the user.
    #[prost(btree_map="string, message", tag="7")]
    pub resource_properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// User specified security marks. These marks are entirely managed by the user
    /// and come from the SecurityMarks resource that belongs to the asset.
    #[prost(message, optional, tag="8")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The time at which the asset was created in Security Command Center.
    #[prost(message, optional, tag="9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the asset was last updated, added, or deleted in Security
    /// Command Center.
    #[prost(message, optional, tag="10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// Security Command Center managed properties. These properties are managed by
    /// Security Command Center and cannot be modified by the user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityCenterProperties {
        /// Immutable. The full resource name of the Google Cloud resource this asset
        /// represents. This field is immutable after create time. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
        #[prost(string, tag="1")]
        pub resource_name: ::prost::alloc::string::String,
        /// The type of the Google Cloud resource. Examples include: APPLICATION,
        /// PROJECT, and ORGANIZATION. This is a case insensitive field defined by
        /// Security Command Center and/or the producer of the resource and is
        /// immutable after create time.
        #[prost(string, tag="2")]
        pub resource_type: ::prost::alloc::string::String,
        /// The full resource name of the immediate parent of the resource. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
        #[prost(string, tag="3")]
        pub resource_parent: ::prost::alloc::string::String,
        /// The full resource name of the project the resource belongs to. See:
        /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
        #[prost(string, tag="4")]
        pub resource_project: ::prost::alloc::string::String,
        /// Owners of the Google Cloud resource.
        #[prost(string, repeated, tag="5")]
        pub resource_owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Security Command Center finding.
///
/// A finding is a record of assessment data (security, risk, health or privacy)
/// ingested into Security Command Center for presentation, notification,
/// analysis, policy testing, and enforcement. For example, an XSS vulnerability
/// in an App Engine application is a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finding {
    /// The relative resource name of this finding. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/findings/{finding_id}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The relative resource name of the source the finding belongs to.
    /// See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// This field is immutable after creation time.
    /// For example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// For findings on Google Cloud resources, the full resource
    /// name of the Google Cloud resource this finding is for. See:
    /// <https://cloud.google.com/apis/design/resource_names#full_resource_name>
    /// When the finding is for a non-Google Cloud resource, the resourceName can
    /// be a customer or partner defined string. This field is immutable after
    /// creation time.
    #[prost(string, tag="3")]
    pub resource_name: ::prost::alloc::string::String,
    /// The state of the finding.
    #[prost(enumeration="finding::State", tag="4")]
    pub state: i32,
    /// The additional taxonomy group within findings from a given source.
    /// This field is immutable after creation time.
    /// Example: "XSS_FLASH_INJECTION"
    #[prost(string, tag="5")]
    pub category: ::prost::alloc::string::String,
    /// The URI that, if available, points to a web page outside of Security
    /// Command Center where additional information about the finding can be found.
    /// This field is guaranteed to be either empty or a well formed URL.
    #[prost(string, tag="6")]
    pub external_uri: ::prost::alloc::string::String,
    /// Source specific properties. These properties are managed by the source
    /// that writes the finding. The key names in the source_properties map must be
    /// between 1 and 255 characters, and must start with a letter and contain
    /// alphanumeric characters or underscores only.
    #[prost(btree_map="string, message", tag="7")]
    pub source_properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// Output only. User specified security marks. These marks are entirely
    /// managed by the user and come from the SecurityMarks resource that belongs
    /// to the finding.
    #[prost(message, optional, tag="8")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The time at which the event took place, or when an update to the finding
    /// occurred. For example, if the finding represents an open firewall it would
    /// capture the time the detector believes the firewall became open. The
    /// accuracy is determined by the detector. If the finding were to be resolved
    /// afterward, this time would reflect when the finding was resolved.
    #[prost(message, optional, tag="9")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the finding was created in Security Command Center.
    #[prost(message, optional, tag="10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Finding`.
pub mod finding {
    /// The state of the finding.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The finding requires attention and has not been addressed yet.
        Active = 1,
        /// The finding has been fixed, triaged as a non-issue or otherwise addressed
        /// and is no longer active.
        Inactive = 2,
    }
}
/// User specified settings that are attached to the Security Command
/// Center organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSettings {
    /// The relative resource name of the settings. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/organizationSettings".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A flag that indicates if Asset Discovery should be enabled. If the flag is
    /// set to `true`, then discovery of assets will occur. If it is set to `false,
    /// all historical assets will remain, but discovery of future assets will not
    /// occur.
    #[prost(bool, tag="2")]
    pub enable_asset_discovery: bool,
    /// The configuration used for Asset Discovery runs.
    #[prost(message, optional, tag="3")]
    pub asset_discovery_config: ::core::option::Option<organization_settings::AssetDiscoveryConfig>,
}
/// Nested message and enum types in `OrganizationSettings`.
pub mod organization_settings {
    /// The configuration used for Asset Discovery runs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetDiscoveryConfig {
        /// The project ids to use for filtering asset discovery.
        #[prost(string, repeated, tag="1")]
        pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The mode to use for filtering asset discovery.
        #[prost(enumeration="asset_discovery_config::InclusionMode", tag="2")]
        pub inclusion_mode: i32,
    }
    /// Nested message and enum types in `AssetDiscoveryConfig`.
    pub mod asset_discovery_config {
        /// The mode of inclusion when running Asset Discovery.
        /// Asset discovery can be limited by explicitly identifying projects to be
        /// included or excluded. If INCLUDE_ONLY is set, then only those projects
        /// within the organization and their children are discovered during asset
        /// discovery. If EXCLUDE is set, then projects that don't match those
        /// projects are discovered during asset discovery. If neither are set, then
        /// all projects within the organization are discovered during asset
        /// discovery.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum InclusionMode {
            /// Unspecified. Setting the mode with this value will disable
            /// inclusion/exclusion filtering for Asset Discovery.
            Unspecified = 0,
            /// Asset Discovery will capture only the resources within the projects
            /// specified. All other resources will be ignored.
            IncludeOnly = 1,
            /// Asset Discovery will ignore all resources under the projects specified.
            /// All other resources will be retrieved.
            Exclude = 2,
        }
    }
}
/// Security Command Center finding source. A finding source
/// is an entity or a mechanism that can produce a finding. A source is like a
/// container of findings that come from the same scanner, logger, monitor, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// The relative resource name of this source. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}"
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The source's display name.
    /// A source's display name must be unique amongst its siblings, for example,
    /// two sources with the same parent can't share the same display name.
    /// The display name must have a length between 1 and 64 characters
    /// (inclusive).
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the source (max of 1024 characters).
    /// Example:
    /// "Web Security Scanner is a web security scanner for common
    /// vulnerabilities in App Engine applications. It can automatically
    /// scan and detect four common vulnerabilities, including cross-site-scripting
    /// (XSS), Flash injection, mixed content (HTTP in HTTPS), and
    /// outdated/insecure libraries."
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
/// Request message for creating a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFindingRequest {
    /// Required. Resource name of the new finding's parent. Its format should be
    /// "organizations/\[organization_id]/sources/[source_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique identifier provided by the client within the parent scope.
    /// It must be alphanumeric and less than or equal to 32 characters and
    /// greater than 0 characters in length.
    #[prost(string, tag="2")]
    pub finding_id: ::prost::alloc::string::String,
    /// Required. The Finding being created. The name and security_marks will be ignored as
    /// they are both output only fields on this resource.
    #[prost(message, optional, tag="3")]
    pub finding: ::core::option::Option<Finding>,
}
/// Request message for creating a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSourceRequest {
    /// Required. Resource name of the new source's parent. Its format should be
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Source being created, only the display_name and description will be
    /// used. All other fields will be ignored.
    #[prost(message, optional, tag="2")]
    pub source: ::core::option::Option<Source>,
}
/// Request message for getting organization settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationSettingsRequest {
    /// Required. Name of the organization to get organization settings for. Its format is
    /// "organizations/\[organization_id\]/organizationSettings".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for getting a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourceRequest {
    /// Required. Relative resource name of the source. Its format is
    /// "organizations/\[organization_id]/source/[source_id\]".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for grouping by assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAssetsRequest {
    /// Required. Name of the organization to groupBy. Its format is
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across assets.
    /// The expression is a list of zero or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are not supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. The fields map to those
    /// defined in the Asset resource. Examples include:
    ///
    /// * name
    /// * security_center_properties.resource_name
    /// * resource_properties.a_property
    /// * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// For example, `resource_properties.size = 100` is a valid filter string.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Expression that defines what assets fields to use for grouping. The string
    /// value should follow SQL syntax: comma separated list of fields. For
    /// example:
    /// "security_center_properties.resource_project,security_center_properties.project".
    ///
    /// The following fields are supported when compare_duration is not set:
    ///
    /// * security_center_properties.resource_project
    /// * security_center_properties.resource_type
    /// * security_center_properties.resource_parent
    ///
    /// The following fields are supported when compare_duration is set:
    ///
    /// * security_center_properties.resource_type
    #[prost(string, tag="3")]
    pub group_by: ::prost::alloc::string::String,
    /// When compare_duration is set, the Asset's "state" property is updated to
    /// indicate whether the asset was added, removed, or remained present during
    /// the compare_duration period of time that precedes the read_time. This is
    /// the time between (read_time - compare_duration) and read_time.
    ///
    /// The state value is derived based on the presence of the asset at the two
    /// points in time. Intermediate state changes between the two times don't
    /// affect the result. For example, the results aren't affected if the asset is
    /// removed and re-created again.
    ///
    /// Possible "state" values when compare_duration is specified:
    ///
    /// * "ADDED": indicates that the asset was not present before
    ///              compare_duration, but present at reference_time.
    /// * "REMOVED": indicates that the asset was present at the start of
    ///              compare_duration, but not present at reference_time.
    /// * "ACTIVE": indicates that the asset was present at both the
    ///              start and the end of the time period defined by
    ///              compare_duration and reference_time.
    ///
    /// This field is ignored if `state` is not a field in `group_by`.
    #[prost(message, optional, tag="4")]
    pub compare_duration: ::core::option::Option<::prost_types::Duration>,
    /// Time used as a reference point when filtering assets. The filter is limited
    /// to assets existing at the supplied time and their values are those at that
    /// specific time. Absence of this field will default to the API's version of
    /// NOW.
    #[prost(message, optional, tag="5")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The value returned by the last `GroupAssetsResponse`; indicates
    /// that this is a continuation of a prior `GroupAssets` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag="7")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="8")]
    pub page_size: i32,
}
/// Response message for grouping by assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupAssetsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag="1")]
    pub group_by_results: ::prost::alloc::vec::Vec<GroupResult>,
    /// Time used for executing the groupBy request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for grouping by findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsRequest {
    /// Required. Name of the source to groupBy. Its format is
    /// "organizations/\[organization_id]/sources/[source_id\]". To groupBy across
    /// all sources provide a source_id of `-`. For example:
    /// organizations/{organization_id}/sources/-
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are not supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///  * name
    ///  * source_properties.a_property
    ///  * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// For example, `source_properties.size = 100` is a valid filter string.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Required. Expression that defines what assets fields to use for grouping (including
    /// `state`). The string value should follow SQL syntax: comma separated list
    /// of fields. For example:
    /// "parent,resource_name".
    ///
    /// The following fields are supported:
    ///
    /// * resource_name
    /// * category
    /// * state
    /// * parent
    #[prost(string, tag="3")]
    pub group_by: ::prost::alloc::string::String,
    /// Time used as a reference point when filtering findings. The filter is
    /// limited to findings existing at the supplied time and their values are
    /// those at that specific time. Absence of this field will default to the
    /// API's version of NOW.
    #[prost(message, optional, tag="4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The value returned by the last `GroupFindingsResponse`; indicates
    /// that this is a continuation of a prior `GroupFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="5")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="6")]
    pub page_size: i32,
}
/// Response message for group by findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupFindingsResponse {
    /// Group results. There exists an element for each existing unique
    /// combination of property/values. The element contains a count for the number
    /// of times those specific property/values appear.
    #[prost(message, repeated, tag="1")]
    pub group_by_results: ::prost::alloc::vec::Vec<GroupResult>,
    /// Time used for executing the groupBy request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Result containing the properties and count of a groupBy request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupResult {
    /// Properties matching the groupBy fields in the request.
    #[prost(btree_map="string, message", tag="1")]
    pub properties: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// Total count of resources for the given properties.
    #[prost(int64, tag="2")]
    pub count: i64,
}
/// Request message for listing sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesRequest {
    /// Required. Resource name of the parent of sources to list. Its format should be
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The value returned by the last `ListSourcesResponse`; indicates
    /// that this is a continuation of a prior `ListSources` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="7")]
    pub page_size: i32,
}
/// Response message for listing sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesResponse {
    /// Sources belonging to the requested parent.
    #[prost(message, repeated, tag="1")]
    pub sources: ::prost::alloc::vec::Vec<Source>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for listing assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. Name of the organization assets should belong to. Its format is
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across assets.
    /// The expression is a list of zero or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are not supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. The fields map to those
    /// defined in the Asset resource. Examples include:
    ///
    /// * name
    /// * security_center_properties.resource_name
    /// * resource_properties.a_property
    /// * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// For example, `resource_properties.size = 100` is a valid filter string.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,resource_properties.a_property". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,resource_properties.a_property". Redundant space characters in the
    /// syntax are insignificant. "name desc,resource_properties.a_property" and "
    /// name     desc  ,   resource_properties.a_property  " are equivalent.
    #[prost(string, tag="3")]
    pub order_by: ::prost::alloc::string::String,
    /// Time used as a reference point when filtering assets. The filter is limited
    /// to assets existing at the supplied time and their values are those at that
    /// specific time. Absence of this field will default to the API's version of
    /// NOW.
    #[prost(message, optional, tag="4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// When compare_duration is set, the ListAssetResult's "state" attribute is
    /// updated to indicate whether the asset was added, removed, or remained
    /// present during the compare_duration period of time that precedes the
    /// read_time. This is the time between (read_time -
    /// compare_duration) and read_time.
    ///
    /// The state value is derived based on the presence of the asset at the two
    /// points in time. Intermediate state changes between the two times don't
    /// affect the result. For example, the results aren't affected if the asset is
    /// removed and re-created again.
    ///
    /// Possible "state" values when compare_duration is specified:
    ///
    /// * "ADDED": indicates that the asset was not present before
    ///              compare_duration, but present at read_time.
    /// * "REMOVED": indicates that the asset was present at the start of
    ///              compare_duration, but not present at read_time.
    /// * "ACTIVE": indicates that the asset was present at both the
    ///              start and the end of the time period defined by
    ///              compare_duration and read_time.
    ///
    /// If compare_duration is not specified, then the only possible state is
    /// "UNUSED", which indicates that the asset is present at read_time.
    #[prost(message, optional, tag="5")]
    pub compare_duration: ::core::option::Option<::prost_types::Duration>,
    /// Optional. A field mask to specify the ListAssetsResult fields to be listed in the
    /// response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag="7")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListAssetsResponse`; indicates
    /// that this is a continuation of a prior `ListAssets` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="8")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="9")]
    pub page_size: i32,
}
/// Response message for listing assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// Assets matching the list request.
    #[prost(message, repeated, tag="1")]
    pub list_assets_results: ::prost::alloc::vec::Vec<list_assets_response::ListAssetsResult>,
    /// Time used for executing the list request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of assets matching the query.
    #[prost(int32, tag="4")]
    pub total_size: i32,
}
/// Nested message and enum types in `ListAssetsResponse`.
pub mod list_assets_response {
    /// Result containing the Asset and its State.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListAssetsResult {
        /// Asset matching the search request.
        #[prost(message, optional, tag="1")]
        pub asset: ::core::option::Option<super::Asset>,
        /// State of the asset.
        #[prost(enumeration="list_assets_result::State", tag="2")]
        pub state: i32,
    }
    /// Nested message and enum types in `ListAssetsResult`.
    pub mod list_assets_result {
        /// State of the asset.
        ///
        /// When querying across two points in time this describes
        /// the change between the two points: ADDED, REMOVED, or ACTIVE.
        /// If there was no compare_duration supplied in the request the state should
        /// be: UNUSED
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum State {
            /// Unspecified state.
            Unspecified = 0,
            /// Request did not specify use of this field in the result.
            Unused = 1,
            /// Asset was added between the points in time.
            Added = 2,
            /// Asset was removed between the points in time.
            Removed = 3,
            /// Asset was active at both point(s) in time.
            Active = 4,
        }
    }
}
/// Request message for listing findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsRequest {
    /// Required. Name of the source the findings belong to. Its format is
    /// "organizations/\[organization_id]/sources/[source_id\]". To list across all
    /// sources provide a source_id of `-`. For example:
    /// organizations/{organization_id}/sources/-
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Expression that defines the filter to apply across findings.
    /// The expression is a list of one or more restrictions combined via logical
    /// operators `AND` and `OR`.
    /// Parentheses are not supported, and `OR` has higher precedence than `AND`.
    ///
    /// Restrictions have the form `<field> <operator> <value>` and may have a `-`
    /// character in front of them to indicate negation. Examples include:
    ///
    ///  * name
    ///  * source_properties.a_property
    ///  * security_marks.marks.marka
    ///
    /// The supported operators are:
    ///
    /// * `=` for all value types.
    /// * `>`, `<`, `>=`, `<=` for integer values.
    /// * `:`, meaning substring matching, for strings.
    ///
    /// The supported value types are:
    ///
    /// * string literals in quotes.
    /// * integer literals without quotes.
    /// * boolean literals `true` and `false` without quotes.
    ///
    /// For example, `source_properties.size = 100` is a valid filter string.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// Expression that defines what fields and order to use for sorting. The
    /// string value should follow SQL syntax: comma separated list of fields. For
    /// example: "name,resource_properties.a_property". The default sorting order
    /// is ascending. To specify descending order for a field, a suffix " desc"
    /// should be appended to the field name. For example: "name
    /// desc,source_properties.a_property". Redundant space characters in the
    /// syntax are insignificant. "name desc,source_properties.a_property" and "
    /// name     desc  ,   source_properties.a_property  " are equivalent.
    #[prost(string, tag="3")]
    pub order_by: ::prost::alloc::string::String,
    /// Time used as a reference point when filtering findings. The filter is
    /// limited to findings existing at the supplied time and their values are
    /// those at that specific time. Absence of this field will default to the
    /// API's version of NOW.
    #[prost(message, optional, tag="4")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A field mask to specify the Finding fields to be listed in the response.
    /// An empty field mask will list all fields.
    #[prost(message, optional, tag="5")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The value returned by the last `ListFindingsResponse`; indicates
    /// that this is a continuation of a prior `ListFindings` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag="6")]
    pub page_token: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response. Default is
    /// 10, minimum is 1, maximum is 1000.
    #[prost(int32, tag="7")]
    pub page_size: i32,
}
/// Response message for listing findings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFindingsResponse {
    /// Findings matching the list request.
    #[prost(message, repeated, tag="1")]
    pub findings: ::prost::alloc::vec::Vec<Finding>,
    /// Time used for executing the list request.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of findings matching the query.
    #[prost(int32, tag="4")]
    pub total_size: i32,
}
/// Request message for updating a finding's state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFindingStateRequest {
    /// Required. The relative resource name of the finding. See:
    /// <https://cloud.google.com/apis/design/resource_names#relative_resource_name>
    /// Example:
    /// "organizations/{organization_id}/sources/{source_id}/finding/{finding_id}".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired State of the finding.
    #[prost(enumeration="finding::State", tag="2")]
    pub state: i32,
    /// Required. The time at which the updated state takes effect.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for running asset discovery for an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAssetDiscoveryRequest {
    /// Required. Name of the organization to run asset discovery for. Its format is
    /// "organizations/\[organization_id\]".
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
}
/// Request message for updating or creating a finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFindingRequest {
    /// Required. The finding resource to update or create if it does not already exist.
    /// parent, security_marks, and update_time will be ignored.
    ///
    /// In the case of creation, the finding id portion of the name must
    /// alphanumeric and less than or equal to 32 characters and greater than 0
    /// characters in length.
    #[prost(message, optional, tag="1")]
    pub finding: ::core::option::Option<Finding>,
    /// The FieldMask to use when updating the finding resource. This field should
    /// not be specified when creating a finding.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating an organization's settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationSettingsRequest {
    /// Required. The organization settings resource to update.
    #[prost(message, optional, tag="1")]
    pub organization_settings: ::core::option::Option<OrganizationSettings>,
    /// The FieldMask to use when updating the settings resource.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSourceRequest {
    /// Required. The source resource to update.
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<Source>,
    /// The FieldMask to use when updating the source resource.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for updating a SecurityMarks resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecurityMarksRequest {
    /// Required. The security marks resource to update.
    #[prost(message, optional, tag="1")]
    pub security_marks: ::core::option::Option<SecurityMarks>,
    /// The FieldMask to use when updating the security marks resource.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The time at which the updated SecurityMarks take effect.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod security_center_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// V1 Beta APIs for Security Center service.
    #[derive(Debug, Clone)]
    pub struct SecurityCenterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecurityCenterClient<T>
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
        ) -> SecurityCenterClient<InterceptedService<T, F>>
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
            SecurityCenterClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a source.
        pub async fn create_source(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/CreateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a finding. The corresponding source must exist for finding creation
        /// to succeed.
        pub async fn create_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/CreateFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy on the specified Source.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the settings for an organization.
        pub async fn get_organization_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationSettingsRequest>,
        ) -> Result<tonic::Response<super::OrganizationSettings>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/GetOrganizationSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a source.
        pub async fn get_source(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/GetSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Filters an organization's assets and  groups them by their specified
        /// properties.
        pub async fn group_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupAssetsRequest>,
        ) -> Result<tonic::Response<super::GroupAssetsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/GroupAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Filters an organization or source's findings and  groups them by their
        /// specified properties.
        ///
        /// To group across all sources provide a `-` as the source id.
        /// Example: /v1beta1/organizations/{organization_id}/sources/-/findings
        pub async fn group_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::GroupFindingsRequest>,
        ) -> Result<tonic::Response<super::GroupFindingsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/GroupFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists an organization's assets.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists an organization or source's findings.
        ///
        /// To list across all sources provide a `-` as the source id.
        /// Example: /v1beta1/organizations/{organization_id}/sources/-/findings
        pub async fn list_findings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFindingsRequest>,
        ) -> Result<tonic::Response<super::ListFindingsResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/ListFindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all sources belonging to an organization.
        pub async fn list_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSourcesRequest>,
        ) -> Result<tonic::Response<super::ListSourcesResponse>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/ListSources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Runs asset discovery. The discovery is tracked with a long-running
        /// operation.
        ///
        /// This API can only be called with limited frequency for an organization. If
        /// it is called too frequently the caller will receive a TOO_MANY_REQUESTS
        /// error.
        pub async fn run_asset_discovery(
            &mut self,
            request: impl tonic::IntoRequest<super::RunAssetDiscoveryRequest>,
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/RunAssetDiscovery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the state of a finding.
        pub async fn set_finding_state(
            &mut self,
            request: impl tonic::IntoRequest<super::SetFindingStateRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/SetFindingState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on the specified Source.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the permissions that a caller has on the specified source.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates or updates a finding. The corresponding source must exist for a
        /// finding creation to succeed.
        pub async fn update_finding(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFindingRequest>,
        ) -> Result<tonic::Response<super::Finding>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/UpdateFinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an organization's settings.
        pub async fn update_organization_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrganizationSettingsRequest>,
        ) -> Result<tonic::Response<super::OrganizationSettings>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/UpdateOrganizationSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a source.
        pub async fn update_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/UpdateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates security marks.
        pub async fn update_security_marks(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecurityMarksRequest>,
        ) -> Result<tonic::Response<super::SecurityMarks>, tonic::Status> {
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
                "/google.cloud.securitycenter.v1beta1.SecurityCenter/UpdateSecurityMarks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Response of asset discovery run
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAssetDiscoveryResponse {
    /// The state of an asset discovery run.
    #[prost(enumeration="run_asset_discovery_response::State", tag="1")]
    pub state: i32,
    /// The duration between asset discovery run start and end
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `RunAssetDiscoveryResponse`.
pub mod run_asset_discovery_response {
    /// The state of an asset discovery run.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Asset discovery run state was unspecified.
        Unspecified = 0,
        /// Asset discovery run completed successfully.
        Completed = 1,
        /// Asset discovery run was cancelled with tasks still pending, as another
        /// run for the same organization was started with a higher priority.
        Superseded = 2,
        /// Asset discovery run was killed and terminated.
        Terminated = 3,
    }
}
