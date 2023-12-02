use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Syntax identifier
///
/// Coded identification of the agency controlling a syntax and syntax level used in an interchange.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq, Default)]
pub enum _0001 {
    /// UN/ECE level A
    /// As defined in the basic code table of ISO 646 with the exceptions of lower case letters, alternative graphic character allocations and national or application-oriented graphic character allocations.
    UNOA,
    /// UN/ECE level B
    /// As defined in the basic code table of ISO 646 with the exceptions of alternative graphic character allocations and national or application-oriented graphic character allocations.
    UNOB,
    /// UN/ECE level C
    /// As defined in ISO 8859-1 : Information processing - Part 1: Latin alphabet No. 1.
    #[default]
    UNOC,
    /// UN/ECE level D
    /// As defined in ISO 8859-2 : Information processing - Part 2: Latin alphabet No. 2.
    UNOD,
    /// UN/ECE level E
    /// As defined in ISO 8859-5 : Information processing - Part 5: Latin/Cyrillic alphabet.
    UNOE,
    /// UN/ECE level F
    /// As defined in ISO 8859-7 : Information processing - Part 7: Latin/Greek alphabet.
    UNOF,
}

/// Syntax version number
///
/// Version number of the syntax identified in the syntax identifier (0001)
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq, Default)]
pub enum _0002 {
    /// Version 1
    /// ISO 9735:1988.
    #[strum(serialize = "1")]
    _1,
    /// Version 2
    /// ISO 9735:1990.
    #[strum(serialize = "2")]
    _2,
    /// Version 3
    /// ISO 9735 Amendment 1:1992.
    #[default]
    #[strum(serialize = "3")]
    _3,
}

