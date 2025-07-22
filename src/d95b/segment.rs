use super::*;
use crate::util::{clean_num, Parser};
use edifact_types_macros::{DisplayOuterSegment, ParseSegment};
use nom::{bytes::complete::take_until, character::complete::not_line_ending, IResult};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    str::FromStr,
};
/// BGM - BEGINNING OF MESSAGE
///
/// To indicate the type and function of a message and to
/// transmit the identifying number.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct BGM {
    pub _010: Option<C002>,
    /// 1004 - Document/message number
    ///
    /// Reference number assigned to the document/message by the issuer.
    /// an..35
    pub _020: Option<String>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

/// CNT - CONTROL TOTAL
///
/// To provide control total.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct CNT {
    /// CONTROL
    pub _010: C270,
}

/// CTA    CONTACT INFORMATION
///
/// To identify a person or a department to whom
/// communication should be directed.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct CTA {
    /// CONTACT FUNCTION, CODED
    ///
    /// C  an..3
    pub _010: Option<String>,
    /// DEPARTMENT OR EMPLOYEE DETAILS
    pub _020: Option<C056>,
}

/// DGS - DANGEROUS GOODS
///
/// To identify dangerous goods.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct DGS {
    /// DANGEROUS GOODS REGULATIONS, CODED
    ///
    /// C  an..3
    pub _010: Option<String>,
    /// HAZARD CODE
    pub _020: Option<C205>,
    /// UNDG INFORMATION
    pub _030: Option<C234>,
    /// DANGEROUS GOODS SHIPMENT FLASHPOINT
    pub _040: Option<C223>,
    /// PACKING GROUP, CODED
    ///
    /// C  an..3
    pub _050: Option<String>,
    /// EMS NUMBER
    ///
    /// C  an..6
    pub _060: Option<String>,
    /// MFAG
    ///
    /// C  an..4
    pub _070: Option<String>,
    /// TREM CARD NUMBER
    ///
    /// C  an..10
    pub _080: Option<String>,
    /// HAZARD IDENTIFICATION
    pub _090: Option<C235>,
    /// DANGEROUS GOODS LABEL
    pub _100: Option<C236>,
    /// PACKING INSTRUCTION, CODED
    ///
    /// C  an..3
    pub _110: Option<String>,
    /// CATEGORY OF MEANS OF TRANSPORT, CODED
    ///
    /// C  an..3
    pub _120: Option<String>,
    /// PERMISSION FOR TRANSPORT, CODED
    ///
    /// C  an..3
    pub _130: Option<String>,
}

/// DIM - DIMENSIONS
///
/// To specify dimensions.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct DIM {
    /// DIMENSION QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// DIMENSIONS
    pub _020: C211,
}

/// DTM - DATE/TIME/PERIOD
///
/// To specify date, and/or time, or period.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct DTM {
    /// DATE/TIME/PERIOD
    pub _010: C507,
}

/// EQA - ATTACHED EQUIPMENT
///
/// To specify attached or related equipment.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct EQA {
    /// EQUIPMENT QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// EQUIPMENT IDENTIFICATION
    pub _020: Option<C237>,
}

/// EQD - EQUIPMENT DETAILS
///
/// To identify a unit of equipment.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct EQD {
    /// EQUIPMENT QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// EQUIPMENT IDENTIFICATION
    pub _020: Option<C237>,
    /// EQUIPMENT SIZE AND TYPE
    pub _030: Option<C224>,
    /// EQUIPMENT SUPPLIER, CODED
    ///
    /// C  an..3
    pub _040: Option<String>,
    /// EQUIPMENT STATUS, CODED
    ///
    /// C  an..3
    pub _050: Option<String>,
    /// FULL/EMPTY INDICATOR, CODED
    ///
    /// C  an..3
    pub _060: Option<String>,
}

/// EQN - NUMBER OF UNITS
///
/// To specify the number of units.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct EQN {
    /// NUMBER OF UNIT DETAILS
    pub _010: C523,
}

/// FTX - Free Text
///
/// To provide free form or coded text information.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct FTX {
    /// TEXT SUBJECT QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// TEXT FUNCTION, CODED
    ///
    /// C  an..3
    pub _020: Option<String>,
    pub _030: Option<C107>,
    pub _040: Option<C108>,
    /// LANGUAGE, CODED
    ///
    /// C  an..3
    pub _050: Option<String>,
}

