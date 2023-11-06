use super::*;
use crate::util::Parser;
use edifact_types_macros::DisplayInnerSegment;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

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
            _010: vars.first().map(|x| _1001::from_str(clean_num(x)).unwrap()),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C076 {
    /// Communication address identifier          M      an..512
    pub _010: String,
    /// Communication address code qualifier      M      an..3
    pub _020: _3155,
}

impl<'a> Parser<&'a str, C076, nom::error::Error<&'a str>> for C076 {
    fn parse(input: &'a str) -> IResult<&'a str, C076> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C076 {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars
                .get(1)
                .map(|x| _3155::from_str(clean_num(x)).unwrap())
                .unwrap(),
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

/// Terms of delivery or transport
///
/// Terms of delivery or transport code from a specified source.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C100 {
    /// Delivery or transport terms description code
    ///
    /// Code specifying the delivery or transport terms.
    /// 1 Use UN/ECE Recommendation No. 5 Incoterms 1990. If not applicable, use appropriate code set in combination with 1131/3055.
    pub _010: Option<_4053>,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
    /// Delivery or transport terms description
    ///
    /// Free form description of delivery or transport terms.
    pub _040: Option<String>,
    /// Delivery or transport terms description
    ///
    /// Free form description of delivery or transport terms.
    pub _050: Option<String>,
}

impl<'a> Parser<&'a str, C100, nom::error::Error<&'a str>> for C100 {
    fn parse(input: &'a str) -> IResult<&'a str, C100> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C100 {
            _010: vars.first().map(|x| _4053::from_str(clean_num(x)).unwrap()),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
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

/// C186 Quantity details
///
/// Quantity information in a transaction, qualified when relevant.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C186 {
    /// Quantity type code qualifier
    ///
    /// Code qualifying the type of quantity.
    pub _010: _6063,
    /// Quantity
    ///
    /// Alphanumeric representation of a quantity.
    pub _020: String,
    /// Measurement unit code
    ///
    /// Code specifying the unit of measurement.
    /// 1 See UN/ECE Recommendation 20, common code.
    pub _030: Option<String>,
}

impl<'a> Parser<&'a str, C186, nom::error::Error<&'a str>> for C186 {
    fn parse(input: &'a str) -> IResult<&'a str, C186> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C186 {
            _010: vars
                .first()
                .map(|x| _6063::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
            _030: vars.get(2).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C200 Charge
///
/// Identification of a charge by code and/or by name.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C200 {
    /// Freight and other charges description identifier
    ///
    /// Code identifying freight and other charges.
    /// 1 Use UN/ECE Recommendation No. 2
    /// Freight costs and charges. If not applicable, use appropriate code in combination with 1131/3055.
    pub _010: Option<String>,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
    /// Freight and other charges description
    ///
    /// Free form description of freight and other charges.
    pub _040: Option<String>,
    /// Payment arrangement code
    ///
    /// Code specifying the arrangements for a payment.
    pub _050: Option<_4237>,
    /// Item identifier
    ///
    /// To identify an item.
    pub _060: Option<String>,
}

impl<'a> Parser<&'a str, C200, nom::error::Error<&'a str>> for C200 {
    fn parse(input: &'a str) -> IResult<&'a str, C200> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C200 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| _4237::from_str(clean_num(x)).unwrap()),
            _060: vars.get(5).map(|x| x.to_string()),
        };
        Ok(("", output))
    }
}

/// C203 Rate/tariff class
///
/// Identification of the applicable rate/tariff class.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C203 {
    /// Rate or tariff class description code
    ///
    /// Code specifying an applicable rate or tariff class.
    /// 1 User or association defined code. May be used in combination with 1131/3055.
    pub _010: _5243,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
    /// Rate or tariff class description
    ///
    /// Free form description of an applicable rate or tariff class.
    pub _040: Option<String>,
    /// Supplementary rate or tariff code
    ///
    /// Code specifying a supplementary rate or tariff.
    /// 1 User or association defined code. May be used in combination with 1131/3055.
    pub _050: Option<String>,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _060: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _070: Option<_3055>,
    /// Supplementary rate or tariff code
    ///
    /// Code specifying a supplementary rate or tariff.
    /// 1 User or association defined code. May be used in combination with 1131/3055.
    pub _080: Option<String>,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _090: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _100: Option<_3055>,
}

