/// A reference to uniquely identify an account according to India's UPI
/// standards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountReference {
    /// IFSC of the account's bank branch.
    #[prost(string, tag = "1")]
    pub ifsc: ::prost::alloc::string::String,
    /// Output only. Type of account. Examples include SAVINGS, CURRENT, etc.
    #[prost(string, tag = "2")]
    pub account_type: ::prost::alloc::string::String,
    /// Unique number for an account in a bank and branch.
    #[prost(string, tag = "3")]
    pub account_number: ::prost::alloc::string::String,
}
/// A participant in a payment settlement transaction processed by the issuer
/// switch. The participant could either be the payer or the payee in the
/// transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettlementParticipant {
    /// The participant information.
    #[prost(message, optional, tag = "1")]
    pub participant: ::core::option::Option<Participant>,
    /// Information about a merchant who is a participant in the payment. This
    /// field will be specified only if the participant is a merchant.
    #[prost(message, optional, tag = "2")]
    pub merchant_info: ::core::option::Option<MerchantInfo>,
    /// Output only. The mobile number of the participant.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub mobile: ::prost::alloc::string::String,
    /// Output only. Additional details about the payment settlement. Values will
    /// be populated depending on whether the settlement transaction succeeded or
    /// failed.
    #[prost(message, optional, tag = "4")]
    pub details: ::core::option::Option<settlement_participant::SettlementDetails>,
}
/// Nested message and enum types in `SettlementParticipant`.
pub mod settlement_participant {
    /// Details about a payment settlement.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SettlementDetails {
        /// Output only. The id for the settlement in the bank's backend system. In
        /// UPI, this maps to the approval reference number. This value will be
        /// present only if this API transaction's state is SUCCEEDED.
        #[prost(string, tag = "1")]
        pub backend_settlement_id: ::prost::alloc::string::String,
        /// Output only. A code indicating additional details about the settlement.
        /// In UPI, this maps to the response code.
        #[prost(string, tag = "2")]
        pub code: ::prost::alloc::string::String,
        /// Output only. A code indicating additional details about the reversal of a
        /// settlement. In UPI, this maps to the reversal response code.
        #[prost(string, tag = "3")]
        pub reversal_code: ::prost::alloc::string::String,
        /// Output only. The amount settled as part of this API transaction. If the
        /// settlement had failed, then this value will be 0.00.
        #[prost(message, optional, tag = "4")]
        pub settled_amount: ::core::option::Option<
            super::super::super::super::super::r#type::Money,
        >,
    }
}
/// A participant's device tags
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceDetails {
    /// The payment application on the device.
    #[prost(string, tag = "1")]
    pub payment_app: ::prost::alloc::string::String,
    /// The capability of the device.
    #[prost(string, tag = "2")]
    pub capability: ::prost::alloc::string::String,
    /// The geo-code of the device.
    #[prost(message, optional, tag = "3")]
    pub geo_code: ::core::option::Option<super::super::super::super::r#type::LatLng>,
    /// The device's ID.
    #[prost(string, tag = "4")]
    pub id: ::prost::alloc::string::String,
    /// The device's IP address.
    #[prost(string, tag = "5")]
    pub ip_address: ::prost::alloc::string::String,
    /// The coarse location of the device.
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// The operating system on the device.
    #[prost(string, tag = "7")]
    pub operating_system: ::prost::alloc::string::String,
    /// The device's telecom provider.
    #[prost(string, tag = "8")]
    pub telecom_provider: ::prost::alloc::string::String,
    /// The type of device.
    #[prost(string, tag = "9")]
    pub r#type: ::prost::alloc::string::String,
}
/// A participant in a transaction processed by the issuer switch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    /// The payment address of the participant. In the UPI system, this will be the
    /// virtual payment address (VPA) of the participant.
    #[prost(string, tag = "1")]
    pub payment_address: ::prost::alloc::string::String,
    /// The persona of the participant.
    #[prost(enumeration = "participant::Persona", tag = "2")]
    pub persona: i32,
    /// The name of the participant.
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    /// Output only. Unique identification of an account according to India's UPI
    /// standards.
    #[prost(message, optional, tag = "4")]
    pub account: ::core::option::Option<AccountReference>,
    /// Output only. The device info of the participant.
    #[prost(message, optional, tag = "5")]
    pub device_details: ::core::option::Option<DeviceDetails>,
    /// Output only. The mobile number of the participant.
    #[prost(string, tag = "6")]
    pub mobile_number: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Participant`.
pub mod participant {
    /// The type of the participant.
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
    pub enum Persona {
        /// Unspecified persona.
        Unspecified = 0,
        /// Participant is an entity.
        Entity = 1,
        /// Participant is a person.
        Person = 2,
    }
    impl Persona {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Persona::Unspecified => "PERSONA_UNSPECIFIED",
                Persona::Entity => "ENTITY",
                Persona::Person => "PERSON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERSONA_UNSPECIFIED" => Some(Self::Unspecified),
                "ENTITY" => Some(Self::Entity),
                "PERSON" => Some(Self::Person),
                _ => None,
            }
        }
    }
}
/// A merchant entity participating in a payment settlement transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantInfo {
    /// A unique identifier for the merchant.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the merchant who is a party in the payment. Includes multiple
    /// possible names for the merchant.
    #[prost(message, optional, tag = "2")]
    pub merchant: ::core::option::Option<MerchantName>,
    /// Additional information about the merchant.
    #[prost(message, optional, tag = "3")]
    pub additional_info: ::core::option::Option<MerchantAdditionalInfo>,
}
/// The name of a merchant who is a participant in a payment settlement
/// transaction. Includes multiple possible names for the merchant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantName {
    /// The brand name of the merchant.
    #[prost(string, tag = "1")]
    pub brand: ::prost::alloc::string::String,
    /// The merchant's legal name.
    #[prost(string, tag = "2")]
    pub legal: ::prost::alloc::string::String,
    /// The franchise name under which the merchant operates.
    #[prost(string, tag = "3")]
    pub franchise: ::prost::alloc::string::String,
}
/// Additional merchant information specific to India's UPI requirements.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantAdditionalInfo {
    /// Merchant Category Code (MCC) as specified by UPI. This is a four-digit
    /// number listed in ISO 18245 for retail financial services.
    #[prost(string, tag = "1")]
    pub category_code: ::prost::alloc::string::String,
    /// A unique identifier for the merchant store where the payment settlement
    /// transaction occurred.
    #[prost(string, tag = "2")]
    pub store_id: ::prost::alloc::string::String,
    /// A unique identifier for the POS terminal in the store where the payment
    /// settlement transaction occurred.
    #[prost(string, tag = "3")]
    pub terminal_id: ::prost::alloc::string::String,
    /// Indicates the type of merchant.
    #[prost(enumeration = "merchant_additional_info::Type", tag = "4")]
    pub r#type: i32,
    /// Indicates the genre of the merchant.
    #[prost(enumeration = "merchant_additional_info::Genre", tag = "5")]
    pub genre: i32,
    /// Indicates the merchant's onboarding type.
    #[prost(enumeration = "merchant_additional_info::OnboardingType", tag = "6")]
    pub onboarding_type: i32,
    /// Indicates the merchant's owner type.
    #[prost(enumeration = "merchant_additional_info::OwnershipType", tag = "7")]
    pub ownership_type: i32,
}
/// Nested message and enum types in `MerchantAdditionalInfo`.
pub mod merchant_additional_info {
    /// Indicates the merchant's type as a small or large merchant.
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
        /// Unspecified merchant type.
        Unspecified = 0,
        /// Large merchant.
        Large = 1,
        /// Small merchant.
        Small = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Large => "LARGE",
                Type::Small => "SMALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LARGE" => Some(Self::Large),
                "SMALL" => Some(Self::Small),
                _ => None,
            }
        }
    }
    /// Indicates whether the merchant is an online or offline merchant.
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
    pub enum Genre {
        /// Unspecified merchant genre.
        Unspecified = 0,
        /// Offline merchant
        Offline = 1,
        /// Online merchant.
        Online = 2,
    }
    impl Genre {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Genre::Unspecified => "GENRE_UNSPECIFIED",
                Genre::Offline => "OFFLINE",
                Genre::Online => "ONLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GENRE_UNSPECIFIED" => Some(Self::Unspecified),
                "OFFLINE" => Some(Self::Offline),
                "ONLINE" => Some(Self::Online),
                _ => None,
            }
        }
    }
    /// Indicates whether the merchant has been onboarded by a bank or an
    /// aggregator.
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
    pub enum OnboardingType {
        /// Unspecified merchant onboarding type.
        Unspecified = 0,
        /// Onboarded by aggreagator.
        Aggregator = 1,
        /// Onboarded by bank.
        Bank = 2,
        /// Onboarded by the UPI network.
        Network = 3,
        /// Onboarded by the TPAP.
        Tpap = 4,
    }
    impl OnboardingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OnboardingType::Unspecified => "ONBOARDING_TYPE_UNSPECIFIED",
                OnboardingType::Aggregator => "AGGREGATOR",
                OnboardingType::Bank => "BANK",
                OnboardingType::Network => "NETWORK",
                OnboardingType::Tpap => "TPAP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ONBOARDING_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AGGREGATOR" => Some(Self::Aggregator),
                "BANK" => Some(Self::Bank),
                "NETWORK" => Some(Self::Network),
                "TPAP" => Some(Self::Tpap),
                _ => None,
            }
        }
    }
    /// Indicates the ownership type of the merchant.
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
    pub enum OwnershipType {
        /// Unspecified merchant ownership type.
        Unspecified = 0,
        /// Properietary ownership.
        Proprietary = 1,
        /// Partnership ownership.
        Partnership = 2,
        /// Public ownership.
        Public = 3,
        /// Private ownership.
        Private = 4,
        /// Other ownership model.
        Others = 5,
    }
    impl OwnershipType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OwnershipType::Unspecified => "OWNERSHIP_TYPE_UNSPECIFIED",
                OwnershipType::Proprietary => "PROPRIETARY",
                OwnershipType::Partnership => "PARTNERSHIP",
                OwnershipType::Public => "PUBLIC",
                OwnershipType::Private => "PRIVATE",
                OwnershipType::Others => "OTHERS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OWNERSHIP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROPRIETARY" => Some(Self::Proprietary),
                "PARTNERSHIP" => Some(Self::Partnership),
                "PUBLIC" => Some(Self::Public),
                "PRIVATE" => Some(Self::Private),
                "OTHERS" => Some(Self::Others),
                _ => None,
            }
        }
    }
}
/// The API type for a transaction. Every transaction processed by the issuer
/// switch will be one of these API types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiType {
    /// Unspecified API type.
    Unspecified = 0,
    /// Balance API. Maps to UPI's `BalEnq` API. This is a metadata
    /// transaction API.
    Balance = 1,
    /// Check transaction status API. Maps to UPI's `ChkTxn` API. This is a
    /// metadata transaction API.
    CheckStatus = 2,
    /// Complaint API. Maps to UPI's `Complaint` API. This is a dispute and issue
    /// resolution API.
    Complaint = 3,
    /// Heart beat API. Maps to UPI's `Hbt` API. This is a metadata transaction
    /// API.
    HeartBeat = 4,
    /// Initiate registration API. Maps to UPI's `Otp` API. This is a metadata
    /// transaction API.
    InitiateRegistration = 5,
    /// List accounts API. Maps to UPI's `ListAccount` API. This is a metadata
    /// transaction API.
    ListAccounts = 6,
    /// Mandate API. Maps to UPI's `Mandate` API. This is a metadata transaction
    /// API.
    Mandate = 7,
    /// Mandate confirmation API. Maps to UPI's `MandateConfirmation` API. This is
    /// a metadata transaction API.
    MandateConfirmation = 8,
    /// Payment settlement API. Maps to UPI's `Pay` API. This is a financial
    /// transaction API.
    SettlePayment = 9,
    /// Update credentials API. Maps to UPI's `SetCre` API. This is a metadata
    /// transaction API.
    UpdateCredentials = 10,
    /// Validate registration API. Maps to UPI's `RegMob` API. This is a metadata
    /// transaction API.
    ValidateRegistration = 11,
    /// Validate customer API. Maps to UPI's `ValCust` API. This is a validation
    /// API.
    ValidateCustomer = 12,
    /// Voucher API. Maps to UPI's `Voucher` API.
    Voucher = 13,
    /// Voucher confirmation API. Maps to UPI's `VoucherConfirmation` API.
    VoucherConfirmation = 14,
    /// Activation API. Maps to UPI's `Activation` API.
    Activation = 15,
}
impl ApiType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiType::Unspecified => "API_TYPE_UNSPECIFIED",
            ApiType::Balance => "BALANCE",
            ApiType::CheckStatus => "CHECK_STATUS",
            ApiType::Complaint => "COMPLAINT",
            ApiType::HeartBeat => "HEART_BEAT",
            ApiType::InitiateRegistration => "INITIATE_REGISTRATION",
            ApiType::ListAccounts => "LIST_ACCOUNTS",
            ApiType::Mandate => "MANDATE",
            ApiType::MandateConfirmation => "MANDATE_CONFIRMATION",
            ApiType::SettlePayment => "SETTLE_PAYMENT",
            ApiType::UpdateCredentials => "UPDATE_CREDENTIALS",
            ApiType::ValidateRegistration => "VALIDATE_REGISTRATION",
            ApiType::ValidateCustomer => "VALIDATE_CUSTOMER",
            ApiType::Voucher => "VOUCHER",
            ApiType::VoucherConfirmation => "VOUCHER_CONFIRMATION",
            ApiType::Activation => "ACTIVATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "API_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BALANCE" => Some(Self::Balance),
            "CHECK_STATUS" => Some(Self::CheckStatus),
            "COMPLAINT" => Some(Self::Complaint),
            "HEART_BEAT" => Some(Self::HeartBeat),
            "INITIATE_REGISTRATION" => Some(Self::InitiateRegistration),
            "LIST_ACCOUNTS" => Some(Self::ListAccounts),
            "MANDATE" => Some(Self::Mandate),
            "MANDATE_CONFIRMATION" => Some(Self::MandateConfirmation),
            "SETTLE_PAYMENT" => Some(Self::SettlePayment),
            "UPDATE_CREDENTIALS" => Some(Self::UpdateCredentials),
            "VALIDATE_REGISTRATION" => Some(Self::ValidateRegistration),
            "VALIDATE_CUSTOMER" => Some(Self::ValidateCustomer),
            "VOUCHER" => Some(Self::Voucher),
            "VOUCHER_CONFIRMATION" => Some(Self::VoucherConfirmation),
            "ACTIVATION" => Some(Self::Activation),
            _ => None,
        }
    }
}
/// The type of a transaction. Every transaction processed by the issuer switch
/// will be one of these transaction types. Transaction types are associated with
/// a particular API type. This associated is documented with each value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    /// Unspecified transaction type.
    Unspecified = 0,
    /// Autoupdate transaction type. This is associated with the `CHECK_STATUS`
    /// API type. Maps to UPI's `AUTOUPDATE` type.
    Autoupdate = 1,
    /// Balance check transaction type. This is associated with the
    /// `BALANCE_ENQUIRY` API type. Maps to UPI's `BalChk` type.
    BalanceCheck = 2,
    /// Balance enquiry transaction type. This is associated with the
    /// `BALANCE_ENQUIRY` API type. Maps to UPI's `BalEnq` type.
    BalanceEnquiry = 3,
    /// Check status transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `CHECKSTATUS` type.
    CheckStatus = 4,
    /// Check transaction type. This is associated with the `CHECK_STATUS` API
    /// type. Maps to UPI's `ChkTxn` type.
    CheckTransaction = 5,
    /// Complaint transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `COMPLAINT` type.
    Complaint = 6,
    /// Create transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `CREATE` type.
    Create = 7,
    /// Credit transaction type. This is associated with the `SETTLE_PAYMENT` API
    /// type. Maps to UPI's `CREDIT` type.
    Credit = 8,
    /// Debit transaction type. This is associated with the `SETTLE_PAYMENT` API
    /// type. Maps to UPI's `DEBIT` type.
    Debit = 9,
    /// Dispute transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `DISPUTE` type.
    Dispute = 10,
    /// Heart beat transaction type. This is associated with `HEART_BEAT` API type.
    /// Maps to UPI's `Hbt` type.
    HeartBeat = 11,
    /// List accounts transaction type. This is associated with `LIST_ACCOUNTS` API
    /// type. Maps to UPI's `ListAccount` type.
    ListAccounts = 12,
    /// Mandate notification transaction type. This is associated with the
    /// `VALIDATE_CUSTOMER` API type. Maps to UPI's `MandateNotification` type.
    MandateNotification = 13,
    /// OTP transaction type. This is associated with the `INITIATE_REGISTRATION`
    /// API type. Maps to UPI's `Otp` type.
    Otp = 14,
    /// Pause transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `PAUSE` type.
    Pause = 15,
    /// Redeem transaction type. This is associated with the `VOUCHER_CONFIRMATION`
    /// API type. Maps to UPI's `REDEEM` type.
    Redeem = 16,
    /// Refund transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `REFUND` type.
    Refund = 17,
    /// Register mobile transaction type. This is associated with the
    /// `VALIDATE_REGISTRATION` API type. Maps to UPI's `RegMob` type.
    RegisterMobile = 18,
    /// Reversal transaction type. This is associated with the `SETTLE_PAYMENT` and
    /// `COMPLAINT` API types. Maps to UPI's `REVERSAL` type.
    Reversal = 19,
    /// Revoke transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `REVOKE` type.
    Revoke = 20,
    /// Status update transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `STATUSUPDATE` type.
    StatusUpdate = 21,
    /// Update transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `UNPAUSE` type.
    Unpause = 22,
    /// Update transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `UPDATE` type.
    Update = 23,
    /// Update credentials transaction type. This is associated with
    /// `UPDATE_CREDENTIALS` API type. Maps to UPI's `SetCre` type.
    UpdateCredentials = 24,
    /// Validate customer transaction type. This is associated with
    /// `VALIDATE_CUSTOMER` API type. Maps to UPI's `ValCust` type.
    ValidateCustomer = 25,
    /// Activation international transaction type. This is associated with
    /// 'ACTIVATION' API type. Maps to UPI's `International` type.
    ActivationInternational = 26,
    /// Activation UPI services transaction type. This is associated with
    /// 'ACTIVATION' API type. Maps to UPI's `UPI Services` type.
    ActivationUpiServices = 27,
}
impl TransactionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionType::Unspecified => "TRANSACTION_TYPE_UNSPECIFIED",
            TransactionType::Autoupdate => "TRANSACTION_TYPE_AUTOUPDATE",
            TransactionType::BalanceCheck => "TRANSACTION_TYPE_BALANCE_CHECK",
            TransactionType::BalanceEnquiry => "TRANSACTION_TYPE_BALANCE_ENQUIRY",
            TransactionType::CheckStatus => "TRANSACTION_TYPE_CHECK_STATUS",
            TransactionType::CheckTransaction => "TRANSACTION_TYPE_CHECK_TRANSACTION",
            TransactionType::Complaint => "TRANSACTION_TYPE_COMPLAINT",
            TransactionType::Create => "TRANSACTION_TYPE_CREATE",
            TransactionType::Credit => "TRANSACTION_TYPE_CREDIT",
            TransactionType::Debit => "TRANSACTION_TYPE_DEBIT",
            TransactionType::Dispute => "TRANSACTION_TYPE_DISPUTE",
            TransactionType::HeartBeat => "TRANSACTION_TYPE_HEART_BEAT",
            TransactionType::ListAccounts => "TRANSACTION_TYPE_LIST_ACCOUNTS",
            TransactionType::MandateNotification => {
                "TRANSACTION_TYPE_MANDATE_NOTIFICATION"
            }
            TransactionType::Otp => "TRANSACTION_TYPE_OTP",
            TransactionType::Pause => "TRANSACTION_TYPE_PAUSE",
            TransactionType::Redeem => "TRANSACTION_TYPE_REDEEM",
            TransactionType::Refund => "TRANSACTION_TYPE_REFUND",
            TransactionType::RegisterMobile => "TRANSACTION_TYPE_REGISTER_MOBILE",
            TransactionType::Reversal => "TRANSACTION_TYPE_REVERSAL",
            TransactionType::Revoke => "TRANSACTION_TYPE_REVOKE",
            TransactionType::StatusUpdate => "TRANSACTION_TYPE_STATUS_UPDATE",
            TransactionType::Unpause => "TRANSACTION_TYPE_UNPAUSE",
            TransactionType::Update => "TRANSACTION_TYPE_UPDATE",
            TransactionType::UpdateCredentials => "TRANSACTION_TYPE_UPDATE_CREDENTIALS",
            TransactionType::ValidateCustomer => "TRANSACTION_TYPE_VALIDATE_CUSTOMER",
            TransactionType::ActivationInternational => {
                "TRANSACTION_TYPE_ACTIVATION_INTERNATIONAL"
            }
            TransactionType::ActivationUpiServices => {
                "TRANSACTION_TYPE_ACTIVATION_UPI_SERVICES"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRANSACTION_TYPE_AUTOUPDATE" => Some(Self::Autoupdate),
            "TRANSACTION_TYPE_BALANCE_CHECK" => Some(Self::BalanceCheck),
            "TRANSACTION_TYPE_BALANCE_ENQUIRY" => Some(Self::BalanceEnquiry),
            "TRANSACTION_TYPE_CHECK_STATUS" => Some(Self::CheckStatus),
            "TRANSACTION_TYPE_CHECK_TRANSACTION" => Some(Self::CheckTransaction),
            "TRANSACTION_TYPE_COMPLAINT" => Some(Self::Complaint),
            "TRANSACTION_TYPE_CREATE" => Some(Self::Create),
            "TRANSACTION_TYPE_CREDIT" => Some(Self::Credit),
            "TRANSACTION_TYPE_DEBIT" => Some(Self::Debit),
            "TRANSACTION_TYPE_DISPUTE" => Some(Self::Dispute),
            "TRANSACTION_TYPE_HEART_BEAT" => Some(Self::HeartBeat),
            "TRANSACTION_TYPE_LIST_ACCOUNTS" => Some(Self::ListAccounts),
            "TRANSACTION_TYPE_MANDATE_NOTIFICATION" => Some(Self::MandateNotification),
            "TRANSACTION_TYPE_OTP" => Some(Self::Otp),
            "TRANSACTION_TYPE_PAUSE" => Some(Self::Pause),
            "TRANSACTION_TYPE_REDEEM" => Some(Self::Redeem),
            "TRANSACTION_TYPE_REFUND" => Some(Self::Refund),
            "TRANSACTION_TYPE_REGISTER_MOBILE" => Some(Self::RegisterMobile),
            "TRANSACTION_TYPE_REVERSAL" => Some(Self::Reversal),
            "TRANSACTION_TYPE_REVOKE" => Some(Self::Revoke),
            "TRANSACTION_TYPE_STATUS_UPDATE" => Some(Self::StatusUpdate),
            "TRANSACTION_TYPE_UNPAUSE" => Some(Self::Unpause),
            "TRANSACTION_TYPE_UPDATE" => Some(Self::Update),
            "TRANSACTION_TYPE_UPDATE_CREDENTIALS" => Some(Self::UpdateCredentials),
            "TRANSACTION_TYPE_VALIDATE_CUSTOMER" => Some(Self::ValidateCustomer),
            "TRANSACTION_TYPE_ACTIVATION_INTERNATIONAL" => {
                Some(Self::ActivationInternational)
            }
            "TRANSACTION_TYPE_ACTIVATION_UPI_SERVICES" => {
                Some(Self::ActivationUpiServices)
            }
            _ => None,
        }
    }
}
/// XmlApiType specifies the API type of the request or response as specified in
/// the XML payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XmlApiType {
    /// Unspecified API type.
    Unspecified = 0,
    /// Balance enquiry request API type. Maps to UPI's `ReqBalEnq` API.
    ReqBalEnq = 1,
    /// Check transaction request API type. Maps to UPI's `ReqChkTxn` API.
    ReqChkTxn = 2,
    /// Complaint request API type. Maps to UPI's `ReqComplaint` API.
    ReqComplaint = 3,
    /// Heart beat request API type. Maps to UPI's `ReqHbt` API.
    ReqHbt = 4,
    /// List accounts request API type. Maps to UPI's `ReqListAccount` API.
    ReqListAccount = 5,
    /// Mandate request  API. Maps to UPI's `ReqMandate` API.
    ReqMandate = 6,
    /// Mandate confirmation request API type. Maps to UPI's
    /// `ReqMandateConfirmation` API.
    ReqMandateConfirmation = 7,
    /// OTP request API. Maps to UPI's `ReqOtp` API.
    ReqOtp = 8,
    /// Payment settlement request API type. Maps to UPI's `ReqPay` API.
    ReqPay = 9,
    /// Register mobile request API type. Maps to UPI's `ReqRegMob` API.
    ReqRegMob = 10,
    /// Update credentials request API type. Maps to UPI's `ReqSetCre` API.
    ReqSetCre = 11,
    /// Validate customer request API type. Maps to UPI's `ReqValCust`.
    ReqValCust = 12,
    /// Create voucher request API type. Maps to UPI's `ReqVoucher`.
    ReqVoucher = 13,
    /// Voucher confirmation request API type. Maps to UPI's
    /// `ReqVoucherConfirmation` API.
    ReqVoucherConfirmation = 14,
    /// Transaction confirmation request API type. Maps to UPI's
    /// `ReqTxnConfirmation` API.
    ReqTxnConfirmation = 15,
    /// Balance enquiry response API type. Maps to UPI's `RespBalEnq` API.
    RespBalEnq = 16,
    /// Check transaction response API type. Maps to UPI's `RespChkTxn` API.
    RespChkTxn = 17,
    /// Complaint response API type. Maps to UPI's `RespComplaint` API.
    RespComplaint = 18,
    /// Heart beat response API type. Maps to UPI's `RespHbt` API.
    RespHbt = 19,
    /// List accounts response API type. Maps to UPI's `RespListAccount` API.
    RespListAccount = 20,
    /// Mandate response API type. Maps to UPI's `RespMandate` API.
    RespMandate = 21,
    /// Mandate confirmation response API type. Maps to UPI's
    /// `RespMandateConfirmation` API.
    RespMandateConfirmation = 22,
    /// OTP response API. Maps to UPI's `RespOtp` API.
    RespOtp = 23,
    /// Payment settlement response API type. Maps to UPI's `RespPay` API.
    RespPay = 24,
    /// Register mobile response API type. Maps to UPI's `RespRegMob` API.
    RespRegMob = 25,
    /// Update credentials response API type. Maps to UPI's `RespSetCre` API.
    RespSetCre = 26,
    /// Validate customer response API type. Maps to UPI's `RespValCust`.
    RespValCust = 27,
    /// Create voucher response API type. Maps to UPI's `RespVoucher`.
    RespVoucher = 28,
    /// Voucher confirmation responseAPI type. Maps to UPI's
    /// `RespVoucherConfirmation` API.
    RespVoucherConfirmation = 29,
    /// Transaction confirmation response API type. Maps to UPI's
    /// `RespTxnConfirmation` API.
    RespTxnConfirmation = 30,
    /// Activation request API type. Maps to UPI's `ReqActivation` API.
    ReqActivation = 31,
    /// Activation response API type. Maps to UPI's `RespActivation` API.
    RespActivation = 32,
}
impl XmlApiType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            XmlApiType::Unspecified => "XML_API_TYPE_UNSPECIFIED",
            XmlApiType::ReqBalEnq => "REQ_BAL_ENQ",
            XmlApiType::ReqChkTxn => "REQ_CHK_TXN",
            XmlApiType::ReqComplaint => "REQ_COMPLAINT",
            XmlApiType::ReqHbt => "REQ_HBT",
            XmlApiType::ReqListAccount => "REQ_LIST_ACCOUNT",
            XmlApiType::ReqMandate => "REQ_MANDATE",
            XmlApiType::ReqMandateConfirmation => "REQ_MANDATE_CONFIRMATION",
            XmlApiType::ReqOtp => "REQ_OTP",
            XmlApiType::ReqPay => "REQ_PAY",
            XmlApiType::ReqRegMob => "REQ_REG_MOB",
            XmlApiType::ReqSetCre => "REQ_SET_CRE",
            XmlApiType::ReqValCust => "REQ_VAL_CUST",
            XmlApiType::ReqVoucher => "REQ_VOUCHER",
            XmlApiType::ReqVoucherConfirmation => "REQ_VOUCHER_CONFIRMATION",
            XmlApiType::ReqTxnConfirmation => "REQ_TXN_CONFIRMATION",
            XmlApiType::RespBalEnq => "RESP_BAL_ENQ",
            XmlApiType::RespChkTxn => "RESP_CHK_TXN",
            XmlApiType::RespComplaint => "RESP_COMPLAINT",
            XmlApiType::RespHbt => "RESP_HBT",
            XmlApiType::RespListAccount => "RESP_LIST_ACCOUNT",
            XmlApiType::RespMandate => "RESP_MANDATE",
            XmlApiType::RespMandateConfirmation => "RESP_MANDATE_CONFIRMATION",
            XmlApiType::RespOtp => "RESP_OTP",
            XmlApiType::RespPay => "RESP_PAY",
            XmlApiType::RespRegMob => "RESP_REG_MOB",
            XmlApiType::RespSetCre => "RESP_SET_CRE",
            XmlApiType::RespValCust => "RESP_VAL_CUST",
            XmlApiType::RespVoucher => "RESP_VOUCHER",
            XmlApiType::RespVoucherConfirmation => "RESP_VOUCHER_CONFIRMATION",
            XmlApiType::RespTxnConfirmation => "RESP_TXN_CONFIRMATION",
            XmlApiType::ReqActivation => "REQ_ACTIVATION",
            XmlApiType::RespActivation => "RESP_ACTIVATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "XML_API_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "REQ_BAL_ENQ" => Some(Self::ReqBalEnq),
            "REQ_CHK_TXN" => Some(Self::ReqChkTxn),
            "REQ_COMPLAINT" => Some(Self::ReqComplaint),
            "REQ_HBT" => Some(Self::ReqHbt),
            "REQ_LIST_ACCOUNT" => Some(Self::ReqListAccount),
            "REQ_MANDATE" => Some(Self::ReqMandate),
            "REQ_MANDATE_CONFIRMATION" => Some(Self::ReqMandateConfirmation),
            "REQ_OTP" => Some(Self::ReqOtp),
            "REQ_PAY" => Some(Self::ReqPay),
            "REQ_REG_MOB" => Some(Self::ReqRegMob),
            "REQ_SET_CRE" => Some(Self::ReqSetCre),
            "REQ_VAL_CUST" => Some(Self::ReqValCust),
            "REQ_VOUCHER" => Some(Self::ReqVoucher),
            "REQ_VOUCHER_CONFIRMATION" => Some(Self::ReqVoucherConfirmation),
            "REQ_TXN_CONFIRMATION" => Some(Self::ReqTxnConfirmation),
            "RESP_BAL_ENQ" => Some(Self::RespBalEnq),
            "RESP_CHK_TXN" => Some(Self::RespChkTxn),
            "RESP_COMPLAINT" => Some(Self::RespComplaint),
            "RESP_HBT" => Some(Self::RespHbt),
            "RESP_LIST_ACCOUNT" => Some(Self::RespListAccount),
            "RESP_MANDATE" => Some(Self::RespMandate),
            "RESP_MANDATE_CONFIRMATION" => Some(Self::RespMandateConfirmation),
            "RESP_OTP" => Some(Self::RespOtp),
            "RESP_PAY" => Some(Self::RespPay),
            "RESP_REG_MOB" => Some(Self::RespRegMob),
            "RESP_SET_CRE" => Some(Self::RespSetCre),
            "RESP_VAL_CUST" => Some(Self::RespValCust),
            "RESP_VOUCHER" => Some(Self::RespVoucher),
            "RESP_VOUCHER_CONFIRMATION" => Some(Self::RespVoucherConfirmation),
            "RESP_TXN_CONFIRMATION" => Some(Self::RespTxnConfirmation),
            "REQ_ACTIVATION" => Some(Self::ReqActivation),
            "RESP_ACTIVATION" => Some(Self::RespActivation),
            _ => None,
        }
    }
}
/// A rule that is executed by the issuer switch while processing an
/// API transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    /// The unique identifier for this resource.
    /// Format: projects/{project}/rules/{rule}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the rule.
    #[prost(string, tag = "2")]
    pub rule_description: ::prost::alloc::string::String,
    /// The API Type for which this rule gets executed. A value of
    /// `API_TYPE_UNSPECIFIED` indicates that the rule is executed for all API
    /// transactions.
    #[prost(enumeration = "ApiType", tag = "3")]
    pub api_type: i32,
    /// The transaction type for which this rule gets executed. A value of
    /// `TRANSACTION_TYPE_UNSPECIFIED` indicates that the rule is executed for
    /// all transaction types.
    #[prost(enumeration = "TransactionType", tag = "4")]
    pub transaction_type: i32,
}
/// The metadata associated with a rule. This defines data that are used by the
/// rule during execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleMetadata {
    /// The unique identifier for this resource.
    /// Format: projects/{project}/rules/{rule}/metadata/{metadata}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the rule metadata.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Type of rule metadata.
    #[prost(enumeration = "rule_metadata::Type", tag = "3")]
    pub r#type: i32,
}
/// Nested message and enum types in `RuleMetadata`.
pub mod rule_metadata {
    /// The type of metadata.
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
        /// Unspecified type.
        Unspecified = 0,
        /// List type. Indicates that the metadata contains a list of values which
        /// the rule requires for execution.
        List = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::List => "LIST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LIST" => Some(Self::List),
                _ => None,
            }
        }
    }
}
/// Represent a single value in a rule's metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleMetadataValue {
    /// Output only. The unique identifier for this resource.
    /// Format: projects/{project}/rules/{rule}/metadata/{metadata}/values/{value}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the resource which could be of type string or
    /// AccountReference. The metadata values for rules
    /// BlockedPayeeAccountReqPayDebitRule, BlockedPayerAccountReqPayDebitRule,
    /// BlockedPayeeAccountReqPayCreditRule and BlockedPayerAccountReqPayCreditRule
    /// should be of type AccountReference. For all other rules, metadata values
    /// should be of type string.
    ///
    /// The length of the `value` field depends on the type of
    /// the value being used for the rule metadata. The following are the minimum
    /// and maximum lengths for the different types of values.
    ///
    /// Value Type | Minimum Length | Maximum Length |
    /// -------- | -------- | -------- |
    /// Bank account IFSC   | 11   | 11   |
    /// Bank account number   | 1   | 255  |
    /// Device identifier   | 1   | 255   |
    /// Mobile number   | 12   | 12  |
    /// Virtual private address (VPA)   | 3   | 255   |
    #[prost(oneof = "rule_metadata_value::Value", tags = "2, 3")]
    pub value: ::core::option::Option<rule_metadata_value::Value>,
}
/// Nested message and enum types in `RuleMetadataValue`.
pub mod rule_metadata_value {
    /// The value of the resource which could be of type string or
    /// AccountReference. The metadata values for rules
    /// BlockedPayeeAccountReqPayDebitRule, BlockedPayerAccountReqPayDebitRule,
    /// BlockedPayeeAccountReqPayCreditRule and BlockedPayerAccountReqPayCreditRule
    /// should be of type AccountReference. For all other rules, metadata values
    /// should be of type string.
    ///
    /// The length of the `value` field depends on the type of
    /// the value being used for the rule metadata. The following are the minimum
    /// and maximum lengths for the different types of values.
    ///
    /// Value Type | Minimum Length | Maximum Length |
    /// -------- | -------- | -------- |
    /// Bank account IFSC   | 11   | 11   |
    /// Bank account number   | 1   | 255  |
    /// Device identifier   | 1   | 255   |
    /// Mobile number   | 12   | 12  |
    /// Virtual private address (VPA)   | 3   | 255   |
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The value for string metadata.
        #[prost(string, tag = "2")]
        Id(::prost::alloc::string::String),
        /// The value for account reference metadata.
        #[prost(message, tag = "3")]
        AccountReference(super::AccountReference),
    }
}
/// Request body for the `ListRules` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRulesRequest {
    /// Required. The parent resource must have the format of `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rules to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 50,
    /// at most 50 rules will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRulesRequest` call.
    /// Specify this parameter to retrieve the next page of rules.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response body for the `ListRules` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRulesResponse {
    /// List of rules satisfying the specified filter criteria.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
    /// Pass this token in a subsequent `ListRulesRequest` call to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of rules matching request criteria across all pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// Request body for the `ListRuleMetadata` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataRequest {
    /// Required. The parent resource. The format is
    /// `projects/{project}/rules/{rule}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rule metadata to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 50,
    /// at most 50 rule metadata will be returned. The maximum value is 1000;
    /// values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRuleMetadataRequest` call.
    /// Specify this parameter to retrieve the next page of rule metadata.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response body for the `ListRuleMetadata` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataResponse {
    /// List of rule metadata associated with the rule.
    #[prost(message, repeated, tag = "1")]
    pub rule_metadata: ::prost::alloc::vec::Vec<RuleMetadata>,
    /// Pass this token in a subsequent `ListRuleMetadataRequest` call to continue
    /// to list results. If all results have been returned, this field is an empty
    /// string or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of rule metadata matching request criteria across all pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// Request body for the `ListRuleMetadataValues` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataValuesRequest {
    /// Required. The parent resource. The format is
    /// `projects/{project}/rules/{rule}/metadata/{metadata}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of metadata values to return. The service may return
    /// fewer than this value. If unspecified or if the specified value is less
    /// than 1, at most 50 rule metadata values will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token received from a previous `ListRuleMetadataValuesRequest`
    /// call. Specify this parameter to retrieve the next page of rule metadata
    /// values.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response body for ListRuleMetadataValues. Contains a List of values for a
