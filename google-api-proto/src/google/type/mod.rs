/// Represents a textual expression in the Common Expression Language (CEL)
/// syntax. CEL is a C-like expression language. The syntax and semantics of CEL
/// are documented at <https://github.com/google/cel-spec.>
///
/// Example (Comparison):
///
///     title: "Summary size limit"
///     description: "Determines if a summary is less than 100 chars"
///     expression: "document.summary.size() < 100"
///
/// Example (Equality):
///
///     title: "Requestor is owner"
///     description: "Determines if requestor is the document owner"
///     expression: "document.owner == request.auth.claims.email"
///
/// Example (Logic):
///
///     title: "Public documents"
///     description: "Determine whether the document should be publicly visible"
///     expression: "document.type != 'private' && document.type != 'internal'"
///
/// Example (Data Manipulation):
///
///     title: "Notification string"
///     description: "Create a notification string with a timestamp."
///     expression: "'New message received at ' + string(document.create_time)"
///
/// The exact variables and functions that may be referenced within an expression
/// are determined by the service that evaluates it. See the service
/// documentation for additional information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
    /// Textual representation of an expression in Common Expression Language
    /// syntax.
    #[prost(string, tag = "1")]
    pub expression: ::prost::alloc::string::String,
    /// Optional. Title for the expression, i.e. a short string describing
    /// its purpose. This can be used e.g. in UIs which allow to enter the
    /// expression.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Optional. Description of the expression. This is a longer text which
    /// describes the expression, e.g. when hovered over it in a UI.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. String indicating the location of the expression for error
    /// reporting, e.g. a file name and a position in the file.
    #[prost(string, tag = "4")]
    pub location: ::prost::alloc::string::String,
}
/// Represents a whole or partial calendar date, such as a birthday. The time of
/// day and time zone are either specified elsewhere or are insignificant. The
/// date is relative to the Gregorian Calendar. This can represent one of the
/// following:
///
/// * A full date, with non-zero year, month, and day values
/// * A month and day value, with a zero year, such as an anniversary
/// * A year on its own, with zero month and day values
/// * A year and month value, with a zero day, such as a credit card expiration
/// date
///
/// Related types are \[google.type.TimeOfDay][google.type.TimeOfDay\] and
/// `google.protobuf.Timestamp`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Date {
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without
    /// a year.
    #[prost(int32, tag = "1")]
    pub year: i32,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a
    /// month and day.
    #[prost(int32, tag = "2")]
    pub month: i32,
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0
    /// to specify a year by itself or a year and month where the day isn't
    /// significant.
    #[prost(int32, tag = "3")]
    pub day: i32,
}
/// An object that represents a latitude/longitude pair. This is expressed as a
/// pair of doubles to represent degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="<http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84>
/// standard</a>. Values must be within normalized ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    #[prost(double, tag = "2")]
    pub longitude: f64,
}
/// Represents a day of the week.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DayOfWeek {
    /// The day of the week is unspecified.
    Unspecified = 0,
    /// Monday
    Monday = 1,
    /// Tuesday
    Tuesday = 2,
    /// Wednesday
    Wednesday = 3,
    /// Thursday
    Thursday = 4,
    /// Friday
    Friday = 5,
    /// Saturday
    Saturday = 6,
    /// Sunday
    Sunday = 7,
}
/// Represents a time interval, encoded as a Timestamp start (inclusive) and a
/// Timestamp end (exclusive).
///
/// The start must be less than or equal to the end.
/// When the start equals the end, the interval is empty (matches no time).
/// When both start and end are unspecified, the interval matches any time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interval {
    /// Optional. Inclusive start of the interval.
    ///
    /// If specified, a Timestamp matching this interval will have to be the same
    /// or after the start.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Exclusive end of the interval.
    ///
    /// If specified, a Timestamp matching this interval will have to be before the
    /// end.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A `CalendarPeriod` represents the abstract concept of a time period that has
