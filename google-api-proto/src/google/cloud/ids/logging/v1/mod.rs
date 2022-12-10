/// A threat detected by Cloud IDS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatLog {
    /// Name of the threat, e,g. "Suspicious HTTP Evasion"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Unique ID of the threat.
    #[prost(string, tag = "13")]
    pub threat_id: ::prost::alloc::string::String,
    /// The time of the alert.
    #[prost(message, optional, tag = "2")]
    pub alert_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Severity of threat.
    #[prost(enumeration = "threat_log::Severity", tag = "19")]
    pub alert_severity: i32,
    /// The type of the threat, e.g. "Spyware".
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// Category (sub-type) of the threat, e.g. "code-execution".
    #[prost(string, tag = "18")]
    pub category: ::prost::alloc::string::String,
    /// The source IP Address of the packet, e.g. "35.191.8.79"
    #[prost(string, tag = "5")]
    pub source_ip_address: ::prost::alloc::string::String,
    /// The source port of the packet, e.g. 8080
    #[prost(int32, tag = "6")]
    pub source_port: i32,
    /// The destination IP Address of the packet, e.g. "192.168.100.2"
    #[prost(string, tag = "7")]
    pub destination_ip_address: ::prost::alloc::string::String,
    /// The destination port of the packet, e.g. 100
    #[prost(int32, tag = "8")]
    pub destination_port: i32,
    /// The IP protocol of the packet, e.g. "TCP".
    #[prost(string, tag = "9")]
    pub ip_protocol: ::prost::alloc::string::String,
    /// The direction of the packet - an optional field.
    #[prost(enumeration = "threat_log::Direction", tag = "10")]
    pub direction: i32,
    /// ID of the Layer 4 session of the threat.
    #[prost(string, tag = "14")]
    pub session_id: ::prost::alloc::string::String,
    /// Number of sessions with same source IP, destination IP, application, and
    /// type seen within 5 seconds.
    #[prost(string, tag = "15")]
    pub repeat_count: ::prost::alloc::string::String,
    /// Application associated with the session.
    #[prost(string, tag = "16")]
    pub application: ::prost::alloc::string::String,
    /// Variable field. URI or filename of the relevant threat, if applicable.
    #[prost(string, tag = "17")]
    pub uri_or_filename: ::prost::alloc::string::String,
    /// CVE IDs of the threat.
    #[prost(string, repeated, tag = "20")]
    pub cves: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Details of the threat reported by the IDS VM
    #[prost(string, tag = "11")]
    pub details: ::prost::alloc::string::String,
    /// The network associated with the IDS Endpoint.
    #[prost(string, tag = "12")]
    pub network: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ThreatLog`.
pub mod threat_log {
    /// Describes the type of severity of the threat.
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
    pub enum Severity {
        /// Default value - should never be used.
        Unspecified = 0,
        Low = 2,
        Medium = 3,
        High = 4,
        Critical = 5,
        Informational = 6,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Low => "LOW",
                Severity::Medium => "MEDIUM",
                Severity::High => "HIGH",
                Severity::Critical => "CRITICAL",
                Severity::Informational => "INFORMATIONAL",
            }
        }
    }
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
    pub enum Direction {
        /// Default value - permitted since Direction is optional.
        Undefined = 0,
        /// Ingress traffic.
        ClientToServer = 1,
        /// Egress traffic.
        ServerToClient = 2,
    }
    impl Direction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Direction::Undefined => "DIRECTION_UNDEFINED",
                Direction::ClientToServer => "CLIENT_TO_SERVER",
                Direction::ServerToClient => "SERVER_TO_CLIENT",
            }
        }
    }
}
/// Traffic detected by Cloud IDS.
/// Fields taken from:
/// <https://docs.paloaltonetworks.com/pan-os/8-1/pan-os-admin/monitoring/use-syslog-for-monitoring/syslog-field-descriptions/traffic-log-fields.html.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficLog {
    /// Time of session start.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Elapsed time of the session.
    #[prost(message, optional, tag = "2")]
    pub elapsed_time: ::core::option::Option<::prost_types::Duration>,
    /// The network associated with the IDS Endpoint.
    #[prost(string, tag = "3")]
    pub network: ::prost::alloc::string::String,
    /// The source IP Address of the packet, e.g. "35.191.8.79"
    #[prost(string, tag = "4")]
    pub source_ip_address: ::prost::alloc::string::String,
    /// The source port of the packet, e.g. 8080
    #[prost(int32, tag = "5")]
    pub source_port: i32,
    /// The destination IP Address of the packet, e.g. "192.168.100.2"
    #[prost(string, tag = "6")]
    pub destination_ip_address: ::prost::alloc::string::String,
    /// The destination port of the packet, e.g. 100
    #[prost(int32, tag = "7")]
    pub destination_port: i32,
    /// The IP protocol of the packet, e.g. "TCP".
    #[prost(string, tag = "8")]
    pub ip_protocol: ::prost::alloc::string::String,
    /// Application associated with the session.
    #[prost(string, tag = "9")]
    pub application: ::prost::alloc::string::String,
    /// The direction of the packet.
    #[prost(string, tag = "12")]
    pub session_id: ::prost::alloc::string::String,
    /// Number of sessions with same source IP, destination IP, application, and
    /// type seen within 5 seconds.
    #[prost(string, tag = "13")]
    pub repeat_count: ::prost::alloc::string::String,
    /// Total number of bytes transferred in the session.
    #[prost(int64, tag = "14")]
    pub total_bytes: i64,
    /// Total number of packets transferred in the session.
    #[prost(int64, tag = "15")]
    pub total_packets: i64,
}