/// given rule metadata resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataValuesResponse {
    /// List of values for a given rule metadata resource identifier.
    #[prost(message, repeated, tag = "1")]
    pub rule_metadata_values: ::prost::alloc::vec::Vec<RuleMetadataValue>,
    /// Pass this token in a subsequent `ListRuleMetadataValuesRequest` call to
    /// continue to list results. If all results have been returned, this field is
    /// an empty string or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request body for the `BatchCreateRuleMetadataValues` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRuleMetadataValuesRequest {
    /// The parent resource shared by all ruleMetadataValue being created. The
    /// format is `projects/{project}/rules/{rule}/metadata/{metadata}`. The
    /// \[CreateRuleMetadataValueRequest.parent][google.cloud.paymentgateway.issuerswitch.v1.CreateRuleMetadataValueRequest.parent\]
    /// field in the
    /// \[CreateRuleMetadataValueRequest][google.cloud.paymentgateway.issuerswitch.v1.CreateRuleMetadataValueRequest\]
    /// messages contained in this request must match this field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the resources to create.
    /// A maximum of 1000 RuleMetadataValues can be created in a batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CreateRuleMetadataValueRequest>,
}
/// Response body for the `BatchCreateRuleMetadataValues` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRuleMetadataValuesResponse {
    /// List of RuleMetadataValue created.
    #[prost(message, repeated, tag = "1")]
    pub rule_metadata_value: ::prost::alloc::vec::Vec<RuleMetadataValue>,
}
/// Request for creating a single `RuleMetadataValue`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuleMetadataValueRequest {
    /// Required. The parent resource where this RuleMetadataValue will be created.
    /// The format is `projects/{project}/rules/{rule}/metadata/{metadata}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The rule metadata value to create or add to a list.
    #[prost(message, optional, tag = "2")]
    pub rule_metadata_value: ::core::option::Option<RuleMetadataValue>,
}
/// Request body for the `BatchDeleteRuleMetadataValues` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteRuleMetadataValuesRequest {
    /// The parent resource shared by all RuleMetadataValues being deleted. The
    /// format is `projects/{project}/rules/{rule}/metadata/{metadata}`. If this is
    /// set, the parent of all of the RuleMetadataValues specified in the
    /// list of names must match this field.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names of the rule metadata values to delete.
    /// A maximum of 1000 RuleMetadataValue can be deleted in a batch.
    /// Format: projects/{project}/rules/{rule}/metadata/{metadata}/values/{value}
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod issuer_switch_rules_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages rules used by the issuer switch's rules engine.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchRulesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IssuerSwitchRulesClient<T>
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
        ) -> IssuerSwitchRulesClient<InterceptedService<T, F>>
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
            IssuerSwitchRulesClient::new(InterceptedService::new(inner, interceptor))
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
        /// List all rules that are applied on transactions by the issuer switch. Rules
        /// can be filtered on API type and transaction type.
        pub async fn list_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRulesResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/ListRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules",
                        "ListRules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all rule metadata for a given rule identifier.
        pub async fn list_rule_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuleMetadataResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/ListRuleMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules",
                        "ListRuleMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all metadata values for a rule metadata identifier.
        pub async fn list_rule_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRuleMetadataValuesResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/ListRuleMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules",
                        "ListRuleMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create (add) multiple values to the list of values under the specified rule
        /// metadata resource.
        pub async fn batch_create_rule_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateRuleMetadataValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCreateRuleMetadataValuesResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/BatchCreateRuleMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules",
                        "BatchCreateRuleMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete (remove) multiple values from the list of values under the specified
        /// rules metadata resource.
        pub async fn batch_delete_rule_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteRuleMetadataValuesRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/BatchDeleteRuleMetadataValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules",
                        "BatchDeleteRuleMetadataValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// A complaint processed by the issuer switch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Complaint {
    /// The name of the complaint. This uniquely identifies the complaint.
    /// Format of name is
    /// projects/{project_id}/complaints/{complaint_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The reason for raising the complaint. This maps adjustment flag
    /// and reason code for the complaint to `reqAdjFlag` and `reqAdjCode` in
    /// complaint request respectively while raising a complaint.
    #[prost(message, optional, tag = "2")]
    pub raise_complaint_adjustment: ::core::option::Option<RaiseComplaintAdjustment>,
    /// Required. Details required for raising / resolving a complaint.
    #[prost(message, optional, tag = "4")]
    pub details: ::core::option::Option<CaseDetails>,
    /// Output only. Response to the raised / resolved complaint.
    #[prost(message, optional, tag = "5")]
    pub response: ::core::option::Option<CaseResponse>,
    /// The reason for resolving the complaint. It provides adjustment values while
    /// resolving and for already resolved complaints. This maps adjustment flag
    /// and reason code for the complaint to `reqAdjFlag` and `reqAdjCode` in
    /// complaint request respectively when a complete resolution is done via
    /// Resolve Complaint API otherwise maps to `respAdjFlag` and `respAdjCode` in
    /// complaint response respectively when a complaint request from UPI is
    /// directly resolved by issuer switch.
    #[prost(message, optional, tag = "6")]
    pub resolve_complaint_adjustment: ::core::option::Option<ResolveComplaintAdjustment>,
}
/// Request for the `CreateComplaint` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateComplaintRequest {
    /// Required. The parent resource for the complaint. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The complaint to be raised.
    #[prost(message, optional, tag = "2")]
    pub complaint: ::core::option::Option<Complaint>,
}
/// Request for the `ResolveComplaint` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveComplaintRequest {
    /// Required. The complaint to be resolved.
    #[prost(message, optional, tag = "1")]
    pub complaint: ::core::option::Option<Complaint>,
}
/// A dispute processed by the issuer switch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dispute {
    /// The name of the dispute. This uniquely identifies the dispute.
    /// Format of name is
    /// projects/{project_id}/disputes/{dispute_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The reason for raising the dispute. This maps adjustment flag
    /// and reason code for the dispute to `reqAdjFlag` and `reqAdjCode` in
    /// complaint request respectively while raising a dispute.
    #[prost(message, optional, tag = "2")]
    pub raise_dispute_adjustment: ::core::option::Option<RaiseDisputeAdjustment>,
    /// Required. Details required for raising/resolving dispute.
    #[prost(message, optional, tag = "4")]
    pub details: ::core::option::Option<CaseDetails>,
    /// Output only. Response to the raised/resolved dispute.
    #[prost(message, optional, tag = "5")]
    pub response: ::core::option::Option<CaseResponse>,
    /// The reason for resolving the dispute. It provides adjustment values while
    /// resolving and for already resolved disputes. This maps adjustment flag
    /// and reason code for the dispute to `reqAdjFlag` and `reqAdjCode` in
    /// dispute request respectively while resolving a dispute.
    #[prost(message, optional, tag = "6")]
    pub resolve_dispute_adjustment: ::core::option::Option<ResolveDisputeAdjustment>,
}
/// Request for the `CreateDispute` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisputeRequest {
    /// Required. The parent resource for the dispute. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The dispute to be raised.
    #[prost(message, optional, tag = "2")]
    pub dispute: ::core::option::Option<Dispute>,
}
/// Request for the `ResolveDispute` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveDisputeRequest {
    /// Required. The dispute to be resolved.
    #[prost(message, optional, tag = "1")]
    pub dispute: ::core::option::Option<Dispute>,
}
/// Details of original transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginalTransaction {
    /// Required. Uniquely identifies the original transaction. This maps to the
    /// `Txn.Id` value of the original transaction in India's UPI system.
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
    /// Required. Retrieval Reference Number (RRN) of the original transaction.
    #[prost(string, tag = "2")]
    pub retrieval_reference_number: ::prost::alloc::string::String,
    /// Timestamp of the original transaction request.
    #[prost(message, optional, tag = "3")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Details of the complaint or dispute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseDetails {
    /// Required. Details of original transaction.
    #[prost(message, optional, tag = "1")]
    pub original_transaction: ::core::option::Option<OriginalTransaction>,
    /// Required. Initiator of the complaint / dispute.
    #[prost(enumeration = "TransactionSubType", tag = "2")]
    pub transaction_sub_type: i32,
    /// Required. The adjustment amount in URCS for the complaint / dispute. This
    /// maps to `reqAdjAmount` in complaint request.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::super::r#type::Money>,
    /// The original response code which has been updated in the complaint
    /// Response. This should map to settlement response code currently available
    /// in URCS system.
    #[prost(string, tag = "4")]
    pub original_settlement_response_code: ::prost::alloc::string::String,
    /// Required. Set to true if the complaint / dispute belongs to current
    /// settlement cycle, false otherwise.
    #[prost(bool, tag = "5")]
    pub current_cycle: bool,
}
/// Response to the complaint or dispute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseResponse {
    /// Complaint Reference Number(CRN) sent by UPI as a reference against the
    /// generated complaint / dispute.
    #[prost(string, tag = "1")]
    pub complaint_reference_number: ::prost::alloc::string::String,
    /// The adjustment amount of the response. This maps to `adjAmt` in
    /// complaint response.
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::super::r#type::Money>,
    /// The adjustment flag in response to the complaint. This maps adjustment flag
    /// in URCS for the complaint transaction to `Resp.Ref.adjFlag` in complaint
    /// response.
    #[prost(string, tag = "3")]
    pub adjustment_flag: ::prost::alloc::string::String,
    /// The adjustment code in response to the complaint. This maps reason code in
    /// URCS for the complaint transaction to `Resp.Ref.adjCode` in complaint
    /// response.
    #[prost(string, tag = "4")]
    pub adjustment_code: ::prost::alloc::string::String,
    /// It defines the Adjustment Reference ID which has been updated in the
    /// complaint response. This maps to `adjRefID` in complaint response.
    #[prost(string, tag = "5")]
    pub adjustment_reference_id: ::prost::alloc::string::String,
    /// Adjustment Remarks. This maps to `adjRemarks` in complaint response.
    #[prost(string, tag = "6")]
    pub adjustment_remarks: ::prost::alloc::string::String,
    /// The Approval Reference Number. This maps to `approvalNum` in complaint
    /// response.
    #[prost(string, tag = "7")]
    pub approval_number: ::prost::alloc::string::String,
    /// Process Status of the transaction. This maps to `procStatus` in complaint
    /// response.
    #[prost(string, tag = "8")]
    pub process_status: ::prost::alloc::string::String,
    /// The adjustment timestamp when bank performs the adjustment for the received
    /// complaint request. This maps to `adjTs` in complaint response.
    #[prost(message, optional, tag = "9")]
    pub adjustment_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The result of the transaction.
    #[prost(enumeration = "case_response::Result", tag = "12")]
    pub result: i32,
    /// The details of the participant of the original financial transaction.
    #[prost(oneof = "case_response::Participant", tags = "10, 11")]
    pub participant: ::core::option::Option<case_response::Participant>,
}
/// Nested message and enum types in `CaseResponse`.
pub mod case_response {
    /// The status of the complaint or dispute transaction. This maps to `result`
    /// in complaint transaction response.
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
    pub enum Result {
        /// Unspecified status.
        Unspecified = 0,
        /// The transaction has successfully completed.
        Success = 1,
        /// The transaction has failed.
        Failure = 2,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unspecified => "RESULT_UNSPECIFIED",
                Result::Success => "SUCCESS",
                Result::Failure => "FAILURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCESS" => Some(Self::Success),
                "FAILURE" => Some(Self::Failure),
                _ => None,
            }
        }
    }
    /// The details of the participant of the original financial transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Participant {
        /// The payer in the original financial transaction.
        #[prost(message, tag = "10")]
        Payer(super::Participant),
        /// The payee in the original financial transaction.
        #[prost(message, tag = "11")]
        Payee(super::Participant),
    }
}
/// The adjusment flag and reason code for raising complaint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaiseComplaintAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This
    /// maps to `reqAdjFlag` in complaint request and `respAdjFlag` in complaint
    /// response.
    #[prost(enumeration = "raise_complaint_adjustment::AdjustmentFlag", tag = "1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This
    /// maps to `reqAdjCode` in complaint request.
    #[prost(enumeration = "raise_complaint_adjustment::ReasonCode", tag = "2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `RaiseComplaintAdjustment`.
