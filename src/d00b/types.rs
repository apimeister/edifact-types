use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Syntax identifier
///
/// Coded identification of the agency controlling a syntax and syntax level used in an interchange.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq, Default)]
pub enum _0001 {
    /// UN/ECE level A
    /// As defined in the basic code table of ISO 646 with the exceptions of lower case letters, alternative graphic character allocations and national or application-oriented graphic character allocations.
    UNOA,
    /// UN/ECE level B
    /// As defined in the basic code table of ISO 646 with the exceptions of alternative graphic character allocations and national or application-oriented graphic character allocations.
    UNOB,
    /// UN/ECE level C
    /// As defined in ISO 8859-1 : Information processing - Part 1: Latin alphabet No. 1.
    #[default]
    UNOC,
    /// UN/ECE level D
    /// As defined in ISO 8859-2 : Information processing - Part 2: Latin alphabet No. 2.
    UNOD,
    /// UN/ECE level E
    /// As defined in ISO 8859-5 : Information processing - Part 5: Latin/Cyrillic alphabet.
    UNOE,
    /// UN/ECE level F
    /// As defined in ISO 8859-7 : Information processing - Part 7: Latin/Greek alphabet.
    UNOF,
}

/// Syntax version number
///
/// Version number of the syntax identified in the syntax identifier (0001)
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
pub enum _0002 {
    /// Version 1
    /// ISO 9735:1988.
    #[serde(rename = "1")]
    _1,
    /// Version 2
    /// ISO 9735:1990.
    #[serde(rename = "2")]
    _2,
    /// Version 3
    /// ISO 9735 Amendment 1:1992.
    #[default]
    #[serde(rename = "3")]
    _3,
}

/// Partner identification code qualifier
///
/// Qualifier referring to the source of codes for the identifiers of interchanging partners.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum _0007 {
    /// DUNS (Data Universal Numbering System)
    ///
    /// Partner identification code assigned by Dun & Bradstreet.
    #[serde(rename = "1")]
    _1,
    /// IATA (International Air Transport Association)
    ///
    /// Partner identification code assigned by the International Air Transport Association.
    #[serde(rename = "4")]
    _4,
    /// INSEE (Institut National de la Statistique et des Etudes Economiques) - SIRET
    ///
    /// French national statistics institute. SIRET means Systeme Informatique du Repertoire des entreprises et de leurs ETablissements.
    #[serde(rename = "5")]
    _5,
    /// UCC Communications ID (Uniform Code Council Communications Identifier)
    ///
    /// The Uniform Code Council Communications Identifier is a ten digit code used to uniquely identify physical and logical locations.
    #[serde(rename = "8")]
    _8,
    /// DUNS (Data Universal Numbering System) with 4 digit suffix
    ///
    /// Partner identification code assigned by Dun & Bradstreet with the 4 digit suffix.
    #[serde(rename = "9")]
    _9,
    /// Telephone number
    ///
    /// Partner identification code corresponds to the partner telephone number.
    #[serde(rename = "12")]
    _12,
    /// EAN (European Article Numbering Association)
    ///
    /// Partner identification code assigned by the European Article Numbering Association.
    #[serde(rename = "14")]
    _14,
    /// AIAG (Automotive Industry Action Group)
    ///
    /// Partner identification code assigned by the Automotive Industry Action Group.
    #[serde(rename = "18")]
    _18,
    /// INSEE (Institut National de la Statistique et des Etudes Economiques) - SIREN
    ///
    /// French national statistics institute. SIREN means Systeme Informatique du Repertoire des ENtreprises (et de leurs etablissements).
    #[serde(rename = "22")]
    _22,
    /// ISO 6523: Organization identification
    ///
    /// Partner identification code specified in ISO 6523 (Structures for the identification of organizations).
    #[serde(rename = "30")]
    _30,
    /// DIN (Deutsches Institut fuer Normung)
    ///
    /// German standardization institute.
    #[serde(rename = "31")]
    _31,
    /// BfA (Bundesversicherungsanstalt fuer Angestellte)
    ///
    /// German social security association.
    #[serde(rename = "33")]
    _33,
    /// National Statistical Agency
    ///
    /// Partner identification code assigned by a national statistical agency.
    #[serde(rename = "34")]
    _34,
    /// GEIS (General Electric Information Services)
    ///
    /// Partner identification code assigned by General Electric Information Services.
    #[serde(rename = "51")]
    _51,
    /// INS (IBM Network Services)
    ///
    /// Partner identification code assigned by IBM Network Services.
    #[serde(rename = "52")]
    _52,
    /// Datenzentrale des Einzelhandels
    ///
    /// German data centre for retail trade.
    #[serde(rename = "53")]
    _53,
    /// Bundesverband der Deutschen Baustoffhaendler
    ///
    /// German building material trade association.
    #[serde(rename = "54")]
    _54,
    /// Bank identifier code
    ///
    /// Partner identification code corresponds to the partner bank identification code.
    #[serde(rename = "55")]
    _55,
    /// KTNet (Korea Trade Network Services)
    ///
    /// Partner identification code assigned by Korea Trade Network Services.
    #[serde(rename = "57")]
    _57,
    /// UPU (Universal Postal Union)
    ///
    /// Partner identification code assigned by the Universal Postal Union.
    #[serde(rename = "58")]
    _58,
    /// ODETTE (Organization for Data Exchange through Tele- Transmission in Europe)
    ///
    /// European automotive industry project.
    #[serde(rename = "59")]
    _59,
    /// SCAC (Standard Carrier Alpha Code)
    ///
    /// Directory of standard multimodal carriers and tariff agent codes. The SCAC lists and codes transportation companies.
    #[serde(rename = "61")]
    _61,
    /// ECA (Electronic Commerce Australia)
    ///
    /// Australian association for electronic commerce.
    #[serde(rename = "63")]
    _63,
    /// TELEBOX 400 (Deutsche Telekom)
    ///
    /// German telecommunications service.
    #[serde(rename = "65")]
    _65,
    /// NHS (National Health Service)
    ///
    /// United Kingdom National Health Service.
    #[serde(rename = "80")]
    _80,
    /// Statens Teleforvaltning
    ///
    /// Norwegian telecommunications regulatory authority (NTRA).
    #[serde(rename = "82")]
    _82,
    /// Athens Chamber of Commerce
    ///
    /// Greek Chamber of Commerce.
    #[serde(rename = "84")]
    _84,
    /// Swiss Chamber of Commerce
    ///
    /// Swiss Chamber of Commerce.
    #[serde(rename = "85")]
    _85,
    /// US Council for International Business
    ///
    /// United States Council for International Business.
    #[serde(rename = "86")]
    _86,
    /// National Federation of Chambers of Commerce and Industry
    ///
    /// Belgium National Federation of Chambers of Commerce and Industry.
    #[serde(rename = "87")]
    _87,
    /// Association of British Chambers of Commerce
    ///
    /// Association of British Chambers of Commerce.
    #[serde(rename = "89")]
    _89,
    /// SITA (Societe Internationale de Telecommunications Aeronautiques)
    ///
    /// SITA (Societe Internationale de Telecommunications Aeronautiques).
    #[serde(rename = "90")]
    _90,
    /// Assigned by seller or seller's agent
    ///
    /// Partner identification code assigned by the seller or seller's agent.
    #[serde(rename = "91")]
    _91,
    /// Assigned by buyer or buyer's agent
    ///
    /// Partner identification code assigned by the buyer or buyer's agent.
    #[serde(rename = "92")]
    _92,
    /// TW, Trade-van
    ///
    /// Trade-van is an EDI VAN service center for customs, transport, and insurance in national and international trade.
    #[serde(rename = "103")]
    _103,
    /// BCNR (Telekurs Banken Clearing Number)
    ///
    /// Swiss national bank number assigned by Telekurs AG for the purpose of identifying a non-clearing banking institution.
    #[serde(rename = "128")]
    _128,
    /// BPI (Telekurs Business Partner Identification)
    ///
    /// Swiss national business number assigned by Telekurs AG for the purpose of identifying a non-clearing banking institution.
    #[serde(rename = "129")]
    _129,
    /// Mutually defined
    ///
    /// Mutually defined between trading partners.
    #[strum(ascii_case_insensitive)]
    ZZZ,
}

/// Recipient's reference/password qualifier
///
/// Qualifier for the recipient's reference or password.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0025 {
    /// Reference
    /// Recipient's reference/password is a reference.
    AA,
    /// Password
    /// Recipient's reference/password is a password.
    BB,
}

/// Processing priority code
///
/// Code determined by the sender requesting processing priority for the interchange.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0029 {
    /// Highest priority
    /// Requested processing priority is the highest.
    A,
}

/// Acknowledgement request
///
/// Code determined by the sender for acknowledgement of the interchange.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum _0031 {
    /// Requested
    /// Acknowledgement is requested.
    #[serde(rename = "1")]
    _1,
}

/// Test indicator
///
/// Indication that the interchange is a test.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum _0035 {
    /// Interchange is a test
    /// Indicates that the interchange is a test.
    #[serde(rename = "1")]
    _1,
}