/// Partner identification code qualifier
///
/// Qualifier referring to the source of codes for the identifiers of interchanging partners.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0007 {
    /// DUNS (Dun & Bradstreet)
    ///
    /// Self explanatory.
    #[strum(serialize = "1")]
    _1,
    /// IATA (International Air Transport Association)
    ///
    /// Self explanatory.
    #[strum(serialize = "4")]
    _4,
    /// INSEE/SIRET
    ///
    /// Self explanatory.
    #[strum(serialize = "5")]
    _5,
    /// UCC Communications ID (Uniform Code Council Communications Identifier)
    ///
    /// The Uniform Code Council Communications Identifier is a ten digit code used to uniquely identify physical and logical locations.
    #[strum(serialize = "8")]
    _8,
    /// DUNS with 4 digit suffix
    ///
    /// Self explanatory.
    #[strum(serialize = "9")]
    _9,
    /// Telephone number
    ///
    /// Self explanatory.
    #[strum(serialize = "12")]
    _12,
    /// EAN (European Article Numbering Association)
    ///
    /// Self explanatory.
    #[strum(serialize = "14")]
    _14,
    /// AIAG (Automotive Industry Action Group)
    ///
    /// Self explanatory.
    #[strum(serialize = "18")]
    _18,
    /// INSEE/SIREN
    ///
    /// Self explanatory.
    #[strum(serialize = "22")]
    _22,
    /// ISO 6523: Organization identification
    ///
    /// Self explanatory.
    #[strum(serialize = "30")]
    _30,
    /// DIN (Deutsches Institut fuer Normung)
    ///
    /// German standardization institute.
    #[strum(serialize = "31")]
    _31,
    /// BfA (Bundesversicherungsanstalt fuer Angestellte)
    ///
    /// German social security association.
    #[strum(serialize = "33")]
    _33,
    /// National Statistical Agency
    ///
    /// Self explanatory.
    #[strum(serialize = "34")]
    _34,
    /// General Electric Information Services
    ///
    /// Self explanatory.
    #[strum(serialize = "51")]
    _51,
    /// IBM Network Services
    ///
    /// Self explanatory.
    #[strum(serialize = "52")]
    _52,
    /// Datenzentrale des Einzelhandels, Germany
    ///
    /// German data centre for retail trade.
    #[strum(serialize = "53")]
    _53,
    /// Bundesverband der Deutschen Baustoffhaendler, Germany
    ///
    /// German building material trade association.
    #[strum(serialize = "54")]
    _54,
    /// Bank identifier code
    ///
    /// Self explanatory.
    #[strum(serialize = "55")]
    _55,
    /// Statens Teleforvaltning
    ///
    /// Norwegian telecommunications regulatory authority (NTRA).
    #[strum(serialize = "56")]
    _56,
    /// KTNet
    ///
    /// Korea Trade Network Services.
    #[strum(serialize = "57")]
    _57,
    /// UPU (Universal Postal Union)
    ///
    /// Self explanatory.
    #[strum(serialize = "58")]
    _58,
    /// ODETTE
    ///
    /// Organization for Data Exchange through Tele-Transmission in Europe (European automotive industry project).
    #[strum(serialize = "59")]
    _59,
    /// SCAC (Standard Carrier Alpha Code)
    ///
    /// Directory of standard multimodal carriers and tariff agent codes. The SCAC lists and codes transportation companies.
    #[strum(serialize = "61")]
    _61,
    /// ECA (Electronic Commerce Australia)
    ///
    /// Australian association for electronic commerce.
    #[strum(serialize = "63")]
    _63,
    /// TELEBOX 400 (Deutsche Bundespost)
    ///
    /// German post office.
    #[strum(serialize = "65")]
    _65,
    /// NHS (National Health Service)
    ///
    /// United Kingdom National Health Service.
    #[strum(serialize = "80")]
    _80,
    /// Athens Chamber of Commerce
    ///
    /// Greek Chamber of Commerce.
    #[strum(serialize = "84")]
    _84,
    /// Swiss Chamber of Commerce
    ///
    /// Swiss Chamber of Commerce.
    #[strum(serialize = "85")]
    _85,
    /// US Council for International Business
    ///
    /// United States Council for International Business.
    #[strum(serialize = "86")]
    _86,
    /// National Federation of Chambers of Commerce and Industry
    ///
    /// Belguim National Federation of Chambers of Commerce andIndustry.
    #[strum(serialize = "87")]
    _87,
    /// Assigned by seller or seller's agent
    ///
    /// Self explanatory.
    #[strum(serialize = "91")]
    _91,
    /// Assigned by buyer or buyer's agent
    ///
    /// Self explanatory.
    #[strum(serialize = "92")]
    _92,
    /// Mutually defined
    ///
    /// Self explanatory.
    #[strum(serialize = "ZZZ")]
    ZZZ,
}

/// Recipient's reference/password qualifier
///
/// Qualifier for the recipient's reference or password.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0025 {
    /// Reference
    /// Self explanatory.
    AA,
    /// Password
    /// Self explanatory.
    BB,
}

/// Processing priority code
///
/// Code determined by the sender requesting processing priority for the interchange.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0029 {
    /// Highest priority
    /// Self explanatory.
    A,
}

/// Acknowledgement request
///
/// Code determined by the sender for acknowledgement of the interchange.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0031 {
    /// Requested
    /// Acknowledgement is requested.
    #[strum(serialize = "1")]
    _1,
}

/// Test indicator
///
/// Indication that the interchange is a test.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0035 {
    /// Interchange is a test
    /// Self explanatory.
    #[strum(serialize = "1")]
    _1,
}

