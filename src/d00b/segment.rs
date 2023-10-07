use super::*;
use crate::util::Parser;
use edifact_types_macros::{DisplayInnerSegment, DisplayOuterSegment};
use nom::{combinator::opt, IResult};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

/// BGM - BEGINNING OF MESSAGE
///
/// A segment indicating the beginning of a message and identifying the consignment for which status is being reported.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BGM {
    pub _010: Option<C002>,
    pub _020: Option<C106>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

impl ::core::fmt::Display for BGM {
    fn fmt<'x>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: Vec<String> = vec![];
        if let Some(val) = &self._010 {
            str.push(val.to_string());
        } else {
            str.push("".to_string());
        }
        if let Some(val) = &self._020 {
            str.push(val.to_string());
        } else {
            str.push("".to_string());
        }
        if let Some(val) = &self._030 {
            str.push(val.to_string());
        } else {
            str.push("".to_string());
        }
        if let Some(val) = &self._040 {
            str.push(val.to_string());
        } else {
            str.push("".to_string());
        }
        let joined = str.join("+");
        let joined = joined.trim_end_matches("+");
        write!(f, "BGM+{}'\n", joined)
    }
}

impl<'a> Parser<&'a str, BGM, nom::error::Error<&'a str>> for BGM {
    fn parse(input: &'a str) -> IResult<&'a str, BGM> {
        let (output_rest, vars) = crate::util::parse_line(input, "BGM")?;
        let mut output = BGM::default();
        let (_, obj) = opt(C002::parse)(vars.first().unwrap())?;
        output._010 = obj;
        let (_, obj) = opt(C106::parse)(vars.get(1).unwrap())?;
        output._020 = obj;
        if let Some(var) = vars.get(2) {
            let obj = _1225::from_str(var).unwrap();
            output._030 = Some(obj);
        }
        if let Some(var) = vars.get(3) {
            let obj = _4343::from_str(var).unwrap();
            output._040 = Some(obj);
        }
        Ok((output_rest, output))
    }
}

/// C002 - DOCUMENT/MESSAGE NAME
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C002 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C002, nom::error::Error<&'a str>> for C002 {
    fn parse(input: &'a str) -> IResult<&'a str, C002> {
        let vars: Vec<&str> = input.split(':').collect();
        let output = C002 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C040 - CARRIER
///
/// Identification of a carrier by code and/or by name. Code preferred.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C040 {
    /// Carrier identifier                 
    pub _010: Option<String>,
    /// Code list identification code      
    pub _020: Option<String>,
    /// Code list responsible agency code
    pub _030: Option<String>,
    /// Carrier name                       
    pub _040: Option<String>,
}

/// C056 DEPARTMENT OR EMPLOYEE DETAILS
///
/// Code and/or name of a department or employee. Code
/// preferred.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C056 {
    /// Department or employee name code          C      an..17
    pub _010: Option<String>,
    /// Department or employee name               C      an..35
    pub _020: Option<String>,
}

/// C058 NAME AND ADDRESS
///
/// Unstructured name and address: one to five lines.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C058 {
    /// Name and address description              M      an..35
    pub _010: String,
    /// Name and address description              C      an..35
    pub _020: Option<String>,
    /// Name and address description              C      an..35
    pub _030: Option<String>,
    /// Name and address description              C      an..35
    pub _040: Option<String>,
    /// Name and address description              C      an..35
    pub _050: Option<String>,
}

/// C059 - STREET
///
/// Street address and/or PO Box number in a structured
/// address: one to four lines.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C059 {
    /// Street and number or post office box identifier                                M      an..35
    pub _010: Option<String>,
    /// Street and number or post office box identifier                                C      an..35
    pub _020: Option<String>,
    /// Street and number or post office box identifier                                C      an..35
    pub _030: Option<String>,
    /// Street and number or post office box identifier                                C      an..35
    pub _040: Option<String>,
}