pub mod raise_complaint_adjustment {
    /// The adjusment flag for raising complaint.
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
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Complaint Raise. This flag maps to the `PBRB` adjustment flag as defined
        /// in NPCI's `UDIR` specification.
        Raise = 1,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::Raise => "RAISE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADJUSTMENT_FLAG_UNSPECIFIED" => Some(Self::Unspecified),
                "RAISE" => Some(Self::Raise),
                _ => None,
            }
        }
    }
    /// The reason for raising complaint.
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
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Customer account has not yet reversed for a declined pay transaction.
        /// This reason code maps to the `U005` reason code as defined in NPCI's
        /// `UDIR` specification.
        CustomerAccountNotReversed = 1,
        /// Goods / services are not provided for approved transaction.
        /// This reason code maps to the `U008` reason code as defined in NPCI's
        /// `UDIR` specification.
        GoodsServicesNotProvided = 2,
        /// Customer account not credited back for declined transaction. This
        /// reason code maps to the `U009` reason code as defined in NPCI's `UDIR`
        /// specification.
        CustomerAccountNotCreditedBack = 3,
        /// Beneficiary account is not credited for successful pay transaction. This
        /// reason code maps to the `U010` reason code as defined in NPCI's `UDIR`
        /// specification.
        BeneficiaryAccountNotCredited = 4,
        /// Credit not processed for cancelled or returned goods and services.
        /// This reason code maps to the `U021` reason code as defined in NPCI's
        /// `UDIR` specification.
        GoodsServicesCreditNotProcessed = 5,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `U022` reason code as defined in
        /// NPCI's `UDIR` specification.
        MerchantNotReceivedConfirmation = 6,
        /// Paid by alternate means / Duplicate payment. This reason code maps to the
        /// `U023` reason code as defined in NPCI's `UDIR` specification.
        PaidByAlternateMeans = 7,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::CustomerAccountNotReversed => "CUSTOMER_ACCOUNT_NOT_REVERSED",
                ReasonCode::GoodsServicesNotProvided => "GOODS_SERVICES_NOT_PROVIDED",
                ReasonCode::CustomerAccountNotCreditedBack => {
                    "CUSTOMER_ACCOUNT_NOT_CREDITED_BACK"
                }
                ReasonCode::BeneficiaryAccountNotCredited => {
                    "BENEFICIARY_ACCOUNT_NOT_CREDITED"
                }
                ReasonCode::GoodsServicesCreditNotProcessed => {
                    "GOODS_SERVICES_CREDIT_NOT_PROCESSED"
                }
                ReasonCode::MerchantNotReceivedConfirmation => {
                    "MERCHANT_NOT_RECEIVED_CONFIRMATION"
                }
                ReasonCode::PaidByAlternateMeans => "PAID_BY_ALTERNATE_MEANS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CUSTOMER_ACCOUNT_NOT_REVERSED" => Some(Self::CustomerAccountNotReversed),
                "GOODS_SERVICES_NOT_PROVIDED" => Some(Self::GoodsServicesNotProvided),
                "CUSTOMER_ACCOUNT_NOT_CREDITED_BACK" => {
                    Some(Self::CustomerAccountNotCreditedBack)
                }
                "BENEFICIARY_ACCOUNT_NOT_CREDITED" => {
                    Some(Self::BeneficiaryAccountNotCredited)
                }
                "GOODS_SERVICES_CREDIT_NOT_PROCESSED" => {
                    Some(Self::GoodsServicesCreditNotProcessed)
                }
                "MERCHANT_NOT_RECEIVED_CONFIRMATION" => {
                    Some(Self::MerchantNotReceivedConfirmation)
                }
                "PAID_BY_ALTERNATE_MEANS" => Some(Self::PaidByAlternateMeans),
                _ => None,
            }
        }
    }
}
/// The adjusment flag and reason code for resolving the complaint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveComplaintAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This
    /// maps to `reqAdjFlag` in complaint request and `respAdjFlag` in complaint
    /// response.
    #[prost(enumeration = "resolve_complaint_adjustment::AdjustmentFlag", tag = "1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This
    /// maps to `reqAdjCode` in complaint request.
    #[prost(enumeration = "resolve_complaint_adjustment::ReasonCode", tag = "2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `ResolveComplaintAdjustment`.
pub mod resolve_complaint_adjustment {
    /// The adjusment flag for resolving the complaint.
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
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Debit Reversal Confirmation. This flag maps to the `DRC` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        DebitReversalConfirmation = 1,
        /// Return. This flag maps to the `RET` adjustment flag as defined in NPCI's
        /// `UDIR` specification.
        Return = 2,
        /// Refund Reversal Confirmation. This flag maps to the `RRC` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        RefundReversalConfirmation = 3,
        /// Transaction Credit Confirmation. This flag maps to the `TCC` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        TransactionCreditConfirmation = 4,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::DebitReversalConfirmation => {
                    "DEBIT_REVERSAL_CONFIRMATION"
                }
                AdjustmentFlag::Return => "RETURN",
                AdjustmentFlag::RefundReversalConfirmation => {
                    "REFUND_REVERSAL_CONFIRMATION"
                }
                AdjustmentFlag::TransactionCreditConfirmation => {
                    "TRANSACTION_CREDIT_CONFIRMATION"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADJUSTMENT_FLAG_UNSPECIFIED" => Some(Self::Unspecified),
                "DEBIT_REVERSAL_CONFIRMATION" => Some(Self::DebitReversalConfirmation),
                "RETURN" => Some(Self::Return),
                "REFUND_REVERSAL_CONFIRMATION" => Some(Self::RefundReversalConfirmation),
                "TRANSACTION_CREDIT_CONFIRMATION" => {
                    Some(Self::TransactionCreditConfirmation)
                }
                _ => None,
            }
        }
    }
    /// The complaint resolution reason code.
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
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Customer account has been reversed online for DRC dispute or beneficiary
        /// account has been credited online for TCC dispute. This reason code maps
        /// to the `102` reason code as defined in NPCI's `UDIR` specification.
        ComplaintResolvedOnline = 1,
        /// Customer account has been reversed now or manually post reconciliation
        /// for DRC dispute or beneficiary account has been credited now or manually
        /// post reconciliation for TCC dispute. This reason code maps to the `103`
        /// reason code as defined in NPCI's `UDIR` specification.
        ComplaintResolvedNowOrManually = 2,
        /// Online decline response failed. This reason code maps to the
        /// `104` reason code as defined in NPCI's `UDIR` specification.
        OriginalTransactionNotDone = 3,
        /// Account closed. This reason code maps to the `114` reason code for
        /// RET dispute as defined in NPCI's `UDIR` specification.
        RetAccountClosed = 4,
        /// Account does not exist. This reason code maps to the `115` reason code
        /// for RET dispute as defined in NPCI's `UDIR` specification.
        RetAccountDoesNotExist = 5,
        /// Party instructions. This reason code maps to the `116` reason code for
        /// RET dispute as defined in NPCI's `UDIR` specification.
        RetPartyInstructions = 6,
        /// NRI account. This reason code maps to the `117` reason code for RET
        /// dispute as defined in NPCI's `UDIR` specification.
        RetNriAccount = 7,
        /// Credit freezed. This reason code maps to the `118` reason code for RET
        /// dispute as defined in NPCI's `UDIR` specification.
        RetCreditFreezed = 8,
        /// Invalid beneficiary details. This reason code maps to the `119` reason
        /// code for RET dispute as defined in NPCI's `UDIR` specification.
        RetInvalidBeneficiaryDetails = 9,
        /// Any other reason. This reason code maps to the `120` reason code for RET
        /// dispute as defined in NPCI's `UDIR` specification.
        RetAnyOtherReason = 10,
        /// Beneficiary bank unable to credit their customer account.
        /// This reason code maps to the `1094` reason code for RET dispute as
        /// defined in NPCI's `UDIR` specification.
        RetBeneficiaryCannotCredit = 11,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `1065` reason code for Credit
        /// adjustment and RET dispute as defined in NPCI's `UDIR` specification.
        RetMerchantNotReceivedConfirmation = 12,
        /// Customer account has been credited. This reason code maps to the `501`
        /// reason code for Refund reversal confirmation dispute as defined in NPCI's
        /// `UDIR` specification.
        RrcCustomerAccountCredited = 13,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::ComplaintResolvedOnline => "COMPLAINT_RESOLVED_ONLINE",
                ReasonCode::ComplaintResolvedNowOrManually => {
                    "COMPLAINT_RESOLVED_NOW_OR_MANUALLY"
                }
                ReasonCode::OriginalTransactionNotDone => "ORIGINAL_TRANSACTION_NOT_DONE",
                ReasonCode::RetAccountClosed => "RET_ACCOUNT_CLOSED",
                ReasonCode::RetAccountDoesNotExist => "RET_ACCOUNT_DOES_NOT_EXIST",
                ReasonCode::RetPartyInstructions => "RET_PARTY_INSTRUCTIONS",
                ReasonCode::RetNriAccount => "RET_NRI_ACCOUNT",
                ReasonCode::RetCreditFreezed => "RET_CREDIT_FREEZED",
                ReasonCode::RetInvalidBeneficiaryDetails => {
                    "RET_INVALID_BENEFICIARY_DETAILS"
                }
                ReasonCode::RetAnyOtherReason => "RET_ANY_OTHER_REASON",
                ReasonCode::RetBeneficiaryCannotCredit => "RET_BENEFICIARY_CANNOT_CREDIT",
                ReasonCode::RetMerchantNotReceivedConfirmation => {
                    "RET_MERCHANT_NOT_RECEIVED_CONFIRMATION"
                }
                ReasonCode::RrcCustomerAccountCredited => "RRC_CUSTOMER_ACCOUNT_CREDITED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "COMPLAINT_RESOLVED_ONLINE" => Some(Self::ComplaintResolvedOnline),
                "COMPLAINT_RESOLVED_NOW_OR_MANUALLY" => {
                    Some(Self::ComplaintResolvedNowOrManually)
                }
                "ORIGINAL_TRANSACTION_NOT_DONE" => Some(Self::OriginalTransactionNotDone),
                "RET_ACCOUNT_CLOSED" => Some(Self::RetAccountClosed),
                "RET_ACCOUNT_DOES_NOT_EXIST" => Some(Self::RetAccountDoesNotExist),
                "RET_PARTY_INSTRUCTIONS" => Some(Self::RetPartyInstructions),
                "RET_NRI_ACCOUNT" => Some(Self::RetNriAccount),
                "RET_CREDIT_FREEZED" => Some(Self::RetCreditFreezed),
                "RET_INVALID_BENEFICIARY_DETAILS" => {
                    Some(Self::RetInvalidBeneficiaryDetails)
                }
                "RET_ANY_OTHER_REASON" => Some(Self::RetAnyOtherReason),
                "RET_BENEFICIARY_CANNOT_CREDIT" => Some(Self::RetBeneficiaryCannotCredit),
                "RET_MERCHANT_NOT_RECEIVED_CONFIRMATION" => {
                    Some(Self::RetMerchantNotReceivedConfirmation)
                }
                "RRC_CUSTOMER_ACCOUNT_CREDITED" => Some(Self::RrcCustomerAccountCredited),
                _ => None,
            }
        }
    }
}
/// The adjusment flag and reason code for raising dispute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaiseDisputeAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This
    /// maps to `reqAdjFlag` in dispute request and `respAdjFlag` in dispute
    /// response.
    #[prost(enumeration = "raise_dispute_adjustment::AdjustmentFlag", tag = "1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This
    /// maps to `reqAdjCode` in dispute request.
    #[prost(enumeration = "raise_dispute_adjustment::ReasonCode", tag = "2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `RaiseDisputeAdjustment`.
