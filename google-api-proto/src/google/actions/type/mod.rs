/// Represents a date and time range. This can represent:
///
/// * A range between points in time with time zone or offset, e.g. the duration
///    of a flight which starts in the "America/New_York" time zone and ends in
///    the "Australia/Sydney" time zone
/// * A range between points in time without time zone/offset info, e.g. an
///    appointment in local time
/// * A range starting at a specific date and time, e.g. the range of time which
///    can be measured in milliseconds since the Unix epoch (period starting with
///    1970-01-01T00:00:00Z)
/// * A range ending at a specific date and time, e.g. range of time before
///    a deadline
///
/// When considering whether a DateTime falls within a DateTimeRange, the start
/// of the range is inclusive and the end is exclusive.
///
/// While [google.type.DateTime][google.type.DateTime] allows zero years, DateTimeRange does not.
/// Year must always be non-zero.
///
/// When both start and end are set, either both or neither must have a
/// time_offset. When set, time_offset can be specified by either utc_offset or
/// time_zone and must match for start and end, that is if start has utc_offset
/// set then end must also have utc_offset set. The values of utc_offset or
/// time_zone need not be the same for start and end.
///
/// When both start and end are set, start must be chronologically less than or
/// equal to end. When start and end are equal, the range is empty.
///
/// The semantics of start and end are the same as those of
/// [google.type.DateTime][google.type.DateTime].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeRange {
    /// DateTime at which the date range begins. If unset, the range has no
    /// beginning bound.
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<super::super::r#type::DateTime>,
    /// DateTime at which the date range ends. If unset, the range has no ending
    /// bound.
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<super::super::r#type::DateTime>,
}
/// Represents a range based on whole or partial calendar dates, e.g. the
/// duration of a hotel reservation or the Common Era. This can represent:
///
/// * A range between full dates, e.g. the duration of a hotel reservation
/// * A range between years, e.g. a historical era
/// * A range between year/month dates, e.g. the duration of a job on a resume
/// * A range beginning in a year, e.g. the Common Era
/// * A range ending on a specific date, e.g. the period of time before an event
///
/// While [google.type.Date][google.type.Date] allows zero years, DateRange does not. Year must
/// always be non-zero.
///
/// End cannot be chronologically before start. For example, if start has year
/// 2000, end cannot have year 1999.
///
/// When both set, start and end must have exactly the same precision. That is,
/// they must have the same fields populated with non-zero values. For example,
/// if start specifies only year and month, then end must also specify only year
/// and month (but not day).
///
/// The date range is inclusive. That is, the dates specified by start and end
/// are part of the date range. For example, the date January 1, 2000 falls
/// within any date with start or end equal to January 1, 2000. When determining
/// whether a date is inside a date range, the date should only be compared to
/// start and end when those values are set.
///
/// When a date and date range are specified to different degrees of precision,
/// the rules for evaluating whether that date is inside the date range are as
/// follows:
///
///   * When comparing the date to the start of the date range, unspecified months
///     should be replaced with 1, and unspecified days should be replaced with 1.
///     For example, the year 2000 is within the date range with start equal to
///     January 1, 2000 and no end. And the date January 1, 2000 is within the
///     date range with start equal to the year 2000 and no end.
///
///   * When comparing the date to the end of the date range, unspecified months
///     should be replaced with 12, and unspecified days should be replaced with
///     the last valid day for the month/year. For example, the year 2000 is
///     within the date range with start equal to January 1, 1999 and end equal to
///     December 31, 2000. And the date December 31, 2001 is within the date range
///     with start equal to the year 2000 and end equal to the year 2001.
///
/// The semantics of start and end are the same as those of [google.type.Date][google.type.Date],
/// except that year must always be non-zero in DateRange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// Date at which the date range begins. If unset, the date range has no
    /// beginning bound.
    #[prost(message, optional, tag = "1")]
    pub start: ::core::option::Option<super::super::r#type::Date>,
    /// Date at which the date range ends. If unset, the date range has no ending
    /// bound.
    #[prost(message, optional, tag = "2")]
    pub end: ::core::option::Option<super::super::r#type::Date>,
}