/// C076 COMMUNICATION CONTACT
///
/// Communication number of a department or employee in
/// a specified channel.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C076 {
    /// Communication address identifier          M      an..512
    pub _010: String,
    /// Communication address code qualifier      M      an..3
    pub _020: String,
}
/// C080 PARTY NAME
///
/// Identification of a transaction party by name, one
/// to five lines. Party name may be formatted.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C080 {
    /// Party name                                M      an..35
    pub _010: String,
    /// Party name                                C      an..35
    pub _020: Option<String>,
    /// Party name                                C      an..35
    pub _030: Option<String>,
    /// Party name                                C      an..35
    pub _040: Option<String>,
    /// Party name                                C      an..35
    pub _050: Option<String>,
    /// Party name format code                    C      an..3
    pub _060: Option<String>,
}

/// C082 PARTY IDENTIFICATION DETAILS
///
/// Identification of a transaction party by code.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C082 {
    /// Party identifier                          M      an..35
    pub _010: String,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
}

/// C106 DOCUMENT/MESSAGE IDENTIFICATION
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C106 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

impl<'a> Parser<&'a str, C106, nom::error::Error<&'a str>> for C106 {
    fn parse(input: &'a str) -> IResult<&'a str, C106> {
        let vars: Vec<&str> = input.split(':').collect();
        let output = C106 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C107 - TEXT REFERENCE
///
/// Coded reference to a standard text and its source.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C107 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C108 - TEXT LITERAL
///
/// Free text; one to five lines.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C108 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
    pub _050: Option<String>,
}

/// C174 VALUE/RANGE
///
/// Measurement value and relevant minimum and maximum
/// values of the measurement range.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C174 {
    /// Measurement unit code                     M      an..3
    pub _010: String,
    /// Measurement value                         C      an..18
    pub _020: Option<String>,
    /// Range minimum value                       C      n..18
    pub _030: Option<String>,
    /// Range maximum value                       C      n..18
    pub _040: Option<String>,
    /// Significant digits quantity               C      n..2
    pub _050: Option<String>,
}

/// C205 HAZARD CODE
///
/// The identification of the dangerous goods in code.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C205 {
    /// Hazard identification code                M      an..7
    pub _010: String,
    /// Additional hazard classification
    /// identifier                                C      an..7
    pub _020: Option<String>,
    /// Hazard code version identifier            C      an..10
    pub _030: Option<String>,
}
/// C208 - IDENTITY NUMBER RANGE
///
/// Goods item identification numbers, start and end of consecutively numbered range.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C208 {
    /// Object identifier                         M      an..35
    pub _010: String,
    /// Object identifier                         C      an..35
    pub _020: Option<String>,
}

/// C210 - MARKS & LABELS
///
/// Shipping marks on packages in free text; one to ten lines.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C210 {
    /// Shipping marks description                M      an..35
    pub _010: String,
    /// Shipping marks description                C      an..35
    pub _020: Option<String>,
    /// Shipping marks description                C      an..35
    pub _030: Option<String>,
    /// Shipping marks description                C      an..35
    pub _040: Option<String>,
    /// Shipping marks description                C      an..35
    pub _050: Option<String>,
    /// Shipping marks description                C      an..35
    pub _060: Option<String>,
    /// Shipping marks description                C      an..35
    pub _070: Option<String>,
    /// Shipping marks description                C      an..35
    pub _080: Option<String>,
    /// Shipping marks description                C      an..35
    pub _090: Option<String>,
    /// Shipping marks description                C      an..35
    pub _100: Option<String>,
}

/// C211 - DIMENSIONS
///
/// Specification of the dimensions of a transportable unit.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
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

/// C213 NUMBER AND TYPE OF PACKAGES
///
/// Number and type of individual parts of a shipment.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
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

/// C215 - SEAL ISSUER
///
/// Identification of the issuer of a seal on equipment
/// either by code or by name.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C215 {
    /// Sealing party name code                   C      an..3
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Sealing party name                        C      an..35
    pub _040: Option<String>,
}

/// C218 HAZARDOUS MATERIAL
///
/// To specify a hazardous material.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C218 {
    /// Hazardous material category name code     C      an..4
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Hazardous material category name          C      an..35
    pub _040: Option<String>,
}

/// C219 - MOVEMENT TYPE
///
/// Description of type of service for movement of cargo.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C219 {
    /// Movement type description code        
    pub _010: Option<String>,
    /// Movement type description              
    pub _020: Option<String>,
}

/// C220 - MODE OF TRANSPORT
///
/// Method of transport code or name. Code preferred.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C220 {
    /// Transport mode name code                  C      an..3
    pub _010: Option<String>,
    /// Transport mode name                       C      an..17
    pub _020: Option<String>,
}