/// Document name code
///
/// Code specifying the document name.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _1001 {
    /// Certificate of analysis
    ///
    /// Certificate providing the values of an analysis.
    #[serde(rename = "1")]
    _1,
    /// Certificate of conformity
    ///
    /// Certificate certifying the conformity to predefined definitions.
    #[serde(rename = "2")]
    _2,
    /// Certificate of quality
    ///
    /// Certificate certifying the quality of goods, services etc.
    #[serde(rename = "3")]
    _3,
    /// Test report
    ///
    /// Report providing the results of a test session.
    #[serde(rename = "4")]
    _4,
    /// Product performance report
    ///
    /// Report specifying the performance values of products.
    #[serde(rename = "5")]
    _5,
    /// Product specification report
    ///
    /// Report providing specification values of products.
    #[serde(rename = "6")]
    _6,
    /// Process data report
    ///
    /// Reports on events during production process.
    #[serde(rename = "7")]
    _7,
    /// First sample test report
    ///
    ///
    #[serde(rename = "8")]
    _8,
    /// Price/sales catalogue
    ///
    ///
    #[serde(rename = "9")]
    _9,
    /// Party information
    ///
    /// Document/message providing basic data concerning a party.
    #[serde(rename = "10")]
    _10,
    /// Federal label approval
    ///
    /// A pre-approved document relating to federal label approval requirements.
    #[serde(rename = "11")]
    _11,
    /// Mill certificate
    ///
    /// Certificate certifying a specific quality of agricultural products.
    #[serde(rename = "12")]
    _12,
    /// Post receipt
    ///
    /// Document/message which evidences the transport of goods by post (e.g. mail, parcel, etc.).
    #[serde(rename = "13")]
    _13,
    /// Weight certificate
    ///
    /// Certificate certifying the weight of goods.
    #[serde(rename = "14")]
    _14,
    /// Weight list
    ///
    /// Document/message specifying the weight of goods.
    #[serde(rename = "15")]
    _15,
    /// Certificate
    ///
    /// Document by means of which the documentary credit applicant specifies the conditions for the certificate and by whom the certificate is to be issued.
    #[serde(rename = "16")]
    _16,
    /// Combined certificate of value and origin
    ///
    /// Document identifying goods in which the issuing authority expressly certifies that the goods originate in a specific country or part of, or group of countries. It also states the price and/or cost of the goods with the purpose of determining the customs origin.
    #[serde(rename = "17")]
    _17,
    /// Movement certificate A.TR.1
    ///
    /// Specific form of transit declaration issued by the exporter (movement certificate).
    #[serde(rename = "18")]
    _18,
    /// Certificate of quantity
    ///
    /// Certificate certifying the quantity of goods, services etc.
    #[serde(rename = "19")]
    _19,
    /// Quality data message
    ///
    /// Usage of QALITY-message.
    #[serde(rename = "20")]
    _20,
    /// Query
    ///
    /// Request information based on defined criteria.
    #[serde(rename = "21")]
    _21,
    /// Response to query
    ///
    /// Self-explanatory.
    #[serde(rename = "22")]
    _22,
    /// Status information
    ///
    /// Information regarding the status of a related message.
    #[serde(rename = "23")]
    _23,
    /// Restow
    ///
    /// Message/document identifying containers that have been unloaded and then reloaded onto the same means of transport.
    #[serde(rename = "24")]
    _24,
    /// Container discharge list
    ///
    /// Message/document itemising containers to be discharged from vessel.
    #[serde(rename = "25")]
    _25,
    /// Corporate superannuation contributions advice
    ///
    /// Document/message providing contributions advice used for corporate superannuation schemes.
    #[serde(rename = "26")]
    _26,
    /// Industry superannuation contributions advice
    ///
    /// Document/message providing contributions advice used for superannuation schemes which are industry wide.
    #[serde(rename = "27")]
    _27,
    /// Corporate superannuation member maintenance message
    ///
    /// Member maintenance message used for corporate superannuation schemes.
    #[serde(rename = "28")]
    _28,
    /// Industry superannuation member maintenance message
    ///
    /// Member maintenance message used for industry wide superannuation schemes.
    #[serde(rename = "29")]
    _29,
    /// Life insurance payroll deductions advice
    ///
    /// Payroll deductions advice used in the life insurance industry.
    #[serde(rename = "30")]
    _30,
    /// Underbond request
    ///
    /// A Message/document requesting to move cargo from one Customs control point to another.
    #[serde(rename = "31")]
    _31,
    /// Underbond approval
    ///
    /// A message/document issuing Customs approval to move cargo from one Customs control point to another.
    #[serde(rename = "32")]
    _32,
    /// Certificate of sealing of export meat lockers
    ///
    /// Document / message issued by the authority in the exporting country evidencing the sealing of export meat lockers.
    #[serde(rename = "33")]
    _33,
    /// Cargo status
    ///
    /// Message identifying the status of cargo.
    #[serde(rename = "34")]
    _34,
    /// Inventory report
    ///
    /// A message specifying information relating to held inventories.
    #[serde(rename = "35")]
    _35,
    /// Identity card
    ///
    /// Official document to identify a person.
    #[serde(rename = "36")]
    _36,
    /// Response to a trade statistics message
    ///
    /// Document/message in which the competent national authorities provide a declarant with an acceptance or a rejection about a received declaration for European statistical purposes.
    #[serde(rename = "37")]
    _37,
    /// Vaccination certificate
    ///
    /// Official document proving immunisation against certain diseases.
    #[serde(rename = "38")]
    _38,
    /// Passport
    ///
    /// An official document giving permission to travel in foreign countries.
    #[serde(rename = "39")]
    _39,
    /// Driving licence (national)
    ///
    /// An official document giving permission to drive a car in a given country.
    #[serde(rename = "40")]
    _40,
    /// Driving licence (international)
    ///
    /// An official document giving a native of one country permission to drive a vehicle in certain other countries.
    #[serde(rename = "41")]
    _41,
    /// Free pass
    ///
    /// A document giving free access to a service.
    #[serde(rename = "42")]
    _42,
    /// Season ticket
    ///
    /// A document giving access to a service for a determined period of time.
    #[serde(rename = "43")]
    _43,
    /// Transport status report
    ///
    /// A message to report the transport status and/or change in the transport status (i.e. event) between agreed parties.
    #[serde(rename = "44")]
    _44,
    /// Transport status request
    ///
    /// A message to request a transport status report (e.g. through the International multimodal status report message IFSTA).
    #[serde(rename = "45")]
    _45,
    /// Banking status
    ///
    /// A banking status document and/or message.
    #[serde(rename = "46")]
    _46,
    /// Extra-Community trade statistical declaration
    ///
    /// Document/message in which a declarant provides information about extra-Community trade of goods required by the body responsible for the collection of trade statistics. Trade by a country in the European Union with a country outside the European Union.
    #[serde(rename = "47")]
    _47,
    /// Written instructions in conformance with ADR article number 10385
    ///
    /// Written instructions relating to dangerous goods and defined in the European Agreement of Dangerous Transport by Road known as ADR (Accord europeen relatif au transport international des marchandises Dangereuses par Route).
    #[serde(rename = "48")]
    _48,
    /// Damage certification
    ///
    /// Official certification that damages to the goods to be transported have been discovered.
    #[serde(rename = "49")]
    _49,
    /// Validated priced tender
    ///
    /// A validated priced tender.
    #[serde(rename = "50")]
    _50,
    /// Price/sales catalogue response
    ///
    /// A document providing a response to a previously sent price/sales catalogue.
    #[serde(rename = "51")]
    _51,
    /// Price negotiation result
    ///
    /// A document providing the result of price negotiations.
    #[serde(rename = "52")]
    _52,
    /// Safety and hazard data sheet
    ///
    /// Document or message to supply advice on a dangerous or hazardous material to industrial customers so as to enable them to take measures to protect their employees and the environment from any potential harmful effects from these material.
    #[serde(rename = "53")]
    _53,
    /// Legal statement of an account
    ///
    /// A statement of an account containing the booked items as in the ledger of the account servicing financial institution.
    #[serde(rename = "54")]
    _54,
    /// Listing statement of an account
    ///
    /// A statement from the account servicing financial institution containing items pending to be booked.
    #[serde(rename = "55")]
    _55,
    /// Closing statement of an account
    ///
    /// Last statement of a period containing the interest calculation and the final balance of the last entry date.
    #[serde(rename = "56")]
    _56,
    /// Transport equipment on-hire report
    ///
    /// Report on the movement of containers or other items of transport equipment to record physical movement activity and establish the beginning of a rental period.
    #[serde(rename = "57")]
    _57,
    /// Transport equipment off-hire report
    ///
    /// Report on the movement of containers or other items of transport equipment to record physical movement activity and establish the end of a rental period.
    #[serde(rename = "58")]
    _58,
    /// Treatment - nil outturn
    ///
    /// No shortage, surplus or damaged outturn resulting from container vessel unpacking.
    #[serde(rename = "59")]
    _59,
    /// Treatment - time-up underbond
    ///
    /// Movement type indicator: goods are moved under customs control for warehousing due to being time-up.
    #[serde(rename = "60")]
    _60,
    /// Treatment - underbond by sea
    ///
    /// Movement type indicator: goods are to move by sea under customs control to a customs office where formalities will be completed.
    #[serde(rename = "61")]
    _61,
    /// Treatment - personal effect
    ///
    /// Cargo consists of personal effects.
    #[serde(rename = "62")]
    _62,
    /// Treatment - timber
    ///
    /// Cargo consists of timber.
    #[serde(rename = "63")]
    _63,
    /// Preliminary credit assessment
    ///
    /// Document/message issued either by a factor to indicate his preliminary credit assessment on a buyer, or by a seller to request a factor's preliminary credit assessment on a buyer.
    #[serde(rename = "64")]
    _64,
    /// Credit cover
    ///
    /// Document/message issued either by a factor to give a credit cover on a buyer, or by a seller to request a factor's credit cover.
    #[serde(rename = "65")]
    _65,
    /// Current account
    ///
    /// Document/message issued by a factor to indicate the money movements of a seller's or another factor's account with him.
    #[serde(rename = "66")]
    _66,
    /// Commercial dispute
    ///
    /// Document/message issued by a party (usually the buyer) to indicate that one or more invoices or one or more credit notes are disputed for payment.
    #[serde(rename = "67")]
    _67,
    /// Chargeback
    ///
    /// Document/message issued by a factor to a seller or to another factor to indicate that the rest of the amounts of one or more invoices uncollectable from buyers are charged back to clear the invoice(s) off the ledger.
    #[serde(rename = "68")]
    _68,
    /// Reassignment
    ///
    /// Document/message issued by a factor to a seller or to another factor to reassign an invoice or credit note previously assigned to him.
    #[serde(rename = "69")]
    _69,
    /// Collateral account
    ///
    /// Document message issued by a factor to indicate the movements of invoices, credit notes and payments of a seller's account.
    #[serde(rename = "70")]
    _70,
    /// Request for payment
    ///
    /// Document/message issued by a creditor to a debtor to request payment of one or more invoices past due.
    #[serde(rename = "71")]
    _71,
    /// Unship permit
    ///
    /// A message or document issuing permission to unship cargo.
    #[serde(rename = "72")]
    _72,
    /// Statistical definitions
    ///
    /// Transmission of one or more statistical definitions.
    #[serde(rename = "73")]
    _73,
    /// Statistical data
    ///
    /// Transmission of one or more items of data or data sets.
    #[serde(rename = "74")]
    _74,
    /// Request for statistical data
    ///
    /// Request for one or more items or data sets of statistical data.
    #[serde(rename = "75")]
    _75,
    /// Call-off delivery
    ///
    /// Document/message to provide split quantities and delivery dates referring to a previous delivery instruction.
    #[serde(rename = "76")]
    _76,
    /// Consignment status report
    ///
    /// Message covers information about the consignment status.
    #[serde(rename = "77")]
    _77,
    /// Inventory movement advice
    ///
    /// Advice of inventory movements.
    #[serde(rename = "78")]
    _78,
    /// Inventory status advice
    ///
    /// Advice of stock on hand.
    #[serde(rename = "79")]
    _79,
    /// Debit note related to goods or services
    ///
    /// Debit information related to a transaction for goods or services to the relevant party.
    #[serde(rename = "80")]
    _80,
    /// Credit note related to goods or services
    ///
    /// Document message used to provide credit information related to a transaction for goods or services to the relevant party.
    #[serde(rename = "81")]
    _81,
    /// Metered services invoice
    ///
    /// Document/message claiming payment for the supply of metered services (e.g., gas, electricity, etc.) supplied to a fixed meter whose consumption is measured over a period of time.
    #[serde(rename = "82")]
    _82,
    /// Credit note related to financial adjustments
    ///
    /// Document message for providing credit information related to financial adjustments to the relevant party, e.g., bonuses.
    #[serde(rename = "83")]
    _83,
    /// Debit note related to financial adjustments
    ///
    /// Document/message for providing debit information related to financial adjustments to the relevant party.
    #[serde(rename = "84")]
    _84,
    /// Customs manifest
    ///
    /// Message/document identifying a customs manifest. The document itemises a list of cargo prepared by shipping companies from bills of landing and presented to customs for formal report of cargo.
    #[serde(rename = "85")]
    _85,
    /// Vessel unpack report
    ///
    /// A document code to indicate that the message being transmitted identifies all short and surplus cargoes off-loaded from a vessel at a specified discharging port.
    #[serde(rename = "86")]
    _86,
    /// General cargo summary manifest report
    ///
    /// A document code to indicate that the message being transmitted is summary manifest information for general cargo.
    #[serde(rename = "87")]
    _87,
    /// Consignment unpack report
    ///
    /// A document code to indicate that the message being transmitted is a consignment unpack report only.
    #[serde(rename = "88")]
    _88,
    /// Meat and meat by-products sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that meat or meat by- products comply with the requirements set by the importing country.
    #[serde(rename = "89")]
    _89,
    /// Meat food products sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that meat food products comply with the requirements set by the importing country.
    #[serde(rename = "90")]
    _90,
    /// Poultry sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that poultry products comply with the requirements set by the importing country.
    #[serde(rename = "91")]
    _91,
    /// Horsemeat sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that horsemeat products comply with the requirements set by the importing country.
    #[serde(rename = "92")]
    _92,
    /// Casing sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that casing products comply with the requirements set by the importing country.
    #[serde(rename = "93")]
    _93,
    /// Pharmaceutical sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that pharmaceutical products comply with the requirements set by the importing country.
    #[serde(rename = "94")]
    _94,
    /// Inedible sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that inedible products comply with the requirements set by the importing country.
    #[serde(rename = "95")]
    _95,
    /// Impending arrival
    ///
    /// Notification of impending arrival details for vessel.
    #[serde(rename = "96")]
    _96,
    /// Means of transport advice
    ///
    /// Message reporting the means of transport used to carry goods or cargo.
    #[serde(rename = "97")]
    _97,
    /// Arrival information
    ///
    /// Message reporting the arrival details of goods or cargo.
    #[serde(rename = "98")]
    _98,
    /// Cargo release notification
    ///
    /// Message/document sent by the cargo handler indicating that the cargo has moved from a Customs controlled premise.
    #[serde(rename = "99")]
    _99,
    /// Excise certificate
    ///
    /// Certificate asserting that the goods have been submitted to the excise authorities before departure from the exporting country or before delivery in case of import traffic.
    #[serde(rename = "100")]
    _100,
    /// Registration document
    ///
    /// An official document providing registration details.
    #[serde(rename = "101")]
    _101,
    /// Tax notification
    ///
    /// Used to specify that the message is a tax notification.
    #[serde(rename = "102")]
    _102,
    /// Transport equipment direct interchange report
    ///
    /// Report on the movement of containers or other items of transport equipment being exchanged, establishing relevant rental periods.
    #[serde(rename = "103")]
    _103,
    /// Transport equipment impending arrival advice
    ///
    /// Advice that containers or other items of transport equipment may be expected to be delivered to a certain location.
    #[serde(rename = "104")]
    _104,
    /// Purchase order
    ///
    /// Document/message issued within an enterprise to initiate the purchase of articles, materials or services required for the production or manufacture of goods to be offered for sale or otherwise supplied to customers.
    #[serde(rename = "105")]
    _105,
    /// Transport equipment damage report
    ///
    /// Report of damaged items of transport equipment that have been returned.
    #[serde(rename = "106")]
    _106,
    /// Transport equipment maintenance and repair work estimate advice
    ///
    /// Advice providing estimates of transport equipment maintenance and repair costs.
    #[serde(rename = "107")]
    _107,
    /// Transport equipment empty release instruction
    ///
    /// Instruction to release an item of empty transport equipment to a specified party or parties.
    #[serde(rename = "108")]
    _108,
    /// Transport movement gate in report
    ///
    /// Report on the inward movement of cargo, containers or other items of transport equipment which have been delivered to a facility by an inland carrier.
    #[serde(rename = "109")]
    _109,
    /// Manufacturing instructions
    ///
    /// Document/message issued within an enterprise to initiate the manufacture of goods to be offered for sale.
    #[serde(rename = "110")]
    _110,
    /// Transport movement gate out report
    ///
    /// Report on the outward movement of cargo, containers or other items of transport equipment (either full or empty) which have been picked up by an inland carrier.
    #[serde(rename = "111")]
    _111,
    /// Transport equipment unpacking instruction
    ///
    /// Instruction to unpack specified cargo from specified containers or other items of transport equipment.
    #[serde(rename = "112")]
    _112,
    /// Transport equipment unpacking report
    ///
    /// Report on the completion of unpacking specified containers or other items of transport equipment.
    #[serde(rename = "113")]
    _113,
    /// Transport equipment pick-up availability request
    ///
    /// Request for confirmation that an item of transport equipment will be available for collection.
    #[serde(rename = "114")]
    _114,
    /// Transport equipment pick-up availability confirmation
    ///
    /// Confirmation that an item of transport equipment is available for collection.
    #[serde(rename = "115")]
    _115,
    /// Transport equipment pick-up report
    ///
    /// Report that an item of transport equipment has been collected.
    #[serde(rename = "116")]
    _116,
    /// Transport equipment shift report
    ///
    /// Report on the movement of containers or other items of transport within a facility.
    #[serde(rename = "117")]
    _117,
    /// Transport discharge instruction
    ///
    /// Instruction to unload specified cargo, containers or transport equipment from a means of transport.
    #[serde(rename = "118")]
    _118,
    /// Transport discharge report
    ///
    /// Report on cargo, containers or transport equipment unloaded from a particular means of transport.
    #[serde(rename = "119")]
    _119,
    /// Stores requisition
    ///
    /// Document/message issued within an enterprise ordering the taking out of stock of goods.
    #[serde(rename = "120")]
    _120,
    /// Transport loading instruction
    ///
    /// Instruction to load cargo, containers or transport equipment onto a means of transport.
    #[serde(rename = "121")]
    _121,
    /// Transport loading report
    ///
    /// Report on completion of loading cargo, containers or other transport equipment onto a means of transport.
    #[serde(rename = "122")]
    _122,
    /// Transport equipment maintenance and repair work authorisation
    ///
    /// Authorisation to have transport equipment repaired or to have maintenance performed.
    #[serde(rename = "123")]
    _123,
    /// Transport departure report
    ///
    /// Report of the departure of a means of transport from a particular facility.
    #[serde(rename = "124")]
    _124,
    /// Transport empty equipment advice
    ///
    /// Advice that an item or items of empty transport equipment are available for return.
    #[serde(rename = "125")]
    _125,
    /// Transport equipment acceptance order
    ///
    /// Order to accept items of transport equipment which are to be delivered by an inland carrier (rail, road or barge) to a specified facility.
    #[serde(rename = "126")]
    _126,
    /// Transport equipment special service instruction
    ///
    /// Instruction to perform a specified service or services on an item or items of transport equipment.
    #[serde(rename = "127")]
    _127,
    /// Transport equipment stock report
    ///
    /// Report on the number of items of transport equipment stored at one or more locations.
    #[serde(rename = "128")]
    _128,
    /// Transport cargo release order
    ///
    /// Order to release cargo or items of transport equipment to a specified party.
    #[serde(rename = "129")]
    _129,
    /// Invoicing data sheet
    ///
    /// Document/message issued within an enterprise containing data about goods sold, to be used as the basis for the preparation of an invoice.
    #[serde(rename = "130")]
    _130,
    /// Transport equipment packing instruction
    ///
    /// Instruction to pack cargo into a container or other item of transport equipment.
    #[serde(rename = "131")]
    _131,
    /// Customs clearance notice
    ///
    /// Notification of customs clearance of cargo or items of transport equipment.
    #[serde(rename = "132")]
    _132,
    /// Customs documents expiration notice
    ///
    /// Notice specifying expiration of Customs documents relating to cargo or items of transport equipment.
    #[serde(rename = "133")]
    _133,
    /// Transport equipment on-hire request
    ///
    /// Request for transport equipment to be made available for hire.
    #[serde(rename = "134")]
    _134,
    /// Transport equipment on-hire order
    ///
    /// Order to release empty items of transport equipment for on-hire to a lessee, and authorising collection by or on behalf of a specified party.
    #[serde(rename = "135")]
    _135,
    /// Transport equipment off-hire request
    ///
    /// Request to terminate the lease on an item of transport equipment at a specified time.
    #[serde(rename = "136")]
    _136,
    /// Transport equipment survey order
    ///
    /// Order to perform a survey on specified items of transport equipment.
    #[serde(rename = "137")]
    _137,
    /// Transport equipment survey order response
    ///
    /// Response to an order to conduct a survey of transport equipment.
    #[serde(rename = "138")]
    _138,
    /// Transport equipment survey report
    ///
    /// Survey report of specified items of transport equipment.
    #[serde(rename = "139")]
    _139,
    /// Packing instructions
    ///
    /// Document/message within an enterprise giving instructions on how goods are to be packed.
    #[serde(rename = "140")]
    _140,
    /// Advising items to be booked to a financial account
    ///
    /// A document and/or message advising of items which have to be booked to a financial account.
    #[serde(rename = "141")]
    _141,
    /// Transport equipment maintenance and repair work estimate order
    ///
    /// Order to draw up an estimate of the costs of maintenance or repair of transport equipment.
    #[serde(rename = "142")]
    _142,
    /// Transport equipment maintenance and repair notice
    ///
    /// Report of transport equipment which has been repaired or has had maintenance performed.
    #[serde(rename = "143")]
    _143,
    /// Empty container disposition order
    ///
    /// Order to make available empty containers.
    #[serde(rename = "144")]
    _144,
    /// Cargo vessel discharge order
    ///
    /// Order that the containers or cargo specified are to be discharged from a vessel.
    #[serde(rename = "145")]
    _145,
    /// Cargo vessel loading order
    ///
    /// Order that specified cargo, containers or groups of containers are to be loaded in or on a vessel.
    #[serde(rename = "146")]
    _146,
    /// Multidrop order
    ///
    /// One purchase order that contains the orders of two or more vendors and the associated delivery points for each.
    #[serde(rename = "147")]
    _147,
    /// Bailment contract
    ///
    /// A document authorizing the bailing of goods.
    #[serde(rename = "148")]
    _148,
    /// Basic agreement
    ///
    /// A document indicating an agreement containing basic terms and conditions applicable to future contracts between two parties.
    #[serde(rename = "149")]
    _149,
    /// Internal transport order
    ///
    /// Document/message giving instructions about the transport of goods within an enterprise.
    #[serde(rename = "150")]
    _150,
    /// Grant
    ///
    /// A document indicating the granting of funds.
    #[serde(rename = "151")]
    _151,
    /// Indefinite delivery indefinite quantity contract
    ///
    /// A document indicating a contract calling for the indefinite deliveries of indefinite quantities of goods.
    #[serde(rename = "152")]
    _152,
    /// Indefinite delivery definite quantity contract
    ///
    /// A document indicating a contract calling for indefinite deliveries of definite quantities.
    #[serde(rename = "153")]
    _153,
    /// Requirements contract
    ///
    /// A document indicating a requirements contract that authorizes the filling of all purchase requirements during a specified contract period.
    #[serde(rename = "154")]
    _154,
    /// Task order
    ///
    /// A document indicating an order that tasks a contractor to perform a specified function.
    #[serde(rename = "155")]
    _155,
    /// Make or buy plan
    ///
    /// A document indicating a plan that identifies which items will be made and which items will be bought.
    #[serde(rename = "156")]
    _156,
    /// Subcontractor plan
    ///
    /// A document indicating a plan that identifies the manufacturer's subcontracting strategy for a specific contract.
    #[serde(rename = "157")]
    _157,
    /// Cost data summary
    ///
    /// A document indicating a summary of cost data.
    #[serde(rename = "158")]
    _158,
    /// Certified cost and price data
    ///
    /// A document indicating cost and price data whose accuracy has been certified.
    #[serde(rename = "159")]
    _159,
    /// Wage determination
    ///
    /// A document indicating a determination of the wages to be paid.
    #[serde(rename = "160")]
    _160,
    /// Contract Funds Status Report (CFSR)
    ///
    /// A report to provide the status of funds applicable to the contract.
    #[serde(rename = "161")]
    _161,
    /// Certified inspection and test results
    ///
    /// A certification as to the accuracy of inspection and test results.
    #[serde(rename = "162")]
    _162,
    /// Material inspection and receiving report
    ///
    /// A report that is both an inspection report for materials and a receiving document.
    #[serde(rename = "163")]
    _163,
    /// Purchasing specification
    ///
    /// A document indicating a specification used to purchase an item.
    #[serde(rename = "164")]
    _164,
    /// Payment or performance bond
    ///
    /// A document indicating a bond that guarantees the payment of monies or a performance.
    #[serde(rename = "165")]
    _165,
    /// Contract security classification specification
    ///
    /// A document that indicates the specification contains the security and classification requirements for a contract.
    #[serde(rename = "166")]
    _166,
    /// Manufacturing specification
    ///
    /// A document indicating the specification of how an item is to be manufactured.
    #[serde(rename = "167")]
    _167,
    /// Buy America certificate of compliance
    ///
    /// A document certifying that more than 50 percent of the cost of an item is attributed to US origin.
    #[serde(rename = "168")]
    _168,
    /// Container off-hire notice
    ///
    /// Notice to return leased containers.
    #[serde(rename = "169")]
    _169,
    /// Cargo acceptance order
    ///
    /// Order to accept cargo to be delivered by a carrier.
    #[serde(rename = "170")]
    _170,
    /// Pick-up notice
    ///
    /// Notice specifying the pick-up of released cargo or containers from a certain address.
    #[serde(rename = "171")]
    _171,
    /// Authorisation to plan and suggest orders
    ///
    /// Document or message that authorises receiver to plan orders, based on information in this message, and send these orders as suggestions to the sender.
    #[serde(rename = "172")]
    _172,
    /// Authorisation to plan and ship orders
    ///
    /// Document or message that authorises receiver to plan and ship orders based on information in this message.
    #[serde(rename = "173")]
    _173,
    /// Drawing
    ///
    /// The document or message is a drawing.
    #[serde(rename = "174")]
    _174,
    /// Cost Performance Report (CPR) format 2
    ///
    /// A report identifying the cost performance on a contract at specified levels of the work breakdown structure (format 2 - organizational categories).
    #[serde(rename = "175")]
    _175,
    /// Cost Schedule Status Report (CSSR)
    ///
    /// A report providing the status of the cost and schedule applicable to a contract.
    #[serde(rename = "176")]
    _176,
    /// Cost Performance Report (CPR) format 1
    ///
    /// A report identifying the cost performance on a contract including the current month's values at specified levels of the work breakdown structure (format 1 - work breakdown structure).
    #[serde(rename = "177")]
    _177,
    /// Cost Performance Report (CPR) format 3
    ///
    /// A report identifying the cost performance on a contract that summarizes changes to a contract over a given reporting period with beginning and ending values (format 3 - baseline).
    #[serde(rename = "178")]
    _178,
    /// Cost Performance Report (CPR) format 4
    ///
    /// A report identifying the cost performance on a contract including forecasts of labour requirements for the remaining portion of the contract (format 4 - staffing).
    #[serde(rename = "179")]
    _179,
    /// Cost Performance Report (CPR) format 5
    ///
    /// A report identifying the cost performance on a contract that summarizes cost or schedule variances (format 5 - explanations and problem analysis).
    #[serde(rename = "180")]
    _180,
    /// Progressive discharge report
    ///
    /// Document or message progressively issued by the container terminal operator in charge of discharging a vessel identifying containers that have been discharged from a specific vessel at that point in time.
    #[serde(rename = "181")]
    _181,
    /// Balance confirmation
    ///
    /// Confirmation of a balance at an entry date.
    #[serde(rename = "182")]
    _182,
    /// Container stripping order
    ///
    /// Order to unload goods from a container.
    #[serde(rename = "183")]
    _183,
    /// Container stuffing order
    ///
    /// Order to stuff specified goods or consignments in a container.
    #[serde(rename = "184")]
    _184,
    /// Conveyance declaration (arrival)
    ///
    /// Declaration to the public authority upon arrival of the conveyance.
    #[serde(rename = "185")]
    _185,
    /// Conveyance declaration (departure)
    ///
    /// Declaration to the public authority upon departure of the conveyance.
    #[serde(rename = "186")]
    _186,
    /// Conveyance declaration (combined)
    ///
    /// Combined declaration of arrival and departure to the public authority.
    #[serde(rename = "187")]
    _187,
    /// Project recovery plan
    ///
    /// A project plan for recovery after a delay or problem resolution.
    #[serde(rename = "188")]
    _188,
    /// Project production plan
    ///
    /// A project plan for the production of goods.
    #[serde(rename = "189")]
    _189,
    /// Statistical and other administrative internal documents
    ///
    /// Documents/messages issued within an enterprise for the for the purpose of collection of production and other internal statistics, and for other administration purposes.
    #[serde(rename = "190")]
    _190,
    /// Project master schedule
    ///
    /// A high level, all encompassing master schedule of activities to complete a project.
    #[serde(rename = "191")]
    _191,
    /// Priced alternate tender bill of quantity
    ///
    /// A priced tender based upon an alternate specification.
    #[serde(rename = "192")]
    _192,
    /// Estimated priced bill of quantity
    ///
    /// An estimate based upon a detailed, quantity based specification (bill of quantity).
    #[serde(rename = "193")]
    _193,
    /// Draft bill of quantity
    ///
    /// Document/message providing a draft bill of quantity, issued in an unpriced form.
    #[serde(rename = "194")]
    _194,
    /// Documentary credit collection instruction
    ///
    /// Instruction for the collection of the documentary credit.
    #[serde(rename = "195")]
    _195,
    /// Request for an amendment of a documentary credit
    ///
    /// Request for an amendment of a documentary credit.
    #[serde(rename = "196")]
    _196,
    /// Documentary credit amendment information
    ///
    /// Documentary credit amendment information.
    #[serde(rename = "197")]
    _197,
    /// Advice of an amendment of a documentary credit
    ///
    /// Advice of an amendment of a documentary credit.
    #[serde(rename = "198")]
    _198,
    /// Response to an amendment of a documentary credit
    ///
    /// Response to an amendment of a documentary credit.
    #[serde(rename = "199")]
    _199,
    /// Documentary credit issuance information
    ///
    /// Provides information on documentary credit issuance.
    #[serde(rename = "200")]
    _200,
    /// Direct payment valuation request
    ///
    /// Request to establish a direct payment valuation.
    #[serde(rename = "201")]
    _201,
    /// Direct payment valuation
    ///
    /// Document/message addressed, for instance, by a general contractor to the owner, in order that a direct payment be made to a subcontractor.
    #[serde(rename = "202")]
    _202,
    /// Provisional payment valuation
    ///
    /// Document/message establishing a provisional payment valuation.
    #[serde(rename = "203")]
    _203,
    /// Payment valuation
    ///
    /// Document/message establishing the financial elements of a situation of works.
    #[serde(rename = "204")]
    _204,
    /// Quantity valuation
    ///
    /// Document/message providing a confirmed assessment, by quantity, of the completed work for a construction contract.
    #[serde(rename = "205")]
    _205,
    /// Quantity valuation request
    ///
    /// Document/message providing an initial assessment, by quantity, of the completed work for a construction contract.
    #[serde(rename = "206")]
    _206,
    /// Contract bill of quantities - BOQ
    ///
    /// Document/message providing a formal specification identifying quantities and prices that are the basis of a contract for a construction project. BOQ means: Bill of quantity.
    #[serde(rename = "207")]
    _207,
    /// Unpriced bill of quantity
    ///
    /// Document/message providing a detailed, quantity based specification, issued in an unpriced form to invite tender prices.
    #[serde(rename = "208")]
    _208,
    /// Priced tender BOQ
    ///
    /// Document/message providing a detailed, quantity based specification, updated with prices to form a tender submission for a construction contract. BOQ means: Bill of quantity.
    #[serde(rename = "209")]
    _209,
    /// Enquiry
    ///
    /// Document/message issued by a party interested in the purchase of goods specified therein and indicating particular, desirable conditions regarding delivery terms, etc., addressed to a prospective supplier with a view to obtaining an offer.
    #[serde(rename = "210")]
    _210,
    /// Interim application for payment
    ///
    /// Document/message containing a provisional assessment in support of a request for payment for completed work for a construction contract.
    #[serde(rename = "211")]
    _211,
    /// Agreement to pay
    ///
    /// Document/message in which the debtor expresses the intention to pay.
    #[serde(rename = "212")]
    _212,
    /// Request for financial cancellation
    ///
    /// The message is a request for financial cancellation.
    #[serde(rename = "213")]
    _213,
    /// Pre-authorised direct debit(s)
    ///
    /// The message contains pre-authorised direct debit(s).
    #[serde(rename = "214")]
    _214,
    /// Letter of intent
    ///
    /// Document/message by means of which a buyer informs a seller that the buyer intends to enter into contractual negotiations.
    #[serde(rename = "215")]
    _215,
    /// Approved unpriced bill of quantity
    ///
    /// Document/message providing an approved detailed, quantity based specification (bill of quantity), in an unpriced form.
    #[serde(rename = "216")]
    _216,
    /// Payment valuation for unscheduled items
    ///
    /// A payment valuation for unscheduled items.
    #[serde(rename = "217")]
    _217,
    /// Final payment request based on completion of work
    ///
    /// The final payment request of a series of payment requests submitted upon completion of all the work.
    #[serde(rename = "218")]
    _218,
    /// Payment request for completed units
    ///
    /// A request for payment for completed units.
    #[serde(rename = "219")]
    _219,
    /// Order
    ///
    /// Document/message by means of which a buyer initiates a transaction with a seller involving the supply of goods or services as specified, according to conditions set out in an offer, or otherwise known to the buyer.
    #[serde(rename = "220")]
    _220,
    /// Blanket order
    ///
    /// Usage of document/message for general order purposes with later split into quantities and delivery dates and maybe delivery locations.
    #[serde(rename = "221")]
    _221,
    /// Spot order
    ///
    /// Document/message ordering the remainder of a production's batch.
    #[serde(rename = "222")]
    _222,
    /// Lease order
    ///
    /// Document/message for goods in leasing contracts.
    #[serde(rename = "223")]
    _223,
    /// Rush order
    ///
    /// Document/message for urgent ordering.
    #[serde(rename = "224")]
    _224,
    /// Repair order
    ///
    /// Document/message to order repair of goods.
    #[serde(rename = "225")]
    _225,
    /// Call off order
    ///
    /// Document/message to provide split quantities and delivery dates referring to a previous blanket order.
    #[serde(rename = "226")]
    _226,
    /// Consignment order
    ///
    /// Order to deliver goods into stock with agreement on payment when goods are sold out of this stock.
    #[serde(rename = "227")]
    _227,
    /// Sample order
    ///
    /// Document/message to order samples.
    #[serde(rename = "228")]
    _228,
    /// Swap order
    ///
    /// Document/message informing buyer or seller of the replacement of goods previously ordered.
    #[serde(rename = "229")]
    _229,
    /// Purchase order change request
    ///
    /// Change to an purchase order already sent.
    #[serde(rename = "230")]
    _230,
    /// Purchase order response
    ///
    /// Response to an purchase order already received.
    #[serde(rename = "231")]
    _231,
    /// Hire order
    ///
    /// Document/message for hiring human resources or renting goods or equipment.
    #[serde(rename = "232")]
    _232,
    /// Spare parts order
    ///
    /// Document/message to order spare parts.
    #[serde(rename = "233")]
    _233,
    /// Campaign price/sales catalogue
    ///
    /// A price/sales catalogue containing special prices which are valid only for a specified period or under specified conditions.
    #[serde(rename = "234")]
    _234,
    /// Container list
    ///
    /// Document or message issued by party identifying the containers for which they are responsible.
    #[serde(rename = "235")]
    _235,
    /// Delivery forecast
    ///
    /// A message which enables the transmission of delivery or product forecasting requirements.
    #[serde(rename = "236")]
    _236,
    /// Cross docking services order
    ///
    /// A document or message to order cross docking services.
    #[serde(rename = "237")]
    _237,
    /// Non-pre-authorised direct debit(s)
    ///
    /// The message contains non-pre-authorised direct debit(s).
    #[serde(rename = "238")]
    _238,
    /// Rejected direct debit(s)
    ///
    /// The message contains rejected direct debit(s).
    #[serde(rename = "239")]
    _239,
    /// Delivery instructions
    ///
    /// Document/message issued by a buyer giving instructions regarding the details of the delivery of goods ordered.
    #[serde(rename = "240")]
    _240,
    /// Delivery schedule
    ///
    /// Usage of DELFOR-message.
    #[serde(rename = "241")]
    _241,
    /// Delivery just-in-time
    ///
    /// Usage of DELJIT-message.
    #[serde(rename = "242")]
    _242,
    /// Pre-authorised direct debit request(s)
    ///
    /// The message contains pre-authorised direct debit request(s).
    #[serde(rename = "243")]
    _243,
    /// Non-pre-authorised direct debit request(s)
    ///
    /// The message contains non-pre-authorised direct debit request(s).
    #[serde(rename = "244")]
    _244,
    /// Delivery release
    ///
    /// Document/message issued by a buyer releasing the despatch of goods after receipt of the Ready for despatch advice from the seller.
    #[serde(rename = "245")]
    _245,
    /// Settlement of a letter of credit
    ///
    /// Settlement of a letter of credit.
    #[serde(rename = "246")]
    _246,
    /// Bank to bank funds transfer
    ///
    /// The message is a bank to bank funds transfer.
    #[serde(rename = "247")]
    _247,
    /// Customer payment order(s)
    ///
    /// The message contains customer payment order(s).
    #[serde(rename = "248")]
    _248,
    /// Low value payment order(s)
    ///
    /// The message contains low value payment order(s) only.
    #[serde(rename = "249")]
    _249,
    /// Crew list declaration
    ///
    /// Declaration regarding crew members aboard the conveyance.
    #[serde(rename = "250")]
    _250,
    /// Inquiry
    ///
    /// This is a request for information.
    #[serde(rename = "251")]
    _251,
    /// Response to previous banking status message
    ///
    /// A response to a previously sent banking status message.
    #[serde(rename = "252")]
    _252,
    /// Project master plan
    ///
    /// A high level, all encompassing master plan to complete a project.
    #[serde(rename = "253")]
    _253,
    /// Project plan
    ///
    /// A plan for project work to be completed.
    #[serde(rename = "254")]
    _254,
    /// Project schedule
    ///
    /// A schedule of project activities to be completed.
    #[serde(rename = "255")]
    _255,
    /// Project planning available resources
    ///
    /// Available resources for project planning purposes.
    #[serde(rename = "256")]
    _256,
    /// Project planning calendar
    ///
    /// Work calendar information for project planning purposes.
    #[serde(rename = "257")]
    _257,
    /// Standing order
    ///
    /// An order to supply fixed quantities of products at fixed regular intervals.
    #[serde(rename = "258")]
    _258,
    /// Cargo movement event log
    ///
    /// A document detailing times and dates of events pertaining to a cargo movement.
    #[serde(rename = "259")]
    _259,
    /// Cargo analysis voyage report
    ///
    /// An analysis of the cargo for a voyage.
    #[serde(rename = "260")]
    _260,
    /// Self billed credit note
    ///
    /// A document which indicates that the customer is claiming credit in a self billing environment.
    #[serde(rename = "261")]
    _261,
    /// Consolidated credit note - goods and services
    ///
    /// Credit note for goods and services that covers multiple transactions involving more than one invoice.
    #[serde(rename = "262")]
    _262,
    /// Inventory adjustment status report
    ///
    /// A message detailing statuses related to the adjustment of inventory.
    #[serde(rename = "263")]
    _263,
    /// Transport equipment movement instruction
    ///
    /// Instruction to perform one or more different movements of transport equipment.
    #[serde(rename = "264")]
    _264,
    /// Transport equipment movement report
    ///
    /// Report on one or more different movements of transport equipment.
    #[serde(rename = "265")]
    _265,
    /// Transport equipment status change report
    ///
    /// Report on one or more changes of status associated with an item or items of transport equipment.
    #[serde(rename = "266")]
    _266,
    /// Fumigation certificate
    ///
    /// Certificate attesting that fumigation has been performed.
    #[serde(rename = "267")]
    _267,
    /// Wine certificate
    ///
    /// Certificate attesting to the quality, origin or appellation of wine.
    #[serde(rename = "268")]
    _268,
    /// Wool health certificate
    ///
    /// Certificate attesting that wool is free from specified risks to human or animal health.
    #[serde(rename = "269")]
    _269,
    /// Delivery note
    ///
    /// Paper document attached to a consignment informing the receiving party about contents of this consignment.
    #[serde(rename = "270")]
    _270,
    /// Packing list
    ///
    /// Document/message specifying the distribution of goods in individual packages (in trade environment the despatch advice message is used for the packing list).
    #[serde(rename = "271")]
    _271,
    /// New code request
    ///
    /// Requesting a new code.
    #[serde(rename = "272")]
    _272,
    /// Code change request
    ///
    /// Request a change to an existing code.
    #[serde(rename = "273")]
    _273,
    /// Simple data element request
    ///
    /// Requesting a new simple data element.
    #[serde(rename = "274")]
    _274,
    /// Simple data element change request
    ///
    /// Request a change to an existing simple data element.
    #[serde(rename = "275")]
    _275,
    /// Composite data element request
    ///
    /// Requesting a new composite data element.
    #[serde(rename = "276")]
    _276,
    /// Composite data element change request
    ///
    /// Request a change to an existing composite data element.
    #[serde(rename = "277")]
    _277,
    /// Segment request
    ///
    /// Request a new segment.
    #[serde(rename = "278")]
    _278,
    /// Segment change request
    ///
    /// Requesting a change to an existing segment.
    #[serde(rename = "279")]
    _279,
    /// New message request
    ///
    /// Request for a new message (NMR).
    #[serde(rename = "280")]
    _280,
    /// Message in development request
    ///
    /// Requesting a Message in Development (MiD).
    #[serde(rename = "281")]
    _281,
    /// Modification of existing message
    ///
    /// Requesting a change to an existing message.
    #[serde(rename = "282")]
    _282,
    /// Tracking number assignment report
    ///
    /// Report of assigned tracking numbers.
    #[serde(rename = "283")]
    _283,
    /// User directory definition
    ///
    /// Document/message defining the contents of a user directory set or parts thereof.
    #[serde(rename = "284")]
    _284,
    /// United Nations standard message request
    ///
    /// Requesting a United Nations Standard Message (UNSM).
    #[serde(rename = "285")]
    _285,
    /// Service directory definition
    ///
    /// Document/message defining the contents of a service directory set or parts thereof.
    #[serde(rename = "286")]
    _286,
    /// Status report
    ///
    /// Message covers information about the status.
    #[serde(rename = "287")]
    _287,
    /// Kanban schedule
    ///
    /// Message to describe a Kanban schedule.
    #[serde(rename = "288")]
    _288,
    /// Product data message
    ///
    /// A message to submit master data, a set of data that is rarely changed, to identify and describe products a supplier offers to their (potential) customer or buyer.
    #[serde(rename = "289")]
    _289,
    /// A claim for parts and/or labour charges
    ///
    /// A claim for parts and/or labour charges incurred .
    #[serde(rename = "290")]
    _290,
    /// Delivery schedule response
    ///
    /// A message providing a response to a previously transmitted delivery schedule.
    #[serde(rename = "291")]
    _291,
    /// Inspection request
    ///
    /// A message requesting a party to inspect items.
    #[serde(rename = "292")]
    _292,
    /// Inspection report
    ///
    /// A message informing a party of the results of an inspection.
    #[serde(rename = "293")]
    _293,
    /// Application acknowledgement and error report
    ///
    /// A message used by an application to acknowledge reception of a message and/or to report any errors.
    #[serde(rename = "294")]
    _294,
    /// Price variation invoice
    ///
    /// An invoice which requests payment for the difference in price between an original invoice and the result of the application of a price variation formula.
    #[serde(rename = "295")]
    _295,
    /// Credit note for price variation
    ///
    /// A credit note which is issued against a price variation invoice.
    #[serde(rename = "296")]
    _296,
    /// Instruction to collect
    ///
    /// A message instructing a party to collect goods.
    #[serde(rename = "297")]
    _297,
    /// Dangerous goods list
    ///
    /// Listing of all details of dangerous goods carried.
    #[serde(rename = "298")]
    _298,
    /// Registration renewal
    ///
    /// Code specifying the continued validity of previously submitted registration information.
    #[serde(rename = "299")]
    _299,
    /// Registration change
    ///
    /// Code specifying the modification of previously submitted registration information.
    #[serde(rename = "300")]
    _300,
    /// Response to registration
    ///
    /// Code specifying a response to an occurrence of a registration message.
    #[serde(rename = "301")]
    _301,
    /// Implementation guideline
    ///
    /// A document specifying the criterion and format for exchanging information in an electronic data interchange syntax.
    #[serde(rename = "302")]
    _302,
    /// Request for transfer
    ///
    /// Document/message is a request for transfer.
    #[serde(rename = "303")]
    _303,
    /// Cost performance report
    ///
    /// A report to convey cost performance data for a project or contract.
    #[serde(rename = "304")]
    _304,
    /// Application error and acknowledgement
    ///
    /// A message to inform a message issuer that a previously sent message has been received by the addressee's application, or that a previously sent message has been rejected by the addressee's application.
    #[serde(rename = "305")]
    _305,
    /// Cash pool financial statement
    ///
    /// A financial statement for a cash pool.
    #[serde(rename = "306")]
    _306,
    /// Sequenced delivery schedule
    ///
    /// Message to describe a sequence of product delivery.
    #[serde(rename = "307")]
    _307,
    /// Delcredere credit note
    ///
    /// A credit note sent to the party paying on behalf of a number of buyers.
    #[serde(rename = "308")]
    _308,
    /// Healthcare discharge report, final
    ///
    /// Final discharge report by healthcare provider.
    #[serde(rename = "309")]
    _309,
    /// Offer/quotation
    ///
    /// Document/message which , with a view to concluding a contract, sets out the conditions under which the goods are offered.
    #[serde(rename = "310")]
    _310,
    /// Request for quote
    ///
    /// Document/message requesting a quote on specified goods or services.
    #[serde(rename = "311")]
    _311,
    /// Acknowledgement message
    ///
    /// Message providing acknowledgement information at the business application level concerning the processing of a message.
    #[serde(rename = "312")]
    _312,
    /// Application error message
    ///
    /// Message indicating that a message was rejected due to errors encountered at the application level.
    #[serde(rename = "313")]
    _313,
    /// Cargo movement voyage summary
    ///
    /// A consolidated voyage summary which contains the information in a certificate of analysis, a voyage analysis and a cargo movement time log for a voyage.
    #[serde(rename = "314")]
    _314,
    /// Contract
    ///
    /// Document/message evidencing an agreement between the seller and the buyer for the supply of goods or services; its effects are equivalent to those of an order followed by an acknowledgement of order.
    #[serde(rename = "315")]
    _315,
    /// Application for usage of berth or mooring facilities
    ///
    /// Document to apply for usage of berth or mooring facilities.
    #[serde(rename = "316")]
    _316,
    /// Application for designation of berthing places
    ///
    /// Document to apply for designation of berthing places.
    #[serde(rename = "317")]
    _317,
    /// Application for shifting from the designated place in port
    ///
    /// Document to apply for shifting from the designated place in port.
    #[serde(rename = "318")]
    _318,
    /// Supplementary document for application for cargo operation of dangerous goods
    ///
    /// Supplementary document to apply for cargo operation of dangerous goods.
    #[serde(rename = "319")]
    _319,
    /// Acknowledgement of order
    ///
    /// Document/message acknowledging an undertaking to fulfil an order and confirming conditions or acceptance of conditions.
    #[serde(rename = "320")]
    _320,
    /// Supplementary document for application for transport of dangerous goods
    ///
    /// Supplementary document to apply for transport of dangerous goods.
    #[serde(rename = "321")]
    _321,
    /// Optical Character Reading (OCR) payment
    ///
    /// Payment effected by an Optical Character Reading (OCR) document.
    #[serde(rename = "322")]
    _322,
    /// Preliminary sales report
    ///
    /// Preliminary sales report sent before all the information is available.
    #[serde(rename = "323")]
    _323,
    /// Transport emergency card
    ///
    /// Official document specifying, for a given dangerous goods item, information such as nature of hazard, protective devices, actions to be taken in case of accident, spillage or fire and first aid to be given.
    #[serde(rename = "324")]
    _324,
    /// Proforma invoice
    ///
    /// Document/message serving as a preliminary invoice, containing - on the whole - the same information as the final invoice, but not actually claiming payment.
    #[serde(rename = "325")]
    _325,
    /// Partial invoice
    ///
    ///
    #[serde(rename = "326")]
    _326,
    /// Operating instructions
    ///
    ///
    #[serde(rename = "327")]
    _327,
    /// Name/product plate
    ///
    /// Plates on goods identifying and describing an article.
    #[serde(rename = "328")]
    _328,
    /// Co-insurance ceding bordereau
    ///
    /// The document or message contains a bordereau describing co-insurance ceding information.
    #[serde(rename = "329")]
    _329,
    /// Request for delivery instructions
    ///
    /// Document/message issued by a supplier requesting instructions from the buyer regarding the details of the delivery of goods ordered.
    #[serde(rename = "330")]
    _330,
    /// Commercial invoice which includes a packing list
    ///
    /// Commercial transaction (invoice) will include a packing list.
    #[serde(rename = "331")]
    _331,
    /// Trade data
    ///
    /// Document/message is for trade data.
    #[serde(rename = "332")]
    _332,
    /// Customs declaration for cargo examination
    ///
    /// Declaration provided to customs for cargo examination.
    #[serde(rename = "333")]
    _333,
    /// Customs declaration for cargo examination, alternate
    ///
    /// Alternate declaration provided to customs for cargo examination.
    #[serde(rename = "334")]
    _334,
    /// Booking request
    ///
    /// Document/message issued by a supplier to a carrier requesting space to be reserved for a specified consignment, indicating desirable conveyance, despatch time, etc.
    #[serde(rename = "335")]
    _335,
    /// Customs crew and conveyance
    ///
    /// Document/message contains information regarding the crew list and conveyance.
    #[serde(rename = "336")]
    _336,
    /// Customs summary declaration with commercial detail, alternate
    ///
    /// Alternate Customs declaration summary with commercial transaction details.
    #[serde(rename = "337")]
    _337,
    /// Items booked to a financial account report
    ///
    /// A message reporting items which have been booked to a financial account.
    #[serde(rename = "338")]
    _338,
    /// Report of transactions which need further information from the receiver
    ///
    /// A message reporting transactions which need further information from the receiver.
    #[serde(rename = "339")]
    _339,
    /// Shipping instructions
    ///
    /// Document/message advising details of cargo and exporter's requirements for its physical movement.
    #[serde(rename = "340")]
    _340,
    /// Shipper's letter of instructions (air)
    ///
    /// Document/message issued by a consignor in which he gives details of a consignment of goods that enables an airline or its agent to prepare an air waybill.
    #[serde(rename = "341")]
    _341,
    /// Report of transactions for information only
    ///
    /// A message reporting transactions for information only.
    #[serde(rename = "342")]
    _342,
    /// Cartage order (local transport)
    ///
    /// Document/message giving instructions regarding local transport of goods, e.g. from the premises of an enterprise to those of a carrier undertaking further transport.
    #[serde(rename = "343")]
    _343,
    /// EDI associated object administration message
    ///
    /// A message giving additional information about the exchange of an EDI associated object.
    #[serde(rename = "344")]
    _344,
    /// Ready for despatch advice
    ///
    /// Document/message issued by a supplier informing a buyer that goods ordered are ready for despatch.
    #[serde(rename = "345")]
    _345,
    /// Summary sales report
    ///
    /// Sales report containing summaries for several earlier sent sales reports.
    #[serde(rename = "346")]
    _346,
    /// Order status enquiry
    ///
    /// A message enquiring the status of previously sent orders.
    #[serde(rename = "347")]
    _347,
    /// Order status report
    ///
    /// A message reporting the status of previously sent orders.
    #[serde(rename = "348")]
    _348,
    /// Declaration regarding the inward and outward movement of vessel
    ///
    /// Document to declare inward and outward movement of a vessel.
    #[serde(rename = "349")]
    _349,
    /// Despatch order
    ///
    /// Document/message issued by a supplier initiating the despatch of goods to a buyer (consignee).
    #[serde(rename = "350")]
    _350,
    /// Despatch advice
    ///
    /// Document/message by means of which the seller or consignor informs the consignee about the despatch of goods.
    #[serde(rename = "351")]
    _351,
    /// Notification of usage of berth or mooring facilities
    ///
    /// Document to notify usage of berth or mooring facilities.
    #[serde(rename = "352")]
    _352,
    /// Application for vessel's entering into port area in night- time
    ///
    /// Document to apply for vessel's entering into port area in night-time.
    #[serde(rename = "353")]
    _353,
    /// Notification of emergency shifting from the designated place in port
    ///
    /// Document to notify shifting from designated place in port once secured at the designated place.
    #[serde(rename = "354")]
    _354,
    /// Customs summary declaration without commercial detail, alternate
    ///
    /// Alternate Customs declaration summary without any commercial transaction details.
    #[serde(rename = "355")]
    _355,
    /// Performance bond
    ///
    /// A document that guarantees performance.
    #[serde(rename = "356")]
    _356,
    /// Payment bond
    ///
    /// A document that guarantees the payment of monies.
    #[serde(rename = "357")]
    _357,
    /// Healthcare discharge report, preliminary
    ///
    /// Preliminary discharge report by healthcare provider.
    #[serde(rename = "358")]
    _358,
    /// Request for provision of a health service
    ///
    /// Document containing request for provision of a health service.
    #[serde(rename = "359")]
    _359,
    /// Advice of distribution of documents
    ///
    /// Document/message in which the party responsible for the issue of a set of trade documents specifies the various recipients of originals and copies of these documents, with an indication of the number of copies distributed to each of them.
    #[serde(rename = "370")]
    _370,
    /// Plan for provision of health service
    ///
    /// Document containing a plan for provision of health service.
    #[serde(rename = "371")]
    _371,
    /// Prescription
    ///
    /// Instructions for the dispensing and use of medicine or remedy.
    #[serde(rename = "372")]
    _372,
    /// Prescription request
    ///
    /// Request to issue a prescription for medicine or remedy.
    #[serde(rename = "373")]
    _373,
    /// Prescription dispensing report
    ///
    /// Document containing information of products dispensed according to a prescription.
    #[serde(rename = "374")]
    _374,
    /// Certificate of shipment
    ///
    /// Certificate providing confirmation that a consignment has been shipped.
    #[serde(rename = "375")]
    _375,
    /// Standing inquiry on product information
    ///
    /// A product inquiry which stands until it is cancelled.
    #[serde(rename = "376")]
    _376,
    /// Commercial invoice
    ///
    /// Document/message claiming payment for goods or services supplied under conditions agreed between seller and buyer.
    #[serde(rename = "380")]
    _380,
    /// Credit note
    ///
    /// Document/message for providing credit information to the relevant party.
    #[serde(rename = "381")]
    _381,
    /// Commission note
    ///
    /// Document/message in which a seller specifies the amount of commission, the percentage of the invoice amount, or some other basis for the calculation of the commission to which a sales agent is entitled.
    #[serde(rename = "382")]
    _382,
    /// Debit note
    ///
    /// Document/message for providing debit information to the relevant party.
    #[serde(rename = "383")]
    _383,
    /// Corrected invoice
    ///
    /// Commercial invoice that includes revised information differing from an earlier submission of the same invoice.
    #[serde(rename = "384")]
    _384,
    /// Consolidated invoice
    ///
    /// Commercial invoice that covers multiple transactions involving more than one vendor.
    #[serde(rename = "385")]
    _385,
    /// Prepayment invoice
    ///
    /// An invoice to pay amounts for goods and services in advance; these amounts will be subtracted from the final invoice.
    #[serde(rename = "386")]
    _386,
    /// Hire invoice
    ///
    /// Document/message for invoicing the hiring of human resources or renting goods or equipment.
    #[serde(rename = "387")]
    _387,
    /// Tax invoice
    ///
    /// An invoice for tax purposes.
    #[serde(rename = "388")]
    _388,
    /// Self-billed invoice
    ///
    /// An invoice the invoicee is producing instead of the seller.
    #[serde(rename = "389")]
    _389,
    /// Delcredere invoice
    ///
    /// An invoice sent to the party paying for a number of buyers.
    #[serde(rename = "390")]
    _390,
    /// Factored invoice
    ///
    /// Invoice assigned to a third party for collection.
    #[serde(rename = "393")]
    _393,
    /// Lease invoice
    ///
    /// Usage of INVOIC-message for goods in leasing contracts.
    #[serde(rename = "394")]
    _394,
    /// Consignment invoice
    ///
    /// Commercial invoice that covers a transaction other than one involving a sale.
    #[serde(rename = "395")]
    _395,
    /// Factored credit note
    ///
    /// Credit note related to assigned invoice(s).
    #[serde(rename = "396")]
    _396,
    /// Commercial account summary response
    ///
    /// A document providing a response to a previously sent commercial account summary message.
    #[serde(rename = "397")]
    _397,
    /// Cross docking despatch advice
    ///
    /// Document by means of which the supplier or consignor informs the buyer, consignee or the distribution centre about the despatch of goods for cross docking.
    #[serde(rename = "398")]
    _398,
    /// Transshipment despatch advice
    ///
    /// Document by means of which the supplier or consignor informs the buyer, consignee or the distribution centre about the despatch of goods for transshipment.
    #[serde(rename = "399")]
    _399,
    /// Exceptional order
    ///
    /// An order which falls outside the framework of an agreement.
    #[serde(rename = "400")]
    _400,
    /// Transshipment order
    ///
    /// An order requesting the supply of products packed according to the final delivery point which will be moved across a dock in a distribution centre without further handling.
    #[serde(rename = "401")]
    _401,
    /// Cross docking order
    ///
    /// An order requesting the supply of products which will be de-consolidated in the distribution centre and re- consolidated according to final delivery location.
    #[serde(rename = "402")]
    _402,
    /// Means of transportation availability information
    ///
    /// Information giving the various availabilities of a means of transportation.
    #[serde(rename = "403")]
    _403,
    /// Means of transportation schedule information
    ///
    /// Information giving the various schedules of a means of transportation.
    #[serde(rename = "404")]
    _404,
    /// Transport equipment delivery notice
    ///
    /// Notification regarding the delivery of transport equipment.
    #[serde(rename = "405")]
    _405,
    /// Instructions for bank transfer
    ///
    /// Document/message containing instructions from a customer to his bank to pay an amount in a specified currency to a nominated party in another country by a method either specified (e.g. teletransmission, air mail) or left to the discretion of the bank.
    #[serde(rename = "409")]
    _409,
    /// Application for banker's draft
    ///
    /// Application by a customer to his bank to issue a banker's draft stating the amount and currency of the draft, the name of the payee and the place and country of payment.
    #[serde(rename = "412")]
    _412,
    /// Collection payment advice
    ///
    /// Document/message whereby a bank advises that a collection has been paid, giving details and methods of funds disposal.
    #[serde(rename = "425")]
    _425,
    /// Documentary credit payment advice
    ///
    /// Document/message whereby a bank advises payment under a documentary credit.
    #[serde(rename = "426")]
    _426,
    /// Documentary credit acceptance advice
    ///
    /// Document/message whereby a bank advises acceptance under a documentary credit.
    #[serde(rename = "427")]
    _427,
    /// Documentary credit negotiation advice
    ///
    /// Document/message whereby a bank advises negotiation under a documentary credit.
    #[serde(rename = "428")]
    _428,
    /// Application for banker's guarantee
    ///
    /// Document/message whereby a customer requests his bank to issue a guarantee in favour of a nominated party in another country, stating the amount and currency and the specific conditions of the guarantee.
    #[serde(rename = "429")]
    _429,
    /// Banker's guarantee
    ///
    /// Document/message in which a bank undertakes to pay out a limited amount of money to a designated party, on conditions stated therein (other than those laid down in the Uniform Customs Practice).
    #[serde(rename = "430")]
    _430,
    /// Documentary credit letter of indemnity
    ///
    /// Document/message in which a beneficiary of a documentary credit accepts responsibility for non-compliance with the terms and conditions of the credit, and undertakes to refund the money received under the credit, with interest and charges accrued.
    #[serde(rename = "431")]
    _431,
    /// Preadvice of a credit
    ///
    /// Preadvice indicating a credit to happen in the future.
    #[serde(rename = "435")]
    _435,
    /// Collection order
    ///
    /// Document/message whereby a bank is instructed (or requested) to handle financial and/or commercial documents in order to obtain acceptance and/or payment, or to deliver documents on such other terms and conditions as may be specified.
    #[serde(rename = "447")]
    _447,
    /// Documents presentation form
    ///
    /// Document/message whereby a draft or similar instrument and/or commercial documents are presented to a bank for acceptance, discounting, negotiation, payment or collection, whether or not against a documentary credit.
    #[serde(rename = "448")]
    _448,
    /// Payment order
    ///
    /// Document/message containing information needed to initiate the payment. It may cover the financial settlement for one or more commercial trade transactions. A payment order is an instruction to the ordered bank to arrange for the payment of one specified amount to the beneficiary.
    #[serde(rename = "450")]
    _450,
    /// Extended payment order
    ///
    /// Document/message containing information needed to initiate the payment. It may cover the financial settlement for several commercial trade transactions, which it is possible to specify in a special payments detail part. It is an instruction to the ordered bank to arrange for the payment of one specified amount to the beneficiary.
    #[serde(rename = "451")]
    _451,
    /// Multiple payment order
    ///
    /// Document/message containing a payment order to debit one or more accounts and to credit one or more beneficiaries.
    #[serde(rename = "452")]
    _452,
    /// Credit advice
    ///
    /// Document/message sent by an account servicing institution to one of its account owners, to inform the account owner of an entry which has been or will be credited to its account for a specified amount on the date indicated.
    #[serde(rename = "454")]
    _454,
    /// Extended credit advice
    ///
    /// Document/message sent by an account servicing institution to one of its account owners, to inform the account owner of an entry that has been or will be credited to its account for a specified amount on the date indicated. It provides extended commercial information concerning the relevant remittance advice.
    #[serde(rename = "455")]
    _455,
    /// Debit advice
    ///
    /// Advice on a debit.
    #[serde(rename = "456")]
    _456,
    /// Reversal of debit
    ///
    /// Reversal of debit accounting entry by bank.
    #[serde(rename = "457")]
    _457,
    /// Reversal of credit
    ///
    /// Reversal of credit accounting entry by bank.
    #[serde(rename = "458")]
    _458,
    /// Documentary credit application
    ///
    /// Document/message whereby a bank is requested to issue a documentary credit on the conditions specified therein.
    #[serde(rename = "460")]
    _460,
    /// Documentary credit
    ///
    /// Document/message in which a bank states that it has issued a documentary credit under which the beneficiary is to obtain payment, acceptance or negotiation on compliance with certain terms and conditions and against presentation of stipulated documents and such drafts as may be specified. The credit may or may not be confirmed by another bank.
    #[serde(rename = "465")]
    _465,
    /// Documentary credit notification
    ///
    /// Document/message issued by an advising bank in order to transmit a documentary credit to a beneficiary, or to another advising bank.
    #[serde(rename = "466")]
    _466,
    /// Documentary credit transfer advice
    ///
    /// Document/message whereby a bank advises that (part of) a documentary credit is being or has been transferred in favour of a second beneficiary.
    #[serde(rename = "467")]
    _467,
    /// Documentary credit amendment notification
    ///
    /// Document/message whereby a bank advises that the terms and conditions of a documentary credit have been amended.
    #[serde(rename = "468")]
    _468,
    /// Documentary credit amendment
    ///
    /// Document/message whereby a bank notifies a beneficiary of the details of an amendment to the terms and conditions of a documentary credit.
    #[serde(rename = "469")]
    _469,
    /// Remittance advice
    ///
    /// Document/message advising of the remittance of payment.
    #[serde(rename = "481")]
    _481,
    /// Banker's draft
    ///
    /// Draft drawn in favour of a third party either by one bank on another bank, or by a branch of a bank on its head office (or vice versa) or upon another branch of the same bank. In either case, the draft should comply with the specifications laid down for cheques in the country in which it is to be payable.
    #[serde(rename = "485")]
    _485,
    /// Bill of exchange
    ///
    /// Document/message, issued and signed in conformity with the applicable legislation, which contains an unconditional order whereby the drawer directs the drawee to pay a definite sum of money to the payee or to his order, on demand or at a definite time, against the surrender of the document itself.
    #[serde(rename = "490")]
    _490,
    /// Promissory note
    ///
    /// Document/message, issued and signed in conformity with the applicable legislation, which contains an unconditional promise whereby the maker undertakes to pay a definite sum of money to the payee or to his order, on demand or at a definite time, against the surrender of the document itself.
    #[serde(rename = "491")]
    _491,
    /// Statement of account message
    ///
    /// Usage of STATAC-message.
    #[serde(rename = "493")]
    _493,
    /// Insurance certificate
    ///
    /// Document/message issued to the insured certifying that insurance has been effected and that a policy has been issued. Such a certificate for a particular cargo is primarily used when good are insured under the terms of a floating or an open policy; at the request of the insured it can be exchanged for a policy.
    #[serde(rename = "520")]
    _520,
    /// Insurance policy
    ///
    /// Document/message issued by the insurer evidencing an agreement to insure and containing the conditions of the agreement concluded whereby the insurer undertakes for a specific fee to indemnify the insured for the losses arising out of the perils and accidents specified in the contract.
    #[serde(rename = "530")]
    _530,
    /// Insurance declaration sheet (bordereau)
    ///
    /// A document/message used when an insured reports to his insurer details of individual shipments which are covered by an insurance contract - an open cover or a floating policy - between the parties.
    #[serde(rename = "550")]
    _550,
    /// Insurer's invoice
    ///
    /// Document/message issued by an insurer specifying the cost of an insurance which has been effected and claiming payment therefore.
    #[serde(rename = "575")]
    _575,
    /// Cover note
    ///
    /// Document/message issued by an insurer (insurance broker, agent, etc.) to notify the insured that his insurance have been carried out.
    #[serde(rename = "580")]
    _580,
    /// Forwarding instructions
    ///
    /// Document/message issued to a freight forwarder, giving instructions regarding the action to be taken by the forwarder for the forwarding of goods described therein.
    #[serde(rename = "610")]
    _610,
    /// Forwarder's advice to import agent
    ///
    /// Document/message issued by a freight forwarder in an exporting country advising his counterpart in an importing country about the forwarding of goods described therein.
    #[serde(rename = "621")]
    _621,
    /// Forwarder's advice to exporter
    ///
    /// Document/message issued by a freight forwarder informing an exporter of the action taken in fulfillment of instructions received.
    #[serde(rename = "622")]
    _622,
    /// Forwarder's invoice
    ///
    /// Invoice issued by a freight forwarder specifying services rendered and costs incurred and claiming payment therefore.
    #[serde(rename = "623")]
    _623,
    /// Forwarder's certificate of receipt
    ///
    /// Non-negotiable document issued by a forwarder to certify that he has assumed control of a specified consignment, with irrevocable instructions to send it to the consignee indicated in the document or to hold it at his disposal. E.g. FIATA-FCR.
    #[serde(rename = "624")]
    _624,
    /// Shipping note
    ///
    /// Document/message provided by the shipper or his agent to the carrier, multimodal transport operator, terminal or other receiving authority, giving information about export consignments offered for transport, and providing for the necessary receipts and declarations of liability. (Sometimes a multipurpose cargo handling document also fulfilling the functions of document 632, 633, 650 and 655).
    #[serde(rename = "630")]
    _630,
    /// Forwarder's warehouse receipt
    ///
    /// Document/message issued by a forwarder acting as Warehouse Keeper acknowledging receipt of goods placed in a warehouse, and stating or referring to the conditions which govern the warehousing and the release of goods. The document contains detailed provisions regarding the rights of holders-by-endorsement, transfer of ownership, etc. E.g. FIATA-FWR.
    #[serde(rename = "631")]
    _631,
    /// Goods receipt
    ///
    /// Document/message issued by a port, warehouse/shed, or terminal operator acknowledging receipt of goods specified therein on conditions stated or referred to in the document.
    #[serde(rename = "632")]
    _632,
    /// Port charges documents
    ///
    /// Documents/messages specifying services rendered, storage and handling costs, demurrage and other charges due to the owner of goods described therein.
    #[serde(rename = "633")]
    _633,
    /// Warehouse warrant
    ///
    /// Negotiable receipt document, issued by a Warehouse Keeper to a person placing goods in a warehouse and conferring title to the goods stored.
    #[serde(rename = "635")]
    _635,
    /// Delivery order
    ///
    /// Document/message issued by a party entitled to authorize the release of goods specified therein to a named consignee, to be retained by the custodian of the goods.
    #[serde(rename = "640")]
    _640,
    /// Handling order
    ///
    /// Document/message issued by a cargo handling organization (port administration, terminal operator, etc.) for the removal or other handling of goods under their care.
    #[serde(rename = "650")]
    _650,
    /// Gate pass
    ///
    /// Document/message authorizing goods specified therein to be brought out of a fenced-in port or terminal area.
    #[serde(rename = "655")]
    _655,
    /// Waybill
    ///
    /// Non-negotiable document evidencing the contract for the transport of cargo.
    #[serde(rename = "700")]
    _700,
    /// Universal (multipurpose) transport document
    ///
    /// Document/message evidencing a contract of carriage covering the movement of goods by any mode of transport, or combination of modes, for national as well as international transport, under any applicable international convention or national law and under the conditions of carriage of any carrier or transport operator undertaking or arranging the transport referred to in the document.
    #[serde(rename = "701")]
    _701,
    /// Goods receipt, carriage
    ///
    /// Document/message issued by a carrier or a carrier's agent, acknowledging receipt for carriage of goods specified therein on conditions stated or referred to in the document, enabling the carrier to issue a transport document.
    #[serde(rename = "702")]
    _702,
    /// House waybill
    ///
    /// The document made out by an agent/consolidator which evidences the contract between the shipper and the agent/consolidator for the arrangement of carriage of goods.
    #[serde(rename = "703")]
    _703,
    /// Master bill of lading
    ///
    /// A bill of lading issued by the master of a vessel (in actuality the owner or charterer of the vessel). It could cover a number of house bills.
    #[serde(rename = "704")]
    _704,
    /// Bill of lading
    ///
    /// Negotiable document/message which evidences a contract of carriage by sea and the taking over or loading of goods by carrier, and by which carrier undertakes to deliver goods against surrender of the document. A provision in the document that goods are to be delivered to the order of a named person, or to order, or to bearer, constitutes such an undertaking.
    #[serde(rename = "705")]
    _705,
    /// Bill of lading original
    ///
    /// The original of the bill of lading issued by a transport company. When issued by the maritime industry it could signify ownership of the cargo.
    #[serde(rename = "706")]
    _706,
    /// Bill of lading copy
    ///
    /// A copy of the bill of lading issued by a transport company.
    #[serde(rename = "707")]
    _707,
    /// Empty container bill
    ///
    /// Bill of lading indicating an empty container.
    #[serde(rename = "708")]
    _708,
    /// Tanker bill of lading
    ///
    /// Document which evidences a transport of liquid bulk cargo.
    #[serde(rename = "709")]
    _709,
    /// Sea waybill
    ///
    /// Non-negotiable document which evidences a contract for the carriage of goods by sea and the taking over of the goods by the carrier, and by which the carrier undertakes to deliver the goods to the consignee named in the document.
    #[serde(rename = "710")]
    _710,
    /// Inland waterway bill of lading
    ///
    /// Negotiable transport document made out to a named person, to order or to bearer, signed by the carrier and handed to the sender after receipt of the goods.
    #[serde(rename = "711")]
    _711,
    /// Non-negotiable maritime transport document (generic)
    ///
    /// Non-negotiable document which evidences a contract for the carriage of goods by sea and the taking over or loading of the goods by the carrier, and by which the carrier undertakes to deliver the goods to the consignee named in the document. E.g. Sea waybill. Remark: Synonymous with "straight" or "non-negotiable Bill of lading" used in certain countries, e.g. Canada.
    #[serde(rename = "712")]
    _712,
    /// Mate's receipt
    ///
    /// Document/message issued by a ship's officer to acknowledge that a specified consignment has been received on board a vessel, and the apparent condition of the goods; enabling the carrier to issue a Bill of lading.
    #[serde(rename = "713")]
    _713,
    /// House bill of lading
    ///
    /// The bill of lading issued not by the carrier but by the freight forwarder/consolidator known by the carrier.
    #[serde(rename = "714")]
    _714,
    /// Letter of indemnity for non-surrender of bill of lading
    ///
    /// Document/message issued by a commercial party or a bank of an insurance company accepting responsibility to the beneficiary of the indemnity in accordance with the terms thereof.
    #[serde(rename = "715")]
    _715,
    /// Forwarder's bill of lading
    ///
    /// Non-negotiable document issued by a freight forwarder evidencing a contract for the carriage of goods by sea and the taking over or loading of the goods by the freight forwarder, and by which the freight forwarder undertakes to deliver the goods to the consignee named in the document.
    #[serde(rename = "716")]
    _716,
    /// Rail consignment note (generic term)
    ///
    /// Transport document constituting a contract for the carriage of goods between the sender and the carrier (the railway). For international rail traffic, this document must conform to the model prescribed by the international conventions concerning carriage of goods by rail, e.g. CIM Convention, SMGS Convention.
    #[serde(rename = "720")]
    _720,
    /// Road list-SMGS
    ///
    /// Accounting document, one copy of which is drawn up for each consignment note; it accompanies the consignment over the whole route and is a rail transport document.
    #[serde(rename = "722")]
    _722,
    /// Escort official recognition
    ///
    /// Document/message which gives right to the owner to exert all functions normally transferred to a guard in a train by which an escorted consignment is transported.
    #[serde(rename = "723")]
    _723,
    /// Recharging document
    ///
    /// Fictitious transport document regarding a previous transport, enabling a carrier's agent to give to another carrier's agent (in a different country) the possibility to collect charges relating to the original transport (rail environment).
    #[serde(rename = "724")]
    _724,
    /// Road consignment note
    ///
    /// Transport document/message which evidences a contract between a carrier and a sender for the carriage of goods by road (generic term). Remark: For international road traffic, this document must contain at least the particulars prescribed by the convention on the contract for the international carriage of goods by road (CMR).
    #[serde(rename = "730")]
    _730,
    /// Air waybill
    ///
    /// Document/message made out by or on behalf of the shipper which evidences the contract between the shipper and carrier(s) for carriage of goods over routes of the carrier(s) and which is identified by the airline prefix issuing the document plus a serial (IATA).
    #[serde(rename = "740")]
    _740,
    /// Master air waybill
    ///
    /// Document/message made out by or on behalf of the agent/consolidator which evidences the contract between the agent/consolidator and carrier(s) for carriage of goods over routes of the carrier(s) for a consignment consisting of goods originated by more than one shipper (IATA).
    #[serde(rename = "741")]
    _741,
    /// Substitute air waybill
    ///
    /// A temporary air waybill which contains only limited information because of the absence of the original.
    #[serde(rename = "743")]
    _743,
    /// Crew's effects declaration
    ///
    /// Declaration to Customs regarding the personal effects of crew members aboard the conveyance; equivalent to IMO FAL 4.
    #[serde(rename = "744")]
    _744,
    /// Passenger list
    ///
    /// Declaration to Customs regarding passengers aboard the conveyance; equivalent to IMO FAL 6.
    #[serde(rename = "745")]
    _745,
    /// Delivery notice (rail transport)
    ///
    /// Document/message created by the consignor or by the departure station, joined to the transport or sent to the consignee, giving the possibility to the consignee or the arrival station to attest the delivery of the goods. The document must be returned to the consignor or to the departure station.
    #[serde(rename = "746")]
    _746,
    /// Despatch note (post parcels)
    ///
    /// Document/message which, according to Article 106 of the "Agreement concerning Postal Parcels" under the UPU convention, is to accompany post parcels.
    #[serde(rename = "750")]
    _750,
    /// Multimodal/combined transport document (generic)
    ///
    /// A transport document used when more than one mode of transportation is involved in the movement of cargo. It is a contract of carriage and receipt of the cargo for a multimodal transport. It indicates the place where the responsible transport company in the move takes responsibility for the cargo, the place where the responsibility of this transport company in the move ends and the conveyances involved.
    #[serde(rename = "760")]
    _760,
    /// Through bill of lading
    ///
    /// Bill of lading which evidences a contract of carriage from one place to another in separate stages of which at least one stage is a sea transit, and by which the issuing carrier accepts responsibility for the carriage as set forth in the through bill of lading.
    #[serde(rename = "761")]
    _761,
    /// Forwarder's certificate of transport
    ///
    /// Negotiable document/message issued by a forwarder to certify that he has taken charge of a specified consignment for despatch and delivery in accordance with the consignor's instructions, as indicated in the document, and that he accepts responsibility for delivery of the goods to the holder of the document through the intermediary of a delivery agent of his choice. E.g. FIATA-FCT.
    #[serde(rename = "763")]
    _763,
    /// Combined transport document (generic)
    ///
    /// Negotiable or non-negotiable document evidencing a contract for the performance and/or procurement of performance of combined transport of goods and bearing on its face either the heading "Negotiable combined transport document issued subject to Uniform Rules for a Combined Transport Document (ICC Brochure No. 298)" or the heading "Non-negotiable Combined Transport Document issued subject to Uniform Rules for a Combined Transport Document (ICC Brochure No. 298)".
    #[serde(rename = "764")]
    _764,
    /// Multimodal transport document (generic)
    ///
    /// Document/message which evidences a multimodal transport contract, the taking in charge of the goods by the multimodal transport operator, and an undertaking by him to deliver the goods in accordance with the terms of the contract. (International Convention on Multimodal Transport of Goods).
    #[serde(rename = "765")]
    _765,
    /// Combined transport bill of lading/multimodal bill of lading
    ///
    /// Document which evidences a multimodal transport contract, the taking in charge of the goods by the multimodal transport operator, and an undertaking by him to deliver the goods in accordance with the terms of the contract.
    #[serde(rename = "766")]
    _766,
    /// Booking confirmation
    ///
    /// Document/message issued by a carrier to confirm that space has been reserved for a consignment in means of transport.
    #[serde(rename = "770")]
    _770,
    /// Calling forward notice
    ///
    /// Instructions for release or delivery of goods.
    #[serde(rename = "775")]
    _775,
    /// Freight invoice
    ///
    /// Document/message issued by a transport operation specifying freight costs and charges incurred for a transport operation and stating conditions of payment.
    #[serde(rename = "780")]
    _780,
    /// Arrival notice (goods)
    ///
    /// Notification from the carrier to the consignee in writing, by telephone or by any other means (express letter, message, telegram, etc.) informing him that a consignment addressed to him is being or will shortly be held at his disposal at a specified point in the place of destination.
    #[serde(rename = "781")]
    _781,
    /// Notice of circumstances preventing delivery (goods)
    ///
    /// Request made by the carrier to the sender, or, as the case may be, the consignee, for instructions as to the disposal of the consignment when circumstances prevent delivery and the return of the goods has not been requested by the consignor in the transport document.
    #[serde(rename = "782")]
    _782,
    /// Notice of circumstances preventing transport (goods)
    ///
    /// Request made by the carrier to the sender, or, the consignee as the case may be, for instructions as to the disposal of the goods when circumstances prevent transport before departure or en route, after acceptance of the consignment concerned.
    #[serde(rename = "783")]
    _783,
    /// Delivery notice (goods)
    ///
    /// Notification in writing, sent by the carrier to the sender, to inform him at his request of the actual date of delivery of the goods.
    #[serde(rename = "784")]
    _784,
    /// Cargo manifest
    ///
    /// Listing of goods comprising the cargo carried in a means of transport or in a transport-unit. The cargo manifest gives the commercial particulars of the goods, such as transport document numbers, consignors, consignees, shipping marks, number and kind of packages and descriptions and quantities of the goods.
    #[serde(rename = "785")]
    _785,
    /// Freight manifest
    ///
    /// Document/message containing the same information as a cargo manifest, and additional details on freight amounts, charges, etc.
    #[serde(rename = "786")]
    _786,
    /// Bordereau
    ///
    /// Document/message used in road transport, listing the cargo carried on a road vehicle, often referring to appended copies of Road consignment note.
    #[serde(rename = "787")]
    _787,
    /// Container manifest (unit packing list)
    ///
    /// Document/message specifying the contents of particular freight containers or other transport units, prepared by the party responsible for their loading into the container or unit.
    #[serde(rename = "788")]
    _788,
    /// Charges note
    ///
    /// Document used by the rail organization to indicate freight charges or additional charges in each case where the departure station is not able to calculate the charges for the total voyage (e.g. tariff not yet updated, part of voyage not covered by the tariff). This document must be considered as joined to the transport.
    #[serde(rename = "789")]
    _789,
    /// Advice of collection
    ///
    /// Document that is joined to the transport or sent by separate means, giving to the departure rail organization the proof that the cash-on delivery amount has been encashed by the arrival rail organization before reimbursement of the consignor.
    #[serde(rename = "790")]
    _790,
    /// Safety of ship certificate
    ///
    /// Document certifying a ship's safety to a specified date.
    #[serde(rename = "791")]
    _791,
    /// Safety of radio certificate
    ///
    /// Document certifying the safety of a ship's radio facilities to a specified date.
    #[serde(rename = "792")]
    _792,
    /// Safety of equipment certificate
    ///
    /// Document certifying the safety of a ship's equipment to a specified date.
    #[serde(rename = "793")]
    _793,
    /// Civil liability for oil certificate
    ///
    /// Document declaring a ship owner's liability for oil propelling or carried on a vessel.
    #[serde(rename = "794")]
    _794,
    /// Loadline document
    ///
    /// Document specifying the limit of a ship's legal submersion under various conditions.
    #[serde(rename = "795")]
    _795,
    /// Derat document
    ///
    /// Document certifying that a ship is free of rats, valid to a specified date.
    #[serde(rename = "796")]
    _796,
    /// Maritime declaration of health
    ///
    /// Document certifying the health condition on board a vessel, valid to a specified date.
    #[serde(rename = "797")]
    _797,
    /// Certificate of registry
    ///
    /// Official certificate stating the vessel's registry.
    #[serde(rename = "798")]
    _798,
    /// Ship's stores declaration
    ///
    /// Declaration to Customs regarding the contents of the ship's stores (equivalent to IMO FAL 3) i.e. goods intended for consumption by passengers/crew on board vessels, aircraft or trains, whether or not sold or landed; goods necessary for operation/maintenance of conveyance, including fuel/lubricants, excluding spare parts/equipment (IMO).
    #[serde(rename = "799")]
    _799,
    /// Export licence, application for
    ///
    /// Application for a permit issued by a government authority permitting exportation of a specified commodity subject to specified conditions as quantity, country of destination, etc.
    #[serde(rename = "810")]
    _810,
    /// Export licence
    ///
    /// Permit issued by a government authority permitting exportation of a specified commodity subject to specified conditions as quantity, country of destination, etc. Synonym: Embargo permit.
    #[serde(rename = "811")]
    _811,
    /// Exchange control declaration, export
    ///
    /// Document/message completed by an exporter/seller as a means whereby the competent body may control that the amount of foreign exchange accrued from a trade transaction is repatriated in accordance with the conditions of payment and exchange control regulations in force.
    #[serde(rename = "812")]
    _812,
    /// Despatch note model T
    ///
    /// European community transit declaration.
    #[serde(rename = "820")]
    _820,
    /// Despatch note model T1
    ///
    /// Transit declaration for goods circulating under internal community transit procedures (between European Union (EU) countries).
    #[serde(rename = "821")]
    _821,
    /// Despatch note model T2
    ///
    /// Ascertainment that the declared goods were originally produced in an European Union (EU) country.
    #[serde(rename = "822")]
    _822,
    /// Control document T5
    ///
    /// Control document (export declaration) used particularly in case of re-sending without use with only VAT collection, refusal, unconformity with contract etc.
    #[serde(rename = "823")]
    _823,
    /// Re-sending consignment note
    ///
    /// Rail consignment note prepared by the consignor for the facilitation of an eventual return to the origin of the goods.
    #[serde(rename = "824")]
    _824,
    /// Despatch note model T2L
    ///
    /// Ascertainment that the declared goods were originally produced in an European Union (EU) country. May only be used for goods that are loaded on one single means of transport in one single departure point for one single delivery point.
    #[serde(rename = "825")]
    _825,
    /// Goods declaration for exportation
    ///
    /// Document/message by which goods are declared for export Customs clearance, conforming to the layout key set out at Appendix I to Annex C.1 concerning outright exportation to the Kyoto convention (CCC). Within a Customs union, "for despatch" may have the same meaning as "for exportation".
    #[serde(rename = "830")]
    _830,
    /// Cargo declaration (departure)
    ///
    /// Generic term, sometimes referred to as Freight declaration, applied to the documents providing the particulars required by the Customs concerning the cargo (freight) carried by commercial means of transport (CCC).
    #[serde(rename = "833")]
    _833,
    /// Application for goods control certificate
    ///
    /// Document/message submitted to a competent body by party requesting a Goods control certificate to be issued in accordance with national or international standards, or conforming to legislation in the importing country, or as specified in the contract.
    #[serde(rename = "840")]
    _840,
    /// Goods control certificate
    ///
    /// Document/message issued by a competent body evidencing the quality of the goods described therein, in accordance with national or international standards, or conforming to legislation in the importing country, or as specified in the contract.
    #[serde(rename = "841")]
    _841,
    /// Application for phytosanitary certificate
    ///
    /// Document/message submitted to a competent body by party requesting a Phytosanitary certificate to be issued.
    #[serde(rename = "850")]
    _850,
    /// Phytosanitary certificate
    ///
    /// Document/message issued by the competent body in the exporting country evidencing that plants, fruit, or vegetables are free from disease and fit for consumption and giving details on fumigation or other treatment to which they may have been subjected.
    #[serde(rename = "851")]
    _851,
    /// Sanitary certificate
    ///
    /// Document/message issued by the competent authority in the exporting country evidencing that alimentary and animal products, including dead animals, are fit for human consumption, and giving details, when relevant, of controls undertaken.
    #[serde(rename = "852")]
    _852,
    /// Veterinary certificate
    ///
    /// Document/message issued by the competent authority in the exporting country evidencing that live animals or birds are not infested or infected with disease, and giving details regarding their provenance, and of vaccinations and other treatment to which they have been subjected.
    #[serde(rename = "853")]
    _853,
    /// Application for inspection certificate
    ///
    /// Document/message submitted to a competent body by a party requesting an Inspection certificate to be issued in accordance with national or international standards, or conforming to legislation in the country in which it is required, or as specified in the contract.
    #[serde(rename = "855")]
    _855,
    /// Inspection certificate
    ///
    /// Document/message issued by a competent body evidencing that the goods described therein have been inspected in accordance with national or international standards, in conformity with legislation in the country in which the inspection is required, or as specified in the contract.
    #[serde(rename = "856")]
    _856,
    /// Certificate of origin, application for
    ///
    /// Document/message submitted to a competent body by an interested party requesting a Certificate of origin to be issued in accordance with relevant criteria, and on the basis of evidence of the origin of the goods.
    #[serde(rename = "860")]
    _860,
    /// Certificate of origin
    ///
    /// Document/message identifying goods, in which the authority or body authorized to issue it certifies expressly that the goods to which the certificate relates originate in a specific country. The word "country" may include a group of countries, a region or a part of a country. This certificate may also include a declaration by the manufacturer, producer, supplier, exporter or other competent person.
    #[serde(rename = "861")]
    _861,
    /// Declaration of origin
    ///
    /// Appropriate statement as to the origin of the goods, made in connection with their exportation by the manufacturer, producer, supplier, exporter or other competent person on the Commercial invoice or any other document relating to the goods (CCC).
    #[serde(rename = "862")]
    _862,
    /// Regional appellation certificate
    ///
    /// Certificate drawn up in accordance with the rules laid down by an authority or approved body, certifying that the goods described therein qualify for a designation specific to the given region (e.g. champagne, port wine, Parmesan cheese).
    #[serde(rename = "863")]
    _863,
    /// Preference certificate of origin
    ///
    /// Description to be provided.
    #[serde(rename = "864")]
    _864,
    /// Certificate of origin form GSP
    ///
    /// Specific form of certificate of origin for goods qualifying for preferential treatment under the generalized system of preferences (includes a combined declaration of origin and certificate, form A).
    #[serde(rename = "865")]
    _865,
    /// Consular invoice
    ///
    /// Document/message to be prepared by an exporter in his country and presented to a diplomatic representation of the importing country for endorsement and subsequently to be presented by the importer in connection with the import of the goods described therein.
    #[serde(rename = "870")]
    _870,
    /// Dangerous goods declaration
    ///
    /// Document/message issued by a consignor in accordance with applicable conventions or regulations, describing hazardous goods or materials for transport purposes, and stating that the latter have been packed and labelled in accordance with the provisions of the relevant conventions or regulations.
    #[serde(rename = "890")]
    _890,
    /// Statistical document, export
    ///
    /// Document/message in which an exporter provides information about exported goods required by the body responsible for the collection of international trade statistics.
    #[serde(rename = "895")]
    _895,
    /// INTRASTAT declaration
    ///
    /// Document/message in which a declarant provides information about goods required by the body responsible for the collection of trade statistics.
    #[serde(rename = "896")]
    _896,
    /// Delivery verification certificate
    ///
    /// Document/message whereby an official authority (Customs or governmental) certifies that goods have been delivered.
    #[serde(rename = "901")]
    _901,
    /// Import licence, application for
    ///
    /// Document/message in which an interested party applies to the competent body for authorization to import either a limited quantity of articles subject to import restrictions, or an unlimited quantity of such articles during a limited period, and specifies the kind of articles, their origin and value, etc.
    #[serde(rename = "910")]
    _910,
    /// Import licence
    ///
    /// Document/message issued by the competent body in accordance with import regulations in force, by which authorization is granted to a named party to import either a limited quantity of designated articles or an unlimited quantity of such articles during a limited period, under conditions specified in the document.
    #[serde(rename = "911")]
    _911,
    /// Customs declaration without commercial detail
    ///
    /// CUSDEC transmission that does not include data from the commercial detail section of the message.
    #[serde(rename = "913")]
    _913,
    /// Customs declaration with commercial and item detail
    ///
    /// CUSDEC transmission that includes data from both the commercial detail and item detail sections of the message.
    #[serde(rename = "914")]
    _914,
    /// Customs declaration without item detail
    ///
    /// CUSDEC transmission that does not include data from the item detail section of the message.
    #[serde(rename = "915")]
    _915,
    /// Related document
    ///
    /// Description to be provided.
    #[serde(rename = "916")]
    _916,
    /// Receipt (Customs)
    ///
    /// Receipt for Customs duty/tax/fee paid.
    #[serde(rename = "917")]
    _917,
    /// Application for exchange allocation
    ///
    /// Document/message whereby an importer/buyer requests the competent body to allocate an amount of foreign exchange to be transferred to an exporter/seller in payment for goods.
    #[serde(rename = "925")]
    _925,
    /// Foreign exchange permit
    ///
    /// Document/message issued by the competent body authorizing an importer/buyer to transfer an amount of foreign exchange to an exporter/seller in payment for goods.
    #[serde(rename = "926")]
    _926,
    /// Exchange control declaration (import)
    ///
    /// Document/message completed by an importer/buyer as a means for the competent body to control that a trade transaction for which foreign exchange has been allocated has been executed and that money has been transferred in accordance with the conditions of payment and the exchange control regulations in force.
    #[serde(rename = "927")]
    _927,
    /// Goods declaration for importation
    ///
    /// Document/message by which goods are declared for import Customs clearance [sister entry of 830].
    #[serde(rename = "929")]
    _929,
    /// Goods declaration for home use
    ///
    /// Document/message by which goods are declared for import Customs clearance according to Annex B.1 (concerning clearance for home use) to the Kyoto convention (CCC).
    #[serde(rename = "930")]
    _930,
    /// Customs immediate release declaration
    ///
    /// Document/message issued by an importer notifying Customs that goods have been removed from an importing means of transport to the importer's premises under a Customs- approved arrangement for immediate release, or requesting authorization to do so.
    #[serde(rename = "931")]
    _931,
    /// Customs delivery note
    ///
    /// Document/message whereby a Customs authority releases goods under its control to be placed at the disposal of the party concerned. Synonym: Customs release note.
    #[serde(rename = "932")]
    _932,
    /// Cargo declaration (arrival)
    ///
    /// Generic term, sometimes referred to as Freight declaration, applied to the documents providing the particulars required by the Customs concerning the cargo (freight) carried by commercial means of transport (CCC).
    #[serde(rename = "933")]
    _933,
    /// Value declaration
    ///
    /// Document/message in which a declarant (importer) states the invoice or other price (e.g. selling price, price of identical goods), and specifies costs for freight, insurance and packing, etc., terms of delivery and payment, any relationship with the trading partner, etc., for the purpose of determining the Customs value of goods imported.
    #[serde(rename = "934")]
    _934,
    /// Customs invoice
    ///
    /// Document/message required by the Customs in an importing country in which an exporter states the invoice or other price (e.g. selling price, price of identical goods), and specifies costs for freight, insurance and packing, etc., terms of delivery and payment, for the purpose of determining the Customs value in the importing country of goods consigned to that country.
    #[serde(rename = "935")]
    _935,
    /// Customs declaration (post parcels)
    ///
    /// Document/message which, according to Article 106 of the "Agreement concerning Postal Parcels" under the UPU Convention, must accompany post parcels and in which the contents of such parcels are specified.
    #[serde(rename = "936")]
    _936,
    /// Tax declaration (value added tax)
    ///
    /// Document/message in which an importer states the pertinent information required by the competent body for assessment of value-added tax.
    #[serde(rename = "937")]
    _937,
    /// Tax declaration (general)
    ///
    /// Document/message containing a general tax declaration.
    #[serde(rename = "938")]
    _938,
    /// Tax demand
    ///
    /// Document/message containing the demand of tax.
    #[serde(rename = "940")]
    _940,
    /// Embargo permit
    ///
    /// Document/message giving the permission to export specified goods.
    #[serde(rename = "941")]
    _941,
    /// Goods declaration for Customs transit
    ///
    /// Document/message by which the sender declares goods for Customs transit according to Annex E.1 (concerning Customs transit) to the Kyoto convention (CCC).
    #[serde(rename = "950")]
    _950,
    /// TIF form
    ///
    /// International Customs transit document by which the sender declares goods for carriage by rail in accordance with the provisions of the 1952 International Convention to facilitate the crossing of frontiers for goods carried by rail (TIF Convention of UIC).
    #[serde(rename = "951")]
    _951,
    /// TIR carnet
    ///
    /// International Customs document (International Transit by Road), issued by a guaranteeing association approved by the Customs authorities, under the cover of which goods are carried, in most cases under Customs seal, in road vehicles and/or containers in compliance with the requirements of the Customs TIR Convention of the International Transport of Goods under cover of TIR Carnets (UN/ECE).
    #[serde(rename = "952")]
    _952,
    /// EC carnet
    ///
    /// EC customs transit document issued by EC customs authorities for transit and/or temporary user of goods within the EC.
    #[serde(rename = "953")]
    _953,
    /// EUR 1 certificate of origin
    ///
    /// Customs certificate used in preferential goods interchanges between EC countries and EC external countries.
    #[serde(rename = "954")]
    _954,
    /// ATA carnet
    ///
    /// International Customs document (Admission Temporaire / Temporary Admission) which, issued under the terms of the ATA Convention (1961), incorporates an internationally valid guarantee and may be used, in lieu of national Customs documents and as security for import duties and taxes, to cover the temporary admission of goods and, where appropriate, the transit of goods. If accepted for controlling the temporary export and reimport of goods, international guarantee does not apply (CCC).
    #[serde(rename = "955")]
    _955,
    /// Single administrative document
    ///
    /// A set of documents, replacing the various (national) forms for Customs declaration within the EC, implemented on 01-01-1988.
    #[serde(rename = "960")]
    _960,
    /// General response (Customs)
    ///
    /// General response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[serde(rename = "961")]
    _961,
    /// Document response (Customs)
    ///
    /// Document response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[serde(rename = "962")]
    _962,
    /// Error response (Customs)
    ///
    /// Error response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[serde(rename = "963")]
    _963,
    /// Package response (Customs)
    ///
    /// Package response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[serde(rename = "964")]
    _964,
    /// Tax calculation/confirmation response (Customs)
    ///
    /// Tax calculation/confirmation response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[serde(rename = "965")]
    _965,
    /// Quota prior allocation certificate
    ///
    /// Document/message issued by the competent body for prior allocation of a quota.
    #[serde(rename = "966")]
    _966,
    /// End use authorization
    ///
    /// Description to be provided.
    #[serde(rename = "990")]
    _990,
    /// Government contract
    ///
    /// Description to be provided.
    #[serde(rename = "991")]
    _991,
    /// Statistical document, import
    ///
    /// Description to be provided.
    #[serde(rename = "995")]
    _995,
    /// Application for documentary credit
    ///
    /// Message with application for opening of a documentary credit.
    #[serde(rename = "996")]
    _996,
    /// Previous Customs document/message
    ///
    /// Indication of the previous Customs document/message concerning the same transaction.
    #[serde(rename = "998")]
    _998,
}

