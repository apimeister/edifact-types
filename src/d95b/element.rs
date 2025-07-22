use super::*;
use crate::util::{clean_num, Parser};
use edifact_types_macros::{DisplayInnerSegment, ParseElement};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

/// C002 - DOCUMENT/MESSAGE NAME
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C002 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

/// C040 - CARRIER
///
/// Identification of a carrier by code and/or by name. Code
/// preferred.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C040 {
    /// Carrier identification
    ///
    /// C  an..17
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Carrier name
    ///
    /// C  an..35
    pub _040: Option<String>,
}

/// C056 - DEPARTMENT OR EMPLOYEE DETAILS
///
/// Code and/or name of a department or employee. Code
/// preferred.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C056 {
    /// Department or employee identification
    ///
    /// C  an..17
    pub _010: Option<String>,
    /// Department or employee
    ///
    /// C  an..35
    pub _020: Option<String>,
}

/// C058 - NAME AND ADDRESS
///
/// Unstructured name and address: one to five lines.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C058 {
    /// Name and address line
    ///
    /// M  an..35
    pub _010: String,
    /// Name and address line
    ///
    /// C  an..35
    pub _020: Option<String>,
    /// Name and address line
    ///
    /// C  an..35
    pub _030: Option<String>,
    /// Name and address line
    ///
    /// C  an..35
    pub _040: Option<String>,
    /// Name and address line
    ///
    /// C  an..35
    pub _050: Option<String>,
}

/// C059 - STREET
///
/// Street address and/or PO Box number in a structured
/// address: one to three lines.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C059 {
    /// Street and number/p.o. box
    ///
    /// M  an..35
    pub _010: String,
    /// Street and number/p.o. box
    ///
    /// C  an..35
    pub _020: Option<String>,
    /// Street and number/p.o. box
    ///
    /// C  an..35
    pub _030: Option<String>,
}

/// C080 - PARTY NAME
///
/// Identification of a transaction party by name, one to five
/// lines. Party name may be formatted.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C080 {
    /// Party name
    ///
    /// M  an..35
    pub _010: String,
    /// Party name
    ///
    /// C  an..35
    pub _020: Option<String>,
    /// Party name
    ///
    /// C  an..35
    pub _030: Option<String>,
    /// Party name
    ///
    /// C  an..35
    pub _040: Option<String>,
    /// Party name
    ///
    /// C  an..35
    pub _050: Option<String>,
    /// Party name format, coded
    ///
    /// C  an..3
    pub _060: Option<String>,
}

/// C082  PARTY IDENTIFICATION DETAILS
///
/// Identification of a transaction party by code.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C082 {
    /// Party id. identification
    ///
    /// M  an..35
    pub _010: String,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
}

/// C107 - TEXT REFERENCE
///
/// Coded reference to a standard text and its source.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C107 {
    /// Free text, coded
    ///
    /// M  an..3
    pub _010: String,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
}

/// C108 - TEXT LITERAL
///
/// Free text; one to five lines.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C108 {
    /// Free text
    ///
    /// M  an..70
    pub _010: String,
    /// Free text
    ///
    /// C  an..70
    pub _020: Option<String>,
    /// Free text
    ///
    /// C  an..70
    pub _030: Option<String>,
    /// Free text
    ///
    /// C  an..70
    pub _040: Option<String>,
    /// Free text
    ///
    /// C  an..70
    pub _050: Option<String>,
}

/// C174 - VALUE/RANGE
///
/// Measurement value and relevant minimum and maximum
/// tolerances in that order.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C174 {
    /// Measure unit qualifier
    ///
    /// M  an..3
    pub _010: String,
    /// Measurement value
    ///
    /// C  n..18
    pub _020: Option<String>,
    /// Range minimum
    ///
    /// C  n..18
    pub _030: Option<String>,
    /// Range maximum
    ///
    /// C  n..18
    pub _040: Option<String>,
    /// Significant digits
    ///
    /// C  n..2
    pub _050: Option<String>,
}

/// C205 - HAZARD CODE
///
/// The identification of the dangerous goods in code.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C205 {
    /// Hazard code identification                        M  an..7
    pub _010: String,
    /// Hazard substance/item/page number                 C  an..7
    pub _020: Option<String>,
    /// Hazard code version number                        C  an..10
    pub _030: Option<String>,
}

