use super::*;
use crate::util::Parser;
use edifact_types_macros::DisplayOuterSegment;
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
            _030: vars.get(2).map(|x| _1225::from_str(clean_num(x)).unwrap()),
            _040: vars.get(3).map(|x| _4343::from_str(clean_num(x)).unwrap()),
        };
        Ok((output_rest, output))
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
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct COM {
    /// C076 - COMMUNICATION CONTACT
    ///
    /// Communication number of a department or employee in a specified channel.
    pub _010: C076,
}

impl<'a> Parser<&'a str, COM, nom::error::Error<&'a str>> for COM {
    fn parse(input: &'a str) -> IResult<&'a str, COM> {
        let (output_rest, vars) = crate::util::parse_line(input, "COM")?;
        let output = COM {
            _010: vars.first().map(|x| C076::parse(x).unwrap().1).unwrap(),
        };
        Ok((output_rest, output))
    }
}

/// CPI Charge payment instructions
///
/// To identify a charge.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct CPI {
    /// CONTACT FUNCTION CODE
    ///
    /// Code specifying the function of a contact (e.g. department or person).
    pub _010: Option<C229>,
    /// C056 - DEPARTMENT OR EMPLOYEE DETAILS
    ///
    /// Code and/or name of a department or employee. Code preferred.
    pub _020: Option<C231>,
    pub _030: Option<_4237>,
}

impl<'a> Parser<&'a str, CPI, nom::error::Error<&'a str>> for CPI {
    fn parse(input: &'a str) -> IResult<&'a str, CPI> {
        let (output_rest, vars) = crate::util::parse_line(input, "CPI")?;
        let output = CPI {
            _010: vars.first().map(|x| C229::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C231::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| _4237::from_str(clean_num(x)).unwrap()),
        };
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
    pub _010: Option<_3139>,
    /// C056 - DEPARTMENT OR EMPLOYEE DETAILS
    ///
    /// Code and/or name of a department or employee. Code preferred.
    pub _020: Option<C056>,
}

impl<'a> Parser<&'a str, CTA, nom::error::Error<&'a str>> for CTA {
    fn parse(input: &'a str) -> IResult<&'a str, CTA> {
        let (output_rest, vars) = crate::util::parse_line(input, "CTA")?;
        let output = CTA {
            _010: vars.first().map(|x| _3139::from_str(clean_num(x)).unwrap()),
            _020: vars.get(1).map(|x| C056::parse(x).unwrap().1),
        };
        Ok((output_rest, output))
    }
}

/// Currencies
///
/// A segment to specify a contact name associated with the party.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct CUX {
    /// Currency details
    ///
    /// The usage to which a currency relates.
    pub _010: Option<C504>,
    /// Currency details
    ///
    /// The usage to which a currency relates.
    pub _020: Option<C504>,
    /// Currency exchange rate
    ///
    /// To specify the rate at which one specified currency is expressed in another specified currency.
    pub _030: Option<String>,
    /// Exchange rate currency market identifier
    ///
    /// To identify an exchange rate currency market.
    pub _040: Option<_6341>,
}