/// Message function code
///
/// Code indicating the function of the message.
#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display, PartialEq)]
#[strum(serialize_all = "snake_case")]
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

/// Code list identification code
///
/// Code identifying a code list.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _1131 {
    /// Logistics code list
    ///
    /// Code list containing logistics and program management activities.
    #[serde(rename = "1")]
    _1,
    /// ICD 9
    ///
    /// A code list containing the International Classification of Diseases, version 9 (ICD 9).
    #[serde(rename = "2")]
    _2,
    /// Operating status
    ///
    /// Code list identifying operating status of an entity.
    #[serde(rename = "3")]
    _3,
    /// DoDAAC (Department of Defense Activity Address Code)
    ///
    /// A code list containing codes assigned to operating military units to identify the name and address of the unit.
    #[serde(rename = "4")]
    _4,
    /// Facility identification
    ///
    /// A code list identifying a facility(ies).
    #[serde(rename = "5")]
    _5,
    /// Application acknowledgement and error codes
    ///
    /// A code list to identify acknowledgement and error codes applicable at the application level.
    #[serde(rename = "6")]
    _6,
    /// Health industry organization identification
    ///
    /// List of codes identifying organizations in the health care industry.
    #[serde(rename = "7")]
    _7,
    /// Electromagnetic transmitter identification
    ///
    /// A code list containing electromagnetic transmitter identifications.
    #[serde(rename = "8")]
    _8,
    /// Military Assistance Program Address Code (MAPAC)
    ///
    /// Lists of codes identifying name and address information for organizations participating in a military assistance program.
    #[serde(rename = "9")]
    _9,
    /// Medicare provider
    ///
    /// A list of codes identifying health care providers under the Medicare program.
    #[serde(rename = "10")]
    _10,
    /// Medicaid provider
    ///
    /// A list of codes identifying health care providers under a Medicaid program.
    #[serde(rename = "11")]
    _11,
    ///
    /// Telephone directory
    #[serde(rename = "12")]
    _12,
    /// Employee identification
    ///
    /// A list of codes identifying employees of an organization.
    #[serde(rename = "13")]
    _13,
    /// Sample extraction location
    ///
    /// Code list identifying the location from which a sample is taken.
    #[serde(rename = "14")]
    _14,
    /// Medical benefits schedule
    ///
    /// Code list containing classifications of medical services for use in determining the medical benefits payable.
    #[serde(rename = "15")]
    _15,
    /// Postcode directory
    ///
    /// [3251] Code defining postal zones or addresses.
    #[serde(rename = "16")]
    _16,
    /// ICD 10
    ///
    /// Code list containing the International Classification of Diseases, version 10 (ICD 10).
    #[serde(rename = "17")]
    _17,
    /// Diagnosis Related Group (DRG)
    ///
    /// Code list containing diagnosis related group classifications.
    #[serde(rename = "18")]
    _18,
    /// Standard text clauses
    ///
    /// A list of codes representing standardized text clauses.
    #[serde(rename = "19")]
    _19,
    /// United Nations Standard Products and Services Classification (UN/SPSC) code
    ///
    /// A code list that provides a hierarchical classification of goods and services for the purposes of resource discovery and spend analysis.
    #[serde(rename = "20")]
    _20,
    /// Policy on claim indicator
    /// Identifies a code list containing indicators referring to policy on claims.
    ///
    /// (legal persons, partnerships, sole proprietorships and their branch offices) and private persons.
    #[serde(rename = "21")]
    _21,
    /// A code list specifying codes assigned by the EDI Registration Authority to register organizations
    ///
    /// EDIRA-Id (EDI Registration Authority Identification)
    #[serde(rename = "22")]
    _22,
    ///
    /// Clearing house automated payment
    #[serde(rename = "23")]
    _23,
    /// Rail handling restrictions and instructions
    ///
    /// A code list specifying rail codes for handling restrictions or instructions.
    #[serde(rename = "24")]
    _24,
    /// Bank identification
    ///
    /// Code for identification of banks.
    #[serde(rename = "25")]
    _25,
    /// Rail harmonized equipment type
    ///
    /// A code list specifying codes for harmonized equipment type in the railway industry.
    #[serde(rename = "26")]
    _26,
    /// Railway frontier and transit point
    ///
    /// A code list specifying frontier or transit points in the railway industry.
    #[serde(rename = "27")]
    _27,
    /// Commercial And Government Entity (CAGE)
    ///
    /// List of codes identifying a commercial and government entity.
    #[serde(rename = "33")]
    _33,
    /// Reinsurance policy attributes
    ///
    /// A list of attributes regarding policies reinsured with a professional reinsurer.
    #[serde(rename = "34")]
    _34,
    /// Rail additional charges
    ///
    /// A code list identifying specific rail charges included in the payment conditions in addition to the freight cost.
    #[serde(rename = "35")]
    _35,
    /// Railway company network
    ///
    /// A code list identifying the different railway companies as member of the International Union of Railways.
    #[serde(rename = "36")]
    _36,
    /// Railway locations
    ///
    /// Code identifying a location in railway environment.
    #[serde(rename = "37")]
    _37,
    /// Railway customer
    ///
    /// A code list identifying rail customers.
    #[serde(rename = "38")]
    _38,
    ///
    /// Rail unified nomenclature of goods
    #[serde(rename = "39")]
    _39,
    /// Reinsurance monetary type
    ///
    /// Identifies the type of reinsurance amounts.
    #[serde(rename = "40")]
    _40,
    ///
    /// Business function
    #[serde(rename = "42")]
    _42,
    /// Clearing House Interbank Payment System Participants ID
    ///
    /// Participants identification of the automated clearing house of the New York Clearing House Association (CHIPS).
    #[serde(rename = "43")]
    _43,
    /// Clearing House Interbank Payment System Universal ID
    ///
    /// Universal identification of the automated clearing house of the New York Clearing House Association (CHIPS).
    #[serde(rename = "44")]
    _44,
    /// United Nations Common Coding System (UNCCS)
    ///
    /// A code list adopted by the United Nations organisations for the procurement of goods and services.
    #[serde(rename = "45")]
    _45,
    /// DUNS (Dun and Bradstreet) +4
    ///
    /// An organization identified by the DUNS number and a 4- character extension.
    #[serde(rename = "46")]
    _46,
    /// Occupation classification
    ///
    /// Identifies the class of occupation.
    #[serde(rename = "47")]
    _47,
    /// Policy reserve valuation type
    ///
    /// Identification of the policy reserve valuation type.
    #[serde(rename = "48")]
    _48,
    /// Life reinsurance message type
    ///
    /// To indicate the type of life reinsurance activity transmitted in the message.
    #[serde(rename = "49")]
    _49,
    /// Value added tax identification
    ///
    /// Value added tax identification code.
    #[serde(rename = "52")]
    _52,
    /// Passport number
    ///
    /// Number assigned to a passport.
    #[serde(rename = "53")]
    _53,
    /// Statistical object
    ///
    /// A statistical object such as a statistical concept, array structure component or statistical nomenclature.
    #[serde(rename = "54")]
    _54,
    /// Quality conformance
    ///
    /// A code list specifying the quality standard a product complies with, e.g. ISO9000, BS5750, etc.
    #[serde(rename = "55")]
    _55,
    /// Safety regulation
    ///
    /// A code list specifying the safety regulations which apply to a product, such as UK COSHH (control of substances hazardous to health) regulations.
    #[serde(rename = "56")]
    _56,
    /// Product code
    ///
    /// Code assigned to a specific product by a controlling agency.
    #[serde(rename = "57")]
    _57,
    /// Business account number
    ///
    /// An identifying number or code assigned by issuing authorities to manage business activities.
    #[serde(rename = "58")]
    _58,
    /// Railway services harmonized code
    ///
    /// Services provided by the different railway organizations.
    #[serde(rename = "59")]
    _59,
    /// Type of financial account
    ///
    /// Identification of the type of financial account.
    #[serde(rename = "60")]
    _60,
    /// Type of assets and liabilities
    ///
    /// Identification of the type of assets and liabilities.
    #[serde(rename = "61")]
    _61,
    /// Requirements indicator
    ///
    /// A code list which specifies various requirements that a customer may have when fulfilling a purchase order.
    #[serde(rename = "62")]
    _62,
    /// Handling action
    ///
    /// Codes for handling action.
    #[serde(rename = "63")]
    _63,
    /// Freight forwarder
    ///
    /// Codes for freight forwarders.
    #[serde(rename = "64")]
    _64,
    /// Shipping agent
    ///
    /// Codes for shipping agents.
    #[serde(rename = "65")]
    _65,
    /// Type of package
    ///
    /// Indication of the type of package codes.
    #[serde(rename = "67")]
    _67,
    /// Type of industrial activity
    ///
    /// Identification of the type of industrial activity.
    #[serde(rename = "68")]
    _68,
    /// Type of survey question
    ///
    /// Identification of the type of survey question.
    #[serde(rename = "69")]
    _69,
    /// Customs inspection type
    ///
    /// A code to indicate the type of inspection performed by customs.
    #[serde(rename = "70")]
    _70,
    /// Nature of transaction
    ///
    /// Identification of the nature of the transaction.
    #[serde(rename = "71")]
    _71,
    /// Container terminal
    ///
    /// Codes for container terminal.
    #[serde(rename = "72")]
    _72,
    /// Insurance information indicator
    ///
    /// Identifies the type of insurance information provided.
    #[serde(rename = "73")]
    _73,
    /// Joint life insurance indicator
    ///
    /// Indicates joint life insurance coverage.
    #[serde(rename = "74")]
    _74,
    /// Bill of lading clauses
    ///
    /// Code list identifying official clauses associated with bills of lading.
    #[serde(rename = "75")]
    _75,
    /// Export commodity classification (US Schedule B)
    ///
    /// Code list containing the commodity classifications applying to goods being exported (United States Schedule B).
    #[serde(rename = "76")]
    _76,
    /// Customs domestic port location codes (US Schedule D)
    ///
    /// Code list containing Customs domestic port locations (United States Schedule D).
    #[serde(rename = "77")]
    _77,
    /// Customs foreign port location codes (US Schedule K)
    ///
    /// Code list containing Customs foreign port locations (United States Schedule K).
    #[serde(rename = "78")]
    _78,
    /// Functional group
    ///
    /// Identifies a group of application related messages.
    #[serde(rename = "79")]
    _79,
    /// Application error code
    ///
    /// A code list specifying application errors.
    #[serde(rename = "80")]
    _80,
    /// Policy type
    ///
    /// To identify the code list for the type of policy.
    #[serde(rename = "81")]
    _81,
    /// Type of insured
    ///
    /// To specify the insured type.
    #[serde(rename = "82")]
    _82,
    /// Occupation code
    ///
    /// Identification of an occupation.
    #[serde(rename = "83")]
    _83,
    /// State code
    ///
    /// A code list of states within a country.
    #[serde(rename = "84")]
    _84,
    /// Technical Assessment Checklist (TAC)
    ///
    /// A code list of technical assessment checklist numbers.
    #[serde(rename = "85")]
    _85,
    /// Syntax notes
    ///
    /// A code list of syntax (dependency) note identifiers.
    #[serde(rename = "86")]
    _86,
    ///
    /// Enhanced party identification
    #[serde(rename = "100")]
    _100,
    ///
    /// Air carrier
    #[serde(rename = "101")]
    _101,
    ///
    /// Size and type
    #[serde(rename = "102")]
    _102,
    /// Call sign directory
    ///
    /// A directory of call signs assigned to transport vehicles.
    #[serde(rename = "103")]
    _103,
    /// Customs area of transaction
    ///
    /// Customs code to indicate the different types of declarations according to the countries involved in the transaction (e.g. box 1/1 of SAD: inter EC Member States, EC-EFTA, EC-third countries, etc.).
    #[serde(rename = "104")]
    _104,
    /// Customs declaration type
    ///
    /// Customs code to indicate the type of declaration according to the different Customs procedures requested (e.g.: import, export, transit).
    #[serde(rename = "105")]
    _105,
    /// Incoterms 1980
    ///
    /// (4110) Code to indicate applicable Incoterm (1980 edition) under which seller undertakes to deliver merchandise to buyer (ICC). Incoterms 1990: use 4053 only.
    #[serde(rename = "106")]
    _106,
    /// Excise duty
    ///
    /// Customs or fiscal authorities code to identify a specific or ad valorem levy on a specific commodity, applied either domestically or at time of importation.
    #[serde(rename = "107")]
    _107,
    ///
    /// Tariff schedule
    #[serde(rename = "108")]
    _108,
    /// Customs indicator
    ///
    /// Customs code for circumstances where only an indication is needed.
    #[serde(rename = "109")]
    _109,
    /// Customs special codes
    ///
    /// Customs code to indicate an exemption to a regulation or a special Customs treatment.
    #[serde(rename = "110")]
    _110,
    /// Statistical nature of transaction
    ///
    /// Indication of the type of contract under which goods are supplied.
    #[serde(rename = "112")]
    _112,
    /// Customs office
    ///
    /// Customs administrative unit competent for the performance of Customs formalities, and the premises or other areas approved for the purpose by the competent authorities (CCC).
    #[serde(rename = "113")]
    _113,
    /// Railcar letter marking
    ///
    /// Codes for all marking codes (in letters) for railcars specifying the type, series, order number, check digit and some technical characteristics.
    #[serde(rename = "114")]
    _114,
    /// Examination facility
    ///
    /// Building or location where merchandise is examined by Customs.
    #[serde(rename = "115")]
    _115,
    /// Customs preference
    ///
    /// Customs code to identify a specific tariff preference.
    #[serde(rename = "116")]
    _116,
    /// Customs procedure
    ///
    /// (9380) Customs code to identify goods which are subject to Customs control (e.g. home use, Customs warehousing, temporary admission, Customs transit).
    #[serde(rename = "117")]
    _117,
    /// Government agency procedure
    ///
    /// Treatment applied by a government agency other than Customs to merchandise under their control.
    #[serde(rename = "118")]
    _118,
    /// Customs simplified procedure
    ///
    /// Customs code to indicate the type of simplified Customs procedure requested by a declarant (CCC).
    #[serde(rename = "119")]
    _119,
    /// Customs status of goods
    ///
    /// Customs code to specify the status accorded by Customs to a consignment e.g. release without further formality, present supporting documents for inspection, etc (CCC).
    #[serde(rename = "120")]
    _120,
    /// Shipment description
    ///
    /// Code to indicate whether a shipment is a total, part or split consignment.
    #[serde(rename = "121")]
    _121,
    /// Commodity
    ///
    /// (7357) Code identifying types of goods for Customs, transport or statistical purposes (generic term).
    #[serde(rename = "122")]
    _122,
    /// Entitlement
    ///
    /// Code to indicate the recipient of a charge amount (IATA).
    #[serde(rename = "123")]
    _123,
    /// Customs transit guarantee
    ///
    /// Customs code to identify the type of guarantee used in a transit movement.
    #[serde(rename = "125")]
    _125,
    /// Accounting information identifier
    ///
    /// Identification of a specific kind of accounting information.
    #[serde(rename = "126")]
    _126,
    /// Customs valuation method
    ///
    /// Customs code to identify the valuation method used to determine the dutiable value of the declared goods.
    #[serde(rename = "127")]
    _127,
    /// Service
    ///
    /// Identification of services.
    #[serde(rename = "128")]
    _128,
    /// Customs warehouse
    ///
    /// Identification and/or location of the Customs warehouse in which goods will be or have been deposited (CCC).
    #[serde(rename = "129")]
    _129,
    /// Special handling
    ///
    /// Code to indicate that the nature of the consignment may necessitate use of special handling procedures (IATA).
    #[serde(rename = "130")]
    _130,
    /// Free zone
    ///
    /// Code identifying the zone within a state where any goods introduced are generally regarded, insofar as import duties and taxes are concerned, as being outside the Customs territory and are not subject to the usual Customs control.
    #[serde(rename = "131")]
    _131,
    /// Charge
    ///
    /// Identification of a type of charge.
    #[serde(rename = "132")]
    _132,
    /// Financial regime
    ///
    /// Nature and methods of a transaction from financial viewpoint.
    #[serde(rename = "133")]
    _133,
    /// Duty, tax or fee payment method
    ///
    /// [4390] Method by which a duty or tax is paid to the relevant administration.
    #[serde(rename = "134")]
    _134,
    /// Rate class
    ///
    /// Code to identify a specific rate category.
    #[serde(rename = "135")]
    _135,
    /// Restrictions and prohibitions placed on the re-use of designated rail wagons
    ///
    /// A code list identifying restrictions and prohibitions placed on the re-use of designated rail wagons.
    #[serde(rename = "136")]
    _136,
    /// Rail harmonized codification of tariffs
    ///
    /// A list of rail tariffs, the coding of which has been harmonized.
    #[serde(rename = "137")]
    _137,
    /// Port
    ///
    /// A location having facilities for means of transport to load or discharge cargo.
    #[serde(rename = "139")]
    _139,
    /// Area
    ///
    /// Codes for specific geographic areas e.g. seas, straits, basins etc.
    #[serde(rename = "140")]
    _140,
    /// Forwarding restrictions
    ///
    /// A code list containing restrictions regarding the forwarding of goods or equipment.
    #[serde(rename = "141")]
    _141,
    /// Train identification
    ///
    /// A code list specifying international train identifications maintained by the UIC (International Union of Railways).
    #[serde(rename = "142")]
    _142,
    /// Removable accessories and special equipment on railcars
    ///
    /// A list of removable accessories and special equipment associated with railcars.
    #[serde(rename = "143")]
    _143,
    /// Rail routes
    ///
    /// A code list identifying routes used in rail transport.
    #[serde(rename = "144")]
    _144,
    /// Airport/city
    ///
    /// As described and published by IATA.
    #[serde(rename = "145")]
    _145,
    /// Means of transport identification
    ///
    /// Code identifying the name or number of a means of transport (vessel, vehicle).
    #[serde(rename = "146")]
    _146,
    /// Document requested by Customs
    ///
    /// Customs code to identify documents requested by Customs in an information interchange.
    #[serde(rename = "147")]
    _147,
    /// Customs release notification
    ///
    /// Authorisation given by Customs to move the goods or not move the goods from the place of registration.
    #[serde(rename = "148")]
    _148,
    /// Customs transit type
    ///
    /// Customs code to indicate the different kinds of transit movement of the goods (e.g. Box 1/3 of the SAD).
    #[serde(rename = "149")]
    _149,
    ///
    /// Financial routing
    #[serde(rename = "150")]
    _150,
    /// Locations for tariff calculations
    ///
    /// A list of locations related to tariff calculations.
    #[serde(rename = "151")]
    _151,
    ///
    /// Materials
    #[serde(rename = "152")]
    _152,
    /// Methods of payment
    ///
    /// Identification of methods of payment.
    #[serde(rename = "153")]
    _153,
    /// Bank branch sorting identification
    ///
    /// Identification of a specific branch of a bank.
    #[serde(rename = "154")]
    _154,
    /// Automated clearing house
    ///
    /// Identification of automated clearing houses.
    #[serde(rename = "155")]
    _155,
    /// Location of goods
    ///
    /// (3384) Indication of the place where goods are located and where they are available for examination.
    #[serde(rename = "156")]
    _156,
    /// Clearing code
    ///
    /// Identification of the responsible bank/clearing house which has cleared or is ordered to do the clearing.
    #[serde(rename = "157")]
    _157,
    /// Terms of delivery
    ///
    /// Code to identify terms of delivery.
    #[serde(rename = "158")]
    _158,
    /// Party identification
    ///
    /// Identification of parties, corporates, etc.
    #[serde(rename = "160")]
    _160,
    /// Goods description
    ///
    /// Identification of a type of goods description.
    #[serde(rename = "161")]
    _161,
    /// Country
    ///
    /// Identification of a country.
    #[serde(rename = "162")]
    _162,
    /// Country sub-entity
    ///
    /// (3228) Identification of country sub-entity (region, department, state, province) defined by appropriate authority.
    #[serde(rename = "163")]
    _163,
    /// Member organizations
    ///
    /// Identification of member organizations.
    #[serde(rename = "164")]
    _164,
    /// Amendment code (Customs)
    ///
    /// Customs code indicating the reason for transmitting an amendment to Customs.
    #[serde(rename = "165")]
    _165,
    /// Social security identification
    ///
    /// Code assigned by the authority competent to issue social security identification to identify a person.
    #[serde(rename = "166")]
    _166,
    /// Tax party identification
    ///
    /// Code assigned by a tax authority to identify a party.
    #[serde(rename = "167")]
    _167,
    /// Rail document names
    ///
    /// Rail specific identifications of documents.
    #[serde(rename = "168")]
    _168,
    /// Harmonized system
    ///
    /// Identification of commodities according to the Harmonized System.
    #[serde(rename = "169")]
    _169,
    ///
    /// Bank securities code
    #[serde(rename = "170")]
    _170,
    /// Carriers
    ///
    /// Code list identifying carriers.
    #[serde(rename = "172")]
    _172,
    /// Export requirements
    ///
    /// Identification of requirements and regulations established by relevant authorities concerning exportation.
    #[serde(rename = "173")]
    _173,
    /// Citizen identification
    #[serde(rename = "174")]
    _174,
    /// Account analysis codes
    ///
    /// Account service charges list.
    #[serde(rename = "175")]
    _175,
    /// Flow of the goods
    ///
    /// List of statistical codes covering the movement of the goods to be declared (e.g. despatch, arrival).
    #[serde(rename = "176")]
    _176,
    /// Statistical procedures
    ///
    /// Indication of the statistical procedure to which the goods are subject.
    #[serde(rename = "177")]
    _177,
    /// Standard text according US embargo regulations
    ///
    /// US government regulations prescribe specific standard text usage. Using codes from this code list prevents full text transmission.
    #[serde(rename = "178")]
    _178,
    /// Standard text for export according national prescriptions
    ///
    /// National export regulations prescribe specific standard text usage. Using codes from this code list prevents full text transmission.
    #[serde(rename = "179")]
    _179,
    /// Airport terminal
    ///
    /// Code identifying terminals or other sub-locations at airports.
    #[serde(rename = "180")]
    _180,
    /// Activity
    ///
    /// Code identifying activities.
    #[serde(rename = "181")]
    _181,
    /// Combiterms 1990
    ///
    /// Code to indicate the applicable Combiterm (1990 edition), used for the purpose of cost distribution between seller according to Incoterms 1990.
    #[serde(rename = "182")]
    _182,
    /// Dangerous goods packing type
    ///
    /// Identification of package types for the description related to dangerous goods.
    #[serde(rename = "183")]
    _183,
    /// Tax assessment method
    ///
    /// A code to identify the tax assessment method.
    #[serde(rename = "184")]
    _184,
    /// Item type
    ///
    /// A code list defining the level of elaboration of a item such as raw material, component, tooling, etc.
    #[serde(rename = "185")]
    _185,
    /// Product supply condition
    ///
    /// A code list specifying the rules according to which a product is supplied, e.g. from stock, available on demand, make on order, etc.
    #[serde(rename = "186")]
    _186,
    /// Supplier's stock turnover
    ///
    /// A code list giving an indication about the level of the supplier's stock turnover.
    #[serde(rename = "187")]
    _187,
    /// Article status
    ///
    /// A code list defining the status of an article from the procurement point of view, e.g. new article, critical article, etc.
    #[serde(rename = "188")]
    _188,
    /// Quality control code
    ///
    /// A code list specifying how the article is classified according to the quality control point of view, e.g. safety item, subject to regulation, etc.
    #[serde(rename = "189")]
    _189,
    /// Item sourcing category
    ///
    /// A code list to specify details related to the sourcing of the corresponding item such as provided by the buyer, from a mandatory source, etc.
    #[serde(rename = "190")]
    _190,
    /// Dumping or countervailing assessment method
    ///
    /// A code to identify the method used to determine the dumping or countervailing duty.
    #[serde(rename = "191")]
    _191,
    /// Dumping specification
    ///
    /// Code list to identify types of goods for dumping purposes.
    #[serde(rename = "192")]
    _192,
    /// Legal event
    ///
    /// Identifies a code list of legal events.
    #[serde(rename = "193")]
    _193,
    /// Record precedence based on its currency in time
    ///
    /// Identifies the priority of a record based on its currency in time.
    #[serde(rename = "194")]
    _194,
    /// Ownership rights
    ///
    /// Identifies a code list containing types of ownership rights.
    #[serde(rename = "195")]
    _195,
    /// Property ownership extent
    ///
    /// Identifies a code list containing the extent of legal rights of possession to property.
    #[serde(rename = "196")]
    _196,
    /// Monetary function detail
    ///
    /// Identifies a code list containing monetary function details.
    #[serde(rename = "197")]
    _197,
    /// Account relationship type
    ///
    /// Identifies a code list containing types of account relationships.
    #[serde(rename = "198")]
    _198,
    /// Account rating
    ///
    /// Identifies the code list containing account rating types.
    #[serde(rename = "199")]
    _199,
    /// Loan type
    ///
    /// Identifies the code list of loan types.
    #[serde(rename = "200")]
    _200,
    /// Claim type
    ///
    /// Identifies the code list containing the claim types.
    #[serde(rename = "201")]
    _201,
    /// Legal case type
    ///
    /// Identifies the code list containing the type of legal cases.
    #[serde(rename = "202")]
    _202,
    /// Court of law event type
    ///
    /// Identifies the code list containing the type of law events.
    #[serde(rename = "203")]
    _203,
    /// Notice type
    ///
    /// Identifies the code list containing the type of notice.
    #[serde(rename = "204")]
    _204,
    /// Ethnicity
    ///
    /// Identifies the code list containing ethnic types.
    #[serde(rename = "205")]
    _205,
    /// Individual participation in company
    ///
    /// Identifies the code list containing the types of participation of an individual within a company.
    #[serde(rename = "206")]
    _206,
    /// Real estate asset type
    ///
    /// Identifies the code list containing the types of real estate assets.
    #[serde(rename = "207")]
    _207,
    /// Asset recurrence
    ///
    /// Identifies the code list containing the types of recurrences of assets.
    #[serde(rename = "208")]
    _208,
    /// Construction material
    ///
    /// Identifies the code list containing types of materials used for construction.
    #[serde(rename = "209")]
    _209,
    /// Information request type
    ///
    /// Identifies a code list containing types of information requests.
    #[serde(rename = "210")]
    _210,
    /// Business change
    ///
    /// Identifies a code list containing business change types.
    #[serde(rename = "211")]
    _211,
    /// Business credit rating
    ///
    /// Identifies a code list containing business credit rating types.
    #[serde(rename = "212")]
    _212,
    /// Corporate financial filing criteria
    ///
    /// Identifies a code list containing criteria for corporate financial filings.
    #[serde(rename = "213")]
    _213,
    /// Reason for public record filing
    ///
    /// Identifies a code list containing reasons for public record filings.
    #[serde(rename = "214")]
    _214,
    /// Registration type
    ///
    /// Identifies a code list containing registration types.
    #[serde(rename = "215")]
    _215,
    /// Stock exchange detail
    ///
    /// Identifies a code list containing stock exchange details.
    #[serde(rename = "216")]
    _216,
    /// Business legal structure type
    ///
    /// Identifies a code list containing business legal structure details.
    #[serde(rename = "217")]
    _217,
    /// Information request result
    ///
    /// Identifies a code list containing information request results.
    #[serde(rename = "218")]
    _218,
    /// Financial information type
    ///
    /// Identifies a code list containing financial information types.
    #[serde(rename = "219")]
    _219,
    /// Consolidation detail
    ///
    /// Identifies a code list containing consolidation details.
    #[serde(rename = "220")]
    _220,
    /// Condition detail
    ///
    /// Identifies a code list containing condition details.
    #[serde(rename = "221")]
    _221,
    /// Financial statement format
    ///
    /// Identifies a code list containing financial statement formats.
    #[serde(rename = "222")]
    _222,
    /// Source of disclosure
    ///
    /// Identifies a code list containing disclosure sources.
    #[serde(rename = "223")]
    _223,
    /// General territory type
    ///
    /// Identifies a code list containing general territory types.
    #[serde(rename = "224")]
    _224,
    /// Roadway type
    ///
    /// Identifies a code list containing roadway types.
    #[serde(rename = "225")]
    _225,
    /// Roadway detail
    ///
    /// Identifies a code list containing roadway details.
    #[serde(rename = "226")]
    _226,
    /// City
    ///
    /// Identifies a code list containing cities.
    #[serde(rename = "227")]
    _227,
    /// County
    ///
    /// Identifies a code list containing counties. A county is any of the territorial divisions of some countries, forming the chief unit of local administration.
    #[serde(rename = "228")]
    _228,
    /// Geographic location
    ///
    /// Identifies a code list containing geographic locations.
    #[serde(rename = "229")]
    _229,
    /// Entity relationship
    ///
    /// Identifies a code list of entity relationships.
    #[serde(rename = "230")]
    _230,
    /// Payment behaviour rating
    ///
    /// Identifies a code list containing payment behaviour ratings.
    #[serde(rename = "231")]
    _231,
    /// Inquiry selection
    ///
    /// Identifies a code list containing inquiry selections.
    #[serde(rename = "232")]
    _232,
    /// Rating summary value
    ///
    /// Identifies a code list containing rating summary values.
    #[serde(rename = "233")]
    _233,
    /// Industry rating
    ///
    /// Identifies a code list containing industry ratings.
    #[serde(rename = "234")]
    _234,
    /// Forecast type
    ///
    /// Identifies a code list containing forecast types.
    #[serde(rename = "235")]
    _235,
    /// Hobby
    ///
    /// Identifies a code list containing hobby types.
    #[serde(rename = "236")]
    _236,
    /// Functional business area
    ///
    /// Identifies a code list containing functional business areas.
    #[serde(rename = "237")]
    _237,
    /// Current asset details
    ///
    /// Identifies a code list containing details of the current asset types.
    #[serde(rename = "238")]
    _238,
    /// Asset details
    ///
    /// Identifies a code list containing details of the asset types.
    #[serde(rename = "239")]
    _239,
    /// Current liability details
    ///
    /// Identifies a code list containing the current liability types.
    #[serde(rename = "240")]
    _240,
    /// Liability details
    ///
    /// Identifies a code list containing details of liability types.
    #[serde(rename = "241")]
    _241,
    /// Financial item reclassification
    ///
    /// Identifies a code list containing financial item reclassifications.
    #[serde(rename = "242")]
    _242,
    /// Financial item allocation
    ///
    /// Identifies a code list containing financial item allocations.
    #[serde(rename = "243")]
    _243,
    /// Reason for financial item detail change
    ///
    /// Identifies a code list containing reasons for the change in financial item details.
    #[serde(rename = "244")]
    _244,
    /// Educational institution type
    ///
    /// Identifies a code list containing educational institution types.
    #[serde(rename = "245")]
    _245,
    /// Educational study area
    ///
    /// Identifies a code list containing educational study areas.
    #[serde(rename = "246")]
    _246,
    /// Security share type
    ///
    /// Identifies a code list containing security share types.
    #[serde(rename = "247")]
    _247,
    /// Insurance coverage detail
    ///
    /// Identifies a code list containing insurance coverage details.
    #[serde(rename = "248")]
    _248,
    /// Property type
    ///
    /// Identifies a code list containing property types.
    #[serde(rename = "249")]
    _249,
    /// Data category
    ///
    /// Identifies a code list containing data categories.
    #[serde(rename = "250")]
    _250,
    /// Information type
    ///
    /// Identifies a code list containing types of information.
    #[serde(rename = "251")]
    _251,
    /// Court of law type
    ///
    /// Identifies a code list containing court of law types.
    #[serde(rename = "252")]
    _252,
    /// Region
    ///
    /// Identifies a code list containing regions that identify an area of the earth's surface, having definable boundaries or characteristics.
    #[serde(rename = "253")]
    _253,
    /// Postal service carrier route
    ///
    /// Identifies a code list containing routes covered by a postal service carrier.
    #[serde(rename = "254")]
    _254,
    /// Continent
    ///
    /// Identifies a code list containing continents, that are any of the main continuous expanses of land.
    #[serde(rename = "255")]
    _255,
    /// Postal district
    ///
    /// Identifies a code list containing territories for the routing of mail.
    #[serde(rename = "256")]
    _256,
    /// Non-postal town
    ///
    /// Identifies a code list containing towns not recognised as a postal entity.
    #[serde(rename = "257")]
    _257,
    /// City subdivision
    ///
    /// Identifies a code list containing subdivisions of a city.
    #[serde(rename = "258")]
    _258,
    /// Financial analysis categories
    ///
    /// Identifies a code list containing financial analysis categories.
    #[serde(rename = "259")]
    _259,
    /// Accord Europeen relatif au transport international des marchandises(ADR).
    ///
    /// A code list identifying dangerous goods for transport purposes.
    #[serde(rename = "260")]
    _260,
    /// Consignee's premises
    ///
    /// Facility controlled by the consignee of cargo.
    #[serde(rename = "261")]
    _261,
    /// Consignor's premises
    ///
    /// Facility controlled by the consignor of cargo.
    #[serde(rename = "262")]
    _262,
    /// Packing and/or unpacking facility
    ///
    /// Facility dedicated to the packing and/or unpacking of cargo.
    #[serde(rename = "263")]
    _263,
    /// Storage facility
    ///
    /// Facility at which goods are stored.
    #[serde(rename = "264")]
    _264,
    /// Repair facility
    ///
    /// Facility at which repairs are carried out.
    #[serde(rename = "265")]
    _265,
    /// Marine berth
    ///
    /// The location within a port where a ship anchors or ties up.
    #[serde(rename = "266")]
    _266,
    /// Marine wharf
    ///
    /// Landing platform where a ship can load and unload.
    #[serde(rename = "267")]
    _267,
    /// Gate
    ///
    /// The location at which access to or from a facility is controlled.
    #[serde(rename = "268")]
    _268,
    /// Warehouse
    ///
    /// A covered facility for the storage and distribution of goods.
    #[serde(rename = "269")]
    _269,
    /// Business classification
    ///
    /// Code list of business classifications.
    #[serde(rename = "270")]
    _270,
    /// Facility security clearance
    ///
    /// Code list specifying the security clearance assigned to a facility.
    #[serde(rename = "271")]
    _271,
    /// Individual security clearance
    ///
    /// Code list specifying the security clearance assigned to an individual.
    #[serde(rename = "272")]
    _272,
    /// Means of communications identifier
    ///
    /// Code list of communication means used to transmit data.
    #[serde(rename = "273")]
    _273,
    /// Mutually defined
    #[strum(ascii_case_insensitive)]
    _ZZZ,
}