/// C211 - DIMENSIONS
///
/// Specification of the dimensions of a transportable unit.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C211 {
    /// Measure unit qualifier
    ///
    /// M  an..3
    pub _010: String,
    /// Length dimension
    ///
    /// C  n..15
    pub _020: Option<String>,
    /// Width dimension
    ///
    /// C  n..15
    pub _030: Option<String>,
    /// Height dimension
    ///
    /// C  n..15
    pub _040: Option<String>,
}

/// C215 - SEAL ISSUER
///
/// Identification of the issuer of a seal on equipment either
/// by code or by name.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C215 {
    /// Sealing party, coded
    ///
    /// C  an..3
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Sealing party
    ///
    /// C  an..35
    pub _040: Option<String>,
}

/// C219 - MOVEMENT TYPE
///
/// Description of type of service for movement of cargo.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C219 {
    /// Movement type, coded
    ///
    /// C  an..3
    pub _010: Option<String>,
    /// Movement type
    ///
    /// C  an..35
    pub _020: Option<String>,
}

/// C220 - MODE OF TRANSPORT
///
/// Method of transport code or name. Code preferred.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C220 {
    /// Mode of transport, coded
    ///
    /// C  an..3
    pub _010: Option<String>,
    /// Mode of transport
    ///
    /// C  an..17
    pub _020: Option<String>,
}

/// C222 - TRANSPORT IDENTIFICATION
///
/// Code and/or name identifying the means of transport.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C222 {
    /// Id. of means of transport identification
    ///
    /// C  an..9
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Id. of the means of transport
    ///
    /// C  an..35
    pub _040: Option<String>,
    /// Nationality of means of transport, coded
    ///
    /// C  an..3
    pub _050: Option<String>,
}

/// C223 - DANGEROUS GOODS SHIPMENT FLASHPOINT
///
/// Temperature at which a vapor according to ISO 1523/73 can
/// be ignited.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C223 {
    /// Shipment flashpoint
    ///
    /// C  n3
    pub _010: Option<String>,
    /// Measure unit qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
}

/// C224 - EQUIPMENT SIZE AND TYPE
///
/// Code and/or name identifying size and type of equipment
/// used in transport. Code preferred.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C224 {
    /// Equipment size and type identification
    ///
    /// C  an..10
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Equipment size and type
    ///
    /// C  an..35
    pub _040: Option<String>,
}

/// C228 - TRANSPORT MEANS
///
/// Code and/or name identifying the type of means of
/// transport.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C228 {
    /// Type of means of transport identification
    ///
    /// C  an..8
    pub _010: Option<String>,
    /// Type of means of transport
    ///
    /// C  an..17
    pub _020: Option<String>,
}

/// C234 - UNDG INFORMATION
///
/// Information on United Nations Dangerous Goods
/// classification.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C234 {
    /// UNDG number
    ///
    /// C  n4
    pub _010: Option<String>,
    /// Dangerous goods flashpoint
    ///
    /// C  an..8
    pub _020: Option<String>,
}

/// C235 - HAZARD IDENTIFICATION
///
/// Identification of the Orange placard required on the means
/// of transport.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C235 {
    /// Hazard identification number, upper part
    ///
    /// C  an..4
    pub _010: Option<String>,
    /// Substance identification number, lower part
    ///
    /// C  an4
    pub _020: Option<String>,
}

/// C236 - DANGEROUS GOODS LABEL
///
/// Markings identifying the type of hazardous goods and
/// similar information.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C236 {
    /// Dangerous goods label marking                     C  an..4
    pub _010: Option<String>,
    /// Dangerous goods label marking                     C  an..4
    pub _020: Option<String>,
    /// Dangerous goods label marking                     C  an..4
    pub _030: Option<String>,
}

/// C237 - EQUIPMENT IDENTIFICATION
///
/// Marks (letters and/or numbers) identifying equipment used
/// for transport such as a container.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C237 {
    /// Equipment identification number
    ///
    /// C  an..17
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Country, coded
    ///
    /// C  an..3
    pub _040: Option<String>,
}

