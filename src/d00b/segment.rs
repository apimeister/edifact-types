use super::*;
use crate::util::Parser;
use edifact_types_macros::{DisplayInnerSegment, DisplayOuterSegment};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

/// BGM - BEGINNING OF MESSAGE
///
/// A segment indicating the beginning of a message and identifying the consignment for which status is being reported.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayOuterSegment)]
pub struct BGM {
    pub _010: Option<C002>,
    pub _020: Option<C106>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

impl<'a> Parser<&'a str, BGM, nom::error::Error<&'a str>> for BGM {
    fn parse(input: &'a str) -> IResult<&'a str, BGM> {
        let (output_rest, vars) = crate::util::parse_line(input, "BGM")?;
        let output = BGM {
            _010: vars.first().map(|x| C002::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C106::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| _1225::from_str(x).unwrap()),
            _040: vars.get(3).map(|x| _4343::from_str(x).unwrap()),
        };
        Ok((output_rest, output))
    }
}

/// C002 - DOCUMENT/MESSAGE NAME
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C002 {
    pub _010: Option<_1001>,
    pub _020: Option<_1131>,
    pub _030: Option<_3055>,
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C002, nom::error::Error<&'a str>> for C002 {
    fn parse(input: &'a str) -> IResult<&'a str, C002> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C002 {
            _010: vars.first().map(|x| _1001::from_str(x).unwrap()),
            _020: vars.get(1).map(|x| _1131::from_str(x).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(x).unwrap()),
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

impl<'a> Parser<&'a str, C040, nom::error::Error<&'a str>> for C040 {
    fn parse(input: &'a str) -> IResult<&'a str, C040> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C040 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C056, nom::error::Error<&'a str>> for C056 {
    fn parse(input: &'a str) -> IResult<&'a str, C056> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C056 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C058, nom::error::Error<&'a str>> for C058 {
    fn parse(input: &'a str) -> IResult<&'a str, C058> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C058 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C059, nom::error::Error<&'a str>> for C059 {
    fn parse(input: &'a str) -> IResult<&'a str, C059> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C059 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C076, nom::error::Error<&'a str>> for C076 {
    fn parse(input: &'a str) -> IResult<&'a str, C076> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C076 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C080, nom::error::Error<&'a str>> for C080 {
    fn parse(input: &'a str) -> IResult<&'a str, C080> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C080 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C082, nom::error::Error<&'a str>> for C082 {
    fn parse(input: &'a str) -> IResult<&'a str, C082> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C082 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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
        let (_, vars) = crate::util::parse_colon_section(input)?;
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

impl<'a> Parser<&'a str, C107, nom::error::Error<&'a str>> for C107 {
    fn parse(input: &'a str) -> IResult<&'a str, C107> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C107 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C108, nom::error::Error<&'a str>> for C108 {
    fn parse(input: &'a str) -> IResult<&'a str, C108> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C108 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C174, nom::error::Error<&'a str>> for C174 {
    fn parse(input: &'a str) -> IResult<&'a str, C174> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C174 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C205, nom::error::Error<&'a str>> for C205 {
    fn parse(input: &'a str) -> IResult<&'a str, C205> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C205 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C208, nom::error::Error<&'a str>> for C208 {
    fn parse(input: &'a str) -> IResult<&'a str, C208> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C208 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C210, nom::error::Error<&'a str>> for C210 {
    fn parse(input: &'a str) -> IResult<&'a str, C210> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C210 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
            _070: vars.get(6).map(|x| x.to_string()),
            _080: vars.get(7).map(|x| x.to_string()),
            _090: vars.get(8).map(|x| x.to_string()),
            _100: vars.get(9).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C211, nom::error::Error<&'a str>> for C211 {
    fn parse(input: &'a str) -> IResult<&'a str, C211> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C211 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C212 Item number identification
///
/// Goods identification for a specified source.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C212 {
    /// Item identifier                             an..35
    ///
    /// To identify an item.
    pub _010: Option<String>,
    /// Item type identification code               an..3
    ///
    /// Coded identification of an item type.
    /// 1 User or association defined code.
    /// May be used in combination with 1131/3055.
    pub _020: Option<String>,
    /// Code list identification code               an..17
    ///
    /// Code identifying a code list.
    pub _030: Option<String>,
    /// Code list responsible agency code           an..3
    ///
    /// Code specifying the agency responsible for a code list.
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C212, nom::error::Error<&'a str>> for C212 {
    fn parse(input: &'a str) -> IResult<&'a str, C212> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C212 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C213, nom::error::Error<&'a str>> for C213 {
    fn parse(input: &'a str) -> IResult<&'a str, C213> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C213 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C215, nom::error::Error<&'a str>> for C215 {
    fn parse(input: &'a str) -> IResult<&'a str, C215> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C215 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C218, nom::error::Error<&'a str>> for C218 {
    fn parse(input: &'a str) -> IResult<&'a str, C218> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C218 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C219, nom::error::Error<&'a str>> for C219 {
    fn parse(input: &'a str) -> IResult<&'a str, C219> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C219 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C220, nom::error::Error<&'a str>> for C220 {
    fn parse(input: &'a str) -> IResult<&'a str, C220> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C220 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C222, nom::error::Error<&'a str>> for C222 {
    fn parse(input: &'a str) -> IResult<&'a str, C222> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C222 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C223, nom::error::Error<&'a str>> for C223 {
    fn parse(input: &'a str) -> IResult<&'a str, C223> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C223 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C224, nom::error::Error<&'a str>> for C224 {
    fn parse(input: &'a str) -> IResult<&'a str, C224> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C224 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C228, nom::error::Error<&'a str>> for C228 {
    fn parse(input: &'a str) -> IResult<&'a str, C228> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C228 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C232 Government action
///
/// Code indicating a type of government action.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C232 {
    /// Government agency identification code
    ///
    /// Code identifying a government agency.
    pub _010: Option<String>,
    /// Government involvement code
    ///
    /// Code indicating the requirement and status of governmental involvement.
    pub _020: Option<String>,
    /// Government action code
    ///
    /// Code specifying a type of government action
    /// such as inspection, detention, fumigation, security.
    pub _030: Option<String>,
    /// Government procedure code
    ///
    /// Code specifying a government procedure.
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C232, nom::error::Error<&'a str>> for C232 {
    fn parse(input: &'a str) -> IResult<&'a str, C232> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C232 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C233 - SERVICE
///
/// To identify a service (which may constitute an additional component to a basic contract).
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C233 {
    pub _010: _7273,
    pub _020: Option<_1131>,
    pub _030: Option<_3055>,
    pub _040: Option<_7273>,
    pub _050: Option<_1131>,
    pub _060: Option<_3055>,
}

impl<'a> Parser<&'a str, C233, nom::error::Error<&'a str>> for C233 {
    fn parse(input: &'a str) -> IResult<&'a str, C233> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C233 {
            _010: vars.first().map(|x| _7273::from_str(x).unwrap()).unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(x).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(x).unwrap()),
            _040: vars.get(3).map(|x| _7273::from_str(x).unwrap()),
            _050: vars.get(4).map(|x| _1131::from_str(x).unwrap()),
            _060: vars.get(5).map(|x| _3055::from_str(x).unwrap()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C234, nom::error::Error<&'a str>> for C234 {
    fn parse(input: &'a str) -> IResult<&'a str, C234> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C234 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C235, nom::error::Error<&'a str>> for C235 {
    fn parse(input: &'a str) -> IResult<&'a str, C235> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C235 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C236, nom::error::Error<&'a str>> for C236 {
    fn parse(input: &'a str) -> IResult<&'a str, C236> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C236 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C237, nom::error::Error<&'a str>> for C237 {
    fn parse(input: &'a str) -> IResult<&'a str, C237> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C237 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C239 - TEMPERATURE SETTING
///
/// The temperature under which the goods are (to be) stored
/// or shipped.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayInnerSegment)]
pub struct C239 {
    /// Temperature value           C  n..15
    pub _010: Option<String>,
    /// Measurement unit code       C  an..3
    pub _020: Option<String>,
}

impl<'a> Parser<&'a str, C239, nom::error::Error<&'a str>> for C239 {
    fn parse(input: &'a str) -> IResult<&'a str, C239> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C239 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C270, nom::error::Error<&'a str>> for C270 {
    fn parse(input: &'a str) -> IResult<&'a str, C270> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C270 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C280 Range
///
/// Range minimum and maximum limits.
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayInnerSegment)]
pub struct C280 {
    /// Measurement unit code    C  an..3
    ///
    /// 1 See UN/ECE Recommendation 20, common code.
    pub _010: String,
    /// Range minimum value      C  an..18
    ///
    /// To specify the minimum value of a range.
    pub _020: Option<String>,
    /// Range maximum value      C  an..18
    ///
    /// To specify the maximum value of a range.
    pub _030: Option<String>,
}

impl<'a> Parser<&'a str, C280, nom::error::Error<&'a str>> for C280 {
    fn parse(input: &'a str) -> IResult<&'a str, C280> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C280 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C401, nom::error::Error<&'a str>> for C401 {
    fn parse(input: &'a str) -> IResult<&'a str, C401> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C401 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C501 Percentage details
///
/// Identification of measurement type.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C501 {
    /// Percentage type code qualifier              C      an..3
    ///
    /// Code qualifying the type of percentage.
    pub _010: String,
    /// Percentage                                  C      an..10
    ///
    /// To specify a percentage.
    pub _020: Option<String>,
    /// Percentage basis identification code        C      an..3
    ///
    /// Code specifying the basis on which a percentage is calculated.
    ///
    /// 1 User or association defined code. May be used in combination with 1131/3055.
    pub _030: Option<String>,
    /// Code list identification code               C      an..17
    ///
    /// Code identifying a code list.
    pub _040: Option<String>,
    /// Code list responsible agency code           C      an..3
    ///
    /// Code specifying the agency responsible for a code list.
    pub _050: Option<String>,
}

impl<'a> Parser<&'a str, C501, nom::error::Error<&'a str>> for C501 {
    fn parse(input: &'a str) -> IResult<&'a str, C501> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C501 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C502, nom::error::Error<&'a str>> for C502 {
    fn parse(input: &'a str) -> IResult<&'a str, C502> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C502 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C503, nom::error::Error<&'a str>> for C503 {
    fn parse(input: &'a str) -> IResult<&'a str, C503> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C503 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C506, nom::error::Error<&'a str>> for C506 {
    fn parse(input: &'a str) -> IResult<&'a str, C506> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C506 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C507 DTM  DATE/TIME/PERIOD
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C507 {
    pub _010: _2005,
    pub _020: Option<String>,
    pub _030: Option<_2379>,
}

impl<'a> Parser<&'a str, C507, nom::error::Error<&'a str>> for C507 {
    fn parse(input: &'a str) -> IResult<&'a str, C507> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C507 {
            _010: vars.first().map(|x| _2005::from_str(x).unwrap()).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| _2379::from_str(x).unwrap()),
        };
        Ok(("", output))
    }
}

/// C516 Monetary amount
///
/// Amount of goods or services stated as a
/// monetary amount in a specified currency.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C516 {
    /// Monetary amount type code qualifier
    ///
    /// Code qualifying the type of monetary amount.
    pub _010: String,
    /// Monetary amount
    ///
    /// To specify a monetary amount.
    pub _020: Option<String>,
    /// Currency identification code
    ///
    /// Code specifying a monetary unit.
    ///
    /// 1 Use ISO 4217 three alpha code.
    pub _030: Option<String>,
    /// Currency type code qualifier
    ///
    /// Code qualifying the type of currency.
    pub _040: Option<String>,
    /// Status description code
    ///
    /// Code specifying a status.
    ///
    /// 1 For transport status, use UN/ECE Recommendation 24.
    pub _050: Option<String>,
}

impl<'a> Parser<&'a str, C516, nom::error::Error<&'a str>> for C516 {
    fn parse(input: &'a str) -> IResult<&'a str, C516> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C516 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C517 - LOCATION IDENTIFICATION
///
/// Identification of a location by code or name.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C517 {
    /// Location name code
    ///
    /// Code specifying the name of the location.
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C517, nom::error::Error<&'a str>> for C517 {
    fn parse(input: &'a str) -> IResult<&'a str, C517> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C517 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C519 - RELATED LOCATION ONE IDENTIFICATION
///
/// Identification the first related location by code or name.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C519 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C519, nom::error::Error<&'a str>> for C519 {
    fn parse(input: &'a str) -> IResult<&'a str, C519> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C519 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C523, nom::error::Error<&'a str>> for C523 {
    fn parse(input: &'a str) -> IResult<&'a str, C523> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C523 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C524, nom::error::Error<&'a str>> for C524 {
    fn parse(input: &'a str) -> IResult<&'a str, C524> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C524 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C536 - CONTRACT AND CARRIAGE CONDITION
///
/// To identify a contract and carriage condition.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C536 {
    pub _010: _4065,
    pub _020: Option<_1131>,
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C536, nom::error::Error<&'a str>> for C536 {
    fn parse(input: &'a str) -> IResult<&'a str, C536> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C536 {
            _010: vars.first().map(|x| _4065::from_str(x).unwrap()).unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(x).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(x).unwrap()),
        };
        Ok(("", output))
    }
}

/// C537 - TRANSPORT PRIORITY
///
/// To indicate the priority of requested transport service.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C537 {
    pub _010: _4219,
    pub _020: Option<_1131>,
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C537, nom::error::Error<&'a str>> for C537 {
    fn parse(input: &'a str) -> IResult<&'a str, C537> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C537 {
            _010: vars
                .first()
                .map(|x: &&str| _4219::from_str(x).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(x).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(x).unwrap()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C553, nom::error::Error<&'a str>> for C553 {
    fn parse(input: &'a str) -> IResult<&'a str, C553> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C553 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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
impl<'a> Parser<&'a str, C555, nom::error::Error<&'a str>> for C555 {
    fn parse(input: &'a str) -> IResult<&'a str, C555> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C555 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C556, nom::error::Error<&'a str>> for C556 {
    fn parse(input: &'a str) -> IResult<&'a str, C556> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C556 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C601, nom::error::Error<&'a str>> for C601 {
    fn parse(input: &'a str) -> IResult<&'a str, C601> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C601 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C703 - NATURE OF CARGO
///
/// Rough classification of a type of cargo.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C703 {
    pub _010: _7085,
    pub _020: Option<_1131>,
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C703, nom::error::Error<&'a str>> for C703 {
    fn parse(input: &'a str) -> IResult<&'a str, C703> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C703 {
            _010: vars.first().map(|x| _7085::from_str(x).unwrap()).unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(x).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(x).unwrap()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C819, nom::error::Error<&'a str>> for C819 {
    fn parse(input: &'a str) -> IResult<&'a str, C819> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C819 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C821 Type of damage
///
/// To specify the type of damage to an object.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C821 {
    /// Damage type description code              C      an..3
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Damage type description                   C      an..35
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C821, nom::error::Error<&'a str>> for C821 {
    fn parse(input: &'a str) -> IResult<&'a str, C821> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C821 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C822 Damage area
///
/// To specify where the damage is on an object.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C822 {
    /// Damage area description code              C      an..4
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Damage area description                   C      an..35
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C822, nom::error::Error<&'a str>> for C822 {
    fn parse(input: &'a str) -> IResult<&'a str, C822> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C822 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C823 Type of unit/component
///
/// To identify the type of unit/component
/// of an object (e.g. lock, door, tyre).
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C823 {
    /// Unit or component type description code   C      an..3
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Unit or component type description        C      an..35
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C823, nom::error::Error<&'a str>> for C823 {
    fn parse(input: &'a str) -> IResult<&'a str, C823> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C823 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C824 Component material
///
/// To identify the material of which
/// a component is composed (e.g. steel, plastics).
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C824 {
    /// Component material description code       C      an..3
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Component material description            C      an..35
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C824, nom::error::Error<&'a str>> for C824 {
    fn parse(input: &'a str) -> IResult<&'a str, C824> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C824 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C825 Damage severity
///
/// To specify the severity of damage to an object.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C825 {
    /// Damage severity description code          C      an..3
    pub _010: Option<String>,
    /// Code list identification code             C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code         C      an..3
    pub _030: Option<String>,
    /// Damage severity description               C      an..35
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C825, nom::error::Error<&'a str>> for C825 {
    fn parse(input: &'a str) -> IResult<&'a str, C825> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C825 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C826 Action
///
/// To indicate an action which has been taken or
///  is to be taken (e.g. in relation to a certain object).
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C826 {
    /// Action request/notification description code         C      an..3
    pub _010: Option<String>,
    /// Code list identification code                        C      an..17
    pub _020: Option<String>,
    /// Code list responsible agency code                    C      an..3
    pub _030: Option<String>,
    /// Action request/notification description              C      an..35
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C826, nom::error::Error<&'a str>> for C826 {
    fn parse(input: &'a str) -> IResult<&'a str, C826> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C826 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, C827, nom::error::Error<&'a str>> for C827 {
    fn parse(input: &'a str) -> IResult<&'a str, C827> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C827 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// CNI - CONSIGNMENT INFORMATION
///
/// A segment to identify a consignment for which status details are given.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct CNI {
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

impl<'a> Parser<&'a str, CNI, nom::error::Error<&'a str>> for CNI {
    fn parse(input: &'a str) -> IResult<&'a str, CNI> {
        let (output_rest, vars) = crate::util::parse_line(input, "CNI")?;
        let output = CNI {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| C503::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct CNT {
    /// CONTROL
    ///
    /// Control total for checking integrity of a message or part of a message.
    pub _010: C270,
}

impl<'a> Parser<&'a str, CNT, nom::error::Error<&'a str>> for CNT {
    fn parse(input: &'a str) -> IResult<&'a str, CNT> {
        let (output_rest, vars) = crate::util::parse_line(input, "CNT")?;
        let (_, obj) = C270::parse(vars.first().unwrap())?;
        let output = CNT { _010: obj };
        Ok((output_rest, output))
    }
}

/// CTA - CONTACT INFORMATION
///
/// A segment to specify a contact name associated with the party.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct CTA {
    /// CONTACT FUNCTION CODE
    ///
    /// Code specifying the function of a contact (e.g. department or person).
    pub _010: Option<String>,
    /// C056 - DEPARTMENT OR EMPLOYEE DETAILS
    ///
    /// Code and/or name of a department or employee. Code preferred.
    pub _020: Option<C056>,
}

/// COD - Component details
///
/// To provide component details of an object (e.g. product, container) such as its type and the material of which it is composed.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct COD {
    pub _010: Option<C823>,
    pub _020: Option<C824>,
}

impl<'a> Parser<&'a str, COD, nom::error::Error<&'a str>> for COD {
    fn parse(input: &'a str) -> IResult<&'a str, COD> {
        let (output_rest, vars) = crate::util::parse_line(input, "COD")?;
        let output = COD {
            _010: vars.first().map(|x| C823::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C824::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}

/// COM - COMMUNICATION CONTACT
///
/// A segment to specify a communication number related to the contact.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct COM {
    /// C076 - COMMUNICATION CONTACT
    ///
    /// Communication number of a department or employee in a specified channel.
    pub _010: C076,
}

/// DAM - Damage
///
/// To specify damage including action taken.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct DAM {
    pub _010: String,
    pub _020: Option<C821>,
    pub _030: Option<C822>,
    pub _040: Option<C825>,
    pub _050: Option<C826>,
}

impl<'a> Parser<&'a str, DAM, nom::error::Error<&'a str>> for DAM {
    fn parse(input: &'a str) -> IResult<&'a str, DAM> {
        let (output_rest, vars) = crate::util::parse_line(input, "DAM")?;
        let output = DAM {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C821::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C822::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C825::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C826::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, DGS, nom::error::Error<&'a str>> for DGS {
    fn parse(input: &'a str) -> IResult<&'a str, DGS> {
        let (output_rest, vars) = crate::util::parse_line(input, "DGS")?;
        let output = DGS {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| C205::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C234::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C223::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
            _070: vars.get(6).map(|x| x.to_string()),
            _080: vars.get(7).map(|x| x.to_string()),
            _090: vars.get(8).map(|x| C235::parse(x).unwrap().1),
            _100: vars.get(9).map(|x| C236::parse(x).unwrap().1),
            _110: vars.get(10).map(|x| x.to_string()),
            _120: vars.get(11).map(|x| x.to_string()),
            _130: vars.get(12).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, DIM, nom::error::Error<&'a str>> for DIM {
    fn parse(input: &'a str) -> IResult<&'a str, DIM> {
        let (output_rest, vars) = crate::util::parse_line(input, "DIM")?;
        let output = DIM {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C211::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
}

/// DOC - DOCUMENT/MESSAGE DETAILS
///
/// A segment to specify document details related to the status code, such as indication which document is missing (status code: document missing).
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct DOC {
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

impl<'a> Parser<&'a str, DOC, nom::error::Error<&'a str>> for DOC {
    fn parse(input: &'a str) -> IResult<&'a str, DOC> {
        let (output_rest, vars) = crate::util::parse_line(input, "DOC")?;
        let output = DOC {
            _010: vars.first().map(|x| C002::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C503::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct DTM {
    pub _010: C507,
}

impl<'a> Parser<&'a str, DTM, nom::error::Error<&'a str>> for DTM {
    fn parse(input: &'a str) -> IResult<&'a str, DTM> {
        let (output_rest, vars) = crate::util::parse_line(input, "DTM")?;
        let output = DTM {
            _010: vars.first().map(|x| C507::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, EQA, nom::error::Error<&'a str>> for EQA {
    fn parse(input: &'a str) -> IResult<&'a str, EQA> {
        let (output_rest, vars) = crate::util::parse_line(input, "EQA")?;
        let output = EQA {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C237::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, EQD, nom::error::Error<&'a str>> for EQD {
    fn parse(input: &'a str) -> IResult<&'a str, EQD> {
        let (output_rest, vars) = crate::util::parse_line(input, "EQD")?;
        let output = EQD {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C237::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C224::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, EQN, nom::error::Error<&'a str>> for EQN {
    fn parse(input: &'a str) -> IResult<&'a str, EQN> {
        let (output_rest, vars) = crate::util::parse_line(input, "EQN")?;
        let output = EQN {
            _010: vars.first().map(|x| C523::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, FTX, nom::error::Error<&'a str>> for FTX {
    fn parse(input: &'a str) -> IResult<&'a str, FTX> {
        let (output_rest, vars) = crate::util::parse_line(input, "FTX")?;
        let output = FTX {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| C107::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C108::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, GID, nom::error::Error<&'a str>> for GID {
    fn parse(input: &'a str) -> IResult<&'a str, GID> {
        let (output_rest, vars) = crate::util::parse_line(input, "GID")?;
        let output = GID {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| C213::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C213::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C213::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C213::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| C213::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, GIN, nom::error::Error<&'a str>> for GIN {
    fn parse(input: &'a str) -> IResult<&'a str, GIN> {
        let (output_rest, vars) = crate::util::parse_line(input, "GIN")?;
        let output = GIN {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| C208::parse(x).unwrap().1).unwrap(),
            _030: vars.get(2).map(|x| C208::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C208::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C208::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| C208::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}

/// GOR Governmental requirements
///
/// To indicate the requirement for a specific governmental action and/or
/// procedure or which specific procedure is valid for a specific part of the transport.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct GOR {
    /// Transport movement code
    ///
    /// Code specifying the transport movement.
    pub _010: Option<String>,
    /// C232 Government action
    ///
    /// Code indicating a type of government action.
    pub _020: Option<C232>,
    /// C232 Government action
    ///
    /// Code indicating a type of government action.
    pub _030: Option<C232>,
    /// C232 Government action
    ///
    /// Code indicating a type of government action.
    pub _040: Option<C232>,
    /// C232 Government action
    ///
    /// Code indicating a type of government action.
    pub _050: Option<C232>,
}

impl<'a> Parser<&'a str, GOR, nom::error::Error<&'a str>> for GOR {
    fn parse(input: &'a str) -> IResult<&'a str, GOR> {
        let (output_rest, vars) = crate::util::parse_line(input, "GOR")?;
        let output = GOR {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| C232::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C232::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C232::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C232::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, HAN, nom::error::Error<&'a str>> for HAN {
    fn parse(input: &'a str) -> IResult<&'a str, HAN> {
        let (output_rest, vars) = crate::util::parse_line(input, "HAN")?;
        let output = HAN {
            _010: vars.first().map(|x| C524::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C218::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, LOC, nom::error::Error<&'a str>> for LOC {
    fn parse(input: &'a str) -> IResult<&'a str, LOC> {
        let (output_rest, vars) = crate::util::parse_line(input, "LOC")?;
        let output = LOC {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| C517::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C519::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C553::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, MEA, nom::error::Error<&'a str>> for MEA {
    fn parse(input: &'a str) -> IResult<&'a str, MEA> {
        let (output_rest, vars) = crate::util::parse_line(input, "MEA")?;
        let output = MEA {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C502::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C174::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
}

/// MOA Monetary amount
///
/// To specify a monetary amount.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct MOA {
    /// C516 Monetary amount
    ///
    /// Amount of goods or services stated as a monetary amount in a specified currency.
    pub _010: C516,
}

impl<'a> Parser<&'a str, MOA, nom::error::Error<&'a str>> for MOA {
    fn parse(input: &'a str) -> IResult<&'a str, MOA> {
        let (output_rest, vars) = crate::util::parse_line(input, "MOA")?;
        let output = MOA {
            _010: vars.first().map(|x| C516::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, NAD, nom::error::Error<&'a str>> for NAD {
    fn parse(input: &'a str) -> IResult<&'a str, NAD> {
        let (output_rest, vars) = crate::util::parse_line(input, "NAD")?;
        let output = NAD {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C082::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C058::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C080::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C059::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| x.to_string()),
            _070: vars.get(6).map(|x| C819::parse(x).unwrap().1),
            _080: vars.get(7).map(|x| x.to_string()),
            _090: vars.get(8).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
}

/// PIA Additional product id
///
/// To specify additional or substitutional item identification codes.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct PIA {
    /// Product identifier code qualifier
    ///
    /// Code qualifying the product identifier.
    pub _010: String,
    /// C212 Item number identification
    ///
    /// Goods identification for a specified source.
    pub _020: C212,
    /// C212 Item number identification
    ///
    /// Goods identification for a specified source.
    pub _030: Option<C212>,
    /// C212 Item number identification
    ///
    /// Goods identification for a specified source.
    pub _040: Option<C212>,
    /// C212 Item number identification
    ///
    /// Goods identification for a specified source.
    pub _050: Option<C212>,
    /// C212 Item number identification
    ///
    /// Goods identification for a specified source.
    pub _060: Option<C212>,
}

impl<'a> Parser<&'a str, PIA, nom::error::Error<&'a str>> for PIA {
    fn parse(input: &'a str) -> IResult<&'a str, PIA> {
        let (output_rest, vars) = crate::util::parse_line(input, "PIA")?;
        let output = PIA {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C212::parse(x).unwrap().1).unwrap(),
            _030: vars.get(2).map(|x| C212::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C212::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C212::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| C212::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}

/// PCD Percentage details
///
/// To specify percentage information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct PCD {
    /// Product identifier code qualifier
    ///
    /// Code qualifying the product identifier.
    pub _010: C501,
    /// Status description code
    ///
    /// Code specifying a status.
    ///
    /// 1 For transport status, use UN/ECE Recommendation 24.
    pub _020: Option<String>,
}

impl<'a> Parser<&'a str, PCD, nom::error::Error<&'a str>> for PCD {
    fn parse(input: &'a str) -> IResult<&'a str, PCD> {
        let (output_rest, vars) = crate::util::parse_line(input, "PCD")?;
        let output = PCD {
            _010: vars.first().map(|x| C501::parse(x).unwrap().1).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, PCI, nom::error::Error<&'a str>> for PCI {
    fn parse(input: &'a str) -> IResult<&'a str, PCI> {
        let (output_rest, vars) = crate::util::parse_line(input, "PCI")?;
        let output = PCI {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| C210::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| C827::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, RFF, nom::error::Error<&'a str>> for RFF {
    fn parse(input: &'a str) -> IResult<&'a str, RFF> {
        let (output_rest, vars) = crate::util::parse_line(input, "RFF")?;
        let output = RFF {
            _010: vars.first().map(|x| C506::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
}

/// RNG Range details
///
/// To identify a range.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct RNG {
    /// Range type code qualifier
    ///
    /// Code qualifying a type of range.
    pub _010: String,
    /// C280 Range
    ///
    /// Range minimum and maximum limits.
    pub _020: Option<C280>,
}

impl<'a> Parser<&'a str, RNG, nom::error::Error<&'a str>> for RNG {
    fn parse(input: &'a str) -> IResult<&'a str, RNG> {
        let (output_rest, vars) = crate::util::parse_line(input, "RNG")?;
        let output = RNG {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C280::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}
/// Syntax identifier
///
/// Identification of the agency controlling the syntax and indication of syntax level, plus the syntax version number.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment, PartialEq, Eq, Default)]
pub struct S001 {
    pub _010: _0001,
    pub _020: _0002,
}

impl<'a> Parser<&'a str, S001, nom::error::Error<&'a str>> for S001 {
    fn parse(input: &'a str) -> IResult<&'a str, S001> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S001 {
            _010: vars.first().map(|x| _0001::from_str(x).unwrap()).unwrap(),
            _020: vars.get(1).map(|x| _0002::from_str(x).unwrap()).unwrap(),
        };
        Ok(("", output))
    }
}
/// Interchange sender
///
/// Identification of the sender of the interchange.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, PartialEq, Eq)]
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

impl<'a> Parser<&'a str, S002, nom::error::Error<&'a str>> for S002 {
    fn parse(input: &'a str) -> IResult<&'a str, S002> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S002 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| {
                println!("{x:?}");
                _0007::from_str(x).unwrap()
            }),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// Interchange recipient
///
/// Identification of the recipient of the interchange.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, PartialEq, Eq)]
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

impl<'a> Parser<&'a str, S003, nom::error::Error<&'a str>> for S003 {
    fn parse(input: &'a str) -> IResult<&'a str, S003> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S003 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| _0007::from_str(x).unwrap()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// Date/time of preparation
///
/// Date/time of preparation of the interchange.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, PartialEq, Eq)]
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

impl<'a> Parser<&'a str, S004, nom::error::Error<&'a str>> for S004 {
    fn parse(input: &'a str) -> IResult<&'a str, S004> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S004 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
        };
        Ok(("", output))
    }
}

/// Recipient's reference, password
///
/// Reference or password as agreed between the communicating partners.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment, PartialEq, Eq)]
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

impl<'a> Parser<&'a str, S005, nom::error::Error<&'a str>> for S005 {
    fn parse(input: &'a str) -> IResult<&'a str, S005> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S005 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| _0025::from_str(x).unwrap()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, S009, nom::error::Error<&'a str>> for S009 {
    fn parse(input: &'a str) -> IResult<&'a str, S009> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S009 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).unwrap().to_string(),
            _030: vars.get(2).unwrap().to_string(),
            _040: vars.get(3).unwrap().to_string(),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
            _070: vars.get(6).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, S010, nom::error::Error<&'a str>> for S010 {
    fn parse(input: &'a str) -> IResult<&'a str, S010> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S010 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, S016, nom::error::Error<&'a str>> for S016 {
    fn parse(input: &'a str) -> IResult<&'a str, S016> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S016 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, S017, nom::error::Error<&'a str>> for S017 {
    fn parse(input: &'a str) -> IResult<&'a str, S017> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S017 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, S018, nom::error::Error<&'a str>> for S018 {
    fn parse(input: &'a str) -> IResult<&'a str, S018> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = S018 {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
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

impl<'a> Parser<&'a str, SEL, nom::error::Error<&'a str>> for SEL {
    fn parse(input: &'a str) -> IResult<&'a str, SEL> {
        let (output_rest, vars) = crate::util::parse_line(input, "SEL")?;
        let output = SEL {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| C215::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| C208::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, SGP, nom::error::Error<&'a str>> for SGP {
    fn parse(input: &'a str) -> IResult<&'a str, SGP> {
        let (output_rest, vars) = crate::util::parse_line(input, "SGP")?;
        let output = SGP {
            _010: vars.first().map(|x| C237::parse(x).unwrap().1).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, STS, nom::error::Error<&'a str>> for STS {
    fn parse(input: &'a str) -> IResult<&'a str, STS> {
        let (output_rest, vars) = crate::util::parse_line(input, "STS")?;
        let output = STS {
            _010: vars.first().map(|x| C601::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C555::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C556::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C556::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C556::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| C556::parse(x).unwrap().1),
            _070: vars.get(6).map(|x| C556::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, TDT, nom::error::Error<&'a str>> for TDT {
    fn parse(input: &'a str) -> IResult<&'a str, TDT> {
        let (output_rest, vars) = crate::util::parse_line(input, "TDT")?;
        let output = TDT {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| C220::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C228::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| C040::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| x.to_string()),
            _070: vars
                .get(6)
                .filter(|x| !x.is_empty())
                .map(|x| C401::parse(x).unwrap().1),
            _080: vars.get(7).map(|x| C222::parse(x).unwrap().1),
            _090: vars.get(8).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, TMD, nom::error::Error<&'a str>> for TMD {
    fn parse(input: &'a str) -> IResult<&'a str, TMD> {
        let (output_rest, vars) = crate::util::parse_line(input, "TMD")?;
        let output = TMD {
            _010: vars.first().map(|x| C219::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok((output_rest, output))
    }
}

/// TMP Temperature
///
/// To specify the temperature setting.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TMP {
    /// Temperature type code qualifier
    ///
    /// Code qualifying the type of a temperature.
    pub _010: String,
    /// Temperature setting
    pub _020: Option<C239>,
}

impl<'a> Parser<&'a str, TMP, nom::error::Error<&'a str>> for TMP {
    fn parse(input: &'a str) -> IResult<&'a str, TMP> {
        let (output_rest, vars) = crate::util::parse_line(input, "TMP")?;
        let output = TMP {
            _010: vars.first().unwrap().to_string(),
            _020: vars.get(1).map(|x| C239::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, TPL, nom::error::Error<&'a str>> for TPL {
    fn parse(input: &'a str) -> IResult<&'a str, TPL> {
        let (output_rest, vars) = crate::util::parse_line(input, "TPL")?;
        let output = TPL {
            _010: vars.first().map(|x| C222::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, TSR, nom::error::Error<&'a str>> for TSR {
    fn parse(input: &'a str) -> IResult<&'a str, TSR> {
        let (output_rest, vars) = crate::util::parse_line(input, "TSR")?;
        let output = TSR {
            _010: vars.first().map(|x| C536::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C233::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C537::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C703::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct UNA {
    _010: String,
}

impl<'a> Parser<&'a str, UNA, nom::error::Error<&'a str>> for UNA {
    fn parse(input: &'a str) -> IResult<&'a str, UNA> {
        let (output_rest, vars) = crate::util::parse_line(input, "UNA")?;
        let output = UNA {
            _010: vars.first().unwrap().to_string(),
        };
        Ok((output_rest, output))
    }
}

/// UNB Interchange header
///
/// To start, identify and specify an interchange.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
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

impl<'a> Parser<&'a str, UNB, nom::error::Error<&'a str>> for UNB {
    fn parse(input: &'a str) -> IResult<&'a str, UNB> {
        let (output_rest, vars) = crate::util::parse_line(input, "UNB")?;
        let output = UNB {
            _010: vars.first().map(|x| S001::parse(x).unwrap().1).unwrap(),
            _020: vars.get(1).map(|x| S002::parse(x).unwrap().1).unwrap(),
            _030: vars.get(2).map(|x| S003::parse(x).unwrap().1).unwrap(),
            _040: vars.get(3).map(|x| S004::parse(x).unwrap().1).unwrap(),
            _050: vars.get(4).map(|x| x.to_string()).unwrap(),
            _060: vars.get(5).map(|x| S005::parse(x).unwrap().1),
            _070: vars.get(6).map(|x| x.to_string()),
            _080: vars.get(7).map(|x| _0029::from_str(x).unwrap()),
            _090: vars.get(8).map(|x| _0031::from_str(x).unwrap()),
            _100: vars.get(9).map(|x| x.to_string()),
            _110: vars.get(10).map(|x| _0035::from_str(x).unwrap()),
        };
        Ok((output_rest, output))
    }
}

/// UNH - MESSAGE HEADER
///
/// To head, identify and specify a message.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct UNH {
    pub _010: String,
    pub _020: S009,
    pub _030: Option<String>,
    pub _040: Option<S010>,
    pub _050: Option<S016>,
    pub _060: Option<S017>,
    pub _070: Option<S018>,
}

impl<'a> Parser<&'a str, UNH, nom::error::Error<&'a str>> for UNH {
    fn parse(input: &'a str) -> IResult<&'a str, UNH> {
        let (output_rest, vars) = crate::util::parse_line(input, "UNH")?;
        let output = UNH {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| S009::parse(x).unwrap().1).unwrap(),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| S010::parse(x).unwrap().1),
            _050: vars.get(4).map(|x| S016::parse(x).unwrap().1),
            _060: vars.get(5).map(|x| S017::parse(x).unwrap().1),
            _070: vars.get(6).map(|x| S018::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct UNZ {
    _010: String,
}

impl<'a> Parser<&'a str, UNZ, nom::error::Error<&'a str>> for UNZ {
    fn parse(input: &'a str) -> IResult<&'a str, UNZ> {
        let (output_rest, vars) = crate::util::parse_line(input, "UNZ")?;
        let output = UNZ {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
        };
        Ok((output_rest, output))
    }
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

impl<'a> Parser<&'a str, UNT, nom::error::Error<&'a str>> for UNT {
    fn parse(input: &'a str) -> IResult<&'a str, UNT> {
        let (output_rest, vars) = crate::util::parse_line(input, "UNT")?;
        let output = UNT {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
        };
        Ok((output_rest, output))
    }
}
