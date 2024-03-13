/// A
/// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
/// represents an individual Certificate Authority. A
/// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
/// can be used to create
/// [Certificates][google.cloud.security.privateca.v1.Certificate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateAuthority {
    /// Output only. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The
    /// [Type][google.cloud.security.privateca.v1.CertificateAuthority.Type] of
    /// this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    #[prost(enumeration = "certificate_authority::Type", tag = "2")]
    pub r#type: i32,
    /// Required. Immutable. The config used to create a self-signed X.509
    /// certificate or CSR.
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<CertificateConfig>,
    /// Required. Immutable. The desired lifetime of the CA certificate. Used to
    /// create the "not_before_time" and "not_after_time" fields inside an X.509
    /// certificate.
    #[prost(message, optional, tag = "4")]
    pub lifetime: ::core::option::Option<::prost_types::Duration>,
    /// Required. Immutable. Used when issuing certificates for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    /// If this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// is a self-signed CertificateAuthority, this key is also used to sign the
    /// self-signed CA certificate. Otherwise, it is used to sign a CSR.
    #[prost(message, optional, tag = "5")]
    pub key_spec: ::core::option::Option<certificate_authority::KeyVersionSpec>,
    /// Optional. If this is a subordinate
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority],
    /// this field will be set with the subordinate configuration, which describes
    /// its issuers. This may be updated, but this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// must continue to validate.
    #[prost(message, optional, tag = "6")]
    pub subordinate_config: ::core::option::Option<SubordinateConfig>,
    /// Output only. The
    /// [CaPool.Tier][google.cloud.security.privateca.v1.CaPool.Tier] of the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] that includes this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    #[prost(enumeration = "ca_pool::Tier", tag = "7")]
    pub tier: i32,
    /// Output only. The
    /// [State][google.cloud.security.privateca.v1.CertificateAuthority.State] for
    /// this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
    #[prost(enumeration = "certificate_authority::State", tag = "8")]
    pub state: i32,
    /// Output only. This
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
    /// certificate chain, including the current
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
    /// certificate. Ordered such that the root issuer is the final element
    /// (consistent with RFC 5246). For a self-signed CA, this will only list the
    /// current
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
    /// certificate.
    #[prost(string, repeated, tag = "9")]
    pub pem_ca_certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. A structured description of this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
    /// CA certificate and its issuers. Ordered as self-to-root.
    #[prost(message, repeated, tag = "10")]
    pub ca_certificate_descriptions: ::prost::alloc::vec::Vec<CertificateDescription>,
    /// Immutable. The name of a Cloud Storage bucket where this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// will publish content, such as the CA certificate and CRLs. This must be a
    /// bucket name, without any prefixes (such as `gs://`) or suffixes (such as
    /// `.googleapis.com`). For example, to use a bucket named `my-bucket`, you
    /// would simply specify `my-bucket`. If not specified, a managed bucket will
    /// be created.
    #[prost(string, tag = "11")]
    pub gcs_bucket: ::prost::alloc::string::String,
    /// Output only. URLs for accessing content published by this CA, such as the
    /// CA certificate and CRLs.
    #[prost(message, optional, tag = "12")]
    pub access_urls: ::core::option::Option<certificate_authority::AccessUrls>,
    /// Output only. The time at which this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// was last updated.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// was soft deleted, if it is in the
    /// [DELETED][google.cloud.security.privateca.v1.CertificateAuthority.State.DELETED]
    /// state.
    #[prost(message, optional, tag = "15")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// will be permanently purged, if it is in the
    /// [DELETED][google.cloud.security.privateca.v1.CertificateAuthority.State.DELETED]
    /// state.
    #[prost(message, optional, tag = "16")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(btree_map = "string, string", tag = "17")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `CertificateAuthority`.
pub mod certificate_authority {
    /// URLs where a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// will publish content.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessUrls {
        /// The URL where this
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
        /// CA certificate is published. This will only be set for CAs that have been
        /// activated.
        #[prost(string, tag = "1")]
        pub ca_certificate_access_url: ::prost::alloc::string::String,
        /// The URLs where this
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
        /// CRLs are published. This will only be set for CAs that have been
        /// activated.
        #[prost(string, repeated, tag = "2")]
        pub crl_access_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A Cloud KMS key configuration that a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// will use.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyVersionSpec {
        #[prost(oneof = "key_version_spec::KeyVersion", tags = "1, 2")]
        pub key_version: ::core::option::Option<key_version_spec::KeyVersion>,
    }
    /// Nested message and enum types in `KeyVersionSpec`.
    pub mod key_version_spec {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum KeyVersion {
            /// The resource name for an existing Cloud KMS CryptoKeyVersion in the
            /// format
            /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
            /// This option enables full flexibility in the key's capabilities and
            /// properties.
            #[prost(string, tag = "1")]
            CloudKmsKeyVersion(::prost::alloc::string::String),
            /// The algorithm to use for creating a managed Cloud KMS key for a for a
            /// simplified experience. All managed keys will be have their
            /// [ProtectionLevel][google.cloud.kms.v1.ProtectionLevel] as `HSM`.
            #[prost(enumeration = "super::SignHashAlgorithm", tag = "2")]
            Algorithm(i32),
        }
    }
    /// The type of a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority],
    /// indicating its issuing chain.
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
        /// Not specified.
        Unspecified = 0,
        /// Self-signed CA.
        SelfSigned = 1,
        /// Subordinate CA. Could be issued by a Private CA
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// or an unmanaged CA.
        Subordinate = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::SelfSigned => "SELF_SIGNED",
                Type::Subordinate => "SUBORDINATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SELF_SIGNED" => Some(Self::SelfSigned),
                "SUBORDINATE" => Some(Self::Subordinate),
                _ => None,
            }
        }
    }
    /// The state of a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority],
    /// indicating if it can be used.
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
        /// Not specified.
        Unspecified = 0,
        /// Certificates can be issued from this CA. CRLs will be generated for this
        /// CA. The CA will be part of the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s trust anchor, and
        /// will be used to issue certificates from the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool].
        Enabled = 1,
        /// Certificates cannot be issued from this CA. CRLs will still be generated.
        /// The CA will be part of the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s trust anchor, but
        /// will not be used to issue certificates from the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool].
        Disabled = 2,
        /// Certificates can be issued from this CA. CRLs will be generated for this
        /// CA. The CA will be part of the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s trust anchor, but
        /// will not be used to issue certificates from the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool].
        Staged = 3,
        /// Certificates cannot be issued from this CA. CRLs will not be generated.
        /// The CA will not be part of the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s trust anchor, and
        /// will not be used to issue certificates from the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool].
        AwaitingUserActivation = 4,
        /// Certificates cannot be issued from this CA. CRLs will not be generated.
        /// The CA may still be recovered by calling
        /// [CertificateAuthorityService.UndeleteCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.UndeleteCertificateAuthority]
        /// before
        /// [expire_time][google.cloud.security.privateca.v1.CertificateAuthority.expire_time].
        /// The CA will not be part of the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s trust anchor, and
        /// will not be used to issue certificates from the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool].
        Deleted = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabled => "ENABLED",
                State::Disabled => "DISABLED",
                State::Staged => "STAGED",
                State::AwaitingUserActivation => "AWAITING_USER_ACTIVATION",
                State::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "STAGED" => Some(Self::Staged),
                "AWAITING_USER_ACTIVATION" => Some(Self::AwaitingUserActivation),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
    /// The algorithm of a Cloud KMS CryptoKeyVersion of a
    /// [CryptoKey][google.cloud.kms.v1.CryptoKey] with the
    /// [CryptoKeyPurpose][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose] value
    /// `ASYMMETRIC_SIGN`. These values correspond to the
    /// [CryptoKeyVersionAlgorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm]
    /// values. For RSA signing algorithms, the PSS algorithms should be preferred,
    /// use PKCS1 algorithms if required for compatibility. For further
    /// recommendations, see
    /// <https://cloud.google.com/kms/docs/algorithms#algorithm_recommendations.>
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
    pub enum SignHashAlgorithm {
        /// Not specified.
        Unspecified = 0,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PSS_2048_SHA256
        RsaPss2048Sha256 = 1,
        /// maps to CryptoKeyVersionAlgorithm. RSA_SIGN_PSS_3072_SHA256
        RsaPss3072Sha256 = 2,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PSS_4096_SHA256
        RsaPss4096Sha256 = 3,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_2048_SHA256
        RsaPkcs12048Sha256 = 6,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_3072_SHA256
        RsaPkcs13072Sha256 = 7,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_4096_SHA256
        RsaPkcs14096Sha256 = 8,
        /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P256_SHA256
        EcP256Sha256 = 4,
        /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P384_SHA384
        EcP384Sha384 = 5,
    }
    impl SignHashAlgorithm {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SignHashAlgorithm::Unspecified => "SIGN_HASH_ALGORITHM_UNSPECIFIED",
                SignHashAlgorithm::RsaPss2048Sha256 => "RSA_PSS_2048_SHA256",
                SignHashAlgorithm::RsaPss3072Sha256 => "RSA_PSS_3072_SHA256",
                SignHashAlgorithm::RsaPss4096Sha256 => "RSA_PSS_4096_SHA256",
                SignHashAlgorithm::RsaPkcs12048Sha256 => "RSA_PKCS1_2048_SHA256",
                SignHashAlgorithm::RsaPkcs13072Sha256 => "RSA_PKCS1_3072_SHA256",
                SignHashAlgorithm::RsaPkcs14096Sha256 => "RSA_PKCS1_4096_SHA256",
                SignHashAlgorithm::EcP256Sha256 => "EC_P256_SHA256",
                SignHashAlgorithm::EcP384Sha384 => "EC_P384_SHA384",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SIGN_HASH_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
                "RSA_PSS_2048_SHA256" => Some(Self::RsaPss2048Sha256),
                "RSA_PSS_3072_SHA256" => Some(Self::RsaPss3072Sha256),
                "RSA_PSS_4096_SHA256" => Some(Self::RsaPss4096Sha256),
                "RSA_PKCS1_2048_SHA256" => Some(Self::RsaPkcs12048Sha256),
                "RSA_PKCS1_3072_SHA256" => Some(Self::RsaPkcs13072Sha256),
                "RSA_PKCS1_4096_SHA256" => Some(Self::RsaPkcs14096Sha256),
                "EC_P256_SHA256" => Some(Self::EcP256Sha256),
                "EC_P384_SHA384" => Some(Self::EcP384Sha384),
                _ => None,
            }
        }
    }
}
/// A [CaPool][google.cloud.security.privateca.v1.CaPool] represents a group of
/// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority]
/// that form a trust anchor. A
/// [CaPool][google.cloud.security.privateca.v1.CaPool] can be used to manage
/// issuance policies for one or more
/// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
/// resources and to rotate CA certificates in and out of the trust anchor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaPool {
    /// Output only. The resource name for this
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] in the format
    /// `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The
    /// [Tier][google.cloud.security.privateca.v1.CaPool.Tier] of this
    /// [CaPool][google.cloud.security.privateca.v1.CaPool].
    #[prost(enumeration = "ca_pool::Tier", tag = "2")]
    pub tier: i32,
    /// Optional. The
    /// [IssuancePolicy][google.cloud.security.privateca.v1.CaPool.IssuancePolicy]
    /// to control how
    /// [Certificates][google.cloud.security.privateca.v1.Certificate] will be
    /// issued from this [CaPool][google.cloud.security.privateca.v1.CaPool].
    #[prost(message, optional, tag = "3")]
    pub issuance_policy: ::core::option::Option<ca_pool::IssuancePolicy>,
    /// Optional. The
    /// [PublishingOptions][google.cloud.security.privateca.v1.CaPool.PublishingOptions]
    /// to follow when issuing
    /// [Certificates][google.cloud.security.privateca.v1.Certificate] from any
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in this [CaPool][google.cloud.security.privateca.v1.CaPool].
    #[prost(message, optional, tag = "4")]
    pub publishing_options: ::core::option::Option<ca_pool::PublishingOptions>,
    /// Optional. Labels with user-defined metadata.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `CaPool`.
