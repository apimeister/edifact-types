use super::*;
use crate::util::{clean_num, Parser};
use edifact_types_macros::{DisplayOuterSegment, ParseSegment};
use nom::{bytes::complete::take_until, character::complete::newline, IResult};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

/// BGM - BEGINNING OF MESSAGE
///
/// A segment indicating the beginning of a message and identifying the consignment for which status is being reported.
#[derive(Debug, Serialize, Deserialize, Clone, Default, DisplayOuterSegment, ParseSegment)]
pub struct BGM {
    pub _010: Option<C002>,
    pub _020: Option<C106>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

/// CNI - CONSIGNMENT INFORMATION
///
/// A segment to identify a consignment for which status details are given.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
pub struct CNT {
    /// CONTROL
    ///
    /// Control total for checking integrity of a message or part of a message.
    pub _010: C270,
}

/// COD - Component details
///
/// To provide component details of an object (e.g. product, container) such as its type and the material of which it is composed.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
pub struct COD {
    pub _010: Option<C823>,
    pub _020: Option<C824>,
}

/// COM - COMMUNICATION CONTACT
///
/// A segment to specify a communication number related to the contact.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment, ParseSegment)]
pub struct COM {
    /// C076 - COMMUNICATION CONTACT
    ///
    /// Communication number of a department or employee in a specified channel.
    pub _010: C076,
}

/// CPI Charge payment instructions
///
/// To identify a charge.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// CTA - CONTACT INFORMATION
///
/// A segment to specify a contact name associated with the party.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// Currencies
///
/// A segment to specify a contact name associated with the party.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// DAM - Damage
///
/// To specify damage including action taken.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
pub struct DAM {
    pub _010: String,
    pub _020: Option<C821>,
    pub _030: Option<C822>,
    pub _040: Option<C825>,
    pub _050: Option<C826>,
}

/// DGS - DANGEROUS GOODS
///
/// A segment to specify dangerous goods details related to the goods item.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
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

#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
pub struct DTM {
    pub _010: C507,
}

/// EQA - ATTACHED EQUIPMENT
///
/// A segment identifying attached equipment or related equipment such as a chassis attached to a container.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
pub struct EQN {
    /// C523 - NUMBER OF UNIT DETAILS
    ///
    /// Identification of number of units and its purpose.
    pub _010: C523,
}

/// FTX - FREE TEXT
///
/// A segment specifying free form or processable supplementary or other information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// GDS Nature of cargo
///
/// To indicate the type of cargo as a general classification.
#[derive(Debug, Serialize, Deserialize, Clone, DisplayOuterSegment, ParseSegment)]
pub struct GDS {
    /// C703 Nature of cargo
    ///
    /// Rough classification of a type of cargo.
    pub _010: C703,
}

/// GID - GOODS ITEM DETAILS
///
/// A segment identifying a goods item.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// GOR Governmental requirements
///
/// To indicate the requirement for a specific governmental action and/or
/// procedure or which specific procedure is valid for a specific part of the transport.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// HAN - HANDLING INSTRUCTIONS
///
/// A segment identifying handling instructions.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// MOA Monetary amount
///
/// To specify a monetary amount.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
pub struct MOA {
    /// C516 Monetary amount
    ///
    /// Amount of goods or services stated as a monetary amount in a specified currency.
    pub _010: C516,
}

/// NAD - NAME AND ADDRESS
///
/// A segment specifying the name and/or address associated with the event such as notify party, terminal address, trucking company for gate move.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// PIA Additional product id
///
/// To specify additional or substitutional item identification codes.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// PCD Percentage details
///
/// To specify percentage information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// PCI - PACKAGE IDENTIFICATION
///
/// A segment specifying marks related to the transport line items.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// PRI Price details
///
/// To specify price information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// QTY Quantity
///
/// To specify a pertinent quantity.
#[derive(Debug, Serialize, Deserialize, DisplayOuterSegment, ParseSegment)]
pub struct QTY {
    /// C186 Quantity details
    ///
    /// Quantity information in a transaction, qualified when relevant.
    pub _010: C186,
}

/// RFF - REFERENCE
///
/// A segment to specify a reference number to equipment.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
pub struct RFF {
    /// C506 - REFERENCE
    ///
    /// Identification of a reference.
    pub _010: C506,
}

/// RNG Range details
///
/// To identify a range.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// SEL - SEAL NUMBER
///
/// A segment identifying seal and seal issuer associated with the equipment.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// TCC Transport charge/rate calculations
///
/// To specify charges.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// TDT - DETAILS OF TRANSPORT
///
/// A segment identifying conveyance related to the status or event such as flight, vessel/voyage.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// TMP Temperature
///
/// To specify the temperature setting.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
pub struct TMP {
    /// Temperature type code qualifier
    ///
    /// Code qualifying the type of a temperature.
    pub _010: String,
    /// Temperature setting
    pub _020: Option<C239>,
}

/// TOD Terms of delivery or transport
///
/// To specify terms of delivery or transport.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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

/// TPL - TRANSPORT PLACEMENT
///
/// A segment to identify the means of transport to which the equipment is linked, necessary in cases where this forms the key to retrieve relevant information.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
pub struct TPL {
    /// C222 - TRANSPORT IDENTIFICATION
    ///
    /// Code and/or name identifying the means of transport.
    pub _010: C222,
}

/// TSR - TRANSPORT SERVICE REQUIREMENTS
///
/// A segment identifying the transport service relating to the consignment.
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
        let (rest, _) = opt(newline)(rest)?;
        if vars.len() != 9 {
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
#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
pub struct UNH {
    pub _010: String,
    pub _020: S009,
    pub _030: Option<String>,
    pub _040: Option<S010>,
    pub _050: Option<S016>,
    pub _060: Option<S017>,
    pub _070: Option<S018>,
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

#[derive(Debug, Serialize, Deserialize, Default, DisplayOuterSegment, ParseSegment)]
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