/// MEA - MEASUREMENTS
///
/// To specify physical measurements, including dimension
/// tolerances, weights and counts.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct MEA {
    /// MEASUREMENT APPLICATION QUALIFIER                     M  an..3
    pub _010: String,
    /// MEASUREMENT DETAILS                                   C  
    pub _020: Option<C502>,
    /// VALUE/RANGE                                           C  
    pub _030: Option<C174>,
    /// SURFACE/LAYER INDICATOR, CODED                        C  an..3
    pub _040: Option<String>,
}

/// LOC - PLACE/LOCATION IDENTIFICATION
///
/// To identify a country/place/location/related location
/// one/related location two.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct LOC {
    pub _010: String,
    pub _020: Option<C517>,
    pub _030: Option<C519>,
    pub _040: Option<C553>,
    /// RELATION, CODED
    ///
    /// C  an..3
    pub _050: Option<String>,
}

/// NAD - NAME AND ADDRESS
///
/// To specify the name/address and their related
/// function, either by CO82 only and/or unstructured by
/// CO58 or structured by CO80 thru 3207.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct NAD {
    pub _010: String,
    pub _020: Option<C082>,
    pub _030: Option<C058>,
    pub _040: Option<C080>,
    pub _050: Option<C059>,
    pub _060: Option<String>,
    pub _070: Option<String>,
    pub _080: Option<String>,
    pub _090: Option<String>,
}

/// RFF - REFERENCE
///
/// To specify a reference.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct RFF {
    // REFERENCE
    pub _010: C506,
}

/// RNG - RANGE DETAILS
///
/// To identify a range.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct RNG {
    /// RANGE TYPE QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// RANGE
    pub _020: Option<C280>,
}
/// SEL - SEAL NUMBER
///
/// To specify a seal number related to equipment.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct SEL {
    /// SEAL NUMBER
    ///
    /// M  an..10
    pub _010: String,
    /// SEAL ISSUER
    pub _020: Option<C215>,
    /// SEAL CONDITION, CODED
    ///
    /// C  an..3
    pub _030: Option<String>,
}

/// TDT - DETAILS OF TRANSPORT
///
/// To specify the transport details such as mode of
/// transport, means of transport, its conveyance
/// reference number and the identification of the means
/// of transport.
/// The segment may be pointed to by the TPL segment.
#[derive(Default, Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
pub struct TDT {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<C220>,
    pub _040: Option<C228>,
    pub _050: Option<C040>,
    pub _060: Option<String>,
    pub _070: Option<C401>,
    pub _080: Option<C222>,
    pub _090: Option<String>,
}

/// TMD - TRANSPORT MOVEMENT DETAILS
///
/// To specify transport movement details for a goods item
/// or equipment.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct TMD {
    /// MOVEMENT TYPE
    pub _010: Option<C219>,
    /// EQUIPMENT PLAN
    ///
    /// C  an..26
    pub _020: Option<String>,
    /// HAULAGE ARRANGEMENTS, CODED
    ///
    /// C  an..3
    pub _030: Option<String>,
}

/// TMP - TEMPERATURE
///
/// To specify the temperature setting.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct TMP {
    /// TEMPERATURE QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// TEMPERATURE SETTING
    pub _020: Option<C239>,
}

/// UNA, Service String advice
///
/// Function: To define the characters selected for use
/// as delimiters and indicators in the rest of the
/// interchange that follows:
///
/// The specifications in the Service string advice take
/// precedence over the specifications for delimiters etc. in
/// segment UNB.  See clause 4.
///
/// When transmitted, the Service string advice must appear
/// immediately before the Interchange Header (UNB) segment and
/// begin with the upper case characters UNA immediately followed
/// by the six characters selected by the sender to indicate, in
/// sequence, the following functions:
/// Repr. | Req. | Name | Remarks
/// --- | --- | --- | ---
/// an1 | M | COMPONENT DATA ELEMENT SEPARATOR |
/// an1 | M | DATA ELEMENT SEPARATOR |
/// an1 | M | DECIMAL NOTATION | Comma or full stop
/// an1 | M | RELEASE INDICATOR | If not used, insert space character
/// an1 | M | Reserved for future use | Insert space character
/// an1 | M | SEGMENT TERMINATOR |
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayOuterSegment)]
pub struct UNA {
    /// an1    M     COMPONENT DATA ELEMENT SEPARATOR
    pub component_data_element_seperator: char,
    /// an1    M     DATA ELEMENT SEPARATOR
    pub data_element_seperator: char,
    /// an1    M     DECIMAL NOTATION       Comma or full stop
    pub decimal_notation: char,
    /// an1    M     RELEASE INDICATOR      If not used, insert space character
    pub release_indicator: char,
    /// an1    M     Reserved for future use    Insert space character
    pub reserved_for_future_use: char,
    /// an1    M     SEGMENT TERMINATOR
    pub segment_terminator: char,
}

