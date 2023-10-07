use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// 1225  Message function code
#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum _1225 {
    /// Cancellation
    ///
    /// Message cancelling a previous transmission for a given
    /// transaction.
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "3")]
    _3,
    #[serde(rename = "4")]
    _4,
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
    #[serde(rename = "53")]
    _53,
    #[serde(rename = "54")]
    _54,
    #[serde(rename = "55")]
    _55,
    #[serde(rename = "56")]
    _56,
    #[serde(rename = "57")]
    _57,
    #[serde(rename = "58")]
    _58,
    #[serde(rename = "59")]
    _59,
    #[serde(rename = "60")]
    _60,
    #[serde(rename = "61")]
    _61,
    #[serde(rename = "62")]
    _62,
    #[serde(rename = "63")]
    _63,
}

/// 2379  Date or time or period format code
#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display)]
#[strum(serialize_all = "camelCase")]
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
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
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
