/// A Challenge from the server used to guarantee freshness of attestations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Challenge {
    /// Output only. The resource name for this Challenge in the format
    /// `projects/*/locations/*/challenges/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which this Challenge was created
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this Challenge will no longer be usable. It
    /// is also the expiration time for any tokens generated from this Challenge.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Indicates if this challenge has been used to generate a token.
    #[prost(bool, tag = "4")]
    pub used: bool,
    /// Output only. Identical to nonce, but as a string.
    #[prost(string, tag = "6")]
    pub tpm_nonce: ::prost::alloc::string::String,
}
/// Message for creating a Challenge
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChallengeRequest {
    /// Required. The resource name of the location where the Challenge will be
    /// used, in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Challenge to be created. Currently this field can be empty as
    /// all the Challenge fields are set by the server.
    #[prost(message, optional, tag = "2")]
    pub challenge: ::core::option::Option<Challenge>,
}
/// A request for an OIDC token, providing all the necessary information needed
/// for this service to verify the plaform state of the requestor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAttestationRequest {
    /// Required. The name of the Challenge whose nonce was used to generate the
    /// attestation, in the format `projects/*/locations/*/challenges/*`. The
    /// provided Challenge will be consumed, and cannot be used again.
    #[prost(string, tag = "1")]
    pub challenge: ::prost::alloc::string::String,
    /// Optional. Credentials used to populate the "emails" claim in the
    /// claims_token.
    #[prost(message, optional, tag = "2")]
    pub gcp_credentials: ::core::option::Option<GcpCredentials>,
    /// Required. The TPM-specific data provided by the attesting platform, used to
    /// populate any of the claims regarding platform state.
    #[prost(message, optional, tag = "3")]
    pub tpm_attestation: ::core::option::Option<TpmAttestation>,
    /// Optional. Optional information related to the Confidential Space TEE.
    #[prost(message, optional, tag = "4")]
    pub confidential_space_info: ::core::option::Option<ConfidentialSpaceInfo>,
    /// Optional. A collection of optional, workload-specified claims that modify
    /// the token output.
    #[prost(message, optional, tag = "5")]
    pub token_options: ::core::option::Option<TokenOptions>,
}
/// A response once an attestation has been successfully verified, containing a
/// signed OIDC token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAttestationResponse {
    /// Output only. Same as claims_token, but as a string.
    #[prost(string, tag = "2")]
    pub oidc_claims_token: ::prost::alloc::string::String,
    /// Output only. A list of messages that carry the partial error details
    /// related to VerifyAttestation.
    #[prost(message, repeated, tag = "3")]
    pub partial_errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Credentials issued by GCP which are linked to the platform attestation. These