/// 1225  Message function code
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "camelCase")]
pub enum _1225 {
    /// Cancellation
    ///
    /// Message cancelling a previous transmission for a given
    /// transaction.
    #[serde(rename = "1")]
    _1,
    /// Addition
    #[serde(rename = "2")]
    _2,
    /// Deletion
    #[serde(rename = "3")]
    _3,
    /// Change
    #[serde(rename = "4")]
    _4,
    /// Replace
    #[serde(rename = "5")]
    _5,
    #[serde(rename = "6")]
    _6,
    #[serde(rename = "7")]
    _7,
    #[serde(rename = "8")]
    _8,
    #[serde(rename = "9")]
    _9,
    #[serde(rename = "10")]
    _10,
    #[serde(rename = "11")]
    _11,
    #[serde(rename = "12")]
    _12,
    #[serde(rename = "13")]
    _13,
    #[serde(rename = "14")]
    _14,
    #[serde(rename = "15")]
    _15,
    #[serde(rename = "16")]
    _16,
    #[serde(rename = "17")]
    _17,
    #[serde(rename = "18")]
    _18,
    #[serde(rename = "19")]
    _19,
    #[serde(rename = "20")]
    _20,
    #[serde(rename = "21")]
    _21,
    #[serde(rename = "22")]
    _22,
    #[serde(rename = "23")]
    _23,
    #[serde(rename = "24")]
    _24,
    #[serde(rename = "25")]
    _25,
    #[serde(rename = "26")]
    _26,
    #[serde(rename = "27")]
    _27,
    #[serde(rename = "28")]
    _28,
    #[serde(rename = "29")]
    _29,
    #[serde(rename = "30")]
    _30,
    #[serde(rename = "31")]
    _31,
    #[serde(rename = "32")]
    _32,
    #[serde(rename = "33")]
    _33,
    #[serde(rename = "34")]
    _34,
    #[serde(rename = "35")]
    _35,
    #[serde(rename = "36")]
    _36,
    #[serde(rename = "37")]
    _37,
    #[serde(rename = "38")]
    _38,
    #[serde(rename = "39")]
    _39,
    #[serde(rename = "40")]
    _40,
    #[serde(rename = "41")]
    _41,
    #[serde(rename = "42")]
    _42,
    #[serde(rename = "43")]
    _43,
    #[serde(rename = "44")]
    _44,
    #[serde(rename = "45")]
    _45,
    #[serde(rename = "46")]
    _46,
    #[serde(rename = "47")]
    _47,
    #[serde(rename = "48")]
    _48,
    #[serde(rename = "49")]
    _49,
    #[serde(rename = "50")]
    _50,
    #[serde(rename = "51")]
    _51,
    #[serde(rename = "52")]
    _52,
    /// 53 Test
    ///
    /// Code indicating the message is to be considered as a
    /// test.
    #[serde(rename = "53")]
    _53,
}