impl<'a> Parser<&'a str, CUX, nom::error::Error<&'a str>> for CUX {
    fn parse(input: &'a str) -> IResult<&'a str, CUX> {
        let (output_rest, vars) = crate::util::parse_line(input, "CUX")?;
        let output = CUX {
            _010: vars.first().map(|x| C504::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C504::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| x.to_string()),
            _040: vars.get(3).map(|x| _6341::from_str(clean_num(x)).unwrap()),
        };
        Ok((output_rest, output))
    }
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

/// GDS Nature of cargo
///
/// To indicate the type of cargo as a general classification.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment)]
pub struct GDS {
    /// C703 Nature of cargo
    ///
    /// Rough classification of a type of cargo.
    pub _010: C703,
}

impl<'a> Parser<&'a str, GDS, nom::error::Error<&'a str>> for GDS {
    fn parse(input: &'a str) -> IResult<&'a str, GDS> {
        let (output_rest, vars) = crate::util::parse_line(input, "GDS")?;
        let output = GDS {
            _010: vars.first().map(|x| C703::parse(x).unwrap().1).unwrap(),
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

/// PRI Price details
///
/// To specify price information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct PRI {
    /// C509 Price information
    ///
    /// Identification of price type, price and related details.
    pub _010: Option<C509>,
    /// Sub-line item price change operation code
    ///
    /// Code specifying the price change operation for a sub- line item.
    pub _020: Option<_5213>,
}

impl<'a> Parser<&'a str, PRI, nom::error::Error<&'a str>> for PRI {
    fn parse(input: &'a str) -> IResult<&'a str, PRI> {
        let (output_rest, vars) = crate::util::parse_line(input, "PRI")?;
        let output = PRI {
            _010: vars.first().map(|x| C509::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| _5213::from_str(clean_num(x)).unwrap()),
        };
        Ok((output_rest, output))
    }
}

/// QTY Quantity
///
/// To specify a pertinent quantity.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment)]
pub struct QTY {
    /// C186 Quantity details
    ///
    /// Quantity information in a transaction, qualified when relevant.
    pub _010: C186,
}

impl<'a> Parser<&'a str, QTY, nom::error::Error<&'a str>> for QTY {
    fn parse(input: &'a str) -> IResult<&'a str, QTY> {
        let (output_rest, vars) = crate::util::parse_line(input, "QTY")?;
        let output = QTY {
            _010: vars.first().map(|x| C186::parse(x).unwrap().1).unwrap(),
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
    /// C555 - STATUS
    ///
    /// To specify a status.
    pub _020: Option<C555>,
    /// C556 - STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _030: Option<C556>,
    /// C556 - STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _040: Option<C556>,
    /// C556 - STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _050: Option<C556>,
    /// C556 - STATUS REASON
    ///
    /// To specify the reason for a status.
    pub _060: Option<C556>,
    /// C556 - STATUS REASON
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

/// TCC Transport charge/rate calculations
///
/// To specify charges.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TCC {
    /// C200 Charge
    ///
    /// Identification of a charge by code and/or by name.
    pub _010: Option<C200>,
    /// C203 Rate/tariff class
    ///
    /// Identification of the applicable rate/tariff class.
    pub _020: Option<C203>,
    /// C528 Commodity/rate detail
    ///
    /// Identification of commodity/rates.
    pub _030: Option<C528>,
    /// C554 Rate/tariff class detail
    ///
    /// Identification of the applicable rate/tariff class.
    pub _040: Option<C554>,
}

impl<'a> Parser<&'a str, TCC, nom::error::Error<&'a str>> for TCC {
    fn parse(input: &'a str) -> IResult<&'a str, TCC> {
        let (output_rest, vars) = crate::util::parse_line(input, "TCC")?;
        let output = TCC {
            _010: vars.first().map(|x| C200::parse(x).unwrap().1),
            _020: vars.get(1).map(|x| C203::parse(x).unwrap().1),
            _030: vars.get(2).map(|x| C528::parse(x).unwrap().1),
            _040: vars.get(3).map(|x| C554::parse(x).unwrap().1),
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

/// TOD Terms of delivery or transport
///
/// To specify terms of delivery or transport.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment)]
pub struct TOD {
    /// Delivery or transport terms function code
    ///
    /// Code specifying the function of delivery or transport terms.
    pub _010: Option<_4055>,
    /// Transport charges payment method
    ///
    /// Code specifying the payment method for transport charges.
    pub _020: Option<_4215>,
    /// Terms of delivery or transport
    ///
    /// Terms of delivery or transport code from a specified source.
    pub _030: Option<C100>,
}

impl<'a> Parser<&'a str, TOD, nom::error::Error<&'a str>> for TOD {
    fn parse(input: &'a str) -> IResult<&'a str, TOD> {
        let (output_rest, vars) = crate::util::parse_line(input, "TOD")?;
        let output = TOD {
            _010: vars.first().map(|x| _4055::from_str(clean_num(x)).unwrap()),
            _020: vars.get(1).map(|x| _4215::from_str(clean_num(x)).unwrap()),
            _030: vars.get(2).map(|x| C100::parse(x).unwrap().1),
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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayOuterSegment)]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, DisplayOuterSegment)]
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
            _080: vars.get(7).map(|x| _0029::from_str(clean_num(x)).unwrap()),
            _090: vars.get(8).map(|x| _0031::from_str(clean_num(x)).unwrap()),
            _100: vars.get(9).map(|x| x.to_string()),
            _110: vars.get(10).map(|x| _0035::from_str(clean_num(x)).unwrap()),
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

/// UNZ Interchange trailer
///
/// To end and check the completeness of an interchange.
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayOuterSegment)]
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

impl<'a> Parser<&'a str, UNZ, nom::error::Error<&'a str>> for UNZ {
    fn parse(input: &'a str) -> IResult<&'a str, UNZ> {
        let (output_rest, vars) = crate::util::parse_line(input, "UNZ")?;
        let output = UNZ {
            _010: vars.first().map(|x| x.to_string()).unwrap(),
            _020: vars.get(1).map(|x| x.to_string()).unwrap(),
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