/// Date or time or period function code qualifier
///
/// Code qualifying the function of a date, time or period.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _2005 {
    /// Service completion date/time, actual
    ///
    /// Actual date/time on which the service was completed.
    #[serde(rename = "1")]
    _1,
    /// Delivery date/time, requested
    ///
    /// Date on which buyer requests goods to be delivered.
    #[serde(rename = "2")]
    _2,
    /// Invoice date/time
    ///
    /// [2376] Date when a Commercial Invoice is issued.
    #[serde(rename = "3")]
    _3,
    /// Order date/time
    ///
    /// [2010] Date when an order is issued.
    #[serde(rename = "4")]
    _4,
    /// Saleable stock demand cover period, expected
    ///
    /// A period of time when saleable stocks are expected to cover demand for a product.
    #[serde(rename = "5")]
    _5,
    /// Moved from location date
    ///
    /// The date an entity moved from a location.
    #[serde(rename = "6")]
    _6,
    /// Effective date/time
    ///
    /// Date and/or time at which specified event or document becomes effective.
    #[serde(rename = "7")]
    _7,
    /// Order received date/time
    ///
    /// Date/time when the purchase order is received by the seller.
    #[serde(rename = "8")]
    _8,
    /// Processing date/time
    ///
    /// Date/time of processing.
    #[serde(rename = "9")]
    _9,
    /// Shipment date/time, requested
    ///
    /// Date on which goods should be shipped or despatched by the supplier.
    #[serde(rename = "10")]
    _10,
    /// Despatch date and or time
    ///
    /// (2170) Date/time on which the goods are or are expected to be despatched or shipped.
    #[serde(rename = "11")]
    _11,
    /// Terms discount due date/time
    ///
    /// Date by which payment should be made if discount terms are to apply.
    #[serde(rename = "12")]
    _12,
    /// Terms net due date
    ///
    /// Date by which payment must be made.
    #[serde(rename = "13")]
    _13,
    /// Payment date/time, deferred
    ///
    /// Date/time when instalments are due.
    #[serde(rename = "14")]
    _14,
    /// Promotion start date/time
    ///
    /// Date/time when promotion activities begin.
    #[serde(rename = "15")]
    _15,
    /// Promotion end date/time
    ///
    /// Date/time when promotion activities end.
    #[serde(rename = "16")]
    _16,
    /// Delivery date/time, estimated
    ///
    /// Date and/or time when the shipper of the goods expects delivery will take place.
    #[serde(rename = "17")]
    _17,
    /// Installation date/time/period
    ///
    ///
    #[serde(rename = "18")]
    _18,
    /// Meat ageing period
    ///
    /// Period of time between slaughter and delivery during which meat is ageing.
    #[serde(rename = "19")]
    _19,
    /// Cheque date/time
    ///
    /// Date/time when cheque is issued.
    #[serde(rename = "20")]
    _20,
    /// Charge back date/time
    ///
    /// Description to be provided.
    #[serde(rename = "21")]
    _21,
    /// Freight bill date/time
    ///
    /// Date/time when freight bill is issued.
    #[serde(rename = "22")]
    _22,
    /// Equipment reconditioning date/time, actual
    ///
    /// Actual date/time of the reconditioning of a piece of equipment.
    #[serde(rename = "23")]
    _23,
    /// Transfer note acceptance date and time
    ///
    /// Date and time when a transfer note (transfer document for transport exclusively using containers as equipment) is recognised as being valid by the carrier.
    #[serde(rename = "24")]
    _24,
    /// Delivery date/time, actual
    ///
    /// Date/time on which goods or consignment are delivered at their destination.
    #[serde(rename = "35")]
    _35,
    /// Expiry date
    ///
    /// Date of expiry of the validity of a referenced document, price information or any other referenced data element with a limited validity period.
    #[serde(rename = "36")]
    _36,
    /// Ship not before date/time
    ///
    /// Goods should not be shipped before given date/time.
    #[serde(rename = "37")]
    _37,
    /// Ship not later than date/time
    ///
    /// Date/time by which the goods should have been shipped.
    #[serde(rename = "38")]
    _38,
    /// Ship week of date
    ///
    /// Date identifying the week during which goods should be shipped.
    #[serde(rename = "39")]
    _39,
    /// Clinical information issue date and/or time
    ///
    /// Date and/or time when clinical information is issued.
    #[serde(rename = "40")]
    _40,
    /// Event duration, expected
    ///
    /// The expected duration of an event.
    #[serde(rename = "41")]
    _41,
    /// Superseded date/time
    ///
    /// Date/time being overlaid by a date given elsewhere.
    #[serde(rename = "42")]
    _42,
    /// Event duration, intended
    ///
    /// The intended duration of an event.
    #[serde(rename = "43")]
    _43,
    /// Availability
    ///
    /// Date/time when received item is available.
    #[serde(rename = "44")]
    _44,
    /// Compilation date and time
    ///
    /// Date and time of the compilation.
    #[serde(rename = "45")]
    _45,
    /// Cancellation date
    ///
    /// Date on which a document or message has been cancelled.
    #[serde(rename = "46")]
    _46,
    /// Statistical time series date
    ///
    /// Date for statistical time series purposes.
    #[serde(rename = "47")]
    _47,
    /// Duration
    ///
    /// Duration.
    #[serde(rename = "48")]
    _48,
    /// Deliver not before and not after dates
    ///
    /// Deliver not before and not after a specific date range.
    #[serde(rename = "49")]
    _49,
    /// Goods receipt date/time
    ///
    /// Date/time upon which the goods were received by a given party.
    #[serde(rename = "50")]
    _50,
    /// Cumulative quantity start date
    ///
    /// First Date for accumulation of delivery quantities.
    #[serde(rename = "51")]
    _51,
    /// Cumulative quantity end date
    ///
    /// Last Date for accumulation of delivery quantities.
    #[serde(rename = "52")]
    _52,
    /// Buyer's local time
    ///
    /// Time at the buyer's location.
    #[serde(rename = "53")]
    _53,
    /// Seller's local time
    ///
    /// Time at the seller's location.
    #[serde(rename = "54")]
    _54,
    /// Confirmed date/time
    ///
    /// Date/time which has been confirmed.
    #[serde(rename = "55")]
    _55,
    /// Original authorisation date and/or time
    ///
    /// Date and/or time when original authorisation was issued.
    #[serde(rename = "56")]
    _56,
    /// Precaution relevant period
    ///
    /// The period when a precaution is relevant.
    #[serde(rename = "57")]
    _57,
    /// Clearance date (Customs)
    ///
    /// (3080) Date on which Customs formalities necessary to allow goods to be exported, to enter home use, or to be placed under another Customs procedure has been accomplished (CCC).
    #[serde(rename = "58")]
    _58,
    /// Inbound movement authorization date
    ///
    /// Inland movement authorization date.
    #[serde(rename = "59")]
    _59,
    /// Engineering change level date
    ///
    /// Date the engineering level of goods is changed.
    #[serde(rename = "60")]
    _60,
    /// Cancel if not delivered by this date
    ///
    ///
    #[serde(rename = "61")]
    _61,
    /// Excluded date
    ///
    /// Date excluded from a period of time.
    #[serde(rename = "62")]
    _62,
    /// Delivery date/time, latest
    ///
    /// Date identifying a point of time after which goods shall not or will not be delivered.
    #[serde(rename = "63")]
    _63,
    /// Delivery date/time, earliest
    ///
    /// Date identifying a point in time before which the goods shall not be delivered.
    #[serde(rename = "64")]
    _64,
    /// Delivery date/time, 1st schedule
    ///
    ///
    #[serde(rename = "65")]
    _65,
    /// Excluded period
    ///
    /// An interval of time excluded from a period of time.
    #[serde(rename = "66")]
    _66,
    /// Delivery date/time, current schedule
    ///
    /// Delivery Date deriving from actual schedule.
    #[serde(rename = "67")]
    _67,
    /// Additional period
    ///
    /// An interval of time added to a period of time.
    #[serde(rename = "68")]
    _68,
    /// Delivery date/time, promised for
    ///
    /// [2138] Date by which, or period within which, the merchandise should be delivered to the buyer, as agreed between the seller and the buyer (generic term).
    #[serde(rename = "69")]
    _69,
    /// Additional date
    ///
    /// Date added to a period of time.
    #[serde(rename = "70")]
    _70,
    /// Delivery date/time, requested for (after and including)
    ///
    /// Delivery is requested to happen after or on given date.
    #[serde(rename = "71")]
    _71,
    /// Delivery date/time, promised for (after and including)
    ///
    /// Delivery might take place earliest at given date.
    #[serde(rename = "72")]
    _72,
    /// Guarantee period
    ///
    /// The period for which the guarantee is or will be granted.
    #[serde(rename = "73")]
    _73,
    /// Delivery date/time, requested for (prior to and including)
    ///
    /// Delivery is requested to happen prior to or including the given date.
    #[serde(rename = "74")]
    _74,
    /// Delivery date/time, promised for (prior to and including)
    ///
    /// Delivery might take place latest at given date.
    #[serde(rename = "75")]
    _75,
    /// Delivery date/time, scheduled for
    ///
    ///
    #[serde(rename = "76")]
    _76,
    /// Specification revision date
    ///
    /// Date of revision to a specification.
    #[serde(rename = "77")]
    _77,
    /// Event date/time/period, actual
    ///
    /// The actual date/time/period an event occurred.
    #[serde(rename = "78")]
    _78,
    /// Shipment date/time, promised for
    ///
    /// Shipment might happen at given date/time.
    #[serde(rename = "79")]
    _79,
    /// Planning end date and/or time, actual
    ///
    /// The actual date and/or time the planning ended.
    #[serde(rename = "80")]
    _80,
    /// Shipment date/time, requested for (after and including)
    ///
    /// Shipment should happen earliest at given date.
    #[serde(rename = "81")]
    _81,
    /// Medicine administration time
    ///
    /// Designated time of day for the administration of medicine.
    #[serde(rename = "82")]
    _82,
    /// Dispensing interval, minimum
    ///
    /// The shortest interval allowed between one dispensing of an item and the next dispensing of the same item.
    #[serde(rename = "83")]
    _83,
    /// Shipment date/time, requested for (prior to and including)
    ///
    /// Shipment should take place latest at given date.
    #[serde(rename = "84")]
    _84,
    /// Shipment date/time, promised for (prior to and including)
    ///
    /// Shipment might take place latest at given date.
    #[serde(rename = "85")]
    _85,
    /// Medication date/time, start
    ///
    /// Date and/or time when medication was started.
    #[serde(rename = "86")]
    _86,
    /// Travel service connection time
    ///
    /// Time elapsing between the arrival of a travel service and the departure of a connecting travel service.
    #[serde(rename = "87")]
    _87,
    /// Summer time, start
    ///
    /// Date/time at which the summer time starts.
    #[serde(rename = "88")]
    _88,
    /// Inquiry date
    ///
    ///
    #[serde(rename = "89")]
    _89,
    /// Report start date
    ///
    ///
    #[serde(rename = "90")]
    _90,
    /// Report end date
    ///
    ///
    #[serde(rename = "91")]
    _91,
    /// Contract effective date
    ///
    /// Date when a contract becomes valid.
    #[serde(rename = "92")]
    _92,
    /// Contract expiry date
    ///
    /// Date when a contract expires.
    #[serde(rename = "93")]
    _93,
    /// Production/manufacture date
    ///
    /// Date on which goods are produced.
    #[serde(rename = "94")]
    _94,
    /// Bill of lading date
    ///
    /// Date as specified on the bill of lading.
    #[serde(rename = "95")]
    _95,
    /// Discharge date/time
    ///
    /// Date/time when goods should, might or have been discharged from the means of transport.
    #[serde(rename = "96")]
    _96,
    /// Transaction creation date
    ///
    ///
    #[serde(rename = "97")]
    _97,
    /// Winter time, start
    ///
    /// Date/time at which the winter time starts.
    #[serde(rename = "98")]
    _98,
    /// Quotation opening date
    ///
    /// The date on which the quotation has been or may be opened.
    #[serde(rename = "99")]
    _99,
    /// Product ageing period before delivery
    ///
    /// Period of time before delivery during which the product is ageing.
    #[serde(rename = "100")]
    _100,
    /// Production date, no schedule established as of
    ///
    /// Date as of there is no valid production schedule.
    #[serde(rename = "101")]
    _101,
    /// Health problem period
    ///
    /// Period of time of health problem.
    #[serde(rename = "102")]
    _102,
    /// Deposit date/time
    ///
    ///
    #[serde(rename = "107")]
    _107,
    /// Postmark date/time
    ///
    ///
    #[serde(rename = "108")]
    _108,
    /// Receive at lockbox date
    ///
    /// The date on which a financial institution, serving as collection agency for a company located in another part of the country, collects an amount of money on behalf of that company.
    #[serde(rename = "109")]
    _109,
    /// Ship date, originally scheduled
    ///
    /// The date on which the shipment of goods was originally scheduled.
    #[serde(rename = "110")]
    _110,
    /// Manifest/ship notice date
    ///
    /// The date of issuance of a manifest or ship notice.
    #[serde(rename = "111")]
    _111,
    /// First interest-bearing date
    ///
    /// The first date from which interest is borne.
    #[serde(rename = "112")]
    _112,
    /// Sample required date
    ///
    /// Date as of a sample has to be available customer defined.
    #[serde(rename = "113")]
    _113,
    /// Tooling required date
    ///
    /// Date as of a tool has to be available customer defined.
    #[serde(rename = "114")]
    _114,
    /// Sample available date
    ///
    /// Date as of a sample will be available seller defined.
    #[serde(rename = "115")]
    _115,
    /// Equipment return period, expected
    ///
    /// Period until which equipment is expected to be hired.
    #[serde(rename = "116")]
    _116,
    /// Delivery date/time, first
    ///
    /// First possible date/time for delivery.
    #[serde(rename = "117")]
    _117,
    /// Cargo booking confirmed date/time
    ///
    /// Date/time at which the cargo booking has been accepted by the carrier.
    #[serde(rename = "118")]
    _118,
    /// Test completion date
    ///
    /// Date when a test has been completed.
    #[serde(rename = "119")]
    _119,
    /// Last interest-bearing date
    ///
    /// The last date from which interest is borne.
    #[serde(rename = "120")]
    _120,
    /// Entry date
    ///
    /// Date of entry.
    #[serde(rename = "121")]
    _121,
    /// Contract completion date
    ///
    /// The date a contract is completed.
    #[serde(rename = "122")]
    _122,
    /// Documentary credit expiry date/time
    ///
    /// The latest date/time for presentation of the documents to the bank where the credit expires.
    #[serde(rename = "123")]
    _123,
    /// Despatch note date
    ///
    /// [2218] Date when a Despatch Note is issued.
    #[serde(rename = "124")]
    _124,
    /// Import licence date
    ///
    /// [2292] Date when Import Licence is issued.
    #[serde(rename = "125")]
    _125,
    /// Contract date
    ///
    /// [2326] Date when a Contract is agreed.
    #[serde(rename = "126")]
    _126,
    /// Previous report date
    ///
    /// Date of the previous report.
    #[serde(rename = "127")]
    _127,
    /// Delivery date/time, last
    ///
    /// Date when the last delivery should be or has been accomplished.
    #[serde(rename = "128")]
    _128,
    /// Exportation date
    ///
    /// Date when imported vessel/merchandise last left the country of export for the country of import.
    #[serde(rename = "129")]
    _129,
    /// Current report date
    ///
    /// Date of the current report.
    #[serde(rename = "130")]
    _130,
    /// Tax point date
    ///
    /// Date on which tax is due or calculated.
    #[serde(rename = "131")]
    _131,
    /// Arrival date/time, estimated
    ///
    /// (2348) Date/time when carrier estimates that a means of transport should arrive at the port of discharge or place of destination.
    #[serde(rename = "132")]
    _132,
    /// Departure date/time, estimated
    ///
    /// Date/time when carrier estimates that a means of transport should depart at the place of departure.
    #[serde(rename = "133")]
    _133,
    /// Rate of exchange date/time
    ///
    /// Date/time on which the exchange rate was fixed.
    #[serde(rename = "134")]
    _134,
    /// Telex date
    ///
    /// Date identifying when a telex message was sent.
    #[serde(rename = "135")]
    _135,
    /// Departure date/time
    ///
    /// [2280] Date (and time) of departure of means of transport.
    #[serde(rename = "136")]
    _136,
    /// Document/message date/time
    ///
    /// (2006) Date/time when a document/message is issued. This may include authentication.
    #[serde(rename = "137")]
    _137,
    /// Payment date
    ///
    /// [2034] Date on which an amount due is made available to the creditor, in accordance with the terms of payment.
    #[serde(rename = "138")]
    _138,
    /// Payment due date
    ///
    /// Date/time at which funds should be made available.
    #[serde(rename = "140")]
    _140,
    /// Presentation date of Goods declaration (Customs)
    ///
    /// [2032] Date on which a Goods declaration is presented or lodged with Customs.
    #[serde(rename = "141")]
    _141,
    /// Labour wage determination date
    ///
    /// The date a labour wage is determined.
    #[serde(rename = "142")]
    _142,
    /// Acceptance date/time of goods
    ///
    /// [2126] Date on which the goods are taken over by the carrier at the place of acceptance (CMR 4).
    #[serde(rename = "143")]
    _143,
    /// Quota date
    ///
    /// Date that the quota applies to.
    #[serde(rename = "144")]
    _144,
    /// Event date
    ///
    /// A date specifying an event.
    #[serde(rename = "145")]
    _145,
    /// Entry date, estimated (Customs)
    ///
    /// Date on which the official date of Customs entry is anticipated.
    #[serde(rename = "146")]
    _146,
    /// Expiry date of export licence
    ///
    /// [2078] Date of expiry of the validity of an Export Licence.
    #[serde(rename = "147")]
    _147,
    /// Acceptance date of Goods declaration (Customs)
    ///
    /// [2036] Date on which a Goods declaration is accepted by Customs in accordance with Customs legislation.
    #[serde(rename = "148")]
    _148,
    /// Invoice date, required
    ///
    /// Date required for invoice issue.
    #[serde(rename = "149")]
    _149,
    /// Declaration/presentation date
    ///
    /// Date when item has been or has to be declared/presented.
    #[serde(rename = "150")]
    _150,
    /// Importation date
    ///
    /// Date on which goods are imported, as determined by the governing Customs administration.
    #[serde(rename = "151")]
    _151,
    /// Exportation date for textiles
    ///
    /// Date when imported textiles last left the country of origin for the country of importation.
    #[serde(rename = "152")]
    _152,
    /// Cancellation date/time, latest
    ///
    /// The latest date/time on which cancellation of the payment order may be requested.
    #[serde(rename = "153")]
    _153,
    /// Acceptance date of document
    ///
    /// The date on which a document was accepted.
    #[serde(rename = "154")]
    _154,
    /// Accounting period start date
    ///
    ///
    #[serde(rename = "155")]
    _155,
    /// Accounting period end date
    ///
    ///
    #[serde(rename = "156")]
    _156,
    /// Validity start date
    ///
    ///
    #[serde(rename = "157")]
    _157,
    /// Horizon start date
    ///
    /// The first date of a period forming a horizon.
    #[serde(rename = "158")]
    _158,
    /// Horizon end date
    ///
    /// The last date of a period forming a horizon.
    #[serde(rename = "159")]
    _159,
    /// Authorization date
    ///
    /// Date when an authorization was given.
    #[serde(rename = "160")]
    _160,
    /// Release date of customer
    ///
    /// Date the customer authorised the goods' release.
    #[serde(rename = "161")]
    _161,
    /// Release date of supplier
    ///
    /// Date when the supplier released goods.
    #[serde(rename = "162")]
    _162,
    /// Processing start date/time
    ///
    /// Date/Time when a specific process starts.
    #[serde(rename = "163")]
    _163,
    /// Processing end date/time
    ///
    /// Date/Time when a specific process ends.
    #[serde(rename = "164")]
    _164,
    /// Tax period start date
    ///
    /// Date when a tax period begins.
    #[serde(rename = "165")]
    _165,
    /// Tax period end date
    ///
    /// Date when a tax period ends.
    #[serde(rename = "166")]
    _166,
    /// Charge period start date
    ///
    /// The charge period's first date.
    #[serde(rename = "167")]
    _167,
    /// Charge period end date
    ///
    /// The charge period's last date.
    #[serde(rename = "168")]
    _168,
    /// Lead time
    ///
    /// Time required between order entry till earliest goods delivery.
    #[serde(rename = "169")]
    _169,
    /// Settlement due date
    ///
    /// More generic than 'payment due date' and therefore more apt for reinsurance/insurance business.
    #[serde(rename = "170")]
    _170,
    /// Reference date/time
    ///
    /// Date/time on which the reference was issued.
    #[serde(rename = "171")]
    _171,
    /// Hired from date
    ///
    /// Date from which an item has been or will be hired.
    #[serde(rename = "172")]
    _172,
    /// Hired until date
    ///
    /// Date until which an item has been or will be hired.
    #[serde(rename = "173")]
    _173,
    /// Advise after date/time
    ///
    /// The information must be advised after the date/time indicated.
    #[serde(rename = "174")]
    _174,
    /// Advise before date/time
    ///
    /// The information must be advised before the date/time indicated.
    #[serde(rename = "175")]
    _175,
    /// Advise completed date/time
    ///
    /// The advise has been completed at the date indicated.
    #[serde(rename = "176")]
    _176,
    /// Advise on date/time
    ///
    /// The information must be advised on the date/time indicated.
    #[serde(rename = "177")]
    _177,
    /// Arrival date/time, actual
    ///
    /// [2106] Date (and time) of arrival of means of transport.
    #[serde(rename = "178")]
    _178,
    /// Booking date/time
    ///
    /// Date at which the booking was made.
    #[serde(rename = "179")]
    _179,
    /// Closing date/time
    ///
    /// Final date for delivering cargo to a liner ship.
    #[serde(rename = "180")]
    _180,
    /// Positioning date/time of equipment
    ///
    /// Date/time when equipment is positioned.
    #[serde(rename = "181")]
    _181,
    /// Issue date
    ///
    /// Date when a document/message has been or will be issued.
    #[serde(rename = "182")]
    _182,
    /// Date, as at
    ///
    /// Date related to a given context.
    #[serde(rename = "183")]
    _183,
    /// Notification date/time
    ///
    /// Date/time of notification.
    #[serde(rename = "184")]
    _184,
    /// Commenced tank cleaning date/time
    ///
    /// The date/and or time tank cleaning was started.
    #[serde(rename = "185")]
    _185,
    /// Departure date/time, actual
    ///
    /// (2280) Date (and time) of departure of means of transport.
    #[serde(rename = "186")]
    _186,
    /// Authentication date/time of document
    ///
    /// Date/time when the document is signed or otherwise authenticated.
    #[serde(rename = "187")]
    _187,
    /// Previous current account date
    ///
    /// Date of the previous current account.
    #[serde(rename = "188")]
    _188,
    /// Departure date/time, scheduled
    ///
    /// Date (and time) of scheduled departure of means of transport.
    #[serde(rename = "189")]
    _189,
    /// Transhipment date/time
    ///
    /// Date and time of the transfer of the goods from one means of transport to another.
    #[serde(rename = "190")]
    _190,
    /// Delivery date/time, expected
    ///
    /// Date/time on which goods are expected to be delivered.
    #[serde(rename = "191")]
    _191,
    /// Expiration date/time of customs document
    ///
    /// Date on which validity of a customs document expires.
    #[serde(rename = "192")]
    _192,
    /// Execution date
    ///
    /// The date when ordered bank initiated the transaction.
    #[serde(rename = "193")]
    _193,
    /// Start date/time
    ///
    /// Date/time on which a period starts.
    #[serde(rename = "194")]
    _194,
    /// Expiry date of import licence
    ///
    /// [2272] Date of expiry of the validity of an Import Licence.
    #[serde(rename = "195")]
    _195,
    /// Departure date/time, earliest
    ///
    /// Date/time of earliest departure of means of transport.
    #[serde(rename = "196")]
    _196,
    /// Lay-time first day
    ///
    /// First of a number of days allowed in a charter party of the loading and discharging of cargo.
    #[serde(rename = "197")]
    _197,
    /// Lay-time last day
    ///
    /// Last of a number of days allowed in a charter party for the loading and discharging of cargo.
    #[serde(rename = "198")]
    _198,
    /// Positioning date/time of goods
    ///
    /// The date and/or time the goods have to be or have been positioned.
    #[serde(rename = "199")]
    _199,
    /// Pick-up/collection date/time of cargo
    ///
    /// Date/time at which the cargo is picked up.
    #[serde(rename = "200")]
    _200,
    /// Pick-up date/time of equipment
    ///
    /// Date/time at which the equipment is picked up.
    #[serde(rename = "201")]
    _201,
    /// Posting date
    ///
    /// The date when an entry is posted to an account.
    #[serde(rename = "202")]
    _202,
    /// Execution date/time, requested
    ///
    /// The date/time on which the ordered bank is requested to initiate the payment order, as specified by the originator (e.g. the date of the debit).
    #[serde(rename = "203")]
    _203,
    /// Release date (Customs)
    ///
    /// Date on which Customs releases merchandise to the carrier or importer.
    #[serde(rename = "204")]
    _204,
    /// Settlement date
    ///
    /// Date for settlement of financial transaction e.g. foreign exchange securities.
    #[serde(rename = "205")]
    _205,
    /// End date/time
    ///
    /// Date/time on which a period (from - to) ends.
    #[serde(rename = "206")]
    _206,
    /// Commenced pumping ballast date/time
    ///
    /// Date/time on which the intake of materials to be carried to improve the trim and the stability of the means of transport, was commenced.
    #[serde(rename = "207")]
    _207,
    /// Departure date/time, ultimate
    ///
    /// Date/time at which a means of transport has to depart ultimately.
    #[serde(rename = "208")]
    _208,
    /// Value date
    ///
    /// Date on which the funds are at the disposal of the beneficiary or cease to be at the disposal of the ordering customer.
    #[serde(rename = "209")]
    _209,
    /// Reinsurance current account period
    ///
    /// Description to be provided.
    #[serde(rename = "210")]
    _210,
    /// 360/30
    ///
    /// Calculation is based on year of 360 days, month of 30 days.
    #[serde(rename = "211")]
    _211,
    /// 360/28-31
    ///
    /// Calculation is based on year of 360 days, month of 28-31 days.
    #[serde(rename = "212")]
    _212,
    /// 365-6/30
    ///
    /// Calculation is based on year of 365-6 days, month of 30 days.
    #[serde(rename = "213")]
    _213,
    /// 365-6/28-31
    ///
    /// Calculation is based on year of 365-6 days, month of 28- 31 days.
    #[serde(rename = "214")]
    _214,
    /// 365/28-31
    ///
    /// Calculation is based on year of 365 days, month of 28-31 days.
    #[serde(rename = "215")]
    _215,
    /// 365/30
    ///
    /// Calculation is based on year of 365 days, month of 30 days.
    #[serde(rename = "216")]
    _216,
    /// From date of award to latest delivery
    ///
    /// Lead time to determine the latest date a delivery can be made based on the date an award is made.
    #[serde(rename = "217")]
    _217,
    /// Authentication/validation date/time
    ///
    ///
    #[serde(rename = "218")]
    _218,
    /// Crossborder date/time
    ///
    /// Date/time at which goods are transferred across a country border.
    #[serde(rename = "219")]
    _219,
    /// Interest period
    ///
    /// Number of days used for the calculation of interests.
    #[serde(rename = "221")]
    _221,
    /// Presentation date, latest
    ///
    /// Latest date for presentation of a document.
    #[serde(rename = "222")]
    _222,
    /// Delivery date/time, deferred
    ///
    /// New date and time of delivery calculated on basis of a consignee's requirement (chargeable).
    #[serde(rename = "223")]
    _223,
    /// Permit to admit date
    ///
    /// Date on which permission was granted to move merchandise into a bonded warehouse or free trade zone.
    #[serde(rename = "224")]
    _224,
    /// Certification of weight date/time
    ///
    /// Date/time at which the carrier proceeds to the weighting of the goods.
    #[serde(rename = "225")]
    _225,
    /// Discrepancy date/time
    ///
    /// Date/time at which a discrepancy has been found.
    #[serde(rename = "226")]
    _226,
    /// Beneficiary's banks due date
    ///
    /// Date on which funds should be made available to the beneficiary's bank.
    #[serde(rename = "227")]
    _227,
    /// Debit value date, requested
    ///
    /// Date on which the account owner wants the debit value to his account.
    #[serde(rename = "228")]
    _228,
    /// Hoses connected date/time
    ///
    /// The date and/or time hoses were connected.
    #[serde(rename = "229")]
    _229,
    /// Hoses disconnected date/time
    ///
    /// The date and/or time hoses were disconnected.
    #[serde(rename = "230")]
    _230,
    /// Arrival date/time, earliest
    ///
    /// Date/time of earliest arrival of means of transport.
    #[serde(rename = "231")]
    _231,
    /// Arrival date/time, scheduled
    ///
    /// Date (and time) of scheduled arrival of means of transport.
    #[serde(rename = "232")]
    _232,
    /// Arrival date/time, ultimate
    ///
    /// Date (and time) of ultimate arrival of means of transport.
    #[serde(rename = "233")]
    _233,
    /// Collection date/time, earliest
    ///
    /// The transport order may be issued before the goods are ready for picking up. This date/time indicates from when on the carrier can have access to the consignment.
    #[serde(rename = "234")]
    _234,
    /// Collection date/time, latest
    ///
    /// In relation with the arrangements agreed between buyer and seller or between sender and main transport it may be necessary to specify the latest collection date/time.
    #[serde(rename = "235")]
    _235,
    /// Completed pumping ballast date/time
    ///
    /// Date/time at which the intake of materials, to be carried to improve the trim and the stability of the means of transport, was completed.
    #[serde(rename = "236")]
    _236,
    /// Completed tank cleaning date/time
    ///
    /// The date and/or time tank cleaning was completed.
    #[serde(rename = "237")]
    _237,
    /// Tanks accepted date/time
    ///
    /// The date and/or time the tanks are to be or have been accepted.
    #[serde(rename = "238")]
    _238,
    /// Tanks inspected date/time
    ///
    /// The date and/or time the tanks are to be or have been inspected.
    #[serde(rename = "239")]
    _239,
    /// Reinsurance accounting period
    ///
    /// To identify a reinsurance account period via start and end dates.
    #[serde(rename = "240")]
    _240,
    /// From date of award to earliest delivery
    ///
    /// Lead time to determine the earliest date a delivery can be made based on the date an award is made.
    #[serde(rename = "241")]
    _241,
    /// Preparation date/time of document
    ///
    /// Date and/or time that the document was prepared.
    #[serde(rename = "242")]
    _242,
    /// Transmission date/time of document
    ///
    ///
    #[serde(rename = "243")]
    _243,
    /// Settlement date, planned
    ///
    ///
    #[serde(rename = "244")]
    _244,
    /// Underwriting year
    ///
    /// Year in which the treaty was commenced.
    #[serde(rename = "245")]
    _245,
    /// Accounting year
    ///
    /// Year considered for accounting of the treaty or portion of the treaty.
    #[serde(rename = "246")]
    _246,
    /// Year of occurrence
    ///
    /// Year in which a specific event (e.g. a loss) took place.
    #[serde(rename = "247")]
    _247,
    /// Loss
    ///
    /// Date, time, period on which a referenced loss occurred.
    #[serde(rename = "248")]
    _248,
    /// Cash call date
    ///
    /// Date on which a cash call was made for a loss suffered and covered.
    #[serde(rename = "249")]
    _249,
    /// Re-exportation date
    ///
    /// Date of re-exportation.
    #[serde(rename = "250")]
    _250,
    /// Re-importation date
    ///
    /// Date of re-importation.
    #[serde(rename = "251")]
    _251,
    /// Arrival date/time at initial port
    ///
    /// Date/time that the conveyance arrives at the initial port in the country of destination.
    #[serde(rename = "252")]
    _252,
    /// Departure date/time from last port of call
    ///
    /// Date/time that conveyance departed from the last foreign port of call.
    #[serde(rename = "253")]
    _253,
    /// Registration date of previous Customs declaration
    ///
    /// Registration date of the Customs declaration for the previous Customs procedure either in the same or another country.
    #[serde(rename = "254")]
    _254,
    /// Availability due date
    ///
    /// Date when ordered items should be available at a specified location.
    #[serde(rename = "255")]
    _255,
    /// From date of award to completion
    ///
    /// Lead time to determine the completion date of an effort based on the date an award is made.
    #[serde(rename = "256")]
    _256,
    /// Calculation date/time/period
    ///
    /// The date/time/period on which a calculation will take, or has taken, place.
    #[serde(rename = "257")]
    _257,
    /// Guarantee date
    ///
    /// Date when a guarantee is placed.
    #[serde(rename = "258")]
    _258,
    /// Conveyance registration date
    ///
    /// Date when a vessel, vehicle or other means of transport was registered by a competent authority.
    #[serde(rename = "259")]
    _259,
    /// Valuation date (Customs)
    ///
    /// Date when Customs valuation was made.
    #[serde(rename = "260")]
    _260,
    /// Release date/time
    ///
    /// Date/time assigned to identify the release of a set of rules, conditions, conventions, productions, etc.
    #[serde(rename = "261")]
    _261,
    /// Closure date/time/period
    ///
    /// Date/time/period when an enterprise is closed.
    #[serde(rename = "262")]
    _262,
    /// Invoicing period
    ///
    /// Period for which an invoice is issued.
    #[serde(rename = "263")]
    _263,
    /// Release frequency
    ///
    /// Frequency of a release.
    #[serde(rename = "264")]
    _264,
    /// Due date
    ///
    ///
    #[serde(rename = "265")]
    _265,
    /// Validation date
    ///
    ///
    #[serde(rename = "266")]
    _266,
    /// Rate/price date/time
    ///
    /// Date/time on which a rate/price is determined.
    #[serde(rename = "267")]
    _267,
    /// Transit time/limits
    ///
    /// Description to be provided.
    #[serde(rename = "268")]
    _268,
    /// Ship during date
    ///
    /// The date identifying the period during or in which the goods should be shipped.
    #[serde(rename = "270")]
    _270,
    /// Ship on or about date
    ///
    /// Date on or about which goods should be shipped.
    #[serde(rename = "271")]
    _271,
    /// Documentary credit presentation period
    ///
    /// The specification of the period of time, expressed in number of days, after the date of issuance of the transport document(s) within which the documents must be presented.
    #[serde(rename = "272")]
    _272,
    /// Validity period
    ///
    /// Dates (from/to)/period referenced documents are valid.
    #[serde(rename = "273")]
    _273,
    /// From date of order receipt to sample ready
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "274")]
    _274,
    /// From date of tooling authorization to sample ready
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "275")]
    _275,
    /// From date of receipt of tooling aids to sample ready
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "276")]
    _276,
    /// From date of sample approval to first product shipment
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "277")]
    _277,
    /// From date of order receipt to shipment
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "278")]
    _278,
    /// From date of order receipt to delivery
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "279")]
    _279,
    /// From last booked order to delivery
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "280")]
    _280,
    /// Date of order lead time
    ///
    /// Lead time is referenced to the date of order.
    #[serde(rename = "281")]
    _281,
    /// Confirmation date lead time
    ///
    /// Lead time is referenced to the date of confirmation.
    #[serde(rename = "282")]
    _282,
    /// Arrival date/time of transport lead time
    ///
    /// Lead time is referenced to the date a transport will arrive or has arrived.
    #[serde(rename = "283")]
    _283,
    /// Before inventory is replenished based on stock check lead time
    ///
    /// Lead time is the defined timespan.
    #[serde(rename = "284")]
    _284,
    /// Invitation to tender date/time
    ///
    /// Date/time on which the invitation to tender has been made available to relevant parties.
    #[serde(rename = "285")]
    _285,
    /// Tender submission date/time
    ///
    /// Date/time on which the tender was submitted.
    #[serde(rename = "286")]
    _286,
    /// Contract award date/time
    ///
    /// Date/time on which the contract is awarded to a tenderer.
    #[serde(rename = "287")]
    _287,
    /// Price base date/time
    ///
    /// Base date/time of prices.
    #[serde(rename = "288")]
    _288,
    /// Interest rate validity period
    ///
    /// Validity period of the interest rate.
    #[serde(rename = "289")]
    _289,
    /// Contractual start date/time
    ///
    /// Date/time on which activities stated in the contract must start.
    #[serde(rename = "290")]
    _290,
    /// Start date/time, planned
    ///
    ///
    #[serde(rename = "291")]
    _291,
    /// Works completion date/time, planned
    ///
    ///
    #[serde(rename = "292")]
    _292,
    /// Works completion date/time, actual
    ///
    ///
    #[serde(rename = "293")]
    _293,
    /// Hand over date/time, planned
    ///
    /// Date/time on which hand over (i.e. the transfer of responsibility for an object or activity such as documentation, system etc. from one party to another) is planned to take place.
    #[serde(rename = "294")]
    _294,
    /// Hand over date/time, actual
    ///
    /// Date/time on which hand over (i.e. the transfer of responsibility for an object or activity such as documentation, system etc. from one party to another) actually takes place.
    #[serde(rename = "295")]
    _295,
    /// Retention release date/time
    ///
    /// Date/time on which the retention is released.
    #[serde(rename = "296")]
    _296,
    /// Retention release date/time, partial
    ///
    /// Date/time on which the retention is partially released.
    #[serde(rename = "297")]
    _297,
    /// Escalation start date
    ///
    /// Value date of the indexes appearing as denominators in an escalation formula.
    #[serde(rename = "298")]
    _298,
    /// Price adjustment start date
    ///
    /// Value date of the indexes appearing as denominators in a price adjustment formula.
    #[serde(rename = "299")]
    _299,
    /// Price adjustment limit date
    ///
    /// Limit value date of indexes used as numerators in a price adjustment formula.
    #[serde(rename = "300")]
    _300,
    /// Value date of index
    ///
    /// Date of validity of index values.
    #[serde(rename = "301")]
    _301,
    /// Publication date
    ///
    ///
    #[serde(rename = "302")]
    _302,
    /// Escalation date
    ///
    /// Value date of indexes appearing as numerators in an escalation formula.
    #[serde(rename = "303")]
    _303,
    /// Price adjustment date
    ///
    /// Value date of indexes appearing as numerators in a price adjustment formula.
    #[serde(rename = "304")]
    _304,
    /// Latest price adjustment date
    ///
    /// Date on which the latest price adjustment took place.
    #[serde(rename = "305")]
    _305,
    /// Work period
    ///
    /// Period of execution of works.
    #[serde(rename = "306")]
    _306,
    /// Payment instruction date/time
    ///
    /// Date/time on which a payment instruction was given.
    #[serde(rename = "307")]
    _307,
    /// Payment valuation presentation date/time
    ///
    /// Date/time on which the payment valuation is presented.
    #[serde(rename = "308")]
    _308,
    /// Blanks value date
    ///
    /// The date on which the funds are at the disposal of the receiving bank.
    #[serde(rename = "309")]
    _309,
    /// Received date/time
    ///
    /// Date/time of receipt.
    #[serde(rename = "310")]
    _310,
    /// On
    ///
    /// Fixed maturity day for deferred payment or time draft(s).
    #[serde(rename = "311")]
    _311,
    /// Ship not before and not after date/time
    ///
    /// Shipment(s) of goods is/are to be made not before the first specified date/time and not after the second specified date/time.
    #[serde(rename = "312")]
    _312,
    /// Order to proceed date
    ///
    /// Issue date of an instruction to start work.
    #[serde(rename = "313")]
    _313,
    /// Planned duration of works
    ///
    ///
    #[serde(rename = "314")]
    _314,
    /// Agreement to pay date
    ///
    /// Date on which the debtor agreed to pay.
    #[serde(rename = "315")]
    _315,
    /// Valuation date/time
    ///
    /// Date/time of valuation.
    #[serde(rename = "316")]
    _316,
    /// Reply date
    ///
    ///
    #[serde(rename = "317")]
    _317,
    /// Request date
    ///
    /// The date on which something was asked for.
    #[serde(rename = "318")]
    _318,
    /// Customer value date
    ///
    /// Date at which funds are taken into account for interest calculation (in debit or credit).
    #[serde(rename = "319")]
    _319,
    /// Declaration reference period
    ///
    /// Reference period of a set of items reported on the same declaration.
    #[serde(rename = "320")]
    _320,
    /// Promotion date/period
    ///
    /// Date/period relevant for specific promotion activities.
    #[serde(rename = "321")]
    _321,
    /// Accounting period
    ///
    /// Self-explanatory.
    #[serde(rename = "322")]
    _322,
    /// Horizon period
    ///
    /// Period forming a (planning) horizon.
    #[serde(rename = "323")]
    _323,
    /// Processing date/period
    ///
    /// Date/period a specific process happened/will happen.
    #[serde(rename = "324")]
    _324,
    /// Tax period
    ///
    /// Period a tax rate/tax amount etc. is applicable.
    #[serde(rename = "325")]
    _325,
    /// Charge period
    ///
    /// Period a specified charge is valid for.
    #[serde(rename = "326")]
    _326,
    /// Instalment payment due date
    ///
    /// Self-explanatory.
    #[serde(rename = "327")]
    _327,
    /// Payroll deduction date/time
    ///
    /// Date/time of a monetary deduction made from the salary of a person on a payroll.
    #[serde(rename = "328")]
    _328,
    /// Birth date/time
    ///
    /// Date/time when a person was born.
    #[serde(rename = "329")]
    _329,
    /// Joined employer date
    ///
    /// Date when a person joins an employer.
    #[serde(rename = "330")]
    _330,
    /// Contributions ceasing date/time
    ///
    /// Date/time when contributions cease.
    #[serde(rename = "331")]
    _331,
    /// Contribution period end date/time
    ///
    /// Date/time when a contribution period ends.
    #[serde(rename = "332")]
    _332,
    /// Part-time working change date/time
    ///
    /// Date/time when the proportion of part-time work changes.
    #[serde(rename = "333")]
    _333,
    /// Status change date/time
    ///
    /// Date/time when a status changes.
    #[serde(rename = "334")]
    _334,
    /// Contribution period start date/time
    ///
    /// Date/time when a contribution period commences.
    #[serde(rename = "335")]
    _335,
    /// Salary change effective date
    ///
    /// Date when a change in salary becomes effective.
    #[serde(rename = "336")]
    _336,
    /// Left employer date
    ///
    /// Date when a person leaves an employer.
    #[serde(rename = "337")]
    _337,
    /// Benefit change date/time
    ///
    /// Date/time when a benefit provided by a service provider is changed.
    #[serde(rename = "338")]
    _338,
    /// Category change date/time
    ///
    /// Date/time when a change of category is made.
    #[serde(rename = "339")]
    _339,
    /// Joined fund date/time
    ///
    /// Date/time when a person joins a fund.
    #[serde(rename = "340")]
    _340,
    /// Waiting time
    ///
    /// The period of time between the moment at which one wants an activity to begin and the moment at which this activity can actually begin.
    #[serde(rename = "341")]
    _341,
    /// On-board date
    ///
    /// The date goods have been loaded on board of a conveyance.
    #[serde(rename = "342")]
    _342,
    /// Date/time of discount termination
    ///
    /// Date/time when the deduction from an amount comes to an end.
    #[serde(rename = "343")]
    _343,
    /// Date/time of interest due
    ///
    /// Date/time when the interest has to be paid.
    #[serde(rename = "344")]
    _344,
    /// Days of operation
    ///
    /// Week days of operation.
    #[serde(rename = "345")]
    _345,
    /// Latest check-in time
    ///
    /// Latest time of check-in.
    #[serde(rename = "346")]
    _346,
    /// Slaughtering start date
    ///
    /// Date on which slaughtering commenced.
    #[serde(rename = "347")]
    _347,
    /// Packing start date
    ///
    /// Date on which packing commenced.
    #[serde(rename = "348")]
    _348,
    /// Packing end date
    ///
    /// Date on which packing completed.
    #[serde(rename = "349")]
    _349,
    /// Test start date
    ///
    /// Date when a test has been started.
    #[serde(rename = "350")]
    _350,
    /// Inspection date
    ///
    /// Date of inspection.
    #[serde(rename = "351")]
    _351,
    /// Slaughtering end date
    ///
    /// Date on which slaughtering completed.
    #[serde(rename = "352")]
    _352,
    /// Accounting transaction date
    ///
    /// Date to which an accounting transaction refers.
    #[serde(rename = "353")]
    _353,
    /// Activity period date range
    ///
    /// A specific date range associated with an activity.
    #[serde(rename = "354")]
    _354,
    /// Contractual delivery date
    ///
    /// The date of delivery contractually agreed between parties.
    #[serde(rename = "355")]
    _355,
    /// Sales date, and or time, and or period
    ///
    /// The date, and or time, and or period on which a sale took place.
    #[serde(rename = "356")]
    _356,
    /// Cancel if not published by this date
    ///
    /// Cancel if not published by this date.
    #[serde(rename = "357")]
    _357,
    /// Scheduled for delivery on or after
    ///
    /// Scheduled for delivery on or after the specified date, and or time.
    #[serde(rename = "358")]
    _358,
    /// Scheduled for delivery on or before
    ///
    /// Scheduled for delivery on or before specified date and or time.
    #[serde(rename = "359")]
    _359,
    /// Sell by date
    ///
    /// The date by which a product should be sold.
    #[serde(rename = "360")]
    _360,
    /// Best before date
    ///
    /// The best before date.
    #[serde(rename = "361")]
    _361,
    /// End availability date
    ///
    /// The end date of availability.
    #[serde(rename = "362")]
    _362,
    /// Total shelf life period
    ///
    /// A period indicating the total shelf life of a product.
    #[serde(rename = "363")]
    _363,
    /// Minimum shelf life remaining at time of despatch period
    ///
    /// Period indicating the minimum shelf life remaining for a product at the time of leaving the supplier.
    #[serde(rename = "364")]
    _364,
    /// Packaging date
    ///
    /// The date on which the packaging of a product took place.
    #[serde(rename = "365")]
    _365,
    /// Inventory report date
    ///
    /// Date on which a inventory report is made.
    #[serde(rename = "366")]
    _366,
    /// Previous meter reading date
    ///
    /// Date on which the previous reading of a meter took place.
    #[serde(rename = "367")]
    _367,
    /// Latest meter reading date
    ///
    /// Date on which the latest reading of a meter took place.
    #[serde(rename = "368")]
    _368,
    /// Date and or time of handling, estimated
    ///
    /// The date and or time when the handling action is estimated to take place.
    #[serde(rename = "369")]
    _369,
    /// Date when container equipment becomes domestic
    ///
    /// The date on which foreign-built container equipment has entered into the commerce of another country and has become domestic equipment.
    #[serde(rename = "370")]
    _370,
    /// Hydrotest date
    ///
    /// The date equipment has been hydrotested.
    #[serde(rename = "371")]
    _371,
    /// Equipment pre-trip date
    ///
    /// The date on which equipment is pre-tripped.
    #[serde(rename = "372")]
    _372,
    /// Mooring, date and time
    ///
    /// Date and time of mooring.
    #[serde(rename = "373")]
    _373,
    /// Road fund tax expiry date
    ///
    /// The date of expiry of the road fund tax.
    #[serde(rename = "374")]
    _374,
    /// Date of first registration
    ///
    /// Date of first registration.
    #[serde(rename = "375")]
    _375,
    /// Biannual terminal inspection date
    ///
    /// The date on which a biannual inspection of a terminal has taken or will take place.
    #[serde(rename = "376")]
    _376,
    /// Federal HighWay Administration (FHWA) inspection date
    ///
    /// The date on which container equipment is to be or has been inspected in accordance with the requirements of the U.S. Federal Highway Administration.
    #[serde(rename = "377")]
    _377,
    /// Container Safety Convention (CSC) inspection date
    ///
    /// The date on which container equipment is to be or has been inspected as per the Container Safety Convention (CSC).
    #[serde(rename = "378")]
    _378,
    /// Periodic inspection date
    ///
    /// The date on which a periodic inspection has to take place.
    #[serde(rename = "379")]
    _379,
    /// Drawing revision date
    ///
    /// Date the drawing revision has been allocated to a design.
    #[serde(rename = "380")]
    _380,
    /// Product lifespan at time of production
    ///
    /// The total lifespan of a product at the time of its production.
    #[serde(rename = "381")]
    _381,
    /// Earliest sale date
    ///
    /// The earliest date on which the product may be made available for sale.
    #[serde(rename = "382")]
    _382,
    /// Cancel if not shipped by this date
    ///
    /// Cancel the order if goods not shipped by this date.
    #[serde(rename = "383")]
    _383,
    /// Previous invoice date
    ///
    /// Indicates the date which was allocated to a previous invoice.
    #[serde(rename = "384")]
    _384,
    /// Repair turnaround time
    ///
    /// Provides the period of time necessary to turnaround a given repair.
    #[serde(rename = "387")]
    _387,
    /// Order amendment binding date
    ///
    /// The date when an order amendment becomes binding for both parties.
    #[serde(rename = "388")]
    _388,
    /// Cure time
    ///
    /// Specifies the length of time that an article was or should be cured.
    #[serde(rename = "389")]
    _389,
    /// From date of award to delivery
    ///
    /// Lead time to determine the delivery date based on the date an award is made.
    #[serde(rename = "390")]
    _390,
    /// From date of receipt of item to approval
    ///
    /// Lead time to determine the date an item will be approved based on the date the item was received.
    #[serde(rename = "391")]
    _391,
    /// Equipment collection or pick-up date/time, earliest
    ///
    /// Date/time on which equipment can be picked up at the earliest.
    #[serde(rename = "392")]
    _392,
    /// Equipment collection or pick-up date/time, planned
    ///
    /// Date/time on which equipment can be picked up, either full or empty, according to a planning.
    #[serde(rename = "393")]
    _393,
    /// Equipment positioning date/time, actual
    ///
    /// Date/time on which equipment was actually positioned (delivered).
    #[serde(rename = "394")]
    _394,
    /// Equipment positioning date/time, estimated
    ///
    /// Date/time on which equipment is estimated to be positioned (delivered).
    #[serde(rename = "395")]
    _395,
    /// Equipment positioning date/time, requested
    ///
    /// Date/time on which equipment is requested to be positioned (delivered).
    #[serde(rename = "396")]
    _396,
    /// Equipment positioning date/time, ultimate
    ///
    /// Date/time on which equipment should be positioned (delivered) at the latest.
    #[serde(rename = "397")]
    _397,
    /// Goods collection or pick-up date/time, planned
    ///
    /// Date/time at which goods can be picked up, according to a planning.
    #[serde(rename = "398")]
    _398,
    /// Goods positioning date/time, expected
    ///
    /// Date/time on which goods are expected to be positioned.
    #[serde(rename = "399")]
    _399,
    /// Cargo release date/time, ultimate
    ///
    /// Ultimate date/time at which goods or equipment should be released.
    #[serde(rename = "400")]
    _400,
    /// Container Safety Convention (CSC) plate expiration date
    ///
    /// Date on which the validity of a Container Safety Convention (CSC) plate expires.
    #[serde(rename = "401")]
    _401,
    /// Document received date/time
    ///
    /// Date/time on which the document was actually received.
    #[serde(rename = "402")]
    _402,
    /// Discharge date/time, actual
    ///
    /// Date/time when the specified goods or transport equipment has or have been discharged from the means of transport.
    #[serde(rename = "403")]
    _403,
    /// Loading date/time, actual
    ///
    /// Date/time when the specified goods or transport equipment has or have been loaded in or on the means of transport.
    #[serde(rename = "404")]
    _404,
    /// Equipment collection or pick-up date/time, actual
    ///
    /// Date/time on which the equipment was actually collected.
    #[serde(rename = "405")]
    _405,
    /// Goods positioning date/time, planned
    ///
    /// The date/time on which the goods will be positioned according to a planning.
    #[serde(rename = "406")]
    _406,
    /// Document requested date/time
    ///
    /// Date/time on which the document is requested by a party.
    #[serde(rename = "407")]
    _407,
    /// Expected container hire from date/time
    ///
    /// Estimated date and time when the containers are expected to go on-hire.
    #[serde(rename = "408")]
    _408,
    /// Order completion date/time, ultimate
    ///
    /// Date/time on which the order should be completed at the latest.
    #[serde(rename = "409")]
    _409,
    /// Equipment repair ready date/time, ultimate
    ///
    /// Ultimate date/time on which a piece of equipment must be repaired.
    #[serde(rename = "410")]
    _410,
    /// Container stuffing date/time, ultimate
    ///
    /// Date/time on which the container stuffing should be completed at the latest.
    #[serde(rename = "411")]
    _411,
    /// Container stripping date/time, ultimate
    ///
    /// Date/time on which the container stripping should be completed at the latest.
    #[serde(rename = "412")]
    _412,
    /// Discharge and loading completed date/time
    ///
    /// Date/time when all discharge and loading operations on the transport means have been completed.
    #[serde(rename = "413")]
    _413,
    /// Equipment stock check date/time
    ///
    /// Date/time on which equipment has been ascertained as being in stock.
    #[serde(rename = "414")]
    _414,
    /// Activity reporting date
    ///
    /// The date applicable to the activity being reported.
    #[serde(rename = "415")]
    _415,
    /// Submission date
    ///
    /// The date of a submission.
    #[serde(rename = "416")]
    _416,
    /// Previous booking date/time
    ///
    /// Date/time at which the previous booking was made.
    #[serde(rename = "417")]
    _417,
    /// Minimum shelf life remaining at time of receipt
    ///
    /// The minimum shelf life remaining at the time of receipt.
    #[serde(rename = "418")]
    _418,
    /// Forecast period
    ///
    /// A period for which a forecast applies.
    #[serde(rename = "419")]
    _419,
    /// Unloaded, date and time
    ///
    /// To report the date and time that an unloading action occurred.
    #[serde(rename = "420")]
    _420,
    /// Estimated acceptance date
    ///
    /// To estimate the date of acceptance.
    #[serde(rename = "421")]
    _421,
    /// Documentary credit issue date
    ///
    /// The date the documentary credit has been issued.
    #[serde(rename = "422")]
    _422,
    /// First date of ordering
    ///
    /// The first date on which ordering may take place.
    #[serde(rename = "423")]
    _423,
    /// Last date of ordering
    ///
    /// The last date on which ordering may take place.
    #[serde(rename = "424")]
    _424,
    /// Original posting date
    ///
    /// Date when the entry was originally posted.
    #[serde(rename = "425")]
    _425,
    /// Reinsurance payment frequency
    ///
    /// The frequency of payments of reinsurance premiums.
    #[serde(rename = "426")]
    _426,
    /// Adjusted age
    ///
    /// The adjusted age used for purposes of calculation.
    #[serde(rename = "427")]
    _427,
    /// Original issue age
    ///
    /// The original issue age.
    #[serde(rename = "428")]
    _428,
    /// Coverage duration
    ///
    /// The period coverage has been in force.
    #[serde(rename = "429")]
    _429,
    /// Coverage issue date
    ///
    /// Date from which the anniversary coverage is measured.
    #[serde(rename = "430")]
    _430,
    /// Flat extra period
    ///
    /// Period for charging the additional extra.
    #[serde(rename = "431")]
    _431,
    /// Paid to date
    ///
    /// Date to which payments have been paid.
    #[serde(rename = "432")]
    _432,
    /// Reinsurance coverage duration
    ///
    /// The period for which reinsurance coverage has been in force.
    #[serde(rename = "433")]
    _433,
    /// Maturity date
    ///
    /// Date at which maturity occurs.
    #[serde(rename = "434")]
    _434,
    /// Reinsurance issue age
    ///
    /// The actual or equivalent age at time of issue.
    #[serde(rename = "435")]
    _435,
    /// Reinsurance paid-up date
    ///
    /// The date up to which the reinsurance has been paid.
    #[serde(rename = "436")]
    _436,
    /// Benefit period
    ///
    /// The period of time for which benefits are provided.
    #[serde(rename = "437")]
    _437,
    /// Disability wait period
    ///
    /// The period of time the insured must be disabled before reinsurance coverage becomes effective.
    #[serde(rename = "438")]
    _438,
    /// Deferred Period
    ///
    /// The period of time for which an activity has been postponed.
    #[serde(rename = "439")]
    _439,
    /// Documentary credit amendment date
    ///
    /// Date of amendment of a documentary credit.
    #[serde(rename = "440")]
    _440,
    /// Last on hire date
    ///
    /// Date the item was last placed on hire.
    #[serde(rename = "441")]
    _441,
    /// Last off hire date
    ///
    /// Date the item was last returned from hire.
    #[serde(rename = "442")]
    _442,
    /// Direct interchange date
    ///
    /// Date the item was directly interchanged.
    #[serde(rename = "443")]
    _443,
    /// Approval date
    ///
    /// Date of approval.
    #[serde(rename = "444")]
    _444,
    /// Original estimate date
    ///
    /// The date of the original estimate.
    #[serde(rename = "445")]
    _445,
    /// Revised estimate date
    ///
    /// The date the estimate was revised.
    #[serde(rename = "446")]
    _446,
    /// Creditor's requested value date
    ///
    /// Date on which the creditor requests to be credited.
    #[serde(rename = "447")]
    _447,
    /// Referenced item creation date
    ///
    /// Creation date of referenced item.
    #[serde(rename = "448")]
    _448,
    /// Date for the last update
    ///
    /// Date for the last update.
    #[serde(rename = "449")]
    _449,
    /// Opening date
    ///
    /// Date of opening.
    #[serde(rename = "450")]
    _450,
    /// Source document capture date
    ///
    /// Date source document data is entered into a business application.
    #[serde(rename = "451")]
    _451,
    /// Trial balance period
    ///
    /// Period covered by the trial balance.
    #[serde(rename = "452")]
    _452,
    /// Date of source document
    ///
    /// The date of the source document.
    #[serde(rename = "453")]
    _453,
    /// Accounting value date
    ///
    /// Date against which the entry has to be legally allocated.
    #[serde(rename = "454")]
    _454,
    /// Expected value date
    ///
    /// Date on which the funds are expected to be at the disposal of the beneficiary.
    #[serde(rename = "455")]
    _455,
    /// Chart of account period
    ///
    /// Period covered by the chart of account.
    #[serde(rename = "456")]
    _456,
    /// Date of separation
    ///
    /// Date of marital separation.
    #[serde(rename = "457")]
    _457,
    /// Date of divorce
    ///
    /// Date when two married persons are officially divorced.
    #[serde(rename = "458")]
    _458,
    /// Date of marriage
    ///
    /// Date when two persons are married.
    #[serde(rename = "459")]
    _459,
    /// Wage period, start date
    ///
    /// Date when a period of wage begins.
    #[serde(rename = "460")]
    _460,
    /// Wage period, end date
    ///
    /// Date when a period of wage ends.
    #[serde(rename = "461")]
    _461,
    /// Working period, start date
    ///
    /// Date when a period of work begins.
    #[serde(rename = "462")]
    _462,
    /// Working period, end date
    ///
    /// Date when a period of work ends.
    #[serde(rename = "463")]
    _463,
    /// Embarkation date and time
    ///
    /// Date and time at which crew and/or passengers board.
    #[serde(rename = "464")]
    _464,
    /// Disembarkation date and time
    ///
    /// Date and time at which crew and/or passengers disembark.
    #[serde(rename = "465")]
    _465,
    /// Time now date
    ///
    /// A time now date used for planning and scheduling purposes.
    #[serde(rename = "466")]
    _466,
    /// Holiday
    ///
    /// A date or period that is a break from work.
    #[serde(rename = "467")]
    _467,
    /// Non working
    ///
    /// To specify a non working date or period.
    #[serde(rename = "468")]
    _468,
    /// Start date or time, earliest
    ///
    /// The earliest date or time for starting.
    #[serde(rename = "469")]
    _469,
    /// Start date or time, latest
    ///
    /// The latest date or time for starting.
    #[serde(rename = "470")]
    _470,
    /// Finish date or time, earliest
    ///
    /// The earliest date or time for finishing.
    #[serde(rename = "471")]
    _471,
    /// Finish date or time, latest
    ///
    /// The latest date or time for finishing.
    #[serde(rename = "472")]
    _472,
    /// Start date or time, mandatory
    ///
    /// The mandatory date or time for starting.
    #[serde(rename = "473")]
    _473,
    /// Finish date or time, mandatory
    ///
    /// The mandatory date or time for finishing.
    #[serde(rename = "474")]
    _474,
    /// Start date or time, actual
    ///
    /// The actual date or time for starting.
    #[serde(rename = "475")]
    _475,
    /// Start date or time, estimated
    ///
    /// The estimated date or time for starting.
    #[serde(rename = "476")]
    _476,
    /// Completion date or time, estimated
    ///
    /// The estimated date or time for completion.
    #[serde(rename = "477")]
    _477,
    /// Start date or time, scheduled
    ///
    /// The scheduled date or time for starting.
    #[serde(rename = "478")]
    _478,
    /// Completion date or time, scheduled
    ///
    /// The scheduled date or time for completion.
    #[serde(rename = "479")]
    _479,
    /// Start date or time, not before
    ///
    /// The not before date or time for starting.
    #[serde(rename = "480")]
    _480,
    /// Start date or time, not after
    ///
    /// The not after date or time for starting.
    #[serde(rename = "481")]
    _481,
    /// Completion date or time, not before
    ///
    /// The not before date or time for completion.
    #[serde(rename = "482")]
    _482,
    /// Completion date or time, not after
    ///
    /// The not after date or time for completion.
    #[serde(rename = "483")]
    _483,
    /// Illness recovery date, expected
    ///
    /// Date when a person is expected to recover from illness.
    #[serde(rename = "484")]
    _484,
    /// Period of illness, start date
    ///
    /// Date when a period of illness began.
    #[serde(rename = "485")]
    _485,
    /// Period of illness, end date
    ///
    /// Date when a period of illness ends.
    #[serde(rename = "486")]
    _486,
    /// Decease date
    ///
    /// Date when a person died.
    #[serde(rename = "487")]
    _487,
    /// Benefit period, start date
    ///
    /// Date when a period of benefit begins.
    #[serde(rename = "488")]
    _488,
    /// Benefit period, end date
    ///
    /// Date when a period of benefit ends.
    #[serde(rename = "489")]
    _489,
    /// Selection period, start date
    ///
    /// Date when a period of selection begins.
    #[serde(rename = "490")]
    _490,
    /// Selection period, end date
    ///
    /// Date when a period of selection ends.
    #[serde(rename = "491")]
    _491,
    /// Balance date/time/period
    ///
    /// The date/time/period of a balance.
    #[serde(rename = "492")]
    _492,
    /// Benefit payments termination date
    ///
    /// To identify the date on which benefit payments have ceased.
    #[serde(rename = "493")]
    _493,
    /// Covered income period
    ///
    /// To identify the period over which covered income is measured.
    #[serde(rename = "494")]
    _494,
    /// Current income period
    ///
    /// To identify the period over which current income is measured.
    #[serde(rename = "495")]
    _495,
    /// Reinstatement date
    ///
    /// Identifies the date of reinstatement.
    #[serde(rename = "496")]
    _496,
    /// Definition of disability duration
    ///
    /// To identify the period for which the definition of disability applies.
    #[serde(rename = "497")]
    _497,
    /// Previous termination date
    ///
    /// Identifies the date of the previous termination.
    #[serde(rename = "498")]
    _498,
    /// Premium change period
    ///
    /// To identify the period of the premium change.
    #[serde(rename = "499")]
    _499,
    /// Off-hire survey date
    ///
    /// Date on which the equipment was surveyed at the end of the current leasing period.
    #[serde(rename = "500")]
    _500,
    /// In service survey date
    ///
    /// Date of survey of equipment while in use.
    #[serde(rename = "501")]
    _501,
    /// On hire survey date
    ///
    /// Date on which the equipment was surveyed at the beginning of the current leasing period.
    #[serde(rename = "502")]
    _502,
    /// Production inspection date
    ///
    /// Date of production inspection.
    #[serde(rename = "503")]
    _503,
    /// Overtime, start date
    ///
    /// Date when a period of overtime begins.
    #[serde(rename = "504")]
    _504,
    /// Overtime, end date
    ///
    /// Date when a period of overtime ends.
    #[serde(rename = "505")]
    _505,
    /// Back order delivery date/time/period
    ///
    /// The date/time/period during which the delivery of a back order will take, or has taken, place.
    #[serde(rename = "506")]
    _506,
    /// Negotiations start date
    ///
    /// The date on which negotiations started.
    #[serde(rename = "507")]
    _507,
    /// Work effective start date
    ///
    /// The date on which work will effectively start.
    #[serde(rename = "508")]
    _508,
    /// Contract binding date
    ///
    /// The date from which a contract becomes binding on the contracting parties.
    #[serde(rename = "509")]
    _509,
    /// Notification time limit
    ///
    /// The time limit which has been set for a notification to take place.
    #[serde(rename = "510")]
    _510,
    /// Time limit
    ///
    /// The time limit in which an event must take place.
    #[serde(rename = "511")]
    _511,
    /// Attendance date and or time and or period
    ///
    /// Date and or time and or period of attendance.
    #[serde(rename = "512")]
    _512,
    /// Accident date and or time
    ///
    /// Date and or time when an accident occurred.
    #[serde(rename = "513")]
    _513,
    /// Adoption date, actual
    ///
    /// Actual date when adoption occurs.
    #[serde(rename = "514")]
    _514,
    /// Reimbursement claim issue date and or time
    ///
    /// Date and or time when a reimbursement claim is issued.
    #[serde(rename = "515")]
    _515,
    /// Hospital admission date and or time
    ///
    /// Date and or time of admission to a hospital.
    #[serde(rename = "516")]
    _516,
    /// Hospital discharge date and or time
    ///
    /// Date and or time of discharge from a hospital.
    #[serde(rename = "517")]
    _517,
    /// Period of care start date and or time
    ///
    /// Date and or time when a period of care starts.
    #[serde(rename = "518")]
    _518,
    /// Period of care end date and or time
    ///
    /// Date and or time when a period of care ends.
    #[serde(rename = "519")]
    _519,
    /// Department admission date and or time
    ///
    /// Date and or time of admission to a department.
    #[serde(rename = "520")]
    _520,
    /// Department discharge date and or time
    ///
    /// Date and or time of discharge from a department.
    #[serde(rename = "521")]
    _521,
    /// Childbirth date and or time, actual
    ///
    /// Actual date and or time of childbirth.
    #[serde(rename = "522")]
    _522,
    /// Prescription issue date and or time
    ///
    /// Date and or time when a prescription was issued.
    #[serde(rename = "523")]
    _523,
    /// Prescription dispensing date and or time
    ///
    /// Date and or time when a prescription was dispensed.
    #[serde(rename = "524")]
    _524,
    /// Clinical examination date and or time
    ///
    /// Date and or time of clinical examination.
    #[serde(rename = "525")]
    _525,
    /// Death date and or time
    ///
    /// Date and or time of death.
    #[serde(rename = "526")]
    _526,
    /// Childbirth date, estimated
    ///
    /// Estimated date of childbirth.
    #[serde(rename = "527")]
    _527,
    /// Last menstrual cycle, start date
    ///
    /// Date when the last menstrual cycle started.
    #[serde(rename = "528")]
    _528,
    /// Pregnancy duration, actual
    ///
    /// Actual duration of pregnancy.
    #[serde(rename = "529")]
    _529,
    /// Fumigation date and/or time
    ///
    /// The date/or time on which fumigation is to occur or has taken place.
    #[serde(rename = "530")]
    _530,
    /// Payment period
    ///
    /// A period of time in which a payment has been or will be made.
    #[serde(rename = "531")]
    _531,
    /// Average delivery delay
    ///
    /// The average delay between deliveries.
    #[serde(rename = "532")]
    _532,
    /// Budget line application date
    ///
    /// The date on which something has been applied to a budget line.
    #[serde(rename = "533")]
    _533,
    /// Date of repair or service
    ///
    /// The date of a repair or service.
    #[serde(rename = "534")]
    _534,
    /// Date of product failure
    ///
    /// The date the product failed.
    #[serde(rename = "535")]
    _535,
    /// Review date
    ///
    /// Date the item was or will be reviewed.
    #[serde(rename = "536")]
    _536,
    /// International review cycle start date
    ///
    /// Date the international review cycle starts.
    #[serde(rename = "537")]
    _537,
    /// International assessment approval for publication date
    ///
    /// Date the Data Maintenance Request (DMR) was approved for publication after completing international review.
    #[serde(rename = "538")]
    _538,
    /// Status assignment date
    ///
    /// Date a status was assigned.
    #[serde(rename = "539")]
    _539,
    /// Instruction's original execution date
    ///
    /// Original execution date for the instruction.
    #[serde(rename = "540")]
    _540,
    /// First published date
    ///
    /// Date when material was first published.
    #[serde(rename = "541")]
    _541,
    /// Last published date
    ///
    /// Date when material was last published.
    #[serde(rename = "542")]
    _542,
    /// Balance sheet date, latest
    ///
    /// Date of the latest balance sheet.
    #[serde(rename = "543")]
    _543,
    /// Security share price as of given date
    ///
    /// Date of the security share price.
    #[serde(rename = "544")]
    _544,
    /// Assigned date
    ///
    /// Date when assigned.
    #[serde(rename = "545")]
    _545,
    /// Business opened date
    ///
    /// Date opened for business.
    #[serde(rename = "546")]
    _546,
    /// Initial financial accounts filed date
    ///
    /// Date when the initial financial accounts were filed.
    #[serde(rename = "547")]
    _547,
    /// Stop work as of given date
    ///
    /// Date work stopped or will stop.
    #[serde(rename = "548")]
    _548,
    /// Completion date
    ///
    /// Date of completion.
    #[serde(rename = "549")]
    _549,
    /// Lease term, start date
    ///
    /// Start date of the lease term.
    #[serde(rename = "550")]
    _550,
    /// Lease term, end date
    ///
    /// End date of the lease term.
    #[serde(rename = "551")]
    _551,
    /// Start date, actual
    ///
    /// Actual date of start.
    #[serde(rename = "552")]
    _552,
    /// Start date, estimated
    ///
    /// Date of estimated start.
    #[serde(rename = "553")]
    _553,
    /// Filed date
    ///
    /// Date when filed.
    #[serde(rename = "554")]
    _554,
    /// Return to work date
    ///
    /// Date of return to work.
    #[serde(rename = "555")]
    _555,
    /// Purchased date
    ///
    /// Date of purchase.
    #[serde(rename = "556")]
    _556,
    /// Returned date
    ///
    /// Date return takes place.
    #[serde(rename = "557")]
    _557,
    /// Changed date
    ///
    /// Date change takes place.
    #[serde(rename = "558")]
    _558,
    /// Terminated date
    ///
    /// Date termination takes place.
    #[serde(rename = "559")]
    _559,
    /// Evaluation date
    ///
    /// Date evaluation takes place.
    #[serde(rename = "560")]
    _560,
    /// Business termination date
    ///
    /// Date the business terminates.
    #[serde(rename = "561")]
    _561,
    /// Release from bankruptcy date
    ///
    /// Date when an entity is released from bankruptcy status.
    #[serde(rename = "562")]
    _562,
    /// Placement date, initial
    ///
    /// Date of initial placement.
    #[serde(rename = "563")]
    _563,
    /// Signature date
    ///
    /// Date of signature.
    #[serde(rename = "564")]
    _564,
    /// Bankruptcy filed date
    ///
    /// Date when bankruptcy was filed.
    #[serde(rename = "565")]
    _565,
    /// End date, scheduled
    ///
    /// Date when activity is scheduled to end.
    #[serde(rename = "566")]
    _566,
    /// Report period
    ///
    /// Period covered by the report.
    #[serde(rename = "567")]
    _567,
    /// Suspended date
    ///
    /// Date of suspension.
    #[serde(rename = "568")]
    _568,
    /// Renewal date
    ///
    /// Date of renewal.
    #[serde(rename = "569")]
    _569,
    /// Reported date
    ///
    /// Date when reported.
    #[serde(rename = "570")]
    _570,
    /// Checked date
    ///
    /// Date when checked.
    #[serde(rename = "571")]
    _571,
    /// Present residence, start date
    ///
    /// The beginning date of residence at present location.
    #[serde(rename = "572")]
    _572,
    /// Employment position, start date
    ///
    /// The start date of employment in a particular position.
    #[serde(rename = "573")]
    _573,
    /// Account closed date
    ///
    /// Date when account was closed.
    #[serde(rename = "574")]
    _574,
    /// Construction date, actual
    ///
    /// Date of actual construction.
    #[serde(rename = "575")]
    _575,
    /// Employment profession start date
    ///
    /// Start date of employment in a particular profession.
    #[serde(rename = "576")]
    _576,
    /// Next review date
    ///
    /// Date of next review.
    #[serde(rename = "577")]
    _577,
    /// Meeting date
    ///
    /// Date of the meeting.
    #[serde(rename = "578")]
    _578,
    /// Administrator ordered date
    ///
    /// Date when an administrator is ordered for a company.
    #[serde(rename = "579")]
    _579,
    /// Last date to file a claim
    ///
    /// Date after which no claim can be filed.
    #[serde(rename = "580")]
    _580,
    /// Convicted date
    ///
    /// Date when convicted.
    #[serde(rename = "581")]
    _581,
    /// Interviewed date
    ///
    /// Date of an interview.
    #[serde(rename = "582")]
    _582,
    /// Last visit date
    ///
    /// Date of last visit.
    #[serde(rename = "583")]
    _583,
    /// Future period
    ///
    /// Period in the future.
    #[serde(rename = "584")]
    _584,
    /// Preceding period
    ///
    /// Period preceding current period.
    #[serde(rename = "585")]
    _585,
    /// Expected problem resolution date
    ///
    /// Date when problem is expected to be resolved.
    #[serde(rename = "586")]
    _586,
    /// Action date
    ///
    /// Date of action.
    #[serde(rename = "587")]
    _587,
    /// Accountant's opinion date
    ///
    /// Date of an accountant's opinion.
    #[serde(rename = "588")]
    _588,
    /// Last activity date
    ///
    /// Date of last activity.
    #[serde(rename = "589")]
    _589,
    /// Resolved date
    ///
    /// Date when resolved.
    #[serde(rename = "590")]
    _590,
    /// Recorded date
    ///
    /// Date when recorded.
    #[serde(rename = "591")]
    _591,
    /// Date of birth, estimated
    ///
    /// The estimated date of birth.
    #[serde(rename = "592")]
    _592,
    /// Last annual report date
    ///
    /// Date of the last annual report.
    #[serde(rename = "593")]
    _593,
    /// Net worth date
    ///
    /// Date of net worth.
    #[serde(rename = "594")]
    _594,
    /// Profit period
    ///
    /// Period over which profit was earned.
    #[serde(rename = "596")]
    _596,
    /// Registration date
    ///
    /// Date when registered.
    #[serde(rename = "597")]
    _597,
    /// Consolidation date
    ///
    /// Date when consolidation occurred.
    #[serde(rename = "598")]
    _598,
    /// Board of directors not authorised as of given date
    ///
    /// As of this date the board of directors is not authorised.
    #[serde(rename = "599")]
    _599,
    /// Board of directors not complete as of given date
    ///
    /// As of this date the board of directors is not fully filled.
    #[serde(rename = "600")]
    _600,
    /// Manager not registered as of given date
    ///
    /// As of this date the manager is not registered.
    #[serde(rename = "601")]
    _601,
    /// Citizenship change date
    ///
    /// Date of citizenship change.
    #[serde(rename = "602")]
    _602,
    /// Participation date
    ///
    /// Date of participation.
    #[serde(rename = "603")]
    _603,
    /// Capitalisation date
    ///
    /// Date of capitalisation.
    #[serde(rename = "604")]
    _604,
    /// Board of directors registration date
    ///
    /// Date when the board of directors was registered.
    #[serde(rename = "605")]
    _605,
    /// Operations ceased date
    ///
    /// Date when operations ceased.
    #[serde(rename = "606")]
    _606,
    /// Satisfaction date
    ///
    /// Date when satisfaction was obtained.
    #[serde(rename = "607")]
    _607,
    /// Legal settlement terms met date
    ///
    /// Date when terms specified in the legal settlement were met.
    #[serde(rename = "608")]
    _608,
    /// Business control change date
    ///
    /// Date when a new authority took control.
    #[serde(rename = "609")]
    _609,
    /// Court registration date
    ///
    /// Date of registration in the court.
    #[serde(rename = "610")]
    _610,
    /// Annual report due date
    ///
    /// Date when annual report is due.
    #[serde(rename = "611")]
    _611,
    /// Asset and liability schedule date
    ///
    /// Date of the asset and liability schedule.
    #[serde(rename = "612")]
    _612,
    /// Annual report mailing date
    ///
    /// Date when the annual report was mailed.
    #[serde(rename = "613")]
    _613,
    /// Annual report filing date
    ///
    /// Date when the annual report was filed.
    #[serde(rename = "614")]
    _614,
    /// Annual report delinquent on date
    ///
    /// Date when annual report was considered delinquent.
    #[serde(rename = "615")]
    _615,
    /// Accounting methodology change date
    ///
    /// Date when accounting methodology was changed.
    #[serde(rename = "616")]
    _616,
    /// Closed until date
    ///
    /// Date when again open.
    #[serde(rename = "617")]
    _617,
    /// Conversion into holding company date
    ///
    /// Date business was converted into a holding company.
    #[serde(rename = "618")]
    _618,
    /// Deed not available as of given date
    ///
    /// Date when deed was not available.
    #[serde(rename = "619")]
    _619,
    /// Detrimental information receipt date
    ///
    /// Date when detrimental information was received.
    #[serde(rename = "620")]
    _620,
    /// Construction date, estimated
    ///
    /// Estimated date of construction.
    #[serde(rename = "621")]
    _621,
    /// Financial information date
    ///
    /// Date of the financial information.
    #[serde(rename = "622")]
    _622,
    /// Graduation date
    ///
    /// Date when graduation occurs.
    #[serde(rename = "623")]
    _623,
    /// Insolvency discharge granted date
    ///
    /// Date when insolvency discharge was granted.
    #[serde(rename = "624")]
    _624,
    /// Incorporation date
    ///
    /// Date of incorporation.
    #[serde(rename = "625")]
    _625,
    /// Inactivity end date
    ///
    /// Date when inactivity ends.
    #[serde(rename = "626")]
    _626,
    /// Last check for balance sheet update date
    ///
    /// Date balance sheet was last checked to determine if update had taken place.
    #[serde(rename = "627")]
    _627,
    /// Last capital change date
    ///
    /// Date of last capital change.
    #[serde(rename = "628")]
    _628,
    /// Letter of agreement date
    ///
    /// Date of a letter of agreement.
    #[serde(rename = "629")]
    _629,
    /// Letter of liability date
    ///
    /// Date of a letter of liability.
    #[serde(rename = "630")]
    _630,
    /// Liquidation date
    ///
    /// Date of liquidation.
    #[serde(rename = "631")]
    _631,
    /// Lowest activity period
    ///
    /// Period of lowest activity.
    #[serde(rename = "632")]
    _632,
    /// Legal structure change date
    ///
    /// Date when legal structure was changed.
    #[serde(rename = "633")]
    _633,
    /// Current name effective date
    ///
    /// Date when current name became effective.
    #[serde(rename = "634")]
    _634,
    /// Not registered as of given date
    ///
    /// Date when not yet registered.
    #[serde(rename = "635")]
    _635,
    /// Current authority control start date
    ///
    /// Date when current authority took control.
    #[serde(rename = "636")]
    _636,
    /// Privilege details verification date
    ///
    /// Date when privilege details were verified.
    #[serde(rename = "637")]
    _637,
    /// Current legal structure effective date
    ///
    /// Date when current legal structure became effective.
    #[serde(rename = "638")]
    _638,
    /// Peak activity period
    ///
    /// Period of peak activity.
    #[serde(rename = "639")]
    _639,
    /// Presentation to bankruptcy receivers date
    ///
    /// Date when presented to the bankruptcy receivers.
    #[serde(rename = "640")]
    _640,
    /// Resignation date
    ///
    /// Date of resignation.
    #[serde(rename = "641")]
    _641,
    /// Legal action closed date
    ///
    /// Date when the legal action was closed.
    #[serde(rename = "642")]
    _642,
    /// Mail receipt date
    ///
    /// Date mail was received.
    #[serde(rename = "643")]
    _643,
    /// Social security claims verification date
    ///
    /// Date when social security claims were verified.
    #[serde(rename = "644")]
    _644,
    /// Sole directorship registration date
    ///
    /// Date when sole directorship was registered.
    #[serde(rename = "645")]
    _645,
    /// Trade style registration date
    ///
    /// Date when trade style was registered.
    #[serde(rename = "646")]
    _646,
    /// Trial start date, scheduled
    ///
    /// Date when a trial is scheduled to begin.
    #[serde(rename = "647")]
    _647,
    /// Trial start date, actual
    ///
    /// Date when the trial actually started.
    #[serde(rename = "648")]
    _648,
    /// Value Added Tax (VAT) claims verification date
    ///
    /// Date when the Value Added Tax (VAT) claims were verified.
    #[serde(rename = "649")]
    _649,
    /// Receivership result date
    ///
    /// Date when the result of the receivership occurs.
    #[serde(rename = "650")]
    _650,
    /// Investigation end date
    ///
    /// The date when an investigation ended.
    #[serde(rename = "651")]
    _651,
    /// Employee temporary laid-off period end date
    ///
    /// The ending date of a period in which employees were temporarily placed out of work.
    #[serde(rename = "652")]
    _652,
    /// Investigation start date
    ///
    /// The date when an investigation began.
    #[serde(rename = "653")]
    _653,
    /// Income period
    ///
    /// The period of time in which income is earned.
    #[serde(rename = "654")]
    _654,
    /// Criminal sentence duration
    ///
    /// The period of time over which a criminal sentence applies.
    #[serde(rename = "655")]
    _655,
    /// Age
    ///
    /// The length of time that a person or thing has existed.
    #[serde(rename = "656")]
    _656,
    /// Receivables collection period
    ///
    /// The period of time over which receivable accounts are collected.
    #[serde(rename = "657")]
    _657,
    /// Comparison period
    ///
    /// The time period covered in a comparison.
    #[serde(rename = "658")]
    _658,
    /// Adjournment
    ///
    /// The period of time over which an adjournment is in effect.
    #[serde(rename = "659")]
    _659,
    /// Court dismissal date
    ///
    /// The date on which a court refused further hearing of a case.
    #[serde(rename = "660")]
    _660,
    /// Insufficient assets judgement date
    ///
    /// The date on which assets were judged to be insufficient.
    #[serde(rename = "661")]
    _661,
    /// Average payment period
    ///
    /// The average period of time over which money has been paid.
    #[serde(rename = "662")]
    _662,
    /// Forecast period start
    ///
    /// The beginning of a forecast period.
    #[serde(rename = "663")]
    _663,
    /// Period extended
    ///
    /// Number of time units added to the original end date/time/period.
    #[serde(rename = "664")]
    _664,
    /// Employee temporary laid-off period start date
    ///
    /// The start date of a period in which employees were temporarily placed out of work.
    #[serde(rename = "665")]
    _665,
    /// Management available date
    ///
    /// Date when management is available.
    #[serde(rename = "666")]
    _666,
    /// Withdrawn date
    ///
    /// The date when something was retracted.
    #[serde(rename = "667")]
    _667,
    /// Claim incurred date
    ///
    /// The date that the claim was incurred.
    #[serde(rename = "668")]
    _668,
    /// Financial coverage period
    ///
    /// The period of time for which financial coverage applies.
    #[serde(rename = "669")]
    _669,
    /// Claim made date
    ///
    /// The date on which a claim was made.
    #[serde(rename = "670")]
    _670,
    /// Stop distribution date
    ///
    /// The date on which distribution is to stop.
    #[serde(rename = "671")]
    _671,
    /// Period assigned
    ///
    /// The period assigned.
    #[serde(rename = "672")]
    _672,
    /// Lease period
    ///
    /// The period associated with a lease.
    #[serde(rename = "673")]
    _673,
    /// Forecast period end date
    ///
    /// The ending date of a forecast period.
    #[serde(rename = "674")]
    _674,
    /// Judgement date
    ///
    /// The date on which a decision from a court of law was rendered.
    #[serde(rename = "675")]
    _675,
    /// Period worked for the company
    ///
    /// Period of time that was worked for the company.
    #[serde(rename = "676")]
    _676,
    /// Transport equipment stuffing date and/or time
    ///
    /// The date and/or time on which the stuffing of transport equipment is to or has taken place.
    #[serde(rename = "677")]
    _677,
    /// Transport equipment stripping date and/or time
    ///
    /// The date and/or time on which the stripping of a transport equipment is to or has taken place.
    #[serde(rename = "678")]
    _678,
    /// Initial request date
    ///
    /// Date of an initial request.
    #[serde(rename = "679")]
    _679,
    /// Period overdue
    ///
    /// The period by which an event is overdue.
    #[serde(rename = "680")]
    _680,
    /// Implementation date/time/period
    ///
    /// A date/time/period within which an implementation is to take place.
    #[serde(rename = "681")]
    _681,
    /// Refusal period
    ///
    /// The period within which a refusal can be made.
    #[serde(rename = "682")]
    _682,
    /// Suspension period
    ///
    /// The period for which something is suspended.
    #[serde(rename = "683")]
    _683,
    /// Deletion date
    ///
    /// The date on which deletion occurs.
    #[serde(rename = "684")]
    _684,
    /// First sale date and/or time and/or period
    ///
    /// The first date, and/or time, and/or period a product was sold.
    #[serde(rename = "685")]
    _685,
    /// Last sale date and/or time and/or period
    ///
    /// The last date, and/or time, and/or period a product was sold.
    #[serde(rename = "686")]
    _686,
    /// Date ready for collection
    ///
    /// A date on which an object is ready for collection.
    #[serde(rename = "687")]
    _687,
    /// Shipping date, no schedule established as of
    ///
    /// As at this date no valid shipping schedule has been established.
    #[serde(rename = "688")]
    _688,
    /// Shipping date and/or time, current schedule
    ///
    /// Shipping date and/or time as currently scheduled.
    #[serde(rename = "689")]
    _689,
    /// Suppliers' average credit period
    ///
    /// The average period of time that credit is extended by suppliers.
    #[serde(rename = "690")]
    _690,
    /// Advising date
    ///
    /// Date of advice.
    #[serde(rename = "691")]
    _691,
    /// Project over target baseline date
    ///
    /// The date an over target baseline was implemented for a project.
    #[serde(rename = "692")]
    _692,
    /// Established date
    ///
    /// Date when an entity was established or created.
    #[serde(rename = "693")]
    _693,
    /// Latest filing period
    ///
    /// Latest period for which a filing may be made.
    #[serde(rename = "694")]
    _694,
    /// Mailing date
    ///
    /// Date when an item may be mailed.
    #[serde(rename = "695")]
    _695,
    /// Date/time of latest accounts filing at public registry
    ///
    /// The latest date/time when financial accounts were filed at public registry.
    #[serde(rename = "696")]
    _696,
    /// Date placed in disfavour
    ///
    /// Date when placed in a disfavoured category or status.
    #[serde(rename = "697")]
    _697,
    /// Employment position start date, estimated
    ///
    /// Estimated start date of employment in a particular position.
    #[serde(rename = "698")]
    _698,
    /// Registered contractor number assignment date, original
    ///
    /// Date when a registered contractor number was originally assigned.
    #[serde(rename = "699")]
    _699,
    /// Ownership change date
    ///
    /// Date when ownership changes.
    #[serde(rename = "700")]
    _700,
    /// Original duration
    ///
    /// Original length of time.
    #[serde(rename = "701")]
    _701,
    /// Period between changes
    ///
    /// The period of time between changes.
    #[serde(rename = "702")]
    _702,
    /// From date of notice to proceed to commencement of performance
    ///
    /// Period of time from notice to proceed until performance commencement.
    #[serde(rename = "703")]
    _703,
    /// From date of notice to proceed to completion
    ///
    /// Period of time from date of notice to proceed until completion.
    #[serde(rename = "704")]
    _704,
    /// Period an event is late due to customer
    ///
    /// The period of time an event is late due to the actions of a customer.
    #[serde(rename = "705")]
    _705,
    /// File generation date and/or time
    ///
    /// Date and, or time of file generation.
    #[serde(rename = "706")]
    _706,
    /// Endorsed certificate issue date
    ///
    /// Date on which a certificate, endorsed by signature or other agreed means, is issued.
    #[serde(rename = "707")]
    _707,
    /// Patient first visit for condition
    ///
    /// The date of the first visit by a patient to a healthcare provider for this condition.
    #[serde(rename = "708")]
    _708,
    /// Admission date and/or time, expected
    ///
    /// Expected date and/or time of admission.
    #[serde(rename = "709")]
    _709,
    /// Symptoms onset, patient alleged
    ///
    /// Date and/or time of onset of symptoms according to the patient.
    #[serde(rename = "710")]
    _710,
    /// Accident benefit period
    ///
    /// To identify the period of time for which benefits are provided in the event of an accident.
    #[serde(rename = "711")]
    _711,
    /// Accident benefit age limit
    ///
    /// To identify the age to which benefits are provided to the insured in the event of an accident.
    #[serde(rename = "712")]
    _712,
    /// Accident lifetime benefit qualification age
    ///
    /// To identify the qualification age for lifetime benefits provided to the insured in the event of an accident.
    #[serde(rename = "713")]
    _713,
    /// Sickness benefit period
    ///
    /// To identify the period of time for which benefits are provided in the event of sickness.
    #[serde(rename = "714")]
    _714,
    /// Sickness benefit age limit
    ///
    /// To identify the age to which benefits are provided to the insured in the event of sickness.
    #[serde(rename = "715")]
    _715,
    /// Sickness lifetime benefit qualification age
    ///
    /// To identify the qualification age for lifetime benefits provided to the insured in the event of sickness.
    #[serde(rename = "716")]
    _716,
    /// Accident insurance elimination period
    ///
    /// To identify the period of time the insured must be disabled in the event of an accident for benefits to be payable by the ceding company.
    #[serde(rename = "717")]
    _717,
    /// Sickness insurance elimination period
    ///
    /// The period of time the insured must be disabled in the event of sickness for benefits to be payable by the ceding company.
    #[serde(rename = "718")]
    _718,
    /// Provider signature date
    ///
    /// Date when the provider signed.
    #[serde(rename = "719")]
    _719,
    /// Condition initial treatment date
    ///
    /// Date when initially treated for this condition.
    #[serde(rename = "720")]
    _720,
    /// Information release authorization date
    ///
    /// Date when the information was authorized to be released.
    #[serde(rename = "721")]
    _721,
    /// Benefit release authorization date
    ///
    /// Date when a benefit is authorized for release.
    #[serde(rename = "722")]
    _722,
    /// Last seen date
    ///
    /// The date when last seen.
    #[serde(rename = "723")]
    _723,
    /// Acute manifestation date
    ///
    /// The date the symptoms manifested themselves in an acute form.
    #[serde(rename = "724")]
    _724,
    /// Similar illness onset date
    ///
    /// The date of the onset of an illness similar to the illness currently being treated.
    #[serde(rename = "725")]
    _725,
    /// Last X-ray date
    ///
    /// The date the last X-ray was taken.
    #[serde(rename = "726")]
    _726,
    /// Placement date, previous
    ///
    /// The date something was previously placed.
    #[serde(rename = "727")]
    _727,
    /// Placement date
    ///
    /// The date something is placed.
    #[serde(rename = "728")]
    _728,
    /// Temporary prosthesis date
    ///
    /// The date a temporary prosthetic device was provided.
    #[serde(rename = "729")]
    _729,
    /// Orthodontic treatment period, remaining
    ///
    /// The period of time that the orthodontic treatment has remaining.
    #[serde(rename = "730")]
    _730,
    /// Orthodontic treatment period, total
    ///
    /// The period of orthodontic treatment from beginning to end.
    #[serde(rename = "731")]
    _731,
    /// Maximum credit granted date
    ///
    /// Date on which the highest credit was granted.
    #[serde(rename = "732")]
    _732,
    /// Last date of accounts filed at public register
    ///
    /// Date on which accounts were last filed at the public register.
    #[serde(rename = "733")]
    _733,
    /// Allowed renewal duration period
    ///
    /// The period of time a company can renew its duration period.
    #[serde(rename = "734")]
    _734,
    /// Offset from Coordinated Universal Time (UTC)
    ///
    /// Number of hour's offset from Coordinated Universal Time (UTC).
    #[serde(rename = "735")]
    _735,
    /// Appointment expiry date
    ///
    /// Date when an appointment will expire.
    #[serde(rename = "736")]
    _736,
    /// Earliest filing period
    ///
    /// Earliest period for which a filing is made.
    #[serde(rename = "737")]
    _737,
    /// Original name change date
    ///
    /// Date when the original name was changed.
    #[serde(rename = "738")]
    _738,
    /// Education start date
    ///
    /// Date education begins at an educational institution.
    #[serde(rename = "739")]
    _739,
    /// Education end date
    ///
    /// Date education is completed at an educational institution.
    #[serde(rename = "740")]
    _740,
    /// Receivership period
    ///
    /// Period of time a receivership lasts.
    #[serde(rename = "741")]
    _741,
    /// Financial information submission date/time
    ///
    /// Date/time when financial information is submitted.
    #[serde(rename = "742")]
    _742,
    /// Purchase order latest possible change date
    ///
    /// Date identifying a point of time after which a purchase order cannot be changed.
    #[serde(rename = "743")]
    _743,
    /// Investment number allocation date
    ///
    /// The date that an investment number was allocated.
    #[serde(rename = "744")]
    _744,
    /// Mutually defined
    #[strum(ascii_case_insensitive)]
    ZZZ,
}