/// 2379  Date or time or period format code
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum _2379 {
    /// DDMMYY
    ///
    /// Calendar date: D = Day; M = Month; Y = Year.
    #[serde(rename = "2")]
    _2,
    /// MMDDYY
    ///
    /// Calendar date: M = Month; D = Day; Y = Year.
    #[serde(rename = "3")]
    _3,
    /// DDMMCCYY
    ///
    /// Calendar date C=Century; Y=Year; M=Month; D=Day.
    _4,
    /// DDMMCCYYHHMM
    ///
    ///  Calendar date and time: C=Century; Y=Year; M=Month;
    /// D=Day; H=Hour; M=Minute.
    _5,
    /// YYMMDD
    ///
    /// Calendar date: Y = Year; M = Month; D = Day.
    _101,
    /// CCYYMMDD
    ///
    /// Calendar date: C = Century ; Y = Year ; M = Month ; D =
    /// Day.
    _102,
    /// YYWWD
    ///
    /// Calendar week day: Y = Year ; W = Week ; D = Day Week
    /// number 01 is always first week of January Day number 1
    /// is always Monday.
    _103,
    /// YYDDD
    ///
    /// Calendar day: Y = Year ; D = Day January the first = Day
    /// 001 Always start numbering the days of the year from
    /// January 1st through December 31st.
    _105,
    /// MMDD
    ///
    /// Day of a month: M = Month; D = Day.
    #[serde(rename = "106")]
    _106,
    /// DDD
    ///
    /// Day's number within a specific year: D = Day.
    #[serde(rename = "107")]
    _107,
    /// WW
    ///
    /// Week's number within a specific year: W = Week.
    #[serde(rename = "108")]
    _108,
    /// MM
    ///
    /// Month's number within a specific year: M = Month.
    #[serde(rename = "109")]
    _109,
    /// DD
    ///
    /// Day's number within is a specific month: D = Day.
    #[serde(rename = "110")]
    _110,
    /// YYMMDDHHMM
    ///
    /// Calendar date including time without seconds: Y = Year;
    /// M = Month; D = Day; H = Hour; M = Minute.
    #[serde(rename = "201")]
    _201,
    /// YYMMDDHHMMSS
    ///
    /// Calendar date including time with seconds: Y = Year; M =
    /// Month; D = Day; H = Hour; m = Minutes = Seconds.
    #[serde(rename = "202")]
    _202,
    /// CCYYMMDDHHMM
    ///
    /// Calendar date including time with minutes: C=Century;
    /// Y=Year; M=Month; D=Day; H=Hour; M=Minutes.
    #[serde(rename = "203")]
    _203,
    /// CCYYMMDDHHMMSS
    ///
    /// Calendar date including time with seconds:
    /// C=Century;Y=Year;
    /// M=Month;D=Day;H=Hour;M=Minute;S=Second.
    _204,
    /// CCYYMMDDHHMMZHHMM
    ///
    /// Calendar date including time and time zone expressed in
    /// hours and minutes.
    /// ZHHMM = time zone given as offset from Coordinated
    /// Universal Time (UTC).
    _205,

    /// YYMMDDHHMMZZZ
    ///
    /// See 201 + Z = Time zone.
    _301,
    /// YYMMDDHHMMSSZZZ
    ///
    /// See 202 + Z = Time zone.
    _302,
    /// CCYYMMDDHHMMZZZ
    ///
    /// See 203 plus Z=Time zone.
    _303,

    /// CCYYMMDDHHMMSSZZZ
    ///
    /// See 204 plus Z=Time zone.
    _304,
    /// MMDDHHMM
    ///
    /// Month, day, hours, minutes; M = Month; D = Day; H =
    /// Hour; M = Minute.
    _305,
    /// DDHHMM
    ///
    /// Day, hours, minutes; D = Day; H = Hour; M = Minute.
    _306,
    /// HHMM
    ///
    /// Time without seconds: H = Hour; m = Minute.
    _401,
    /// HHMMSS
    ///
    /// Time with seconds: H = Hour; m = Minute; s = Seconds.
    _402,
    /// HHMMSSZZZ
    ///
    /// See 402 plus Z=Time zone.
    _404,
    /// MMMMSS
    ///
    /// Time without hours: m=minutes, s=seconds.
    _405,
    /// ZHHMM
    ///
    /// Offset from Coordinated Universal Time (UTC) where Z is
    /// plus (+) or minus (-).
    _406,
    /// HHMMHHMM
    ///
    /// Time span without seconds: H = Hour; m = Minute;.
    _501,
    /// HHMMSS-HHMMSS
    ///
    /// Format of period to be given without hyphen.
    _502,
    /// HHMMSSZZZ-HHMMSSZZZ
    ///
    /// Format of period to be given without hyphen.
    _503,
    ///   CC
    ///              Century.
    _600,
    ///   YY
    ///              Calendar year: Y = Year.
    _601,
    ///   CCYY
    ///              Calendar year including century: C = Century; Y = Year.
    _602,
    ///   YYS
    ///              Semester in a calendar year: Y = Year; S = Semester.
    _603,
    /// CCYYS
    ///
    /// Semester in a calendar year: C = Century; Y = Year; S =
    /// Semester.
    _604,
    /// CCYYQ
    ///
    /// Quarter in a calendar year: C = Century; Y = Year; Q =
    /// Quarter.
    _608,
    /// YYMM
    ///
    /// Month within a calendar year: Y = Year; M = Month.
    _609,
    /// CCYYMM
    ///
    /// Month within a calendar year: CC = Century; Y = Year; M
    /// = Month.
    _610,

    /// YYMMA
    ///
    /// Format of period to be given without hyphen (A = ten
    ///     days period).
    _613,

    /// CCYYMMA
    ///
    /// Format of period to be given without hyphen (A = ten
    ///       days period).
    _614,

    /// YYWW
    /// Week within a calendar year: Y = Year; W = Week 1st week
    ///       of January = week 01.
    _615,
    /// CCYYWW
    ///
    /// Week within a calendar year: CC = Century; Y = Year; W =
    /// Week (1st week of January = week 01).
    _616,
    /// YY-YY
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _701,
    /// CCYY-CCYY
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _702,
    /// YYS-YYS
    ///
    /// Format of period to be given without hyphen.
    _703,
    /// CCYYS-CCYYS
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _704,
    /// YYPYYP
    ///
    /// Format of period to be given without hyphen (P = period
    /// of 4 months).
    _705,
    /// CCYYP-CCYYP
    ///
    /// Format of period to be given without hyphen (P = period
    /// of 4 months).
    _706,
    /// YYQ-YYQ
    ///
    /// Format of period to be given without hyphen.
    _707,
    /// CCYYQ-CCYYQ
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _708,
    /// YYMM-YYMM
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _709,
    /// CCYYMM-CCYYMM
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    #[serde(rename = "710")]
    _710,
    ///   CCYYMMDD-CCYYMMDD
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _711,
    /// YYMMDDHHMM-YYMMDDHHMM
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _713,
    /// YYWW-YYWW
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _715,
    /// CCYYWW-CCYYWW
    ///
    /// Format of period to be given without hyphen.
    _716,
    /// YYMMDD-YYMMDD
    ///
    /// Format of period to be given in actual message without
    /// hyphen.
    _717,
    /// CCYYMMDD-CCYYMMDD
    ///
    /// Format of period to be given without hyphen.
    _718,
    /// CCYYMMDDHHMM-CCYYMMDDHHMM
    ///
    /// A period of time which includes the century, year,
    /// month, day, hour and minute. Format of period to be
    /// given in actual message without hyphen.
    _719,
    /// DHHMM-DHHMM
    ///
    /// Format of period to be given without hyphen (D=day of
    /// the week, 1=Monday; 2=Tuesday; ... 7=Sunday).
    _720,
    /// Year
    ///
    /// To indicate a quantity of years.
    _901,
    /// Month
    ///
    /// To indicate a quantity of months.
    _902,
    /// Week
    ///
    /// To indicate a quantity of weeks.
    _903,
    /// Day
    ///
    /// To indicate a quantity of days.
    _904,
    /// Hour
    ///
    /// To indicate a quantity of hours.
    _905,
    /// Minute
    ///
    /// To indicate a quantity of minutes.
    _906,
    /// Second
    ///
    /// To indicate a quantity of seconds.
    #[serde(rename = "807")]
    _807,
    /// Semester
    ///
    /// To indicate a quantity of semesters (six months).
    _808,
    /// Four months period
    ///
    /// To indicate a quantity of four months periods.
    #[serde(rename = "809")]
    _809,
    /// Trimester
    ///
    /// To indicate a quantity of trimesters (three months).
    _810,
    /// Half month
    ///
    /// To indicate a quantity of half months.
    #[serde(rename = "811")]
    _811,
    /// Ten days
    ///
    /// To indicate a quantity of ten days periods.
    #[serde(rename = "812")]
    _812,
    /// Day of the week
    ///
    /// Numeric representation of the day (Monday = 1).
    #[serde(rename = "813")]
    _813,
    /// Working days
    ///
    /// Number of working days.
    #[serde(rename = "814")]
    _814,
}

/// 4343  Response type code
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, EnumString, Display)]
pub enum _4343 {
    AA,
    AB,
    AC,
    AD,
    AE,
    AF,
    AG,
    AH,
    AI,
    AJ,
    AP,
    AQ,
    AR,
    AS,
    CA,
    CO,
    NA,
    RE,
    UR,
    US,
}