/// C239 - TEMPERATURE SETTING
///
/// The temperature under which the goods are (to be) stored
/// or shipped.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C239 {
    /// Temperature setting                               C  n3
    pub _010: Option<String>,
    /// Measure unit qualifier                            C  an..3
    pub _020: Option<String>,
}

/// C270 - CONTROL
///
/// Control total for checking integrity of a message or part
/// of a message.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C270 {
    /// Control qualifier
    ///
    /// M  an..3
    pub _010: String,
    /// Control value
    ///
    /// M  n..18
    pub _020: String,
    /// Measure unit qualifier
    ///
    /// C  an..3
    pub _030: Option<String>,
}

/// C280 - RANGE
///
/// Range minimum and maximum limits.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C280 {
    /// Measure unit qualifier                            M  an..3
    pub _010: String,
    /// Range minimum                                     C  n..18
    pub _020: Option<String>,
    /// Range maximum                                     C  n..18
    pub _030: Option<String>,
}

/// C401 - EXCESS TRANSPORTATION INFORMATION
///
/// To provide details of reason for, and responsibility for,
/// use of transportation other than normally utilized.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C401 {
    /// Excess transportation reason, coded
    ///
    /// M  an..3
    pub _010: String,
    /// Excess transportation responsibility, coded
    ///
    /// M  an..3
    pub _020: String,
    /// Customer authorization number
    ///
    /// C  an..17
    pub _030: Option<String>,
}

/// C502 - MEASUREMENT DETAILS
///
/// Identification of measurement type.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C502 {
    /// Measurement dimension, coded                      C  an..3
    pub _010: Option<String>,
    /// Measurement significance, coded                   C  an..3
    pub _020: Option<String>,
    /// Measurement attribute, coded                      C  an..3
    pub _030: Option<String>,
    /// Measurement attribute                             C  an..70
    pub _040: Option<String>,
}

/// C506 - REFERENCE
///
/// Identification of a reference.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C506 {
    /// Reference qualifier
    ///
    /// M  an..3
    pub _010: String,
    /// Reference number
    ///
    /// C  an..35
    pub _020: Option<String>,
    /// Line number
    ///
    /// C  an..6
    pub _030: Option<String>,
    /// Reference version number
    ///
    /// C  an..35
    pub _040: Option<String>,
}

/// C507 - DATE/TIME/PERIOD
///
/// Date and/or time, or period relevant to the specified
/// date/time/period type.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C507 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C517 - LOCATION IDENTIFICATION
///
/// Identification of a location by code or name.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C517 {
    /// Place/location identification
    ///
    /// C  an..25
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Place/location
    ///
    /// C  an..17
    pub _040: Option<String>,
}

/// C519 - RELATED LOCATION ONE IDENTIFICATION
///
/// Identification the first related location by code or name.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C519 {
    /// Related place/location one identification
    ///
    /// C  an..25
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Related place/location one
    ///
    /// C  an..70
    pub _040: Option<String>,
}

/// C523 - NUMBER OF UNIT DETAILS
///
/// Identification of number of units and its purpose.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C523 {
    /// Number of units                                   C  n..15
    pub _010: Option<String>,
    /// Number of units qualifier                         C  an..3
    pub _020: Option<String>,
}

/// C553 - RELATED LOCATION TWO IDENTIFICATION
///
/// Identification of second related location by code or name.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C553 {
    /// Related place/location two identification
    ///
    /// C  an..25
    pub _010: Option<String>,
    /// Code list qualifier
    ///
    /// C  an..3
    pub _020: Option<String>,
    /// Code list responsible agency, coded
    ///
    /// C  an..3
    pub _030: Option<String>,
    /// Related place/location two
    ///
    /// C  an..70
    pub _040: Option<String>,
}

/// Syntax identifier
///
/// Identification of the agency controlling the syntax and indication of syntax level, plus the syntax version number.
#[derive(
    Debug, Serialize, Deserialize, Clone, DisplayInnerSegment, ParseElement, PartialEq, Eq, Default,
)]
pub struct S001 {
    pub _010: _0001,
    pub _020: _0002,
}