/// C222 - TRANSPORT IDENTIFICATION
///
/// Code and/or name identifying the means of transport.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C222 {
    /// Transport means identification name identifier   
    pub _010: Option<String>,
    /// Code list identification code          
    pub _020: Option<String>,
    /// Code list responsible agency code     
    pub _030: Option<String>,
    /// Transport means identification name   
    pub _040: Option<String>,
    /// Transport means nationality code      
    pub _050: Option<String>,
}

/// C223 DANGEROUS GOODS SHIPMENT FLASHPOINT
///
/// Temperature at which a vapor can be ignited as per
/// ISO 1523/73.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C223 {
    /// Shipment flashpoint value                 C      n3
    pub _010: Option<String>,
    /// Measurement unit code                     C      an..3
    pub _020: Option<String>,
}

/// C224 EQUIPMENT SIZE AND TYPE
///
/// Code and or name identifying size and type of equipment. Code preferred.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C224 {
    /// Equipment size and type description code  C      an..10
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Equipment size and type description       C      an..35
    pub _040: Option<String>,
}
/// C228 - TRANSPORT MEANS
///
/// Code and/or name identifying the type of means of transport.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C228 {
    /// Transport means description code    
    pub _010: Option<String>,
    /// Transport means description          
    pub _020: Option<String>,
}

/// C233 - SERVICE
///
/// To identify a service (which may constitute an additional component to a basic contract).
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C233 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
    pub _050: Option<String>,
    pub _060: Option<String>,
}

/// C234 UNDG INFORMATION
///
/// Information on dangerous goods, taken from the
/// United Nations Dangerous Goods classification.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayInnerSegment)]
pub struct C234 {
    /// United Nations Dangerous Goods (UNDG)
    /// identifier                                C      n4
    pub _010: Option<String>,
    /// Dangerous goods flashpoint value          C      an..8
    pub _020: Option<String>,
}

/// C235 HAZARD IDENTIFICATION PLACARD DETAILS
///
/// These numbers appear on the hazard identification
/// placard required on the means of transport.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayInnerSegment)]
pub struct C235 {
    /// Orange hazard placard upper part
    /// identifier                                C      an..4
    pub _010: Option<String>,
    /// Orange hazard placard lower part
    /// identifier                                C      an4
    pub _020: Option<String>,
}
/// C236 DANGEROUS GOODS LABEL
///
/// Markings identifying the type of hazardous goods and
/// similar information.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayInnerSegment)]
pub struct C236 {
    /// Dangerous goods marking identifier        C      an..4
    pub _010: Option<String>,
    /// Dangerous goods marking identifier        C      an..4
    pub _020: Option<String>,
    /// Dangerous goods marking identifier        C      an..4
    pub _030: Option<String>,
}

/// C237 - EQUIPMENT IDENTIFICATION
///
/// Marks (letters/numbers) identifying equipment.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayInnerSegment)]
pub struct C237 {
    /// Equipment identifier                      C      an..17
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Country name code                         C      an..3
    pub _040: Option<String>,
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

/// C401 - EXCESS TRANSPORTATION INFORMATION
///
/// To provide details of reason for, and responsibility
/// for, use of transportation other than normally
/// utilized.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C401 {
    /// Excess transportation reason code         M      an..3
    pub _010: String,
    /// Excess transportation responsibility code M      an..3
    pub _020: String,
    /// Customer shipment authorisation
    /// identifier                   
    pub _030: Option<String>,
}

/// C502 MEASUREMENT DETAILS
///
/// Identification of measurement type.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C502 {
    /// Measured attribute code                   C      an..3
    pub _010: Option<String>,
    /// Measurement significance code             C      an..3
    pub _020: Option<String>,
    /// Non-discrete measurement name code        C      an..17
    pub _030: Option<String>,
    /// Non-discrete measurement name             C      an..70
    pub _040: Option<String>,
}