/// Date or time or period format code
///
/// Code specifying the representation of a date, time or period.
#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
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
    /// Calendar date and time: C=Century; Y=Year; M=Month;
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
    /// CC
    /// Century.
    _600,
    /// YY
    /// Calendar year: Y = Year.
    _601,
    /// CCYY
    /// Calendar year including century: C = Century; Y = Year.
    _602,
    /// YYS
    /// Semester in a calendar year: Y = Year; S = Semester.
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
    /// Format of period to be given without hyphen (A = ten days period).
    _613,

    /// CCYYMMA
    ///
    /// Format of period to be given without hyphen (A = ten days period).
    _614,

    /// YYWW
    /// Week within a calendar year: Y = Year; W = Week 1st week of January = week 01.
    _615,
    /// CCYYWW
    ///
    /// Week within a calendar year: CC = Century; Y = Year;
    /// W = Week (1st week of January = week 01).
    _616,
    /// YY-YY
    ///
    /// Format of period to be given in actual message without hyphen.
    _701,
    /// CCYY-CCYY
    ///
    /// Format of period to be given in actual message without hyphen.
    _702,
    /// YYS-YYS
    ///
    /// Format of period to be given without hyphen.
    _703,
    /// CCYYS-CCYYS
    ///
    /// Format of period to be given in actual message without hyphen.
    _704,
    /// YYPYYP
    ///
    /// Format of period to be given without hyphen (P = period of 4 months).
    _705,
    /// CCYYP-CCYYP
    ///
    /// Format of period to be given without hyphen (P = period of 4 months).
    _706,
    /// YYQ-YYQ
    ///
    /// Format of period to be given without hyphen.
    _707,
    /// CCYYQ-CCYYQ
    ///
    /// Format of period to be given in actual message without hyphen.
    _708,
    /// YYMM-YYMM
    ///
    /// Format of period to be given in actual message without hyphen.
    _709,
    /// CCYYMM-CCYYMM
    ///
    /// Format of period to be given in actual message without hyphen.
    #[serde(rename = "710")]
    _710,
    /// CCYYMMDD-CCYYMMDD
    ///
    /// Format of period to be given in actual message without hyphen.
    _711,
    /// YYMMDDHHMM-YYMMDDHHMM
    ///
    /// Format of period to be given in actual message without hyphen.
    _713,
    /// YYWW-YYWW
    ///
    /// Format of period to be given in actual message without hyphen.
    _715,
    /// CCYYWW-CCYYWW
    ///
    /// Format of period to be given without hyphen.
    _716,
    /// YYMMDD-YYMMDD
    ///
    /// Format of period to be given in actual message without hyphen.
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