pub mod ca_pool {
    /// Options relating to the publication of each
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
    /// CA certificate and CRLs and their inclusion as extensions in issued
    /// [Certificates][google.cloud.security.privateca.v1.Certificate]. The options
    /// set here apply to certificates issued by any
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the [CaPool][google.cloud.security.privateca.v1.CaPool].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PublishingOptions {
        /// Optional. When true, publishes each
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
        /// CA certificate and includes its URL in the "Authority Information Access"
        /// X.509 extension in all issued
        /// [Certificates][google.cloud.security.privateca.v1.Certificate]. If this
        /// is false, the CA certificate will not be published and the corresponding
        /// X.509 extension will not be written in issued certificates.
        #[prost(bool, tag = "1")]
        pub publish_ca_cert: bool,
        /// Optional. When true, publishes each
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]'s
        /// CRL and includes its URL in the "CRL Distribution Points" X.509 extension
        /// in all issued
        /// [Certificates][google.cloud.security.privateca.v1.Certificate]. If this
        /// is false, CRLs will not be published and the corresponding X.509
        /// extension will not be written in issued certificates. CRLs will expire 7
        /// days from their creation. However, we will rebuild daily. CRLs are also
        /// rebuilt shortly after a certificate is revoked.
        #[prost(bool, tag = "2")]
        pub publish_crl: bool,
    }
    /// Defines controls over all certificate issuance within a
    /// [CaPool][google.cloud.security.privateca.v1.CaPool].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IssuancePolicy {
        /// Optional. If any
        /// [AllowedKeyType][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.AllowedKeyType]
        /// is specified, then the certificate request's public key must match one of
        /// the key types listed here. Otherwise, any key may be used.
        #[prost(message, repeated, tag = "1")]
        pub allowed_key_types: ::prost::alloc::vec::Vec<issuance_policy::AllowedKeyType>,
        /// Optional. The maximum lifetime allowed for issued
        /// [Certificates][google.cloud.security.privateca.v1.Certificate]. Note that
        /// if the issuing
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// expires before a
        /// [Certificate][google.cloud.security.privateca.v1.Certificate]'s requested
        /// maximum_lifetime, the effective lifetime will be explicitly truncated to
        /// match it.
        #[prost(message, optional, tag = "2")]
        pub maximum_lifetime: ::core::option::Option<::prost_types::Duration>,
        /// Optional. If specified, then only methods allowed in the
        /// [IssuanceModes][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.IssuanceModes]
        /// may be used to issue
        /// [Certificates][google.cloud.security.privateca.v1.Certificate].
        #[prost(message, optional, tag = "3")]
        pub allowed_issuance_modes: ::core::option::Option<
            issuance_policy::IssuanceModes,
        >,
        /// Optional. A set of X.509 values that will be applied to all certificates
        /// issued through this [CaPool][google.cloud.security.privateca.v1.CaPool].
        /// If a certificate request includes conflicting values for the same
        /// properties, they will be overwritten by the values defined here. If a
        /// certificate request uses a
        /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
        /// that defines conflicting
        /// [predefined_values][google.cloud.security.privateca.v1.CertificateTemplate.predefined_values]
        /// for the same properties, the certificate issuance request will fail.
        #[prost(message, optional, tag = "4")]
        pub baseline_values: ::core::option::Option<super::X509Parameters>,
        /// Optional. Describes constraints on identities that may appear in
        /// [Certificates][google.cloud.security.privateca.v1.Certificate] issued
        /// through this [CaPool][google.cloud.security.privateca.v1.CaPool]. If this
        /// is omitted, then this [CaPool][google.cloud.security.privateca.v1.CaPool]
        /// will not add restrictions on a certificate's identity.
        #[prost(message, optional, tag = "5")]
        pub identity_constraints: ::core::option::Option<
            super::CertificateIdentityConstraints,
        >,
        /// Optional. Describes the set of X.509 extensions that may appear in a
        /// [Certificate][google.cloud.security.privateca.v1.Certificate] issued
        /// through this [CaPool][google.cloud.security.privateca.v1.CaPool]. If a
        /// certificate request sets extensions that don't appear in the
        /// [passthrough_extensions][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.passthrough_extensions],
        /// those extensions will be dropped. If a certificate request uses a
        /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
        /// with
        /// [predefined_values][google.cloud.security.privateca.v1.CertificateTemplate.predefined_values]
        /// that don't appear here, the certificate issuance request will fail. If
        /// this is omitted, then this
        /// [CaPool][google.cloud.security.privateca.v1.CaPool] will not add
        /// restrictions on a certificate's X.509 extensions. These constraints do
        /// not apply to X.509 extensions set in this
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s
        /// [baseline_values][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.baseline_values].
        #[prost(message, optional, tag = "6")]
        pub passthrough_extensions: ::core::option::Option<
            super::CertificateExtensionConstraints,
        >,
    }
    /// Nested message and enum types in `IssuancePolicy`.
    pub mod issuance_policy {
        /// Describes a "type" of key that may be used in a
        /// [Certificate][google.cloud.security.privateca.v1.Certificate] issued from
        /// a [CaPool][google.cloud.security.privateca.v1.CaPool]. Note that a single
        /// [AllowedKeyType][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.AllowedKeyType]
        /// may refer to either a fully-qualified key algorithm, such as RSA 4096, or
        /// a family of key algorithms, such as any RSA key.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AllowedKeyType {
            #[prost(oneof = "allowed_key_type::KeyType", tags = "1, 2")]
            pub key_type: ::core::option::Option<allowed_key_type::KeyType>,
        }
        /// Nested message and enum types in `AllowedKeyType`.
        pub mod allowed_key_type {
            /// Describes an RSA key that may be used in a
            /// [Certificate][google.cloud.security.privateca.v1.Certificate] issued
            /// from a [CaPool][google.cloud.security.privateca.v1.CaPool].
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct RsaKeyType {
                /// Optional. The minimum allowed RSA modulus size (inclusive), in bits.
                /// If this is not set, or if set to zero, the service-level min RSA
                /// modulus size will continue to apply.
                #[prost(int64, tag = "1")]
                pub min_modulus_size: i64,
                /// Optional. The maximum allowed RSA modulus size (inclusive), in bits.
                /// If this is not set, or if set to zero, the service will not enforce
                /// an explicit upper bound on RSA modulus sizes.
                #[prost(int64, tag = "2")]
                pub max_modulus_size: i64,
            }
            /// Describes an Elliptic Curve key that may be used in a
            /// [Certificate][google.cloud.security.privateca.v1.Certificate] issued
            /// from a [CaPool][google.cloud.security.privateca.v1.CaPool].
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct EcKeyType {
                /// Optional. A signature algorithm that must be used. If this is
                /// omitted, any EC-based signature algorithm will be allowed.
                #[prost(enumeration = "ec_key_type::EcSignatureAlgorithm", tag = "1")]
                pub signature_algorithm: i32,
            }
            /// Nested message and enum types in `EcKeyType`.
            pub mod ec_key_type {
                /// Describes an elliptic curve-based signature algorithm that may be
                /// used in a
                /// [Certificate][google.cloud.security.privateca.v1.Certificate] issued
                /// from a [CaPool][google.cloud.security.privateca.v1.CaPool].
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
                pub enum EcSignatureAlgorithm {
                    /// Not specified. Signifies that any signature algorithm may be used.
                    Unspecified = 0,
                    /// Refers to the Elliptic Curve Digital Signature Algorithm over the
                    /// NIST P-256 curve.
                    EcdsaP256 = 1,
                    /// Refers to the Elliptic Curve Digital Signature Algorithm over the
                    /// NIST P-384 curve.
                    EcdsaP384 = 2,
                    /// Refers to the Edwards-curve Digital Signature Algorithm over curve
                    /// 25519, as described in RFC 8410.
                    Eddsa25519 = 3,
                }
                impl EcSignatureAlgorithm {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            EcSignatureAlgorithm::Unspecified => {
                                "EC_SIGNATURE_ALGORITHM_UNSPECIFIED"
                            }
                            EcSignatureAlgorithm::EcdsaP256 => "ECDSA_P256",
                            EcSignatureAlgorithm::EcdsaP384 => "ECDSA_P384",
                            EcSignatureAlgorithm::Eddsa25519 => "EDDSA_25519",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "EC_SIGNATURE_ALGORITHM_UNSPECIFIED" => {
                                Some(Self::Unspecified)
                            }
                            "ECDSA_P256" => Some(Self::EcdsaP256),
                            "ECDSA_P384" => Some(Self::EcdsaP384),
                            "EDDSA_25519" => Some(Self::Eddsa25519),
                            _ => None,
                        }
                    }
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum KeyType {
                /// Represents an allowed RSA key type.
                #[prost(message, tag = "1")]
                Rsa(RsaKeyType),
                /// Represents an allowed Elliptic Curve key type.
                #[prost(message, tag = "2")]
                EllipticCurve(EcKeyType),
            }
        }
        /// [IssuanceModes][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.IssuanceModes]
        /// specifies the allowed ways in which
        /// [Certificates][google.cloud.security.privateca.v1.Certificate] may be
        /// requested from this [CaPool][google.cloud.security.privateca.v1.CaPool].
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IssuanceModes {
            /// Optional. When true, allows callers to create
            /// [Certificates][google.cloud.security.privateca.v1.Certificate] by
            /// specifying a CSR.
            #[prost(bool, tag = "1")]
            pub allow_csr_based_issuance: bool,
            /// Optional. When true, allows callers to create
            /// [Certificates][google.cloud.security.privateca.v1.Certificate] by
            /// specifying a
            /// [CertificateConfig][google.cloud.security.privateca.v1.CertificateConfig].
            #[prost(bool, tag = "2")]
            pub allow_config_based_issuance: bool,
        }
    }
    /// The tier of a [CaPool][google.cloud.security.privateca.v1.CaPool],
    /// indicating its supported functionality and/or billing SKU.
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
    pub enum Tier {
        /// Not specified.
        Unspecified = 0,
        /// Enterprise tier.
        Enterprise = 1,
        /// DevOps tier.
        Devops = 2,
    }
    impl Tier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Tier::Unspecified => "TIER_UNSPECIFIED",
                Tier::Enterprise => "ENTERPRISE",
                Tier::Devops => "DEVOPS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIER_UNSPECIFIED" => Some(Self::Unspecified),
                "ENTERPRISE" => Some(Self::Enterprise),
                "DEVOPS" => Some(Self::Devops),
                _ => None,
            }
        }
    }
}
/// A
/// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
/// corresponds to a signed X.509 certificate Revocation List (CRL). A CRL
/// contains the serial numbers of certificates that should no longer be trusted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateRevocationList {
    /// Output only. The resource name for this
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// in the format `projects/*/locations/*/caPools/*certificateAuthorities/*/
    ///     certificateRevocationLists/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The CRL sequence number that appears in pem_crl.
    #[prost(int64, tag = "2")]
    pub sequence_number: i64,
    /// Output only. The revoked serial numbers that appear in pem_crl.
    #[prost(message, repeated, tag = "3")]
    pub revoked_certificates: ::prost::alloc::vec::Vec<
        certificate_revocation_list::RevokedCertificate,
    >,
    /// Output only. The PEM-encoded X.509 CRL.
    #[prost(string, tag = "4")]
    pub pem_crl: ::prost::alloc::string::String,
    /// Output only. The location where 'pem_crl' can be accessed.
    #[prost(string, tag = "5")]
    pub access_url: ::prost::alloc::string::String,
    /// Output only. The
    /// [State][google.cloud.security.privateca.v1.CertificateRevocationList.State]
    /// for this
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList].
    #[prost(enumeration = "certificate_revocation_list::State", tag = "6")]
    pub state: i32,
    /// Output only. The time at which this
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// was updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The revision ID of this
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList].
    /// A new revision is committed whenever a new CRL is published. The format is
    /// an 8-character hexadecimal string.
    #[prost(string, tag = "9")]
    pub revision_id: ::prost::alloc::string::String,
    /// Optional. Labels with user-defined metadata.
    #[prost(btree_map = "string, string", tag = "10")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `CertificateRevocationList`.
