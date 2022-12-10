/// **Multi-cluster Ingress**: The configuration for the MultiClusterIngress
/// feature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureSpec {
    /// Fully-qualified Membership name which hosts the MultiClusterIngress CRD.
    /// Example: `projects/foo-proj/locations/global/memberships/bar`
    #[prost(string, tag = "1")]
    pub config_membership: ::prost::alloc::string::String,
}