pub mod raise_dispute_adjustment {
    /// The adjusment flag for raising dispute.
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
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Chargeback Raise. This flag maps to the `B` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ChargebackRaise = 1,
        /// Fraud Chargeback Raise. This flag maps to the `FC` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        FraudChargebackRaise = 2,
        /// Wrong Credit Chargeback Raise. This flag maps to the `WC` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        WrongCreditChargebackRaise = 3,
        /// Deferred Chargeback Raise. This flag maps to the `FB` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        DeferredChargebackRaise = 4,
        /// Pre-Arbitration Raise. This flag maps to the `P` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        PreArbitrationRaise = 5,
        /// Deferred Pre-Arbitration Raise. This flag maps to the `FP` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationRaise = 6,
        /// Arbitration Raise. This flag maps to the `AR` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationRaise = 7,
        /// Deferred Arbitration Raise. This flag maps to the `FAR` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        DeferredArbitrationRaise = 8,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::ChargebackRaise => "CHARGEBACK_RAISE",
                AdjustmentFlag::FraudChargebackRaise => "FRAUD_CHARGEBACK_RAISE",
                AdjustmentFlag::WrongCreditChargebackRaise => {
                    "WRONG_CREDIT_CHARGEBACK_RAISE"
                }
                AdjustmentFlag::DeferredChargebackRaise => "DEFERRED_CHARGEBACK_RAISE",
                AdjustmentFlag::PreArbitrationRaise => "PRE_ARBITRATION_RAISE",
                AdjustmentFlag::DeferredPreArbitrationRaise => {
                    "DEFERRED_PRE_ARBITRATION_RAISE"
                }
                AdjustmentFlag::ArbitrationRaise => "ARBITRATION_RAISE",
                AdjustmentFlag::DeferredArbitrationRaise => "DEFERRED_ARBITRATION_RAISE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADJUSTMENT_FLAG_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK_RAISE" => Some(Self::ChargebackRaise),
                "FRAUD_CHARGEBACK_RAISE" => Some(Self::FraudChargebackRaise),
                "WRONG_CREDIT_CHARGEBACK_RAISE" => Some(Self::WrongCreditChargebackRaise),
                "DEFERRED_CHARGEBACK_RAISE" => Some(Self::DeferredChargebackRaise),
                "PRE_ARBITRATION_RAISE" => Some(Self::PreArbitrationRaise),
                "DEFERRED_PRE_ARBITRATION_RAISE" => {
                    Some(Self::DeferredPreArbitrationRaise)
                }
                "ARBITRATION_RAISE" => Some(Self::ArbitrationRaise),
                "DEFERRED_ARBITRATION_RAISE" => Some(Self::DeferredArbitrationRaise),
                _ => None,
            }
        }
    }
    /// The reason for raising dispute.
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
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Remitter account is debited but beneficiary account is not credited.
        /// This reason code maps to the `108` reason code as defined in
        /// NPCI's `UDIR` specification.
        ChargebackRaiseRemitterDebitedBeneficiaryNotCredited = 1,
        /// Remitter bank customer still disputes that beneficiary account is not
        /// credited. This reason code maps to the `109` reason code as defined in
        /// NPCI's `UDIR` specification.
        PreArbitrationRaiseBeneficiaryNotCredited = 2,
        /// TCC has been raised but customer still complaining that beneficiary
        /// account is not credited. This reason code maps to the `121` reason code
        /// as defined in NPCI's `UDIR` specification.
        DeferredChargebackRaiseBeneficiaryNotCredited = 3,
        /// Customer is still complaining for not crediting the beneficiary
        /// customer account. This reason code maps to the `124` reason code as
        /// defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationRaiseBeneficiaryNotCredited = 4,
        /// Customer is complaining even after raising Deferred Chargeback and
        /// Pre-Arbitration on Deferred Chargeback where both have been rejected by
        /// beneficiary bank. This reason code maps to the `127` reason code as
        /// defined in NPCI's `UDIR` specification.
        DeferredArbitrationRaiseDeferredChargebackPreArbitrationRejected = 5,
        /// Chargeback on fraudulent transaction. This reason code maps to the `128`
        /// reason code as defined in NPCI's `UDIR` specification.
        ChargebackOnFraud = 6,
        /// Credit not processed for cancelled or returned goods and services. This
        /// reason code maps to the `1061` reason code as defined in NPCI's `UDIR`
        /// specification.
        GoodsServicesCreditNotProcessed = 7,
        /// Goods and services not as described / defective. This reason code maps to
        /// the `1062` reason code as defined in NPCI's `UDIR` specification.
        GoodsServicesDefective = 8,
        /// Paid by alternate means. This reason code maps to the `1063` reason code
        /// as defined in NPCI's `UDIR` specification.
        PaidByAlternateMeans = 9,
        /// Goods or services not provided / not received. This reason code maps to
        /// the `1064` reason code as defined in NPCI's `UDIR` specification.
        GoodsServicesNotReceived = 10,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `1065` reason code for chargeback
        /// raise and deferred chargeback raise as defined in NPCI's `UDIR`
        /// specification.
        MerchantNotReceivedConfirmation = 11,
        /// Transaction not steeled within the specified timeframes. This reason code
        /// maps to the `1081` reason code as defined in NPCI's `UDIR` specification.
        TransactionNotSteeled = 12,
        /// Duplicate / Multiple transaction. This reason code maps to the `1084`
        /// reason code as defined in NPCI's `UDIR` specification.
        DuplicateTransaction = 13,
        /// Card holder was charged more than the transaction amount.
        /// This reason code maps to the `1085` reason code for Chargeback raise
        /// dispute as defined in NPCI's `UDIR` specification.
        ChargebackCardHolderChargedMore = 14,
        /// Customer is still claiming that services are not delivered. This reason
        /// code maps to the `1097` reason code as defined in NPCI's `UDIR`
        /// specification.
        CustomerClaimingGoodsServicesNotDelivered = 15,
        /// Both the parties denied to agree. This reason code maps to the `1100`
        /// reason code as defined in NPCI's `UDIR` specification.
        PartiesDenied = 16,
        /// Customer transferred funds to the unintended beneficiary account. This
        /// reason code maps to the `WC1` reason code as defined in NPCI's `UDIR`
        /// specification.
        FundsTransferredToUnintendedBeneficiary = 17,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::ChargebackRaiseRemitterDebitedBeneficiaryNotCredited => {
                    "CHARGEBACK_RAISE_REMITTER_DEBITED_BENEFICIARY_NOT_CREDITED"
                }
                ReasonCode::PreArbitrationRaiseBeneficiaryNotCredited => {
                    "PRE_ARBITRATION_RAISE_BENEFICIARY_NOT_CREDITED"
                }
                ReasonCode::DeferredChargebackRaiseBeneficiaryNotCredited => {
                    "DEFERRED_CHARGEBACK_RAISE_BENEFICIARY_NOT_CREDITED"
                }
                ReasonCode::DeferredPreArbitrationRaiseBeneficiaryNotCredited => {
                    "DEFERRED_PRE_ARBITRATION_RAISE_BENEFICIARY_NOT_CREDITED"
                }
                ReasonCode::DeferredArbitrationRaiseDeferredChargebackPreArbitrationRejected => {
                    "DEFERRED_ARBITRATION_RAISE_DEFERRED_CHARGEBACK_PRE_ARBITRATION_REJECTED"
                }
                ReasonCode::ChargebackOnFraud => "CHARGEBACK_ON_FRAUD",
                ReasonCode::GoodsServicesCreditNotProcessed => {
                    "GOODS_SERVICES_CREDIT_NOT_PROCESSED"
                }
                ReasonCode::GoodsServicesDefective => "GOODS_SERVICES_DEFECTIVE",
                ReasonCode::PaidByAlternateMeans => "PAID_BY_ALTERNATE_MEANS",
                ReasonCode::GoodsServicesNotReceived => "GOODS_SERVICES_NOT_RECEIVED",
                ReasonCode::MerchantNotReceivedConfirmation => {
                    "MERCHANT_NOT_RECEIVED_CONFIRMATION"
                }
                ReasonCode::TransactionNotSteeled => "TRANSACTION_NOT_STEELED",
                ReasonCode::DuplicateTransaction => "DUPLICATE_TRANSACTION",
                ReasonCode::ChargebackCardHolderChargedMore => {
                    "CHARGEBACK_CARD_HOLDER_CHARGED_MORE"
                }
                ReasonCode::CustomerClaimingGoodsServicesNotDelivered => {
                    "CUSTOMER_CLAIMING_GOODS_SERVICES_NOT_DELIVERED"
                }
                ReasonCode::PartiesDenied => "PARTIES_DENIED",
                ReasonCode::FundsTransferredToUnintendedBeneficiary => {
                    "FUNDS_TRANSFERRED_TO_UNINTENDED_BENEFICIARY"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK_RAISE_REMITTER_DEBITED_BENEFICIARY_NOT_CREDITED" => {
                    Some(Self::ChargebackRaiseRemitterDebitedBeneficiaryNotCredited)
                }
                "PRE_ARBITRATION_RAISE_BENEFICIARY_NOT_CREDITED" => {
                    Some(Self::PreArbitrationRaiseBeneficiaryNotCredited)
                }
                "DEFERRED_CHARGEBACK_RAISE_BENEFICIARY_NOT_CREDITED" => {
                    Some(Self::DeferredChargebackRaiseBeneficiaryNotCredited)
                }
                "DEFERRED_PRE_ARBITRATION_RAISE_BENEFICIARY_NOT_CREDITED" => {
                    Some(Self::DeferredPreArbitrationRaiseBeneficiaryNotCredited)
                }
                "DEFERRED_ARBITRATION_RAISE_DEFERRED_CHARGEBACK_PRE_ARBITRATION_REJECTED" => {
                    Some(
                        Self::DeferredArbitrationRaiseDeferredChargebackPreArbitrationRejected,
                    )
                }
                "CHARGEBACK_ON_FRAUD" => Some(Self::ChargebackOnFraud),
                "GOODS_SERVICES_CREDIT_NOT_PROCESSED" => {
                    Some(Self::GoodsServicesCreditNotProcessed)
                }
                "GOODS_SERVICES_DEFECTIVE" => Some(Self::GoodsServicesDefective),
                "PAID_BY_ALTERNATE_MEANS" => Some(Self::PaidByAlternateMeans),
                "GOODS_SERVICES_NOT_RECEIVED" => Some(Self::GoodsServicesNotReceived),
                "MERCHANT_NOT_RECEIVED_CONFIRMATION" => {
                    Some(Self::MerchantNotReceivedConfirmation)
                }
                "TRANSACTION_NOT_STEELED" => Some(Self::TransactionNotSteeled),
                "DUPLICATE_TRANSACTION" => Some(Self::DuplicateTransaction),
                "CHARGEBACK_CARD_HOLDER_CHARGED_MORE" => {
                    Some(Self::ChargebackCardHolderChargedMore)
                }
                "CUSTOMER_CLAIMING_GOODS_SERVICES_NOT_DELIVERED" => {
                    Some(Self::CustomerClaimingGoodsServicesNotDelivered)
                }
                "PARTIES_DENIED" => Some(Self::PartiesDenied),
                "FUNDS_TRANSFERRED_TO_UNINTENDED_BENEFICIARY" => {
                    Some(Self::FundsTransferredToUnintendedBeneficiary)
                }
                _ => None,
            }
        }
    }
}
/// The adjusment flag and reason code for resolving the dispute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveDisputeAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This
    /// maps to `reqAdjFlag` in dispute request and `respAdjFlag` in dispute
    /// response.
    #[prost(enumeration = "resolve_dispute_adjustment::AdjustmentFlag", tag = "1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This
    /// maps to `reqAdjCode` in dispute request.
    #[prost(enumeration = "resolve_dispute_adjustment::ReasonCode", tag = "2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `ResolveDisputeAdjustment`.
