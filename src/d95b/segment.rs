use super::*;
use serde::{Deserialize, Serialize};

/// BGM - BEGINNING OF MESSAGE
///
/// To indicate the type and function of a message and to
/// transmit the identifying number.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bgm {
    pub _010: Option<C002>,
    /// 1004 - Document/message number
    ///
    /// Reference number assigned to the document/message by the issuer.
    /// an..35
    pub _020: Option<String>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

/// C002 - DOCUMENT/MESSAGE NAME
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct C507 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C517 - LOCATION IDENTIFICATION
///
/// Identification of a location by code or name.
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct C523 {
    /// Number of units                                   C  n..15
    pub _010: Option<String>,
    /// Number of units qualifier                         C  an..3
    pub _020: Option<String>,
}

/// C553 - RELATED LOCATION TWO IDENTIFICATION
///
/// Identification of second related location by code or name.
#[derive(Debug, Serialize, Deserialize, Default)]
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

/// CNT - CONTROL TOTAL
///
/// To provide control total.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cnt {
    /// CONTROL
    pub _010: C270,
}

/// CTA    CONTACT INFORMATION
///
/// To identify a person or a department to whom
/// communication should be directed.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cta {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dgs {
    /// DANGEROUS GOODS REGULATIONS, CODED                    C  an..3
    pub _010: Option<String>,
    /// HAZARD CODE                                           C  
    pub _020: Option<C205>,
    /// UNDG INFORMATION                                      C  
    pub _030: Option<C234>,
    /// DANGEROUS GOODS SHIPMENT FLASHPOINT                   C  
    pub _040: Option<C223>,
    /// PACKING GROUP, CODED                                  C  an..3
    pub _050: Option<String>,
    /// EMS NUMBER                                            C  an..6
    pub _060: Option<String>,
    /// MFAG                                                  C  an..4
    pub _070: Option<String>,
    /// TREM CARD NUMBER                                      C  an..10
    pub _080: Option<String>,
    /// HAZARD IDENTIFICATION                                 C  
    pub _090: Option<C235>,
    /// DANGEROUS GOODS LABEL                                 C  
    pub _100: Option<C236>,
    /// PACKING INSTRUCTION, CODED                            C  an..3
    pub _110: Option<String>,
    /// CATEGORY OF MEANS OF TRANSPORT, CODED                 C  an..3
    pub _120: Option<String>,
    /// PERMISSION FOR TRANSPORT, CODED                       C  an..3
    pub _130: Option<String>,
}

/// DIM - DIMENSIONS
///
/// To specify dimensions.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dim {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dtm {
    /// DATE/TIME/PERIOD
    pub _010: C507,
}

/// EQA - ATTACHED EQUIPMENT
///
/// To specify attached or related equipment.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Eqa {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Eqd {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Eqn {
    /// NUMBER OF UNIT DETAILS
    pub _010: C523,
}

/// FTX - Free Text
///
/// To provide free form or coded text information.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ftx {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mea {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Loc {
    pub _010: String,
    pub _020: C517,
    pub _030: C519,
    pub _040: C553,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Nad {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rff {
    // REFERENCE
    pub _010: C506,
}

/// RNG - RANGE DETAILS
///
/// To identify a range.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Rng {
    /// RANGE TYPE QUALIFIER
    ///
    /// M  an..3
    pub _010: String,
    /// RANGE
    pub _020: Option<C280>,
}

/// MESSAGE IDENTIFIER
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct S009 {
    /// Message type
    ///
    /// M  an..6
    pub _0065: String,
    /// Message version number
    ///
    /// M  an..3
    pub _0052: String,
    /// Message release number
    ///
    /// M  an..3
    pub _0054: String,
    /// Controlling agency
    ///
    /// M  an..2
    pub _0051: String,
    /// Association assigned code
    ///
    /// C  an..6
    pub _0057: Option<String>,
}

/// STATUS OF THE TRANSFER
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct S010 {
    /// Sequence of transfers
    /// M  n..2
    pub _0070: String,
    /// First and last transfer
    ///
    /// C  a1
    pub _0073: Option<String>,
}

/// SEL - SEAL NUMBER
///
/// To specify a seal number related to equipment.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sel {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tdt {
    pub _010: String,
    pub _020: String,
    pub _030: C220,
    pub _040: C228,
    pub _050: C040,
    pub _060: String,
    pub _070: C401,
    pub _080: C222,
    pub _090: String,
}

/// TMD - TRANSPORT MOVEMENT DETAILS
///
/// To specify transport movement details for a goods item
/// or equipment.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tmd {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tmp {
    /// TEMPERATURE QUALIFIER                                 M  an..3
    pub _010: String,
    /// TEMPERATURE SETTING
    pub _020: Option<C239>,
}

/// UNH - MESSAGE HEADER
///
/// To head, identify and specify a message.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Unh {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Unt {
    /// NUMBER OF SEGMENTS IN THE MESSAGE
    ///
    /// M  n..6
    pub _010: String,
    /// MESSAGE REFERENCE NUMBER
    ///
    /// M  an..14
    pub _020: String,
}