/// C503 DOCUMENT/MESSAGE DETAILS
///
/// Identification of document/message by number,
/// status, source and/or language.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C503 {
    /// Document identifier                       C      an..35
    pub _010: Option<String>,
    /// Document status code                      C      an..3
    pub _020: Option<String>,
    /// Document source description               C      an..70
    pub _030: Option<String>,
    /// Language name code                        C      an..3
    pub _040: Option<String>,
    /// Version identifier                        C      an..9
    pub _050: Option<String>,
    /// Revision identifier                       C      an..6
    pub _060: Option<String>,
}
/// C506 - REFERENCE
///
/// Identification of a reference.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C506 {
    /// Reference code qualifier                  M      an..3
    pub _010: String,
    /// Reference identifier                      C      an..70
    pub _020: Option<String>,
    /// Document line identifier                  C      an..6
    pub _030: Option<String>,
    /// Reference version identifier              C      an..35
    pub _040: Option<String>,
    /// Revision identifier                       C      an..6
    pub _050: Option<String>,
}
/// C507 DTM  DATE/TIME/PERIOD
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C507 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<super::_2379>,
}

/// C517 - LOCATION IDENTIFICATION
///
/// Identification of a location by code or name.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C519 {
    _010: Option<String>,
    _020: Option<String>,
    _030: Option<String>,
    _040: Option<String>,
}

/// C523 NUMBER OF UNIT DETAILS
///
/// Identification of number of units and its purpose.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C523 {
    /// Units quantity                            C      n..15
    pub _010: Option<String>,
    /// Unit type code qualifier                  C      an..3
    pub _020: Option<String>,
}
/// C524 HANDLING INSTRUCTIONS
///
/// Instruction for the handling of goods, products or
/// articles in shipment, storage etc.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C524 {
    /// Handling instruction description code     C      an..3
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Handling instruction description          C      an..70
    pub _040: Option<String>,
}

/// C536 - CONTRACT AND CARRIAGE CONDITION
///
/// To identify a contract and carriage condition.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C536 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C537 - TRANSPORT PRIORITY
///
/// To indicate the priority of requested transport service.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C537 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C553 - RELATED LOCATION TWO IDENTIFICATION
///
/// Identification of second related location by code or name.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C553 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

/// C555 - STATUS
///
/// To specify a status.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C555 {
    /// Status description code                
    pub _010: String,
    /// Code list identification code           
    pub _020: Option<String>,
    /// Code list responsible agency code      
    pub _030: Option<String>,
    /// Status description                      
    pub _040: Option<String>,
}

/// C556 - STATUS REASON
///
/// To specify the reason for a status.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C556 {
    /// Status reason description code       
    pub _010: String,
    /// Code list identification code         
    pub _020: Option<String>,
    /// Code list responsible agency code    
    pub _030: Option<String>,
    /// Status reason description              
    pub _040: Option<String>,
}

/// C601 - STATUS CATEGORY
///
/// To specify the category of the status.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C601 {
    /// Status category code                   
    pub _010: String,
    /// Code list identification code           
    pub _020: Option<String>,
    /// Code list responsible agency code      
    pub _030: Option<String>,
}

/// C703 - NATURE OF CARGO
///
/// Rough classification of a type of cargo.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C703 {
    pub _010: String,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

/// C819 - COUNTRY SUB-ENTITY DETAILS
///
/// To specify a part of a country (eg county or part of
/// a city).
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C819 {
    /// Country sub-entity name code              C      an..9
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Country sub-entity name                   C      an..35
    pub _040: Option<String>,
}
/// C827 - TYPE OF MARKING
///
/// Specification of the type of marking that reflects
/// the method that was used and the conventions adhered
/// to for marking (e.g. of packages).
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C827 {
    /// Marking type code                         M      an..3
    pub _010: String,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
}

/// CNI - CONSIGNMENT INFORMATION
///
/// A segment to identify a consignment for which status details are given.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Cni {
    /// CONSOLIDATION ITEM NUMBER
    ///
    /// To specify a consignment within a consolidation.
    pub _010: Option<String>,
    /// C503 - DOCUMENT/MESSAGE DETAILS
    ///
    /// Identification of document/message by number, status, source and/or language.
    pub _020: Option<C503>,
    /// CONSIGNMENT LOAD SEQUENCE IDENTIFIER
    ///
    /// To identify the loading sequence of a consignment or consignments.
    pub _030: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Cnt {
    /// CONTROL
    ///
    /// Control total for checking integrity of a message or part of a message.
    pub _010: C270,
}