pub mod resolve_dispute_adjustment {
    /// The adjusment flag for resolving the dispute.
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
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Re-presentment Raise. This flag maps to the `R` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        RePresentmentRaise = 1,
        /// Deferred Re-presentment Raise. This flag maps to the `FR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredRePresentmentRaise = 2,
        /// Chargeback Acceptance. This flag maps to the `A` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ChargebackAcceptance = 3,
        /// Deferred Chargeback Acceptance. This flag maps to the `FA` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredChargebackAcceptance = 4,
        /// Pre-Arbitration Acceptance. This flag maps to the `AP` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        PreArbitrationAcceptance = 5,
        /// Deferred Pre-Arbitration Acceptance. This flag maps to the `FAP`
        /// adjustment flag as defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationAcceptance = 6,
        /// Pre-Arbitration Declined. This flag maps to the `PR` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        PreArbitrationDeclined = 7,
        /// Deferred Pre-Arbitration Declined. This flag maps to the `FPR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationDeclined = 8,
        /// Arbitration Acceptance. This flag maps to the `ACA` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationAcceptance = 9,
        /// Arbitration Continuation. This flag maps to the `ACC` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationContinuation = 10,
        /// Arbitration Withdrawn. This flag maps to the `ACW` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationWithdrawn = 11,
        /// Arbitration Verdict. This flag maps to the `ACV` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationVerdict = 12,
        /// Credit Adjustment. This flag maps to the `C` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        CreditAdjustment = 13,
        /// Fraud Chargeback Representment. This flag maps to the `FCR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        FraudChargebackRepresentment = 14,
        /// Fraud Chargeback Accept. This flag maps to the `FCA` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        FraudChargebackAccept = 15,
        /// Wrong Credit Representment. This flag maps to the `WR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        WrongCreditRepresentment = 16,
        /// Wrong Credit Chargeback Acceptance. This flag maps to the `WA` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        WrongCreditChargebackAcceptance = 17,
        /// Manual Adjustment. This flag maps to the `MA` adjustment flag as defined
        /// in NPCI's `UDIR` specification.
        ManualAdjustment = 18,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::RePresentmentRaise => "RE_PRESENTMENT_RAISE",
                AdjustmentFlag::DeferredRePresentmentRaise => {
                    "DEFERRED_RE_PRESENTMENT_RAISE"
                }
                AdjustmentFlag::ChargebackAcceptance => "CHARGEBACK_ACCEPTANCE",
                AdjustmentFlag::DeferredChargebackAcceptance => {
                    "DEFERRED_CHARGEBACK_ACCEPTANCE"
                }
                AdjustmentFlag::PreArbitrationAcceptance => "PRE_ARBITRATION_ACCEPTANCE",
                AdjustmentFlag::DeferredPreArbitrationAcceptance => {
                    "DEFERRED_PRE_ARBITRATION_ACCEPTANCE"
                }
                AdjustmentFlag::PreArbitrationDeclined => "PRE_ARBITRATION_DECLINED",
                AdjustmentFlag::DeferredPreArbitrationDeclined => {
                    "DEFERRED_PRE_ARBITRATION_DECLINED"
                }
                AdjustmentFlag::ArbitrationAcceptance => "ARBITRATION_ACCEPTANCE",
                AdjustmentFlag::ArbitrationContinuation => "ARBITRATION_CONTINUATION",
                AdjustmentFlag::ArbitrationWithdrawn => "ARBITRATION_WITHDRAWN",
                AdjustmentFlag::ArbitrationVerdict => "ARBITRATION_VERDICT",
                AdjustmentFlag::CreditAdjustment => "CREDIT_ADJUSTMENT",
                AdjustmentFlag::FraudChargebackRepresentment => {
                    "FRAUD_CHARGEBACK_REPRESENTMENT"
                }
                AdjustmentFlag::FraudChargebackAccept => "FRAUD_CHARGEBACK_ACCEPT",
                AdjustmentFlag::WrongCreditRepresentment => "WRONG_CREDIT_REPRESENTMENT",
                AdjustmentFlag::WrongCreditChargebackAcceptance => {
                    "WRONG_CREDIT_CHARGEBACK_ACCEPTANCE"
                }
                AdjustmentFlag::ManualAdjustment => "MANUAL_ADJUSTMENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADJUSTMENT_FLAG_UNSPECIFIED" => Some(Self::Unspecified),
                "RE_PRESENTMENT_RAISE" => Some(Self::RePresentmentRaise),
                "DEFERRED_RE_PRESENTMENT_RAISE" => Some(Self::DeferredRePresentmentRaise),
                "CHARGEBACK_ACCEPTANCE" => Some(Self::ChargebackAcceptance),
                "DEFERRED_CHARGEBACK_ACCEPTANCE" => {
                    Some(Self::DeferredChargebackAcceptance)
                }
                "PRE_ARBITRATION_ACCEPTANCE" => Some(Self::PreArbitrationAcceptance),
                "DEFERRED_PRE_ARBITRATION_ACCEPTANCE" => {
                    Some(Self::DeferredPreArbitrationAcceptance)
                }
                "PRE_ARBITRATION_DECLINED" => Some(Self::PreArbitrationDeclined),
                "DEFERRED_PRE_ARBITRATION_DECLINED" => {
                    Some(Self::DeferredPreArbitrationDeclined)
                }
                "ARBITRATION_ACCEPTANCE" => Some(Self::ArbitrationAcceptance),
                "ARBITRATION_CONTINUATION" => Some(Self::ArbitrationContinuation),
                "ARBITRATION_WITHDRAWN" => Some(Self::ArbitrationWithdrawn),
                "ARBITRATION_VERDICT" => Some(Self::ArbitrationVerdict),
                "CREDIT_ADJUSTMENT" => Some(Self::CreditAdjustment),
                "FRAUD_CHARGEBACK_REPRESENTMENT" => {
                    Some(Self::FraudChargebackRepresentment)
                }
                "FRAUD_CHARGEBACK_ACCEPT" => Some(Self::FraudChargebackAccept),
                "WRONG_CREDIT_REPRESENTMENT" => Some(Self::WrongCreditRepresentment),
                "WRONG_CREDIT_CHARGEBACK_ACCEPTANCE" => {
                    Some(Self::WrongCreditChargebackAcceptance)
                }
                "MANUAL_ADJUSTMENT" => Some(Self::ManualAdjustment),
                _ => None,
            }
        }
    }
    /// The dispute resolution reason code.
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
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Beneficiary bank unable to credit their customer account for Chargeback
        /// Acceptance dispute or duplicate processing for Pre Arbitration Acceptance
        /// dispute. This reason code maps to the `111` reason code as defined in
        /// NPCI's `UDIR` specification.
        ChargebackBeneficiaryCannotCreditOrPreArbitrationDuplicateProcess = 1,
        /// Beneficiary account has been credited online. This reason code maps to
        /// the `112` reason code for Pre-arbitration declined dispute as defined in
        /// NPCI's `UDIR` specification.
        PreArbitrationDeclinedBeneficiaryCreditedOnline = 3,
        /// Beneficiary account has been credited manually post reconciliation. This
        /// reason code maps to the `113` reason code for Pre-arbitration declined
        /// dispute as defined in NPCI's `UDIR` specification.
        PreArbitrationDeclinedBeneficiaryCreditedManually = 4,
        /// Customer account is not credited, TCC raised inadvertently. This reason
        /// code maps to the `122` reason code as defined in NPCI's `UDIR`
        /// specification.
        DeferredChargebackAcceptanceAccountNotCreditedTccRaised = 5,
        /// Customer account is credited successfully and TCC raised accordingly.
        /// This reason code maps to the `123` reason code as defined in NPCI's
        /// `UDIR` specification.
        DeferredRePresentmentRaiseAccountCreditedTccRaised = 6,
        /// Customer account is not credited, TCC and Re-Presentment raised
        /// inadvertently. This reason code maps to the `125` reason code as defined
        /// in NPCI's `UDIR` specification.
        DeferredPreArbitrationAcceptanceAccountNotCredited = 7,
        /// Customer account is credited successfully and TCC and Re-Presentment
        /// raised accordingly. This reason code maps to the `126` reason code as
        /// defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationDeclinedAccountCredited = 8,
        /// Amount has been recovered successfully from the fraudulent customer
        /// account. This reason code maps to the `129` reason code as defined
        /// in NPCI's `UDIR` specification.
        FraudChargebackAcceptAmountRecoveredFromFraudulentAccount = 9,
        /// Lien marked however, customer account is not having sufficient balance to
        /// debit. This reason code maps to the `130` reason code for
        /// Fraud chargeback representment dispute as defined in NPCI's `UDIR`
        /// specification.
        FraudChargebackRepresentmentLienMarkedInsufficientBalance = 10,
        /// FIR Copy not provided for the disputed transaction. This reason code maps
        /// to the `131` reason code as defined in NPCI's `UDIR` specification.
        FraudChargebackRepresentmentFirNotProvided = 11,
        /// Other reason for Fraud chargeback representment dispute. This reason code
        /// maps to the `132` reason code as defined in NPCI's `UDIR` specification.
        FraudChargebackRepresentmentReasonOthers = 12,
        /// Beneficiary account credited online. This reason code maps to the `208`
        /// reason code for Re-presentment raise dispute as defined in NPCI's `UDIR`
        /// specification.
        RePresentmentRaiseBeneficiaryCreditedOnline = 13,
        /// Beneficiary account credited manually post reconciliation. This reason
        /// code maps to the `209` reason code for Re-presentment raise dispute as
        /// defined in NPCI's `UDIR` specification.
        RePresentmentRaiseBeneficiaryCreditedManually = 14,
        /// Credit not processed for cancelled or returned goods and services. This
        /// reason code maps to the `1061` reason code as defined in NPCI's `UDIR`
        /// specification.
        CreditAdjustmentGoodsServicesCreditNotProcessed = 15,
        /// Goods and Services not as described / defective. This reason code maps to
        /// the `1062` reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentGoodsServicesDefective = 16,
        /// Paid by alternate means. This reason code maps to the `1063` reason code
        /// as defined in NPCI's `UDIR` specification.
        CreditAdjustmentPaidByAlternateMeans = 17,
        /// Goods or Services Not Provided / Not Received. This reason code maps to
        /// the `1064` reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentGoodsServicesNotReceived = 18,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `1065` reason code for Credit
        /// adjustment as defined in NPCI's `UDIR` specification.
        CreditAdjustmentMerchantNotReceivedConfirmation = 19,
        /// Duplicate /Multiple Transaction. This reason code maps to the `1084`
        /// reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentDuplicateTransaction = 20,
        /// Other reason for Credit adjustment. This reason code maps to the `1090`
        /// reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentReasonOthers = 21,
        /// Non Matching account number. This reason code maps to the `1091`
        /// reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentNonMatchingAccountNumber = 22,
        /// Card holder was charged more than the transaction amount.
        /// This reason code maps to the `1092` reason code as defined in NPCI's
        /// `UDIR` specification.
        CreditAdjustmentCardHolderChargedMore = 23,
        /// Credit not Processed. This reason code maps to the `1093` reason code as
        /// defined in NPCI's `UDIR` specification.
        CreditAdjustmentCreditNotProcessed = 24,
        /// Beneficiary bank unable to credit their customer account. This reason
        /// code maps to the `1094` reason code for Credit Adjustment dispute as
        /// defined in NPCI's `UDIR` specification.
        CreditAdjustmentBeneficiaryCannotCredit = 25,
        /// Merchant was unable to provide the service. This reason code maps to the
        /// `1095` reason code as defined in NPCI's `UDIR` specification.
        ChargebackAcceptanceMerchantCannotProvideService = 26,
        /// Services/Goods provided see the supporting document. This reason code
        /// maps to the `1096` reason code as defined in NPCI's `UDIR` specification.
        RePresentmentRaiseGoodsServicesProvided = 27,
        /// Services provided later see supporting documents. This reason code maps
        /// to the `1098` reason code as defined in NPCI's `UDIR` specification.
        PreArbitrationDeclinedServicesProvidedLater = 28,
        /// Services not provided by the merchant. This reason code maps to the
        /// `1099` reason code as defined in NPCI's `UDIR` specification.
        PreArbitrationAcceptanceServicesNotProvidedByMerchant = 29,
        /// Illegible Fulfilment. This reason code maps to the `1101` reason code for
        /// arbitration acceptance dispute as defined in NPCI's `UDIR` specification.
        ArbitrationAcceptanceIllegibleFulfilment = 30,
        /// Customer has still not received the service. This reason code maps to the
        /// `1102` reason code as defined in NPCI's `UDIR` specification.
        ArbitrationContinuationCustomerStillNotReceivedService = 31,
        /// Customer has received the service later. This reason code maps to the
        /// `1103` reason code as defined in NPCI's `UDIR` specification.
        ArbitrationWithdrawnCustomerReceivedServiceLater = 32,
        /// Panel will give the verdict. This reason code maps to the `1104` reason
        /// code as defined in NPCI's `UDIR` specification.
        ArbitrationVerdictPanelVerdict = 33,
        /// Manual adjustment. This reason code maps to the `2001` reason code as
        /// defined in NPCI's `UDIR` specification.
        ManualAdjustmentReason = 34,
        /// Attributing to the Customer. This reason code maps to the `AC` reason
        /// code as defined in NPCI's `UDIR` specification.
        AttributingCustomer = 35,
        /// Attributing to the Technical issue at bank/aggregator/merchant. This
        /// reason code maps to the `AT` reason code as defined in NPCI's `UDIR`
        /// specification.
        AttributingTechnicalIssue = 36,
        /// Amount has been recovered successfully from the unintended customer
        /// account. This reason code maps to the `WC2` reason code as defined in
        /// NPCI's `UDIR` specification.
        WrongCreditChargebackAcceptanceAmountRecovered = 37,
        /// Lien marked however customer account is not having sufficient balance to
        /// debit the customer account. This reason code maps to the `WC3` reason
        /// code for Wrong credit representment dispute as defined in NPCI's `UDIR`
        /// specification.
        WrongCreditRepresentmentLienMarkedInsufficientBalance = 38,
        /// Customer is not accessible for obtaining debit confirmation. This reason
        /// code maps to the `WC4` reason code as defined in NPCI's `UDIR`
        /// specification.
        WrongCreditRepresentmentCustomerInaccessible = 39,
        /// Other reason for Wrong credit representment. This reason code maps to the
        /// `WC5` reason code as defined in NPCI's `UDIR` specification.
        WrongCreditRepresentmentReasonOthers = 40,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::ChargebackBeneficiaryCannotCreditOrPreArbitrationDuplicateProcess => {
                    "CHARGEBACK_BENEFICIARY_CANNOT_CREDIT_OR_PRE_ARBITRATION_DUPLICATE_PROCESS"
                }
                ReasonCode::PreArbitrationDeclinedBeneficiaryCreditedOnline => {
                    "PRE_ARBITRATION_DECLINED_BENEFICIARY_CREDITED_ONLINE"
                }
                ReasonCode::PreArbitrationDeclinedBeneficiaryCreditedManually => {
                    "PRE_ARBITRATION_DECLINED_BENEFICIARY_CREDITED_MANUALLY"
                }
                ReasonCode::DeferredChargebackAcceptanceAccountNotCreditedTccRaised => {
                    "DEFERRED_CHARGEBACK_ACCEPTANCE_ACCOUNT_NOT_CREDITED_TCC_RAISED"
                }
                ReasonCode::DeferredRePresentmentRaiseAccountCreditedTccRaised => {
                    "DEFERRED_RE_PRESENTMENT_RAISE_ACCOUNT_CREDITED_TCC_RAISED"
                }
                ReasonCode::DeferredPreArbitrationAcceptanceAccountNotCredited => {
                    "DEFERRED_PRE_ARBITRATION_ACCEPTANCE_ACCOUNT_NOT_CREDITED"
                }
                ReasonCode::DeferredPreArbitrationDeclinedAccountCredited => {
                    "DEFERRED_PRE_ARBITRATION_DECLINED_ACCOUNT_CREDITED"
                }
                ReasonCode::FraudChargebackAcceptAmountRecoveredFromFraudulentAccount => {
                    "FRAUD_CHARGEBACK_ACCEPT_AMOUNT_RECOVERED_FROM_FRAUDULENT_ACCOUNT"
                }
                ReasonCode::FraudChargebackRepresentmentLienMarkedInsufficientBalance => {
                    "FRAUD_CHARGEBACK_REPRESENTMENT_LIEN_MARKED_INSUFFICIENT_BALANCE"
                }
                ReasonCode::FraudChargebackRepresentmentFirNotProvided => {
                    "FRAUD_CHARGEBACK_REPRESENTMENT_FIR_NOT_PROVIDED"
                }
                ReasonCode::FraudChargebackRepresentmentReasonOthers => {
                    "FRAUD_CHARGEBACK_REPRESENTMENT_REASON_OTHERS"
                }
                ReasonCode::RePresentmentRaiseBeneficiaryCreditedOnline => {
                    "RE_PRESENTMENT_RAISE_BENEFICIARY_CREDITED_ONLINE"
                }
                ReasonCode::RePresentmentRaiseBeneficiaryCreditedManually => {
                    "RE_PRESENTMENT_RAISE_BENEFICIARY_CREDITED_MANUALLY"
                }
                ReasonCode::CreditAdjustmentGoodsServicesCreditNotProcessed => {
                    "CREDIT_ADJUSTMENT_GOODS_SERVICES_CREDIT_NOT_PROCESSED"
                }
                ReasonCode::CreditAdjustmentGoodsServicesDefective => {
                    "CREDIT_ADJUSTMENT_GOODS_SERVICES_DEFECTIVE"
                }
                ReasonCode::CreditAdjustmentPaidByAlternateMeans => {
                    "CREDIT_ADJUSTMENT_PAID_BY_ALTERNATE_MEANS"
                }
                ReasonCode::CreditAdjustmentGoodsServicesNotReceived => {
                    "CREDIT_ADJUSTMENT_GOODS_SERVICES_NOT_RECEIVED"
                }
                ReasonCode::CreditAdjustmentMerchantNotReceivedConfirmation => {
                    "CREDIT_ADJUSTMENT_MERCHANT_NOT_RECEIVED_CONFIRMATION"
                }
                ReasonCode::CreditAdjustmentDuplicateTransaction => {
                    "CREDIT_ADJUSTMENT_DUPLICATE_TRANSACTION"
                }
                ReasonCode::CreditAdjustmentReasonOthers => {
                    "CREDIT_ADJUSTMENT_REASON_OTHERS"
                }
                ReasonCode::CreditAdjustmentNonMatchingAccountNumber => {
                    "CREDIT_ADJUSTMENT_NON_MATCHING_ACCOUNT_NUMBER"
                }
                ReasonCode::CreditAdjustmentCardHolderChargedMore => {
                    "CREDIT_ADJUSTMENT_CARD_HOLDER_CHARGED_MORE"
                }
                ReasonCode::CreditAdjustmentCreditNotProcessed => {
                    "CREDIT_ADJUSTMENT_CREDIT_NOT_PROCESSED"
                }
                ReasonCode::CreditAdjustmentBeneficiaryCannotCredit => {
                    "CREDIT_ADJUSTMENT_BENEFICIARY_CANNOT_CREDIT"
                }
                ReasonCode::ChargebackAcceptanceMerchantCannotProvideService => {
                    "CHARGEBACK_ACCEPTANCE_MERCHANT_CANNOT_PROVIDE_SERVICE"
                }
                ReasonCode::RePresentmentRaiseGoodsServicesProvided => {
                    "RE_PRESENTMENT_RAISE_GOODS_SERVICES_PROVIDED"
                }
                ReasonCode::PreArbitrationDeclinedServicesProvidedLater => {
                    "PRE_ARBITRATION_DECLINED_SERVICES_PROVIDED_LATER"
                }
                ReasonCode::PreArbitrationAcceptanceServicesNotProvidedByMerchant => {
                    "PRE_ARBITRATION_ACCEPTANCE_SERVICES_NOT_PROVIDED_BY_MERCHANT"
                }
                ReasonCode::ArbitrationAcceptanceIllegibleFulfilment => {
                    "ARBITRATION_ACCEPTANCE_ILLEGIBLE_FULFILMENT"
                }
                ReasonCode::ArbitrationContinuationCustomerStillNotReceivedService => {
                    "ARBITRATION_CONTINUATION_CUSTOMER_STILL_NOT_RECEIVED_SERVICE"
                }
                ReasonCode::ArbitrationWithdrawnCustomerReceivedServiceLater => {
                    "ARBITRATION_WITHDRAWN_CUSTOMER_RECEIVED_SERVICE_LATER"
                }
                ReasonCode::ArbitrationVerdictPanelVerdict => {
                    "ARBITRATION_VERDICT_PANEL_VERDICT"
                }
                ReasonCode::ManualAdjustmentReason => "MANUAL_ADJUSTMENT_REASON",
                ReasonCode::AttributingCustomer => "ATTRIBUTING_CUSTOMER",
                ReasonCode::AttributingTechnicalIssue => "ATTRIBUTING_TECHNICAL_ISSUE",
                ReasonCode::WrongCreditChargebackAcceptanceAmountRecovered => {
                    "WRONG_CREDIT_CHARGEBACK_ACCEPTANCE_AMOUNT_RECOVERED"
                }
                ReasonCode::WrongCreditRepresentmentLienMarkedInsufficientBalance => {
                    "WRONG_CREDIT_REPRESENTMENT_LIEN_MARKED_INSUFFICIENT_BALANCE"
                }
                ReasonCode::WrongCreditRepresentmentCustomerInaccessible => {
                    "WRONG_CREDIT_REPRESENTMENT_CUSTOMER_INACCESSIBLE"
                }
                ReasonCode::WrongCreditRepresentmentReasonOthers => {
                    "WRONG_CREDIT_REPRESENTMENT_REASON_OTHERS"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK_BENEFICIARY_CANNOT_CREDIT_OR_PRE_ARBITRATION_DUPLICATE_PROCESS" => {
                    Some(
                        Self::ChargebackBeneficiaryCannotCreditOrPreArbitrationDuplicateProcess,
                    )
                }
                "PRE_ARBITRATION_DECLINED_BENEFICIARY_CREDITED_ONLINE" => {
                    Some(Self::PreArbitrationDeclinedBeneficiaryCreditedOnline)
                }
                "PRE_ARBITRATION_DECLINED_BENEFICIARY_CREDITED_MANUALLY" => {
                    Some(Self::PreArbitrationDeclinedBeneficiaryCreditedManually)
                }
                "DEFERRED_CHARGEBACK_ACCEPTANCE_ACCOUNT_NOT_CREDITED_TCC_RAISED" => {
                    Some(Self::DeferredChargebackAcceptanceAccountNotCreditedTccRaised)
                }
                "DEFERRED_RE_PRESENTMENT_RAISE_ACCOUNT_CREDITED_TCC_RAISED" => {
                    Some(Self::DeferredRePresentmentRaiseAccountCreditedTccRaised)
                }
                "DEFERRED_PRE_ARBITRATION_ACCEPTANCE_ACCOUNT_NOT_CREDITED" => {
                    Some(Self::DeferredPreArbitrationAcceptanceAccountNotCredited)
                }
                "DEFERRED_PRE_ARBITRATION_DECLINED_ACCOUNT_CREDITED" => {
                    Some(Self::DeferredPreArbitrationDeclinedAccountCredited)
                }
                "FRAUD_CHARGEBACK_ACCEPT_AMOUNT_RECOVERED_FROM_FRAUDULENT_ACCOUNT" => {
                    Some(Self::FraudChargebackAcceptAmountRecoveredFromFraudulentAccount)
                }
                "FRAUD_CHARGEBACK_REPRESENTMENT_LIEN_MARKED_INSUFFICIENT_BALANCE" => {
                    Some(Self::FraudChargebackRepresentmentLienMarkedInsufficientBalance)
                }
                "FRAUD_CHARGEBACK_REPRESENTMENT_FIR_NOT_PROVIDED" => {
                    Some(Self::FraudChargebackRepresentmentFirNotProvided)
                }
                "FRAUD_CHARGEBACK_REPRESENTMENT_REASON_OTHERS" => {
                    Some(Self::FraudChargebackRepresentmentReasonOthers)
                }
                "RE_PRESENTMENT_RAISE_BENEFICIARY_CREDITED_ONLINE" => {
                    Some(Self::RePresentmentRaiseBeneficiaryCreditedOnline)
                }
                "RE_PRESENTMENT_RAISE_BENEFICIARY_CREDITED_MANUALLY" => {
                    Some(Self::RePresentmentRaiseBeneficiaryCreditedManually)
                }
                "CREDIT_ADJUSTMENT_GOODS_SERVICES_CREDIT_NOT_PROCESSED" => {
                    Some(Self::CreditAdjustmentGoodsServicesCreditNotProcessed)
                }
                "CREDIT_ADJUSTMENT_GOODS_SERVICES_DEFECTIVE" => {
                    Some(Self::CreditAdjustmentGoodsServicesDefective)
                }
                "CREDIT_ADJUSTMENT_PAID_BY_ALTERNATE_MEANS" => {
                    Some(Self::CreditAdjustmentPaidByAlternateMeans)
                }
                "CREDIT_ADJUSTMENT_GOODS_SERVICES_NOT_RECEIVED" => {
                    Some(Self::CreditAdjustmentGoodsServicesNotReceived)
                }
                "CREDIT_ADJUSTMENT_MERCHANT_NOT_RECEIVED_CONFIRMATION" => {
                    Some(Self::CreditAdjustmentMerchantNotReceivedConfirmation)
                }
                "CREDIT_ADJUSTMENT_DUPLICATE_TRANSACTION" => {
                    Some(Self::CreditAdjustmentDuplicateTransaction)
                }
                "CREDIT_ADJUSTMENT_REASON_OTHERS" => {
                    Some(Self::CreditAdjustmentReasonOthers)
                }
                "CREDIT_ADJUSTMENT_NON_MATCHING_ACCOUNT_NUMBER" => {
                    Some(Self::CreditAdjustmentNonMatchingAccountNumber)
                }
                "CREDIT_ADJUSTMENT_CARD_HOLDER_CHARGED_MORE" => {
                    Some(Self::CreditAdjustmentCardHolderChargedMore)
                }
                "CREDIT_ADJUSTMENT_CREDIT_NOT_PROCESSED" => {
                    Some(Self::CreditAdjustmentCreditNotProcessed)
                }
                "CREDIT_ADJUSTMENT_BENEFICIARY_CANNOT_CREDIT" => {
                    Some(Self::CreditAdjustmentBeneficiaryCannotCredit)
                }
                "CHARGEBACK_ACCEPTANCE_MERCHANT_CANNOT_PROVIDE_SERVICE" => {
                    Some(Self::ChargebackAcceptanceMerchantCannotProvideService)
                }
                "RE_PRESENTMENT_RAISE_GOODS_SERVICES_PROVIDED" => {
                    Some(Self::RePresentmentRaiseGoodsServicesProvided)
                }
                "PRE_ARBITRATION_DECLINED_SERVICES_PROVIDED_LATER" => {
                    Some(Self::PreArbitrationDeclinedServicesProvidedLater)
                }
                "PRE_ARBITRATION_ACCEPTANCE_SERVICES_NOT_PROVIDED_BY_MERCHANT" => {
                    Some(Self::PreArbitrationAcceptanceServicesNotProvidedByMerchant)
                }
                "ARBITRATION_ACCEPTANCE_ILLEGIBLE_FULFILMENT" => {
                    Some(Self::ArbitrationAcceptanceIllegibleFulfilment)
                }
                "ARBITRATION_CONTINUATION_CUSTOMER_STILL_NOT_RECEIVED_SERVICE" => {
                    Some(Self::ArbitrationContinuationCustomerStillNotReceivedService)
                }
                "ARBITRATION_WITHDRAWN_CUSTOMER_RECEIVED_SERVICE_LATER" => {
                    Some(Self::ArbitrationWithdrawnCustomerReceivedServiceLater)
                }
                "ARBITRATION_VERDICT_PANEL_VERDICT" => {
                    Some(Self::ArbitrationVerdictPanelVerdict)
                }
                "MANUAL_ADJUSTMENT_REASON" => Some(Self::ManualAdjustmentReason),
                "ATTRIBUTING_CUSTOMER" => Some(Self::AttributingCustomer),
                "ATTRIBUTING_TECHNICAL_ISSUE" => Some(Self::AttributingTechnicalIssue),
                "WRONG_CREDIT_CHARGEBACK_ACCEPTANCE_AMOUNT_RECOVERED" => {
                    Some(Self::WrongCreditChargebackAcceptanceAmountRecovered)
                }
                "WRONG_CREDIT_REPRESENTMENT_LIEN_MARKED_INSUFFICIENT_BALANCE" => {
                    Some(Self::WrongCreditRepresentmentLienMarkedInsufficientBalance)
                }
                "WRONG_CREDIT_REPRESENTMENT_CUSTOMER_INACCESSIBLE" => {
                    Some(Self::WrongCreditRepresentmentCustomerInaccessible)
                }
                "WRONG_CREDIT_REPRESENTMENT_REASON_OTHERS" => {
                    Some(Self::WrongCreditRepresentmentReasonOthers)
                }
                _ => None,
            }
        }
    }
}
/// Metadata for CreateComplaint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateComplaintMetadata {}
/// Metadata for ResolveComplaint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveComplaintMetadata {}
/// Metadata for CreateDispute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisputeMetadata {}
/// Metadata for ResolveDispute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveDisputeMetadata {}
/// The subtype of the complaint or dispute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionSubType {
    /// Unspecified transaction subtype.
    Unspecified = 0,
    /// Beneficiary transaction subtype.
    Beneficiary = 1,
    /// Remitter transaction subtype.
    Remitter = 2,
}
impl TransactionSubType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionSubType::Unspecified => "TRANSACTION_SUB_TYPE_UNSPECIFIED",
            TransactionSubType::Beneficiary => "TRANSACTION_SUB_TYPE_BENEFICIARY",
            TransactionSubType::Remitter => "TRANSACTION_SUB_TYPE_REMITTER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSACTION_SUB_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRANSACTION_SUB_TYPE_BENEFICIARY" => Some(Self::Beneficiary),
            "TRANSACTION_SUB_TYPE_REMITTER" => Some(Self::Remitter),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod issuer_switch_resolutions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Creates and resolves UPI complaints and disputes.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchResolutionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IssuerSwitchResolutionsClient<T>
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
        ) -> IssuerSwitchResolutionsClient<InterceptedService<T, F>>
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
            IssuerSwitchResolutionsClient::new(
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
        /// Create a complaint. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [CreateComplaintMetadata][google.cloud.paymentgateway.issuerswitch.v1.CreateComplaintMetadata]
        /// - `response`:
        /// [Complaint][google.cloud.paymentgateway.issuerswitch.v1.Complaint]
        pub async fn create_complaint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateComplaintRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/CreateComplaint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions",
                        "CreateComplaint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resolve a complaint. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [ResolveComplaintMetadata][google.cloud.paymentgateway.issuerswitch.v1.ResolveComplaintMetadata]
        /// - `response`:
        /// [Complaint][google.cloud.paymentgateway.issuerswitch.v1.Complaint]
        pub async fn resolve_complaint(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveComplaintRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/ResolveComplaint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions",
                        "ResolveComplaint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Create a dispute. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [CreateDisputeMetadata][google.cloud.paymentgateway.issuerswitch.v1.CreateDisputeMetadata]
        /// - `response`:
        /// [Dispute][google.cloud.paymentgateway.issuerswitch.v1.Dispute]
        pub async fn create_dispute(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDisputeRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/CreateDispute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions",
                        "CreateDispute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Resolve a dispute. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [ResolveDisputeMetadata][google.cloud.paymentgateway.issuerswitch.v1.ResolveDisputeMetadata]
        /// - `response`:
        /// [Dispute][google.cloud.paymentgateway.issuerswitch.v1.Dispute]
        pub async fn resolve_dispute(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveDisputeRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/ResolveDispute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions",
                        "ResolveDispute",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Information about a transaction processed by the issuer switch.
/// The fields in this type are common across both financial and metadata
/// transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// Output only. An identifier that is mandatorily present in every transaction
    /// processed via UPI. This maps to UPI's transaction ID.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The API type of the transaction.
    #[prost(enumeration = "ApiType", tag = "2")]
    pub api_type: i32,
    /// Output only. The transaction type.
    #[prost(enumeration = "TransactionType", tag = "3")]
    pub transaction_type: i32,
    /// Output only. The transaction subtype.
    #[prost(enumeration = "transaction_info::TransactionSubType", tag = "4")]
    pub transaction_sub_type: i32,
    /// Output only. The transaction's state.
    #[prost(enumeration = "transaction_info::State", tag = "5")]
    pub state: i32,
    /// Metadata about the API transaction.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::core::option::Option<transaction_info::TransactionMetadata>,
    /// Output only. Any error details for the current API transaction, if the
    /// state is `FAILED`.
    #[prost(message, optional, tag = "7")]
    pub error_details: ::core::option::Option<transaction_info::TransactionErrorDetails>,
    /// Output only. Information about the adapter invocation from the issuer
    /// switch for processing this API transaction.
    #[prost(message, optional, tag = "8")]
    pub adapter_info: ::core::option::Option<transaction_info::AdapterInfo>,
    /// Risk information as provided by the payments orchestrator.
    #[prost(message, repeated, tag = "9")]
    pub risk_info: ::prost::alloc::vec::Vec<transaction_info::TransactionRiskInfo>,
}
/// Nested message and enum types in `TransactionInfo`.
pub mod transaction_info {
    /// Common metadata about an API transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionMetadata {
        /// Output only. The time at which the transaction resource was created by
        /// the issuer switch.
        #[prost(message, optional, tag = "1")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. The time at which the transaction resource was last updated
        /// by the issuer switch.
        #[prost(message, optional, tag = "2")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output only. A reference id for the API transaction.
        #[prost(string, tag = "3")]
        pub reference_id: ::prost::alloc::string::String,
        /// Output only. A reference URI to this API transaction.
        #[prost(string, tag = "4")]
        pub reference_uri: ::prost::alloc::string::String,
        /// Output only. A descriptive note about this API transaction.
        #[prost(string, tag = "5")]
        pub description: ::prost::alloc::string::String,
        /// Output only. The initiation mode of this API transaction. In UPI, the
        /// values are as defined by the UPI API specification.
        #[prost(string, tag = "6")]
        pub initiation_mode: ::prost::alloc::string::String,
        /// Output only. The purpose code of this API transaction. In UPI, the values
        /// are as defined by the UPI API specification.
        #[prost(string, tag = "7")]
        pub purpose_code: ::prost::alloc::string::String,
        /// Output only. The reference category of this API transaction.
        #[prost(string, tag = "8")]
        pub reference_category: ::prost::alloc::string::String,
    }
    /// All details about any error in the processing of an API transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionErrorDetails {
        /// Output only. Error code of the failed transaction.
        #[prost(string, tag = "1")]
        pub error_code: ::prost::alloc::string::String,
        /// Output only. Error description for the failed transaction.
        #[prost(string, tag = "2")]
        pub error_message: ::prost::alloc::string::String,
        /// Output only. Error code as per the UPI specification. The issuer switch
        /// maps the ErrorCode to an appropriate error code that complies with the
        /// UPI specification.
        #[prost(string, tag = "3")]
        pub upi_error_code: ::prost::alloc::string::String,
    }
    /// Information about an adapter invocation triggered as part of the
    /// processing of an API transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdapterInfo {
        /// Output only. List of adapter request IDs (colon separated) used when
        /// invoking the Adapter APIs for fulfilling a transaction request.
        #[prost(string, tag = "1")]
        pub request_ids: ::prost::alloc::string::String,
        /// Output only. Response metadata included by the adapter in its response to
        /// an API invocation from the issuer switch.
        #[prost(message, optional, tag = "2")]
        pub response_metadata: ::core::option::Option<adapter_info::ResponseMetadata>,
    }
    /// Nested message and enum types in `AdapterInfo`.
    pub mod adapter_info {
        /// Metadata about a response that the adapter includes in its response
        /// to the issuer switch.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResponseMetadata {
            /// A map of name-value pairs.
            #[prost(btree_map = "string, string", tag = "1")]
            pub values: ::prost::alloc::collections::BTreeMap<
                ::prost::alloc::string::String,
                ::prost::alloc::string::String,
            >,
        }
    }
    /// Information about the transaction's risk evaluation as provided by the
    /// payments orchestrator.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransactionRiskInfo {
        /// Entity providing the risk score. This could either be the payment service
        /// provider or the payment orchestrator (UPI, etc).
        #[prost(string, tag = "1")]
        pub provider: ::prost::alloc::string::String,
        /// Type of risk. Examples include `TXNRISK`.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Numeric value of risk evaluation ranging from 0 (No Risk) to 100 (Maximum
        /// Risk).
        #[prost(string, tag = "3")]
        pub value: ::prost::alloc::string::String,
    }
    /// Specifies the current state of the transaction.
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
        /// Unspecified state.
        Unspecified = 0,
        /// The transaction has successfully completed.
        Succeeded = 1,
        /// The transaction has failed.
        Failed = 2,
        /// The transaction has timed out.
        TimedOut = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::TimedOut => "TIMED_OUT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "TIMED_OUT" => Some(Self::TimedOut),
                _ => None,
            }
        }
    }
    /// The subtype of a transaction. This value is used only for certain API type
    /// and transaction type combinations.
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
    pub enum TransactionSubType {
        /// Unspecified transaction subtype.
        Unspecified = 0,
        /// Collect subtype. This is used in a `SETTLE_PAYMENT` API type
        /// transaction, with the transaction type as either
        /// `TRANSACTION_TYPE_CREDIT` or `TRANSACTION_TYPE_DEBIT` when the payment
        /// was initiated by a collect request.
        Collect = 1,
        /// Debit subtype. This is used in a `SETTLE_PAYMENT` API type transaction,
        /// with the transaction type as `TRANSACTION_TYPE_REVERSAL` when the
        /// original payment was a debit request.
        Debit = 2,
        /// Pay subtype. This is used in a `SETTLE_PAYMENT` API type transaction,
        /// with the transaction type as either `TRANSACTION_TYPE_CREDIT` or
        /// `TRANSACTION_TYPE_DEBIT` when the payment was initiated by a pay request.
        Pay = 3,
        /// Beneficiary subtype. This is used in a `COMPLAINT` API type transaction,
        /// when the complaint / dispute request is initiated / received by the
        /// beneficiary bank.
        Beneficiary = 4,
        /// Remitter subtype. This is used in a `COMPLAINT` API type transaction,
        /// when the complaint / dispute request is initiated / received by the
        /// remitter bank.
        Remitter = 5,
        /// Refund subtype. This is used in a `SETTLE_PAYMENT` API type transaction,
        /// with the transaction type as `TRANSACTION_TYPE_CREDIT` when the payment
        /// was initiated in response to a refund.
        Refund = 6,
        /// Credit subtype. This is used in a `SETTLE_PAYMENT` API type transaction,
        /// with the transaction type as `TRANSACTION_TYPE_REVERSAL` when the
        /// original payment was a credit request.
        Credit = 7,
    }
    impl TransactionSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionSubType::Unspecified => "TRANSACTION_SUB_TYPE_UNSPECIFIED",
                TransactionSubType::Collect => "COLLECT",
                TransactionSubType::Debit => "DEBIT",
                TransactionSubType::Pay => "PAY",
                TransactionSubType::Beneficiary => "BENEFICIARY",
                TransactionSubType::Remitter => "REMITTER",
                TransactionSubType::Refund => "REFUND",
                TransactionSubType::Credit => "CREDIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRANSACTION_SUB_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "COLLECT" => Some(Self::Collect),
                "DEBIT" => Some(Self::Debit),
                "PAY" => Some(Self::Pay),
                "BENEFICIARY" => Some(Self::Beneficiary),
                "REMITTER" => Some(Self::Remitter),
                "REFUND" => Some(Self::Refund),
                "CREDIT" => Some(Self::Credit),
                _ => None,
            }
        }
    }
}
/// A metadata API transaction processed by the issuer switch. This
/// includes UPI APIs such as List Accounts, Balance Enquiry, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataTransaction {
    /// The name of the metadata transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/metadataTransaction/{metadata_transaction_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<TransactionInfo>,
    /// Output only. The initiator of the metadata transaction.
    #[prost(message, optional, tag = "3")]
    pub initiator: ::core::option::Option<Participant>,
}
/// A financial API transaction processed by the issuer switch. In UPI, this maps
/// to the Pay API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialTransaction {
    /// The name of the financial transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/financialTransactions/{financial_transaction_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<TransactionInfo>,
    /// Output only. A 12 digit numeric code associated with the request. It could
    /// contain leading 0s. In UPI, this is also known as as the customer reference
    /// or the UPI transaction ID.
    #[prost(string, tag = "3")]
    pub retrieval_reference_number: ::prost::alloc::string::String,
    /// Output only. The payer in the transaction.
    #[prost(message, optional, tag = "4")]
    pub payer: ::core::option::Option<SettlementParticipant>,
    /// Output only. The payee in the transaction.
    #[prost(message, optional, tag = "5")]
    pub payee: ::core::option::Option<SettlementParticipant>,
    /// Output only. The amount for payment settlement in the transaction.
    #[prost(message, optional, tag = "6")]
    pub amount: ::core::option::Option<super::super::super::super::r#type::Money>,
    /// A list of rules specified by the payments orchestrator for this API
    /// transaction.
    #[prost(message, repeated, tag = "7")]
    pub payment_rules: ::prost::alloc::vec::Vec<financial_transaction::PaymentRule>,
}
/// Nested message and enum types in `FinancialTransaction`.
pub mod financial_transaction {
    /// A payment rule as provided by the payments orchestrator.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PaymentRule {
        /// The rule's name.
        #[prost(enumeration = "payment_rule::PaymentRuleName", tag = "1")]
        pub payment_rule: i32,
        /// The rule's value.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `PaymentRule`.
    pub mod payment_rule {
        /// An enum of the possible rule names.
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
        pub enum PaymentRuleName {
            /// Rule name unspecified.
            Unspecified = 0,
            /// The `expire after` rule.
            ExpireAfter = 1,
            /// The `min amount` rule.
            MinAmount = 2,
        }
        impl PaymentRuleName {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    PaymentRuleName::Unspecified => "PAYMENT_RULE_NAME_UNSPECIFIED",
                    PaymentRuleName::ExpireAfter => "EXPIRE_AFTER",
                    PaymentRuleName::MinAmount => "MIN_AMOUNT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "PAYMENT_RULE_NAME_UNSPECIFIED" => Some(Self::Unspecified),
                    "EXPIRE_AFTER" => Some(Self::ExpireAfter),
                    "MIN_AMOUNT" => Some(Self::MinAmount),
                    _ => None,
                }
            }
        }
    }
}
/// A mandate processed by the issuer switch. In UPI, this maps to the Mandate
/// API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MandateTransaction {
    /// The name of the mandate transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/mandateTransactions/{mandate_transaction_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag = "2")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
    /// Output only. This maps to Unique Mandate Number (UMN) in UPI specification.
    #[prost(string, tag = "3")]
    pub unique_mandate_number: ::prost::alloc::string::String,
    /// Output only. The payer in the transaction.
    #[prost(message, optional, tag = "4")]
    pub payer: ::core::option::Option<SettlementParticipant>,
    /// Output only. The payee in the transaction.
    #[prost(message, optional, tag = "5")]
    pub payee: ::core::option::Option<SettlementParticipant>,
    /// Output only. The type of recurrence pattern of the mandate.
    #[prost(enumeration = "mandate_transaction::RecurrencePatternType", tag = "6")]
    pub recurrence_pattern: i32,
    /// Output only. The type of recurrence rule of the mandate.
    #[prost(enumeration = "mandate_transaction::RecurrenceRuleType", tag = "7")]
    pub recurrence_rule_type: i32,
    /// Output only. The recurrence rule value of the mandate. This is a value from
    /// 1 to 31.
    #[prost(int32, tag = "8")]
    pub recurrence_rule_value: i32,
    /// Output only. The start date of the mandate.
    #[prost(message, optional, tag = "9")]
    pub start_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Output only. The end date of the mandate.
    #[prost(message, optional, tag = "10")]
    pub end_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Output only. If true, this specifies mandate can be revoked.
    #[prost(bool, tag = "11")]
    pub revokable: bool,
    /// Output only. The amount of the mandate.
    #[prost(double, tag = "12")]
    pub amount: f64,
    /// Output only. The amount rule type of the mandate.
    #[prost(enumeration = "mandate_transaction::AmountRuleType", tag = "13")]
    pub amount_rule: i32,
    /// Output only. The Block funds reference generated by the bank, this will be
    /// available only when Recurrence is ONETIME.
    #[prost(string, tag = "14")]
    pub approval_reference: ::prost::alloc::string::String,
    /// Output only. If true, this specifies the mandate transaction requested
    /// funds to be blocked.
    #[prost(bool, tag = "15")]
    pub block_funds: bool,
    /// Output only. The mandate's name.
    #[prost(string, tag = "16")]
    pub mandate_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MandateTransaction`.
pub mod mandate_transaction {
    /// RecurrencePatternType specifies the recurrence pattern type of the mandate.
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
    pub enum RecurrencePatternType {
        /// Unspecified recurrence pattern.
        Unspecified = 0,
        /// As presented recurrence pattern.
        AsPresented = 1,
        /// Bi monthly recurrence pattern.
        Bimonthly = 2,
        /// Daily recurrence pattern.
        Daily = 3,
        /// Bi weekly recurrence pattern.
        Fortnightly = 4,
        /// Half yearly recurrence pattern.
        HalfYearly = 5,
        /// Monthly recurrence pattern.
        Monthly = 6,
        /// One time recurrence pattern.
        OneTime = 7,
        /// Quarterly recurrence pattern.
        Quarterly = 8,
        /// Weekly recurrence pattern.
        Weekly = 9,
        /// Yearly recurrence pattern.
        Yearly = 10,
    }
    impl RecurrencePatternType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecurrencePatternType::Unspecified => {
                    "RECURRENCE_PATTERN_TYPE_UNSPECIFIED"
                }
                RecurrencePatternType::AsPresented => "AS_PRESENTED",
                RecurrencePatternType::Bimonthly => "BIMONTHLY",
                RecurrencePatternType::Daily => "DAILY",
                RecurrencePatternType::Fortnightly => "FORTNIGHTLY",
                RecurrencePatternType::HalfYearly => "HALF_YEARLY",
                RecurrencePatternType::Monthly => "MONTHLY",
                RecurrencePatternType::OneTime => "ONE_TIME",
                RecurrencePatternType::Quarterly => "QUARTERLY",
                RecurrencePatternType::Weekly => "WEEKLY",
                RecurrencePatternType::Yearly => "YEARLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECURRENCE_PATTERN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AS_PRESENTED" => Some(Self::AsPresented),
                "BIMONTHLY" => Some(Self::Bimonthly),
                "DAILY" => Some(Self::Daily),
                "FORTNIGHTLY" => Some(Self::Fortnightly),
                "HALF_YEARLY" => Some(Self::HalfYearly),
                "MONTHLY" => Some(Self::Monthly),
                "ONE_TIME" => Some(Self::OneTime),
                "QUARTERLY" => Some(Self::Quarterly),
                "WEEKLY" => Some(Self::Weekly),
                "YEARLY" => Some(Self::Yearly),
                _ => None,
            }
        }
    }
    /// RecurrenceRuleType specifies the recurrence rule type of mandate.
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
    pub enum RecurrenceRuleType {
        /// Unspecified recurrence rule type.
        Unspecified = 0,
        /// After recurrence rule type.
        After = 1,
        /// Before recurrence rule type.
        Before = 2,
        /// On recurrence rule type.
        On = 3,
    }
    impl RecurrenceRuleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecurrenceRuleType::Unspecified => "RECURRENCE_RULE_TYPE_UNSPECIFIED",
                RecurrenceRuleType::After => "AFTER",
                RecurrenceRuleType::Before => "BEFORE",
                RecurrenceRuleType::On => "ON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RECURRENCE_RULE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AFTER" => Some(Self::After),
                "BEFORE" => Some(Self::Before),
                "ON" => Some(Self::On),
                _ => None,
            }
        }
    }
    /// AmountRuleType specifies the type of rule associated with the mandate
    /// amount.
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
    pub enum AmountRuleType {
        /// Unspecified amount rule.
        Unspecified = 0,
        /// Exact amount rule. Amount specified is the exact amount for which
        /// mandate could be granted.
        Exact = 1,
        /// Max amount rule. Amount specified is the maximum amount for which
        /// mandate could be granted.
        Max = 2,
    }
    impl AmountRuleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AmountRuleType::Unspecified => "AMOUNT_RULE_TYPE_UNSPECIFIED",
                AmountRuleType::Exact => "EXACT",
                AmountRuleType::Max => "MAX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AMOUNT_RULE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EXACT" => Some(Self::Exact),
                "MAX" => Some(Self::Max),
                _ => None,
            }
        }
    }
}
/// A complaint API transaction processed by the issuer switch. In
/// UPI, this maps to the Complaint API.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplaintTransaction {
    /// The name of the complaint transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/complaintTransactions/{complaint_transaction_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<TransactionInfo>,
    /// Information about the complaint transaction. It can be one of Complaint or
    /// Dispute.
    #[prost(oneof = "complaint_transaction::Case", tags = "3, 4")]
    pub case: ::core::option::Option<complaint_transaction::Case>,
}
/// Nested message and enum types in `ComplaintTransaction`.
pub mod complaint_transaction {
    /// Information about the complaint transaction. It can be one of Complaint or
    /// Dispute.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Case {
        /// Output only. Information about the complaint transaction when it is of
        /// type complaint.
        #[prost(message, tag = "3")]
        Complaint(super::Complaint),
        /// Output only. Information about the complaint transaction when it is of
        /// type dispute.
        #[prost(message, tag = "4")]
        Dispute(super::Dispute),
    }
}
/// Request for the `ListMetadataTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMetadataTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListMetadataTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of metadata transactions.
    ///
    /// A filter expression consists of a field name, a comparison
    /// operator, and a value for filtering. The value must be a string, a
    /// number, or a boolean. The comparison operator must be one of: `<`, `>` or
    /// `=`. Filters are not case sensitive.
    ///
    /// The following fields in the `MetadataTransaction` are eligible for
    /// filtering:
    ///
    ///    * `apiType` - The API type of the metadata transaction. Must be one of
    ///    \[ApiType][google.cloud.paymentgateway.issuerswitch.v1.ApiType\] values.
    ///    Allowed comparison operators: `=`.
    ///    * `transactionType` - The transaction type of the metadata transaction.
    ///    Must be one of
    ///    \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\]
    ///    values. Allowed comparison operators: `=`.
    ///    * `transactionID` - The UPI transaction ID of the metadata transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison operators: `>`,
    ///    `<`.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///    * `apiType = LIST_ACCOUNTS` -  - The API type is _LIST_ACCOUNTS_.
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * `(apiType = LIST_ACCOUNTS) AND (create_time <
    ///    \"2021-08-15T14:50:00Z\")` - The API type is _LIST_ACCOUNTS_ and
    ///    the transaction was received before _2021-08-15 14:50:00 UTC_.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request for the `ListFinancialTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFinancialTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListFinancialTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListFinancialTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of financial transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `FinancialTransaction` are eligible for
    /// filtering:
    ///
    ///    * `transactionID` - The UPI transaction ID of the financial
    ///    transaction. Allowed comparison operators: `=`.
    ///    * `RRN` - The retrieval reference number of the transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payerVPA` - The VPA of the payer in a financial transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payeeVPA` - The VPA of the payee in a financial transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payerMobileNumber` - The mobile number of the payer in a financial
    ///       transaction. Allowed comparison operators: `=`.
    ///    * `payeeMobileNumber` - The mobile number of the payee in a financial
    ///       transaction. Allowed comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison operators: `>`,
    ///    `<`.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///    * `rrn = 123456789123` - The RRN is _123456789123_.
    ///    * `payerVpa = example@goog` - The VPA of the payer is the string
    ///    _example@goog_.
    ///    * `(payeeVpa = example@goog) AND (createTime < "2021-08-15T14:50:00Z")`
    ///    - The VPA of the payee is _example@goog_ and the transaction was received
    ///    before _2021-08-15 14:50:00 UTC_.
    ///    * `createTime > "2021-08-15T14:50:00Z" AND createTime <
    ///    "2021-08-16T14:50:00Z"` - The transaction was received between
    ///    _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request for the `ListMandateTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMandateTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMandateTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListMandateTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of mandate transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `Mandate` are eligible for
    /// filtering:
    ///
    ///    * `uniqueMandateNumber` - UPI Unique Mandate Number (UMN). Allowed
    ///    comparison operators: `=`.
    ///    * `transactionID` - The transaction ID of the mandate transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `transactionType` - The transaction type of the mandate
    ///    transaction. Must be one of
    ///    \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\]
    ///    values. For mandate transactions, only valid transaction types are
    ///    `TRANSACTION_TYPE_CREATE`, `TRANSACTION_TYPE_REVOKE` and
    ///    `TRANSACTION_TYPE_UPDATE`. Allowed comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison
    ///    operators: `>`, `<`.
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///    * `recurrencePattern = MONTHLY` - The recurrence pattern type is
    ///    monthly.
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * `payerVPA = example@okbank` - The VPA of the payer is the string
    ///    _example@okbank_.
    ///    * `(payerVPA = example@okbank) AND (createTime <
    ///    "2021-08-15T14:50:00Z")`
    ///    - The payer VPA example@okbank and the transaction was received
    ///    before _2021-08-15 14:50:00 UTC_.
    ///    * `createTime > "2021-08-15T14:50:00Z" AND createTime <
    ///    "2021-08-16T14:50:00Z"` - The transaction was received between
    ///    _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    ///    * `startDate > "2021-08-15" AND startDate < "2021-08-17"` - The start
    ///    date for mandate is between _2021-08-15_ and _2021-08-17_.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request for the `ListComplaintTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListComplaintTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListComplaintTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListComplaintTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of complaint transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `Complaint` are eligible for
    /// filtering:
    ///
    ///    * `transactionID` - The transaction ID of the complaint transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `transactionType` - The transaction type of the complaint
    ///    transaction. Must be one of
    ///    \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\]
    ///    values. For complaint transactions, only valid transaction types are
    ///   `TRANSACTION_TYPE_CHECK_STATUS`, `TRANSACTION_TYPE_COMPLAINT`,
    ///   `TRANSACTION_TYPE_REVERSAL`, `TRANSACTION_TYPE_DISPUTE`,
    ///   `TRANSACTION_TYPE_REFUND` or `TRANSACTION_TYPE_STATUS_UPDATE`. Allowed
    ///    comparison operators: `=`.
    ///    * `originalRRN` - The retrieval reference number of the original
    ///    transaction for which complaint / dispute was raised / resolved. Allowed
    ///    comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison
    ///    operators: `>`, `<`.
    ///    * `state` - The state of the transaction. Must be one of
    ///    \[TransactionInfo.State][google.cloud.paymentgateway.issuerswitch.v1.TransactionInfo.State\]
    ///    values. Allowed comparison operators: `=`.
    ///    * `errorCode` - Use this filter to list complaint transactions which
    ///    have failed a particular error code. Allowed comparison
    ///    operators: `=`.
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * (createTime < "2021-08-15T14:50:00Z")`
    ///    - The transaction was received before _2021-08-15 14:50:00 UTC_.
    ///    * `createTime > "2021-08-15T14:50:00Z" AND createTime <
    ///    "2021-08-16T14:50:00Z"` - The transaction was received between
    ///    _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for the `ListMetadataTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataTransactionsResponse {
    /// List of non financial metadata transactions satisfying the filtered
    /// request.
    #[prost(message, repeated, tag = "1")]
    pub metadata_transactions: ::prost::alloc::vec::Vec<MetadataTransaction>,
    /// Pass this token in the ListMetadataTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response for the `ListFinancialTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFinancialTransactionsResponse {
    /// List of financial transactions satisfying the filtered request.
    #[prost(message, repeated, tag = "1")]
    pub financial_transactions: ::prost::alloc::vec::Vec<FinancialTransaction>,
    /// Pass this token in the ListFinancialTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response for the `ListMandateTransactionsResponse` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMandateTransactionsResponse {
    /// List of mandate transactions satisfying the filtered request.
    #[prost(message, repeated, tag = "1")]
    pub mandate_transactions: ::prost::alloc::vec::Vec<MandateTransaction>,
    /// Pass this token in the ListMandateTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response for the `ListComplaintTransactionsResponse` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListComplaintTransactionsResponse {
    /// List of complaint transactions satisfying the filtered request.
    #[prost(message, repeated, tag = "1")]
    pub complaint_transactions: ::prost::alloc::vec::Vec<ComplaintTransaction>,
    /// Pass this token in the ListComplaintTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `ExportFinancialTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFinancialTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Transaction type for the financial transaction API. The possible values for
    /// transaction type are
    ///
    /// * TRANSACTION_TYPE_CREDIT
    /// * TRANSACTION_TYPE_DEBIT
    /// * TRANSACTION_TYPE_REVERSAL
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration = "TransactionType", tag = "2")]
    pub transaction_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ExportMetadataTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// API type of the metadata transaction API. The possible values for API type
    /// are
    ///
    /// * BALANCE
    /// * CHECK_STATUS
    /// * HEART_BEAT
    /// * INITIATE_REGISTRATION
    /// * LIST_ACCOUNTS
    /// * UPDATE_CREDENTIALS
    /// * VALIDATE_REGISTRATION
    ///
    /// If no API type is specified, records of all the above API types will be
    /// exported.
    #[prost(enumeration = "ApiType", tag = "2")]
    pub api_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ExportMandateTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMandateTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Transaction type for the mandate transaction API.  The possible values for
    /// transaction type are
    ///
    /// * TRANSACTION_TYPE_CREATE
    /// * TRANSACTION_TYPE_REVOKE
    /// * TRANSACTION_TYPE_UPDATE
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration = "TransactionType", tag = "2")]
    pub transaction_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ExportComplaintTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportComplaintTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Transaction type for the complaint transaction API. The possible values for
    /// transaction type are
    ///
    /// * TRANSACTION_TYPE_CHECK_STATUS
    /// * TRANSACTION_TYPE_COMPLAINT
    /// * TRANSACTION_TYPE_DISPUTE
    /// * TRANSACTION_TYPE_REFUND
    /// * TRANSACTION_TYPE_REVERSAL
    /// * TRANSACTION_TYPE_STATUS_UPDATE
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration = "TransactionType", tag = "2")]
    pub transaction_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response for the `ExportFinancialTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFinancialTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag = "1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Response for the `ExportMetadataTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag = "1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Response for the `ExportMandateTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMandateTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag = "1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Response for the `ExportComplaintTransactions` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportComplaintTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag = "1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Metadata for ExportFinancialTransactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFinancialTransactionsMetadata {
    /// Output only. The time at which the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for ExportMandateTransactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMandateTransactionsMetadata {
    /// Output only. The time at which the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for ExportMetadataTransactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataTransactionsMetadata {
    /// Output only. The time at which the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for ExportComplaintTransactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportComplaintTransactionsMetadata {
    /// Output only. The time at which the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod issuer_switch_transactions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Fetch the issuer switch participant.
    /// Lists and exports transactions processed by the issuer switch.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchTransactionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IssuerSwitchTransactionsClient<T>
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
        ) -> IssuerSwitchTransactionsClient<InterceptedService<T, F>>
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
            IssuerSwitchTransactionsClient::new(
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
        /// List metadata transactions that satisfy the specified filter criteria.
        pub async fn list_metadata_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMetadataTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListMetadataTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ListMetadataTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List financial transactions that satisfy specified filter criteria.
        pub async fn list_financial_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFinancialTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFinancialTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListFinancialTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ListFinancialTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List mandate transactions that satisfy specified filter criteria.
        pub async fn list_mandate_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMandateTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMandateTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListMandateTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ListMandateTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List complaint transactions that satisfy specified filter criteria.
        pub async fn list_complaint_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListComplaintTransactionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListComplaintTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListComplaintTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ListComplaintTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Export financial transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [ExportFinancialTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportFinancialTransactionsMetadata]
        /// - `response`:
        /// [ExportFinancialTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportFinancialTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// 1. `TransactionID`
        ///     * **Min Length** - 35 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - UPI transaction ID.
        /// 1. `TransactionType`
        ///     * **Min Length** - 22 characters
        ///     * **Max Length** - 25 characters
        ///     * **Description** - Type of the transaction. This will be one of
        ///     `TRANSACTION_TYPE_CREDIT`, `TRANSACTION_TYPE_DEBIT` or
        ///     `TRANSACTION_TYPE_REVERSAL`.
        /// 1. `TransactionSubType`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 7 characters
        ///     * **Description** - Subtype of the transaction. This will be one of
        ///     `COLLECT`, or `PAY`.
        /// 1. `CreationTime`
        ///     * **Min Length** - 20 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Timestamp (in UTC) indicating when the issuer
        ///     switch created the transaction resource for processing the transaction.
        ///     The format will be as per RFC-3339. Example : 2022-11-22T23:00:05Z
        /// 1. `State`
        ///     * **Min Length** - 6 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - State of the transaction. This will be one of
        ///     `FAILED`, `SUCCEEDED`, or `TIMED_OUT`.
        /// 1. `RRN`
        ///     * **Min Length** - 12 characters
        ///     * **Max Length** - 12 characters
        ///     * **Description** - Retrieval reference number associated with the
        ///     transaction.
        /// 1. `PayerVPA`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Virtual Payment Address (VPA) of the payer.
        /// 1. `PayerMobileNumber`
        ///     * **Min Length** - 12 characters
        ///     * **Max Length** - 12 characters
        ///     * **Description** - Mobile number of the payer.
        /// 1. `PayerIFSC`
        ///     * **Min Length** - 11 characters
        ///     * **Max Length** - 11 characters
        ///     * **Description** - IFSC of the payer's bank account.
        /// 1. `PayerAccountNumber`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 30 characters
        ///     * **Description** - Payer's bank account number.
        /// 1. `PayerAccountType`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 7 characters
        ///     * **Description** - Payer's bank account type. This will be one of
        ///     `SAVINGS`, `DEFAULT`, `CURRENT`, `NRE`, `NRO`, `PPIWALLET`,
        ///     `BANKWALLET`, `CREDIT`, `SOD`, or `UOD`.
        /// 1. `PayeeVPA`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Virtual Payment Address (VPA) of the payee.
        /// 1. `PayeeMobileNumber`
        ///     * **Min Length** - 12 characters
        ///     * **Max Length** - 12 characters
        ///     * **Description** - Payee's mobile number.
        /// 1. `PayeeIFSC`
        ///     * **Min Length** - 11 characters
        ///     * **Max Length** - 11 characters
        ///     * **Description** - IFSC of the payee's bank account.
        /// 1. `PayeeAccountNumber`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 30 characters
        ///     * **Description** - Payee's bank account number.
        /// 1. `PayeeAccountType`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 10 characters
        ///     * **Description** - Payee's bank account type. This will be one of
        ///     `SAVINGS`, `DEFAULT`, `CURRENT`, `NRE`, `NRO`, `PPIWALLET`,
        ///     `BANKWALLET`, `CREDIT`, `SOD`, or `UOD`.
        /// 1. `PayeeMerchantID`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Payee's merchant ID, only if the payee is a
        ///     merchant.
        /// 1. `PayeeMerchantName`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Payee's merchant name, only if the payee is a
        ///     merchant.
        /// 1. `PayeeMCC`
        ///     * **Min Length** - 4 characters
        ///     * **Max Length** - 4 characters
        ///     * **Description** - Payee's Merchant Category Code (MCC), only if the
        ///     payee is a merchant.
        /// 1. `Currency`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 3 characters
        ///     * **Description** - Currency of the amount involved in the transaction.
        ///     The currency codes are defined in ISO 4217.
        /// 1. `Amount`
        ///     * **Description** - Amount involved in the transaction.
        /// 1. `AdapterRequestIDs`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 2,000 characters
        ///     * **Description** - List of adapter request IDs (colon separated) used
        ///     when invoking the Adapter APIs for fulfilling a transaction request.
        /// 1. `ErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Error code of a failed transaction.
        /// 1. `ErrorMessage`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 10,000 characters
        ///     * **Description** - Error description for a failed transaction.
        /// 1. `UPIErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 3 characters
        ///     * **Description** - Error code as per the UPI specification. The issuer
        ///     switch maps the ErrorCode to an appropriate error code that complies
        ///     with the UPI specification.
        /// 1. `PayerDeviceInfoTypeAppName`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Payment application name on the payer's device.
        /// 1. `PayerDeviceInfoTypeCapability`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Capability of the payer's device.
        /// 1. `PayerDeviceInfoTypeGeoCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 15 characters
        ///     * **Description** - Geo code of the payer's device. This will include
        ///     floating point values for latitude and longitude (separated by colon).
        /// 1. `PayerDeviceInfoTypeID`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Device ID of the payer's device.
        /// 1. `PayerDeviceInfoTypeIP`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 39 characters
        ///     * **Description** - IP address of the payer's device.
        /// 1. `PayerDeviceInfoTypeLocation`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 40 characters
        ///     * **Description** - Coarse location of the payer's device.
        /// 1. `PayerDeviceInfoTypeOS`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Operating system on the payer's device.
        /// 1. `PayerDeviceInfoTypeTelecomProvider`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Telecom provider for the payer's device.
        /// 1. `PayerDeviceInfoTypeDeviceType`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - Type of the payer's device. This will be one of
        ///     'MOB', 'INET', 'USDC/USDB', 'POS'.
        /// 1. `PayeeDeviceInfoTypeAppName`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Payment application name on the payee's device.
        /// 1. `PayeeDeviceInfoTypeCapability`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Capability of the payee's device.
        /// 1. `PayeeDeviceInfoTypeGeoCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 15 characters
        ///     * **Description** - Geo code of the payee's device. This will include
        ///     floating point values for latitude and longitude (separated by colon).
        /// 1. `PayeeDeviceInfoTypeID`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Device ID of the payee's device.
        /// 1. `PayeeDeviceInfoTypeIP`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 39 characters
        ///     * **Description** - IP address of the payee's device.
        /// 1. `PayeeDeviceInfoTypeLocation`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 40 characters
        ///     * **Description** - Coarse location of the payee's device.
        /// 1. `PayeeDeviceInfoTypeOS`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Operating system on the payee's device.
        /// 1. `PayeeDeviceInfoTypeTelecomProvider`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Telecom provider for the payee's device.
        /// 1. `PayeeDeviceInfoTypeDeviceType`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - Type of the payee's device. This will be one of
        ///     'MOB', 'INET', 'USDC/USDB', 'POS'.
        /// 1. `ReferenceID`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Consumer reference number to identify loan number,
        ///     order id etc.
        /// 1. `ReferenceURI`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - URL for the  transaction.
        /// 1. `ReferenceCategory`
        ///     * **Min Length** - 2 characters
        ///     * **Max Length** - 2 characters
        ///     * **Description** - Reference category.
        pub async fn export_financial_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportFinancialTransactionsRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportFinancialTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ExportFinancialTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Export metadata transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [ExportMetadataTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportMetadataTransactionsMetadata]
        /// - `response`:
        /// [ExportMetadataTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportMetadataTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// 1. `TransactionID`
        ///     * **Min Length** - 35 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - UPI transaction ID.
        /// 1. `APIType`
        ///     * **Description** - The transaction's API type. The value will be of
        ///     the [ApiType][google.cloud.paymentgateway.issuerswitch.v1.ApiType]
        ///     enum.
        /// 1. `TransactionType`
        ///     * **Description** - Type of the transaction. The value will be of the
        ///     [TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType]
        ///     enum.
        /// 1. `CreationTime`
        ///     * **Min Length** - 20 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Timestamp (in UTC) indicating when the issuer
        ///     switch created the transaction resource for processing the transaction.
        ///     The format will be as per RFC-3339. Example : 2022-11-22T23:00:05Z
        /// 1. `State`
        ///     * **Min Length** - 6 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - State of the transaction. This will be one of
        ///     `FAILED`, `SUCCEEDED`, or `TIMED_OUT`.
        /// 1. `OriginVPA`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Virtual Payment Address (VPA) of the originator of
        ///     the transaction.
        /// 1. `AdapterRequestIDs`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 2,000 characters
        ///     * **Description** - List of adapter request IDs (colon separated) used
        ///     when invoking the Adapter APIs for fulfilling a transaction request.
        /// 1. `ErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Error code of the failed transaction.
        /// 1. `ErrorMessage`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 10,000 characters
        ///     * **Description** - Error description for the failed transaction.
        /// 1. `UPIErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 3 characters
        ///     * **Description** - Error code as per the UPI specification. The issuer
        ///     switch maps the ErrorCode to an appropriate error code that complies
        ///     with the UPI specification.
        pub async fn export_metadata_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportMetadataTransactionsRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportMetadataTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ExportMetadataTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Export mandate transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [ExportMandateTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportMandateTransactionsMetadata]
        /// - `response`:
        /// [ExportMandateTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportMandateTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// 1. `TransactionID`
        ///     * **Min Length** - 35 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - UPI transaction ID.
        /// 1. `UniqueMandateNumber`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 70 characters
        ///     * **Description** - UPI Unique Mandate Number.
        /// 1. `TransactionType`
        ///     * **Min Length** - 23 characters
        ///     * **Max Length** - 23 characters
        ///     * **Description** - Type of the transaction. This will be one of
        ///     `TRANSACTION_TYPE_CREATE`, `TRANSACTION_TYPE_REVOKE`,
        ///     `TRANSACTION_TYPE_UPDATE`, `TRANSACTION_TYPE_PAUSE` or
        ///     `TRANSACTION_TYPE_UNPAUSE`.
        /// 1. `CreationTime`
        ///     * **Min Length** - 20 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Timestamp (in UTC) indicating when the issuer
        ///     switch created the transaction resource for processing the transaction.
        ///     The format will be as per RFC-3339. Example : 2022-11-22T23:00:05Z
        /// 1. `State`
        ///     * **Min Length** - 6 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - State of the transaction. This will be one of
        ///     `FAILED`, `SUCCEEDED`, or `TIMED_OUT`.
        /// 1. `PayerVPA`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Virtual Payment Address (VPA) of the payer.
        /// 1. `PayerMobileNumber`
        ///     * **Min Length** - 12 characters
        ///     * **Max Length** - 12 characters
        ///     * **Description** - Mobile number of the payer.
        /// 1. `PayerIFSC`
        ///     * **Min Length** - 11 characters
        ///     * **Max Length** - 11 characters
        ///     * **Description** - IFSC of the payer's bank account.
        /// 1. `PayerAccountNumber`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 30 characters
        ///     * **Description** - Payer's bank account number.
        /// 1. `PayerAccountType`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 7 characters
        ///     * **Description** - Payer's bank account type. This will be one of
        ///     `SAVINGS`, `DEFAULT`, `CURRENT`, `NRE`, `NRO`, `PPIWALLET`,
        ///     `BANKWALLET`, `CREDIT`, `SOD`, or `UOD`.
        /// 1. `PayeeVPA`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Virtual Payment Address (VPA) of the payee.
        /// 1. `PayeeMobileNumber`
        ///     * **Min Length** - 12 characters
        ///     * **Max Length** - 12 characters
        ///     * **Description** - Mobile number of the payee.
        /// 1. `PayeeIFSC`
        ///     * **Min Length** - 11 characters
        ///     * **Max Length** - 11 characters
        ///     * **Description** - IFSC of the payee's bank account.
        /// 1. `PayeeAccountNumber`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 30 characters
        ///     * **Description** - Payee's bank account number.
        /// 1. `PayeeAccountType`
        ///     * **Min Length** - 3 characters
        ///     * **Max Length** - 10 characters
        ///     * **Description** - Payee's bank account type. This will be one of
        ///     `SAVINGS`, `DEFAULT`, `CURRENT`, `NRE`, `NRO`, `PPIWALLET`,
        ///     `BANKWALLET`, `CREDIT`, `SOD`, or `UOD`.
        /// 1. `PayeeMerchantID`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 30 characters
        ///     * **Description** - Payee's merchant ID, only if the payee is a
        ///     merchant
        /// 1. `PayeeMerchantName`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Payee's merchant name, only if the payee is a
        ///     merchant.
        /// 1. `PayeeMCC`
        ///     * **Min Length** - 4 characters
        ///     * **Max Length** - 4 characters
        ///     * **Description** - Payee's Merchant Category Code (MCC), only if the
        ///     payee is a merchant.
        /// 1. `Amount`
        ///     * **Description** - Amount specified in the mandate.
        /// 1. `RecurrencePattern`
        ///     * **Description** - Reccurence pattern of the mandate. The value will
        ///     be of the
        ///     [MandateTransaction.RecurrencePatternType][google.cloud.paymentgateway.issuerswitch.v1.MandateTransaction.RecurrencePatternType]
        ///     enum.
        /// 1. `RecurrenceRuleType`
        ///     * **Description** - Reccurrence rule type of the mandate. The value
        ///     will be of the
        ///     [MandateTransaction.RecurrenceRuleType][google.cloud.paymentgateway.issuerswitch.v1.MandateTransaction.RecurrenceRuleType]
        ///     enum.
        /// 1. `RecurrenceRuleValue`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 2 characters
        ///     * **Description** - Recurrence rule value of the mandate. This will be
        ///     an integer between 1 and 31.
        /// 1. `Revokeable`
        ///     * **Min Length** - 4 characters
        ///     * **Max Length** - 5 characters
        ///     * **Description** - Boolean value specifying if the mandate is
        ///     revokable.
        /// 1. `StartDate`
        ///     * **Min Length** - 10 characters
        ///     * **Max Length** - 10 characters
        ///     * **Description** - The start date of the mandate in `DD-MM-YYYY`
        ///     format.
        /// 1. `EndDate`
        ///     * **Min Length** - 10 characters
        ///     * **Max Length** - 10 characters
        ///     * **Description** - The end date of the mandate in `DD-MM-YYYY` format.
        /// 1. `AmountRuleType`
        ///     * **Description** - The amount rule of the mandate. The value will be
        ///     of the
        ///     [MandateTransaction.AmountRuleType][google.cloud.paymentgateway.issuerswitch.v1.MandateTransaction.AmountRuleType]
        ///     enum.
        /// 1. `ApprovalReference`
        ///     * **Min Length** - 6 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - The block funds reference generated by the bank, if
        ///     funds have been blocked for the mandate. This column will have a value
        ///     only when the RecurrencePattern is ONETIME.
        /// 1. `BlockFunds`
        ///     * **Min Length** - 4 characters
        ///     * **Max Length** - 5 characters
        ///     * **Description** - Boolean value specifying if the mandate transaction
        ///     requested to block funds.
        /// 1. `LastUpdateTime`
        ///     * **Min Length** - 20 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Timestamp (in UTC) indicating when was the last
        ///     modification made to the mandate. The format will be as per RFC-3339.
        ///     Example : 2022-11-22T23:00:05Z
        /// 1. `AdapterRequestIDs`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 2,000 characters
        ///     * **Description** - List of adapter request IDs (colon separated) used
        ///     when invoking the Adapter APIs for fulfilling a transaction request.
        /// 1. `ErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Error code of the failed transaction.
        /// 1. `ErrorMessage`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 10,000 characters
        ///     * **Description** - Error description for the failed transaction.
        /// 1. `UPIErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 3 characters
        ///     * **Description** - Error code as per the UPI specification. The issuer
        ///     switch maps the ErrorCode to an appropriate error code that complies
        ///     with the UPI specification.
        /// 1. `PayerDeviceInfoTypeAppName`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Payment application name on the payer's device.
        /// 1. `PayerDeviceInfoTypeCapability`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Capability of the payer's device.
        /// 1. `PayerDeviceInfoTypeGeoCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 15 characters
        ///     * **Description** - Geo code of the payer's device. This will include
        ///     floating point values for latitude and longitude (separated by colon).
        /// 1. `PayerDeviceInfoTypeID`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Device ID of the payer's device.
        /// 1. `PayerDeviceInfoTypeIP`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 39 characters
        ///     * **Description** - IP address of the payer's device.
        /// 1. `PayerDeviceInfoTypeLocation`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 40 characters
        ///     * **Description** - Coarse location of the payer's device.
        /// 1. `PayerDeviceInfoTypeOS`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Operating system on the payer's device.
        /// 1. `PayerDeviceInfoTypeTelecomProvider`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Telecom provider for the payer's device.
        /// 1. `PayerDeviceInfoTypeDeviceType`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - Type of the payer's device. This will be one of
        ///     'MOB', 'INET', 'USDC/USDB', 'POS'.
        /// 1. `PayeeDeviceInfoTypeAppName`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Payment application name on the payee's device.
        /// 1. `PayeeDeviceInfoTypeCapability`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Capability of the payee's device.
        /// 1. `PayeeDeviceInfoTypeGeoCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 15 characters
        ///     * **Description** - Geo code of the payee's device. This will include
        ///     floating point values for latitude and longitude (separated by colon).
        /// 1. `PayeeDeviceInfoTypeID`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Device ID of the payee's device.
        /// 1. `PayeeDeviceInfoTypeIP`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 39 characters
        ///     * **Description** - IP address of the payee's device.
        /// 1. `PayeeDeviceInfoTypeLocation`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 40 characters
        ///     * **Description** - Coarse location of the payee's device.
        /// 1. `PayeeDeviceInfoTypeOS`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Operating system on the payee's device.
        /// 1. `PayeeDeviceInfoTypeTelecomProvider`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 99 characters
        ///     * **Description** - Telecom provider for the payee's device.
        /// 1. `PayeeDeviceInfoTypeDeviceType`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - Type of the payee's device. This will be one of
        ///     `MOB`, `INET`, `USDC/USDB`, `POS`.
        /// 1. `ReferenceID`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Consumer reference number to identify loan number,
        ///     order id etc.
        /// 1. `ReferenceURI`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - URL for the  transaction.
        /// 1. `ReferenceCategory`
        ///     * **Min Length** - 2 characters
        ///     * **Max Length** - 2 characters
        ///     * **Description** - Reference category.
        /// 1. `MandateName`
        ///     * **Min Length** - 1 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - The mandate's name.
        pub async fn export_mandate_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportMandateTransactionsRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportMandateTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ExportMandateTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Export complaint transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`:
        /// [ExportComplaintTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportComplaintTransactionsMetadata]
        /// - `response`:
        /// [ExportComplaintTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportComplaintTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// 1. `TransactionID`
        ///     * **Min Length** - 35 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - UPI transaction ID.
        /// 1. `TransactionType`
        ///     * **Min Length** - 23 characters
        ///     * **Max Length** - 30 characters
        ///     * **Description** - Type of the transaction. This will be one of
        ///     `TRANSACTION_TYPE_CHECK_STATUS`, `TRANSACTION_TYPE_COMPLAINT`,
        ///     `TRANSACTION_TYPE_REVERSAL`, `TRANSACTION_TYPE_DISPUTE`,
        ///     `TRANSACTION_TYPE_REFUND`, or `TRANSACTION_TYPE_STATUS_UPDATE`.
        /// 1. `CreationTime`
        ///     * **Min Length** - 20 characters
        ///     * **Max Length** - 20 characters
        ///     * **Description** - Timestamp (in UTC) indicating when the issuer
        ///     switch created the transaction resource for processing the transaction.
        ///     The format will be as per RFC-3339. Example : 2022-11-22T23:00:05Z
        /// 1: `State`
        ///     * **Min Length** - 6 characters
        ///     * **Max Length** - 9 characters
        ///     * **Description** - State of the transaction. This will be one of
        ///     `FAILED`, `SUCCEEDED`, or `TIMED_OUT`.
        /// 1. `OriginalRRN`
        ///     * **Min Length** - 12 characters
        ///     * **Max Length** - 12 characters
        ///     * **Description** - Retrieval reference number of the original payment
        ///     transaction.
        /// 1. `BankType`
        ///     * **Min Length** - 8 characters
        ///     * **Max Length** - 11 characters
        ///     * **Description** - The subtype of the transaction based on the bank
        ///     involved. This will be one of `BENEFICIARY`, or `REMITTER`.
        /// 1. `OriginalTransactionID`
        ///     * **Min Length** - 35 characters
        ///     * **Max Length** - 35 characters
        ///     * **Description** - Transaction ID of the original unresolved
        ///     transaction.
        /// 1. `RaiseComplaintAdjFlag`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the type of action to raise the
        ///     complaint.
        /// 1. `RaiseComplaintAdjCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the reason of action to raise the
        ///     complaint.
        /// 1. `ResolveComplaintAdjFlag`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the type of action to resolve the
        ///     complaint.
        /// 1. `ResolveComplaintAdjCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the reason of action to resolve the
        ///     complaint.
        /// 1. `RaiseDisputeAdjFlag`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the type of action to raise the dispute.
        /// 1. `RaiseDisputeAdjCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the reason of action to raise the
        ///     dispute.
        /// 1. `ResolveDisputeAdjFlag`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the type of action to resolve the
        ///     dispute.
        /// 1. `ResolveDisputeAdjCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the reason of action to resolve the
        ///     dispute.
        /// 1. `Amount`
        ///     * **Description** - Amount to be resolved.
        /// 1. `CurrentCycle`
        ///     * **Min Length** - 4 characters
        ///     * **Max Length** - 5 characters
        ///     * **Description** - Boolean value specifying if the complaint / dispute
        ///     belongs to current settlement cycle or not.
        /// 1. `CRN`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Defines the Complaint Reference number.
        /// 1. `AdjTime`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the time when the resolution was done.
        /// 1. `RespAdjFlag`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the response category type.
        /// 1. `RespAdjCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the response reason used.
        /// 1. `AdjRemarks`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Indicates the additional remarks for the complaint
        ///     / dispute.
        /// 1. `AdapterRequestIDs`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 2,000 characters
        ///     * **Description** - List of adapter request IDs (colon separated) used
        ///     when invoking the Adapter APIs for fulfilling a transaction request.
        /// 1. `ErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 255 characters
        ///     * **Description** - Error code of the failed transaction.
        /// 1. `ErrorMessage`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 10,000 characters
        ///     * **Description** - Error description for the failed transaction.
        /// 1. `UPIErrorCode`
        ///     * **Min Length** - 0 characters
        ///     * **Max Length** - 3 characters
        ///     * **Description** - Error code as per the UPI specification. The issuer
        ///     switch service maps the ErrorCode to an appropriate error code that
        ///     complies with the UPI specification.
        pub async fn export_complaint_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportComplaintTransactionsRequest>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportComplaintTransactions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions",
                        "ExportComplaintTransactions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The payload for the log entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpiTransaction {
    /// A human readable message about the log entry.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The severity of the log entry.
    #[prost(
        enumeration = "super::super::super::super::logging::r#type::LogSeverity",
        tag = "2"
    )]
    pub severity: i32,
    /// The API type of the transaction.
    #[prost(enumeration = "ApiType", tag = "3")]
    pub api_type: i32,
    /// The XML API type of the transaction.
    #[prost(enumeration = "XmlApiType", tag = "4")]
    pub xml_api_type: i32,
    /// The type of the transaction.
    #[prost(enumeration = "TransactionType", tag = "5")]
    pub transaction_type: i32,
    /// UPI's transaction ID.
    #[prost(string, tag = "6")]
    pub transaction_id: ::prost::alloc::string::String,
    /// UPI's message ID.
    #[prost(string, tag = "7")]
    pub message_id: ::prost::alloc::string::String,
    /// The payment's RRN. This will be present only for payment related
    /// transactions.
    #[prost(string, tag = "8")]
    pub rrn: ::prost::alloc::string::String,
    /// The timestamp at which the payload was received by the issuer switch.
    #[prost(message, optional, tag = "9")]
    pub payload_receipt_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp at which the payload was sent by the issuer switch.
    #[prost(message, optional, tag = "10")]
    pub payload_sent_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Status of the transaction which could be SUCCESS or FAILURE. This will be
    /// populated only after transaction is complete.
    #[prost(enumeration = "transaction_info::State", tag = "11")]
    pub status: i32,
    /// Issuer switch error code. This will be present only for failed
    /// transactions.
    #[prost(string, tag = "12")]
    pub error_code: ::prost::alloc::string::String,
    /// UPI error code that was sent back to NPCI. This will be present only for
    /// failed transactions.
    #[prost(string, tag = "13")]
    pub upi_error_code: ::prost::alloc::string::String,
    /// Issuer switch error message. This will be present only for failed
    /// transactions.
    #[prost(string, tag = "14")]
    pub error_message: ::prost::alloc::string::String,
    /// The ack, request or response payload.
    #[prost(oneof = "upi_transaction::Payload", tags = "15, 16")]
    pub payload: ::core::option::Option<upi_transaction::Payload>,
}
/// Nested message and enum types in `UpiTransaction`.
pub mod upi_transaction {
    /// The ack, request or response payload.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The payload in XML format sent to the issuer switch.
        #[prost(string, tag = "15")]
        Sent(::prost::alloc::string::String),
        /// The payload in XML format received by the issuer switch.
        #[prost(string, tag = "16")]
        Received(::prost::alloc::string::String),
    }
}
/// Request for the
/// \[FetchParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.FetchParticipant\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchParticipantRequest {
    /// Required. The parent resource for the participants. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The account details of the issuer participant.
    #[prost(message, optional, tag = "2")]
    pub account_reference: ::core::option::Option<AccountReference>,
}
/// A customer of the bank who participates in transactions processed by the
/// issuer switch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssuerParticipant {
    /// Required. The account details of the issuer participant. Only the
    /// account_number and ifsc fields will be used.
    #[prost(message, optional, tag = "1")]
    pub account_reference: ::core::option::Option<AccountReference>,
    /// Output only. The mobile number of the participant.
    #[prost(string, tag = "2")]
    pub mobile_number: ::prost::alloc::string::String,
    /// Output only. The current state of the participant.
    #[prost(enumeration = "issuer_participant::State", tag = "3")]
    pub state: i32,
    /// Optional. Additional metadata about the participant.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<issuer_participant::Metadata>,
    /// Output only. The current count of consecutive incorrect MPIN attempts.
    #[prost(int32, tag = "5")]
    pub mpin_failure_count: i32,
    /// Output only. The time when participant's MPIN got locked due to too many
    /// incorrect attempts.
    #[prost(message, optional, tag = "6")]
    pub mpin_locked_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the participant's account was onboarded to PGIS.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the participant was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `IssuerParticipant`.