/// Code list responsible agency code
///
/// Code specifying the agency responsible for a code list.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _3055 {
    /// CCC (Customs Co-operation Council)
    ///
    #[serde(rename = "1")]
    _1,
    /// CEC (Commission of the European Communities)
    ///
    /// Generic: see also 140, 141, 142, 162.
    #[serde(rename = "2")]
    _2,
    /// IATA (International Air Transport Association)
    ///
    /// The airline industry's international organisation.
    #[serde(rename = "3")]
    _3,
    /// ICC (International Chamber of Commerce)
    ///
    #[serde(rename = "4")]
    _4,
    /// ISO (International Organization for Standardization)
    ///
    #[serde(rename = "5")]
    _5,
    /// UN/ECE (United Nations - Economic Commission for Europe)
    ///
    #[serde(rename = "6")]
    _6,
    /// CEFIC (Conseil Europeen des Federations de l'Industrie Chimique)
    ///
    /// EDI project for chemical industry.
    #[serde(rename = "7")]
    _7,
    /// EDIFICE
    ///
    /// Standardised electronic commerce forum for companies with interests in computing, electronics and telecommunications.
    #[serde(rename = "8")]
    _8,
    /// EAN (International Article Numbering association)
    ///
    #[serde(rename = "9")]
    _9,
    /// ODETTE
    ///
    /// Organization for Data Exchange through Tele-Transmission in Europe (European automotive industry project).
    #[serde(rename = "10")]
    _10,
    /// Lloyd's register of shipping
    ///
    /// A register of ocean going vessels maintained by Lloyd's of London.
    #[serde(rename = "11")]
    _11,
    /// UIC (International union of railways)
    ///
    /// International Union of Railways.
    #[serde(rename = "12")]
    _12,
    /// ICAO (International Civil Aviation Organisation)
    ///
    #[serde(rename = "13")]
    _13,
    /// ICS (International Chamber of Shipping)
    ///
    #[serde(rename = "14")]
    _14,
    /// RINET (Reinsurance and Insurance Network)
    ///
    #[serde(rename = "15")]
    _15,
    /// US, D&B (Dun & Bradstreet Corporation)
    ///
    /// Identifies the Dun & Bradstreet Corporation, United States.
    #[serde(rename = "16")]
    _16,
    /// S.W.I.F.T.
    ///
    /// Society for Worldwide Interbank Financial Telecommunications s.c.
    #[serde(rename = "17")]
    _17,
    /// Conventions on SAD and transit (EC and EFTA)
    ///
    /// SAD = Single Administrative Document.
    #[serde(rename = "18")]
    _18,
    /// FRRC (Federal Reserve Routing Code)
    ///
    #[serde(rename = "19")]
    _19,
    /// BIC (Bureau International des Containeurs)
    ///
    /// The container industry's international organisation responsible for the issuance of container-related codes.
    #[serde(rename = "20")]
    _20,
    /// Assigned by transport company
    ///
    /// Codes assigned by a transport company.
    #[serde(rename = "21")]
    _21,
    /// US, ISA (Information Systems Agreement)
    ///
    /// Codes assigned by the ISA for use by its members.
    #[serde(rename = "22")]
    _22,
    /// FR, EDITRANSPORT
    ///
    /// French association developing EDI in transport logistics.
    #[serde(rename = "23")]
    _23,
    /// AU, ROA (Railways of Australia)
    ///
    /// Maintains code lists which are accepted by Australian government railways.
    #[serde(rename = "24")]
    _24,
    /// EDITEX (Europe)
    ///
    /// EDI group for the textile and clothing industry.
    #[serde(rename = "25")]
    _25,
    /// NL, Foundation Uniform Transport Code
    ///
    /// Foundation Uniform Transport Code is the EDI organisation for shippers, carriers and other logistic service providers in the Netherlands.
    #[serde(rename = "26")]
    _26,
    /// US, FDA (Food and Drug Administration)
    ///
    /// U.S. food and drug administration.
    #[serde(rename = "27")]
    _27,
    /// EDITEUR (European book sector electronic data interchange group)
    ///
    /// Code identifying the pan European user group for the book industry as an organisation responsible for code values in the book industry.
    #[serde(rename = "28")]
    _28,
    /// GB, FLEETNET
    ///
    /// Association of fleet vehicle hiring and leasing companies in the UK.
    #[serde(rename = "29")]
    _29,
    /// GB, ABTA (Association of British Travel Agencies)
    ///
    /// ABTA, Association of British Travel Agencies.
    #[serde(rename = "30")]
    _30,
    /// FI, Finish State Railway
    ///
    /// Finish State Railway.
    #[serde(rename = "31")]
    _31,
    /// PL, Polish State Railway
    ///
    /// Polish State Railway.
    #[serde(rename = "32")]
    _32,
    /// BG, Bulgaria State Railway
    ///
    /// Bulgaria State Railway.
    #[serde(rename = "33")]
    _33,
    /// RO, Rumanian State Railway
    ///
    /// Rumanian State Railway.
    #[serde(rename = "34")]
    _34,
    /// CZ, Tchechian State Railway
    ///
    /// Tchechian State Railway.
    #[serde(rename = "35")]
    _35,
    /// HU, Hungarian State Railway
    ///
    /// Hungarian State Railway.
    #[serde(rename = "36")]
    _36,
    /// GB, British Railways
    ///
    /// British Railways.
    #[serde(rename = "37")]
    _37,
    /// ES, Spanish National Railway
    ///
    /// Spanish National Railway.
    #[serde(rename = "38")]
    _38,
    /// SE, Swedish State Railway
    ///
    /// Swedish State Railway.
    #[serde(rename = "39")]
    _39,
    /// NO, Norwegian State Railway
    ///
    /// Norwegian State Railway.
    #[serde(rename = "40")]
    _40,
    /// DE, German Railway
    ///
    /// German Railway.
    #[serde(rename = "41")]
    _41,
    /// AT, Austrian Federal Railways
    ///
    /// Austrian Federal Railways.
    #[serde(rename = "42")]
    _42,
    /// LU, Luxembourg National Railway Company
    ///
    /// Luxembourg National Railway Company.
    #[serde(rename = "43")]
    _43,
    /// IT, Italian State Railways
    ///
    /// Italian State Railways.
    #[serde(rename = "44")]
    _44,
    /// NL, Netherlands Railways
    ///
    /// Netherlands Railways.
    #[serde(rename = "45")]
    _45,
    /// CH, Swiss Federal Railways
    ///
    /// Swiss Federal Railways.
    #[serde(rename = "46")]
    _46,
    /// DK, Danish State Railways
    ///
    /// Danish State Railways.
    #[serde(rename = "47")]
    _47,
    /// FR, French National Railway Company
    ///
    /// French National Railway Company.
    #[serde(rename = "48")]
    _48,
    /// BE, Belgian National Railway Company
    ///
    /// Belgian National Railway Company.
    #[serde(rename = "49")]
    _49,
    /// PT, Portuguese Railways
    ///
    /// Portuguese Railways.
    #[serde(rename = "50")]
    _50,
    /// SK, Slovakian State Railways
    ///
    /// Slovakian State Railways.
    #[serde(rename = "51")]
    _51,
    /// IE, Irish Transport Company
    ///
    /// Irish Transport Company.
    #[serde(rename = "52")]
    _52,
    /// FIATA (International Federation of Freight Forwarders Associations)
    ///
    /// International Federation of Freight Forwarders Associations.
    #[serde(rename = "53")]
    _53,
    /// IMO (International Maritime Organisation)
    ///
    /// International Maritime Organisation.
    #[serde(rename = "54")]
    _54,
    /// US, DOT (United States Department of Transportation)
    ///
    /// United States Department of Transportation.
    #[serde(rename = "55")]
    _55,
    /// TW, Trade-van
    ///
    /// Trade-van is an EDI/VAN service centre for customs, transport, and insurance in national and international trade.
    #[serde(rename = "56")]
    _56,
    /// TW, Chinese Taipei Customs
    ///
    /// Customs authorities of Chinese Taipei responsible for collecting import duties and preventing smuggling.
    #[serde(rename = "57")]
    _57,
    /// EUROFER
    ///
    /// European steel organisation - EDI project for the European steel industry.
    #[serde(rename = "58")]
    _58,
    /// DE, EDIBAU
    ///
    /// National body responsible for the German codification in the construction area.
    #[serde(rename = "59")]
    _59,
    /// Assigned by national trade agency
    ///
    /// The code list is from a national agency.
    #[serde(rename = "60")]
    _60,
    /// Association Europeenne des Constructeurs de Materiel Aerospatial (AECMA)
    ///
    /// A code to identify the Association Europeenne des Constructeurs de Materiel Aeropsatial (European Association of Aerospace Products Manufacturers) as an authorizing agency for code lists.
    #[serde(rename = "61")]
    _61,
    /// US, DIstilled Spirits Council of the United States (DISCUS)
    ///
    /// United States DIstilled Spirits Council of the United States (DISCUS).
    #[serde(rename = "62")]
    _62,
    /// North Atlantic Treaty Organization (NATO)
    ///
    /// A code to identify the North Atlantic Treaty Organization (NATO) as an authorizing agency for code lists.
    #[serde(rename = "63")]
    _63,
    /// FR, EDIFRANCE
    ///
    /// French association responsible for coordination and promotion of EDI application in France.
    #[serde(rename = "64")]
    _64,
    /// FR, GENCOD
    ///
    /// French organization responsible for EDI and Barcoding application in the retail sector.
    #[serde(rename = "65")]
    _65,
    /// MY, Malaysian Customs and Excise
    ///
    /// Malaysia Royal Customs and Excise.
    #[serde(rename = "66")]
    _66,
    /// MY, Malaysia Central Bank
    ///
    /// Malaysia Central Bank is a regulatory body set up by the government to charge with promoting economic monetary and credit condition favourable to commercial and industrial activity.
    #[serde(rename = "67")]
    _67,
    /// US, Bureau of Alcohol, Tobacco and Firearms (BATF)
    ///
    /// United States Bureau of Alcohol, Tobacco and Firearms (BATF).
    #[serde(rename = "68")]
    _68,
    /// US, National Alcohol Beverage Control Association (NABCA)
    ///
    /// United States National Alcohol Beverage Control Association (NABCA).
    #[serde(rename = "69")]
    _69,
    /// MY, Dagang.Net
    ///
    /// Malaysia, Dagang.Net is a national clearing house which provide EDI/VAN service for customs, transport, retail and financial and other industries in the national and international trade.
    #[serde(rename = "70")]
    _70,
    /// US, FCC (Federal Communications Commission)
    ///
    /// A code representing the United States Federal Communication Commission (FCC).
    #[serde(rename = "71")]
    _71,
    /// US, MARAD (Maritime Administration)
    ///
    /// A code representing the United States Maritime Administration (MARAD) under the Department of Transportation (DOT).
    #[serde(rename = "72")]
    _72,
    /// US, DSAA (Defense Security Assistance Agency)
    ///
    /// A code representing the United States Defense Security Assistance Agency (DSAA) under the Department of Defense (DOD).
    #[serde(rename = "73")]
    _73,
    /// US, NRC (Nuclear Regulatory Commission)
    ///
    /// A code representing the United States Nuclear Regulatory Commission (NRC).
    #[serde(rename = "74")]
    _74,
    /// US, ODTC (Office of Defense Trade Controls)
    ///
    /// A code representing the United States Office of Defense Trade Controls (ODTC) under the Department of State.
    #[serde(rename = "75")]
    _75,
    /// US, ATF (Bureau of Alcohol, Tobacco and Firearms)
    ///
    /// A code representing the United States Bureau of Alcohol, Tobacco and Firearms, Department of Treasury (ATF).
    #[serde(rename = "76")]
    _76,
    /// US, BXA (Bureau of Export Administration)
    ///
    /// A code representing the United States Bureau of Export Administration (BXA) under the Department of Commerce (DOC) .
    #[serde(rename = "77")]
    _77,
    /// US, FWS (Fish and Wildlife Service)
    ///
    /// A code depicting the United States Fish and Wildlife Service (FWS).
    #[serde(rename = "78")]
    _78,
    /// US, OFAC (Office of Foreign Assets Control)
    ///
    /// A code representing the United States Office of Foreign Assets Controls (OFAC).
    #[serde(rename = "79")]
    _79,
    /// BRMA/RAA - LIMNET - RINET Joint Venture
    ///
    /// Joint venture between BRMA (Brokers & Reinsurance Markets Association) / RAA (Reinsurance Association of America) - LIMNET (London Insurance Market Network) - RINET (Reinsurance and Insurance Network).
    #[serde(rename = "80")]
    _80,
    /// RU, (SFT) Society for Financial Telecommunications
    ///
    /// Russian company representing the users of the Global Financial Telecommunication Network (GFTN).
    #[serde(rename = "81")]
    _81,
    /// NO, Enhetsregisteret ved Bronnoysundregisterne
    ///
    /// The co-ordinating register for companies and business units of companies at the Bronnoysund register centre.
    #[serde(rename = "82")]
    _82,
    /// US, National Retail Federation
    ///
    /// The National Retail Federation is the trade association for the general merchandise retailing industry. In addition to providing support and education services, they also maintain and publish standard colour and size codes for the retail industry.
    #[serde(rename = "83")]
    _83,
    /// DE, BRD (Gesetzgeber der Bundesrepublik Deutschland)
    ///
    /// German legislature.
    #[serde(rename = "84")]
    _84,
    /// North America, Telecommunications Industry Forum
    ///
    /// Trade association representing telecommunications service providers, equipment manufacturers, suppliers to the industry and customers.
    #[serde(rename = "85")]
    _85,
    /// Assigned by party originating the message
    ///
    /// Codes assigned by the party originating the message.
    #[serde(rename = "86")]
    _86,
    /// Assigned by carrier
    ///
    /// Codes assigned by the carrier.
    #[serde(rename = "87")]
    _87,
    /// Assigned by owner of operation
    ///
    /// Assigned by owner of operation (e.g. used in construction).
    #[serde(rename = "88")]
    _88,
    /// Assigned by distributor
    ///
    #[serde(rename = "89")]
    _89,
    /// Assigned by manufacturer
    ///
    #[serde(rename = "90")]
    _90,
    /// Assigned by seller or seller's agent
    ///
    #[serde(rename = "91")]
    _91,
    /// Assigned by buyer or buyer's agent
    ///
    #[serde(rename = "92")]
    _92,
    /// AT, Austrian Customs
    ///
    #[serde(rename = "93")]
    _93,
    /// AT, Austrian PTT
    ///
    #[serde(rename = "94")]
    _94,
    /// AU, Australian Customs Service
    ///
    /// Australian Customs Service.
    #[serde(rename = "95")]
    _95,
    /// CA, Revenue Canada, Customs and Excise
    ///
    #[serde(rename = "96")]
    _96,
    /// CH, Administration federale des contributions
    ///
    /// Indirect taxation (e.g. turn-over/sales taxes).
    #[serde(rename = "97")]
    _97,
    /// CH, Direction generale des douanes
    ///
    /// Customs (incl. ISO alpha 2 country code).
    #[serde(rename = "98")]
    _98,
    /// CH, Division des importations et exportations, OFAEE
    ///
    /// Import and export licences.
    #[serde(rename = "99")]
    _99,
    /// CH, Entreprise des PTT
    ///
    /// Telephone (voice/data) + telex numbers, postcodes, postal account numbers.
    #[serde(rename = "100")]
    _100,
    /// CH, Carbura
    ///
    /// Centrale suisse pour l'importation de carburants et combustibles liquides (Oil products).
    #[serde(rename = "101")]
    _101,
    /// CH, Centrale suisse pour l'importation du charbon
    ///
    /// Coal.
    #[serde(rename = "102")]
    _102,
    /// CH, Office fiduciaire des importateurs de denrees alimentaires
    ///
    /// Foodstuff.
    #[serde(rename = "103")]
    _103,
    /// CH, Association suisse code des articles
    ///
    /// Swiss article numbering association.
    #[serde(rename = "104")]
    _104,
    /// DK, Ministry of taxation, Central Customs and Tax Administration
    ///
    /// Danish Customs administration.
    #[serde(rename = "105")]
    _105,
    /// FR, Direction generale des douanes et droits indirects
    ///
    /// French Customs.
    #[serde(rename = "106")]
    _106,
    /// FR, INSEE
    ///
    /// Institut National de la Statistique et des Etudes Economiques.
    #[serde(rename = "107")]
    _107,
    /// FR, Banque de France
    ///
    #[serde(rename = "108")]
    _108,
    /// GB, H.M. Customs & Excise
    ///
    #[serde(rename = "109")]
    _109,
    /// IE, Revenue Commissioners, Customs AEP project
    ///
    #[serde(rename = "110")]
    _110,
    /// US, U.S. Customs Service
    ///
    #[serde(rename = "111")]
    _111,
    /// US, U.S. Census Bureau
    ///
    /// The Bureau of the Census of the U.S. Dept. of Commerce.
    #[serde(rename = "112")]
    _112,
    /// US, UCC (Uniform Code Council)
    ///
    /// The Uniform Code Council (UCC) is a not-for-profit organization which manages and administers EDI and product bar code standards for the U.S. retail industry. The UCC also maintains U.P.C. manufacturer identifiers, EDI communications identifiers and various EDI code lists specific to retailing. The UCC is located in Dayton, OH, USA.
    #[serde(rename = "113")]
    _113,
    /// US, ABA (American Bankers Association)
    ///
    #[serde(rename = "114")]
    _114,
    /// US, DODAAC (Department Of Defense Active Agency Code)
    ///
    #[serde(rename = "115")]
    _115,
    /// US, ANSI ASC X12
    ///
    /// American National Standards Institute ASC X12.
    #[serde(rename = "116")]
    _116,
    /// AT, Geldausgabeautomaten-Service Gesellschaft m.b.H.
    ///
    /// Description to be provided.
    #[serde(rename = "117")]
    _117,
    /// SE, Svenska Bankfoereningen
    ///
    /// Swedish bankers association.
    #[serde(rename = "118")]
    _118,
    /// IT, Associazione Bancaria Italiana
    ///
    #[serde(rename = "119")]
    _119,
    /// IT, Socieata' Interbancaria per l'Automazione
    ///
    #[serde(rename = "120")]
    _120,
    /// CH, Telekurs AG
    ///
    #[serde(rename = "121")]
    _121,
    /// CH, Swiss Securities Clearing Corporation
    ///
    #[serde(rename = "122")]
    _122,
    /// NO, Norwegian Interbank Research Organization
    ///
    #[serde(rename = "123")]
    _123,
    /// NO, Norwegian Bankers' Association
    ///
    #[serde(rename = "124")]
    _124,
    /// FI, The Finnish Bankers' Association
    ///
    #[serde(rename = "125")]
    _125,
    /// US, NCCMA (Account Analysis Codes)
    ///
    #[serde(rename = "126")]
    _126,
    /// DE, ARE (AbRechnungs Einheit)
    ///
    /// A German code for subsidiary unit number.
    #[serde(rename = "127")]
    _127,
    /// BE, Belgian Bankers' Association
    ///
    #[serde(rename = "128")]
    _128,
    /// BE, Belgian Ministry of Finance
    ///
    /// VAT numbers.
    #[serde(rename = "129")]
    _129,
    /// DK, PBS (Pengainstitutternes Betalings Service)
    ///
    #[serde(rename = "130")]
    _130,
    /// DE, German Bankers Association
    ///
    #[serde(rename = "131")]
    _131,
    /// GB, BACS Limited
    ///
    #[serde(rename = "132")]
    _132,
    /// GB, Association for Payment Clearing Services
    ///
    #[serde(rename = "133")]
    _133,
    /// GB, CHAPS and Town Clearing Company Limited
    ///
    #[serde(rename = "134")]
    _134,
    /// GB, The Clearing House
    ///
    #[serde(rename = "135")]
    _135,
    /// GB, Article Number Association (UK) Limited
    ///
    /// EAN bar-coding.
    #[serde(rename = "136")]
    _136,
    /// AT, Verband oesterreichischer Banken und Bankiers
    ///
    /// Austrian bankers association.
    #[serde(rename = "137")]
    _137,
    /// FR, CFONB (Comite francais d'organ. et de normalisation bancaires)
    ///
    /// National body responsible for the French codification in banking activity.
    #[serde(rename = "138")]
    _138,
    /// UPU (Universal Postal Union)
    ///
    /// (a..3 country code).
    #[serde(rename = "139")]
    _139,
    /// CEC (Commission of the European Communities), DG/XXI-01
    ///
    /// (Computerization within Customs area).
    #[serde(rename = "140")]
    _140,
    /// CEC (Commission of the European Communities), DG/XXI-B-1
    ///
    /// Description to be provided.
    #[serde(rename = "141")]
    _141,
    /// CEC (Commission of the European Communities), DG/XXXIV
    ///
    /// Statistical Office of the European Communities: e.g. Geonomenclature.
    #[serde(rename = "142")]
    _142,
    /// NZ, New Zealand Customs
    ///
    #[serde(rename = "143")]
    _143,
    /// NL, Netherlands Customs
    ///
    #[serde(rename = "144")]
    _144,
    /// SE, Swedish Customs
    ///
    #[serde(rename = "145")]
    _145,
    /// DE, German Customs
    ///
    #[serde(rename = "146")]
    _146,
    /// BE, Belgian Customs
    ///
    #[serde(rename = "147")]
    _147,
    /// ES, Spanish Customs
    ///
    #[serde(rename = "148")]
    _148,
    /// IL, Israel Customs
    ///
    #[serde(rename = "149")]
    _149,
    /// HK, Hong Kong Customs
    ///
    #[serde(rename = "150")]
    _150,
    /// JP, Japan Customs
    ///
    #[serde(rename = "151")]
    _151,
    /// SA, Saudi Arabia Customs
    ///
    #[serde(rename = "152")]
    _152,
    /// IT, Italian Customs
    ///
    #[serde(rename = "153")]
    _153,
    /// GR, Greek Customs
    ///
    #[serde(rename = "154")]
    _154,
    /// PT, Portuguese Customs
    ///
    #[serde(rename = "155")]
    _155,
    /// LU, Luxembourg Customs
    ///
    #[serde(rename = "156")]
    _156,
    /// NO, Norwegian Customs
    ///
    #[serde(rename = "157")]
    _157,
    /// FI, Finnish Customs
    ///
    #[serde(rename = "158")]
    _158,
    /// IS, Iceland Customs
    ///
    #[serde(rename = "159")]
    _159,
    /// LI, Liechtenstein authority
    ///
    /// (Identification of relevant responsible agency for e.g. banking/financial matters still pending. For e.g. Customs, currency, post/telephone: see relevant CH entry).
    #[serde(rename = "160")]
    _160,
    /// UNCTAD (United Nations - Conference on Trade And Development)
    ///
    #[serde(rename = "161")]
    _161,
    /// CEC (Commission of the European Communities), DG/XIII-D-5
    ///
    /// (TEDIS - incl. CEBIS -, INSIS and CADDIA projects).
    #[serde(rename = "162")]
    _162,
    /// US, FMC (Federal Maritime Commission)
    ///
    #[serde(rename = "163")]
    _163,
    /// US, DEA (Drug Enforcement Agency)
    ///
    #[serde(rename = "164")]
    _164,
    /// US, DCI (Distribution Codes, INC.)
    ///
    #[serde(rename = "165")]
    _165,
    /// US, National Motor Freight Classification Association
    ///
    /// The organisation in the USA which is responsible for code maintenance in the trucking industry.
    #[serde(rename = "166")]
    _166,
    /// US, AIAG (Automotive Industry Action Group)
    ///
    #[serde(rename = "167")]
    _167,
    /// US, FIPS (Federal Information Publishing Standard)
    ///
    #[serde(rename = "168")]
    _168,
    /// CA, SCC (Standards Council of Canada)
    ///
    #[serde(rename = "169")]
    _169,
    /// CA, CPA (Canadian Payment Association)
    ///
    #[serde(rename = "170")]
    _170,
    /// NL, Interpay Girale Services
    ///
    /// Interpay Girale Services.
    #[serde(rename = "171")]
    _171,
    /// NL, Interpay Debit Card Services
    ///
    /// Interpay Debit Card Services.
    #[serde(rename = "172")]
    _172,
    /// NO, NORPRO
    ///
    #[serde(rename = "173")]
    _173,
    /// DE, DIN (Deutsches Institut fuer Normung)
    ///
    /// German standardization institute.
    #[serde(rename = "174")]
    _174,
    /// FCI (Factors Chain International)
    ///
    #[serde(rename = "175")]
    _175,
    /// BR, Banco Central do Brazil
    ///
    /// Self-explanatory.
    #[serde(rename = "176")]
    _176,
    /// AU, LIFA (Life Insurance Federation of Australia)
    ///
    /// Life Insurance Federation of Australia.
    #[serde(rename = "177")]
    _177,
    /// AU, SAA (Standards Association of Australia)
    ///
    /// Standards Association of Australia.
    #[serde(rename = "178")]
    _178,
    /// US, Air transport association of America
    ///
    /// U.S. -based trade association representing the major North American scheduled airlines.
    #[serde(rename = "179")]
    _179,
    /// DE, BIA (Berufsgenossenschaftliches Institut fuer Arbeitssicherheit)
    ///
    /// German institute of the workmen's compensation board.
    #[serde(rename = "180")]
    _180,
    /// Edibuild
    ///
    /// EDI organization for companies in the construction industry.
    #[serde(rename = "181")]
    _181,
    /// US, Standard Carrier Alpha Code (Motor)
    ///
    /// Organisation maintaining the SCAC lists and transportation operating in North America.
    #[serde(rename = "182")]
    _182,
    /// US, American Petroleum Institute
    ///
    /// US-based trade association representing oil and natural gas producers, shippers, refineries, marketers, and major suppliers to the industry.
    #[serde(rename = "183")]
    _183,
    /// AU, ACOS (Australian Chamber of Shipping)
    ///
    /// The national organisation for the maritime industry in Australia.
    #[serde(rename = "184")]
    _184,
    /// DE, BDI (Bundesverband der Deutschen Industrie e.V.)
    ///
    /// German industry association.
    #[serde(rename = "185")]
    _185,
    /// US, GSA (General Services Administration)
    ///
    /// The US General Services Administration.
    #[serde(rename = "186")]
    _186,
    /// US, DLMSO (Defense Logistics Management Standards Office)
    ///
    /// The Defense Logistics Management Standards Office.
    #[serde(rename = "187")]
    _187,
    /// US, NIST (National Institute of Standards and Technology)
    ///
    /// The US National Institute of Standards and Technology.
    #[serde(rename = "188")]
    _188,
    /// US, DoD (Department of Defense)
    ///
    /// The US Department of Defense.
    #[serde(rename = "189")]
    _189,
    /// US, VA (Department of Veterans Affairs)
    ///
    /// The Department of Veterans Affairs.
    #[serde(rename = "190")]
    _190,
    /// IAPSO (United Nations Inter-Agency Procurement Services Office)
    ///
    /// United Nations organization responsible for maintaining the United Nations Common Coding System (UNCCS) which is used extensively by UN agencies in procurement and statistical analysis.
    #[serde(rename = "191")]
    _191,
    /// Shipper's association
    ///
    /// Code assigned by a shipper's association.
    #[serde(rename = "192")]
    _192,
    /// EU, European Telecommunications Informatics Services (ETIS)
    ///
    /// European Telecommunications Informatics Services is a non-profit cooperative organisation owned by European public network operators, working in the field of information technology.
    #[serde(rename = "193")]
    _193,
    /// AU, AQIS (Australian Quarantine and Inspection Service)
    ///
    /// Australian Quarantine and Inspection Service.
    #[serde(rename = "194")]
    _194,
    /// CO, DIAN (Direccion de Impuestos y Aduanas Nacionales)
    ///
    /// The Colombian customs organization.
    #[serde(rename = "195")]
    _195,
    /// US, COPAS (Council of Petroleum Accounting Society)
    ///
    /// Organization supplying codes of oil field equipment and tubular goods used by joint operators in the petroleum industry.
    #[serde(rename = "196")]
    _196,
    /// US, DISA (Data Interchange Standards Association)
    ///
    /// The organization maintaining code lists under the administration of the data interchange standards association.
    #[serde(rename = "197")]
    _197,
    /// CO, Superintendencia Bancaria De Colombia
    ///
    /// The organization which assigns identification numbers to financial institutions conducting business in Colombia.
    #[serde(rename = "198")]
    _198,
    /// FR, Direction de la Comptabilite Publique
    ///
    /// The French public accounting office.
    #[serde(rename = "199")]
    _199,
    /// NL, EAN Netherlands
    ///
    /// Netherlands based European Article Numbering association (EAN).
    #[serde(rename = "200")]
    _200,
    /// US, WSSA(Wine and Spirits Shippers Association)
    ///
    /// United States based Wine and Spirits Shippers association.
    #[serde(rename = "201")]
    _201,
    /// PT, Banco de Portugal
    ///
    /// Portuguese Central Bank.
    #[serde(rename = "202")]
    _202,
    /// FR, GALIA (Groupement pour l'Amelioration des Liaisons dans l'Industrie Automobile)
    ///
    /// The national organisation representing France in ODETTE (Organisation for Data Exchanges through Tele- Transmission in Europe).
    #[serde(rename = "203")]
    _203,
    /// DE, VDA (Verband der Automobilindustrie E.V.)
    ///
    /// The national organisation representing Germany in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[serde(rename = "204")]
    _204,
    /// IT, ODETTE Italy
    ///
    /// The national organisation representing Italy in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[serde(rename = "205")]
    _205,
    /// NL, ODETTE Netherlands
    ///
    /// The national organisation representing Netherlands in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[serde(rename = "206")]
    _206,
    /// ES, ODETTE Spain
    ///
    /// The national organisation representing Spain in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[serde(rename = "207")]
    _207,
    /// SE, ODETTE Sweden
    ///
    /// The national organisation representing Scandinavian countries in ODETTE (Organisation for Data Exchange through Tele-Transmission in Europe).
    #[serde(rename = "208")]
    _208,
    /// GB, ODETTE United Kingdom
    ///
    /// The national organisation representing UK in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[serde(rename = "209")]
    _209,
    /// EU, EDI for financial, informational, cost, accounting, auditing and social areas (EDIFICAS) - Europe
    ///
    /// European association dealing with accounting and auditing.
    #[serde(rename = "210")]
    _210,
    /// FR, EDI for financial, informational, cost, accounting, auditing and social areas (EDIFICAS) - France
    ///
    /// French association dealing with accounting and auditing.
    #[serde(rename = "211")]
    _211,
    /// DE, Deutsch Telekom AG
    ///
    /// German telecommunication services agency.
    #[serde(rename = "212")]
    _212,
    /// JP, NACCS Center (Nippon Automated Cargo Clearance System Operations Organization)
    ///
    /// NACCS (Nippon Automated Cargo Clearance System Operation Organization) Center is the operations organization of the automated cargo clearance system in Japan.
    #[serde(rename = "213")]
    _213,
    /// US, AISI (American Iron and Steel Institute)
    ///
    /// American iron and steel institute.
    #[serde(rename = "214")]
    _214,
    /// AU, APCA (Australian Payments Clearing Association)
    ///
    /// Australian association responsible for the management of payment clearing.
    #[serde(rename = "215")]
    _215,
    /// US, Department of Labor
    ///
    /// To identify the United States department of labour.
    #[serde(rename = "216")]
    _216,
    /// US, N.A.I.C. (National Association of Insurance Commissioners)
    ///
    /// To identify the United States, National Association of Insurance Commissioners.
    #[serde(rename = "217")]
    _217,
    /// GB, The Association of British Insurers
    ///
    /// An association that administers code lists on behalf of the UK insurance community.
    #[serde(rename = "218")]
    _218,
    /// FR, d'ArvA
    ///
    /// Value added network administering insurance code lists on behalf of the French insurance community.
    #[serde(rename = "219")]
    _219,
    /// FI, Finnish tax board
    ///
    /// Finnish tax board.
    #[serde(rename = "220")]
    _220,
    /// FR, CNAMTS (Caisse Nationale de l'Assurance Maladie des Travailleurs Salaries)
    ///
    /// The French public institution funding health-care for salaried workers.
    #[serde(rename = "221")]
    _221,
    /// DK, Danish National Board of Health
    ///
    /// The national authority responsible for the supervision of health activities in Denmark.
    #[serde(rename = "222")]
    _222,
    /// DK, Danish Ministry of Home Affairs
    ///
    /// The ministry responsible for all interior affairs concerning the Danish people.
    #[serde(rename = "223")]
    _223,
    /// US, Aluminum Association
    ///
    /// Organization that assigns identification numbers for the aluminum industry.
    #[serde(rename = "224")]
    _224,
    /// US, CIDX (Chemical Industry Data Exchange)
    ///
    /// Organization that assigns identification numbers for the chemical Industry.
    #[serde(rename = "225")]
    _225,
    /// US, Carbide Manufacturers
    ///
    /// Organization that assigns identification numbers for the iron and carbide manufacturing industry.
    #[serde(rename = "226")]
    _226,
    /// US, NWDA (National Wholesale Druggist Association)
    ///
    /// Organization that assigns identification numbers for the wholesale drug industry.
    #[serde(rename = "227")]
    _227,
    /// US, EIA (Electronic Industry Association)
    ///
    /// Organization that assigns identification numbers for the electronic industry.
    #[serde(rename = "228")]
    _228,
    /// US, American Paper Institute
    ///
    /// Organization that assigns identification numbers for the American paper industry.
    #[serde(rename = "229")]
    _229,
    /// US, VICS (Voluntary Inter-Industry Commerce Standards)
    ///
    /// Organization that assigns identification numbers for the retail industry.
    #[serde(rename = "230")]
    _230,
    /// Copper and Brass Fabricators Council
    ///
    /// Organization that assigns identification numbers for the copper and brass fabricators industry.
    #[serde(rename = "231")]
    _231,
    /// GB, Inland Revenue
    ///
    /// Code identifying the government department responsible for assessing and collecting revenue consisting of taxes and inland duties in Great Britain.
    #[serde(rename = "232")]
    _232,
    /// US, OMB (Office of Management and Budget)
    ///
    /// Codes are assigned by the United States Office of Management and Budget.
    #[serde(rename = "233")]
    _233,
    /// DE, Siemens AG
    ///
    /// Siemens AG, Germany.
    #[serde(rename = "234")]
    _234,
    /// AU, Tradegate (Electronic Commerce Australia)
    ///
    /// Australian industry body coordinating codes for use in local and international commerce and trade.
    #[serde(rename = "235")]
    _235,
    /// US, United States Postal Service (USPS)
    ///
    /// Code specifying the official postal service of the United States.
    #[serde(rename = "236")]
    _236,
    /// US, United States health industry
    ///
    /// Code assigned by the United States health industry.
    #[serde(rename = "237")]
    _237,
    /// US, TDCC (Transportation Data Coordinating Committee)
    ///
    /// United States Transportation Data Coordinating Committee.
    #[serde(rename = "238")]
    _238,
    /// US, HL7 (Health Level 7)
    ///
    /// United States, electronic data interchange standards- making organization, Health Level 7.
    #[serde(rename = "239")]
    _239,
    /// US, CHIPS (Clearing House Interbank Payment Systems)
    ///
    /// United States financial clearing house.
    #[serde(rename = "240")]
    _240,
    /// PT, SIBS (Sociedade Interbancaria de Servicos)
    ///
    /// Portuguese automated clearing house.
    #[serde(rename = "241")]
    _241,
    /// NL, Interpay Giraal
    ///
    /// Interpay Giraal.
    #[serde(rename = "242")]
    _242,
    /// NL, Interpay Cards
    ///
    /// Interpay Cards.
    #[serde(rename = "243")]
    _243,
    /// US, Department of Health and Human Services
    ///
    /// United States Department of Health and Human Services.
    #[serde(rename = "244")]
    _244,
    /// DK, EAN (European Article Numbering) Denmark
    ///
    /// Denmark based European Article Numbering (EAN) association.
    #[serde(rename = "245")]
    _245,
    /// DE, Centrale fuer Coorganisation GMBH
    ///
    /// German representation of European Article Numbering (EAN) International.
    #[serde(rename = "246")]
    _246,
    /// US, HBICC (Health Industry Business Communication Council)
    ///
    /// Code identifying the United States HIBCC (Health Industry Business Communication Council).
    #[serde(rename = "247")]
    _247,
    /// US, ASTM (American Society of Testing and Materials)
    ///
    /// A not-for-profit organization that provides a forum for producers, users, ultimate consumers, and those having a general interest (representatives of government and academia) to meet on common ground and write standards for materials, products, systems, and services.
    #[serde(rename = "248")]
    _248,
    /// IP (Institute of Petroleum)
    ///
    /// An independent European centre for the advancement and dissemination of technical, economic and professional knowledge relating to the international oil and gas industry.
    #[serde(rename = "249")]
    _249,
    /// US, UOP (Universal Oil Products)
    ///
    /// An United States based organization that provides products, services and technology primarily in the areas of petroleum refining, olefins, aromatics, and gas processing.
    #[serde(rename = "250")]
    _250,
    /// AU, HIC (Health Insurance Commission)
    ///
    /// Australian agency responsible for administering the Health Insurance Act.
    #[serde(rename = "251")]
    _251,
    /// AU, AIHW (Australian Institute of Health and Welfare)
    ///
    /// Australian statutory authority responsible for the national collection of health related statistics and health related data definitions.
    #[serde(rename = "252")]
    _252,
    /// AU, NCCH (National Centre for Classification in Health)
    ///
    /// Australian national authority responsible for healthcare classifications.
    #[serde(rename = "253")]
    _253,
    /// AU, DOH (Australian Department of Health)
    ///
    /// Australian government department responsible for administration of health policy.
    #[serde(rename = "254")]
    _254,
    /// AU, ADA (Australian Dental Association)
    ///
    /// Industry association responsible for the classification of dental services in Australia.
    #[serde(rename = "255")]
    _255,
    /// US, AAR (Association of American Railroads)
    ///
    /// The official United States organization of the railroads in North America.
    #[serde(rename = "256")]
    _256,
    /// US, UN/SPSC (United Nations Standard Products and Services Classification) association
    ///
    /// The agency responsible for the maintenance of the United Nations standard products and services classification code.
    #[serde(rename = "257")]
    _257,
    /// JP, Japanese Ministry of Transport
    ///
    /// Japanese Ministry of Transport.
    #[serde(rename = "258")]
    _258,
    /// JP, Japanese Maritime Safety Agency
    ///
    /// Japanese Maritime Safety Agency.
    #[serde(rename = "259")]
    _259,
    /// Ediel Nordic forum
    ///
    /// A code to identify Ediel Nordic forum, which is an organization standardizing the use of EDI between the participants in the Nordic power market.
    #[serde(rename = "260")]
    _260,
    /// EEG7, European Expert Group 7 (Insurance)
    ///
    /// European Expert Group 7 for Insurance.
    #[serde(rename = "261")]
    _261,
    /// DE, GDV (Gesamtverband der Deutschen Versicherungswirtschaft e.V.)
    ///
    /// Gesamtverband der Deutschen Versicherungswirtschaft e.V. (German Insurance Association).
    #[serde(rename = "262")]
    _262,
    /// CA, CSIO (Centre for Study of Insurance Operations)
    ///
    /// The Centre for Study of Insurance Operations (CSIO) in Canada.
    #[serde(rename = "263")]
    _263,
    /// FR, AGF (Assurances Generales de France)
    ///
    /// Code lists are administered by Assurances Generales de France (AGF).
    #[serde(rename = "264")]
    _264,
    /// SE, Central bank
    ///
    /// Swedish central bank.
    #[serde(rename = "265")]
    _265,
    /// US, DoA (Department of Agriculture)
    ///
    /// Department of Agriculture, United States federal agency.
    #[serde(rename = "266")]
    _266,
    /// RU, Russian Bank Identification Code (BIC)
    ///
    /// BIC is used for party identification in the bank of Russia payment system and is a subdivision directory for the bank of Russia.
    #[serde(rename = "267")]
    _267,
    /// FR, DGI (Direction Generale des Impots)
    ///
    /// French taxation authority.
    #[serde(rename = "268")]
    _268,
    /// GRE (Reference Group of Experts)
    ///
    /// An international association that administers code lists on behalf of business credit information users and providers.
    #[serde(rename = "269")]
    _269,
    /// Concord EDI group
    ///
    /// An organisation of international transport equipment leasing companies and transport equipment repair providers responsible for promoting the use of EDI standards and standard business terms.
    #[serde(rename = "270")]
    _270,
    /// InterContainer InterFrigo
    ///
    /// European railway associated organisation involved in the transport of containers by rail.
    #[serde(rename = "271")]
    _271,
    /// Joint Automotive Industry agency
    ///
    /// The Joint Automotive Industry (JAI) agency is in charge of code lists that are common to automotive industry groups.
    #[serde(rename = "272")]
    _272,
    /// CH, SCC (Swiss Chambers of Commerce)
    ///
    /// Swiss Chambers of Commerce.
    #[serde(rename = "273")]
    _273,
    /// ITIGG (International Transport Implementation Guidelines Group)
    ///
    /// ITIGG is the UN/EDIFACT transport message development group's organisation responsible for the issuance of globally harmonised transport-related codes.
    #[serde(rename = "274")]
    _274,
    /// ES, Banco de España
    ///
    /// The Spanish central bank.
    #[serde(rename = "275")]
    _275,
    /// Mutually defined
    #[strum(ascii_case_insensitive)]
    ZZZ,
}