/// a canonical start. Grammatically, "the start of the current
/// `CalendarPeriod`." All calendar times begin at midnight UTC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CalendarPeriod {
    /// Undefined period, raises an error.
    Unspecified = 0,
    /// A day.
    Day = 1,
    /// A week. Weeks begin on Monday, following
    /// [ISO 8601](<https://en.wikipedia.org/wiki/ISO_week_date>).
    Week = 2,
    /// A fortnight. The first calendar fortnight of the year begins at the start
    /// of week 1 according to
    /// [ISO 8601](<https://en.wikipedia.org/wiki/ISO_week_date>).
    Fortnight = 3,
    /// A month.
    Month = 4,
    /// A quarter. Quarters start on dates 1-Jan, 1-Apr, 1-Jul, and 1-Oct of each
    /// year.
    Quarter = 5,
    /// A half-year. Half-years start on dates 1-Jan and 1-Jul.
    Half = 6,
    /// A year.
    Year = 7,
}
/// Represents a fraction in terms of a numerator divided by a denominator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fraction {
    /// The numerator in the fraction, e.g. 2 in 2/3.
    #[prost(int64, tag = "1")]
    pub numerator: i64,
    /// The value by which the numerator is divided, e.g. 3 in 2/3. Must be
    /// positive.
    #[prost(int64, tag = "2")]
    pub denominator: i64,
}
/// Represents a month in the Gregorian calendar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Month {
    /// The unspecified month.
    Unspecified = 0,
    /// The month of January.
    January = 1,
    /// The month of February.
    February = 2,
    /// The month of March.
    March = 3,
    /// The month of April.
    April = 4,
    /// The month of May.
    May = 5,
    /// The month of June.
    June = 6,
    /// The month of July.
    July = 7,
    /// The month of August.
    August = 8,
    /// The month of September.
    September = 9,
    /// The month of October.
    October = 10,
    /// The month of November.
    November = 11,
    /// The month of December.
    December = 12,
}
/// An object representing a phone number, suitable as an API wire format.
///
/// This representation:
///
///  - should not be used for locale-specific formatting of a phone number, such
///    as "+1 (650) 253-0000 ext. 123"
///
///  - is not designed for efficient storage
///  - may not be suitable for dialing - specialized libraries (see references)
///    should be used to parse the number for that purpose
///
/// To do something meaningful with this number, such as format it for various
/// use-cases, convert it to an `i18n.phonenumbers.PhoneNumber` object first.
///
/// For instance, in Java this would be:
///
///    com.google.type.PhoneNumber wireProto =
///        com.google.type.PhoneNumber.newBuilder().build();
///    com.google.i18n.phonenumbers.Phonenumber.PhoneNumber phoneNumber =
///        PhoneNumberUtil.getInstance().parse(wireProto.getE164Number(), "ZZ");
///    if (!wireProto.getExtension().isEmpty()) {
///      phoneNumber.setExtension(wireProto.getExtension());
///    }
///
///  Reference(s):
///   - <https://github.com/google/libphonenumber>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhoneNumber {
    /// The phone number's extension. The extension is not standardized in ITU
    /// recommendations, except for being defined as a series of numbers with a
    /// maximum length of 40 digits. Other than digits, some other dialing
    /// characters such as ',' (indicating a wait) or '#' may be stored here.
    ///
    /// Note that no regions currently use extensions with short codes, so this
    /// field is normally only set in conjunction with an E.164 number. It is held
    /// separately from the E.164 number to allow for short code extensions in the
    /// future.
    #[prost(string, tag = "3")]
    pub extension: ::prost::alloc::string::String,
    /// Required.  Either a regular number, or a short code.  New fields may be
    /// added to the oneof below in the future, so clients should ignore phone
    /// numbers for which none of the fields they coded against are set.
    #[prost(oneof = "phone_number::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<phone_number::Kind>,
}
/// Nested message and enum types in `PhoneNumber`.
pub mod phone_number {
    /// An object representing a short code, which is a phone number that is
    /// typically much shorter than regular phone numbers and can be used to
    /// address messages in MMS and SMS systems, as well as for abbreviated dialing
    /// (e.g. "Text 611 to see how many minutes you have remaining on your plan.").
    ///
    /// Short codes are restricted to a region and are not internationally
    /// dialable, which means the same short code can exist in different regions,
    /// with different usage and pricing, even if those regions share the same
    /// country calling code (e.g. US and CA).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShortCode {
        /// Required. The BCP-47 region code of the location where calls to this
        /// short code can be made, such as "US" and "BB".
        ///
        /// Reference(s):
        ///  - <http://www.unicode.org/reports/tr35/#unicode_region_subtag>
        #[prost(string, tag = "1")]
        pub region_code: ::prost::alloc::string::String,
        /// Required. The short code digits, without a leading plus ('+') or country
        /// calling code, e.g. "611".
        #[prost(string, tag = "2")]
        pub number: ::prost::alloc::string::String,
    }
    /// Required.  Either a regular number, or a short code.  New fields may be
    /// added to the oneof below in the future, so clients should ignore phone
    /// numbers for which none of the fields they coded against are set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// The phone number, represented as a leading plus sign ('+'), followed by a
        /// phone number that uses a relaxed ITU E.164 format consisting of the
        /// country calling code (1 to 3 digits) and the subscriber number, with no
        /// additional spaces or formatting, e.g.:
        ///  - correct: "+15552220123"
        ///  - incorrect: "+1 (555) 222-01234 x123".
        ///
        /// The ITU E.164 format limits the latter to 12 digits, but in practice not
        /// all countries respect that, so we relax that restriction here.
        /// National-only numbers are not allowed.
        ///
        /// References:
        ///  - <https://www.itu.int/rec/T-REC-E.164-201011-I>
        ///  - <https://en.wikipedia.org/wiki/E.164.>
        ///  - <https://en.wikipedia.org/wiki/List_of_country_calling_codes>
        #[prost(string, tag = "1")]
        E164Number(::prost::alloc::string::String),
        /// A short code.
        ///
        /// Reference(s):
        ///  - <https://en.wikipedia.org/wiki/Short_code>
        #[prost(message, tag = "2")]
        ShortCode(ShortCode),
    }
}
/// Localized variant of a text in a particular language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalizedText {
    /// Localized string in the language corresponding to `language_code' below.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The text's BCP-47 language code, such as "en-US" or "sr-Latn".
    ///
    /// For more information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
}
/// Represents an amount of money with its currency type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[prost(string, tag = "1")]
    pub currency_code: ::prost::alloc::string::String,
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    #[prost(int64, tag = "2")]
    pub units: i64,
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    #[prost(int32, tag = "3")]
    pub nanos: i32,
}
/// A quaternion is defined as the quotient of two directed lines in a
/// three-dimensional space or equivalently as the quotient of two Euclidean
/// vectors (<https://en.wikipedia.org/wiki/Quaternion>).
///
/// Quaternions are often used in calculations involving three-dimensional
/// rotations (<https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation>),
/// as they provide greater mathematical robustness by avoiding the gimbal lock
/// problems that can be encountered when using Euler angles
/// (<https://en.wikipedia.org/wiki/Gimbal_lock>).
///
/// Quaternions are generally represented in this form:
///
///     w + xi + yj + zk
///
/// where x, y, z, and w are real numbers, and i, j, and k are three imaginary
/// numbers.
///
/// Our naming choice `(x, y, z, w)` comes from the desire to avoid confusion for
/// those interested in the geometric properties of the quaternion in the 3D
/// Cartesian space. Other texts often use alternative names or subscripts, such
/// as `(a, b, c, d)`, `(1, i, j, k)`, or `(0, 1, 2, 3)`, which are perhaps
/// better suited for mathematical interpretations.
///
/// To avoid any confusion, as well as to maintain compatibility with a large
/// number of software libraries, the quaternions represented using the protocol
/// buffer below *must* follow the Hamilton convention, which defines `ij = k`
/// (i.e. a right-handed algebra), and therefore:
///
///     i^2 = j^2 = k^2 = ijk = −1
///     ij = −ji = k
///     jk = −kj = i
///     ki = −ik = j
///
/// Please DO NOT use this to represent quaternions that follow the JPL
/// convention, or any of the other quaternion flavors out there.
///
/// Definitions:
///
///   - Quaternion norm (or magnitude): `sqrt(x^2 + y^2 + z^2 + w^2)`.
///   - Unit (or normalized) quaternion: a quaternion whose norm is 1.
///   - Pure quaternion: a quaternion whose scalar component (`w`) is 0.
///   - Rotation quaternion: a unit quaternion used to represent rotation.
///   - Orientation quaternion: a unit quaternion used to represent orientation.
///
/// A quaternion can be normalized by dividing it by its norm. The resulting
/// quaternion maintains the same direction, but has a norm of 1, i.e. it moves
/// on the unit sphere. This is generally necessary for rotation and orientation
/// quaternions, to avoid rounding errors:
/// <https://en.wikipedia.org/wiki/Rotation_formalisms_in_three_dimensions>
///
/// Note that `(x, y, z, w)` and `(-x, -y, -z, -w)` represent the same rotation,
/// but normalization would be even more useful, e.g. for comparison purposes, if
/// it would produce a unique representation. It is thus recommended that `w` be
/// kept positive, which can be achieved by changing all the signs when `w` is
/// negative.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quaternion {
    /// The x component.
    #[prost(double, tag = "1")]
    pub x: f64,
    /// The y component.
    #[prost(double, tag = "2")]
    pub y: f64,
    /// The z component.
    #[prost(double, tag = "3")]
    pub z: f64,
    /// The scalar component.
    #[prost(double, tag = "4")]
    pub w: f64,
}
/// Represents a postal address, e.g. for postal delivery or payments addresses.
/// Given a postal address, a postal service can deliver items to a premise, P.O.
/// Box or similar.
/// It is not intended to model geographical locations (roads, towns,
/// mountains).
///
/// In typical usage an address would be created via user input or from importing
/// existing data, depending on the type of process.
///
/// Advice on address input / editing:
///  - Use an i18n-ready address widget such as
///    <https://github.com/google/libaddressinput>)
/// - Users should not be presented with UI elements for input or editing of
///   fields outside countries where that field is used.
///
/// For more guidance on how to use this schema, please see:
/// <https://support.google.com/business/answer/6397478>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostalAddress {
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is
    /// the latest revision.
    ///
    /// All new revisions **must** be backward compatible with old revisions.
    #[prost(int32, tag = "1")]
    pub revision: i32,
    /// Required. CLDR region code of the country/region of the address. This
    /// is never inferred and it is up to the user to ensure the value is
    /// correct. See <http://cldr.unicode.org/> and
    /// <http://www.unicode.org/cldr/charts/30/supplemental/territory_information.html>
    /// for details. Example: "CH" for Switzerland.
    #[prost(string, tag = "2")]
    pub region_code: ::prost::alloc::string::String,
    /// Optional. BCP-47 language code of the contents of this address (if
    /// known). This is often the UI language of the input form or is expected
    /// to match one of the languages used in the address' country/region, or their
    /// transliterated equivalents.
    /// This can affect formatting in certain countries, but is not critical
    /// to the correctness of the data and will never affect any validation or
    /// other non-formatting related operations.
    ///
    /// If this value is not known, it should be omitted (rather than specifying a
    /// possibly incorrect default).
    ///
    /// Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Postal code of the address. Not all countries use or require
    /// postal codes to be present, but where they are used, they may trigger
    /// additional validation with other parts of the address (e.g. state/zip
    /// validation in the U.S.A.).
    #[prost(string, tag = "4")]
    pub postal_code: ::prost::alloc::string::String,
    /// Optional. Additional, country-specific, sorting code. This is not used
    /// in most regions. Where it is used, the value is either a string like
    /// "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number
    /// alone, representing the "sector code" (Jamaica), "delivery area indicator"
    /// (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[prost(string, tag = "5")]
    pub sorting_code: ::prost::alloc::string::String,
    /// Optional. Highest administrative subdivision which is used for postal
    /// addresses of a country or region.
    /// For example, this can be a state, a province, an oblast, or a prefecture.
    /// Specifically, for Spain this is the province and not the autonomous
    /// community (e.g. "Barcelona" and not "Catalonia").
    /// Many countries don't use an administrative area in postal addresses. E.g.
    /// in Switzerland this should be left unpopulated.
    #[prost(string, tag = "6")]
    pub administrative_area: ::prost::alloc::string::String,
    /// Optional. Generally refers to the city/town portion of the address.
    /// Examples: US city, IT comune, UK post town.
    /// In regions of the world where localities are not well defined or do not fit
    /// into this structure well, leave locality empty and use address_lines.
    #[prost(string, tag = "7")]
    pub locality: ::prost::alloc::string::String,
    /// Optional. Sublocality of the address.
    /// For example, this can be neighborhoods, boroughs, districts.
    #[prost(string, tag = "8")]
    pub sublocality: ::prost::alloc::string::String,
    /// Unstructured address lines describing the lower levels of an address.
    ///
    /// Because values in address_lines do not have type information and may
    /// sometimes contain multiple values in a single field (e.g.
    /// "Austin, TX"), it is important that the line order is clear. The order of
    /// address lines should be "envelope order" for the country/region of the
    /// address. In places where this can vary (e.g. Japan), address_language is
    /// used to make it explicit (e.g. "ja" for large-to-small ordering and
    /// "ja-Latn" or "en" for small-to-large). This way, the most specific line of
    /// an address can be selected based on the language.
    ///
    /// The minimum permitted structural representation of an address consists
    /// of a region_code with all remaining information placed in the
    /// address_lines. It would be possible to format such an address very
    /// approximately without geocoding, but no semantic reasoning could be
    /// made about any of the address components until it was at least
    /// partially resolved.
    ///
    /// Creating an address only containing a region_code and address_lines, and
    /// then geocoding is the recommended way to handle completely unstructured
    /// addresses (as opposed to guessing which parts of the address should be
    /// localities or administrative areas).
    #[prost(string, repeated, tag = "9")]
    pub address_lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The recipient at the address.
    /// This field may, under certain circumstances, contain multiline information.
    /// For example, it might contain "care of" information.
    #[prost(string, repeated, tag = "10")]
    pub recipients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The name of the organization at the address.
    #[prost(string, tag = "11")]
    pub organization: ::prost::alloc::string::String,
}
/// Represents a color in the RGBA color space. This representation is designed
/// for simplicity of conversion to/from color representations in various
/// languages over compactness. For example, the fields of this representation
/// can be trivially provided to the constructor of `java.awt.Color` in Java; it
/// can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha`
/// method in iOS; and, with just a little work, it can be easily formatted into
/// a CSS `rgba()` string in JavaScript.
///
/// This reference page doesn't carry information about the absolute color
/// space
/// that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB,
/// DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color
/// space.
///
/// When color equality needs to be decided, implementations, unless
/// documented otherwise, treat two colors as equal if all their red,
/// green, blue, and alpha values each differ by at most 1e-5.
///
/// Example (Java):
///
///      import com.google.type.Color;
///
///      // ...
///      public static java.awt.Color fromProto(Color protocolor) {
///        float alpha = protocolor.hasAlpha()
///            ? protocolor.getAlpha().getValue()
///            : 1.0;
///
///        return new java.awt.Color(
///            protocolor.getRed(),
///            protocolor.getGreen(),
///            protocolor.getBlue(),
///            alpha);
///      }
///
///      public static Color toProto(java.awt.Color color) {
///        float red = (float) color.getRed();
///        float green = (float) color.getGreen();
///        float blue = (float) color.getBlue();
///        float denominator = 255.0;
///        Color.Builder resultBuilder =
///            Color
///                .newBuilder()
///                .setRed(red / denominator)
///                .setGreen(green / denominator)
///                .setBlue(blue / denominator);
///        int alpha = color.getAlpha();
///        if (alpha != 255) {
///          result.setAlpha(
///              FloatValue
///                  .newBuilder()
///                  .setValue(((float) alpha) / denominator)
///                  .build());
///        }
///        return resultBuilder.build();
///      }
///      // ...
///
/// Example (iOS / Obj-C):
///
///      // ...
///      static UIColor* fromProto(Color* protocolor) {
///         float red = [protocolor red];
///         float green = [protocolor green];
///         float blue = [protocolor blue];
///         FloatValue* alpha_wrapper = [protocolor alpha];
///         float alpha = 1.0;
///         if (alpha_wrapper != nil) {
///           alpha = [alpha_wrapper value];
///         }
///         return [UIColor colorWithRed:red green:green blue:blue alpha:alpha];
///      }
///
///      static Color* toProto(UIColor* color) {
///          CGFloat red, green, blue, alpha;
///          if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) {
///            return nil;
///          }
///          Color* result = [[Color alloc] init];
///          [result setRed:red];
///          [result setGreen:green];
///          [result setBlue:blue];
///          if (alpha <= 0.9999) {
///            [result setAlpha:floatWrapperWithValue(alpha)];
///          }
///          [result autorelease];
///          return result;
///     }
///     // ...
///
///  Example (JavaScript):
///
///     // ...
///
///     var protoToCssColor = function(rgb_color) {
///        var redFrac = rgb_color.red || 0.0;
///        var greenFrac = rgb_color.green || 0.0;
///        var blueFrac = rgb_color.blue || 0.0;
///        var red = Math.floor(redFrac * 255);
///        var green = Math.floor(greenFrac * 255);
///        var blue = Math.floor(blueFrac * 255);
///
///        if (!('alpha' in rgb_color)) {
///           return rgbToCssColor(red, green, blue);
///        }
///
///        var alphaFrac = rgb_color.alpha.value || 0.0;
///        var rgbParams = [red, green, blue].join(',');
///        return ['rgba(', rgbParams, ',', alphaFrac, ')'].join('');
///     };
///
///     var rgbToCssColor = function(red, green, blue) {
///       var rgbNumber = new Number((red << 16) | (green << 8) | blue);
///       var hexString = rgbNumber.toString(16);
///       var missingZeros = 6 - hexString.length;
///       var resultBuilder = \['#'\];
///       for (var i = 0; i < missingZeros; i++) {
///          resultBuilder.push('0');
///       }
///       resultBuilder.push(hexString);
///       return resultBuilder.join('');
///     };
///
///     // ...
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    /// The amount of red in the color as a value in the interval [0, 1].
    #[prost(float, tag = "1")]
    pub red: f32,
    /// The amount of green in the color as a value in the interval [0, 1].
    #[prost(float, tag = "2")]
    pub green: f32,
    /// The amount of blue in the color as a value in the interval [0, 1].
    #[prost(float, tag = "3")]
    pub blue: f32,
    /// The fraction of this color that should be applied to the pixel. That is,
    /// the final pixel color is defined by the equation:
    ///
    ///   `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)`
    ///
    /// This means that a value of 1.0 corresponds to a solid color, whereas
    /// a value of 0.0 corresponds to a completely transparent color. This
    /// uses a wrapper message rather than a simple float scalar so that it is
    /// possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is rendered as a solid color
    /// (as if the alpha value had been explicitly given a value of 1.0).
    #[prost(message, optional, tag = "4")]
    pub alpha: ::core::option::Option<f32>,
}
/// Represents a time of day. The date and time zone are either not significant
/// or are specified elsewhere. An API may choose to allow leap seconds. Related
/// types are \[google.type.Date][google.type.Date\] and
/// `google.protobuf.Timestamp`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose
    /// to allow the value "24:00:00" for scenarios like business closing time.
    #[prost(int32, tag = "1")]
    pub hours: i32,
    /// Minutes of hour of day. Must be from 0 to 59.
    #[prost(int32, tag = "2")]
    pub minutes: i32,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may
    /// allow the value 60 if it allows leap-seconds.
    #[prost(int32, tag = "3")]
    pub seconds: i32,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    #[prost(int32, tag = "4")]
    pub nanos: i32,
}
/// A representation of a decimal value, such as 2.5. Clients may convert values
/// into language-native decimal formats, such as Java's \[BigDecimal][\] or
/// Python's \[decimal.Decimal][\].
///
/// \[BigDecimal\]:
/// <https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/math/BigDecimal.html>
/// \[decimal.Decimal\]: <https://docs.python.org/3/library/decimal.html>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decimal {
    /// The decimal value, as a string.
    ///
    /// The string representation consists of an optional sign, `+` (`U+002B`)
    /// or `-` (`U+002D`), followed by a sequence of zero or more decimal digits
    /// ("the integer"), optionally followed by a fraction, optionally followed
    /// by an exponent.
    ///
    /// The fraction consists of a decimal point followed by zero or more decimal
    /// digits. The string must contain at least one digit in either the integer
    /// or the fraction. The number formed by the sign, the integer and the
    /// fraction is referred to as the significand.
    ///
    /// The exponent consists of the character `e` (`U+0065`) or `E` (`U+0045`)
    /// followed by one or more decimal digits.
    ///
    /// Services **should** normalize decimal values before storing them by:
    ///
    ///   - Removing an explicitly-provided `+` sign (`+2.5` -> `2.5`).
    ///   - Replacing a zero-length integer value with `0` (`.5` -> `0.5`).
    ///   - Coercing the exponent character to lower-case (`2.5E8` -> `2.5e8`).
    ///   - Removing an explicitly-provided zero exponent (`2.5e0` -> `2.5`).
    ///
    /// Services **may** perform additional normalization based on its own needs
    /// and the internal decimal implementation selected, such as shifting the
    /// decimal point and exponent value together (example: `2.5e-1` <-> `0.25`).
    /// Additionally, services **may** preserve trailing zeroes in the fraction
    /// to indicate increased precision, but are not required to do so.
    ///
    /// Note that only the `.` character is supported to divide the integer
    /// and the fraction; `,` **should not** be supported regardless of locale.
    /// Additionally, thousand separators **should not** be supported. If a
    /// service does support them, values **must** be normalized.
    ///
    /// The ENBF grammar is:
    ///
    ///     DecimalString =
    ///       \[Sign\] Significand \[Exponent\];
    ///
    ///     Sign = '+' | '-';
    ///
    ///     Significand =
    ///       Digits \['.'\] \[Digits\] | \[Digits\] '.' Digits;
    ///
    ///     Exponent = ('e' | 'E') \[Sign\] Digits;
    ///
    ///     Digits = { '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' };
    ///
    /// Services **should** clearly document the range of supported values, the
    /// maximum supported precision (total number of digits), and, if applicable,
    /// the scale (number of digits after the decimal point), as well as how it
    /// behaves when receiving out-of-bounds values.
    ///
    /// Services **may** choose to accept values passed as input even when the
    /// value has a higher precision or scale than the service supports, and
    /// **should** round the value to fit the supported scale. Alternatively, the
    /// service **may** error with `400 Bad Request` (`INVALID_ARGUMENT` in gRPC)
    /// if precision would be lost.
    ///
    /// Services **should** error with `400 Bad Request` (`INVALID_ARGUMENT` in
    /// gRPC) if the service receives a value outside of the supported range.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Represents civil time (or occasionally physical time).
