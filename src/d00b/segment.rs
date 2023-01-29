use macros::{DisplayInnerSegment, DisplayOuterSegment};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

use super::{_1225, _4343};

/// BGM	BEGINNING OF MESSAGE
///
/// A segment indicating the beginning of a message and identifying the consignment for which status is being reported.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Bgm {
    pub _010: Option<C002>,
    pub _020: Option<C106>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

/// C002 - DOCUMENT/MESSAGE NAME
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C002 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

/// C106 DOCUMENT/MESSAGE IDENTIFICATION
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C106 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C107 - TEXT REFERENCE
///
/// Coded reference to a standard text and its source.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C107 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C108 - TEXT LITERAL
///
/// Free text; one to five lines.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C108 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
    pub _050: Option<String>,
}

/// C211 - DIMENSIONS
///
/// Specification of the dimensions of a transportable unit.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C211 {
    /// Measurement unit code
    ///
    /// Code specifying the unit of measurement.
    pub _010: String,
    /// Length dimension value
    ///
    /// To specify the value of a length dimension.
    pub _020: Option<String>,
    /// Width dimension value
    ///
    /// To specify the value of a width dimension.
    pub _030: Option<String>,
    /// Height dimension value
    ///
    /// To specify the value of a height dimension.
    pub _040: Option<String>,
}

/// C233 - SERVICE
///
/// To identify a service (which may constitute an additional component to a basic contract).
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C233 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
    pub _050: Option<String>,
    pub _060: Option<String>,
}

/// C270 - CONTROL
///
/// Control total for checking integrity of a message or part
/// of a message.
#[derive(Debug, Serialize, Deserialize, Default, DisplayInnerSegment)]
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

/// C507 DTM  DATE/TIME/PERIOD
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C507 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<super::_2379>,
}

/// C517 - LOCATION IDENTIFICATION
///
/// Identification of a location by code or name.
#[derive(Debug, Serialize, Deserialize, DisplayInnerSegment)]
pub struct C517 {
    /// Location name code
    ///
    /// Code specifying the name of the location.
    _010: Option<String>,
    _020: Option<String>,
    _030: Option<String>,
    _040: Option<String>,
}

/// C519 - RELATED LOCATION ONE IDENTIFICATION
///
/// Identification the first related location by code or name.
#[derive(Debug, Serialize, Deserialize, DisplayInnerSegment)]
pub struct C519 {
    _010: Option<String>,
    _020: Option<String>,
    _030: Option<String>,
    _040: Option<String>,
}

/// C536 - CONTRACT AND CARRIAGE CONDITION
///
/// To identify a contract and carriage condition.
#[derive(Debug, Serialize, Deserialize, DisplayInnerSegment)]
pub struct C536 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C537 - TRANSPORT PRIORITY
///
/// To indicate the priority of requested transport service.
#[derive(Debug, Serialize, Deserialize, DisplayInnerSegment)]
pub struct C537 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C553 - RELATED LOCATION TWO IDENTIFICATION
///
/// Identification of second related location by code or name.
#[derive(Debug, Serialize, Deserialize, DisplayInnerSegment)]
pub struct C553 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

/// C703 - NATURE OF CARGO
///
/// Rough classification of a type of cargo.
#[derive(Debug, Serialize, Deserialize, DisplayInnerSegment)]
pub struct C703 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Cnt {
    /// CONTROL
    ///
    /// Control total for checking integrity of a message or part of a message.
    pub _010: C270,
}

/// DIM	- DIMENSIONS
///
/// A segment specifying dimensions of a goods item.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct Dim {
    /// DIMENSION TYPE CODE QUALIFIER
    ///
    /// Code qualifying the type of the dimension.
    pub _010: String,
    /// C211 - DIMENSIONS
    ///
    /// Specification of the dimensions of a transportable unit.
    pub _020: C211,
}

#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct Dtm {
    pub _010: C507,
}

/// EQN	- NUMBER OF UNITS
///
/// A segment specifying the number of units to which the given measurement is applicable.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct Eqn {
    /// C523 - NUMBER OF UNIT DETAILS
    ///
    /// Identification of number of units and its purpose.
    pub _010: C523,
}

/// FTX - FREE TEXT
///
/// A segment specifying free form or processable supplementary or other information.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Ftx {
    /// TEXT SUBJECT CODE QUALIFIER
    ///
    /// Code qualifying the subject of the text.
    pub _010: String,
    /// FREE TEXT FUNCTION CODE
    ///
    /// Code specifying the function of free text.
    pub _020: Option<String>,
    /// TEXT REFERENCE
    ///
    /// Coded reference to a standard text and its source.
    pub _030: Option<C107>,
    /// TEXT LITERAL
    ///
    /// Free text; one to five lines.
    pub _040: Option<C108>,
    /// LANGUAGE NAME CODE
    ///
    /// Code specifying the language name.
    pub _050: Option<String>,
    /// FREE TEXT FORMAT CODE
    ///
    /// Code specifying the format of free text.
    pub _060: Option<String>,
}