/// Interchange sender
///
/// Identification of the sender of the interchange.
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, ParseElement, PartialEq, Eq,
)]
pub struct S002 {
    /// Sender identification
    ///
    /// Name or coded representation of the sender of a data interchange.
    /// Code or name as specified in IA.
    pub _010: String,
    pub _020: Option<_0007>,
    /// Address for reverse routing
    ///
    /// Address specified by the sender of an interchange to be included by the recipient in the response interchanges to facilitate internal routing.
    pub _030: Option<String>,
}

/// Interchange recipient
///
/// Identification of the recipient of the interchange.
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, ParseElement, PartialEq, Eq,
)]
pub struct S003 {
    /// Recipient identification
    ///
    /// Name or coded representation of the recipient of a data interchange.
    /// Code or name as specified in IA.
    pub _010: String,
    pub _020: Option<_0007>,
    /// Routing address
    ///
    /// Address specified by the recipient of an interchange to be included by the sender and used by the recipient for routing of received interchanges inside his organization.
    /// If used, normally coded sub-address for onward routing.
    pub _030: Option<String>,
}

/// Date/time of preparation
///
/// Date/time of preparation of the interchange.
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, ParseElement, PartialEq, Eq,
)]
pub struct S004 {
    /// Date of preparation
    ///
    /// Local date when an interchange or a functional group was prepared.
    /// YYMMDD
    pub _010: String,
    /// Time of preparation
    ///
    /// Local time of day when an interchange or a functional group was prepared.
    /// HHMM
    pub _020: String,
}

/// Recipient's reference, password
///
/// Reference or password as agreed between the communicating partners.
#[derive(
    Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, ParseElement, PartialEq, Eq,
)]
pub struct S005 {
    /// Recipient's reference/password
    ///
    /// Unique reference assigned by the recipient to the data interchange or a password
    /// to the recipient's system or to a third party network as specified in the partners interchange agreement.
    /// As specified in IA. May be password to recipient's system or to third party network.
    pub _010: String,
    /// Recipient's reference/password qualifier
    ///
    /// Qualifier for the recipient's reference or password.
    /// Used if specified in IA.
    pub _020: Option<_0025>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct S009 {
    /// Message type
    ///
    /// M   an1..6
    pub _010: String,
    /// Message version number
    ///
    /// M   an1..3
    pub _020: String,
    /// Message release number
    ///
    /// M   an1..3
    pub _030: String,
    /// Controlling agency, coded
    ///
    /// M   an1..3
    pub _040: String,
    /// Association assigned code
    ///
    /// C   an1..6
    pub _050: Option<String>,
}

impl<'a> Parser<&'a str, S009, nom::error::Error<&'a str>> for S009 {
    fn parse(input: &'a str) -> IResult<&'a str, S009> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let v = vars.get(1).unwrap().to_string();
        let r = vars.get(2).unwrap().to_string();
        if format!("{v}{r}") != VERSION {
            return Err(nom::Err::Error(nom::error::Error::new(
                "File supplied EDIFACT Version/Release in UNH segment. Please use the correct parsing feature",
                nom::error::ErrorKind::Verify,
            )));
        }
        let output = S009 {
            _010: vars.first().unwrap().to_string(),
            _020: v,
            _030: r,
            _040: vars.get(3).unwrap().to_string(),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C213 - NUMBER AND TYPE OF PACKAGES
///
/// Number and type of individual parts of a shipment.
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct C213 {
    /// Package quantity                          C      n..8
    pub _010: Option<String>,
    /// Package type description code             C      an..17
    pub _020: Option<String>,
    /// Code list identification code             C      an..17
    pub _030: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _040: Option<String>,
    /// Type of packages                          C      an..35
    pub _050: Option<String>,
    /// Packaging related description code        C      an..3
    pub _060: Option<String>,
}

/// STATUS OF THE TRANSFER
#[derive(
    Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone, DisplayInnerSegment, ParseElement,
)]
pub struct S010 {
    /// Sequence of transfers
    ///
    /// M  n..2
    pub _010: String,
    /// First and last transfer
    ///
    /// C  a1
    pub _020: Option<String>,
}