///
/// This type can represent a civil time in one of a few possible ways:
///
///  * When utc_offset is set and time_zone is unset: a civil time on a calendar
///    day with a particular offset from UTC.
///  * When time_zone is set and utc_offset is unset: a civil time on a calendar
///    day in a particular time zone.
///  * When neither time_zone nor utc_offset is set: a civil time on a calendar
///    day in local time.
///
/// The date is relative to the Proleptic Gregorian Calendar.
///
/// If year is 0, the DateTime is considered not to have a specific year. month
/// and day must have valid, non-zero values.
///
/// This type may also be used to represent a physical time if all the date and
/// time fields are set and either case of the `time_offset` oneof is set.
/// Consider using `Timestamp` message for physical time instead. If your use
/// case also would like to store the user's timezone, that can be done in
/// another field.
///
/// This type is more flexible than some applications may want. Make sure to
/// document and validate your application's limitations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTime {
    /// Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a
    /// datetime without a year.
    #[prost(int32, tag = "1")]
    pub year: i32,
    /// Required. Month of year. Must be from 1 to 12.
    #[prost(int32, tag = "2")]
    pub month: i32,
    /// Required. Day of month. Must be from 1 to 31 and valid for the year and
    /// month.
    #[prost(int32, tag = "3")]
    pub day: i32,
    /// Required. Hours of day in 24 hour format. Should be from 0 to 23. An API
    /// may choose to allow the value "24:00:00" for scenarios like business
    /// closing time.
    #[prost(int32, tag = "4")]
    pub hours: i32,
    /// Required. Minutes of hour of day. Must be from 0 to 59.
    #[prost(int32, tag = "5")]
    pub minutes: i32,
    /// Required. Seconds of minutes of the time. Must normally be from 0 to 59. An
    /// API may allow the value 60 if it allows leap-seconds.
    #[prost(int32, tag = "6")]
    pub seconds: i32,
    /// Required. Fractions of seconds in nanoseconds. Must be from 0 to
    /// 999,999,999.
    #[prost(int32, tag = "7")]
    pub nanos: i32,
    /// Optional. Specifies either the UTC offset or the time zone of the DateTime.
    /// Choose carefully between them, considering that time zone data may change
    /// in the future (for example, a country modifies their DST start/end dates,
    /// and future DateTimes in the affected range had already been stored).
    /// If omitted, the DateTime is considered to be in local time.
    #[prost(oneof = "date_time::TimeOffset", tags = "8, 9")]
    pub time_offset: ::core::option::Option<date_time::TimeOffset>,
}
/// Nested message and enum types in `DateTime`.
pub mod date_time {
    /// Optional. Specifies either the UTC offset or the time zone of the DateTime.
    /// Choose carefully between them, considering that time zone data may change
    /// in the future (for example, a country modifies their DST start/end dates,
    /// and future DateTimes in the affected range had already been stored).
    /// If omitted, the DateTime is considered to be in local time.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TimeOffset {
        /// UTC offset. Must be whole seconds, between -18 hours and +18 hours.
        /// For example, a UTC offset of -4:00 would be represented as
        /// { seconds: -14400 }.
        #[prost(message, tag = "8")]
        UtcOffset(::prost_types::Duration),
        /// Time zone.
        #[prost(message, tag = "9")]
        TimeZone(super::TimeZone),
    }
}
/// Represents a time zone from the
/// [IANA Time Zone Database](<https://www.iana.org/time-zones>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeZone {
    /// IANA Time Zone Database time zone, e.g. "America/New_York".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Optional. IANA Time Zone Database version number, e.g. "2019a".
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