/// CTA - CONTACT INFORMATION
///
/// A segment to specify a contact name associated with the party.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Cta {
    /// CONTACT FUNCTION CODE
    ///
    /// Code specifying the function of a contact (e.g. department or person).
    pub _010: Option<String>,
    /// C056 - DEPARTMENT OR EMPLOYEE DETAILS
    ///
    /// Code and/or name of a department or employee. Code preferred.
    pub _020: Option<C056>,
}

/// COM - COMMUNICATION CONTACT
///
/// A segment to specify a communication number related to the contact.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct Com {
    /// C076 - COMMUNICATION CONTACT
    ///
    /// Communication number of a department or employee in a specified channel.
    pub _010: C076,
}

/// DGS - DANGEROUS GOODS
///
/// A segment to specify dangerous goods details related to the goods item.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct DGS {
    /// DANGEROUS GOODS REGULATIONS CODE
    ///
    /// Code specifying a dangerous goods regulation.
    pub _010: Option<String>,
    /// C205 - HAZARD CODE
    ///
    /// The identification of the dangerous goods in code.
    pub _020: Option<C205>,
    /// C234 - UNDG INFORMATION
    ///
    /// Information on dangerous goods, taken from the United Nations Dangerous Goods classification.
    pub _030: Option<C234>,
    /// C223 - DANGEROUS GOODS SHIPMENT FLASHPOINT
    ///
    /// Temperature at which a vapor can be ignited as per ISO 1523/73.
    pub _040: Option<C223>,
    /// PACKAGING DANGER LEVEL CODE
    ///
    /// Code specifying the level of danger for which the packaging must cater.
    pub _050: Option<String>,
    /// EMERGENCY PROCEDURE FOR SHIPS IDENTIFIER
    ///
    /// To identify the emergency procedure number for ships transporting dangerous goods. Synonym: EMS Number.
    pub _060: Option<String>,
    /// HAZARD MEDICAL FIRST AID GUIDE IDENTIFIER
    ///
    /// To identify a Medical First Aid Guide (MFAG) for hazardous goods.
    pub _070: Option<String>,
    /// TRANSPORT EMERGENCY CARD IDENTIFIER
    ///
    /// To identify a transport emergency (TREM) card.
    pub _080: Option<String>,
    /// C235 - HAZARD IDENTIFICATION PLACARD DETAILS
    ///
    /// These numbers appear on the hazard identification placard required on the means of transport.
    pub _090: Option<C235>,
    /// C236 - DANGEROUS GOODS LABEL
    ///
    /// Markings identifying the type of hazardous goods and similar information.
    pub _100: Option<C236>,
    /// PACKING INSTRUCTION TYPE CODE
    ///
    /// Code specifying a type of packing instruction.
    pub _110: Option<String>,
    /// HAZARDOUS MEANS OF TRANSPORT CATEGORY CODE
    ///
    /// Code specifying the category of means of transport for carrying hazardous goods.
    pub _120: Option<String>,
    /// HAZARDOUS CARGO TRANSPORT AUTHORISATION CODE
    ///
    /// Code specifying the authorisation for the transportation of hazardous cargo.
    pub _130: Option<String>,
}

/// DIM - DIMENSIONS
///
/// A segment specifying dimensions of a goods item.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct DIM {
    /// DIMENSION TYPE CODE QUALIFIER
    ///
    /// Code qualifying the type of the dimension.
    pub _010: String,
    /// C211 - DIMENSIONS
    ///
    /// Specification of the dimensions of a transportable unit.
    pub _020: C211,
}