pub mod certificate_revocation_list {
    /// Describes a revoked
    /// [Certificate][google.cloud.security.privateca.v1.Certificate].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RevokedCertificate {
        /// The resource name for the
        /// [Certificate][google.cloud.security.privateca.v1.Certificate] in the
        /// format `projects/*/locations/*/caPools/*/certificates/*`.
        #[prost(string, tag = "1")]
        pub certificate: ::prost::alloc::string::String,
        /// The serial number of the
        /// [Certificate][google.cloud.security.privateca.v1.Certificate].
        #[prost(string, tag = "2")]
        pub hex_serial_number: ::prost::alloc::string::String,
        /// The reason the
        /// [Certificate][google.cloud.security.privateca.v1.Certificate] was
        /// revoked.
        #[prost(enumeration = "super::RevocationReason", tag = "3")]
        pub revocation_reason: i32,
    }
    /// The state of a
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList],
    /// indicating if it is current.
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
        /// Not specified.
        Unspecified = 0,
        /// The
        /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
        /// is up to date.
        Active = 1,
        /// The
        /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
        /// is no longer current.
        Superseded = 2,
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
                State::Superseded => "SUPERSEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SUPERSEDED" => Some(Self::Superseded),
                _ => None,
            }
        }
    }
}
/// A [Certificate][google.cloud.security.privateca.v1.Certificate] corresponds
/// to a signed X.509 certificate issued by a
/// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificate {
    /// Output only. The resource name for this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] in the format
    /// `projects/*/locations/*/caPools/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The resource name of the issuing
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "4")]
    pub issuer_certificate_authority: ::prost::alloc::string::String,
    /// Required. Immutable. The desired lifetime of a certificate. Used to create
    /// the "not_before_time" and "not_after_time" fields inside an X.509
    /// certificate. Note that the lifetime may be truncated if it would extend
    /// past the life of any certificate authority in the issuing chain.
    #[prost(message, optional, tag = "5")]
    pub lifetime: ::core::option::Option<::prost_types::Duration>,
    /// Immutable. The resource name for a
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// used to issue this certificate, in the format
    /// `projects/*/locations/*/certificateTemplates/*`.
    /// If this is specified, the caller must have the necessary permission to
    /// use this template. If this is omitted, no template will be used.
    /// This template must be in the same location as the
    /// [Certificate][google.cloud.security.privateca.v1.Certificate].
    #[prost(string, tag = "6")]
    pub certificate_template: ::prost::alloc::string::String,
    /// Immutable. Specifies how the
    /// [Certificate][google.cloud.security.privateca.v1.Certificate]'s identity
    /// fields are to be decided. If this is omitted, the `DEFAULT` subject mode
    /// will be used.
    #[prost(enumeration = "SubjectRequestMode", tag = "7")]
    pub subject_mode: i32,
    /// Output only. Details regarding the revocation of this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate]. This
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] is considered
    /// revoked if and only if this field is present.
    #[prost(message, optional, tag = "8")]
    pub revocation_details: ::core::option::Option<certificate::RevocationDetails>,
    /// Output only. The pem-encoded, signed X.509 certificate.
    #[prost(string, tag = "9")]
    pub pem_certificate: ::prost::alloc::string::String,
    /// Output only. A structured description of the issued X.509 certificate.
    #[prost(message, optional, tag = "10")]
    pub certificate_description: ::core::option::Option<CertificateDescription>,
    /// Output only. The chain that may be used to verify the X.509 certificate.
    /// Expected to be in issuer-to-root order according to RFC 5246.
    #[prost(string, repeated, tag = "11")]
    pub pem_certificate_chain: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The time at which this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] was created.
    #[prost(message, optional, tag = "12")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] was updated.
    #[prost(message, optional, tag = "13")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(btree_map = "string, string", tag = "14")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The config used to create a signed X.509 certificate.
    #[prost(oneof = "certificate::CertificateConfig", tags = "2, 3")]
    pub certificate_config: ::core::option::Option<certificate::CertificateConfig>,
}
/// Nested message and enum types in `Certificate`.
pub mod certificate {
    /// Describes fields that are relavent to the revocation of a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RevocationDetails {
        /// Indicates why a
        /// [Certificate][google.cloud.security.privateca.v1.Certificate] was
        /// revoked.
        #[prost(enumeration = "super::RevocationReason", tag = "1")]
        pub revocation_state: i32,
        /// The time at which this
        /// [Certificate][google.cloud.security.privateca.v1.Certificate] was
        /// revoked.
        #[prost(message, optional, tag = "2")]
        pub revocation_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// The config used to create a signed X.509 certificate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CertificateConfig {
        /// Immutable. A pem-encoded X.509 certificate signing request (CSR).
        #[prost(string, tag = "2")]
        PemCsr(::prost::alloc::string::String),
        /// Immutable. A description of the certificate and key that does not require
        /// X.509 or ASN.1.
        #[prost(message, tag = "3")]
        Config(super::CertificateConfig),
    }
}
/// A
/// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
/// refers to a managed template for certificate issuance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateTemplate {
    /// Output only. The resource name for this
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// in the format `projects/*/locations/*/certificateTemplates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The maximum lifetime allowed for issued
    /// [Certificates][google.cloud.security.privateca.v1.Certificate] that use
    /// this template. If the issuing
    /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s
    /// [IssuancePolicy][google.cloud.security.privateca.v1.CaPool.IssuancePolicy]
    /// specifies a
    /// [maximum_lifetime][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.maximum_lifetime]
    /// the minimum of the two durations will be the maximum lifetime for issued
    /// [Certificates][google.cloud.security.privateca.v1.Certificate]. Note that
    /// if the issuing
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// expires before a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate]'s requested
    /// maximum_lifetime, the effective lifetime will be explicitly truncated
    ///   to match it.
    #[prost(message, optional, tag = "9")]
    pub maximum_lifetime: ::core::option::Option<::prost_types::Duration>,
    /// Optional. A set of X.509 values that will be applied to all issued
    /// certificates that use this template. If the certificate request includes
    /// conflicting values for the same properties, they will be overwritten by the
    /// values defined here. If the issuing
    /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s
    /// [IssuancePolicy][google.cloud.security.privateca.v1.CaPool.IssuancePolicy]
    /// defines conflicting
    /// [baseline_values][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.baseline_values]
    /// for the same properties, the certificate issuance request will fail.
    #[prost(message, optional, tag = "2")]
    pub predefined_values: ::core::option::Option<X509Parameters>,
    /// Optional. Describes constraints on identities that may be appear in
    /// [Certificates][google.cloud.security.privateca.v1.Certificate] issued using
    /// this template. If this is omitted, then this template will not add
    /// restrictions on a certificate's identity.
    #[prost(message, optional, tag = "3")]
    pub identity_constraints: ::core::option::Option<CertificateIdentityConstraints>,
    /// Optional. Describes the set of X.509 extensions that may appear in a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] issued using
    /// this
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
    /// If a certificate request sets extensions that don't appear in the
    /// [passthrough_extensions][google.cloud.security.privateca.v1.CertificateTemplate.passthrough_extensions],
    /// those extensions will be dropped. If the issuing
    /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s
    /// [IssuancePolicy][google.cloud.security.privateca.v1.CaPool.IssuancePolicy]
    /// defines
    /// [baseline_values][google.cloud.security.privateca.v1.CaPool.IssuancePolicy.baseline_values]
    /// that don't appear here, the certificate issuance request will fail. If this
    /// is omitted, then this template will not add restrictions on a certificate's
    /// X.509 extensions. These constraints do not apply to X.509 extensions set in
    /// this
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]'s
    /// [predefined_values][google.cloud.security.privateca.v1.CertificateTemplate.predefined_values].
    #[prost(message, optional, tag = "4")]
    pub passthrough_extensions: ::core::option::Option<CertificateExtensionConstraints>,
    /// Optional. A human-readable description of scenarios this template is
    /// intended for.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The time at which this
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(btree_map = "string, string", tag = "8")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// An [X509Parameters][google.cloud.security.privateca.v1.X509Parameters] is
/// used to describe certain fields of an X.509 certificate, such as the key
/// usage fields, fields specific to CA certificates, certificate policy
/// extensions and custom extensions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509Parameters {
    /// Optional. Indicates the intended use for keys that correspond to a
    /// certificate.
    #[prost(message, optional, tag = "1")]
    pub key_usage: ::core::option::Option<KeyUsage>,
    /// Optional. Describes options in this
    /// [X509Parameters][google.cloud.security.privateca.v1.X509Parameters] that
    /// are relevant in a CA certificate.
    #[prost(message, optional, tag = "2")]
    pub ca_options: ::core::option::Option<x509_parameters::CaOptions>,
    /// Optional. Describes the X.509 certificate policy object identifiers, per
    /// <https://tools.ietf.org/html/rfc5280#section-4.2.1.4.>
    #[prost(message, repeated, tag = "3")]
    pub policy_ids: ::prost::alloc::vec::Vec<ObjectId>,
    /// Optional. Describes Online Certificate Status Protocol (OCSP) endpoint
    /// addresses that appear in the "Authority Information Access" extension in
    /// the certificate.
    #[prost(string, repeated, tag = "4")]
    pub aia_ocsp_servers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Describes the X.509 name constraints extension.
    #[prost(message, optional, tag = "6")]
    pub name_constraints: ::core::option::Option<x509_parameters::NameConstraints>,
    /// Optional. Describes custom X.509 extensions.
    #[prost(message, repeated, tag = "5")]
    pub additional_extensions: ::prost::alloc::vec::Vec<X509Extension>,
}
/// Nested message and enum types in `X509Parameters`.
pub mod x509_parameters {
    /// Describes values that are relevant in a CA certificate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaOptions {
        /// Optional. Refers to the "CA" X.509 extension, which is a boolean value.
        /// When this value is missing, the extension will be omitted from the CA
        /// certificate.
        #[prost(bool, optional, tag = "1")]
        pub is_ca: ::core::option::Option<bool>,
        /// Optional. Refers to the path length restriction X.509 extension. For a CA
        /// certificate, this value describes the depth of subordinate CA
        /// certificates that are allowed.
        /// If this value is less than 0, the request will fail.
        /// If this value is missing, the max path length will be omitted from the
        /// CA certificate.
        #[prost(int32, optional, tag = "2")]
        pub max_issuer_path_length: ::core::option::Option<i32>,
    }
    /// Describes the X.509 name constraints extension, per
    /// <https://tools.ietf.org/html/rfc5280#section-4.2.1.10>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NameConstraints {
        /// Indicates whether or not the name constraints are marked critical.
        #[prost(bool, tag = "1")]
        pub critical: bool,
        /// Contains permitted DNS names. Any DNS name that can be
        /// constructed by simply adding zero or more labels to
        /// the left-hand side of the name satisfies the name constraint.
        /// For example, `example.com`, `www.example.com`, `www.sub.example.com`
        /// would satisfy `example.com` while `example1.com` does not.
        #[prost(string, repeated, tag = "2")]
        pub permitted_dns_names: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Contains excluded DNS names. Any DNS name that can be
        /// constructed by simply adding zero or more labels to
        /// the left-hand side of the name satisfies the name constraint.
        /// For example, `example.com`, `www.example.com`, `www.sub.example.com`
        /// would satisfy `example.com` while `example1.com` does not.
        #[prost(string, repeated, tag = "3")]
        pub excluded_dns_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Contains the permitted IP ranges. For IPv4 addresses, the ranges
        /// are expressed using CIDR notation as specified in RFC 4632.
        /// For IPv6 addresses, the ranges are expressed in similar encoding as IPv4
        /// addresses.
        #[prost(string, repeated, tag = "4")]
        pub permitted_ip_ranges: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Contains the excluded IP ranges. For IPv4 addresses, the ranges
        /// are expressed using CIDR notation as specified in RFC 4632.
        /// For IPv6 addresses, the ranges are expressed in similar encoding as IPv4
        /// addresses.
        #[prost(string, repeated, tag = "5")]
        pub excluded_ip_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Contains the permitted email addresses. The value can be a particular
        /// email address, a hostname to indicate all email addresses on that host or
        /// a domain with a leading period (e.g. `.example.com`) to indicate
        /// all email addresses in that domain.
        #[prost(string, repeated, tag = "6")]
        pub permitted_email_addresses: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Contains the excluded email addresses. The value can be a particular
        /// email address, a hostname to indicate all email addresses on that host or
        /// a domain with a leading period (e.g. `.example.com`) to indicate
        /// all email addresses in that domain.
        #[prost(string, repeated, tag = "7")]
        pub excluded_email_addresses: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// Contains the permitted URIs that apply to the host part of the name.
        /// The value can be a hostname or a domain with a
        /// leading period (like `.example.com`)
        #[prost(string, repeated, tag = "8")]
        pub permitted_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Contains the excluded URIs that apply to the host part of the name.
        /// The value can be a hostname or a domain with a
        /// leading period (like `.example.com`)
        #[prost(string, repeated, tag = "9")]
        pub excluded_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Describes a subordinate CA's issuers. This is either a resource name to a