impl<'a> Parser<&'a str, C203, nom::error::Error<&'a str>> for C203 {
    fn parse(input: &'a str) -> IResult<&'a str, C203> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C203 {
            _010: vars
                .first()
                .map(|x| _5243::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| x.to_string()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _070: vars.get(6).map(|x| _3055::from_str(clean_num(x)).unwrap()),
            _080: vars.get(7).map(|x| x.to_string()),
            _090: vars.get(8).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _100: vars.get(9).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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

/// C229 Charge category
///
/// Identification of a category or a zone of charges.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C229 {
    /// Charge category code
    ///
    /// Code specifying the category of charges.
    /// 1 User or association defined code. May be used in combination with 1131/3055.   
    pub _010: _5237,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C229, nom::error::Error<&'a str>> for C229 {
    fn parse(input: &'a str) -> IResult<&'a str, C229> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C229 {
            _010: vars
                .first()
                .map(|x| _5237::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
        };
        Ok(("", output))
    }
}

/// C231 Method of payment
///
/// Code identifying the method of payment.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C231 {
    /// Transport charges payment method code
    ///
    /// Code specifying the payment method for transport charges.  
    pub _010: _4215,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C231, nom::error::Error<&'a str>> for C231 {
    fn parse(input: &'a str) -> IResult<&'a str, C231> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C231 {
            _010: vars
                .first()
                .map(|x| _4215::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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
            _010: vars
                .first()
                .map(|x| _7273::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| _7273::from_str(clean_num(x)).unwrap()),
            _050: vars.get(4).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _060: vars.get(5).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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

/// C504 Currency details
///
/// The usage to which a currency relates.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C504 {
    /// Currency usage code qualifier
    ///
    /// Code qualifying the usage of a currency.
    pub _010: _6347,
    /// Currency identification code
    ///
    /// Code specifying a monetary unit.
    pub _020: Option<String>,
    /// Currency type code qualifier
    ///
    /// Code qualifying the type of currency.
    pub _030: Option<_6343>,
    /// Currency rate value
    ///
    /// To specify the value of the multiplication factor used in expressing currency units.
    pub _040: Option<String>,
}

impl<'a> Parser<&'a str, C504, nom::error::Error<&'a str>> for C504 {
    fn parse(input: &'a str) -> IResult<&'a str, C504> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C504 {
            _010: vars
                .first()
                .map(|x| _6347::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| _6343::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| x.to_string()),
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
            _010: vars
                .first()
                .map(|x| _2005::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| _2379::from_str(clean_num(x)).unwrap()),
        };
        Ok(("", output))
    }
}

/// C509 Price information
///
/// Identification of price type, price and related details.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C509 {
    /// Price code qualifier
    ///
    /// Code qualifying a price.
    /// 1 Code set of 5387 may be used also.
    pub _010: _5125,
    /// Price amount
    ///
    /// To specify a price.
    pub _020: Option<String>,
    /// Price type code
    ///
    /// Code specifying the type of price.
    pub _030: Option<_5375>,
    /// Price specification code
    ///
    /// Code identifying pricing specification.
    pub _040: Option<_5387>,
    /// Unit price basis value
    ///
    /// To specify the basis for a unit price.
    pub _050: Option<String>,
    /// Measurement unit code
    ///
    /// Code specifying the unit of measurement.
    /// 1 See UN/ECE Recommendation 20, common code.
    pub _060: Option<String>,
}

impl<'a> Parser<&'a str, C509, nom::error::Error<&'a str>> for C509 {
    fn parse(input: &'a str) -> IResult<&'a str, C509> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C509 {
            _010: vars
                .first()
                .map(|x| _5125::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| x.to_string()),
            _030: vars.get(2).map(|x| _5375::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| _5387::from_str(clean_num(x)).unwrap()),
            _050: vars.get(4).map(|x| x.to_string()),
            _060: vars.get(5).map(|x| x.to_string()),
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

/// C528 Commodity/rate detail
///
/// Identification of commodity/rates.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayInnerSegment)]
pub struct C528 {
    /// Commodity identification code
    ///
    /// Code identifying a commodity for Customs, transport or statistical purposes (generic term).
    /// 1 User or association defined code. May be used in combination with 1131/3055.
    pub _010: Option<String>,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C528, nom::error::Error<&'a str>> for C528 {
    fn parse(input: &'a str) -> IResult<&'a str, C528> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C528 {
            _010: vars.first().map(|x| x.to_string()),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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
            _010: vars
                .first()
                .map(|x| _4065::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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
                .map(|x: &&str| _4219::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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

/// C554 Rate/tariff class detail
///
/// Identification of the applicable rate/tariff class.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayInnerSegment)]
pub struct C554 {
    /// Rate or tariff class description code
    ///
    /// Code specifying an applicable rate or tariff class.
    /// 1 User or association defined code. May be used in combination with 1131/3055.
    pub _010: Option<_5243>,
    /// Code list identification code
    ///
    /// Code identifying a code list.
    pub _020: Option<_1131>,
    /// Code list responsible agency code
    ///
    /// Code specifying the agency responsible for a code list.
    pub _030: Option<_3055>,
}

impl<'a> Parser<&'a str, C554, nom::error::Error<&'a str>> for C554 {
    fn parse(input: &'a str) -> IResult<&'a str, C554> {
        let (_, vars) = crate::util::parse_colon_section(input)?;
        let output = C554 {
            _010: vars.first().map(|x| _5243::from_str(clean_num(x)).unwrap()),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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
            _010: vars
                .first()
                .map(|x| _7085::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars.get(1).map(|x| _1131::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| _3055::from_str(clean_num(x)).unwrap()),
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
            _010: vars
                .first()
                .map(|x| _0001::from_str(clean_num(x)).unwrap())
                .unwrap(),
            _020: vars
                .get(1)
                .map(|x| _0002::from_str(clean_num(x)).unwrap())
                .unwrap(),
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
            _020: vars.get(1).map(|x| _0007::from_str(clean_num(x)).unwrap()),
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
            _020: vars.get(1).map(|x| _0007::from_str(clean_num(x)).unwrap()),
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
            _020: vars.get(1).map(|x| _0025::from_str(clean_num(x)).unwrap()),
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