pub mod issuer_participant {
    /// The metadata of the participant.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metadata {
        /// Optional. Additional metadata about a particular participant as key-value
        /// pairs. These values are returned by the bank adapter/card adapter in
        /// response to the SearchAccounts/InitiateRegistration APIs.
        #[prost(btree_map = "string, string", tag = "1")]
        pub values: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// The state of the participant.
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
        /// Unspecified state.
        Unspecified = 0,
        /// The participant is inactive for all UPI transactions. The issuer switch
        /// will return the `AM` error to the UPI payments orchestrator for any
        /// operation involving MPIN verification for the participant. They need to
        /// register with UPI again and provide a new MPIN.
        Inactive = 1,
        /// The participant is active for all UPI transactions.
        Active = 2,
        /// The participants MPIN has been locked because they have exceeded the
        /// threshold for maximum number of incorrect MPIN verification attempts. No
        /// UPI transactions will be permitted until the participant's MPIN has been
        /// reset.
        MpinLocked = 3,
        /// The participants mobile number has been changed in the issuer bank. Any
        /// transaction involving MPIN verification of the participant will return a
        /// `B1` error to the UPI payments orchestrator. The user will be forced to
        /// re-register with their changed mobile number.
        MobileNumberChanged = 4,
        /// The participant is registering for UPI transactions for the first time.
        NewRegistrationInitiated = 5,
        /// The participant had already registered for UPI transactions but is now
        /// registering again or resetting their MPIN.
        ReRegistrationInitiated = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Inactive => "INACTIVE",
                State::Active => "ACTIVE",
                State::MpinLocked => "MPIN_LOCKED",
                State::MobileNumberChanged => "MOBILE_NUMBER_CHANGED",
                State::NewRegistrationInitiated => "NEW_REGISTRATION_INITIATED",
                State::ReRegistrationInitiated => "RE_REGISTRATION_INITIATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "INACTIVE" => Some(Self::Inactive),
                "ACTIVE" => Some(Self::Active),
                "MPIN_LOCKED" => Some(Self::MpinLocked),
                "MOBILE_NUMBER_CHANGED" => Some(Self::MobileNumberChanged),
                "NEW_REGISTRATION_INITIATED" => Some(Self::NewRegistrationInitiated),
                "RE_REGISTRATION_INITIATED" => Some(Self::ReRegistrationInitiated),
                _ => None,
            }
        }
    }
}
/// Request for the
/// \[UpdateIssuerParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.UpdateIssuerParticipant\]
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIssuerParticipantRequest {
    /// Required. The parent resource for the participants. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The participant to update.
    #[prost(message, optional, tag = "2")]
    pub issuer_participant: ::core::option::Option<IssuerParticipant>,
    /// Required. The list of fields to update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the
/// \[ActivateParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.ActivateParticipant\],
/// \[DeactivateParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.DeactivateParticipant\]
/// and
/// \[MobileNumberUpdated][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.MobileNumberChanged\]
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipantStateChangeRequest {
    /// Required. The parent resource for the participant. The format is
    /// `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The identifier for the issuer participant. One of the two values must be
    /// specified.
    #[prost(oneof = "participant_state_change_request::Id", tags = "2, 3")]
    pub id: ::core::option::Option<participant_state_change_request::Id>,
}
/// Nested message and enum types in `ParticipantStateChangeRequest`.
pub mod participant_state_change_request {
    /// The identifier for the issuer participant. One of the two values must be
    /// specified.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// Optional. The account details of the issuer participant.
        #[prost(message, tag = "2")]
        AccountReference(super::AccountReference),
        /// Optional. The mobile number of the issuer participant.
        #[prost(string, tag = "3")]
        MobileNumber(::prost::alloc::string::String),
    }
}
/// Response for the
/// \[ActivateParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.ActivateParticipant\],
/// \[DeactivateParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.DeactivateParticipant\]
/// and
/// \[MobileNumberChanged][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.MobileNumberChanged\]
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssuerParticipants {
    /// Output only. The list of updated participants.
    #[prost(message, repeated, tag = "1")]
    pub participants: ::prost::alloc::vec::Vec<IssuerParticipant>,
}
/// Generated client implementations.
pub mod issuer_switch_participants_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service that allows for the management of participants in the issuer
    /// switch.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchParticipantsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IssuerSwitchParticipantsClient<T>
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
        ) -> IssuerSwitchParticipantsClient<InterceptedService<T, F>>
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
            IssuerSwitchParticipantsClient::new(
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
        /// Fetch the issuer switch participant. This method can be used to retrieve
        /// all details of a participant in the issuer switch.
        ///
        /// In UPI, the participant is identified by their account's IFSC and their
        /// account number.
        pub async fn fetch_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerParticipant>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants/FetchParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants",
                        "FetchParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update the issuer switch participant. Currently, this API only allows for
        /// the
        /// [metadata][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.metadata]
        /// field to be updated.
        ///
        /// The `number` of key-value pairs in the `metadata` field, the length of each
        /// `key` and the length of each `value` should be within the thresholds
        /// defined for them in the issuer switch configuration. Any violation of these
        /// thresholds will cause this API to return an error. The default values for
        /// these thresholds are:
        ///
        /// * `Maximum number` of key-value pairs - `5`
        /// * `Maximum length` of a key - `100`
        /// * `Maximum length` of a value - `500`
        ///
        /// **Note** that this method replaces any existing `metadata` field value in
        /// the participant with the new value. Specifically, it does not do a merge.
        /// If key-value pairs are to be added/removed from the metadata, then
        /// callers must follow the following steps:
        ///
        /// 1. Invoke the
        ///   [FetchParticipant][google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants.FetchParticipant]
        ///    API to get the current value of the `metadata` field.
        /// 1. Update the `metadata` map to add/remove key-value pairs from it.
        /// 1. Update the `metadata` in the issuer switch using this method.
        pub async fn update_issuer_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIssuerParticipantRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerParticipant>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants/UpdateIssuerParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants",
                        "UpdateIssuerParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Activate the issuer switch participant for UPI transactions. This API
        /// sets the state of the participant to
        /// [ACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.ACTIVE].
        /// A participant in the `ACTIVE` state can perform all UPI operations
        /// normally.
        ///
        /// The behavior of this API varies based on the current state of the
        /// participant.
        ///
        /// *   Current state is
        ///     [ACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.ACTIVE]
        ///     : This API will make no change to the participant's state and returns a
        ///     successful response.
        /// *    Current state is
        ///     [INACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.INACTIVE]
        ///     : If an _MPIN_ has already been provisioned for the participant, then
        ///     this API will change the state of the participant to `ACTIVE`. Else,
        ///     this API will return an error.
        /// *   Current state is
        ///     [MOBILE_NUMBER_CHANGED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.MOBILE_NUMBER_CHANGED]
        ///     : The state cannot be changed to `ACTIVE`. This API will return an
        ///     error.
        /// *   Current state is
        ///     [NEW_REGISTRATION_INITIATED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.NEW_REGISTRATION_INITIATED]
        ///     : The state cannot be changed to `ACTIVE`. This API will return an
        ///     error.
        /// *   Current state is
        ///     [RE_REGISTRATION_INITIATED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.RE_REGISTRATION_INITIATED]
        ///     : The state cannot be changed to `ACTIVE`. This API will return an
        ///     error.
        pub async fn activate_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipantStateChangeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerParticipants>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants/ActivateParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants",
                        "ActivateParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deactivate the issuer switch participant for UPI transactions. This API
        /// sets the state of the participant to
        /// [INACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.INACTIVE].
        /// An `INACTIVE` participant cannot perform any UPI operation which involves
        /// MPIN verification.
        ///
        /// The behavior of this API varies based on the current state of the
        /// participant.
        ///
        /// *   Current state is
        ///     [ACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.ACTIVE]
        ///     : The state will change to `INACTIVE`. The user will be forced to
        ///     re-register with UPI and reset their MPIN  to perform any UPI
        ///     operations.
        /// *   Current state is
        ///     [INACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.INACTIVE]
        ///     : This API will make no change to the participant's state and returns a
        ///     successful response.
        /// *   Current state is
        ///     [MOBILE_NUMBER_CHANGED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.MOBILE_NUMBER_CHANGED]
        ///     : The state cannot be changed to `INACTIVE`. This API will return an
        ///     error.
        /// *   Current state is
        ///     [NEW_REGISTRATION_INITIATED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.NEW_REGISTRATION_INITIATED]
        ///     : The state cannot be changed to `INACTIVE`. This API will return an
        ///     error.
        /// *   Current state is
        ///     [RE_REGISTRATION_INITIATED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.RE_REGISTRATION_INITIATED]
        ///     : The state cannot be changed to `INACTIVE`. This API will return an
        ///     error.
        pub async fn deactivate_participant(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipantStateChangeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerParticipants>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants/DeactivateParticipant",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants",
                        "DeactivateParticipant",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Mark the state of the issuer switch participant as _mobile number changed_
        /// to prevent UPI transactions by the user. This API sets the state of the
        /// participant to
        /// [MOBILE_NUMBER_CHANGED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.MOBILE_NUMBER_CHANGED].
        ///
        /// Any UPI operation for a participant in the `MOBILE_NUMBER_CHANGED` state
        /// will cause the issuer switch to return a `B1` error to the UPI payments
        /// orchestrator which would force the user to re-register with UPI.
        ///
        /// The behavior of this API varies based on the current state of the
        /// participant.
        ///
        /// *   Current state is
        ///     [ACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.ACTIVE]
        ///     : The state will change to `MOBILE_NUMBER_CHANGED`. Any operation
        ///     involving MPIN verification of the participant will return a `B1` error
        ///     to the UPI payments orchestrator. The user will be forced to
        ///     re-register with their changed mobile number.
        /// *   Current state is
        ///     [INACTIVE][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.INACTIVE]
        ///     : The state will change to `MOBILE_NUMBER_CHANGED`. Any operation
        ///     involving MPIN verification of the participant will return a `B1` error
        ///     to the UPI payments orchestrator. The user will be forced to
        ///     re-register with their changed mobile number.
        /// *   Current state is
        ///     [MOBILE_NUMBER_CHANGED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.MOBILE_NUMBER_CHANGED]
        ///     : This API will make no change to the participant's state and returns a
        ///     successful response.
        /// *   Current state is
        ///     [NEW_REGISTRATION_INITIATED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.NEW_REGISTRATION_INITIATED]
        ///     : The state cannot be changed to `MOBILE_NUMBER_CHANGED`. This API will
        ///     return an error.
        /// *   Current state is
        ///     [RE_REGISTRATION_INITIATED][google.cloud.paymentgateway.issuerswitch.v1.IssuerParticipant.State.RE_REGISTRATION_INITIATED]
        ///     : The state will change to `MOBILE_NUMBER_CHANGED`. Any operation
        ///     involving MPIN verification of the participant will return a `B1` error
        ///     to the UPI payments orchestrator. The user will be forced to
        ///     re-register with their changed mobile number.
        pub async fn mobile_number_changed(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipantStateChangeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IssuerParticipants>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants/MobileNumberChanged",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchParticipants",
                        "MobileNumberChanged",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