/// 4065 Contract and carriage condition code
///
/// Code to identify the conditions of contract and carriage.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _4065 {
    /// AVC conditions
    /// General conditions of transport 1983 latest revision laid down by the Stichting Vervoeradres The Hague.
    #[serde(rename = "1")]
    _1,
    /// Special agreement for parcels transport
    /// Appliance of a non published special agreement signed between a customer and the carrier (mandatory requested by the consignor) for parcels transport.
    #[serde(rename = "2")]
    _2,
    /// Special agreement for full loading transport
    /// Appliance of a non published special agreement signed between a customer and the carrier (mandatory requested by the consignor) for full load transport.
    #[serde(rename = "3")]
    _3,
    /// Combined transport
    /// A transport which involves more than one mode of transportation.
    #[serde(rename = "4")]
    _4,
    /// FIATA combined transport bill of lading
    /// Standard conditions of a combined transport bill of lading issued by FIATA.
    #[serde(rename = "5")]
    _5,
    /// Freight forwarders national conditions
    /// The contract and carriage conditions as established by freight forwarders on a national basis.
    #[serde(rename = "6")]
    _6,
    /// Normal tariff, parcels transport
    /// Appliance of the published legal tariff in case of parcels transport (required or not by the consignor.
    #[serde(rename = "7")]
    _7,
    /// Normal tariff, full loading transport
    /// Appliance of the published legal tariff in case of full load transport (required or not by the consignor).
    #[serde(rename = "8")]
    _8,
    /// Ordinary
    /// Carrier will choose the cheapest tariff in the legally published tariffs for parcels or full load transports (no tariff required by the consignor).
    #[serde(rename = "9")]
    _9,
    /// Port to port
    /// The transport will only be port to port, no inland transport would have to be provided under the contract.
    #[serde(rename = "10")]
    _10,
    /// CMR carnet
    /// Conditions in accordance with the convention of the contract for the international carriage of goods by road.
    #[serde(rename = "11")]
    _11,
    /// Special tariff, parcels transport
    /// Appliance of the legally published "special" tariff in case or parcels transport (tariff requested by the consignor).
    #[serde(rename = "12")]
    _12,
    /// Special tariff, full transport
    /// Appliance of the legally published "special tariff" in case of full load transport (tariff requested by the consignor).
    #[serde(rename = "13")]
    _13,
    /// Through transport
    /// The transport that is contracted not only from port to port, but from one inland location to another inland location.
    #[serde(rename = "14")]
    _14,
    /// Cancel space allocation
    /// Indication that space previously allocated on a flight is to be cancelled.
    #[serde(rename = "15")]
    _15,
    /// Report sale of space
    /// Indication that a sale has been made against a space allocation on a specific flight.
    #[serde(rename = "16")]
    _16,
    /// Alternative space allocation
    /// Indication that space is being requested for a specific flight and that an alternative is acceptable.
    #[serde(rename = "17")]
    _17,
    /// No alternative space allocation
    /// Indication that space is being requested for a specific flight and that an alternative is not acceptable.
    #[serde(rename = "18")]
    _18,
    /// Allotment sale
    /// Indication that space is being sold against a space allocation allotment on a specific flight.
    #[serde(rename = "19")]
    _19,
    /// Confirmation of space
    /// Indication that space requested has been confirmed on a specific flight.
    #[serde(rename = "20")]
    _20,
    /// Unable to confirm
    /// Indication that airline is unable to confirm the space allocation on a specific flight.
    #[serde(rename = "21")]
    _21,
    /// Non-operative flight
    /// Indication that airline is unable to confirm space on a specific flight since the flight does not operate.
    #[serde(rename = "22")]
    _22,
    /// Wait list
    /// Indication that the space allocation request has been assigned to a wait list.
    #[serde(rename = "23")]
    _23,
    /// Prior space allocation request
    /// Indication that a space allocation on a specific flight has already been requested.
    #[serde(rename = "24")]
    _24,
    /// Holding confirmed space allocation
    /// Indication that space is being held as confirmed on a specific flight.
    #[serde(rename = "25")]
    _25,
    /// Holding wait list
    /// Indication that space allocation request on a specific flight has been assigned to a wait list.
    #[serde(rename = "26")]
    _26,
    /// Door-to-door
    /// The carrier is responsible for the intermodal carriage of cargo including both the pre-carriage and the on- carriage.
    #[serde(rename = "27")]
    _27,
    /// Door-to-pier
    /// The carrier is responsible for the intermodal carriage of cargo including the pre-carriage, but excluding the on-carriage.
    #[serde(rename = "28")]
    _28,
    /// Pier-to-door
    /// The carrier is responsible for the intermodal carriage of cargo including the on-carriage, but excluding the pre-carriage.
    #[serde(rename = "29")]
    _29,
    /// Pier-to-pier
    /// The carrier of intermodal cargo is only responsible for the main carriage.
    #[serde(rename = "30")]
    _30,
    /// Space cancellation noted
    /// Indication that space previously allocated on a means of transport has been cancelled.
    #[serde(rename = "31")]
    _31,
    /// Mini landbridge service
    /// Cargo moving from a coastal port for delivery at an inland location or cargo received at an inland location moving to a coastal port for subsequent ocean transportation.
    #[serde(rename = "32")]
    _32,
    /// Space cancellation noted
    /// Indication that space previously allocated on a flight has been cancelled.
    #[serde(rename = "33")]
    _33,
    /// Speed level - required
    /// Maximum speed required on an itinerary or part of this itinerary to be able to assume some services.
    #[serde(rename = "34")]
    _34,
    /// Speed level - adopted
    /// Real speed used on an itinerary or part of this itinerary (for technical reasons, some limitation can be imposed or some higher speed could be used).
    #[serde(rename = "35")]
    _35,
    /// Normal tariff, less than full load transport
    /// Application of the published legal tariff in case of less than full load transport (required or not by the consignor).
    #[serde(rename = "36")]
    _36,
    /// Re-expedition special tariff
    /// Indication that a special tariff must be used in the case of a re-expedition.
    #[serde(rename = "37")]
    _37,
    /// Transport arrangement by the requester
    /// The service requester has the responsibility of making transport arrangement.
    #[serde(rename = "38")]
    _38,
    /// Transport arrangement by the provider
    /// The service provider has the responsibility of making transport arrangement.
    #[serde(rename = "39")]
    _39,
    /// Transport arrangement by the patient
    /// The patient has the responsibility of making transport arrangement.
    #[serde(rename = "40")]
    _40,
}