/// GIN	GOODS IDENTITY NUMBER
///
/// A segment specifying identity numbers related to the transport line items.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Gin {
    /// OBJECT IDENTIFICATION CODE QUALIFIER
    ///
    /// Code qualifying the identification of an object.
    pub _010: String,
    /// C208 - IDENTITY NUMBER RANGE
    ///
    /// Goods item identification numbers, start and end of consecutively numbered range.
    pub _020: C208,
    /// C208 - IDENTITY NUMBER RANGE
    ///
    /// Goods item identification numbers, start and end of consecutively numbered range.
    pub _030: Option<C208>,
    /// C208 - IDENTITY NUMBER RANGE
    ///
    /// Goods item identification numbers, start and end of consecutively numbered range.
    pub _040: Option<C208>,
    /// C208 - IDENTITY NUMBER RANGE
    ///
    /// Goods item identification numbers, start and end of consecutively numbered range.
    pub _050: Option<C208>,
    /// C208 - IDENTITY NUMBER RANGE
    ///
    /// Goods item identification numbers, start and end of consecutively numbered range.
    pub _060: Option<C208>,
}

/// LOC	- PLACE/LOCATION IDENTIFICATION
///
/// A segment identifying a place/location which applies to the consignment such as consignment origin and destination.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Loc {
    /// LOCATION FUNCTION CODE QUALIFIER
    ///
    /// Code identifying the function of a location.
    pub _010: String,
    /// LOCATION IDENTIFICATION
    ///
    /// Identification of a location by code or name.
    pub _020: Option<C517>,
    /// RELATED LOCATION ONE IDENTIFICATION
    ///
    /// Identification the first related location by code or name.
    pub _030: Option<C519>,
    /// RELATED LOCATION TWO IDENTIFICATION
    ///
    /// Identification of second related location by code or name.
    pub _040: Option<C553>,
    /// RELATION CODE
    ///
    /// Code specifying a relation.
    pub _050: Option<String>,
}

/// MEA	- MEASUREMENTS
///
/// A segment specifying measurements, other than dimension, of a goods item.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Mea {
    /// MEASUREMENT PURPOSE CODE QUALIFIER
    ///
    /// Code qualifying the purpose of the measurement.
    pub _010: String,
    /// C502 - MEASUREMENT DETAILS
    ///
    /// Identification of measurement type.
    pub _020: Option<C502>,
    /// C174 - VALUE/RANGE
    ///
    /// Measurement value and relevant minimum and maximum values of the measurement range.
    pub _030: Option<C174>,
    /// SURFACE OR LAYER CODE
    ///
    /// Code specifying the surface or layer of an object.
    pub _040: Option<String>,
}

/// NAD - NAME AND ADDRESS
///
/// A segment specifying the name and/or address associated with the event such as notify party, terminal address, trucking company for gate move.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Nad {
    /// PARTY FUNCTION CODE QUALIFIER
    ///
    /// Code giving specific meaning to a party.
    pub _010: String,
    /// C082 - PARTY IDENTIFICATION DETAILS
    ///
    /// Identification of a transaction party by code.
    pub _020: Option<C082>,
    /// C058 - NAME AND ADDRESS
    ///
    /// Unstructured name and address: one to five lines.
    pub _030: Option<C058>,
    /// C080 - PARTY NAME
    ///
    /// Identification of a transaction party by name, one to five lines. Party name may be formatted.
    pub _040: Option<C080>,
    /// C059 - STREET
    ///
    /// Street address and/or PO Box number in a structured address: one to four lines.
    pub _050: Option<C059>,
    /// CITY NAME
    ///
    /// Name of a city.
    pub _060: Option<String>,
    /// C819 - COUNTRY SUB-ENTITY DETAILS
    ///
    /// To specify a part of a country (eg county or part of a city).
    pub _070: Option<C819>,
    /// POSTAL IDENTIFICATION CODE
    ///
    /// Code specifying the postal zone or address.
    pub _080: Option<String>,
    /// COUNTRY NAME CODE
    ///
    /// Identification of the name of the country or other geographical entity as specified in ISO 3166.
    pub _090: Option<String>,
}