impl<'a> Parser<&'a str, UNA, nom::error::Error<&'a str>> for UNA {
    fn parse(input: &'a str) -> IResult<&'a str, UNA> {
        let (rest, vars) = take_until("UNB")(input)?;
        if vars.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                rest,
                nom::error::ErrorKind::TakeUntil,
            )));
        }
        // look for trailing newline
        let vars = not_line_ending(vars)?.1;
        if vars.len() != 9 {
            println!("UNA Segment found, but malformed:\n{vars:?}");
            panic!("UNA Segment malformed, needs to be exactly 6 characters")
        }
        let vars = vars.strip_prefix("UNA").unwrap();
        let mut vars = vars.chars();
        let una = UNA {
            component_data_element_seperator: vars.next().unwrap(),
            data_element_seperator: vars.next().unwrap(),
            decimal_notation: vars.next().unwrap(),
            release_indicator: vars.next().unwrap(),
            reserved_for_future_use: vars.next().unwrap(),
            segment_terminator: vars.next().unwrap(),
        };
        Ok((rest, una))
    }
}

/// UNB Interchange header
///
/// To start, identify and specify an interchange.
#[derive(
    Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, DisplayOuterSegment, ParseSegment,
)]
pub struct UNB {
    _010: S001,
    _020: S002,
    _030: S003,
    _040: S004,
    /// Interchange control reference
    ///
    /// Unique reference assigned by the sender to an interchange.
    /// Shall be identical in UNB and UNZ.
    _050: String,
    _060: Option<S005>,
    _070: Option<String>,
    _080: Option<_0029>,
    _090: Option<_0031>,
    _100: Option<String>,
    _110: Option<_0035>,
}

/// UNH - MESSAGE HEADER
///
/// To head, identify and specify a message.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct UNH {
    /// MESSAGE REFERENCE NUMBER
    ///
    /// M  an..14
    pub _010: String,
    /// MESSAGE IDENTIFIER
    pub _020: Option<S009>,
    /// COMMON ACCESS REFERENCE
    ///
    /// C  an..35
    pub _030: Option<String>,
    /// STATUS OF THE TRANSFER
    pub _040: Option<S010>,
}

/// UNT - MESSAGE TRAILER
///
/// To end and check the completeness of a message.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct UNT {
    /// NUMBER OF SEGMENTS IN THE MESSAGE
    ///
    /// M  n..6
    pub _010: String,
    /// MESSAGE REFERENCE NUMBER
    ///
    /// M  an..14
    pub _020: String,
}

/// GDS - NATURE OF CARGO
///
/// To identify or specify the goods carried.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct GDS {
    /// NATURE OF CARGO
    ///
    /// M  an..35
    pub _010: String,
}

/// GID - GOODS ITEM DETAILS
///
/// To identify the number of packages of the goods.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayOuterSegment, ParseSegment)]
pub struct GID {
    /// GOODS ITEM NUMBER
    ///
    /// M  n..5
    pub _010: String,
    /// NUMBER AND TYPE OF PACKAGES
    pub _020: Option<C213>,
    /// CONSIGNMENT LOAD SEQUENCE NUMBER
    ///
    /// C  n..6
    pub _030: Option<String>,
}

/// UNZ Interchange trailer
///
/// To end and check the completeness of an interchange.
#[derive(
    Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayOuterSegment, ParseSegment,
)]
pub struct UNZ {
    /// Interchange control count
    ///
    /// The count either of the number of messages or, if used, of the number of functional groups in an interchange. One of these counts shall appear.
    _010: String,
    /// Interchange control reference
    ///
    /// Unique reference assigned by the sender to an interchange.
    /// Shall be identical in UNB and UNZ.
    _020: String,
}