/// Transport service priority code
///
/// Code specifying the priority of a transport service.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _4219 {
    /// Express
    ///
    /// Express treatment (if by rail, legal express regime for parcels transport).
    #[serde(rename = "1")]
    _1,
    /// High speed
    ///
    /// Transport under legal international rail convention (CIM) concluded between rail organizations and based on fast routing and specified timetables.
    #[serde(rename = "2")]
    _2,
    /// Normal speed
    ///
    /// Transport under legal international rail convention (CIM) concluded between rail organizations.
    #[serde(rename = "3")]
    _3,
    /// Post service
    ///
    /// Transport under conditions specified by UPU (Universal Postal Union) and Rail organizations (parcels transport only).
    #[serde(rename = "4")]
    _4,
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

/// Cargo type classification code
///
/// Code specifying the classification of a type of cargo.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _7085 {
    #[serde(rename = "1")]
    /// Documents
    ///
    /// Printed, typed or written matter including leaflets, pamphlets, certificates etc., which are not subject to import duties and taxes, restrictions and prohibitions.
    _1,
    #[serde(rename = "2")]
    /// Low value non-dutiable consignments
    ///
    /// Imported consignments/items/goods in respect of which Customs duties and other taxes are waived as they are below a value determined by the Customs administration.
    _2,
    #[serde(rename = "3")]
    /// Low value dutiable consignments
    ///
    /// Imported consignments/items/goods in respect of which Customs duties and other taxes are payable are below a certain amount as determined by the Customs administration.
    _3,
    #[serde(rename = "4")]
    /// High value consignments
    ///
    /// Imported consignments/items/goods which are determined as having a value above a certain amount fixed by the Customs administration, which may or may not attract duties and taxes.
    _4,
    #[serde(rename = "5")]
    /// Other non-containerized
    ///
    /// Non-containerized cargo which cannot be categorized by any of the other nature of cargo code.
    _5,
    #[serde(rename = "6")]
    /// Vehicles
    ///
    /// Vehicles which are not stowed in containers.
    _6,
    #[serde(rename = "7")]
    /// Roll-on roll-off
    ///
    /// Cargo transported or to be transported on roll-on roll- off vessels and which is transportable on its own wheels or stowed on special heavy duty trailers.
    _7,
    #[serde(rename = "8")]
    /// Palletized
    ///
    /// Non-containerized cargo which is palletized.
    _8,
    #[serde(rename = "9")]
    /// Containerized
    ///
    /// Cargo stowed or to be stowed in a container.
    _9,
    #[serde(rename = "10")]
    /// Breakbulk
    ///
    /// Non-containerized cargo stowed in vessels' holds.
    _10,
    #[serde(rename = "11")]
    /// Hazardous cargo
    ///
    /// Cargo with dangerous properties, according to appropriate dangerous goods regulations.
    _11,
    #[serde(rename = "12")]
    /// General cargo
    ///
    /// Cargo of a general nature, not otherwise specified.
    _12,
    #[serde(rename = "13")]
    /// Liquid cargo
    ///
    /// Cargo in liquid form.
    _13,
    #[serde(rename = "14")]
    /// Temperature controlled cargo
    ///
    /// Cargo transported under specified temperature conditions.
    _14,
    #[serde(rename = "15")]
    /// Environmental pollutant cargo
    ///
    /// Cargo is an environmental pollutant.
    _15,
    #[serde(rename = "16")]
    /// Not-hazardous cargo
    ///
    /// Cargo which is not hazardous.
    _16,
    #[serde(rename = "17")]
    /// Diplomatic
    ///
    /// Cargo transported under diplomatic conditions.
    _17,
    #[serde(rename = "18")]
    /// Military
    ///
    /// Cargo for military purposes.
    _18,
    #[serde(rename = "19")]
    /// Obnoxious
    ///
    /// Cargo that is objectionable to human senses.
    _19,
    #[serde(rename = "20")]
    /// Out of gauge
    ///
    /// Cargo that has at least one non-standard dimension.
    _20,
    #[serde(rename = "21")]
    /// Household goods and personal effects
    ///
    /// Cargo consisting of household goods and personal effects.
    _21,
    #[serde(rename = "22")]
    /// Frozen cargo
    ///
    /// Cargo of frozen products.
    _22,
}

/// Service requirement code
///
/// Code specifying a service requirement.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum _7273 {
    #[serde(rename = "1")]
    /// Carrier loads
    ///
    /// The cargo is loaded in the equipment by the carrier.
    _1,
    #[serde(rename = "2")]
    /// Full loads
    ///
    /// Container to be stuffed or stripped under responsibility and for account of the shipper or the consignee.
    _2,
    #[serde(rename = "3")]
    /// Less than full loads
    ///
    /// Container to be stuffed and stripped for account and risk of the carrier.
    _3,
    #[serde(rename = "4")]
    /// Shipper loads
    ///
    /// The cargo is loaded in the equipment by the shipper.
    _4,
    #[serde(rename = "5")]
    /// To be delivered
    ///
    /// The cargo is to be delivered as instructed.
    _5,
    #[serde(rename = "6")]
    /// To be kept
    ///
    /// The cargo is to be retained awaiting further instructions.
    _6,
    #[serde(rename = "7")]
    /// Transhipment allowed
    ///
    /// Transhipment of goods is allowed.
    _7,
    #[serde(rename = "8")]
    /// Transhipment not allowed
    ///
    /// Transhipment of goods is not allowed.
    _8,
    #[serde(rename = "9")]
    /// Partial shipment allowed
    ///
    /// Partial shipment is allowed.
    _9,
    #[serde(rename = "10")]
    /// Partial shipment not allowed
    ///
    /// Partial shipment is not allowed.
    _10,
    #[serde(rename = "11")]
    /// Partial shipment and/or drawing allowed
    ///
    /// Partial shipment and/or drawing is allowed.
    _11,
    #[serde(rename = "12")]
    /// Partial shipment and/or drawing not allowed
    ///
    /// Partial shipment and/or drawing is not allowed.
    _12,
    #[serde(rename = "13")]
    /// Carrier unloads
    ///
    /// The cargo is to be unloaded from the equipment by the carrier.
    _13,
    #[serde(rename = "14")]
    /// Shipper unloads
    ///
    /// The cargo is to be unloaded from the equipment by the shipper.
    _14,
    #[serde(rename = "15")]
    /// Consignee unloads
    ///
    /// The cargo is to be unloaded from the equipment by the consignee.
    _15,
    #[serde(rename = "16")]
    /// Consignee loads
    ///
    /// The cargo is to be loaded in the equipment by the consignee.
    _16,
    #[serde(rename = "17")]
    /// Exclusive usage of equipment
    ///
    /// Usage of the equipment is reserved for exclusive use.
    _17,
    #[serde(rename = "18")]
    /// Non exclusive usage of equipment
    ///
    /// Usage of the equipment is not reserved for exclusive use.
    _18,
    #[serde(rename = "19")]
    /// Direct delivery
    ///
    /// Consignment for direct delivery to the consignee.
    _19,
    #[serde(rename = "20")]
    /// Direct pick-up
    ///
    /// Consignment for direct pick-up from the consignee.
    _20,
    #[serde(rename = "21")]
    /// Request for delivery advice services
    ///
    /// The service provider is requested to advise about delivery.
    _21,
    #[serde(rename = "22")]
    /// Do not arrange customs clearance
    ///
    /// Indication that the recipient of the message is not to arrange customs clearance.
    _22,
    #[serde(rename = "23")]
    /// Arrange customs clearance
    ///
    /// Indication that the recipient of the message is to arrange customs clearance.
    _23,
    #[serde(rename = "24")]
    /// Check container condition
    ///
    /// Condition of the container is to be checked.
    _24,
    #[serde(rename = "25")]
    /// Damaged containers allowed
    ///
    /// Damaged containers are allowed.
    _25,
    #[serde(rename = "26")]
    /// Dirty containers allowed
    ///
    /// Dirty containers are allowed.
    _26,
    #[serde(rename = "27")]
    /// Fork lift holes not required
    ///
    /// Container needs not to be equipped with pocket holes, but they are allowed.
    _27,
    #[serde(rename = "28")]
    /// Fork lift holes required
    ///
    /// Container must be equipped with pocket holes.
    _28,
    #[serde(rename = "29")]
    /// Insure goods during transport
    ///
    /// Indication that the recipient of the message is to insure the goods during transport.
    _29,
    #[serde(rename = "30")]
    /// Arrange main-carriage
    ///
    /// Indication that the recipient of the message is to arrange the main-carriage.
    _30,
    #[serde(rename = "31")]
    /// Arrange on-carriage
    ///
    /// Indication that the recipient of the message is to arrange the on-carriage.
    _31,
    #[serde(rename = "32")]
    /// Arrange pre-carriage
    ///
    /// Indication that the recipient of the message is to arrange the pre-carriage.
    _32,
    #[serde(rename = "33")]
    /// Report container safety convention information
    ///
    /// Indication that the information on the Container Safety Convention plate (CSC-plate) should be reported.
    _33,
    #[serde(rename = "34")]
    /// Check seals
    ///
    /// Sealing up of the container is to be checked.
    _34,
    #[serde(rename = "35")]
    /// Container must be clean
    ///
    /// Container is to be released or delivered clean.
    _35,
    #[serde(rename = "36")]
    /// Request for proof of delivery
    ///
    /// The service provider is requested to provide proof of delivery.
    _36,
    #[serde(rename = "37")]
    /// Request for Customs procedure
    ///
    /// The service provider is requested to perform Customs procedure.
    _37,
    #[serde(rename = "38")]
    /// Request for administration services
    ///
    /// The service provider is requested to perform administration services.
    _38,
    #[serde(rename = "39")]
    /// Transport insulated under Intercontainer INTERFRIGO conditions
    ///
    /// Insulated transport under Intercontainer INTERFRIGO (joint European railways agreement) conditions.
    _39,
    #[serde(rename = "40")]
    /// Transport mechanically refrigerated under Intercontainer INTERFRIGO conditions
    ///
    /// Mechanically refrigerated transport under Intercontainer INTERFRIGO (joint European railways agreement) conditions.
    _40,
    #[serde(rename = "41")]
    /// Cool or freeze service, not under Intercontainer INTERFRIGO conditions
    ///
    /// Cool or freeze service not under Intercontainer INTERFRIGO (joint European railways agreement) conditions.
    _41,
    #[serde(rename = "42")]
    /// Transhipment overseas
    ///
    /// Transport equipment is to be transferred overseas.
    _42,
    #[serde(rename = "43")]
    /// Station delivery
    ///
    /// The specified equipment destination station is also the place of delivery of the goods.
    _43,
    #[serde(rename = "44")]
    /// Non station delivery
    ///
    /// The specified equipment destination station is not the place of delivery of the goods.
    _44,
    #[serde(rename = "45")]
    /// Cleaning or disinfecting
    ///
    /// The service required is cleaning or disinfection.
    _45,
    #[serde(rename = "46")]
    /// Close ventilation valve
    ///
    /// The ventilation valve of the equipment must be closed.
    _46,
    #[serde(rename = "47")]
    /// Consignment held for pick-up
    ///
    /// The consignment is to be held until it is picked up.
    _47,
    #[serde(rename = "48")]
    /// Refrigeration unit check
    ///
    /// Refrigeration unit has to be checked.
    _48,
    #[serde(rename = "49")]
    /// Customs clearance at arrival country by carrier
    ///
    /// The carrier is to arrange customs clearance in the arrival country.
    _49,
    #[serde(rename = "50")]
    /// Customs clearance at departure country by carrier
    ///
    /// The carrier is to arrange customs clearance in the departure country.
    _50,
    #[serde(rename = "51")]
    /// Heating for live animals
    ///
    /// Heating for live animals has to be provided.
    _51,
    #[serde(rename = "52")]
    /// Goods humidification
    ///
    /// Humidification of the goods has to be performed.
    _52,
    #[serde(rename = "53")]
    /// Ensure load is secure
    ///
    /// The load must be checked for correct stowage.
    _53,
    #[serde(rename = "54")]
    /// Open ventilation valve
    ///
    /// The ventilation valve of the equipment must be opened.
    _54,
    #[serde(rename = "55")]
    /// Phytosanitary control
    ///
    /// Phytosanitary control to be performed.
    _55,
    #[serde(rename = "56")]
    /// Tare check by carrier
    ///
    /// Carrier must check the tare of the equipment and attached items.
    _56,
    #[serde(rename = "57")]
    /// Temperature check
    ///
    /// The temperature must be checked.
    _57,
    #[serde(rename = "58")]
    /// Weighing of goods
    ///
    /// The goods have to be weighed.
    _58,
    #[serde(rename = "59")]
    /// Escort
    ///
    /// An escort is required.
    _59,
    #[serde(rename = "60")]
    /// No escort
    ///
    /// An escort is not required.
    _60,
}