/// PCI - PACKAGE IDENTIFICATION
///
/// A segment specifying marks related to the transport line items.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Pci {
    /// MARKING INSTRUCTIONS CODE
    ///
    /// Code specifying instructions for marking.
    pub _010: Option<String>,
    /// C210 - MARKS & LABELS
    ///
    /// Shipping marks on packages in free text; one to ten lines.
    pub _020: Option<C210>,
    /// CONTAINER OR PACKAGE CONTENTS INDICATOR CODE
    ///
    /// Code indicating the contents of container or package.
    pub _030: Option<String>,
    /// C827 - TYPE OF MARKING
    ///
    /// Specification of the type of marking that reflects the method that was used and the conventions adhered to for marking (e.g. of packages).
    pub _040: Option<C827>,
}

/// RFF - REFERENCE
///
/// A segment to specify a reference number to equipment.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Rff {
    /// C506 - REFERENCE
    ///
    /// Identification of a reference.
    pub _010: C506,
}

#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
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
    /// Code list directory version number
    ///
    /// C   an1..6
    pub _060: Option<String>,
    /// Message type sub-function identification
    ///
    /// C   an1..6
    pub _070: Option<String>,
}

/// STATUS OF THE TRANSFER
#[derive(Debug, Serialize, Deserialize, Default, DisplayInnerSegment)]
pub struct S010 {
    /// Sequence of transfers
    /// M  n..2
    pub _010: String,
    /// First and last transfer
    ///
    /// C  a1
    pub _020: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayInnerSegment)]
pub struct S016 {
    /// Message subset identification
    ///
    /// Coded identification of a message subset, assigned by its controlling agency.
    pub _010: String,
    /// Message subset version number
    ///
    /// Version number of the message subset.
    pub _020: Option<String>,
    /// Message subset release number
    ///
    /// Release number within the message subset version number.
    pub _030: Option<String>,
    /// Controlling agency, coded
    pub _040: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayInnerSegment)]
pub struct S017 {
    /// Message implementation guideline identification
    ///
    /// Coded identification of the message implementation guideline, assigned by its controlling agency.
    pub _010: String,
    /// Message implementation guideline version number
    ///
    /// Version number of the message implementation guideline.
    pub _020: Option<String>,
    /// Message implementation guideline release number
    ///
    /// Release number within the message implementation guideline version number.
    pub _030: Option<String>,
    /// Controlling agency, coded
    ///
    /// Code identifying a controlling agency.
    pub _040: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayInnerSegment)]
pub struct S018 {
    /// Scenario identification
    ///
    /// Code identifying scenario.
    pub _010: String,
    /// Scenario version number
    ///
    /// Version number of a scenario.
    pub _020: Option<String>,
    /// Scenario release number
    ///
    /// Release number within the scenario version number.
    pub _030: Option<String>,
    /// Controlling agency, coded
    ///
    /// Code identifying a controlling agency.
    pub _040: Option<String>,
}

/// SEL - SEAL NUMBER
///
/// A segment identifying seal and seal issuer associated with the equipment.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Sel {
    /// SEAL IDENTIFIER
    ///
    /// To identify a seal.
    pub _010: Option<String>,
    /// SEAL ISSUER
    ///
    /// Identification of the issuer of a seal on equipment either by code or by name.
    pub _020: Option<C215>,
    /// SEAL CONDITION CODE
    ///
    /// Code specifying the condition of a seal.
    pub _030: Option<String>,
    /// C208 - IDENTITY NUMBER RANGE
    ///
    /// Goods item identification numbers, start and end of consecutively numbered range.
    pub _040: Option<C208>,
}

/// TSR	TRANSPORT SERVICE REQUIREMENTS
///
/// A segment identifying the transport service relating to the consignment.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct Tsr {
    /// CONTRACT AND CARRIAGE CONDITION
    ///
    /// To identify a contract and carriage condition.
    pub _010: Option<C536>,
    /// SERVICE
    ///
    /// To identify a service (which may constitute an additional component to a basic contract).
    pub _020: Option<C233>,
    /// TRANSPORT PRIORITY
    ///
    /// To indicate the priority of requested transport service.
    pub _030: Option<C537>,
    /// NATURE OF CARGO
    ///
    /// Rough classification of a type of cargo.
    pub _040: Option<C703>,
}

/// UNH - MESSAGE HEADER
///
/// To head, identify and specify a message.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Unh {
    _010: Option<String>,
    _020: Option<S009>,
    _030: Option<String>,
    _040: Option<S010>,
    _050: Option<S016>,
    _060: Option<S017>,
    _070: Option<S018>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Unt {
    /// NUMBER OF SEGMENTS IN A MESSAGE
    ///
    /// The number of segments in a message body, plus the message header segment and message trailer segment.
    pub _010: String,
    /// MESSAGE REFERENCE NUMBER
    ///
    /// Unique message reference assigned by the sender.
    pub _020: String,
}