/// will be verified server-side as part of attestaion verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpCredentials {
    /// Same as id_tokens, but as a string.
    #[prost(string, repeated, tag = "2")]
    pub service_account_id_tokens: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Options to modify claims in the token to generate custom-purpose tokens.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenOptions {
    /// Optional. Optional string to issue the token with a custom audience claim.
    /// Required if one or more nonces are specified.
    #[prost(string, tag = "1")]
    pub audience: ::prost::alloc::string::String,
    /// Optional. Optional parameter to place one or more nonces in the eat_nonce
    /// claim in the output token. The minimum size for JSON-encoded EATs is 10
    /// bytes and the maximum size is 74 bytes.
    #[prost(string, repeated, tag = "2")]
    pub nonce: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Optional token type to select what type of token to return.
    #[prost(enumeration = "TokenType", tag = "3")]
    pub token_type: i32,
}
/// TPM2 data containing everything necessary to validate any platform state
/// measured into the TPM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpmAttestation {
    /// TPM2 PCR Quotes generated by calling TPM2_Quote on each PCR bank.
    #[prost(message, repeated, tag = "1")]
    pub quotes: ::prost::alloc::vec::Vec<tpm_attestation::Quote>,
    /// The binary TCG Event Log containing events measured into the TPM by the
    /// platform firmware and operating system. Formatted as described in the
    /// "TCG PC Client Platform Firmware Profile Specification".
    #[prost(bytes = "bytes", tag = "2")]
    pub tcg_event_log: ::prost::bytes::Bytes,
    /// An Event Log containing additional events measured into the TPM that are
    /// not already present in the tcg_event_log. Formatted as described in the
    /// "Canonical Event Log Format" TCG Specification.
    #[prost(bytes = "bytes", tag = "3")]
    pub canonical_event_log: ::prost::bytes::Bytes,
    /// DER-encoded X.509 certificate of the Attestation Key (otherwise known as
    /// an AK or a TPM restricted signing key) used to generate the quotes.
    #[prost(bytes = "bytes", tag = "4")]
    pub ak_cert: ::prost::bytes::Bytes,
    /// List of DER-encoded X.509 certificates which, together with the ak_cert,
    /// chain back to a trusted Root Certificate.
    #[prost(bytes = "bytes", repeated, tag = "5")]
    pub cert_chain: ::prost::alloc::vec::Vec<::prost::bytes::Bytes>,
}
/// Nested message and enum types in `TpmAttestation`.
pub mod tpm_attestation {
    /// Information about Platform Control Registers (PCRs) including a signature
    /// over their values, which can be used for remote validation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Quote {
        /// The hash algorithm of the PCR bank being quoted, encoded as a TPM_ALG_ID
        #[prost(int32, tag = "1")]
        pub hash_algo: i32,
        /// Raw binary values of each PCRs being quoted.
        #[prost(btree_map = "int32, bytes", tag = "2")]
        pub pcr_values: ::prost::alloc::collections::BTreeMap<
            i32,
            ::prost::bytes::Bytes,
        >,
        /// TPM2 quote, encoded as a TPMS_ATTEST
        #[prost(bytes = "bytes", tag = "3")]
        pub raw_quote: ::prost::bytes::Bytes,
        /// TPM2 signature, encoded as a TPMT_SIGNATURE
        #[prost(bytes = "bytes", tag = "4")]
        pub raw_signature: ::prost::bytes::Bytes,
    }
}
/// ConfidentialSpaceInfo contains information related to the Confidential Space
/// TEE.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialSpaceInfo {
    /// Optional. A list of signed entities containing container image signatures
    /// that can be used for server-side signature verification.
    #[prost(message, repeated, tag = "1")]
    pub signed_entities: ::prost::alloc::vec::Vec<SignedEntity>,
}
/// SignedEntity represents an OCI image object containing everything necessary
/// to verify container image signatures.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedEntity {
    /// Optional. A list of container image signatures attached to an OCI image
    /// object.
    #[prost(message, repeated, tag = "1")]
    pub container_image_signatures: ::prost::alloc::vec::Vec<ContainerImageSignature>,
}
/// ContainerImageSignature holds necessary metadata to verify a container image
/// signature.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerImageSignature {
    /// Optional. The binary signature payload following the SimpleSigning format
    /// <https://github.com/sigstore/cosign/blob/main/specs/SIGNATURE_SPEC.md#simple-signing.>
    /// This payload includes the container image digest.
    #[prost(bytes = "bytes", tag = "1")]
    pub payload: ::prost::bytes::Bytes,
    /// Optional. A signature over the payload.
    /// The container image digest is incorporated into the signature as follows:
    /// 1. Generate a SimpleSigning format payload that includes the container
    /// image digest.
    /// 2. Generate a signature over SHA256 digest of the payload.
    /// The signature generation process can be represented as follows:
    /// `Sign(sha256(SimpleSigningPayload(sha256(Image Manifest))))`
    #[prost(bytes = "bytes", tag = "2")]
    pub signature: ::prost::bytes::Bytes,
    /// Optional. Reserved for future use.
    #[prost(bytes = "bytes", tag = "3")]
    pub public_key: ::prost::bytes::Bytes,
    /// Optional. Reserved for future use.
    #[prost(enumeration = "SigningAlgorithm", tag = "4")]
    pub sig_alg: i32,
}
/// SigningAlgorithm enumerates all the supported signing algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SigningAlgorithm {
    /// Unspecified signing algorithm.
    Unspecified = 0,
    /// RSASSA-PSS with a SHA256 digest.
    RsassaPssSha256 = 1,
    /// RSASSA-PKCS1 v1.5 with a SHA256 digest.
    RsassaPkcs1v15Sha256 = 2,
    /// ECDSA on the P-256 Curve with a SHA256 digest.
    EcdsaP256Sha256 = 3,
}
impl SigningAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SigningAlgorithm::Unspecified => "SIGNING_ALGORITHM_UNSPECIFIED",
            SigningAlgorithm::RsassaPssSha256 => "RSASSA_PSS_SHA256",
            SigningAlgorithm::RsassaPkcs1v15Sha256 => "RSASSA_PKCS1V15_SHA256",
            SigningAlgorithm::EcdsaP256Sha256 => "ECDSA_P256_SHA256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNING_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
            "RSASSA_PSS_SHA256" => Some(Self::RsassaPssSha256),
            "RSASSA_PKCS1V15_SHA256" => Some(Self::RsassaPkcs1v15Sha256),
            "ECDSA_P256_SHA256" => Some(Self::EcdsaP256Sha256),
            _ => None,
        }
    }
}
/// Token type enum contains the different types of token responses Confidential
/// Space supports
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TokenType {
    /// Unspecified token type
    Unspecified = 0,
    /// OpenID Connect (OIDC) token type
    Oidc = 1,
}
impl TokenType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TokenType::Unspecified => "TOKEN_TYPE_UNSPECIFIED",
            TokenType::Oidc => "TOKEN_TYPE_OIDC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TOKEN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TOKEN_TYPE_OIDC" => Some(Self::Oidc),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod confidential_computing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct ConfidentialComputingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConfidentialComputingClient<T>
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
        ) -> ConfidentialComputingClient<InterceptedService<T, F>>
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
            ConfidentialComputingClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new Challenge in a given project and location.
        pub async fn create_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChallengeRequest>,
        ) -> std::result::Result<tonic::Response<super::Challenge>, tonic::Status> {
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
                "/google.cloud.confidentialcomputing.v1.ConfidentialComputing/CreateChallenge",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.confidentialcomputing.v1.ConfidentialComputing",
                        "CreateChallenge",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Verifies the provided attestation info, returning a signed OIDC token.
        pub async fn verify_attestation(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyAttestationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VerifyAttestationResponse>,
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
                "/google.cloud.confidentialcomputing.v1.ConfidentialComputing/VerifyAttestation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.confidentialcomputing.v1.ConfidentialComputing",
                        "VerifyAttestation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