/// known issuing
/// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority],
/// or a PEM issuer certificate chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubordinateConfig {
    #[prost(oneof = "subordinate_config::SubordinateConfig", tags = "1, 2")]
    pub subordinate_config: ::core::option::Option<
        subordinate_config::SubordinateConfig,
    >,
}
/// Nested message and enum types in `SubordinateConfig`.
pub mod subordinate_config {
    /// This message describes a subordinate CA's issuer certificate chain. This
    /// wrapper exists for compatibility reasons.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubordinateConfigChain {
        /// Required. Expected to be in leaf-to-root order according to RFC 5246.
        #[prost(string, repeated, tag = "1")]
        pub pem_certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubordinateConfig {
        /// Required. This can refer to a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// that was used to create a subordinate
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
        /// This field is used for information and usability purposes only. The
        /// resource name is in the format
        /// `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
        #[prost(string, tag = "1")]
        CertificateAuthority(::prost::alloc::string::String),
        /// Required. Contains the PEM certificate chain for the issuers of this
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority],
        /// but not pem certificate for this CA itself.
        #[prost(message, tag = "2")]
        PemIssuerChain(SubordinateConfigChain),
    }
}
/// A [PublicKey][google.cloud.security.privateca.v1.PublicKey] describes a
/// public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    /// Required. A public key. The padding and encoding
    /// must match with the `KeyFormat` value specified for the `format` field.
    #[prost(bytes = "bytes", tag = "1")]
    pub key: ::prost::bytes::Bytes,
    /// Required. The format of the public key.
    #[prost(enumeration = "public_key::KeyFormat", tag = "2")]
    pub format: i32,
}
/// Nested message and enum types in `PublicKey`.
pub mod public_key {
    /// Types of public keys formats that are supported. Currently, only `PEM`
    /// format is supported.
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
    pub enum KeyFormat {
        /// Default unspecified value.
        Unspecified = 0,
        /// The key is PEM-encoded as defined in [RFC
        /// 7468](<https://tools.ietf.org/html/rfc7468>). It can be any of the
        /// following: a PEM-encoded PKCS#1/RFC 3447 RSAPublicKey
        /// structure, an RFC 5280
        /// [SubjectPublicKeyInfo](<https://tools.ietf.org/html/rfc5280#section-4.1>)
        /// or a PEM-encoded X.509 certificate signing request (CSR). If a
        /// [SubjectPublicKeyInfo](<https://tools.ietf.org/html/rfc5280#section-4.1>)
        /// is specified, it can contain a A PEM-encoded PKCS#1/RFC 3447 RSAPublicKey
        /// or a NIST P-256/secp256r1/prime256v1 or P-384 key. If a CSR is specified,
        /// it will used solely for the purpose of extracting the public key. When
        /// generated by the service, it will always be an RFC 5280
        /// [SubjectPublicKeyInfo](<https://tools.ietf.org/html/rfc5280#section-4.1>)
        /// structure containing an algorithm identifier and a key.
        Pem = 1,
    }
    impl KeyFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeyFormat::Unspecified => "KEY_FORMAT_UNSPECIFIED",
                KeyFormat::Pem => "PEM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KEY_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "PEM" => Some(Self::Pem),
                _ => None,
            }
        }
    }
}
/// A [CertificateConfig][google.cloud.security.privateca.v1.CertificateConfig]
/// describes an X.509 certificate or CSR that is to be created, as an
/// alternative to using ASN.1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateConfig {
    /// Required. Specifies some of the values in a certificate that are related to
    /// the subject.
    #[prost(message, optional, tag = "1")]
    pub subject_config: ::core::option::Option<certificate_config::SubjectConfig>,
    /// Required. Describes how some of the technical X.509 fields in a certificate
    /// should be populated.
    #[prost(message, optional, tag = "2")]
    pub x509_config: ::core::option::Option<X509Parameters>,
    /// Optional. The public key that corresponds to this config. This is, for
    /// example, used when issuing
    /// [Certificates][google.cloud.security.privateca.v1.Certificate], but not
    /// when creating a self-signed
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// or
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// CSR.
    #[prost(message, optional, tag = "3")]
    pub public_key: ::core::option::Option<PublicKey>,
    /// Optional. When specified this provides a custom SKI to be used in the
    /// certificate. This should only be used to maintain a SKI of an existing CA
    /// originally created outside CAS, which was not generated using method (1)
    /// described in RFC 5280 section 4.2.1.2.
    #[prost(message, optional, tag = "4")]
    pub subject_key_id: ::core::option::Option<certificate_config::KeyId>,
}
/// Nested message and enum types in `CertificateConfig`.
pub mod certificate_config {
    /// These values are used to create the distinguished name and subject
    /// alternative name fields in an X.509 certificate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubjectConfig {
        /// Optional. Contains distinguished name fields such as the common name,
        /// location and organization.
        #[prost(message, optional, tag = "1")]
        pub subject: ::core::option::Option<super::Subject>,
        /// Optional. The subject alternative name fields.
        #[prost(message, optional, tag = "2")]
        pub subject_alt_name: ::core::option::Option<super::SubjectAltNames>,
    }
    /// A KeyId identifies a specific public key, usually by hashing the public
    /// key.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyId {
        /// Required. The value of this KeyId encoded in lowercase hexadecimal. This
        /// is most likely the 160 bit SHA-1 hash of the public key.
        #[prost(string, tag = "1")]
        pub key_id: ::prost::alloc::string::String,
    }
}
/// A
/// [CertificateDescription][google.cloud.security.privateca.v1.CertificateDescription]
/// describes an X.509 certificate or CSR that has been issued, as an alternative
/// to using ASN.1 / X.509.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateDescription {
    /// Describes some of the values in a certificate that are related to the
    /// subject and lifetime.
    #[prost(message, optional, tag = "1")]
    pub subject_description: ::core::option::Option<
        certificate_description::SubjectDescription,
    >,
    /// Describes some of the technical X.509 fields in a certificate.
    #[prost(message, optional, tag = "2")]
    pub x509_description: ::core::option::Option<X509Parameters>,
    /// The public key that corresponds to an issued certificate.
    #[prost(message, optional, tag = "3")]
    pub public_key: ::core::option::Option<PublicKey>,
    /// Provides a means of identifiying certificates that contain a particular
    /// public key, per <https://tools.ietf.org/html/rfc5280#section-4.2.1.2.>
    #[prost(message, optional, tag = "4")]
    pub subject_key_id: ::core::option::Option<certificate_description::KeyId>,
    /// Identifies the subject_key_id of the parent certificate, per
    /// <https://tools.ietf.org/html/rfc5280#section-4.2.1.1>
    #[prost(message, optional, tag = "5")]
    pub authority_key_id: ::core::option::Option<certificate_description::KeyId>,
    /// Describes a list of locations to obtain CRL information, i.e.
    /// the DistributionPoint.fullName described by
    /// <https://tools.ietf.org/html/rfc5280#section-4.2.1.13>
    #[prost(string, repeated, tag = "6")]
    pub crl_distribution_points: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Describes lists of issuer CA certificate URLs that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[prost(string, repeated, tag = "7")]
    pub aia_issuing_certificate_urls: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// The hash of the x.509 certificate.
    #[prost(message, optional, tag = "8")]
    pub cert_fingerprint: ::core::option::Option<
        certificate_description::CertificateFingerprint,
    >,
}
/// Nested message and enum types in `CertificateDescription`.
pub mod certificate_description {
    /// These values describe fields in an issued X.509 certificate such as the
    /// distinguished name, subject alternative names, serial number, and lifetime.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubjectDescription {
        /// Contains distinguished name fields such as the common name, location and
        /// / organization.
        #[prost(message, optional, tag = "1")]
        pub subject: ::core::option::Option<super::Subject>,
        /// The subject alternative name fields.
        #[prost(message, optional, tag = "2")]
        pub subject_alt_name: ::core::option::Option<super::SubjectAltNames>,
        /// The serial number encoded in lowercase hexadecimal.
        #[prost(string, tag = "3")]
        pub hex_serial_number: ::prost::alloc::string::String,
        /// For convenience, the actual lifetime of an issued certificate.
        #[prost(message, optional, tag = "4")]
        pub lifetime: ::core::option::Option<::prost_types::Duration>,
        /// The time at which the certificate becomes valid.
        #[prost(message, optional, tag = "5")]
        pub not_before_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The time after which the certificate is expired.
        /// Per RFC 5280, the validity period for a certificate is the period of time
        /// from not_before_time through not_after_time, inclusive.
        /// Corresponds to 'not_before_time' + 'lifetime' - 1 second.
        #[prost(message, optional, tag = "6")]
        pub not_after_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// A KeyId identifies a specific public key, usually by hashing the public
    /// key.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyId {
        /// Optional. The value of this KeyId encoded in lowercase hexadecimal. This
        /// is most likely the 160 bit SHA-1 hash of the public key.
        #[prost(string, tag = "1")]
        pub key_id: ::prost::alloc::string::String,
    }
    /// A group of fingerprints for the x509 certificate.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateFingerprint {
        /// The SHA 256 hash, encoded in hexadecimal, of the DER x509 certificate.
        #[prost(string, tag = "1")]
        pub sha256_hash: ::prost::alloc::string::String,
    }
}
/// An [ObjectId][google.cloud.security.privateca.v1.ObjectId] specifies an
/// object identifier (OID). These provide context and describe types in ASN.1
/// messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectId {
    /// Required. The parts of an OID path. The most significant parts of the path
    /// come first.
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub object_id_path: ::prost::alloc::vec::Vec<i32>,
}
/// An [X509Extension][google.cloud.security.privateca.v1.X509Extension]
/// specifies an X.509 extension, which may be used in different parts of X.509
/// objects like certificates, CSRs, and CRLs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509Extension {
    /// Required. The OID for this X.509 extension.
    #[prost(message, optional, tag = "1")]
    pub object_id: ::core::option::Option<ObjectId>,
    /// Optional. Indicates whether or not this extension is critical (i.e., if the
    /// client does not know how to handle this extension, the client should
    /// consider this to be an error).
    #[prost(bool, tag = "2")]
    pub critical: bool,
    /// Required. The value of this X.509 extension.
    #[prost(bytes = "bytes", tag = "3")]
    pub value: ::prost::bytes::Bytes,
}
/// A [KeyUsage][google.cloud.security.privateca.v1.KeyUsage] describes key usage
/// values that may appear in an X.509 certificate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyUsage {
    /// Describes high-level ways in which a key may be used.
    #[prost(message, optional, tag = "1")]
    pub base_key_usage: ::core::option::Option<key_usage::KeyUsageOptions>,
    /// Detailed scenarios in which a key may be used.
    #[prost(message, optional, tag = "2")]
    pub extended_key_usage: ::core::option::Option<key_usage::ExtendedKeyUsageOptions>,
    /// Used to describe extended key usages that are not listed in the
    /// [KeyUsage.ExtendedKeyUsageOptions][google.cloud.security.privateca.v1.KeyUsage.ExtendedKeyUsageOptions]
    /// message.
    #[prost(message, repeated, tag = "3")]
    pub unknown_extended_key_usages: ::prost::alloc::vec::Vec<ObjectId>,
}
/// Nested message and enum types in `KeyUsage`.
pub mod key_usage {
    /// [KeyUsage.KeyUsageOptions][google.cloud.security.privateca.v1.KeyUsage.KeyUsageOptions]
    /// corresponds to the key usage values described in
    /// <https://tools.ietf.org/html/rfc5280#section-4.2.1.3.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyUsageOptions {
        /// The key may be used for digital signatures.
        #[prost(bool, tag = "1")]
        pub digital_signature: bool,
        /// The key may be used for cryptographic commitments. Note that this may
        /// also be referred to as "non-repudiation".
        #[prost(bool, tag = "2")]
        pub content_commitment: bool,
        /// The key may be used to encipher other keys.
        #[prost(bool, tag = "3")]
        pub key_encipherment: bool,
        /// The key may be used to encipher data.
        #[prost(bool, tag = "4")]
        pub data_encipherment: bool,
        /// The key may be used in a key agreement protocol.
        #[prost(bool, tag = "5")]
        pub key_agreement: bool,
        /// The key may be used to sign certificates.
        #[prost(bool, tag = "6")]
        pub cert_sign: bool,
        /// The key may be used sign certificate revocation lists.
        #[prost(bool, tag = "7")]
        pub crl_sign: bool,
        /// The key may be used to encipher only.
        #[prost(bool, tag = "8")]
        pub encipher_only: bool,
        /// The key may be used to decipher only.
        #[prost(bool, tag = "9")]
        pub decipher_only: bool,
    }
    /// [KeyUsage.ExtendedKeyUsageOptions][google.cloud.security.privateca.v1.KeyUsage.ExtendedKeyUsageOptions]
    /// has fields that correspond to certain common OIDs that could be specified
    /// as an extended key usage value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtendedKeyUsageOptions {
        /// Corresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as "TLS WWW
        /// server authentication", though regularly used for non-WWW TLS.
        #[prost(bool, tag = "1")]
        pub server_auth: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as "TLS WWW
        /// client authentication", though regularly used for non-WWW TLS.
        #[prost(bool, tag = "2")]
        pub client_auth: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as "Signing of
        /// downloadable executable code client authentication".
        #[prost(bool, tag = "3")]
        pub code_signing: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as "Email
        /// protection".
        #[prost(bool, tag = "4")]
        pub email_protection: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as "Binding
        /// the hash of an object to a time".
        #[prost(bool, tag = "5")]
        pub time_stamping: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as "Signing
        /// OCSP responses".
        #[prost(bool, tag = "6")]
        pub ocsp_signing: bool,
    }
}
/// [Subject][google.cloud.security.privateca.v1.Subject] describes parts of a
/// distinguished name that, in turn, describes the subject of the certificate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    /// The "common name" of the subject.
    #[prost(string, tag = "1")]
    pub common_name: ::prost::alloc::string::String,
    /// The country code of the subject.
    #[prost(string, tag = "2")]
    pub country_code: ::prost::alloc::string::String,
    /// The organization of the subject.
    #[prost(string, tag = "3")]
    pub organization: ::prost::alloc::string::String,
    /// The organizational_unit of the subject.
    #[prost(string, tag = "4")]
    pub organizational_unit: ::prost::alloc::string::String,
    /// The locality or city of the subject.
    #[prost(string, tag = "5")]
    pub locality: ::prost::alloc::string::String,
    /// The province, territory, or regional state of the subject.
    #[prost(string, tag = "6")]
    pub province: ::prost::alloc::string::String,
    /// The street address of the subject.
    #[prost(string, tag = "7")]
    pub street_address: ::prost::alloc::string::String,
    /// The postal code of the subject.
    #[prost(string, tag = "8")]
    pub postal_code: ::prost::alloc::string::String,
}
/// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames]
/// corresponds to a more modern way of listing what the asserted identity is in
/// a certificate (i.e., compared to the "common name" in the distinguished
/// name).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAltNames {
    /// Contains only valid, fully-qualified host names.
    #[prost(string, repeated, tag = "1")]
    pub dns_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Contains only valid RFC 3986 URIs.
    #[prost(string, repeated, tag = "2")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Contains only valid RFC 2822 E-mail addresses.
    #[prost(string, repeated, tag = "3")]
    pub email_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Contains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses.
    #[prost(string, repeated, tag = "4")]
    pub ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Contains additional subject alternative name values.
    /// For each custom_san, the `value` field must contain an ASN.1 encoded
    /// UTF8String.
    #[prost(message, repeated, tag = "5")]
    pub custom_sans: ::prost::alloc::vec::Vec<X509Extension>,
}
/// Describes constraints on a
/// [Certificate][google.cloud.security.privateca.v1.Certificate]'s
/// [Subject][google.cloud.security.privateca.v1.Subject] and
/// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateIdentityConstraints {
    /// Optional. A CEL expression that may be used to validate the resolved X.509
    /// Subject and/or Subject Alternative Name before a certificate is signed. To
    /// see the full allowed syntax and some examples, see
    /// <https://cloud.google.com/certificate-authority-service/docs/using-cel>
    #[prost(message, optional, tag = "1")]
    pub cel_expression: ::core::option::Option<super::super::super::super::r#type::Expr>,
    /// Required. If this is true, the
    /// [Subject][google.cloud.security.privateca.v1.Subject] field may be copied
    /// from a certificate request into the signed certificate. Otherwise, the
    /// requested [Subject][google.cloud.security.privateca.v1.Subject] will be
    /// discarded.
    #[prost(bool, optional, tag = "2")]
    pub allow_subject_passthrough: ::core::option::Option<bool>,
    /// Required. If this is true, the
    /// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames]
    /// extension may be copied from a certificate request into the signed
    /// certificate. Otherwise, the requested
    /// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames] will
    /// be discarded.
    #[prost(bool, optional, tag = "3")]
    pub allow_subject_alt_names_passthrough: ::core::option::Option<bool>,
}
/// Describes a set of X.509 extensions that may be part of some certificate
/// issuance controls.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateExtensionConstraints {
    /// Optional. A set of named X.509 extensions. Will be combined with
    /// [additional_extensions][google.cloud.security.privateca.v1.CertificateExtensionConstraints.additional_extensions]
    /// to determine the full set of X.509 extensions.
    #[prost(
        enumeration = "certificate_extension_constraints::KnownCertificateExtension",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub known_extensions: ::prost::alloc::vec::Vec<i32>,
    /// Optional. A set of [ObjectIds][google.cloud.security.privateca.v1.ObjectId]
    /// identifying custom X.509 extensions. Will be combined with
    /// [known_extensions][google.cloud.security.privateca.v1.CertificateExtensionConstraints.known_extensions]
    /// to determine the full set of X.509 extensions.
    #[prost(message, repeated, tag = "2")]
    pub additional_extensions: ::prost::alloc::vec::Vec<ObjectId>,
}
/// Nested message and enum types in `CertificateExtensionConstraints`.
pub mod certificate_extension_constraints {
    /// Describes well-known X.509 extensions that can appear in a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate], not
    /// including the
    /// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames]
    /// extension.
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
    pub enum KnownCertificateExtension {
        /// Not specified.
        Unspecified = 0,
        /// Refers to a certificate's Key Usage extension, as described in [RFC 5280
        /// section 4.2.1.3](<https://tools.ietf.org/html/rfc5280#section-4.2.1.3>).
        /// This corresponds to the
        /// [KeyUsage.base_key_usage][google.cloud.security.privateca.v1.KeyUsage.base_key_usage]
        /// field.
        BaseKeyUsage = 1,
        /// Refers to a certificate's Extended Key Usage extension, as described in
        /// [RFC 5280
        /// section 4.2.1.12](<https://tools.ietf.org/html/rfc5280#section-4.2.1.12>).
        /// This corresponds to the
        /// [KeyUsage.extended_key_usage][google.cloud.security.privateca.v1.KeyUsage.extended_key_usage]
        /// message.
        ExtendedKeyUsage = 2,
        /// Refers to a certificate's Basic Constraints extension, as described in
        /// [RFC 5280
        /// section 4.2.1.9](<https://tools.ietf.org/html/rfc5280#section-4.2.1.9>).
        /// This corresponds to the
        /// [X509Parameters.ca_options][google.cloud.security.privateca.v1.X509Parameters.ca_options]
        /// field.
        CaOptions = 3,
        /// Refers to a certificate's Policy object identifiers, as described in
        /// [RFC 5280
        /// section 4.2.1.4](<https://tools.ietf.org/html/rfc5280#section-4.2.1.4>).
        /// This corresponds to the
        /// [X509Parameters.policy_ids][google.cloud.security.privateca.v1.X509Parameters.policy_ids]
        /// field.
        PolicyIds = 4,
        /// Refers to OCSP servers in a certificate's Authority Information Access
        /// extension, as described in
        /// [RFC 5280
        /// section 4.2.2.1](<https://tools.ietf.org/html/rfc5280#section-4.2.2.1>),
        /// This corresponds to the
        /// [X509Parameters.aia_ocsp_servers][google.cloud.security.privateca.v1.X509Parameters.aia_ocsp_servers]
        /// field.
        AiaOcspServers = 5,
        /// Refers to Name Constraints extension as described in
        /// [RFC 5280
        /// section 4.2.1.10](<https://tools.ietf.org/html/rfc5280#section-4.2.1.10>)
        NameConstraints = 6,
    }
    impl KnownCertificateExtension {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KnownCertificateExtension::Unspecified => {
                    "KNOWN_CERTIFICATE_EXTENSION_UNSPECIFIED"
                }
                KnownCertificateExtension::BaseKeyUsage => "BASE_KEY_USAGE",
                KnownCertificateExtension::ExtendedKeyUsage => "EXTENDED_KEY_USAGE",
                KnownCertificateExtension::CaOptions => "CA_OPTIONS",
                KnownCertificateExtension::PolicyIds => "POLICY_IDS",
                KnownCertificateExtension::AiaOcspServers => "AIA_OCSP_SERVERS",
                KnownCertificateExtension::NameConstraints => "NAME_CONSTRAINTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KNOWN_CERTIFICATE_EXTENSION_UNSPECIFIED" => Some(Self::Unspecified),
                "BASE_KEY_USAGE" => Some(Self::BaseKeyUsage),
                "EXTENDED_KEY_USAGE" => Some(Self::ExtendedKeyUsage),
                "CA_OPTIONS" => Some(Self::CaOptions),
                "POLICY_IDS" => Some(Self::PolicyIds),
                "AIA_OCSP_SERVERS" => Some(Self::AiaOcspServers),
                "NAME_CONSTRAINTS" => Some(Self::NameConstraints),
                _ => None,
            }
        }
    }
}
/// A [RevocationReason][google.cloud.security.privateca.v1.RevocationReason]
/// indicates whether a
/// [Certificate][google.cloud.security.privateca.v1.Certificate] has been
/// revoked, and the reason for revocation. These correspond to standard
/// revocation reasons from RFC 5280. Note that the enum labels and values in
/// this definition are not the same ASN.1 values defined in RFC 5280. These
/// values will be translated to the correct ASN.1 values when a CRL is created.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RevocationReason {
    /// Default unspecified value. This value does indicate that a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] has been
    /// revoked, but that a reason has not been recorded.
    Unspecified = 0,
    /// Key material for this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] may have
    /// leaked.
    KeyCompromise = 1,
    /// The key material for a certificate authority in the issuing path may have
    /// leaked.
    CertificateAuthorityCompromise = 2,
    /// The subject or other attributes in this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] have changed.
    AffiliationChanged = 3,
    /// This [Certificate][google.cloud.security.privateca.v1.Certificate] has been
    /// superseded.
    Superseded = 4,
    /// This [Certificate][google.cloud.security.privateca.v1.Certificate] or
    /// entities in the issuing path have ceased to operate.
    CessationOfOperation = 5,
    /// This [Certificate][google.cloud.security.privateca.v1.Certificate] should
    /// not be considered valid, it is expected that it may become valid in the
    /// future.
    CertificateHold = 6,
    /// This [Certificate][google.cloud.security.privateca.v1.Certificate] no
    /// longer has permission to assert the listed attributes.
    PrivilegeWithdrawn = 7,
    /// The authority which determines appropriate attributes for a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] may have been
    /// compromised.
    AttributeAuthorityCompromise = 8,
}
impl RevocationReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RevocationReason::Unspecified => "REVOCATION_REASON_UNSPECIFIED",
            RevocationReason::KeyCompromise => "KEY_COMPROMISE",
            RevocationReason::CertificateAuthorityCompromise => {
                "CERTIFICATE_AUTHORITY_COMPROMISE"
            }
            RevocationReason::AffiliationChanged => "AFFILIATION_CHANGED",
            RevocationReason::Superseded => "SUPERSEDED",
            RevocationReason::CessationOfOperation => "CESSATION_OF_OPERATION",
            RevocationReason::CertificateHold => "CERTIFICATE_HOLD",
            RevocationReason::PrivilegeWithdrawn => "PRIVILEGE_WITHDRAWN",
            RevocationReason::AttributeAuthorityCompromise => {
                "ATTRIBUTE_AUTHORITY_COMPROMISE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REVOCATION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
            "KEY_COMPROMISE" => Some(Self::KeyCompromise),
            "CERTIFICATE_AUTHORITY_COMPROMISE" => {
                Some(Self::CertificateAuthorityCompromise)
            }
            "AFFILIATION_CHANGED" => Some(Self::AffiliationChanged),
            "SUPERSEDED" => Some(Self::Superseded),
            "CESSATION_OF_OPERATION" => Some(Self::CessationOfOperation),
            "CERTIFICATE_HOLD" => Some(Self::CertificateHold),
            "PRIVILEGE_WITHDRAWN" => Some(Self::PrivilegeWithdrawn),
            "ATTRIBUTE_AUTHORITY_COMPROMISE" => Some(Self::AttributeAuthorityCompromise),
            _ => None,
        }
    }
}
/// Describes the way in which a
/// [Certificate][google.cloud.security.privateca.v1.Certificate]'s
/// [Subject][google.cloud.security.privateca.v1.Subject] and/or
/// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames] will be
/// resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubjectRequestMode {
    /// Not specified.
    Unspecified = 0,
    /// The default mode used in most cases. Indicates that the certificate's
    /// [Subject][google.cloud.security.privateca.v1.Subject] and/or
    /// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames] are
    /// specified in the certificate request. This mode requires the caller to have
    /// the `privateca.certificates.create` permission.
    Default = 1,
    /// A mode reserved for special cases. Indicates that the certificate should
    /// have one SPIFFE
    /// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames] set
    /// by the service based on the caller's identity. This mode will ignore any
    /// explicitly specified [Subject][google.cloud.security.privateca.v1.Subject]
    /// and/or
    /// [SubjectAltNames][google.cloud.security.privateca.v1.SubjectAltNames] in
    /// the certificate request. This mode requires the caller to have the
    /// `privateca.certificates.createForSelf` permission.
    ReflectedSpiffe = 2,
}
impl SubjectRequestMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubjectRequestMode::Unspecified => "SUBJECT_REQUEST_MODE_UNSPECIFIED",
            SubjectRequestMode::Default => "DEFAULT",
            SubjectRequestMode::ReflectedSpiffe => "REFLECTED_SPIFFE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBJECT_REQUEST_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "DEFAULT" => Some(Self::Default),
            "REFLECTED_SPIFFE" => Some(Self::ReflectedSpiffe),
            _ => None,
        }
    }
}
/// Request message for
/// [CertificateAuthorityService.CreateCertificate][google.cloud.security.privateca.v1.CertificateAuthorityService.CreateCertificate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateRequest {
    /// Required. The resource name of the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] associated with the
    /// [Certificate][google.cloud.security.privateca.v1.Certificate], in the
    /// format `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. It must be unique within a location and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`. This field is required when using a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the Enterprise [CertificateAuthority.Tier][], but is optional and its
    /// value is ignored otherwise.
    #[prost(string, tag = "2")]
    pub certificate_id: ::prost::alloc::string::String,
    /// Required. A [Certificate][google.cloud.security.privateca.v1.Certificate]
    /// with initial field values.
    #[prost(message, optional, tag = "3")]
    pub certificate: ::core::option::Option<Certificate>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If this is true, no
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] resource will
    /// be persisted regardless of the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool]'s
    /// [tier][google.cloud.security.privateca.v1.CaPool.tier], and the returned
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] will not
    /// contain the
    /// [pem_certificate][google.cloud.security.privateca.v1.Certificate.pem_certificate]
    /// field.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// Optional. The resource ID of the
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// that should issue the certificate.  This optional field will ignore the
    /// load-balancing scheme of the Pool and directly issue the certificate from
    /// the CA with the specified ID, contained in the same
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] referenced by `parent`.
    /// Per-CA quota rules apply. If left empty, a
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// will be chosen from the [CaPool][google.cloud.security.privateca.v1.CaPool]
    /// by the service. For example, to issue a
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] from a
    /// Certificate Authority with resource name
    /// "projects/my-project/locations/us-central1/caPools/my-pool/certificateAuthorities/my-ca",
    /// you can set the
    /// [parent][google.cloud.security.privateca.v1.CreateCertificateRequest.parent]
    /// to "projects/my-project/locations/us-central1/caPools/my-pool" and the
    /// [issuing_certificate_authority_id][google.cloud.security.privateca.v1.CreateCertificateRequest.issuing_certificate_authority_id]
    /// to "my-ca".
    #[prost(string, tag = "6")]
    pub issuing_certificate_authority_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificate][google.cloud.security.privateca.v1.CertificateAuthorityService.GetCertificate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateRequest {
    /// Required. The [name][google.cloud.security.privateca.v1.Certificate.name]
    /// of the [Certificate][google.cloud.security.privateca.v1.Certificate] to
    /// get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListCertificates][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificates].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificatesRequest {
    /// Required. The resource name of the location associated with the
    /// [Certificates][google.cloud.security.privateca.v1.Certificate], in the
    /// format `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Limit on the number of
    /// [Certificates][google.cloud.security.privateca.v1.Certificate] to include
    /// in the response. Further
    /// [Certificates][google.cloud.security.privateca.v1.Certificate] can
    /// subsequently be obtained by including the
    /// [ListCertificatesResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificatesResponse.next_page_token]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificatesResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificatesResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response. For
    /// details on supported filters and syntax, see [Certificates Filtering
    /// documentation](<https://cloud.google.com/certificate-authority-service/docs/sorting-filtering-certificates#filtering_support>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted. For details on
    /// supported fields and syntax, see [Certificates Sorting
    /// documentation](<https://cloud.google.com/certificate-authority-service/docs/sorting-filtering-certificates#sorting_support>).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.ListCertificates][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificates].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificatesResponse {
    /// The list of [Certificates][google.cloud.security.privateca.v1.Certificate].
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::prost::alloc::vec::Vec<Certificate>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificatesRequest.next_page_token][] to retrieve the
    /// next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CertificateAuthorityService.RevokeCertificate][google.cloud.security.privateca.v1.CertificateAuthorityService.RevokeCertificate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeCertificateRequest {
    /// Required. The resource name for this
    /// [Certificate][google.cloud.security.privateca.v1.Certificate] in the format
    /// `projects/*/locations/*/caPools/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The
    /// [RevocationReason][google.cloud.security.privateca.v1.RevocationReason] for
    /// revoking this certificate.
    #[prost(enumeration = "RevocationReason", tag = "2")]
    pub reason: i32,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCertificate][google.cloud.security.privateca.v1.CertificateAuthorityService.UpdateCertificate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateRequest {
    /// Required. [Certificate][google.cloud.security.privateca.v1.Certificate]
    /// with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate: ::core::option::Option<Certificate>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ActivateCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.ActivateCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateCertificateAuthorityRequest {
    /// Required. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The signed CA certificate issued from
    /// [FetchCertificateAuthorityCsrResponse.pem_csr][google.cloud.security.privateca.v1.FetchCertificateAuthorityCsrResponse.pem_csr].
    #[prost(string, tag = "2")]
    pub pem_ca_certificate: ::prost::alloc::string::String,
    /// Required. Must include information about the issuer of
    /// 'pem_ca_certificate', and any further issuers until the self-signed CA.
    #[prost(message, optional, tag = "3")]
    pub subordinate_config: ::core::option::Option<SubordinateConfig>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.CreateCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.CreateCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateAuthorityRequest {
    /// Required. The resource name of the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] associated with the
    /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority],
    /// in the format `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`
    #[prost(string, tag = "2")]
    pub certificate_authority_id: ::prost::alloc::string::String,
    /// Required. A
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// with initial field values.
    #[prost(message, optional, tag = "3")]
    pub certificate_authority: ::core::option::Option<CertificateAuthority>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.DisableCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.DisableCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableCertificateAuthorityRequest {
    /// Required. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. This field allows this CA to be disabled even if it's being
    /// depended on by another resource. However, doing so may result in unintended
    /// and unrecoverable effects on any dependent resource(s) since the CA will
    /// no longer be able to issue certificates.
    #[prost(bool, tag = "3")]
    pub ignore_dependent_resources: bool,
}
/// Request message for
/// [CertificateAuthorityService.EnableCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.EnableCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCertificateAuthorityRequest {
    /// Required. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.FetchCertificateAuthorityCsr][google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCertificateAuthorityCsr].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchCertificateAuthorityCsrRequest {
    /// Required. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.FetchCertificateAuthorityCsr][google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCertificateAuthorityCsr].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchCertificateAuthorityCsrResponse {
    /// Output only. The PEM-encoded signed certificate signing request (CSR).
    #[prost(string, tag = "1")]
    pub pem_csr: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.GetCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateAuthorityRequest {
    /// Required. The
    /// [name][google.cloud.security.privateca.v1.CertificateAuthority.name] of the
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListCertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificateAuthorities].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateAuthoritiesRequest {
    /// Required. The resource name of the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] associated with the
    /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority],
    /// in the format `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Limit on the number of
    /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority]
    /// to include in the response. Further
    /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority]
    /// can subsequently be obtained by including the
    /// [ListCertificateAuthoritiesResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificateAuthoritiesResponse.next_page_token]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificateAuthoritiesResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificateAuthoritiesResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.ListCertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificateAuthorities].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateAuthoritiesResponse {
    /// The list of
    /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority].
    #[prost(message, repeated, tag = "1")]
    pub certificate_authorities: ::prost::alloc::vec::Vec<CertificateAuthority>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificateAuthoritiesRequest.next_page_token][] to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CertificateAuthorityService.UndeleteCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.UndeleteCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteCertificateAuthorityRequest {
    /// Required. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.DeleteCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.DeleteCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCertificateAuthorityRequest {
    /// Required. The resource name for this
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. This field allows the CA to be deleted even if the CA has
    /// active certs. Active certs include both unrevoked and unexpired certs.
    #[prost(bool, tag = "4")]
    pub ignore_active_certificates: bool,
    /// Optional. If this flag is set, the Certificate Authority will be deleted as
    /// soon as possible without a 30-day grace period where undeletion would have
    /// been allowed. If you proceed, there will be no way to recover this CA.
    #[prost(bool, tag = "5")]
    pub skip_grace_period: bool,
    /// Optional. This field allows this ca to be deleted even if it's being
    /// depended on by another resource. However, doing so may result in unintended
    /// and unrecoverable effects on any dependent resource(s) since the CA will
    /// no longer be able to issue certificates.
    #[prost(bool, tag = "6")]
    pub ignore_dependent_resources: bool,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.UpdateCertificateAuthority].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateAuthorityRequest {
    /// Required.
    /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
    /// with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate_authority: ::core::option::Option<CertificateAuthority>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.CreateCaPool][google.cloud.security.privateca.v1.CertificateAuthorityService.CreateCaPool].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCaPoolRequest {
    /// Required. The resource name of the location associated with the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`
    #[prost(string, tag = "2")]
    pub ca_pool_id: ::prost::alloc::string::String,
    /// Required. A [CaPool][google.cloud.security.privateca.v1.CaPool] with
    /// initial field values.
    #[prost(message, optional, tag = "3")]
    pub ca_pool: ::core::option::Option<CaPool>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCaPool][google.cloud.security.privateca.v1.CertificateAuthorityService.UpdateCaPool].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCaPoolRequest {
    /// Required. [CaPool][google.cloud.security.privateca.v1.CaPool] with updated
    /// values.
    #[prost(message, optional, tag = "1")]
    pub ca_pool: ::core::option::Option<CaPool>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.DeleteCaPool][google.cloud.security.privateca.v1.CertificateAuthorityService.DeleteCaPool].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCaPoolRequest {
    /// Required. The resource name for this
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] in the format
    /// `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. This field allows this pool to be deleted even if it's being
    /// depended on by another resource. However, doing so may result in unintended
    /// and unrecoverable effects on any dependent resource(s) since the pool will
    /// no longer be able to issue certificates.
    #[prost(bool, tag = "4")]
    pub ignore_dependent_resources: bool,
}
/// Request message for
/// [CertificateAuthorityService.FetchCaCerts][google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCaCerts].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchCaCertsRequest {
    /// Required. The resource name for the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] in the format
    /// `projects/*/locations/*/caPools/*`.
    #[prost(string, tag = "1")]
    pub ca_pool: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.FetchCaCerts][google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCaCerts].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchCaCertsResponse {
    /// The PEM encoded CA certificate chains of all Certificate Authorities in
    /// this [CaPool][google.cloud.security.privateca.v1.CaPool] in the ENABLED,
    /// DISABLED, or STAGED states.
    #[prost(message, repeated, tag = "1")]
    pub ca_certs: ::prost::alloc::vec::Vec<fetch_ca_certs_response::CertChain>,
}
/// Nested message and enum types in `FetchCaCertsResponse`.
pub mod fetch_ca_certs_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertChain {
        /// The certificates that form the CA chain, from leaf to root order.
        #[prost(string, repeated, tag = "1")]
        pub certificates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Request message for
/// [CertificateAuthorityService.GetCaPool][google.cloud.security.privateca.v1.CertificateAuthorityService.GetCaPool].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCaPoolRequest {
    /// Required. The [name][google.cloud.security.privateca.v1.CaPool.name] of the
    /// [CaPool][google.cloud.security.privateca.v1.CaPool] to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListCaPools][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCaPools].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCaPoolsRequest {
    /// Required. The resource name of the location associated with the
    /// [CaPools][google.cloud.security.privateca.v1.CaPool], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Limit on the number of
    /// [CaPools][google.cloud.security.privateca.v1.CaPool] to include in the
    /// response. Further [CaPools][google.cloud.security.privateca.v1.CaPool] can
    /// subsequently be obtained by including the
    /// [ListCaPoolsResponse.next_page_token][google.cloud.security.privateca.v1.ListCaPoolsResponse.next_page_token]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCaPoolsResponse.next_page_token][google.cloud.security.privateca.v1.ListCaPoolsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.ListCaPools][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCaPools].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCaPoolsResponse {
    /// The list of [CaPools][google.cloud.security.privateca.v1.CaPool].
    #[prost(message, repeated, tag = "1")]
    pub ca_pools: ::prost::alloc::vec::Vec<CaPool>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificateAuthoritiesRequest.next_page_token][] to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificateRevocationList][google.cloud.security.privateca.v1.CertificateAuthorityService.GetCertificateRevocationList].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateRevocationListRequest {
    /// Required. The
    /// [name][google.cloud.security.privateca.v1.CertificateRevocationList.name]
    /// of the
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListCertificateRevocationLists][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificateRevocationLists].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateRevocationListsRequest {
    /// Required. The resource name of the location associated with the
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1.CertificateRevocationList],
    /// in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Limit on the number of
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// to include in the response. Further
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// can subsequently be obtained by including the
    /// [ListCertificateRevocationListsResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificateRevocationListsResponse.next_page_token]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificateRevocationListsResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificateRevocationListsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.ListCertificateRevocationLists][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificateRevocationLists].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateRevocationListsResponse {
    /// The list of
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1.CertificateRevocationList].
    #[prost(message, repeated, tag = "1")]
    pub certificate_revocation_lists: ::prost::alloc::vec::Vec<
        CertificateRevocationList,
    >,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificateRevocationListsRequest.next_page_token][] to retrieve the
    /// next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCertificateRevocationList][google.cloud.security.privateca.v1.CertificateAuthorityService.UpdateCertificateRevocationList].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateRevocationListRequest {
    /// Required.
    /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList]
    /// with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate_revocation_list: ::core::option::Option<CertificateRevocationList>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.CreateCertificateTemplate][google.cloud.security.privateca.v1.CertificateAuthorityService.CreateCertificateTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateTemplateRequest {
    /// Required. The resource name of the location associated with the
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate],
    /// in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`
    #[prost(string, tag = "2")]
    pub certificate_template_id: ::prost::alloc::string::String,
    /// Required. A
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// with initial field values.
    #[prost(message, optional, tag = "3")]
    pub certificate_template: ::core::option::Option<CertificateTemplate>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.DeleteCertificateTemplate][google.cloud.security.privateca.v1.CertificateAuthorityService.DeleteCertificateTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCertificateTemplateRequest {
    /// Required. The resource name for this
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// in the format `projects/*/locations/*/certificateTemplates/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificateTemplate][google.cloud.security.privateca.v1.CertificateAuthorityService.GetCertificateTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateTemplateRequest {
    /// Required. The
    /// [name][google.cloud.security.privateca.v1.CertificateTemplate.name] of the
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListCertificateTemplates][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificateTemplates].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateTemplatesRequest {
    /// Required. The resource name of the location associated with the
    /// [CertificateTemplates][google.cloud.security.privateca.v1.CertificateTemplate],
    /// in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Limit on the number of
    /// [CertificateTemplates][google.cloud.security.privateca.v1.CertificateTemplate]
    /// to include in the response. Further
    /// [CertificateTemplates][google.cloud.security.privateca.v1.CertificateTemplate]
    /// can subsequently be obtained by including the
    /// [ListCertificateTemplatesResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificateTemplatesResponse.next_page_token]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificateTemplatesResponse.next_page_token][google.cloud.security.privateca.v1.ListCertificateTemplatesResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// [CertificateAuthorityService.ListCertificateTemplates][google.cloud.security.privateca.v1.CertificateAuthorityService.ListCertificateTemplates].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateTemplatesResponse {
    /// The list of
    /// [CertificateTemplates][google.cloud.security.privateca.v1.CertificateTemplate].
    #[prost(message, repeated, tag = "1")]
    pub certificate_templates: ::prost::alloc::vec::Vec<CertificateTemplate>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificateTemplatesRequest.next_page_token][] to retrieve
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCertificateTemplate][google.cloud.security.privateca.v1.CertificateAuthorityService.UpdateCertificateTemplate].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateTemplateRequest {
    /// Required.
    /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
    /// with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate_template: ::core::option::Option<CertificateTemplate>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that
    /// if you must retry your request, the server will know to ignore the request
    /// if it has already been completed. The server will guarantee that for at
    /// least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
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
    /// of the operation. Operations that have successfully been cancelled
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
pub mod certificate_authority_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// [Certificate Authority
    /// Service][google.cloud.security.privateca.v1.CertificateAuthorityService]
    /// manages private certificate authorities and issued certificates.
    #[derive(Debug, Clone)]
    pub struct CertificateAuthorityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CertificateAuthorityServiceClient<T>
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
        ) -> CertificateAuthorityServiceClient<InterceptedService<T, F>>
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
            CertificateAuthorityServiceClient::new(
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
        /// Create a new [Certificate][google.cloud.security.privateca.v1.Certificate]
        /// in a given Project, Location from a particular
        /// [CaPool][google.cloud.security.privateca.v1.CaPool].
        pub async fn create_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateRequest>,
        ) -> std::result::Result<tonic::Response<super::Certificate>, tonic::Status> {
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/CreateCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "CreateCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a [Certificate][google.cloud.security.privateca.v1.Certificate].
        pub async fn get_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateRequest>,
        ) -> std::result::Result<tonic::Response<super::Certificate>, tonic::Status> {
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/GetCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "GetCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists [Certificates][google.cloud.security.privateca.v1.Certificate].
        pub async fn list_certificates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCertificatesResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/ListCertificates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "ListCertificates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Revoke a [Certificate][google.cloud.security.privateca.v1.Certificate].
        pub async fn revoke_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeCertificateRequest>,
        ) -> std::result::Result<tonic::Response<super::Certificate>, tonic::Status> {
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/RevokeCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "RevokeCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update a [Certificate][google.cloud.security.privateca.v1.Certificate].
        /// Currently, the only field you can update is the
        /// [labels][google.cloud.security.privateca.v1.Certificate.labels] field.
        pub async fn update_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateRequest>,
        ) -> std::result::Result<tonic::Response<super::Certificate>, tonic::Status> {
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/UpdateCertificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "UpdateCertificate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Activate a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// that is in state
        /// [AWAITING_USER_ACTIVATION][google.cloud.security.privateca.v1.CertificateAuthority.State.AWAITING_USER_ACTIVATION]
        /// and is of type
        /// [SUBORDINATE][google.cloud.security.privateca.v1.CertificateAuthority.Type.SUBORDINATE].
        /// After the parent Certificate Authority signs a certificate signing request
        /// from
        /// [FetchCertificateAuthorityCsr][google.cloud.security.privateca.v1.CertificateAuthorityService.FetchCertificateAuthorityCsr],
        /// this method can complete the activation process.
        pub async fn activate_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/ActivateCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "ActivateCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a new
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// in a given Project and Location.
        pub async fn create_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/CreateCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "CreateCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Disable a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
        pub async fn disable_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/DisableCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "DisableCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Enable a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
        pub async fn enable_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/EnableCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "EnableCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Fetch a certificate signing request (CSR) from a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// that is in state
        /// [AWAITING_USER_ACTIVATION][google.cloud.security.privateca.v1.CertificateAuthority.State.AWAITING_USER_ACTIVATION]
        /// and is of type
        /// [SUBORDINATE][google.cloud.security.privateca.v1.CertificateAuthority.Type.SUBORDINATE].
        /// The CSR must then be signed by the desired parent Certificate Authority,
        /// which could be another
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// resource, or could be an on-prem certificate authority. See also
        /// [ActivateCertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthorityService.ActivateCertificateAuthority].
        pub async fn fetch_certificate_authority_csr(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchCertificateAuthorityCsrRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchCertificateAuthorityCsrResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/FetchCertificateAuthorityCsr",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "FetchCertificateAuthorityCsr",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
        pub async fn get_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CertificateAuthority>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/GetCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "GetCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists
        /// [CertificateAuthorities][google.cloud.security.privateca.v1.CertificateAuthority].
        pub async fn list_certificate_authorities(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificateAuthoritiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCertificateAuthoritiesResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/ListCertificateAuthorities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "ListCertificateAuthorities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Undelete a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority]
        /// that has been deleted.
        pub async fn undelete_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/UndeleteCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "UndeleteCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
        pub async fn delete_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/DeleteCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "DeleteCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update a
        /// [CertificateAuthority][google.cloud.security.privateca.v1.CertificateAuthority].
        pub async fn update_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateAuthorityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/UpdateCertificateAuthority",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "UpdateCertificateAuthority",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a [CaPool][google.cloud.security.privateca.v1.CaPool].
        pub async fn create_ca_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCaPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/CreateCaPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "CreateCaPool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update a [CaPool][google.cloud.security.privateca.v1.CaPool].
        pub async fn update_ca_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCaPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/UpdateCaPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "UpdateCaPool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a [CaPool][google.cloud.security.privateca.v1.CaPool].
        pub async fn get_ca_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCaPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::CaPool>, tonic::Status> {
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/GetCaPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "GetCaPool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists [CaPools][google.cloud.security.privateca.v1.CaPool].
        pub async fn list_ca_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCaPoolsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCaPoolsResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/ListCaPools",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "ListCaPools",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete a [CaPool][google.cloud.security.privateca.v1.CaPool].
        pub async fn delete_ca_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCaPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/DeleteCaPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "DeleteCaPool",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// FetchCaCerts returns the current trust anchor for the
        /// [CaPool][google.cloud.security.privateca.v1.CaPool]. This will include CA
        /// certificate chains for all Certificate Authorities in the ENABLED,
        /// DISABLED, or STAGED states.
        pub async fn fetch_ca_certs(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchCaCertsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchCaCertsResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/FetchCaCerts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "FetchCaCerts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a
        /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList].
        pub async fn get_certificate_revocation_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateRevocationListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CertificateRevocationList>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/GetCertificateRevocationList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "GetCertificateRevocationList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists
        /// [CertificateRevocationLists][google.cloud.security.privateca.v1.CertificateRevocationList].
        pub async fn list_certificate_revocation_lists(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListCertificateRevocationListsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListCertificateRevocationListsResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/ListCertificateRevocationLists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "ListCertificateRevocationLists",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update a
        /// [CertificateRevocationList][google.cloud.security.privateca.v1.CertificateRevocationList].
        pub async fn update_certificate_revocation_list(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateCertificateRevocationListRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/UpdateCertificateRevocationList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "UpdateCertificateRevocationList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a new
        /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate]
        /// in a given Project and Location.
        pub async fn create_certificate_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/CreateCertificateTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "CreateCertificateTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteCertificateTemplate deletes a
        /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
        pub async fn delete_certificate_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCertificateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/DeleteCertificateTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "DeleteCertificateTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a
        /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
        pub async fn get_certificate_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CertificateTemplate>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/GetCertificateTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "GetCertificateTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists
        /// [CertificateTemplates][google.cloud.security.privateca.v1.CertificateTemplate].
        pub async fn list_certificate_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificateTemplatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCertificateTemplatesResponse>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/ListCertificateTemplates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "ListCertificateTemplates",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update a
        /// [CertificateTemplate][google.cloud.security.privateca.v1.CertificateTemplate].
        pub async fn update_certificate_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.security.privateca.v1.CertificateAuthorityService/UpdateCertificateTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.security.privateca.v1.CertificateAuthorityService",
                        "UpdateCertificateTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