/// DOC - DOCUMENT/MESSAGE DETAILS
///
/// A segment to specify document details related to the status code, such as indication which document is missing (status code: document missing).
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct Doc {
    /// C002 - DOCUMENT/MESSAGE NAME
    ///
    /// Identification of a type of document/message by code or name. Code preferred.
    pub _010: Option<C002>,
    /// C503 - DOCUMENT/MESSAGE DETAILS
    ///
    /// Identification of document/message by number, status, source and/or language.
    pub _020: Option<C503>,
    /// COMMUNICATION MEDIUM TYPE CODE
    ///
    /// Code specifying the type of communication medium.
    pub _030: Option<String>,
    /// DOCUMENT COPIES REQUIRED QUANTITY
    ///
    /// Quantity of document copies required.
    pub _040: Option<String>,
    /// DOCUMENT ORIGINALS REQUIRED QUANTITY
    ///
    /// Quantity of document originals required.
    pub _050: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct DTM {
    pub _010: C507,
}

/// EQA - ATTACHED EQUIPMENT
///
/// A segment identifying attached equipment or related equipment such as a chassis attached to a container.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct EQA {
    /// EQUIPMENT TYPE CODE QUALIFIER
    ///
    /// Code qualifying a type of equipment.
    pub _010: String,
    /// C237 - EQUIPMENT IDENTIFICATION
    ///
    /// Marks (letters/numbers) identifying equipment.
    pub _020: Option<C237>,
}

/// EQD - EQUIPMENT DETAILS
///
/// A segment identifying equipment related to status or event such as a container of a multi-container consignment.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct EQD {
    /// EQUIPMENT TYPE CODE QUALIFIER
    ///
    /// Code qualifying a type of equipment.
    pub _010: String,
    /// C237 - EQUIPMENT IDENTIFICATION
    ///
    /// Marks (letters/numbers) identifying equipment.
    pub _020: Option<C237>,
    /// C224 - EQUIPMENT SIZE AND TYPE
    ///
    /// Code and or name identifying size and type of equipment. Code preferred.
    pub _030: Option<C224>,
    /// EQUIPMENT SUPPLIER CODE
    ///
    /// Code specifying the party that is the supplier of the equipment.
    pub _040: Option<String>,
    /// EQUIPMENT STATUS CODE
    ///
    /// Code specifying the status of equipment.
    pub _050: Option<String>,
    /// FULL OR EMPTY INDICATOR CODE
    ///
    /// Code indicating whether an object is full or empty.
    pub _060: Option<String>,
}

/// EQN - NUMBER OF UNITS
///
/// A segment specifying the number of units to which the given measurement is applicable.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct EQN {
    /// C523 - NUMBER OF UNIT DETAILS
    ///
    /// Identification of number of units and its purpose.
    pub _010: C523,
}

/// FTX - FREE TEXT
///
/// A segment specifying free form or processable supplementary or other information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct FTX {
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

/// GID - GOODS ITEM DETAILS
///
/// A segment identifying a goods item.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct GID {
    /// GOODS ITEM NUMBER
    ///
    /// To specify a goods item within a consignment.
    pub _010: Option<String>,
    /// C213 - NUMBER AND TYPE OF PACKAGES
    ///
    /// Number and type of individual parts of a shipment.
    pub _020: Option<C213>,
    /// C213 - NUMBER AND TYPE OF PACKAGES
    ///
    /// Number and type of individual parts of a shipment.
    pub _030: Option<C213>,
    /// C213 - NUMBER AND TYPE OF PACKAGES
    ///
    /// Number and type of individual parts of a shipment.
    pub _040: Option<C213>,
    /// C213 - NUMBER AND TYPE OF PACKAGES
    ///
    /// Number and type of individual parts of a shipment.
    pub _050: Option<C213>,
    /// C213 - NUMBER AND TYPE OF PACKAGES
    ///
    /// Number and type of individual parts of a shipment.
    pub _060: Option<C213>,
}

/// GIN - GOODS IDENTITY NUMBER
///
/// A segment specifying identity numbers related to the transport line items.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct GIN {
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

/// HAN - HANDLING INSTRUCTIONS
///
/// A segment identifying handling instructions.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct HAN {
    /// C524 - HANDLING INSTRUCTIONS
    ///
    /// Instruction for the handling of goods, products or articles in shipment, storage etc.
    pub _010: Option<C524>,
    /// C218 - HAZARDOUS MATERIAL
    ///
    /// To specify a hazardous material.
    pub _020: Option<C218>,
}

/// LOC - PLACE/LOCATION IDENTIFICATION
///
/// A segment identifying a place/location which applies to the consignment such as consignment origin and destination.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct LOC {
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

/// MEA - MEASUREMENTS
///
/// A segment specifying measurements, other than dimension, of a goods item.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct MEA {
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct NAD {
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct PCI {
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct RFF {
    /// C506 - REFERENCE
    ///
    /// Identification of a reference.
    pub _010: C506,
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct SEL {
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

/// SGP - SPLIT GOODS PLACEMENT
///
/// A segment to identify equipment in which (part of) a goods item is transported.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct SGP {
    /// C237 - EQUIPMENT IDENTIFICATION
    ///
    /// Marks (letters/numbers) identifying equipment.
    pub _010: C237,
    /// PACKAGE QUANTITY
    ///
    /// To specify the number of packages.
    pub _020: Option<String>,
}

/// STS - STATUS
///
/// A segment specifying the status relating to a consignment (e.g. loaded).
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct STS {
    /// C601 - STATUS CATEGORY
    ///
    /// To specify the category of the status.
    pub _010: Option<C601>,
    /// C555STATUS
    ///
    /// To specify a status.
    pub _020: Option<C555>,
    /// C556 -STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _030: Option<C556>,
    /// C556 -STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _040: Option<C556>,
    /// C556 -STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _050: Option<C556>,
    /// C556 -STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _060: Option<C556>,
    /// C556 -STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _070: Option<C556>,
}

/// TDT - DETAILS OF TRANSPORT
///
/// A segment identifying conveyance related to the status or event such as flight, vessel/voyage.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TDT {
    /// TRANSPORT STAGE CODE QUALIFIER
    ///
    /// Code qualifying a specific stage of transport.
    pub _010: String,
    /// MEANS OF TRANSPORT JOURNEY IDENTIFIER
    ///
    /// To identify a journey of a means of transport.
    pub _020: Option<String>,
    /// C220 - MODE OF TRANSPORT
    ///
    /// Method of transport code or name. Code preferred.
    pub _030: Option<C220>,
    /// C228- TRANSPORT MEANS
    ///
    /// Code and/or name identifying the type of means of transport.
    pub _040: Option<C228>,
    /// C040 - CARRIER
    ///
    /// Identification of a carrier by code and/or by name. Code preferred.
    pub _050: Option<C040>,
    /// TRANSIT DIRECTION INDICATOR CODE
    ///
    /// Code specifying the direction of transport.
    pub _060: Option<String>,
    /// C401 - EXCESS TRANSPORTATION INFORMATION
    ///
    /// To provide details of reason for, and responsibility for, use of transportation other than normally utilized.
    pub _070: Option<C401>,
    /// C222 - TRANSPORT IDENTIFICATION
    ///
    /// Code and/or name identifying the means of transport.
    pub _080: Option<C222>,
    /// TRANSPORT MEANS OWNERSHIP INDICATOR CODE
    ///
    /// Code indicating the ownership of a means of transport.
    pub _090: Option<String>,
}
/// TMD - TRANSPORT MOVEMENT DETAILS
///
/// A segment to specify transport movement details related to the equipment.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TMD {
    /// C219 - MOVEMENT TYPE
    ///
    /// Description of type of service for movement of cargo.
    pub _010: Option<C219>,
    /// EQUIPMENT PLAN DESCRIPTION
    ///
    /// Free form description of the equipment plan.
    pub _020: Option<String>,
    /// HAULAGE ARRANGEMENTS CODE
    ///
    /// Code specifying the arrangement for the haulage of goods.
    pub _030: Option<String>,
}
/// TPL - TRANSPORT PLACEMENT
///
/// A segment to identify the means of transport to which the equipment is linked, necessary in cases where this forms the key to retrieve relevant information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TPL {
    /// C222 - TRANSPORT IDENTIFICATION
    ///
    /// Code and/or name identifying the means of transport.
    pub _010: C222,
}

/// TSR - TRANSPORT SERVICE REQUIREMENTS
///
/// A segment identifying the transport service relating to the consignment.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TSR {
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
pub struct UNH {
    _010: Option<String>,
    _020: Option<S009>,
    _030: Option<String>,
    _040: Option<S010>,
    _050: Option<S016>,
    _060: Option<S017>,
    _070: Option<S018>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct UNT {
    /// NUMBER OF SEGMENTS IN A MESSAGE
    ///
    /// The number of segments in a message body, plus the message header segment and message trailer segment.
    pub _010: String,
    /// MESSAGE REFERENCE NUMBER
    ///
    /// Unique message reference assigned by the sender.
    pub _020: String,
}
