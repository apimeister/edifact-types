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
pub enum _0002 {
    /// Version 1
    /// ISO 9735:1988.
    #[strum(serialize = "1")]
    _1,
    /// Version 2
    /// ISO 9735:1990.
    #[strum(serialize = "2")]
    _2,
    /// Version 3
    /// ISO 9735 Amendment 1:1992.
    #[default]
    #[strum(serialize = "3")]
    _3,
}

/// Partner identification code qualifier
///
/// Qualifier referring to the source of codes for the identifiers of interchanging partners.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0007 {
    /// DUNS (Data Universal Numbering System)
    ///
    /// Partner identification code assigned by Dun & Bradstreet.
    #[strum(serialize = "1")]
    _1,
    /// IATA (International Air Transport Association)
    ///
    /// Partner identification code assigned by the International Air Transport Association.
    #[strum(serialize = "4")]
    _4,
    /// INSEE (Institut National de la Statistique et des Etudes Economiques) - SIRET
    ///
    /// French national statistics institute. SIRET means Systeme Informatique du Repertoire des entreprises et de leurs ETablissements.
    #[strum(serialize = "5")]
    _5,
    /// UCC Communications ID (Uniform Code Council Communications Identifier)
    ///
    /// The Uniform Code Council Communications Identifier is a ten digit code used to uniquely identify physical and logical locations.
    #[strum(serialize = "8")]
    _8,
    /// DUNS (Data Universal Numbering System) with 4 digit suffix
    ///
    /// Partner identification code assigned by Dun & Bradstreet with the 4 digit suffix.
    #[strum(serialize = "9")]
    _9,
    /// Telephone number
    ///
    /// Partner identification code corresponds to the partner telephone number.
    #[strum(serialize = "12")]
    _12,
    /// EAN (European Article Numbering Association)
    ///
    /// Partner identification code assigned by the European Article Numbering Association.
    #[strum(serialize = "14")]
    _14,
    /// AIAG (Automotive Industry Action Group)
    ///
    /// Partner identification code assigned by the Automotive Industry Action Group.
    #[strum(serialize = "18")]
    _18,
    /// INSEE (Institut National de la Statistique et des Etudes Economiques) - SIREN
    ///
    /// French national statistics institute. SIREN means Systeme Informatique du Repertoire des ENtreprises (et de leurs etablissements).
    #[strum(serialize = "22")]
    _22,
    /// ISO 6523: Organization identification
    ///
    /// Partner identification code specified in ISO 6523 (Structures for the identification of organizations).
    #[strum(serialize = "30")]
    _30,
    /// DIN (Deutsches Institut fuer Normung)
    ///
    /// German standardization institute.
    #[strum(serialize = "31")]
    _31,
    /// BfA (Bundesversicherungsanstalt fuer Angestellte)
    ///
    /// German social security association.
    #[strum(serialize = "33")]
    _33,
    /// National Statistical Agency
    ///
    /// Partner identification code assigned by a national statistical agency.
    #[strum(serialize = "34")]
    _34,
    /// GEIS (General Electric Information Services)
    ///
    /// Partner identification code assigned by General Electric Information Services.
    #[strum(serialize = "51")]
    _51,
    /// INS (IBM Network Services)
    ///
    /// Partner identification code assigned by IBM Network Services.
    #[strum(serialize = "52")]
    _52,
    /// Datenzentrale des Einzelhandels
    ///
    /// German data centre for retail trade.
    #[strum(serialize = "53")]
    _53,
    /// Bundesverband der Deutschen Baustoffhaendler
    ///
    /// German building material trade association.
    #[strum(serialize = "54")]
    _54,
    /// Bank identifier code
    ///
    /// Partner identification code corresponds to the partner bank identification code.
    #[strum(serialize = "55")]
    _55,
    /// KTNet (Korea Trade Network Services)
    ///
    /// Partner identification code assigned by Korea Trade Network Services.
    #[strum(serialize = "57")]
    _57,
    /// UPU (Universal Postal Union)
    ///
    /// Partner identification code assigned by the Universal Postal Union.
    #[strum(serialize = "58")]
    _58,
    /// ODETTE (Organization for Data Exchange through Tele- Transmission in Europe)
    ///
    /// European automotive industry project.
    #[strum(serialize = "59")]
    _59,
    /// SCAC (Standard Carrier Alpha Code)
    ///
    /// Directory of standard multimodal carriers and tariff agent codes. The SCAC lists and codes transportation companies.
    #[strum(serialize = "61")]
    _61,
    /// ECA (Electronic Commerce Australia)
    ///
    /// Australian association for electronic commerce.
    #[strum(serialize = "63")]
    _63,
    /// TELEBOX 400 (Deutsche Telekom)
    ///
    /// German telecommunications service.
    #[strum(serialize = "65")]
    _65,
    /// NHS (National Health Service)
    ///
    /// United Kingdom National Health Service.
    #[strum(serialize = "80")]
    _80,
    /// Statens Teleforvaltning
    ///
    /// Norwegian telecommunications regulatory authority (NTRA).
    #[strum(serialize = "82")]
    _82,
    /// Athens Chamber of Commerce
    ///
    /// Greek Chamber of Commerce.
    #[strum(serialize = "84")]
    _84,
    /// Swiss Chamber of Commerce
    ///
    /// Swiss Chamber of Commerce.
    #[strum(serialize = "85")]
    _85,
    /// US Council for International Business
    ///
    /// United States Council for International Business.
    #[strum(serialize = "86")]
    _86,
    /// National Federation of Chambers of Commerce and Industry
    ///
    /// Belgium National Federation of Chambers of Commerce and Industry.
    #[strum(serialize = "87")]
    _87,
    /// Association of British Chambers of Commerce
    ///
    /// Association of British Chambers of Commerce.
    #[strum(serialize = "89")]
    _89,
    /// SITA (Societe Internationale de Telecommunications Aeronautiques)
    ///
    /// SITA (Societe Internationale de Telecommunications Aeronautiques).
    #[strum(serialize = "90")]
    _90,
    /// Assigned by seller or seller's agent
    ///
    /// Partner identification code assigned by the seller or seller's agent.
    #[strum(serialize = "91")]
    _91,
    /// Assigned by buyer or buyer's agent
    ///
    /// Partner identification code assigned by the buyer or buyer's agent.
    #[strum(serialize = "92")]
    _92,
    /// TW, Trade-van
    ///
    /// Trade-van is an EDI VAN service center for customs, transport, and insurance in national and international trade.
    #[strum(serialize = "103")]
    _103,
    /// BCNR (Telekurs Banken Clearing Number)
    ///
    /// Swiss national bank number assigned by Telekurs AG for the purpose of identifying a non-clearing banking institution.
    #[strum(serialize = "128")]
    _128,
    /// BPI (Telekurs Business Partner Identification)
    ///
    /// Swiss national business number assigned by Telekurs AG for the purpose of identifying a non-clearing banking institution.
    #[strum(serialize = "129")]
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
pub enum _0031 {
    /// Requested
    /// Acknowledgement is requested.
    #[strum(serialize = "1")]
    _1,
}

/// Test indicator
///
/// Indication that the interchange is a test.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq, Eq)]
pub enum _0035 {
    /// Interchange is a test
    /// Indicates that the interchange is a test.
    #[strum(serialize = "1")]
    _1,
}

/// Document name code
///
/// Code specifying the document name.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone, PartialEq)]
pub enum _1001 {
    /// Certificate of analysis
    ///
    /// Certificate providing the values of an analysis.
    #[strum(serialize = "1")]
    _1,
    /// Certificate of conformity
    ///
    /// Certificate certifying the conformity to predefined definitions.
    #[strum(serialize = "2")]
    _2,
    /// Certificate of quality
    ///
    /// Certificate certifying the quality of goods, services etc.
    #[strum(serialize = "3")]
    _3,
    /// Test report
    ///
    /// Report providing the results of a test session.
    #[strum(serialize = "4")]
    _4,
    /// Product performance report
    ///
    /// Report specifying the performance values of products.
    #[strum(serialize = "5")]
    _5,
    /// Product specification report
    ///
    /// Report providing specification values of products.
    #[strum(serialize = "6")]
    _6,
    /// Process data report
    ///
    /// Reports on events during production process.
    #[strum(serialize = "7")]
    _7,
    /// First sample test report
    ///
    ///
    #[strum(serialize = "8")]
    _8,
    /// Price/sales catalogue
    ///
    ///
    #[strum(serialize = "9")]
    _9,
    /// Party information
    ///
    /// Document/message providing basic data concerning a party.
    #[strum(serialize = "10")]
    _10,
    /// Federal label approval
    ///
    /// A pre-approved document relating to federal label approval requirements.
    #[strum(serialize = "11")]
    _11,
    /// Mill certificate
    ///
    /// Certificate certifying a specific quality of agricultural products.
    #[strum(serialize = "12")]
    _12,
    /// Post receipt
    ///
    /// Document/message which evidences the transport of goods by post (e.g. mail, parcel, etc.).
    #[strum(serialize = "13")]
    _13,
    /// Weight certificate
    ///
    /// Certificate certifying the weight of goods.
    #[strum(serialize = "14")]
    _14,
    /// Weight list
    ///
    /// Document/message specifying the weight of goods.
    #[strum(serialize = "15")]
    _15,
    /// Certificate
    ///
    /// Document by means of which the documentary credit applicant specifies the conditions for the certificate and by whom the certificate is to be issued.
    #[strum(serialize = "16")]
    _16,
    /// Combined certificate of value and origin
    ///
    /// Document identifying goods in which the issuing authority expressly certifies that the goods originate in a specific country or part of, or group of countries. It also states the price and/or cost of the goods with the purpose of determining the customs origin.
    #[strum(serialize = "17")]
    _17,
    /// Movement certificate A.TR.1
    ///
    /// Specific form of transit declaration issued by the exporter (movement certificate).
    #[strum(serialize = "18")]
    _18,
    /// Certificate of quantity
    ///
    /// Certificate certifying the quantity of goods, services etc.
    #[strum(serialize = "19")]
    _19,
    /// Quality data message
    ///
    /// Usage of QALITY-message.
    #[strum(serialize = "20")]
    _20,
    /// Query
    ///
    /// Request information based on defined criteria.
    #[strum(serialize = "21")]
    _21,
    /// Response to query
    ///
    /// Self-explanatory.
    #[strum(serialize = "22")]
    _22,
    /// Status information
    ///
    /// Information regarding the status of a related message.
    #[strum(serialize = "23")]
    _23,
    /// Restow
    ///
    /// Message/document identifying containers that have been unloaded and then reloaded onto the same means of transport.
    #[strum(serialize = "24")]
    _24,
    /// Container discharge list
    ///
    /// Message/document itemising containers to be discharged from vessel.
    #[strum(serialize = "25")]
    _25,
    /// Corporate superannuation contributions advice
    ///
    /// Document/message providing contributions advice used for corporate superannuation schemes.
    #[strum(serialize = "26")]
    _26,
    /// Industry superannuation contributions advice
    ///
    /// Document/message providing contributions advice used for superannuation schemes which are industry wide.
    #[strum(serialize = "27")]
    _27,
    /// Corporate superannuation member maintenance message
    ///
    /// Member maintenance message used for corporate superannuation schemes.
    #[strum(serialize = "28")]
    _28,
    /// Industry superannuation member maintenance message
    ///
    /// Member maintenance message used for industry wide superannuation schemes.
    #[strum(serialize = "29")]
    _29,
    /// Life insurance payroll deductions advice
    ///
    /// Payroll deductions advice used in the life insurance industry.
    #[strum(serialize = "30")]
    _30,
    /// Underbond request
    ///
    /// A Message/document requesting to move cargo from one Customs control point to another.
    #[strum(serialize = "31")]
    _31,
    /// Underbond approval
    ///
    /// A message/document issuing Customs approval to move cargo from one Customs control point to another.
    #[strum(serialize = "32")]
    _32,
    /// Certificate of sealing of export meat lockers
    ///
    /// Document / message issued by the authority in the exporting country evidencing the sealing of export meat lockers.
    #[strum(serialize = "33")]
    _33,
    /// Cargo status
    ///
    /// Message identifying the status of cargo.
    #[strum(serialize = "34")]
    _34,
    /// Inventory report
    ///
    /// A message specifying information relating to held inventories.
    #[strum(serialize = "35")]
    _35,
    /// Identity card
    ///
    /// Official document to identify a person.
    #[strum(serialize = "36")]
    _36,
    /// Response to a trade statistics message
    ///
    /// Document/message in which the competent national authorities provide a declarant with an acceptance or a rejection about a received declaration for European statistical purposes.
    #[strum(serialize = "37")]
    _37,
    /// Vaccination certificate
    ///
    /// Official document proving immunisation against certain diseases.
    #[strum(serialize = "38")]
    _38,
    /// Passport
    ///
    /// An official document giving permission to travel in foreign countries.
    #[strum(serialize = "39")]
    _39,
    /// Driving licence (national)
    ///
    /// An official document giving permission to drive a car in a given country.
    #[strum(serialize = "40")]
    _40,
    /// Driving licence (international)
    ///
    /// An official document giving a native of one country permission to drive a vehicle in certain other countries.
    #[strum(serialize = "41")]
    _41,
    /// Free pass
    ///
    /// A document giving free access to a service.
    #[strum(serialize = "42")]
    _42,
    /// Season ticket
    ///
    /// A document giving access to a service for a determined period of time.
    #[strum(serialize = "43")]
    _43,
    /// Transport status report
    ///
    /// A message to report the transport status and/or change in the transport status (i.e. event) between agreed parties.
    #[strum(serialize = "44")]
    _44,
    /// Transport status request
    ///
    /// A message to request a transport status report (e.g. through the International multimodal status report message IFSTA).
    #[strum(serialize = "45")]
    _45,
    /// Banking status
    ///
    /// A banking status document and/or message.
    #[strum(serialize = "46")]
    _46,
    /// Extra-Community trade statistical declaration
    ///
    /// Document/message in which a declarant provides information about extra-Community trade of goods required by the body responsible for the collection of trade statistics. Trade by a country in the European Union with a country outside the European Union.
    #[strum(serialize = "47")]
    _47,
    /// Written instructions in conformance with ADR article number 10385
    ///
    /// Written instructions relating to dangerous goods and defined in the European Agreement of Dangerous Transport by Road known as ADR (Accord europeen relatif au transport international des marchandises Dangereuses par Route).
    #[strum(serialize = "48")]
    _48,
    /// Damage certification
    ///
    /// Official certification that damages to the goods to be transported have been discovered.
    #[strum(serialize = "49")]
    _49,
    /// Validated priced tender
    ///
    /// A validated priced tender.
    #[strum(serialize = "50")]
    _50,
    /// Price/sales catalogue response
    ///
    /// A document providing a response to a previously sent price/sales catalogue.
    #[strum(serialize = "51")]
    _51,
    /// Price negotiation result
    ///
    /// A document providing the result of price negotiations.
    #[strum(serialize = "52")]
    _52,
    /// Safety and hazard data sheet
    ///
    /// Document or message to supply advice on a dangerous or hazardous material to industrial customers so as to enable them to take measures to protect their employees and the environment from any potential harmful effects from these material.
    #[strum(serialize = "53")]
    _53,
    /// Legal statement of an account
    ///
    /// A statement of an account containing the booked items as in the ledger of the account servicing financial institution.
    #[strum(serialize = "54")]
    _54,
    /// Listing statement of an account
    ///
    /// A statement from the account servicing financial institution containing items pending to be booked.
    #[strum(serialize = "55")]
    _55,
    /// Closing statement of an account
    ///
    /// Last statement of a period containing the interest calculation and the final balance of the last entry date.
    #[strum(serialize = "56")]
    _56,
    /// Transport equipment on-hire report
    ///
    /// Report on the movement of containers or other items of transport equipment to record physical movement activity and establish the beginning of a rental period.
    #[strum(serialize = "57")]
    _57,
    /// Transport equipment off-hire report
    ///
    /// Report on the movement of containers or other items of transport equipment to record physical movement activity and establish the end of a rental period.
    #[strum(serialize = "58")]
    _58,
    /// Treatment - nil outturn
    ///
    /// No shortage, surplus or damaged outturn resulting from container vessel unpacking.
    #[strum(serialize = "59")]
    _59,
    /// Treatment - time-up underbond
    ///
    /// Movement type indicator: goods are moved under customs control for warehousing due to being time-up.
    #[strum(serialize = "60")]
    _60,
    /// Treatment - underbond by sea
    ///
    /// Movement type indicator: goods are to move by sea under customs control to a customs office where formalities will be completed.
    #[strum(serialize = "61")]
    _61,
    /// Treatment - personal effect
    ///
    /// Cargo consists of personal effects.
    #[strum(serialize = "62")]
    _62,
    /// Treatment - timber
    ///
    /// Cargo consists of timber.
    #[strum(serialize = "63")]
    _63,
    /// Preliminary credit assessment
    ///
    /// Document/message issued either by a factor to indicate his preliminary credit assessment on a buyer, or by a seller to request a factor's preliminary credit assessment on a buyer.
    #[strum(serialize = "64")]
    _64,
    /// Credit cover
    ///
    /// Document/message issued either by a factor to give a credit cover on a buyer, or by a seller to request a factor's credit cover.
    #[strum(serialize = "65")]
    _65,
    /// Current account
    ///
    /// Document/message issued by a factor to indicate the money movements of a seller's or another factor's account with him.
    #[strum(serialize = "66")]
    _66,
    /// Commercial dispute
    ///
    /// Document/message issued by a party (usually the buyer) to indicate that one or more invoices or one or more credit notes are disputed for payment.
    #[strum(serialize = "67")]
    _67,
    /// Chargeback
    ///
    /// Document/message issued by a factor to a seller or to another factor to indicate that the rest of the amounts of one or more invoices uncollectable from buyers are charged back to clear the invoice(s) off the ledger.
    #[strum(serialize = "68")]
    _68,
    /// Reassignment
    ///
    /// Document/message issued by a factor to a seller or to another factor to reassign an invoice or credit note previously assigned to him.
    #[strum(serialize = "69")]
    _69,
    /// Collateral account
    ///
    /// Document message issued by a factor to indicate the movements of invoices, credit notes and payments of a seller's account.
    #[strum(serialize = "70")]
    _70,
    /// Request for payment
    ///
    /// Document/message issued by a creditor to a debtor to request payment of one or more invoices past due.
    #[strum(serialize = "71")]
    _71,
    /// Unship permit
    ///
    /// A message or document issuing permission to unship cargo.
    #[strum(serialize = "72")]
    _72,
    /// Statistical definitions
    ///
    /// Transmission of one or more statistical definitions.
    #[strum(serialize = "73")]
    _73,
    /// Statistical data
    ///
    /// Transmission of one or more items of data or data sets.
    #[strum(serialize = "74")]
    _74,
    /// Request for statistical data
    ///
    /// Request for one or more items or data sets of statistical data.
    #[strum(serialize = "75")]
    _75,
    /// Call-off delivery
    ///
    /// Document/message to provide split quantities and delivery dates referring to a previous delivery instruction.
    #[strum(serialize = "76")]
    _76,
    /// Consignment status report
    ///
    /// Message covers information about the consignment status.
    #[strum(serialize = "77")]
    _77,
    /// Inventory movement advice
    ///
    /// Advice of inventory movements.
    #[strum(serialize = "78")]
    _78,
    /// Inventory status advice
    ///
    /// Advice of stock on hand.
    #[strum(serialize = "79")]
    _79,
    /// Debit note related to goods or services
    ///
    /// Debit information related to a transaction for goods or services to the relevant party.
    #[strum(serialize = "80")]
    _80,
    /// Credit note related to goods or services
    ///
    /// Document message used to provide credit information related to a transaction for goods or services to the relevant party.
    #[strum(serialize = "81")]
    _81,
    /// Metered services invoice
    ///
    /// Document/message claiming payment for the supply of metered services (e.g., gas, electricity, etc.) supplied to a fixed meter whose consumption is measured over a period of time.
    #[strum(serialize = "82")]
    _82,
    /// Credit note related to financial adjustments
    ///
    /// Document message for providing credit information related to financial adjustments to the relevant party, e.g., bonuses.
    #[strum(serialize = "83")]
    _83,
    /// Debit note related to financial adjustments
    ///
    /// Document/message for providing debit information related to financial adjustments to the relevant party.
    #[strum(serialize = "84")]
    _84,
    /// Customs manifest
    ///
    /// Message/document identifying a customs manifest. The document itemises a list of cargo prepared by shipping companies from bills of landing and presented to customs for formal report of cargo.
    #[strum(serialize = "85")]
    _85,
    /// Vessel unpack report
    ///
    /// A document code to indicate that the message being transmitted identifies all short and surplus cargoes off-loaded from a vessel at a specified discharging port.
    #[strum(serialize = "86")]
    _86,
    /// General cargo summary manifest report
    ///
    /// A document code to indicate that the message being transmitted is summary manifest information for general cargo.
    #[strum(serialize = "87")]
    _87,
    /// Consignment unpack report
    ///
    /// A document code to indicate that the message being transmitted is a consignment unpack report only.
    #[strum(serialize = "88")]
    _88,
    /// Meat and meat by-products sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that meat or meat by- products comply with the requirements set by the importing country.
    #[strum(serialize = "89")]
    _89,
    /// Meat food products sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that meat food products comply with the requirements set by the importing country.
    #[strum(serialize = "90")]
    _90,
    /// Poultry sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that poultry products comply with the requirements set by the importing country.
    #[strum(serialize = "91")]
    _91,
    /// Horsemeat sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that horsemeat products comply with the requirements set by the importing country.
    #[strum(serialize = "92")]
    _92,
    /// Casing sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that casing products comply with the requirements set by the importing country.
    #[strum(serialize = "93")]
    _93,
    /// Pharmaceutical sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that pharmaceutical products comply with the requirements set by the importing country.
    #[strum(serialize = "94")]
    _94,
    /// Inedible sanitary certificate
    ///
    /// Document or message issued by the competent authority in the exporting country evidencing that inedible products comply with the requirements set by the importing country.
    #[strum(serialize = "95")]
    _95,
    /// Impending arrival
    ///
    /// Notification of impending arrival details for vessel.
    #[strum(serialize = "96")]
    _96,
    /// Means of transport advice
    ///
    /// Message reporting the means of transport used to carry goods or cargo.
    #[strum(serialize = "97")]
    _97,
    /// Arrival information
    ///
    /// Message reporting the arrival details of goods or cargo.
    #[strum(serialize = "98")]
    _98,
    /// Cargo release notification
    ///
    /// Message/document sent by the cargo handler indicating that the cargo has moved from a Customs controlled premise.
    #[strum(serialize = "99")]
    _99,
    /// Excise certificate
    ///
    /// Certificate asserting that the goods have been submitted to the excise authorities before departure from the exporting country or before delivery in case of import traffic.
    #[strum(serialize = "100")]
    _100,
    /// Registration document
    ///
    /// An official document providing registration details.
    #[strum(serialize = "101")]
    _101,
    /// Tax notification
    ///
    /// Used to specify that the message is a tax notification.
    #[strum(serialize = "102")]
    _102,
    /// Transport equipment direct interchange report
    ///
    /// Report on the movement of containers or other items of transport equipment being exchanged, establishing relevant rental periods.
    #[strum(serialize = "103")]
    _103,
    /// Transport equipment impending arrival advice
    ///
    /// Advice that containers or other items of transport equipment may be expected to be delivered to a certain location.
    #[strum(serialize = "104")]
    _104,
    /// Purchase order
    ///
    /// Document/message issued within an enterprise to initiate the purchase of articles, materials or services required for the production or manufacture of goods to be offered for sale or otherwise supplied to customers.
    #[strum(serialize = "105")]
    _105,
    /// Transport equipment damage report
    ///
    /// Report of damaged items of transport equipment that have been returned.
    #[strum(serialize = "106")]
    _106,
    /// Transport equipment maintenance and repair work estimate advice
    ///
    /// Advice providing estimates of transport equipment maintenance and repair costs.
    #[strum(serialize = "107")]
    _107,
    /// Transport equipment empty release instruction
    ///
    /// Instruction to release an item of empty transport equipment to a specified party or parties.
    #[strum(serialize = "108")]
    _108,
    /// Transport movement gate in report
    ///
    /// Report on the inward movement of cargo, containers or other items of transport equipment which have been delivered to a facility by an inland carrier.
    #[strum(serialize = "109")]
    _109,
    /// Manufacturing instructions
    ///
    /// Document/message issued within an enterprise to initiate the manufacture of goods to be offered for sale.
    #[strum(serialize = "110")]
    _110,
    /// Transport movement gate out report
    ///
    /// Report on the outward movement of cargo, containers or other items of transport equipment (either full or empty) which have been picked up by an inland carrier.
    #[strum(serialize = "111")]
    _111,
    /// Transport equipment unpacking instruction
    ///
    /// Instruction to unpack specified cargo from specified containers or other items of transport equipment.
    #[strum(serialize = "112")]
    _112,
    /// Transport equipment unpacking report
    ///
    /// Report on the completion of unpacking specified containers or other items of transport equipment.
    #[strum(serialize = "113")]
    _113,
    /// Transport equipment pick-up availability request
    ///
    /// Request for confirmation that an item of transport equipment will be available for collection.
    #[strum(serialize = "114")]
    _114,
    /// Transport equipment pick-up availability confirmation
    ///
    /// Confirmation that an item of transport equipment is available for collection.
    #[strum(serialize = "115")]
    _115,
    /// Transport equipment pick-up report
    ///
    /// Report that an item of transport equipment has been collected.
    #[strum(serialize = "116")]
    _116,
    /// Transport equipment shift report
    ///
    /// Report on the movement of containers or other items of transport within a facility.
    #[strum(serialize = "117")]
    _117,
    /// Transport discharge instruction
    ///
    /// Instruction to unload specified cargo, containers or transport equipment from a means of transport.
    #[strum(serialize = "118")]
    _118,
    /// Transport discharge report
    ///
    /// Report on cargo, containers or transport equipment unloaded from a particular means of transport.
    #[strum(serialize = "119")]
    _119,
    /// Stores requisition
    ///
    /// Document/message issued within an enterprise ordering the taking out of stock of goods.
    #[strum(serialize = "120")]
    _120,
    /// Transport loading instruction
    ///
    /// Instruction to load cargo, containers or transport equipment onto a means of transport.
    #[strum(serialize = "121")]
    _121,
    /// Transport loading report
    ///
    /// Report on completion of loading cargo, containers or other transport equipment onto a means of transport.
    #[strum(serialize = "122")]
    _122,
    /// Transport equipment maintenance and repair work authorisation
    ///
    /// Authorisation to have transport equipment repaired or to have maintenance performed.
    #[strum(serialize = "123")]
    _123,
    /// Transport departure report
    ///
    /// Report of the departure of a means of transport from a particular facility.
    #[strum(serialize = "124")]
    _124,
    /// Transport empty equipment advice
    ///
    /// Advice that an item or items of empty transport equipment are available for return.
    #[strum(serialize = "125")]
    _125,
    /// Transport equipment acceptance order
    ///
    /// Order to accept items of transport equipment which are to be delivered by an inland carrier (rail, road or barge) to a specified facility.
    #[strum(serialize = "126")]
    _126,
    /// Transport equipment special service instruction
    ///
    /// Instruction to perform a specified service or services on an item or items of transport equipment.
    #[strum(serialize = "127")]
    _127,
    /// Transport equipment stock report
    ///
    /// Report on the number of items of transport equipment stored at one or more locations.
    #[strum(serialize = "128")]
    _128,
    /// Transport cargo release order
    ///
    /// Order to release cargo or items of transport equipment to a specified party.
    #[strum(serialize = "129")]
    _129,
    /// Invoicing data sheet
    ///
    /// Document/message issued within an enterprise containing data about goods sold, to be used as the basis for the preparation of an invoice.
    #[strum(serialize = "130")]
    _130,
    /// Transport equipment packing instruction
    ///
    /// Instruction to pack cargo into a container or other item of transport equipment.
    #[strum(serialize = "131")]
    _131,
    /// Customs clearance notice
    ///
    /// Notification of customs clearance of cargo or items of transport equipment.
    #[strum(serialize = "132")]
    _132,
    /// Customs documents expiration notice
    ///
    /// Notice specifying expiration of Customs documents relating to cargo or items of transport equipment.
    #[strum(serialize = "133")]
    _133,
    /// Transport equipment on-hire request
    ///
    /// Request for transport equipment to be made available for hire.
    #[strum(serialize = "134")]
    _134,
    /// Transport equipment on-hire order
    ///
    /// Order to release empty items of transport equipment for on-hire to a lessee, and authorising collection by or on behalf of a specified party.
    #[strum(serialize = "135")]
    _135,
    /// Transport equipment off-hire request
    ///
    /// Request to terminate the lease on an item of transport equipment at a specified time.
    #[strum(serialize = "136")]
    _136,
    /// Transport equipment survey order
    ///
    /// Order to perform a survey on specified items of transport equipment.
    #[strum(serialize = "137")]
    _137,
    /// Transport equipment survey order response
    ///
    /// Response to an order to conduct a survey of transport equipment.
    #[strum(serialize = "138")]
    _138,
    /// Transport equipment survey report
    ///
    /// Survey report of specified items of transport equipment.
    #[strum(serialize = "139")]
    _139,
    /// Packing instructions
    ///
    /// Document/message within an enterprise giving instructions on how goods are to be packed.
    #[strum(serialize = "140")]
    _140,
    /// Advising items to be booked to a financial account
    ///
    /// A document and/or message advising of items which have to be booked to a financial account.
    #[strum(serialize = "141")]
    _141,
    /// Transport equipment maintenance and repair work estimate order
    ///
    /// Order to draw up an estimate of the costs of maintenance or repair of transport equipment.
    #[strum(serialize = "142")]
    _142,
    /// Transport equipment maintenance and repair notice
    ///
    /// Report of transport equipment which has been repaired or has had maintenance performed.
    #[strum(serialize = "143")]
    _143,
    /// Empty container disposition order
    ///
    /// Order to make available empty containers.
    #[strum(serialize = "144")]
    _144,
    /// Cargo vessel discharge order
    ///
    /// Order that the containers or cargo specified are to be discharged from a vessel.
    #[strum(serialize = "145")]
    _145,
    /// Cargo vessel loading order
    ///
    /// Order that specified cargo, containers or groups of containers are to be loaded in or on a vessel.
    #[strum(serialize = "146")]
    _146,
    /// Multidrop order
    ///
    /// One purchase order that contains the orders of two or more vendors and the associated delivery points for each.
    #[strum(serialize = "147")]
    _147,
    /// Bailment contract
    ///
    /// A document authorizing the bailing of goods.
    #[strum(serialize = "148")]
    _148,
    /// Basic agreement
    ///
    /// A document indicating an agreement containing basic terms and conditions applicable to future contracts between two parties.
    #[strum(serialize = "149")]
    _149,
    /// Internal transport order
    ///
    /// Document/message giving instructions about the transport of goods within an enterprise.
    #[strum(serialize = "150")]
    _150,
    /// Grant
    ///
    /// A document indicating the granting of funds.
    #[strum(serialize = "151")]
    _151,
    /// Indefinite delivery indefinite quantity contract
    ///
    /// A document indicating a contract calling for the indefinite deliveries of indefinite quantities of goods.
    #[strum(serialize = "152")]
    _152,
    /// Indefinite delivery definite quantity contract
    ///
    /// A document indicating a contract calling for indefinite deliveries of definite quantities.
    #[strum(serialize = "153")]
    _153,
    /// Requirements contract
    ///
    /// A document indicating a requirements contract that authorizes the filling of all purchase requirements during a specified contract period.
    #[strum(serialize = "154")]
    _154,
    /// Task order
    ///
    /// A document indicating an order that tasks a contractor to perform a specified function.
    #[strum(serialize = "155")]
    _155,
    /// Make or buy plan
    ///
    /// A document indicating a plan that identifies which items will be made and which items will be bought.
    #[strum(serialize = "156")]
    _156,
    /// Subcontractor plan
    ///
    /// A document indicating a plan that identifies the manufacturer's subcontracting strategy for a specific contract.
    #[strum(serialize = "157")]
    _157,
    /// Cost data summary
    ///
    /// A document indicating a summary of cost data.
    #[strum(serialize = "158")]
    _158,
    /// Certified cost and price data
    ///
    /// A document indicating cost and price data whose accuracy has been certified.
    #[strum(serialize = "159")]
    _159,
    /// Wage determination
    ///
    /// A document indicating a determination of the wages to be paid.
    #[strum(serialize = "160")]
    _160,
    /// Contract Funds Status Report (CFSR)
    ///
    /// A report to provide the status of funds applicable to the contract.
    #[strum(serialize = "161")]
    _161,
    /// Certified inspection and test results
    ///
    /// A certification as to the accuracy of inspection and test results.
    #[strum(serialize = "162")]
    _162,
    /// Material inspection and receiving report
    ///
    /// A report that is both an inspection report for materials and a receiving document.
    #[strum(serialize = "163")]
    _163,
    /// Purchasing specification
    ///
    /// A document indicating a specification used to purchase an item.
    #[strum(serialize = "164")]
    _164,
    /// Payment or performance bond
    ///
    /// A document indicating a bond that guarantees the payment of monies or a performance.
    #[strum(serialize = "165")]
    _165,
    /// Contract security classification specification
    ///
    /// A document that indicates the specification contains the security and classification requirements for a contract.
    #[strum(serialize = "166")]
    _166,
    /// Manufacturing specification
    ///
    /// A document indicating the specification of how an item is to be manufactured.
    #[strum(serialize = "167")]
    _167,
    /// Buy America certificate of compliance
    ///
    /// A document certifying that more than 50 percent of the cost of an item is attributed to US origin.
    #[strum(serialize = "168")]
    _168,
    /// Container off-hire notice
    ///
    /// Notice to return leased containers.
    #[strum(serialize = "169")]
    _169,
    /// Cargo acceptance order
    ///
    /// Order to accept cargo to be delivered by a carrier.
    #[strum(serialize = "170")]
    _170,
    /// Pick-up notice
    ///
    /// Notice specifying the pick-up of released cargo or containers from a certain address.
    #[strum(serialize = "171")]
    _171,
    /// Authorisation to plan and suggest orders
    ///
    /// Document or message that authorises receiver to plan orders, based on information in this message, and send these orders as suggestions to the sender.
    #[strum(serialize = "172")]
    _172,
    /// Authorisation to plan and ship orders
    ///
    /// Document or message that authorises receiver to plan and ship orders based on information in this message.
    #[strum(serialize = "173")]
    _173,
    /// Drawing
    ///
    /// The document or message is a drawing.
    #[strum(serialize = "174")]
    _174,
    /// Cost Performance Report (CPR) format 2
    ///
    /// A report identifying the cost performance on a contract at specified levels of the work breakdown structure (format 2 - organizational categories).
    #[strum(serialize = "175")]
    _175,
    /// Cost Schedule Status Report (CSSR)
    ///
    /// A report providing the status of the cost and schedule applicable to a contract.
    #[strum(serialize = "176")]
    _176,
    /// Cost Performance Report (CPR) format 1
    ///
    /// A report identifying the cost performance on a contract including the current month's values at specified levels of the work breakdown structure (format 1 - work breakdown structure).
    #[strum(serialize = "177")]
    _177,
    /// Cost Performance Report (CPR) format 3
    ///
    /// A report identifying the cost performance on a contract that summarizes changes to a contract over a given reporting period with beginning and ending values (format 3 - baseline).
    #[strum(serialize = "178")]
    _178,
    /// Cost Performance Report (CPR) format 4
    ///
    /// A report identifying the cost performance on a contract including forecasts of labour requirements for the remaining portion of the contract (format 4 - staffing).
    #[strum(serialize = "179")]
    _179,
    /// Cost Performance Report (CPR) format 5
    ///
    /// A report identifying the cost performance on a contract that summarizes cost or schedule variances (format 5 - explanations and problem analysis).
    #[strum(serialize = "180")]
    _180,
    /// Progressive discharge report
    ///
    /// Document or message progressively issued by the container terminal operator in charge of discharging a vessel identifying containers that have been discharged from a specific vessel at that point in time.
    #[strum(serialize = "181")]
    _181,
    /// Balance confirmation
    ///
    /// Confirmation of a balance at an entry date.
    #[strum(serialize = "182")]
    _182,
    /// Container stripping order
    ///
    /// Order to unload goods from a container.
    #[strum(serialize = "183")]
    _183,
    /// Container stuffing order
    ///
    /// Order to stuff specified goods or consignments in a container.
    #[strum(serialize = "184")]
    _184,
    /// Conveyance declaration (arrival)
    ///
    /// Declaration to the public authority upon arrival of the conveyance.
    #[strum(serialize = "185")]
    _185,
    /// Conveyance declaration (departure)
    ///
    /// Declaration to the public authority upon departure of the conveyance.
    #[strum(serialize = "186")]
    _186,
    /// Conveyance declaration (combined)
    ///
    /// Combined declaration of arrival and departure to the public authority.
    #[strum(serialize = "187")]
    _187,
    /// Project recovery plan
    ///
    /// A project plan for recovery after a delay or problem resolution.
    #[strum(serialize = "188")]
    _188,
    /// Project production plan
    ///
    /// A project plan for the production of goods.
    #[strum(serialize = "189")]
    _189,
    /// Statistical and other administrative internal documents
    ///
    /// Documents/messages issued within an enterprise for the for the purpose of collection of production and other internal statistics, and for other administration purposes.
    #[strum(serialize = "190")]
    _190,
    /// Project master schedule
    ///
    /// A high level, all encompassing master schedule of activities to complete a project.
    #[strum(serialize = "191")]
    _191,
    /// Priced alternate tender bill of quantity
    ///
    /// A priced tender based upon an alternate specification.
    #[strum(serialize = "192")]
    _192,
    /// Estimated priced bill of quantity
    ///
    /// An estimate based upon a detailed, quantity based specification (bill of quantity).
    #[strum(serialize = "193")]
    _193,
    /// Draft bill of quantity
    ///
    /// Document/message providing a draft bill of quantity, issued in an unpriced form.
    #[strum(serialize = "194")]
    _194,
    /// Documentary credit collection instruction
    ///
    /// Instruction for the collection of the documentary credit.
    #[strum(serialize = "195")]
    _195,
    /// Request for an amendment of a documentary credit
    ///
    /// Request for an amendment of a documentary credit.
    #[strum(serialize = "196")]
    _196,
    /// Documentary credit amendment information
    ///
    /// Documentary credit amendment information.
    #[strum(serialize = "197")]
    _197,
    /// Advice of an amendment of a documentary credit
    ///
    /// Advice of an amendment of a documentary credit.
    #[strum(serialize = "198")]
    _198,
    /// Response to an amendment of a documentary credit
    ///
    /// Response to an amendment of a documentary credit.
    #[strum(serialize = "199")]
    _199,
    /// Documentary credit issuance information
    ///
    /// Provides information on documentary credit issuance.
    #[strum(serialize = "200")]
    _200,
    /// Direct payment valuation request
    ///
    /// Request to establish a direct payment valuation.
    #[strum(serialize = "201")]
    _201,
    /// Direct payment valuation
    ///
    /// Document/message addressed, for instance, by a general contractor to the owner, in order that a direct payment be made to a subcontractor.
    #[strum(serialize = "202")]
    _202,
    /// Provisional payment valuation
    ///
    /// Document/message establishing a provisional payment valuation.
    #[strum(serialize = "203")]
    _203,
    /// Payment valuation
    ///
    /// Document/message establishing the financial elements of a situation of works.
    #[strum(serialize = "204")]
    _204,
    /// Quantity valuation
    ///
    /// Document/message providing a confirmed assessment, by quantity, of the completed work for a construction contract.
    #[strum(serialize = "205")]
    _205,
    /// Quantity valuation request
    ///
    /// Document/message providing an initial assessment, by quantity, of the completed work for a construction contract.
    #[strum(serialize = "206")]
    _206,
    /// Contract bill of quantities - BOQ
    ///
    /// Document/message providing a formal specification identifying quantities and prices that are the basis of a contract for a construction project. BOQ means: Bill of quantity.
    #[strum(serialize = "207")]
    _207,
    /// Unpriced bill of quantity
    ///
    /// Document/message providing a detailed, quantity based specification, issued in an unpriced form to invite tender prices.
    #[strum(serialize = "208")]
    _208,
    /// Priced tender BOQ
    ///
    /// Document/message providing a detailed, quantity based specification, updated with prices to form a tender submission for a construction contract. BOQ means: Bill of quantity.
    #[strum(serialize = "209")]
    _209,
    /// Enquiry
    ///
    /// Document/message issued by a party interested in the purchase of goods specified therein and indicating particular, desirable conditions regarding delivery terms, etc., addressed to a prospective supplier with a view to obtaining an offer.
    #[strum(serialize = "210")]
    _210,
    /// Interim application for payment
    ///
    /// Document/message containing a provisional assessment in support of a request for payment for completed work for a construction contract.
    #[strum(serialize = "211")]
    _211,
    /// Agreement to pay
    ///
    /// Document/message in which the debtor expresses the intention to pay.
    #[strum(serialize = "212")]
    _212,
    /// Request for financial cancellation
    ///
    /// The message is a request for financial cancellation.
    #[strum(serialize = "213")]
    _213,
    /// Pre-authorised direct debit(s)
    ///
    /// The message contains pre-authorised direct debit(s).
    #[strum(serialize = "214")]
    _214,
    /// Letter of intent
    ///
    /// Document/message by means of which a buyer informs a seller that the buyer intends to enter into contractual negotiations.
    #[strum(serialize = "215")]
    _215,
    /// Approved unpriced bill of quantity
    ///
    /// Document/message providing an approved detailed, quantity based specification (bill of quantity), in an unpriced form.
    #[strum(serialize = "216")]
    _216,
    /// Payment valuation for unscheduled items
    ///
    /// A payment valuation for unscheduled items.
    #[strum(serialize = "217")]
    _217,
    /// Final payment request based on completion of work
    ///
    /// The final payment request of a series of payment requests submitted upon completion of all the work.
    #[strum(serialize = "218")]
    _218,
    /// Payment request for completed units
    ///
    /// A request for payment for completed units.
    #[strum(serialize = "219")]
    _219,
    /// Order
    ///
    /// Document/message by means of which a buyer initiates a transaction with a seller involving the supply of goods or services as specified, according to conditions set out in an offer, or otherwise known to the buyer.
    #[strum(serialize = "220")]
    _220,
    /// Blanket order
    ///
    /// Usage of document/message for general order purposes with later split into quantities and delivery dates and maybe delivery locations.
    #[strum(serialize = "221")]
    _221,
    /// Spot order
    ///
    /// Document/message ordering the remainder of a production's batch.
    #[strum(serialize = "222")]
    _222,
    /// Lease order
    ///
    /// Document/message for goods in leasing contracts.
    #[strum(serialize = "223")]
    _223,
    /// Rush order
    ///
    /// Document/message for urgent ordering.
    #[strum(serialize = "224")]
    _224,
    /// Repair order
    ///
    /// Document/message to order repair of goods.
    #[strum(serialize = "225")]
    _225,
    /// Call off order
    ///
    /// Document/message to provide split quantities and delivery dates referring to a previous blanket order.
    #[strum(serialize = "226")]
    _226,
    /// Consignment order
    ///
    /// Order to deliver goods into stock with agreement on payment when goods are sold out of this stock.
    #[strum(serialize = "227")]
    _227,
    /// Sample order
    ///
    /// Document/message to order samples.
    #[strum(serialize = "228")]
    _228,
    /// Swap order
    ///
    /// Document/message informing buyer or seller of the replacement of goods previously ordered.
    #[strum(serialize = "229")]
    _229,
    /// Purchase order change request
    ///
    /// Change to an purchase order already sent.
    #[strum(serialize = "230")]
    _230,
    /// Purchase order response
    ///
    /// Response to an purchase order already received.
    #[strum(serialize = "231")]
    _231,
    /// Hire order
    ///
    /// Document/message for hiring human resources or renting goods or equipment.
    #[strum(serialize = "232")]
    _232,
    /// Spare parts order
    ///
    /// Document/message to order spare parts.
    #[strum(serialize = "233")]
    _233,
    /// Campaign price/sales catalogue
    ///
    /// A price/sales catalogue containing special prices which are valid only for a specified period or under specified conditions.
    #[strum(serialize = "234")]
    _234,
    /// Container list
    ///
    /// Document or message issued by party identifying the containers for which they are responsible.
    #[strum(serialize = "235")]
    _235,
    /// Delivery forecast
    ///
    /// A message which enables the transmission of delivery or product forecasting requirements.
    #[strum(serialize = "236")]
    _236,
    /// Cross docking services order
    ///
    /// A document or message to order cross docking services.
    #[strum(serialize = "237")]
    _237,
    /// Non-pre-authorised direct debit(s)
    ///
    /// The message contains non-pre-authorised direct debit(s).
    #[strum(serialize = "238")]
    _238,
    /// Rejected direct debit(s)
    ///
    /// The message contains rejected direct debit(s).
    #[strum(serialize = "239")]
    _239,
    /// Delivery instructions
    ///
    /// Document/message issued by a buyer giving instructions regarding the details of the delivery of goods ordered.
    #[strum(serialize = "240")]
    _240,
    /// Delivery schedule
    ///
    /// Usage of DELFOR-message.
    #[strum(serialize = "241")]
    _241,
    /// Delivery just-in-time
    ///
    /// Usage of DELJIT-message.
    #[strum(serialize = "242")]
    _242,
    /// Pre-authorised direct debit request(s)
    ///
    /// The message contains pre-authorised direct debit request(s).
    #[strum(serialize = "243")]
    _243,
    /// Non-pre-authorised direct debit request(s)
    ///
    /// The message contains non-pre-authorised direct debit request(s).
    #[strum(serialize = "244")]
    _244,
    /// Delivery release
    ///
    /// Document/message issued by a buyer releasing the despatch of goods after receipt of the Ready for despatch advice from the seller.
    #[strum(serialize = "245")]
    _245,
    /// Settlement of a letter of credit
    ///
    /// Settlement of a letter of credit.
    #[strum(serialize = "246")]
    _246,
    /// Bank to bank funds transfer
    ///
    /// The message is a bank to bank funds transfer.
    #[strum(serialize = "247")]
    _247,
    /// Customer payment order(s)
    ///
    /// The message contains customer payment order(s).
    #[strum(serialize = "248")]
    _248,
    /// Low value payment order(s)
    ///
    /// The message contains low value payment order(s) only.
    #[strum(serialize = "249")]
    _249,
    /// Crew list declaration
    ///
    /// Declaration regarding crew members aboard the conveyance.
    #[strum(serialize = "250")]
    _250,
    /// Inquiry
    ///
    /// This is a request for information.
    #[strum(serialize = "251")]
    _251,
    /// Response to previous banking status message
    ///
    /// A response to a previously sent banking status message.
    #[strum(serialize = "252")]
    _252,
    /// Project master plan
    ///
    /// A high level, all encompassing master plan to complete a project.
    #[strum(serialize = "253")]
    _253,
    /// Project plan
    ///
    /// A plan for project work to be completed.
    #[strum(serialize = "254")]
    _254,
    /// Project schedule
    ///
    /// A schedule of project activities to be completed.
    #[strum(serialize = "255")]
    _255,
    /// Project planning available resources
    ///
    /// Available resources for project planning purposes.
    #[strum(serialize = "256")]
    _256,
    /// Project planning calendar
    ///
    /// Work calendar information for project planning purposes.
    #[strum(serialize = "257")]
    _257,
    /// Standing order
    ///
    /// An order to supply fixed quantities of products at fixed regular intervals.
    #[strum(serialize = "258")]
    _258,
    /// Cargo movement event log
    ///
    /// A document detailing times and dates of events pertaining to a cargo movement.
    #[strum(serialize = "259")]
    _259,
    /// Cargo analysis voyage report
    ///
    /// An analysis of the cargo for a voyage.
    #[strum(serialize = "260")]
    _260,
    /// Self billed credit note
    ///
    /// A document which indicates that the customer is claiming credit in a self billing environment.
    #[strum(serialize = "261")]
    _261,
    /// Consolidated credit note - goods and services
    ///
    /// Credit note for goods and services that covers multiple transactions involving more than one invoice.
    #[strum(serialize = "262")]
    _262,
    /// Inventory adjustment status report
    ///
    /// A message detailing statuses related to the adjustment of inventory.
    #[strum(serialize = "263")]
    _263,
    /// Transport equipment movement instruction
    ///
    /// Instruction to perform one or more different movements of transport equipment.
    #[strum(serialize = "264")]
    _264,
    /// Transport equipment movement report
    ///
    /// Report on one or more different movements of transport equipment.
    #[strum(serialize = "265")]
    _265,
    /// Transport equipment status change report
    ///
    /// Report on one or more changes of status associated with an item or items of transport equipment.
    #[strum(serialize = "266")]
    _266,
    /// Fumigation certificate
    ///
    /// Certificate attesting that fumigation has been performed.
    #[strum(serialize = "267")]
    _267,
    /// Wine certificate
    ///
    /// Certificate attesting to the quality, origin or appellation of wine.
    #[strum(serialize = "268")]
    _268,
    /// Wool health certificate
    ///
    /// Certificate attesting that wool is free from specified risks to human or animal health.
    #[strum(serialize = "269")]
    _269,
    /// Delivery note
    ///
    /// Paper document attached to a consignment informing the receiving party about contents of this consignment.
    #[strum(serialize = "270")]
    _270,
    /// Packing list
    ///
    /// Document/message specifying the distribution of goods in individual packages (in trade environment the despatch advice message is used for the packing list).
    #[strum(serialize = "271")]
    _271,
    /// New code request
    ///
    /// Requesting a new code.
    #[strum(serialize = "272")]
    _272,
    /// Code change request
    ///
    /// Request a change to an existing code.
    #[strum(serialize = "273")]
    _273,
    /// Simple data element request
    ///
    /// Requesting a new simple data element.
    #[strum(serialize = "274")]
    _274,
    /// Simple data element change request
    ///
    /// Request a change to an existing simple data element.
    #[strum(serialize = "275")]
    _275,
    /// Composite data element request
    ///
    /// Requesting a new composite data element.
    #[strum(serialize = "276")]
    _276,
    /// Composite data element change request
    ///
    /// Request a change to an existing composite data element.
    #[strum(serialize = "277")]
    _277,
    /// Segment request
    ///
    /// Request a new segment.
    #[strum(serialize = "278")]
    _278,
    /// Segment change request
    ///
    /// Requesting a change to an existing segment.
    #[strum(serialize = "279")]
    _279,
    /// New message request
    ///
    /// Request for a new message (NMR).
    #[strum(serialize = "280")]
    _280,
    /// Message in development request
    ///
    /// Requesting a Message in Development (MiD).
    #[strum(serialize = "281")]
    _281,
    /// Modification of existing message
    ///
    /// Requesting a change to an existing message.
    #[strum(serialize = "282")]
    _282,
    /// Tracking number assignment report
    ///
    /// Report of assigned tracking numbers.
    #[strum(serialize = "283")]
    _283,
    /// User directory definition
    ///
    /// Document/message defining the contents of a user directory set or parts thereof.
    #[strum(serialize = "284")]
    _284,
    /// United Nations standard message request
    ///
    /// Requesting a United Nations Standard Message (UNSM).
    #[strum(serialize = "285")]
    _285,
    /// Service directory definition
    ///
    /// Document/message defining the contents of a service directory set or parts thereof.
    #[strum(serialize = "286")]
    _286,
    /// Status report
    ///
    /// Message covers information about the status.
    #[strum(serialize = "287")]
    _287,
    /// Kanban schedule
    ///
    /// Message to describe a Kanban schedule.
    #[strum(serialize = "288")]
    _288,
    /// Product data message
    ///
    /// A message to submit master data, a set of data that is rarely changed, to identify and describe products a supplier offers to their (potential) customer or buyer.
    #[strum(serialize = "289")]
    _289,
    /// A claim for parts and/or labour charges
    ///
    /// A claim for parts and/or labour charges incurred .
    #[strum(serialize = "290")]
    _290,
    /// Delivery schedule response
    ///
    /// A message providing a response to a previously transmitted delivery schedule.
    #[strum(serialize = "291")]
    _291,
    /// Inspection request
    ///
    /// A message requesting a party to inspect items.
    #[strum(serialize = "292")]
    _292,
    /// Inspection report
    ///
    /// A message informing a party of the results of an inspection.
    #[strum(serialize = "293")]
    _293,
    /// Application acknowledgement and error report
    ///
    /// A message used by an application to acknowledge reception of a message and/or to report any errors.
    #[strum(serialize = "294")]
    _294,
    /// Price variation invoice
    ///
    /// An invoice which requests payment for the difference in price between an original invoice and the result of the application of a price variation formula.
    #[strum(serialize = "295")]
    _295,
    /// Credit note for price variation
    ///
    /// A credit note which is issued against a price variation invoice.
    #[strum(serialize = "296")]
    _296,
    /// Instruction to collect
    ///
    /// A message instructing a party to collect goods.
    #[strum(serialize = "297")]
    _297,
    /// Dangerous goods list
    ///
    /// Listing of all details of dangerous goods carried.
    #[strum(serialize = "298")]
    _298,
    /// Registration renewal
    ///
    /// Code specifying the continued validity of previously submitted registration information.
    #[strum(serialize = "299")]
    _299,
    /// Registration change
    ///
    /// Code specifying the modification of previously submitted registration information.
    #[strum(serialize = "300")]
    _300,
    /// Response to registration
    ///
    /// Code specifying a response to an occurrence of a registration message.
    #[strum(serialize = "301")]
    _301,
    /// Implementation guideline
    ///
    /// A document specifying the criterion and format for exchanging information in an electronic data interchange syntax.
    #[strum(serialize = "302")]
    _302,
    /// Request for transfer
    ///
    /// Document/message is a request for transfer.
    #[strum(serialize = "303")]
    _303,
    /// Cost performance report
    ///
    /// A report to convey cost performance data for a project or contract.
    #[strum(serialize = "304")]
    _304,
    /// Application error and acknowledgement
    ///
    /// A message to inform a message issuer that a previously sent message has been received by the addressee's application, or that a previously sent message has been rejected by the addressee's application.
    #[strum(serialize = "305")]
    _305,
    /// Cash pool financial statement
    ///
    /// A financial statement for a cash pool.
    #[strum(serialize = "306")]
    _306,
    /// Sequenced delivery schedule
    ///
    /// Message to describe a sequence of product delivery.
    #[strum(serialize = "307")]
    _307,
    /// Delcredere credit note
    ///
    /// A credit note sent to the party paying on behalf of a number of buyers.
    #[strum(serialize = "308")]
    _308,
    /// Healthcare discharge report, final
    ///
    /// Final discharge report by healthcare provider.
    #[strum(serialize = "309")]
    _309,
    /// Offer/quotation
    ///
    /// Document/message which , with a view to concluding a contract, sets out the conditions under which the goods are offered.
    #[strum(serialize = "310")]
    _310,
    /// Request for quote
    ///
    /// Document/message requesting a quote on specified goods or services.
    #[strum(serialize = "311")]
    _311,
    /// Acknowledgement message
    ///
    /// Message providing acknowledgement information at the business application level concerning the processing of a message.
    #[strum(serialize = "312")]
    _312,
    /// Application error message
    ///
    /// Message indicating that a message was rejected due to errors encountered at the application level.
    #[strum(serialize = "313")]
    _313,
    /// Cargo movement voyage summary
    ///
    /// A consolidated voyage summary which contains the information in a certificate of analysis, a voyage analysis and a cargo movement time log for a voyage.
    #[strum(serialize = "314")]
    _314,
    /// Contract
    ///
    /// Document/message evidencing an agreement between the seller and the buyer for the supply of goods or services; its effects are equivalent to those of an order followed by an acknowledgement of order.
    #[strum(serialize = "315")]
    _315,
    /// Application for usage of berth or mooring facilities
    ///
    /// Document to apply for usage of berth or mooring facilities.
    #[strum(serialize = "316")]
    _316,
    /// Application for designation of berthing places
    ///
    /// Document to apply for designation of berthing places.
    #[strum(serialize = "317")]
    _317,
    /// Application for shifting from the designated place in port
    ///
    /// Document to apply for shifting from the designated place in port.
    #[strum(serialize = "318")]
    _318,
    /// Supplementary document for application for cargo operation of dangerous goods
    ///
    /// Supplementary document to apply for cargo operation of dangerous goods.
    #[strum(serialize = "319")]
    _319,
    /// Acknowledgement of order
    ///
    /// Document/message acknowledging an undertaking to fulfil an order and confirming conditions or acceptance of conditions.
    #[strum(serialize = "320")]
    _320,
    /// Supplementary document for application for transport of dangerous goods
    ///
    /// Supplementary document to apply for transport of dangerous goods.
    #[strum(serialize = "321")]
    _321,
    /// Optical Character Reading (OCR) payment
    ///
    /// Payment effected by an Optical Character Reading (OCR) document.
    #[strum(serialize = "322")]
    _322,
    /// Preliminary sales report
    ///
    /// Preliminary sales report sent before all the information is available.
    #[strum(serialize = "323")]
    _323,
    /// Transport emergency card
    ///
    /// Official document specifying, for a given dangerous goods item, information such as nature of hazard, protective devices, actions to be taken in case of accident, spillage or fire and first aid to be given.
    #[strum(serialize = "324")]
    _324,
    /// Proforma invoice
    ///
    /// Document/message serving as a preliminary invoice, containing - on the whole - the same information as the final invoice, but not actually claiming payment.
    #[strum(serialize = "325")]
    _325,
    /// Partial invoice
    ///
    ///
    #[strum(serialize = "326")]
    _326,
    /// Operating instructions
    ///
    ///
    #[strum(serialize = "327")]
    _327,
    /// Name/product plate
    ///
    /// Plates on goods identifying and describing an article.
    #[strum(serialize = "328")]
    _328,
    /// Co-insurance ceding bordereau
    ///
    /// The document or message contains a bordereau describing co-insurance ceding information.
    #[strum(serialize = "329")]
    _329,
    /// Request for delivery instructions
    ///
    /// Document/message issued by a supplier requesting instructions from the buyer regarding the details of the delivery of goods ordered.
    #[strum(serialize = "330")]
    _330,
    /// Commercial invoice which includes a packing list
    ///
    /// Commercial transaction (invoice) will include a packing list.
    #[strum(serialize = "331")]
    _331,
    /// Trade data
    ///
    /// Document/message is for trade data.
    #[strum(serialize = "332")]
    _332,
    /// Customs declaration for cargo examination
    ///
    /// Declaration provided to customs for cargo examination.
    #[strum(serialize = "333")]
    _333,
    /// Customs declaration for cargo examination, alternate
    ///
    /// Alternate declaration provided to customs for cargo examination.
    #[strum(serialize = "334")]
    _334,
    /// Booking request
    ///
    /// Document/message issued by a supplier to a carrier requesting space to be reserved for a specified consignment, indicating desirable conveyance, despatch time, etc.
    #[strum(serialize = "335")]
    _335,
    /// Customs crew and conveyance
    ///
    /// Document/message contains information regarding the crew list and conveyance.
    #[strum(serialize = "336")]
    _336,
    /// Customs summary declaration with commercial detail, alternate
    ///
    /// Alternate Customs declaration summary with commercial transaction details.
    #[strum(serialize = "337")]
    _337,
    /// Items booked to a financial account report
    ///
    /// A message reporting items which have been booked to a financial account.
    #[strum(serialize = "338")]
    _338,
    /// Report of transactions which need further information from the receiver
    ///
    /// A message reporting transactions which need further information from the receiver.
    #[strum(serialize = "339")]
    _339,
    /// Shipping instructions
    ///
    /// Document/message advising details of cargo and exporter's requirements for its physical movement.
    #[strum(serialize = "340")]
    _340,
    /// Shipper's letter of instructions (air)
    ///
    /// Document/message issued by a consignor in which he gives details of a consignment of goods that enables an airline or its agent to prepare an air waybill.
    #[strum(serialize = "341")]
    _341,
    /// Report of transactions for information only
    ///
    /// A message reporting transactions for information only.
    #[strum(serialize = "342")]
    _342,
    /// Cartage order (local transport)
    ///
    /// Document/message giving instructions regarding local transport of goods, e.g. from the premises of an enterprise to those of a carrier undertaking further transport.
    #[strum(serialize = "343")]
    _343,
    /// EDI associated object administration message
    ///
    /// A message giving additional information about the exchange of an EDI associated object.
    #[strum(serialize = "344")]
    _344,
    /// Ready for despatch advice
    ///
    /// Document/message issued by a supplier informing a buyer that goods ordered are ready for despatch.
    #[strum(serialize = "345")]
    _345,
    /// Summary sales report
    ///
    /// Sales report containing summaries for several earlier sent sales reports.
    #[strum(serialize = "346")]
    _346,
    /// Order status enquiry
    ///
    /// A message enquiring the status of previously sent orders.
    #[strum(serialize = "347")]
    _347,
    /// Order status report
    ///
    /// A message reporting the status of previously sent orders.
    #[strum(serialize = "348")]
    _348,
    /// Declaration regarding the inward and outward movement of vessel
    ///
    /// Document to declare inward and outward movement of a vessel.
    #[strum(serialize = "349")]
    _349,
    /// Despatch order
    ///
    /// Document/message issued by a supplier initiating the despatch of goods to a buyer (consignee).
    #[strum(serialize = "350")]
    _350,
    /// Despatch advice
    ///
    /// Document/message by means of which the seller or consignor informs the consignee about the despatch of goods.
    #[strum(serialize = "351")]
    _351,
    /// Notification of usage of berth or mooring facilities
    ///
    /// Document to notify usage of berth or mooring facilities.
    #[strum(serialize = "352")]
    _352,
    /// Application for vessel's entering into port area in night- time
    ///
    /// Document to apply for vessel's entering into port area in night-time.
    #[strum(serialize = "353")]
    _353,
    /// Notification of emergency shifting from the designated place in port
    ///
    /// Document to notify shifting from designated place in port once secured at the designated place.
    #[strum(serialize = "354")]
    _354,
    /// Customs summary declaration without commercial detail, alternate
    ///
    /// Alternate Customs declaration summary without any commercial transaction details.
    #[strum(serialize = "355")]
    _355,
    /// Performance bond
    ///
    /// A document that guarantees performance.
    #[strum(serialize = "356")]
    _356,
    /// Payment bond
    ///
    /// A document that guarantees the payment of monies.
    #[strum(serialize = "357")]
    _357,
    /// Healthcare discharge report, preliminary
    ///
    /// Preliminary discharge report by healthcare provider.
    #[strum(serialize = "358")]
    _358,
    /// Request for provision of a health service
    ///
    /// Document containing request for provision of a health service.
    #[strum(serialize = "359")]
    _359,
    /// Advice of distribution of documents
    ///
    /// Document/message in which the party responsible for the issue of a set of trade documents specifies the various recipients of originals and copies of these documents, with an indication of the number of copies distributed to each of them.
    #[strum(serialize = "370")]
    _370,
    /// Plan for provision of health service
    ///
    /// Document containing a plan for provision of health service.
    #[strum(serialize = "371")]
    _371,
    /// Prescription
    ///
    /// Instructions for the dispensing and use of medicine or remedy.
    #[strum(serialize = "372")]
    _372,
    /// Prescription request
    ///
    /// Request to issue a prescription for medicine or remedy.
    #[strum(serialize = "373")]
    _373,
    /// Prescription dispensing report
    ///
    /// Document containing information of products dispensed according to a prescription.
    #[strum(serialize = "374")]
    _374,
    /// Certificate of shipment
    ///
    /// Certificate providing confirmation that a consignment has been shipped.
    #[strum(serialize = "375")]
    _375,
    /// Standing inquiry on product information
    ///
    /// A product inquiry which stands until it is cancelled.
    #[strum(serialize = "376")]
    _376,
    /// Commercial invoice
    ///
    /// Document/message claiming payment for goods or services supplied under conditions agreed between seller and buyer.
    #[strum(serialize = "380")]
    _380,
    /// Credit note
    ///
    /// Document/message for providing credit information to the relevant party.
    #[strum(serialize = "381")]
    _381,
    /// Commission note
    ///
    /// Document/message in which a seller specifies the amount of commission, the percentage of the invoice amount, or some other basis for the calculation of the commission to which a sales agent is entitled.
    #[strum(serialize = "382")]
    _382,
    /// Debit note
    ///
    /// Document/message for providing debit information to the relevant party.
    #[strum(serialize = "383")]
    _383,
    /// Corrected invoice
    ///
    /// Commercial invoice that includes revised information differing from an earlier submission of the same invoice.
    #[strum(serialize = "384")]
    _384,
    /// Consolidated invoice
    ///
    /// Commercial invoice that covers multiple transactions involving more than one vendor.
    #[strum(serialize = "385")]
    _385,
    /// Prepayment invoice
    ///
    /// An invoice to pay amounts for goods and services in advance; these amounts will be subtracted from the final invoice.
    #[strum(serialize = "386")]
    _386,
    /// Hire invoice
    ///
    /// Document/message for invoicing the hiring of human resources or renting goods or equipment.
    #[strum(serialize = "387")]
    _387,
    /// Tax invoice
    ///
    /// An invoice for tax purposes.
    #[strum(serialize = "388")]
    _388,
    /// Self-billed invoice
    ///
    /// An invoice the invoicee is producing instead of the seller.
    #[strum(serialize = "389")]
    _389,
    /// Delcredere invoice
    ///
    /// An invoice sent to the party paying for a number of buyers.
    #[strum(serialize = "390")]
    _390,
    /// Factored invoice
    ///
    /// Invoice assigned to a third party for collection.
    #[strum(serialize = "393")]
    _393,
    /// Lease invoice
    ///
    /// Usage of INVOIC-message for goods in leasing contracts.
    #[strum(serialize = "394")]
    _394,
    /// Consignment invoice
    ///
    /// Commercial invoice that covers a transaction other than one involving a sale.
    #[strum(serialize = "395")]
    _395,
    /// Factored credit note
    ///
    /// Credit note related to assigned invoice(s).
    #[strum(serialize = "396")]
    _396,
    /// Commercial account summary response
    ///
    /// A document providing a response to a previously sent commercial account summary message.
    #[strum(serialize = "397")]
    _397,
    /// Cross docking despatch advice
    ///
    /// Document by means of which the supplier or consignor informs the buyer, consignee or the distribution centre about the despatch of goods for cross docking.
    #[strum(serialize = "398")]
    _398,
    /// Transshipment despatch advice
    ///
    /// Document by means of which the supplier or consignor informs the buyer, consignee or the distribution centre about the despatch of goods for transshipment.
    #[strum(serialize = "399")]
    _399,
    /// Exceptional order
    ///
    /// An order which falls outside the framework of an agreement.
    #[strum(serialize = "400")]
    _400,
    /// Transshipment order
    ///
    /// An order requesting the supply of products packed according to the final delivery point which will be moved across a dock in a distribution centre without further handling.
    #[strum(serialize = "401")]
    _401,
    /// Cross docking order
    ///
    /// An order requesting the supply of products which will be de-consolidated in the distribution centre and re- consolidated according to final delivery location.
    #[strum(serialize = "402")]
    _402,
    /// Means of transportation availability information
    ///
    /// Information giving the various availabilities of a means of transportation.
    #[strum(serialize = "403")]
    _403,
    /// Means of transportation schedule information
    ///
    /// Information giving the various schedules of a means of transportation.
    #[strum(serialize = "404")]
    _404,
    /// Transport equipment delivery notice
    ///
    /// Notification regarding the delivery of transport equipment.
    #[strum(serialize = "405")]
    _405,
    /// Instructions for bank transfer
    ///
    /// Document/message containing instructions from a customer to his bank to pay an amount in a specified currency to a nominated party in another country by a method either specified (e.g. teletransmission, air mail) or left to the discretion of the bank.
    #[strum(serialize = "409")]
    _409,
    /// Application for banker's draft
    ///
    /// Application by a customer to his bank to issue a banker's draft stating the amount and currency of the draft, the name of the payee and the place and country of payment.
    #[strum(serialize = "412")]
    _412,
    /// Collection payment advice
    ///
    /// Document/message whereby a bank advises that a collection has been paid, giving details and methods of funds disposal.
    #[strum(serialize = "425")]
    _425,
    /// Documentary credit payment advice
    ///
    /// Document/message whereby a bank advises payment under a documentary credit.
    #[strum(serialize = "426")]
    _426,
    /// Documentary credit acceptance advice
    ///
    /// Document/message whereby a bank advises acceptance under a documentary credit.
    #[strum(serialize = "427")]
    _427,
    /// Documentary credit negotiation advice
    ///
    /// Document/message whereby a bank advises negotiation under a documentary credit.
    #[strum(serialize = "428")]
    _428,
    /// Application for banker's guarantee
    ///
    /// Document/message whereby a customer requests his bank to issue a guarantee in favour of a nominated party in another country, stating the amount and currency and the specific conditions of the guarantee.
    #[strum(serialize = "429")]
    _429,
    /// Banker's guarantee
    ///
    /// Document/message in which a bank undertakes to pay out a limited amount of money to a designated party, on conditions stated therein (other than those laid down in the Uniform Customs Practice).
    #[strum(serialize = "430")]
    _430,
    /// Documentary credit letter of indemnity
    ///
    /// Document/message in which a beneficiary of a documentary credit accepts responsibility for non-compliance with the terms and conditions of the credit, and undertakes to refund the money received under the credit, with interest and charges accrued.
    #[strum(serialize = "431")]
    _431,
    /// Preadvice of a credit
    ///
    /// Preadvice indicating a credit to happen in the future.
    #[strum(serialize = "435")]
    _435,
    /// Collection order
    ///
    /// Document/message whereby a bank is instructed (or requested) to handle financial and/or commercial documents in order to obtain acceptance and/or payment, or to deliver documents on such other terms and conditions as may be specified.
    #[strum(serialize = "447")]
    _447,
    /// Documents presentation form
    ///
    /// Document/message whereby a draft or similar instrument and/or commercial documents are presented to a bank for acceptance, discounting, negotiation, payment or collection, whether or not against a documentary credit.
    #[strum(serialize = "448")]
    _448,
    /// Payment order
    ///
    /// Document/message containing information needed to initiate the payment. It may cover the financial settlement for one or more commercial trade transactions. A payment order is an instruction to the ordered bank to arrange for the payment of one specified amount to the beneficiary.
    #[strum(serialize = "450")]
    _450,
    /// Extended payment order
    ///
    /// Document/message containing information needed to initiate the payment. It may cover the financial settlement for several commercial trade transactions, which it is possible to specify in a special payments detail part. It is an instruction to the ordered bank to arrange for the payment of one specified amount to the beneficiary.
    #[strum(serialize = "451")]
    _451,
    /// Multiple payment order
    ///
    /// Document/message containing a payment order to debit one or more accounts and to credit one or more beneficiaries.
    #[strum(serialize = "452")]
    _452,
    /// Credit advice
    ///
    /// Document/message sent by an account servicing institution to one of its account owners, to inform the account owner of an entry which has been or will be credited to its account for a specified amount on the date indicated.
    #[strum(serialize = "454")]
    _454,
    /// Extended credit advice
    ///
    /// Document/message sent by an account servicing institution to one of its account owners, to inform the account owner of an entry that has been or will be credited to its account for a specified amount on the date indicated. It provides extended commercial information concerning the relevant remittance advice.
    #[strum(serialize = "455")]
    _455,
    /// Debit advice
    ///
    /// Advice on a debit.
    #[strum(serialize = "456")]
    _456,
    /// Reversal of debit
    ///
    /// Reversal of debit accounting entry by bank.
    #[strum(serialize = "457")]
    _457,
    /// Reversal of credit
    ///
    /// Reversal of credit accounting entry by bank.
    #[strum(serialize = "458")]
    _458,
    /// Documentary credit application
    ///
    /// Document/message whereby a bank is requested to issue a documentary credit on the conditions specified therein.
    #[strum(serialize = "460")]
    _460,
    /// Documentary credit
    ///
    /// Document/message in which a bank states that it has issued a documentary credit under which the beneficiary is to obtain payment, acceptance or negotiation on compliance with certain terms and conditions and against presentation of stipulated documents and such drafts as may be specified. The credit may or may not be confirmed by another bank.
    #[strum(serialize = "465")]
    _465,
    /// Documentary credit notification
    ///
    /// Document/message issued by an advising bank in order to transmit a documentary credit to a beneficiary, or to another advising bank.
    #[strum(serialize = "466")]
    _466,
    /// Documentary credit transfer advice
    ///
    /// Document/message whereby a bank advises that (part of) a documentary credit is being or has been transferred in favour of a second beneficiary.
    #[strum(serialize = "467")]
    _467,
    /// Documentary credit amendment notification
    ///
    /// Document/message whereby a bank advises that the terms and conditions of a documentary credit have been amended.
    #[strum(serialize = "468")]
    _468,
    /// Documentary credit amendment
    ///
    /// Document/message whereby a bank notifies a beneficiary of the details of an amendment to the terms and conditions of a documentary credit.
    #[strum(serialize = "469")]
    _469,
    /// Remittance advice
    ///
    /// Document/message advising of the remittance of payment.
    #[strum(serialize = "481")]
    _481,
    /// Banker's draft
    ///
    /// Draft drawn in favour of a third party either by one bank on another bank, or by a branch of a bank on its head office (or vice versa) or upon another branch of the same bank. In either case, the draft should comply with the specifications laid down for cheques in the country in which it is to be payable.
    #[strum(serialize = "485")]
    _485,
    /// Bill of exchange
    ///
    /// Document/message, issued and signed in conformity with the applicable legislation, which contains an unconditional order whereby the drawer directs the drawee to pay a definite sum of money to the payee or to his order, on demand or at a definite time, against the surrender of the document itself.
    #[strum(serialize = "490")]
    _490,
    /// Promissory note
    ///
    /// Document/message, issued and signed in conformity with the applicable legislation, which contains an unconditional promise whereby the maker undertakes to pay a definite sum of money to the payee or to his order, on demand or at a definite time, against the surrender of the document itself.
    #[strum(serialize = "491")]
    _491,
    /// Statement of account message
    ///
    /// Usage of STATAC-message.
    #[strum(serialize = "493")]
    _493,
    /// Insurance certificate
    ///
    /// Document/message issued to the insured certifying that insurance has been effected and that a policy has been issued. Such a certificate for a particular cargo is primarily used when good are insured under the terms of a floating or an open policy; at the request of the insured it can be exchanged for a policy.
    #[strum(serialize = "520")]
    _520,
    /// Insurance policy
    ///
    /// Document/message issued by the insurer evidencing an agreement to insure and containing the conditions of the agreement concluded whereby the insurer undertakes for a specific fee to indemnify the insured for the losses arising out of the perils and accidents specified in the contract.
    #[strum(serialize = "530")]
    _530,
    /// Insurance declaration sheet (bordereau)
    ///
    /// A document/message used when an insured reports to his insurer details of individual shipments which are covered by an insurance contract - an open cover or a floating policy - between the parties.
    #[strum(serialize = "550")]
    _550,
    /// Insurer's invoice
    ///
    /// Document/message issued by an insurer specifying the cost of an insurance which has been effected and claiming payment therefore.
    #[strum(serialize = "575")]
    _575,
    /// Cover note
    ///
    /// Document/message issued by an insurer (insurance broker, agent, etc.) to notify the insured that his insurance have been carried out.
    #[strum(serialize = "580")]
    _580,
    /// Forwarding instructions
    ///
    /// Document/message issued to a freight forwarder, giving instructions regarding the action to be taken by the forwarder for the forwarding of goods described therein.
    #[strum(serialize = "610")]
    _610,
    /// Forwarder's advice to import agent
    ///
    /// Document/message issued by a freight forwarder in an exporting country advising his counterpart in an importing country about the forwarding of goods described therein.
    #[strum(serialize = "621")]
    _621,
    /// Forwarder's advice to exporter
    ///
    /// Document/message issued by a freight forwarder informing an exporter of the action taken in fulfillment of instructions received.
    #[strum(serialize = "622")]
    _622,
    /// Forwarder's invoice
    ///
    /// Invoice issued by a freight forwarder specifying services rendered and costs incurred and claiming payment therefore.
    #[strum(serialize = "623")]
    _623,
    /// Forwarder's certificate of receipt
    ///
    /// Non-negotiable document issued by a forwarder to certify that he has assumed control of a specified consignment, with irrevocable instructions to send it to the consignee indicated in the document or to hold it at his disposal. E.g. FIATA-FCR.
    #[strum(serialize = "624")]
    _624,
    /// Shipping note
    ///
    /// Document/message provided by the shipper or his agent to the carrier, multimodal transport operator, terminal or other receiving authority, giving information about export consignments offered for transport, and providing for the necessary receipts and declarations of liability. (Sometimes a multipurpose cargo handling document also fulfilling the functions of document 632, 633, 650 and 655).
    #[strum(serialize = "630")]
    _630,
    /// Forwarder's warehouse receipt
    ///
    /// Document/message issued by a forwarder acting as Warehouse Keeper acknowledging receipt of goods placed in a warehouse, and stating or referring to the conditions which govern the warehousing and the release of goods. The document contains detailed provisions regarding the rights of holders-by-endorsement, transfer of ownership, etc. E.g. FIATA-FWR.
    #[strum(serialize = "631")]
    _631,
    /// Goods receipt
    ///
    /// Document/message issued by a port, warehouse/shed, or terminal operator acknowledging receipt of goods specified therein on conditions stated or referred to in the document.
    #[strum(serialize = "632")]
    _632,
    /// Port charges documents
    ///
    /// Documents/messages specifying services rendered, storage and handling costs, demurrage and other charges due to the owner of goods described therein.
    #[strum(serialize = "633")]
    _633,
    /// Warehouse warrant
    ///
    /// Negotiable receipt document, issued by a Warehouse Keeper to a person placing goods in a warehouse and conferring title to the goods stored.
    #[strum(serialize = "635")]
    _635,
    /// Delivery order
    ///
    /// Document/message issued by a party entitled to authorize the release of goods specified therein to a named consignee, to be retained by the custodian of the goods.
    #[strum(serialize = "640")]
    _640,
    /// Handling order
    ///
    /// Document/message issued by a cargo handling organization (port administration, terminal operator, etc.) for the removal or other handling of goods under their care.
    #[strum(serialize = "650")]
    _650,
    /// Gate pass
    ///
    /// Document/message authorizing goods specified therein to be brought out of a fenced-in port or terminal area.
    #[strum(serialize = "655")]
    _655,
    /// Waybill
    ///
    /// Non-negotiable document evidencing the contract for the transport of cargo.
    #[strum(serialize = "700")]
    _700,
    /// Universal (multipurpose) transport document
    ///
    /// Document/message evidencing a contract of carriage covering the movement of goods by any mode of transport, or combination of modes, for national as well as international transport, under any applicable international convention or national law and under the conditions of carriage of any carrier or transport operator undertaking or arranging the transport referred to in the document.
    #[strum(serialize = "701")]
    _701,
    /// Goods receipt, carriage
    ///
    /// Document/message issued by a carrier or a carrier's agent, acknowledging receipt for carriage of goods specified therein on conditions stated or referred to in the document, enabling the carrier to issue a transport document.
    #[strum(serialize = "702")]
    _702,
    /// House waybill
    ///
    /// The document made out by an agent/consolidator which evidences the contract between the shipper and the agent/consolidator for the arrangement of carriage of goods.
    #[strum(serialize = "703")]
    _703,
    /// Master bill of lading
    ///
    /// A bill of lading issued by the master of a vessel (in actuality the owner or charterer of the vessel). It could cover a number of house bills.
    #[strum(serialize = "704")]
    _704,
    /// Bill of lading
    ///
    /// Negotiable document/message which evidences a contract of carriage by sea and the taking over or loading of goods by carrier, and by which carrier undertakes to deliver goods against surrender of the document. A provision in the document that goods are to be delivered to the order of a named person, or to order, or to bearer, constitutes such an undertaking.
    #[strum(serialize = "705")]
    _705,
    /// Bill of lading original
    ///
    /// The original of the bill of lading issued by a transport company. When issued by the maritime industry it could signify ownership of the cargo.
    #[strum(serialize = "706")]
    _706,
    /// Bill of lading copy
    ///
    /// A copy of the bill of lading issued by a transport company.
    #[strum(serialize = "707")]
    _707,
    /// Empty container bill
    ///
    /// Bill of lading indicating an empty container.
    #[strum(serialize = "708")]
    _708,
    /// Tanker bill of lading
    ///
    /// Document which evidences a transport of liquid bulk cargo.
    #[strum(serialize = "709")]
    _709,
    /// Sea waybill
    ///
    /// Non-negotiable document which evidences a contract for the carriage of goods by sea and the taking over of the goods by the carrier, and by which the carrier undertakes to deliver the goods to the consignee named in the document.
    #[strum(serialize = "710")]
    _710,
    /// Inland waterway bill of lading
    ///
    /// Negotiable transport document made out to a named person, to order or to bearer, signed by the carrier and handed to the sender after receipt of the goods.
    #[strum(serialize = "711")]
    _711,
    /// Non-negotiable maritime transport document (generic)
    ///
    /// Non-negotiable document which evidences a contract for the carriage of goods by sea and the taking over or loading of the goods by the carrier, and by which the carrier undertakes to deliver the goods to the consignee named in the document. E.g. Sea waybill. Remark: Synonymous with "straight" or "non-negotiable Bill of lading" used in certain countries, e.g. Canada.
    #[strum(serialize = "712")]
    _712,
    /// Mate's receipt
    ///
    /// Document/message issued by a ship's officer to acknowledge that a specified consignment has been received on board a vessel, and the apparent condition of the goods; enabling the carrier to issue a Bill of lading.
    #[strum(serialize = "713")]
    _713,
    /// House bill of lading
    ///
    /// The bill of lading issued not by the carrier but by the freight forwarder/consolidator known by the carrier.
    #[strum(serialize = "714")]
    _714,
    /// Letter of indemnity for non-surrender of bill of lading
    ///
    /// Document/message issued by a commercial party or a bank of an insurance company accepting responsibility to the beneficiary of the indemnity in accordance with the terms thereof.
    #[strum(serialize = "715")]
    _715,
    /// Forwarder's bill of lading
    ///
    /// Non-negotiable document issued by a freight forwarder evidencing a contract for the carriage of goods by sea and the taking over or loading of the goods by the freight forwarder, and by which the freight forwarder undertakes to deliver the goods to the consignee named in the document.
    #[strum(serialize = "716")]
    _716,
    /// Rail consignment note (generic term)
    ///
    /// Transport document constituting a contract for the carriage of goods between the sender and the carrier (the railway). For international rail traffic, this document must conform to the model prescribed by the international conventions concerning carriage of goods by rail, e.g. CIM Convention, SMGS Convention.
    #[strum(serialize = "720")]
    _720,
    /// Road list-SMGS
    ///
    /// Accounting document, one copy of which is drawn up for each consignment note; it accompanies the consignment over the whole route and is a rail transport document.
    #[strum(serialize = "722")]
    _722,
    /// Escort official recognition
    ///
    /// Document/message which gives right to the owner to exert all functions normally transferred to a guard in a train by which an escorted consignment is transported.
    #[strum(serialize = "723")]
    _723,
    /// Recharging document
    ///
    /// Fictitious transport document regarding a previous transport, enabling a carrier's agent to give to another carrier's agent (in a different country) the possibility to collect charges relating to the original transport (rail environment).
    #[strum(serialize = "724")]
    _724,
    /// Road consignment note
    ///
    /// Transport document/message which evidences a contract between a carrier and a sender for the carriage of goods by road (generic term). Remark: For international road traffic, this document must contain at least the particulars prescribed by the convention on the contract for the international carriage of goods by road (CMR).
    #[strum(serialize = "730")]
    _730,
    /// Air waybill
    ///
    /// Document/message made out by or on behalf of the shipper which evidences the contract between the shipper and carrier(s) for carriage of goods over routes of the carrier(s) and which is identified by the airline prefix issuing the document plus a serial (IATA).
    #[strum(serialize = "740")]
    _740,
    /// Master air waybill
    ///
    /// Document/message made out by or on behalf of the agent/consolidator which evidences the contract between the agent/consolidator and carrier(s) for carriage of goods over routes of the carrier(s) for a consignment consisting of goods originated by more than one shipper (IATA).
    #[strum(serialize = "741")]
    _741,
    /// Substitute air waybill
    ///
    /// A temporary air waybill which contains only limited information because of the absence of the original.
    #[strum(serialize = "743")]
    _743,
    /// Crew's effects declaration
    ///
    /// Declaration to Customs regarding the personal effects of crew members aboard the conveyance; equivalent to IMO FAL 4.
    #[strum(serialize = "744")]
    _744,
    /// Passenger list
    ///
    /// Declaration to Customs regarding passengers aboard the conveyance; equivalent to IMO FAL 6.
    #[strum(serialize = "745")]
    _745,
    /// Delivery notice (rail transport)
    ///
    /// Document/message created by the consignor or by the departure station, joined to the transport or sent to the consignee, giving the possibility to the consignee or the arrival station to attest the delivery of the goods. The document must be returned to the consignor or to the departure station.
    #[strum(serialize = "746")]
    _746,
    /// Despatch note (post parcels)
    ///
    /// Document/message which, according to Article 106 of the "Agreement concerning Postal Parcels" under the UPU convention, is to accompany post parcels.
    #[strum(serialize = "750")]
    _750,
    /// Multimodal/combined transport document (generic)
    ///
    /// A transport document used when more than one mode of transportation is involved in the movement of cargo. It is a contract of carriage and receipt of the cargo for a multimodal transport. It indicates the place where the responsible transport company in the move takes responsibility for the cargo, the place where the responsibility of this transport company in the move ends and the conveyances involved.
    #[strum(serialize = "760")]
    _760,
    /// Through bill of lading
    ///
    /// Bill of lading which evidences a contract of carriage from one place to another in separate stages of which at least one stage is a sea transit, and by which the issuing carrier accepts responsibility for the carriage as set forth in the through bill of lading.
    #[strum(serialize = "761")]
    _761,
    /// Forwarder's certificate of transport
    ///
    /// Negotiable document/message issued by a forwarder to certify that he has taken charge of a specified consignment for despatch and delivery in accordance with the consignor's instructions, as indicated in the document, and that he accepts responsibility for delivery of the goods to the holder of the document through the intermediary of a delivery agent of his choice. E.g. FIATA-FCT.
    #[strum(serialize = "763")]
    _763,
    /// Combined transport document (generic)
    ///
    /// Negotiable or non-negotiable document evidencing a contract for the performance and/or procurement of performance of combined transport of goods and bearing on its face either the heading "Negotiable combined transport document issued subject to Uniform Rules for a Combined Transport Document (ICC Brochure No. 298)" or the heading "Non-negotiable Combined Transport Document issued subject to Uniform Rules for a Combined Transport Document (ICC Brochure No. 298)".
    #[strum(serialize = "764")]
    _764,
    /// Multimodal transport document (generic)
    ///
    /// Document/message which evidences a multimodal transport contract, the taking in charge of the goods by the multimodal transport operator, and an undertaking by him to deliver the goods in accordance with the terms of the contract. (International Convention on Multimodal Transport of Goods).
    #[strum(serialize = "765")]
    _765,
    /// Combined transport bill of lading/multimodal bill of lading
    ///
    /// Document which evidences a multimodal transport contract, the taking in charge of the goods by the multimodal transport operator, and an undertaking by him to deliver the goods in accordance with the terms of the contract.
    #[strum(serialize = "766")]
    _766,
    /// Booking confirmation
    ///
    /// Document/message issued by a carrier to confirm that space has been reserved for a consignment in means of transport.
    #[strum(serialize = "770")]
    _770,
    /// Calling forward notice
    ///
    /// Instructions for release or delivery of goods.
    #[strum(serialize = "775")]
    _775,
    /// Freight invoice
    ///
    /// Document/message issued by a transport operation specifying freight costs and charges incurred for a transport operation and stating conditions of payment.
    #[strum(serialize = "780")]
    _780,
    /// Arrival notice (goods)
    ///
    /// Notification from the carrier to the consignee in writing, by telephone or by any other means (express letter, message, telegram, etc.) informing him that a consignment addressed to him is being or will shortly be held at his disposal at a specified point in the place of destination.
    #[strum(serialize = "781")]
    _781,
    /// Notice of circumstances preventing delivery (goods)
    ///
    /// Request made by the carrier to the sender, or, as the case may be, the consignee, for instructions as to the disposal of the consignment when circumstances prevent delivery and the return of the goods has not been requested by the consignor in the transport document.
    #[strum(serialize = "782")]
    _782,
    /// Notice of circumstances preventing transport (goods)
    ///
    /// Request made by the carrier to the sender, or, the consignee as the case may be, for instructions as to the disposal of the goods when circumstances prevent transport before departure or en route, after acceptance of the consignment concerned.
    #[strum(serialize = "783")]
    _783,
    /// Delivery notice (goods)
    ///
    /// Notification in writing, sent by the carrier to the sender, to inform him at his request of the actual date of delivery of the goods.
    #[strum(serialize = "784")]
    _784,
    /// Cargo manifest
    ///
    /// Listing of goods comprising the cargo carried in a means of transport or in a transport-unit. The cargo manifest gives the commercial particulars of the goods, such as transport document numbers, consignors, consignees, shipping marks, number and kind of packages and descriptions and quantities of the goods.
    #[strum(serialize = "785")]
    _785,
    /// Freight manifest
    ///
    /// Document/message containing the same information as a cargo manifest, and additional details on freight amounts, charges, etc.
    #[strum(serialize = "786")]
    _786,
    /// Bordereau
    ///
    /// Document/message used in road transport, listing the cargo carried on a road vehicle, often referring to appended copies of Road consignment note.
    #[strum(serialize = "787")]
    _787,
    /// Container manifest (unit packing list)
    ///
    /// Document/message specifying the contents of particular freight containers or other transport units, prepared by the party responsible for their loading into the container or unit.
    #[strum(serialize = "788")]
    _788,
    /// Charges note
    ///
    /// Document used by the rail organization to indicate freight charges or additional charges in each case where the departure station is not able to calculate the charges for the total voyage (e.g. tariff not yet updated, part of voyage not covered by the tariff). This document must be considered as joined to the transport.
    #[strum(serialize = "789")]
    _789,
    /// Advice of collection
    ///
    /// Document that is joined to the transport or sent by separate means, giving to the departure rail organization the proof that the cash-on delivery amount has been encashed by the arrival rail organization before reimbursement of the consignor.
    #[strum(serialize = "790")]
    _790,
    /// Safety of ship certificate
    ///
    /// Document certifying a ship's safety to a specified date.
    #[strum(serialize = "791")]
    _791,
    /// Safety of radio certificate
    ///
    /// Document certifying the safety of a ship's radio facilities to a specified date.
    #[strum(serialize = "792")]
    _792,
    /// Safety of equipment certificate
    ///
    /// Document certifying the safety of a ship's equipment to a specified date.
    #[strum(serialize = "793")]
    _793,
    /// Civil liability for oil certificate
    ///
    /// Document declaring a ship owner's liability for oil propelling or carried on a vessel.
    #[strum(serialize = "794")]
    _794,
    /// Loadline document
    ///
    /// Document specifying the limit of a ship's legal submersion under various conditions.
    #[strum(serialize = "795")]
    _795,
    /// Derat document
    ///
    /// Document certifying that a ship is free of rats, valid to a specified date.
    #[strum(serialize = "796")]
    _796,
    /// Maritime declaration of health
    ///
    /// Document certifying the health condition on board a vessel, valid to a specified date.
    #[strum(serialize = "797")]
    _797,
    /// Certificate of registry
    ///
    /// Official certificate stating the vessel's registry.
    #[strum(serialize = "798")]
    _798,
    /// Ship's stores declaration
    ///
    /// Declaration to Customs regarding the contents of the ship's stores (equivalent to IMO FAL 3) i.e. goods intended for consumption by passengers/crew on board vessels, aircraft or trains, whether or not sold or landed; goods necessary for operation/maintenance of conveyance, including fuel/lubricants, excluding spare parts/equipment (IMO).
    #[strum(serialize = "799")]
    _799,
    /// Export licence, application for
    ///
    /// Application for a permit issued by a government authority permitting exportation of a specified commodity subject to specified conditions as quantity, country of destination, etc.
    #[strum(serialize = "810")]
    _810,
    /// Export licence
    ///
    /// Permit issued by a government authority permitting exportation of a specified commodity subject to specified conditions as quantity, country of destination, etc. Synonym: Embargo permit.
    #[strum(serialize = "811")]
    _811,
    /// Exchange control declaration, export
    ///
    /// Document/message completed by an exporter/seller as a means whereby the competent body may control that the amount of foreign exchange accrued from a trade transaction is repatriated in accordance with the conditions of payment and exchange control regulations in force.
    #[strum(serialize = "812")]
    _812,
    /// Despatch note model T
    ///
    /// European community transit declaration.
    #[strum(serialize = "820")]
    _820,
    /// Despatch note model T1
    ///
    /// Transit declaration for goods circulating under internal community transit procedures (between European Union (EU) countries).
    #[strum(serialize = "821")]
    _821,
    /// Despatch note model T2
    ///
    /// Ascertainment that the declared goods were originally produced in an European Union (EU) country.
    #[strum(serialize = "822")]
    _822,
    /// Control document T5
    ///
    /// Control document (export declaration) used particularly in case of re-sending without use with only VAT collection, refusal, unconformity with contract etc.
    #[strum(serialize = "823")]
    _823,
    /// Re-sending consignment note
    ///
    /// Rail consignment note prepared by the consignor for the facilitation of an eventual return to the origin of the goods.
    #[strum(serialize = "824")]
    _824,
    /// Despatch note model T2L
    ///
    /// Ascertainment that the declared goods were originally produced in an European Union (EU) country. May only be used for goods that are loaded on one single means of transport in one single departure point for one single delivery point.
    #[strum(serialize = "825")]
    _825,
    /// Goods declaration for exportation
    ///
    /// Document/message by which goods are declared for export Customs clearance, conforming to the layout key set out at Appendix I to Annex C.1 concerning outright exportation to the Kyoto convention (CCC). Within a Customs union, "for despatch" may have the same meaning as "for exportation".
    #[strum(serialize = "830")]
    _830,
    /// Cargo declaration (departure)
    ///
    /// Generic term, sometimes referred to as Freight declaration, applied to the documents providing the particulars required by the Customs concerning the cargo (freight) carried by commercial means of transport (CCC).
    #[strum(serialize = "833")]
    _833,
    /// Application for goods control certificate
    ///
    /// Document/message submitted to a competent body by party requesting a Goods control certificate to be issued in accordance with national or international standards, or conforming to legislation in the importing country, or as specified in the contract.
    #[strum(serialize = "840")]
    _840,
    /// Goods control certificate
    ///
    /// Document/message issued by a competent body evidencing the quality of the goods described therein, in accordance with national or international standards, or conforming to legislation in the importing country, or as specified in the contract.
    #[strum(serialize = "841")]
    _841,
    /// Application for phytosanitary certificate
    ///
    /// Document/message submitted to a competent body by party requesting a Phytosanitary certificate to be issued.
    #[strum(serialize = "850")]
    _850,
    /// Phytosanitary certificate
    ///
    /// Document/message issued by the competent body in the exporting country evidencing that plants, fruit, or vegetables are free from disease and fit for consumption and giving details on fumigation or other treatment to which they may have been subjected.
    #[strum(serialize = "851")]
    _851,
    /// Sanitary certificate
    ///
    /// Document/message issued by the competent authority in the exporting country evidencing that alimentary and animal products, including dead animals, are fit for human consumption, and giving details, when relevant, of controls undertaken.
    #[strum(serialize = "852")]
    _852,
    /// Veterinary certificate
    ///
    /// Document/message issued by the competent authority in the exporting country evidencing that live animals or birds are not infested or infected with disease, and giving details regarding their provenance, and of vaccinations and other treatment to which they have been subjected.
    #[strum(serialize = "853")]
    _853,
    /// Application for inspection certificate
    ///
    /// Document/message submitted to a competent body by a party requesting an Inspection certificate to be issued in accordance with national or international standards, or conforming to legislation in the country in which it is required, or as specified in the contract.
    #[strum(serialize = "855")]
    _855,
    /// Inspection certificate
    ///
    /// Document/message issued by a competent body evidencing that the goods described therein have been inspected in accordance with national or international standards, in conformity with legislation in the country in which the inspection is required, or as specified in the contract.
    #[strum(serialize = "856")]
    _856,
    /// Certificate of origin, application for
    ///
    /// Document/message submitted to a competent body by an interested party requesting a Certificate of origin to be issued in accordance with relevant criteria, and on the basis of evidence of the origin of the goods.
    #[strum(serialize = "860")]
    _860,
    /// Certificate of origin
    ///
    /// Document/message identifying goods, in which the authority or body authorized to issue it certifies expressly that the goods to which the certificate relates originate in a specific country. The word "country" may include a group of countries, a region or a part of a country. This certificate may also include a declaration by the manufacturer, producer, supplier, exporter or other competent person.
    #[strum(serialize = "861")]
    _861,
    /// Declaration of origin
    ///
    /// Appropriate statement as to the origin of the goods, made in connection with their exportation by the manufacturer, producer, supplier, exporter or other competent person on the Commercial invoice or any other document relating to the goods (CCC).
    #[strum(serialize = "862")]
    _862,
    /// Regional appellation certificate
    ///
    /// Certificate drawn up in accordance with the rules laid down by an authority or approved body, certifying that the goods described therein qualify for a designation specific to the given region (e.g. champagne, port wine, Parmesan cheese).
    #[strum(serialize = "863")]
    _863,
    /// Preference certificate of origin
    ///
    /// Description to be provided.
    #[strum(serialize = "864")]
    _864,
    /// Certificate of origin form GSP
    ///
    /// Specific form of certificate of origin for goods qualifying for preferential treatment under the generalized system of preferences (includes a combined declaration of origin and certificate, form A).
    #[strum(serialize = "865")]
    _865,
    /// Consular invoice
    ///
    /// Document/message to be prepared by an exporter in his country and presented to a diplomatic representation of the importing country for endorsement and subsequently to be presented by the importer in connection with the import of the goods described therein.
    #[strum(serialize = "870")]
    _870,
    /// Dangerous goods declaration
    ///
    /// Document/message issued by a consignor in accordance with applicable conventions or regulations, describing hazardous goods or materials for transport purposes, and stating that the latter have been packed and labelled in accordance with the provisions of the relevant conventions or regulations.
    #[strum(serialize = "890")]
    _890,
    /// Statistical document, export
    ///
    /// Document/message in which an exporter provides information about exported goods required by the body responsible for the collection of international trade statistics.
    #[strum(serialize = "895")]
    _895,
    /// INTRASTAT declaration
    ///
    /// Document/message in which a declarant provides information about goods required by the body responsible for the collection of trade statistics.
    #[strum(serialize = "896")]
    _896,
    /// Delivery verification certificate
    ///
    /// Document/message whereby an official authority (Customs or governmental) certifies that goods have been delivered.
    #[strum(serialize = "901")]
    _901,
    /// Import licence, application for
    ///
    /// Document/message in which an interested party applies to the competent body for authorization to import either a limited quantity of articles subject to import restrictions, or an unlimited quantity of such articles during a limited period, and specifies the kind of articles, their origin and value, etc.
    #[strum(serialize = "910")]
    _910,
    /// Import licence
    ///
    /// Document/message issued by the competent body in accordance with import regulations in force, by which authorization is granted to a named party to import either a limited quantity of designated articles or an unlimited quantity of such articles during a limited period, under conditions specified in the document.
    #[strum(serialize = "911")]
    _911,
    /// Customs declaration without commercial detail
    ///
    /// CUSDEC transmission that does not include data from the commercial detail section of the message.
    #[strum(serialize = "913")]
    _913,
    /// Customs declaration with commercial and item detail
    ///
    /// CUSDEC transmission that includes data from both the commercial detail and item detail sections of the message.
    #[strum(serialize = "914")]
    _914,
    /// Customs declaration without item detail
    ///
    /// CUSDEC transmission that does not include data from the item detail section of the message.
    #[strum(serialize = "915")]
    _915,
    /// Related document
    ///
    /// Description to be provided.
    #[strum(serialize = "916")]
    _916,
    /// Receipt (Customs)
    ///
    /// Receipt for Customs duty/tax/fee paid.
    #[strum(serialize = "917")]
    _917,
    /// Application for exchange allocation
    ///
    /// Document/message whereby an importer/buyer requests the competent body to allocate an amount of foreign exchange to be transferred to an exporter/seller in payment for goods.
    #[strum(serialize = "925")]
    _925,
    /// Foreign exchange permit
    ///
    /// Document/message issued by the competent body authorizing an importer/buyer to transfer an amount of foreign exchange to an exporter/seller in payment for goods.
    #[strum(serialize = "926")]
    _926,
    /// Exchange control declaration (import)
    ///
    /// Document/message completed by an importer/buyer as a means for the competent body to control that a trade transaction for which foreign exchange has been allocated has been executed and that money has been transferred in accordance with the conditions of payment and the exchange control regulations in force.
    #[strum(serialize = "927")]
    _927,
    /// Goods declaration for importation
    ///
    /// Document/message by which goods are declared for import Customs clearance [sister entry of 830].
    #[strum(serialize = "929")]
    _929,
    /// Goods declaration for home use
    ///
    /// Document/message by which goods are declared for import Customs clearance according to Annex B.1 (concerning clearance for home use) to the Kyoto convention (CCC).
    #[strum(serialize = "930")]
    _930,
    /// Customs immediate release declaration
    ///
    /// Document/message issued by an importer notifying Customs that goods have been removed from an importing means of transport to the importer's premises under a Customs- approved arrangement for immediate release, or requesting authorization to do so.
    #[strum(serialize = "931")]
    _931,
    /// Customs delivery note
    ///
    /// Document/message whereby a Customs authority releases goods under its control to be placed at the disposal of the party concerned. Synonym: Customs release note.
    #[strum(serialize = "932")]
    _932,
    /// Cargo declaration (arrival)
    ///
    /// Generic term, sometimes referred to as Freight declaration, applied to the documents providing the particulars required by the Customs concerning the cargo (freight) carried by commercial means of transport (CCC).
    #[strum(serialize = "933")]
    _933,
    /// Value declaration
    ///
    /// Document/message in which a declarant (importer) states the invoice or other price (e.g. selling price, price of identical goods), and specifies costs for freight, insurance and packing, etc., terms of delivery and payment, any relationship with the trading partner, etc., for the purpose of determining the Customs value of goods imported.
    #[strum(serialize = "934")]
    _934,
    /// Customs invoice
    ///
    /// Document/message required by the Customs in an importing country in which an exporter states the invoice or other price (e.g. selling price, price of identical goods), and specifies costs for freight, insurance and packing, etc., terms of delivery and payment, for the purpose of determining the Customs value in the importing country of goods consigned to that country.
    #[strum(serialize = "935")]
    _935,
    /// Customs declaration (post parcels)
    ///
    /// Document/message which, according to Article 106 of the "Agreement concerning Postal Parcels" under the UPU Convention, must accompany post parcels and in which the contents of such parcels are specified.
    #[strum(serialize = "936")]
    _936,
    /// Tax declaration (value added tax)
    ///
    /// Document/message in which an importer states the pertinent information required by the competent body for assessment of value-added tax.
    #[strum(serialize = "937")]
    _937,
    /// Tax declaration (general)
    ///
    /// Document/message containing a general tax declaration.
    #[strum(serialize = "938")]
    _938,
    /// Tax demand
    ///
    /// Document/message containing the demand of tax.
    #[strum(serialize = "940")]
    _940,
    /// Embargo permit
    ///
    /// Document/message giving the permission to export specified goods.
    #[strum(serialize = "941")]
    _941,
    /// Goods declaration for Customs transit
    ///
    /// Document/message by which the sender declares goods for Customs transit according to Annex E.1 (concerning Customs transit) to the Kyoto convention (CCC).
    #[strum(serialize = "950")]
    _950,
    /// TIF form
    ///
    /// International Customs transit document by which the sender declares goods for carriage by rail in accordance with the provisions of the 1952 International Convention to facilitate the crossing of frontiers for goods carried by rail (TIF Convention of UIC).
    #[strum(serialize = "951")]
    _951,
    /// TIR carnet
    ///
    /// International Customs document (International Transit by Road), issued by a guaranteeing association approved by the Customs authorities, under the cover of which goods are carried, in most cases under Customs seal, in road vehicles and/or containers in compliance with the requirements of the Customs TIR Convention of the International Transport of Goods under cover of TIR Carnets (UN/ECE).
    #[strum(serialize = "952")]
    _952,
    /// EC carnet
    ///
    /// EC customs transit document issued by EC customs authorities for transit and/or temporary user of goods within the EC.
    #[strum(serialize = "953")]
    _953,
    /// EUR 1 certificate of origin
    ///
    /// Customs certificate used in preferential goods interchanges between EC countries and EC external countries.
    #[strum(serialize = "954")]
    _954,
    /// ATA carnet
    ///
    /// International Customs document (Admission Temporaire / Temporary Admission) which, issued under the terms of the ATA Convention (1961), incorporates an internationally valid guarantee and may be used, in lieu of national Customs documents and as security for import duties and taxes, to cover the temporary admission of goods and, where appropriate, the transit of goods. If accepted for controlling the temporary export and reimport of goods, international guarantee does not apply (CCC).
    #[strum(serialize = "955")]
    _955,
    /// Single administrative document
    ///
    /// A set of documents, replacing the various (national) forms for Customs declaration within the EC, implemented on 01-01-1988.
    #[strum(serialize = "960")]
    _960,
    /// General response (Customs)
    ///
    /// General response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[strum(serialize = "961")]
    _961,
    /// Document response (Customs)
    ///
    /// Document response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[strum(serialize = "962")]
    _962,
    /// Error response (Customs)
    ///
    /// Error response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[strum(serialize = "963")]
    _963,
    /// Package response (Customs)
    ///
    /// Package response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[strum(serialize = "964")]
    _964,
    /// Tax calculation/confirmation response (Customs)
    ///
    /// Tax calculation/confirmation response message to permit the transfer of data from Customs to the transmitter of the previous message.
    #[strum(serialize = "965")]
    _965,
    /// Quota prior allocation certificate
    ///
    /// Document/message issued by the competent body for prior allocation of a quota.
    #[strum(serialize = "966")]
    _966,
    /// End use authorization
    ///
    /// Description to be provided.
    #[strum(serialize = "990")]
    _990,
    /// Government contract
    ///
    /// Description to be provided.
    #[strum(serialize = "991")]
    _991,
    /// Statistical document, import
    ///
    /// Description to be provided.
    #[strum(serialize = "995")]
    _995,
    /// Application for documentary credit
    ///
    /// Message with application for opening of a documentary credit.
    #[strum(serialize = "996")]
    _996,
    /// Previous Customs document/message
    ///
    /// Indication of the previous Customs document/message concerning the same transaction.
    #[strum(serialize = "998")]
    _998,
}

/// Message function code
///
/// Code indicating the function of the message.
#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display, PartialEq)]
pub enum _1225 {
    /// Cancellation
    ///
    /// Message cancelling a previous transmission for a given
    /// transaction.
    #[strum(serialize = "1")]
    _1,
    #[strum(serialize = "2")]
    _2,
    #[strum(serialize = "3")]
    _3,
    #[strum(serialize = "4")]
    _4,
    #[strum(serialize = "5")]
    _5,
    #[strum(serialize = "6")]
    _6,
    #[strum(serialize = "7")]
    _7,
    #[strum(serialize = "8")]
    _8,
    #[strum(serialize = "9")]
    _9,
    #[strum(serialize = "10")]
    _10,
    #[strum(serialize = "11")]
    _11,
    #[strum(serialize = "12")]
    _12,
    #[strum(serialize = "13")]
    _13,
    #[strum(serialize = "14")]
    _14,
    #[strum(serialize = "15")]
    _15,
    #[strum(serialize = "16")]
    _16,
    #[strum(serialize = "17")]
    _17,
    #[strum(serialize = "18")]
    _18,
    #[strum(serialize = "19")]
    _19,
    #[strum(serialize = "20")]
    _20,
    #[strum(serialize = "21")]
    _21,
    #[strum(serialize = "22")]
    _22,
    #[strum(serialize = "23")]
    _23,
    #[strum(serialize = "24")]
    _24,
    #[strum(serialize = "25")]
    _25,
    #[strum(serialize = "26")]
    _26,
    #[strum(serialize = "27")]
    _27,
    #[strum(serialize = "28")]
    _28,
    #[strum(serialize = "29")]
    _29,
    #[strum(serialize = "30")]
    _30,
    #[strum(serialize = "31")]
    _31,
    #[strum(serialize = "32")]
    _32,
    #[strum(serialize = "33")]
    _33,
    #[strum(serialize = "34")]
    _34,
    #[strum(serialize = "35")]
    _35,
    #[strum(serialize = "36")]
    _36,
    #[strum(serialize = "37")]
    _37,
    #[strum(serialize = "38")]
    _38,
    #[strum(serialize = "39")]
    _39,
    #[strum(serialize = "40")]
    _40,
    #[strum(serialize = "41")]
    _41,
    #[strum(serialize = "42")]
    _42,
    #[strum(serialize = "43")]
    _43,
    #[strum(serialize = "44")]
    _44,
    #[strum(serialize = "45")]
    _45,
    #[strum(serialize = "46")]
    _46,
    #[strum(serialize = "47")]
    _47,
    #[strum(serialize = "48")]
    _48,
    #[strum(serialize = "49")]
    _49,
    #[strum(serialize = "50")]
    _50,
    #[strum(serialize = "51")]
    _51,
    #[strum(serialize = "52")]
    _52,
    #[strum(serialize = "53")]
    _53,
    #[strum(serialize = "54")]
    _54,
    #[strum(serialize = "55")]
    _55,
    #[strum(serialize = "56")]
    _56,
    #[strum(serialize = "57")]
    _57,
    #[strum(serialize = "58")]
    _58,
    #[strum(serialize = "59")]
    _59,
    #[strum(serialize = "60")]
    _60,
    #[strum(serialize = "61")]
    _61,
    #[strum(serialize = "62")]
    _62,
    #[strum(serialize = "63")]
    _63,
}

/// Code list identification code
///
/// Code identifying a code list.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _1131 {
    /// Logistics code list
    ///
    /// Code list containing logistics and program management activities.
    #[strum(serialize = "1")]
    _1,
    /// ICD 9
    ///
    /// A code list containing the International Classification of Diseases, version 9 (ICD 9).
    #[strum(serialize = "2")]
    _2,
    /// Operating status
    ///
    /// Code list identifying operating status of an entity.
    #[strum(serialize = "3")]
    _3,
    /// DoDAAC (Department of Defense Activity Address Code)
    ///
    /// A code list containing codes assigned to operating military units to identify the name and address of the unit.
    #[strum(serialize = "4")]
    _4,
    /// Facility identification
    ///
    /// A code list identifying a facility(ies).
    #[strum(serialize = "5")]
    _5,
    /// Application acknowledgement and error codes
    ///
    /// A code list to identify acknowledgement and error codes applicable at the application level.
    #[strum(serialize = "6")]
    _6,
    /// Health industry organization identification
    ///
    /// List of codes identifying organizations in the health care industry.
    #[strum(serialize = "7")]
    _7,
    /// Electromagnetic transmitter identification
    ///
    /// A code list containing electromagnetic transmitter identifications.
    #[strum(serialize = "8")]
    _8,
    /// Military Assistance Program Address Code (MAPAC)
    ///
    /// Lists of codes identifying name and address information for organizations participating in a military assistance program.
    #[strum(serialize = "9")]
    _9,
    /// Medicare provider
    ///
    /// A list of codes identifying health care providers under the Medicare program.
    #[strum(serialize = "10")]
    _10,
    /// Medicaid provider
    ///
    /// A list of codes identifying health care providers under a Medicaid program.
    #[strum(serialize = "11")]
    _11,
    ///
    /// Telephone directory
    #[strum(serialize = "12")]
    _12,
    /// Employee identification
    ///
    /// A list of codes identifying employees of an organization.
    #[strum(serialize = "13")]
    _13,
    /// Sample extraction location
    ///
    /// Code list identifying the location from which a sample is taken.
    #[strum(serialize = "14")]
    _14,
    /// Medical benefits schedule
    ///
    /// Code list containing classifications of medical services for use in determining the medical benefits payable.
    #[strum(serialize = "15")]
    _15,
    /// Postcode directory
    ///
    /// [3251] Code defining postal zones or addresses.
    #[strum(serialize = "16")]
    _16,
    /// ICD 10
    ///
    /// Code list containing the International Classification of Diseases, version 10 (ICD 10).
    #[strum(serialize = "17")]
    _17,
    /// Diagnosis Related Group (DRG)
    ///
    /// Code list containing diagnosis related group classifications.
    #[strum(serialize = "18")]
    _18,
    /// Standard text clauses
    ///
    /// A list of codes representing standardized text clauses.
    #[strum(serialize = "19")]
    _19,
    /// United Nations Standard Products and Services Classification (UN/SPSC) code
    ///
    /// A code list that provides a hierarchical classification of goods and services for the purposes of resource discovery and spend analysis.
    #[strum(serialize = "20")]
    _20,
    /// Policy on claim indicator
    /// Identifies a code list containing indicators referring to policy on claims.
    ///
    /// (legal persons, partnerships, sole proprietorships and their branch offices) and private persons.
    #[strum(serialize = "21")]
    _21,
    /// A code list specifying codes assigned by the EDI Registration Authority to register organizations
    ///
    /// EDIRA-Id (EDI Registration Authority Identification)
    #[strum(serialize = "22")]
    _22,
    ///
    /// Clearing house automated payment
    #[strum(serialize = "23")]
    _23,
    /// Rail handling restrictions and instructions
    ///
    /// A code list specifying rail codes for handling restrictions or instructions.
    #[strum(serialize = "24")]
    _24,
    /// Bank identification
    ///
    /// Code for identification of banks.
    #[strum(serialize = "25")]
    _25,
    /// Rail harmonized equipment type
    ///
    /// A code list specifying codes for harmonized equipment type in the railway industry.
    #[strum(serialize = "26")]
    _26,
    /// Railway frontier and transit point
    ///
    /// A code list specifying frontier or transit points in the railway industry.
    #[strum(serialize = "27")]
    _27,
    /// Commercial And Government Entity (CAGE)
    ///
    /// List of codes identifying a commercial and government entity.
    #[strum(serialize = "33")]
    _33,
    /// Reinsurance policy attributes
    ///
    /// A list of attributes regarding policies reinsured with a professional reinsurer.
    #[strum(serialize = "34")]
    _34,
    /// Rail additional charges
    ///
    /// A code list identifying specific rail charges included in the payment conditions in addition to the freight cost.
    #[strum(serialize = "35")]
    _35,
    /// Railway company network
    ///
    /// A code list identifying the different railway companies as member of the International Union of Railways.
    #[strum(serialize = "36")]
    _36,
    /// Railway locations
    ///
    /// Code identifying a location in railway environment.
    #[strum(serialize = "37")]
    _37,
    /// Railway customer
    ///
    /// A code list identifying rail customers.
    #[strum(serialize = "38")]
    _38,
    ///
    /// Rail unified nomenclature of goods
    #[strum(serialize = "39")]
    _39,
    /// Reinsurance monetary type
    ///
    /// Identifies the type of reinsurance amounts.
    #[strum(serialize = "40")]
    _40,
    ///
    /// Business function
    #[strum(serialize = "42")]
    _42,
    /// Clearing House Interbank Payment System Participants ID
    ///
    /// Participants identification of the automated clearing house of the New York Clearing House Association (CHIPS).
    #[strum(serialize = "43")]
    _43,
    /// Clearing House Interbank Payment System Universal ID
    ///
    /// Universal identification of the automated clearing house of the New York Clearing House Association (CHIPS).
    #[strum(serialize = "44")]
    _44,
    /// United Nations Common Coding System (UNCCS)
    ///
    /// A code list adopted by the United Nations organisations for the procurement of goods and services.
    #[strum(serialize = "45")]
    _45,
    /// DUNS (Dun and Bradstreet) +4
    ///
    /// An organization identified by the DUNS number and a 4- character extension.
    #[strum(serialize = "46")]
    _46,
    /// Occupation classification
    ///
    /// Identifies the class of occupation.
    #[strum(serialize = "47")]
    _47,
    /// Policy reserve valuation type
    ///
    /// Identification of the policy reserve valuation type.
    #[strum(serialize = "48")]
    _48,
    /// Life reinsurance message type
    ///
    /// To indicate the type of life reinsurance activity transmitted in the message.
    #[strum(serialize = "49")]
    _49,
    /// Value added tax identification
    ///
    /// Value added tax identification code.
    #[strum(serialize = "52")]
    _52,
    /// Passport number
    ///
    /// Number assigned to a passport.
    #[strum(serialize = "53")]
    _53,
    /// Statistical object
    ///
    /// A statistical object such as a statistical concept, array structure component or statistical nomenclature.
    #[strum(serialize = "54")]
    _54,
    /// Quality conformance
    ///
    /// A code list specifying the quality standard a product complies with, e.g. ISO9000, BS5750, etc.
    #[strum(serialize = "55")]
    _55,
    /// Safety regulation
    ///
    /// A code list specifying the safety regulations which apply to a product, such as UK COSHH (control of substances hazardous to health) regulations.
    #[strum(serialize = "56")]
    _56,
    /// Product code
    ///
    /// Code assigned to a specific product by a controlling agency.
    #[strum(serialize = "57")]
    _57,
    /// Business account number
    ///
    /// An identifying number or code assigned by issuing authorities to manage business activities.
    #[strum(serialize = "58")]
    _58,
    /// Railway services harmonized code
    ///
    /// Services provided by the different railway organizations.
    #[strum(serialize = "59")]
    _59,
    /// Type of financial account
    ///
    /// Identification of the type of financial account.
    #[strum(serialize = "60")]
    _60,
    /// Type of assets and liabilities
    ///
    /// Identification of the type of assets and liabilities.
    #[strum(serialize = "61")]
    _61,
    /// Requirements indicator
    ///
    /// A code list which specifies various requirements that a customer may have when fulfilling a purchase order.
    #[strum(serialize = "62")]
    _62,
    /// Handling action
    ///
    /// Codes for handling action.
    #[strum(serialize = "63")]
    _63,
    /// Freight forwarder
    ///
    /// Codes for freight forwarders.
    #[strum(serialize = "64")]
    _64,
    /// Shipping agent
    ///
    /// Codes for shipping agents.
    #[strum(serialize = "65")]
    _65,
    /// Type of package
    ///
    /// Indication of the type of package codes.
    #[strum(serialize = "67")]
    _67,
    /// Type of industrial activity
    ///
    /// Identification of the type of industrial activity.
    #[strum(serialize = "68")]
    _68,
    /// Type of survey question
    ///
    /// Identification of the type of survey question.
    #[strum(serialize = "69")]
    _69,
    /// Customs inspection type
    ///
    /// A code to indicate the type of inspection performed by customs.
    #[strum(serialize = "70")]
    _70,
    /// Nature of transaction
    ///
    /// Identification of the nature of the transaction.
    #[strum(serialize = "71")]
    _71,
    /// Container terminal
    ///
    /// Codes for container terminal.
    #[strum(serialize = "72")]
    _72,
    /// Insurance information indicator
    ///
    /// Identifies the type of insurance information provided.
    #[strum(serialize = "73")]
    _73,
    /// Joint life insurance indicator
    ///
    /// Indicates joint life insurance coverage.
    #[strum(serialize = "74")]
    _74,
    /// Bill of lading clauses
    ///
    /// Code list identifying official clauses associated with bills of lading.
    #[strum(serialize = "75")]
    _75,
    /// Export commodity classification (US Schedule B)
    ///
    /// Code list containing the commodity classifications applying to goods being exported (United States Schedule B).
    #[strum(serialize = "76")]
    _76,
    /// Customs domestic port location codes (US Schedule D)
    ///
    /// Code list containing Customs domestic port locations (United States Schedule D).
    #[strum(serialize = "77")]
    _77,
    /// Customs foreign port location codes (US Schedule K)
    ///
    /// Code list containing Customs foreign port locations (United States Schedule K).
    #[strum(serialize = "78")]
    _78,
    /// Functional group
    ///
    /// Identifies a group of application related messages.
    #[strum(serialize = "79")]
    _79,
    /// Application error code
    ///
    /// A code list specifying application errors.
    #[strum(serialize = "80")]
    _80,
    /// Policy type
    ///
    /// To identify the code list for the type of policy.
    #[strum(serialize = "81")]
    _81,
    /// Type of insured
    ///
    /// To specify the insured type.
    #[strum(serialize = "82")]
    _82,
    /// Occupation code
    ///
    /// Identification of an occupation.
    #[strum(serialize = "83")]
    _83,
    /// State code
    ///
    /// A code list of states within a country.
    #[strum(serialize = "84")]
    _84,
    /// Technical Assessment Checklist (TAC)
    ///
    /// A code list of technical assessment checklist numbers.
    #[strum(serialize = "85")]
    _85,
    /// Syntax notes
    ///
    /// A code list of syntax (dependency) note identifiers.
    #[strum(serialize = "86")]
    _86,
    ///
    /// Enhanced party identification
    #[strum(serialize = "100")]
    _100,
    ///
    /// Air carrier
    #[strum(serialize = "101")]
    _101,
    ///
    /// Size and type
    #[strum(serialize = "102")]
    _102,
    /// Call sign directory
    ///
    /// A directory of call signs assigned to transport vehicles.
    #[strum(serialize = "103")]
    _103,
    /// Customs area of transaction
    ///
    /// Customs code to indicate the different types of declarations according to the countries involved in the transaction (e.g. box 1/1 of SAD: inter EC Member States, EC-EFTA, EC-third countries, etc.).
    #[strum(serialize = "104")]
    _104,
    /// Customs declaration type
    ///
    /// Customs code to indicate the type of declaration according to the different Customs procedures requested (e.g.: import, export, transit).
    #[strum(serialize = "105")]
    _105,
    /// Incoterms 1980
    ///
    /// (4110) Code to indicate applicable Incoterm (1980 edition) under which seller undertakes to deliver merchandise to buyer (ICC). Incoterms 1990: use 4053 only.
    #[strum(serialize = "106")]
    _106,
    /// Excise duty
    ///
    /// Customs or fiscal authorities code to identify a specific or ad valorem levy on a specific commodity, applied either domestically or at time of importation.
    #[strum(serialize = "107")]
    _107,
    ///
    /// Tariff schedule
    #[strum(serialize = "108")]
    _108,
    /// Customs indicator
    ///
    /// Customs code for circumstances where only an indication is needed.
    #[strum(serialize = "109")]
    _109,
    /// Customs special codes
    ///
    /// Customs code to indicate an exemption to a regulation or a special Customs treatment.
    #[strum(serialize = "110")]
    _110,
    /// Statistical nature of transaction
    ///
    /// Indication of the type of contract under which goods are supplied.
    #[strum(serialize = "112")]
    _112,
    /// Customs office
    ///
    /// Customs administrative unit competent for the performance of Customs formalities, and the premises or other areas approved for the purpose by the competent authorities (CCC).
    #[strum(serialize = "113")]
    _113,
    /// Railcar letter marking
    ///
    /// Codes for all marking codes (in letters) for railcars specifying the type, series, order number, check digit and some technical characteristics.
    #[strum(serialize = "114")]
    _114,
    /// Examination facility
    ///
    /// Building or location where merchandise is examined by Customs.
    #[strum(serialize = "115")]
    _115,
    /// Customs preference
    ///
    /// Customs code to identify a specific tariff preference.
    #[strum(serialize = "116")]
    _116,
    /// Customs procedure
    ///
    /// (9380) Customs code to identify goods which are subject to Customs control (e.g. home use, Customs warehousing, temporary admission, Customs transit).
    #[strum(serialize = "117")]
    _117,
    /// Government agency procedure
    ///
    /// Treatment applied by a government agency other than Customs to merchandise under their control.
    #[strum(serialize = "118")]
    _118,
    /// Customs simplified procedure
    ///
    /// Customs code to indicate the type of simplified Customs procedure requested by a declarant (CCC).
    #[strum(serialize = "119")]
    _119,
    /// Customs status of goods
    ///
    /// Customs code to specify the status accorded by Customs to a consignment e.g. release without further formality, present supporting documents for inspection, etc (CCC).
    #[strum(serialize = "120")]
    _120,
    /// Shipment description
    ///
    /// Code to indicate whether a shipment is a total, part or split consignment.
    #[strum(serialize = "121")]
    _121,
    /// Commodity
    ///
    /// (7357) Code identifying types of goods for Customs, transport or statistical purposes (generic term).
    #[strum(serialize = "122")]
    _122,
    /// Entitlement
    ///
    /// Code to indicate the recipient of a charge amount (IATA).
    #[strum(serialize = "123")]
    _123,
    /// Customs transit guarantee
    ///
    /// Customs code to identify the type of guarantee used in a transit movement.
    #[strum(serialize = "125")]
    _125,
    /// Accounting information identifier
    ///
    /// Identification of a specific kind of accounting information.
    #[strum(serialize = "126")]
    _126,
    /// Customs valuation method
    ///
    /// Customs code to identify the valuation method used to determine the dutiable value of the declared goods.
    #[strum(serialize = "127")]
    _127,
    /// Service
    ///
    /// Identification of services.
    #[strum(serialize = "128")]
    _128,
    /// Customs warehouse
    ///
    /// Identification and/or location of the Customs warehouse in which goods will be or have been deposited (CCC).
    #[strum(serialize = "129")]
    _129,
    /// Special handling
    ///
    /// Code to indicate that the nature of the consignment may necessitate use of special handling procedures (IATA).
    #[strum(serialize = "130")]
    _130,
    /// Free zone
    ///
    /// Code identifying the zone within a state where any goods introduced are generally regarded, insofar as import duties and taxes are concerned, as being outside the Customs territory and are not subject to the usual Customs control.
    #[strum(serialize = "131")]
    _131,
    /// Charge
    ///
    /// Identification of a type of charge.
    #[strum(serialize = "132")]
    _132,
    /// Financial regime
    ///
    /// Nature and methods of a transaction from financial viewpoint.
    #[strum(serialize = "133")]
    _133,
    /// Duty, tax or fee payment method
    ///
    /// [4390] Method by which a duty or tax is paid to the relevant administration.
    #[strum(serialize = "134")]
    _134,
    /// Rate class
    ///
    /// Code to identify a specific rate category.
    #[strum(serialize = "135")]
    _135,
    /// Restrictions and prohibitions placed on the re-use of designated rail wagons
    ///
    /// A code list identifying restrictions and prohibitions placed on the re-use of designated rail wagons.
    #[strum(serialize = "136")]
    _136,
    /// Rail harmonized codification of tariffs
    ///
    /// A list of rail tariffs, the coding of which has been harmonized.
    #[strum(serialize = "137")]
    _137,
    /// Port
    ///
    /// A location having facilities for means of transport to load or discharge cargo.
    #[strum(serialize = "139")]
    _139,
    /// Area
    ///
    /// Codes for specific geographic areas e.g. seas, straits, basins etc.
    #[strum(serialize = "140")]
    _140,
    /// Forwarding restrictions
    ///
    /// A code list containing restrictions regarding the forwarding of goods or equipment.
    #[strum(serialize = "141")]
    _141,
    /// Train identification
    ///
    /// A code list specifying international train identifications maintained by the UIC (International Union of Railways).
    #[strum(serialize = "142")]
    _142,
    /// Removable accessories and special equipment on railcars
    ///
    /// A list of removable accessories and special equipment associated with railcars.
    #[strum(serialize = "143")]
    _143,
    /// Rail routes
    ///
    /// A code list identifying routes used in rail transport.
    #[strum(serialize = "144")]
    _144,
    /// Airport/city
    ///
    /// As described and published by IATA.
    #[strum(serialize = "145")]
    _145,
    /// Means of transport identification
    ///
    /// Code identifying the name or number of a means of transport (vessel, vehicle).
    #[strum(serialize = "146")]
    _146,
    /// Document requested by Customs
    ///
    /// Customs code to identify documents requested by Customs in an information interchange.
    #[strum(serialize = "147")]
    _147,
    /// Customs release notification
    ///
    /// Authorisation given by Customs to move the goods or not move the goods from the place of registration.
    #[strum(serialize = "148")]
    _148,
    /// Customs transit type
    ///
    /// Customs code to indicate the different kinds of transit movement of the goods (e.g. Box 1/3 of the SAD).
    #[strum(serialize = "149")]
    _149,
    ///
    /// Financial routing
    #[strum(serialize = "150")]
    _150,
    /// Locations for tariff calculations
    ///
    /// A list of locations related to tariff calculations.
    #[strum(serialize = "151")]
    _151,
    ///
    /// Materials
    #[strum(serialize = "152")]
    _152,
    /// Methods of payment
    ///
    /// Identification of methods of payment.
    #[strum(serialize = "153")]
    _153,
    /// Bank branch sorting identification
    ///
    /// Identification of a specific branch of a bank.
    #[strum(serialize = "154")]
    _154,
    /// Automated clearing house
    ///
    /// Identification of automated clearing houses.
    #[strum(serialize = "155")]
    _155,
    /// Location of goods
    ///
    /// (3384) Indication of the place where goods are located and where they are available for examination.
    #[strum(serialize = "156")]
    _156,
    /// Clearing code
    ///
    /// Identification of the responsible bank/clearing house which has cleared or is ordered to do the clearing.
    #[strum(serialize = "157")]
    _157,
    /// Terms of delivery
    ///
    /// Code to identify terms of delivery.
    #[strum(serialize = "158")]
    _158,
    /// Party identification
    ///
    /// Identification of parties, corporates, etc.
    #[strum(serialize = "160")]
    _160,
    /// Goods description
    ///
    /// Identification of a type of goods description.
    #[strum(serialize = "161")]
    _161,
    /// Country
    ///
    /// Identification of a country.
    #[strum(serialize = "162")]
    _162,
    /// Country sub-entity
    ///
    /// (3228) Identification of country sub-entity (region, department, state, province) defined by appropriate authority.
    #[strum(serialize = "163")]
    _163,
    /// Member organizations
    ///
    /// Identification of member organizations.
    #[strum(serialize = "164")]
    _164,
    /// Amendment code (Customs)
    ///
    /// Customs code indicating the reason for transmitting an amendment to Customs.
    #[strum(serialize = "165")]
    _165,
    /// Social security identification
    ///
    /// Code assigned by the authority competent to issue social security identification to identify a person.
    #[strum(serialize = "166")]
    _166,
    /// Tax party identification
    ///
    /// Code assigned by a tax authority to identify a party.
    #[strum(serialize = "167")]
    _167,
    /// Rail document names
    ///
    /// Rail specific identifications of documents.
    #[strum(serialize = "168")]
    _168,
    /// Harmonized system
    ///
    /// Identification of commodities according to the Harmonized System.
    #[strum(serialize = "169")]
    _169,
    ///
    /// Bank securities code
    #[strum(serialize = "170")]
    _170,
    /// Carriers
    ///
    /// Code list identifying carriers.
    #[strum(serialize = "172")]
    _172,
    /// Export requirements
    ///
    /// Identification of requirements and regulations established by relevant authorities concerning exportation.
    #[strum(serialize = "173")]
    _173,
    /// Citizen identification
    #[strum(serialize = "174")]
    _174,
    /// Account analysis codes
    ///
    /// Account service charges list.
    #[strum(serialize = "175")]
    _175,
    /// Flow of the goods
    ///
    /// List of statistical codes covering the movement of the goods to be declared (e.g. despatch, arrival).
    #[strum(serialize = "176")]
    _176,
    /// Statistical procedures
    ///
    /// Indication of the statistical procedure to which the goods are subject.
    #[strum(serialize = "177")]
    _177,
    /// Standard text according US embargo regulations
    ///
    /// US government regulations prescribe specific standard text usage. Using codes from this code list prevents full text transmission.
    #[strum(serialize = "178")]
    _178,
    /// Standard text for export according national prescriptions
    ///
    /// National export regulations prescribe specific standard text usage. Using codes from this code list prevents full text transmission.
    #[strum(serialize = "179")]
    _179,
    /// Airport terminal
    ///
    /// Code identifying terminals or other sub-locations at airports.
    #[strum(serialize = "180")]
    _180,
    /// Activity
    ///
    /// Code identifying activities.
    #[strum(serialize = "181")]
    _181,
    /// Combiterms 1990
    ///
    /// Code to indicate the applicable Combiterm (1990 edition), used for the purpose of cost distribution between seller according to Incoterms 1990.
    #[strum(serialize = "182")]
    _182,
    /// Dangerous goods packing type
    ///
    /// Identification of package types for the description related to dangerous goods.
    #[strum(serialize = "183")]
    _183,
    /// Tax assessment method
    ///
    /// A code to identify the tax assessment method.
    #[strum(serialize = "184")]
    _184,
    /// Item type
    ///
    /// A code list defining the level of elaboration of a item such as raw material, component, tooling, etc.
    #[strum(serialize = "185")]
    _185,
    /// Product supply condition
    ///
    /// A code list specifying the rules according to which a product is supplied, e.g. from stock, available on demand, make on order, etc.
    #[strum(serialize = "186")]
    _186,
    /// Supplier's stock turnover
    ///
    /// A code list giving an indication about the level of the supplier's stock turnover.
    #[strum(serialize = "187")]
    _187,
    /// Article status
    ///
    /// A code list defining the status of an article from the procurement point of view, e.g. new article, critical article, etc.
    #[strum(serialize = "188")]
    _188,
    /// Quality control code
    ///
    /// A code list specifying how the article is classified according to the quality control point of view, e.g. safety item, subject to regulation, etc.
    #[strum(serialize = "189")]
    _189,
    /// Item sourcing category
    ///
    /// A code list to specify details related to the sourcing of the corresponding item such as provided by the buyer, from a mandatory source, etc.
    #[strum(serialize = "190")]
    _190,
    /// Dumping or countervailing assessment method
    ///
    /// A code to identify the method used to determine the dumping or countervailing duty.
    #[strum(serialize = "191")]
    _191,
    /// Dumping specification
    ///
    /// Code list to identify types of goods for dumping purposes.
    #[strum(serialize = "192")]
    _192,
    /// Legal event
    ///
    /// Identifies a code list of legal events.
    #[strum(serialize = "193")]
    _193,
    /// Record precedence based on its currency in time
    ///
    /// Identifies the priority of a record based on its currency in time.
    #[strum(serialize = "194")]
    _194,
    /// Ownership rights
    ///
    /// Identifies a code list containing types of ownership rights.
    #[strum(serialize = "195")]
    _195,
    /// Property ownership extent
    ///
    /// Identifies a code list containing the extent of legal rights of possession to property.
    #[strum(serialize = "196")]
    _196,
    /// Monetary function detail
    ///
    /// Identifies a code list containing monetary function details.
    #[strum(serialize = "197")]
    _197,
    /// Account relationship type
    ///
    /// Identifies a code list containing types of account relationships.
    #[strum(serialize = "198")]
    _198,
    /// Account rating
    ///
    /// Identifies the code list containing account rating types.
    #[strum(serialize = "199")]
    _199,
    /// Loan type
    ///
    /// Identifies the code list of loan types.
    #[strum(serialize = "200")]
    _200,
    /// Claim type
    ///
    /// Identifies the code list containing the claim types.
    #[strum(serialize = "201")]
    _201,
    /// Legal case type
    ///
    /// Identifies the code list containing the type of legal cases.
    #[strum(serialize = "202")]
    _202,
    /// Court of law event type
    ///
    /// Identifies the code list containing the type of law events.
    #[strum(serialize = "203")]
    _203,
    /// Notice type
    ///
    /// Identifies the code list containing the type of notice.
    #[strum(serialize = "204")]
    _204,
    /// Ethnicity
    ///
    /// Identifies the code list containing ethnic types.
    #[strum(serialize = "205")]
    _205,
    /// Individual participation in company
    ///
    /// Identifies the code list containing the types of participation of an individual within a company.
    #[strum(serialize = "206")]
    _206,
    /// Real estate asset type
    ///
    /// Identifies the code list containing the types of real estate assets.
    #[strum(serialize = "207")]
    _207,
    /// Asset recurrence
    ///
    /// Identifies the code list containing the types of recurrences of assets.
    #[strum(serialize = "208")]
    _208,
    /// Construction material
    ///
    /// Identifies the code list containing types of materials used for construction.
    #[strum(serialize = "209")]
    _209,
    /// Information request type
    ///
    /// Identifies a code list containing types of information requests.
    #[strum(serialize = "210")]
    _210,
    /// Business change
    ///
    /// Identifies a code list containing business change types.
    #[strum(serialize = "211")]
    _211,
    /// Business credit rating
    ///
    /// Identifies a code list containing business credit rating types.
    #[strum(serialize = "212")]
    _212,
    /// Corporate financial filing criteria
    ///
    /// Identifies a code list containing criteria for corporate financial filings.
    #[strum(serialize = "213")]
    _213,
    /// Reason for public record filing
    ///
    /// Identifies a code list containing reasons for public record filings.
    #[strum(serialize = "214")]
    _214,
    /// Registration type
    ///
    /// Identifies a code list containing registration types.
    #[strum(serialize = "215")]
    _215,
    /// Stock exchange detail
    ///
    /// Identifies a code list containing stock exchange details.
    #[strum(serialize = "216")]
    _216,
    /// Business legal structure type
    ///
    /// Identifies a code list containing business legal structure details.
    #[strum(serialize = "217")]
    _217,
    /// Information request result
    ///
    /// Identifies a code list containing information request results.
    #[strum(serialize = "218")]
    _218,
    /// Financial information type
    ///
    /// Identifies a code list containing financial information types.
    #[strum(serialize = "219")]
    _219,
    /// Consolidation detail
    ///
    /// Identifies a code list containing consolidation details.
    #[strum(serialize = "220")]
    _220,
    /// Condition detail
    ///
    /// Identifies a code list containing condition details.
    #[strum(serialize = "221")]
    _221,
    /// Financial statement format
    ///
    /// Identifies a code list containing financial statement formats.
    #[strum(serialize = "222")]
    _222,
    /// Source of disclosure
    ///
    /// Identifies a code list containing disclosure sources.
    #[strum(serialize = "223")]
    _223,
    /// General territory type
    ///
    /// Identifies a code list containing general territory types.
    #[strum(serialize = "224")]
    _224,
    /// Roadway type
    ///
    /// Identifies a code list containing roadway types.
    #[strum(serialize = "225")]
    _225,
    /// Roadway detail
    ///
    /// Identifies a code list containing roadway details.
    #[strum(serialize = "226")]
    _226,
    /// City
    ///
    /// Identifies a code list containing cities.
    #[strum(serialize = "227")]
    _227,
    /// County
    ///
    /// Identifies a code list containing counties. A county is any of the territorial divisions of some countries, forming the chief unit of local administration.
    #[strum(serialize = "228")]
    _228,
    /// Geographic location
    ///
    /// Identifies a code list containing geographic locations.
    #[strum(serialize = "229")]
    _229,
    /// Entity relationship
    ///
    /// Identifies a code list of entity relationships.
    #[strum(serialize = "230")]
    _230,
    /// Payment behaviour rating
    ///
    /// Identifies a code list containing payment behaviour ratings.
    #[strum(serialize = "231")]
    _231,
    /// Inquiry selection
    ///
    /// Identifies a code list containing inquiry selections.
    #[strum(serialize = "232")]
    _232,
    /// Rating summary value
    ///
    /// Identifies a code list containing rating summary values.
    #[strum(serialize = "233")]
    _233,
    /// Industry rating
    ///
    /// Identifies a code list containing industry ratings.
    #[strum(serialize = "234")]
    _234,
    /// Forecast type
    ///
    /// Identifies a code list containing forecast types.
    #[strum(serialize = "235")]
    _235,
    /// Hobby
    ///
    /// Identifies a code list containing hobby types.
    #[strum(serialize = "236")]
    _236,
    /// Functional business area
    ///
    /// Identifies a code list containing functional business areas.
    #[strum(serialize = "237")]
    _237,
    /// Current asset details
    ///
    /// Identifies a code list containing details of the current asset types.
    #[strum(serialize = "238")]
    _238,
    /// Asset details
    ///
    /// Identifies a code list containing details of the asset types.
    #[strum(serialize = "239")]
    _239,
    /// Current liability details
    ///
    /// Identifies a code list containing the current liability types.
    #[strum(serialize = "240")]
    _240,
    /// Liability details
    ///
    /// Identifies a code list containing details of liability types.
    #[strum(serialize = "241")]
    _241,
    /// Financial item reclassification
    ///
    /// Identifies a code list containing financial item reclassifications.
    #[strum(serialize = "242")]
    _242,
    /// Financial item allocation
    ///
    /// Identifies a code list containing financial item allocations.
    #[strum(serialize = "243")]
    _243,
    /// Reason for financial item detail change
    ///
    /// Identifies a code list containing reasons for the change in financial item details.
    #[strum(serialize = "244")]
    _244,
    /// Educational institution type
    ///
    /// Identifies a code list containing educational institution types.
    #[strum(serialize = "245")]
    _245,
    /// Educational study area
    ///
    /// Identifies a code list containing educational study areas.
    #[strum(serialize = "246")]
    _246,
    /// Security share type
    ///
    /// Identifies a code list containing security share types.
    #[strum(serialize = "247")]
    _247,
    /// Insurance coverage detail
    ///
    /// Identifies a code list containing insurance coverage details.
    #[strum(serialize = "248")]
    _248,
    /// Property type
    ///
    /// Identifies a code list containing property types.
    #[strum(serialize = "249")]
    _249,
    /// Data category
    ///
    /// Identifies a code list containing data categories.
    #[strum(serialize = "250")]
    _250,
    /// Information type
    ///
    /// Identifies a code list containing types of information.
    #[strum(serialize = "251")]
    _251,
    /// Court of law type
    ///
    /// Identifies a code list containing court of law types.
    #[strum(serialize = "252")]
    _252,
    /// Region
    ///
    /// Identifies a code list containing regions that identify an area of the earth's surface, having definable boundaries or characteristics.
    #[strum(serialize = "253")]
    _253,
    /// Postal service carrier route
    ///
    /// Identifies a code list containing routes covered by a postal service carrier.
    #[strum(serialize = "254")]
    _254,
    /// Continent
    ///
    /// Identifies a code list containing continents, that are any of the main continuous expanses of land.
    #[strum(serialize = "255")]
    _255,
    /// Postal district
    ///
    /// Identifies a code list containing territories for the routing of mail.
    #[strum(serialize = "256")]
    _256,
    /// Non-postal town
    ///
    /// Identifies a code list containing towns not recognised as a postal entity.
    #[strum(serialize = "257")]
    _257,
    /// City subdivision
    ///
    /// Identifies a code list containing subdivisions of a city.
    #[strum(serialize = "258")]
    _258,
    /// Financial analysis categories
    ///
    /// Identifies a code list containing financial analysis categories.
    #[strum(serialize = "259")]
    _259,
    /// Accord Europeen relatif au transport international des marchandises(ADR).
    ///
    /// A code list identifying dangerous goods for transport purposes.
    #[strum(serialize = "260")]
    _260,
    /// Consignee's premises
    ///
    /// Facility controlled by the consignee of cargo.
    #[strum(serialize = "261")]
    _261,
    /// Consignor's premises
    ///
    /// Facility controlled by the consignor of cargo.
    #[strum(serialize = "262")]
    _262,
    /// Packing and/or unpacking facility
    ///
    /// Facility dedicated to the packing and/or unpacking of cargo.
    #[strum(serialize = "263")]
    _263,
    /// Storage facility
    ///
    /// Facility at which goods are stored.
    #[strum(serialize = "264")]
    _264,
    /// Repair facility
    ///
    /// Facility at which repairs are carried out.
    #[strum(serialize = "265")]
    _265,
    /// Marine berth
    ///
    /// The location within a port where a ship anchors or ties up.
    #[strum(serialize = "266")]
    _266,
    /// Marine wharf
    ///
    /// Landing platform where a ship can load and unload.
    #[strum(serialize = "267")]
    _267,
    /// Gate
    ///
    /// The location at which access to or from a facility is controlled.
    #[strum(serialize = "268")]
    _268,
    /// Warehouse
    ///
    /// A covered facility for the storage and distribution of goods.
    #[strum(serialize = "269")]
    _269,
    /// Business classification
    ///
    /// Code list of business classifications.
    #[strum(serialize = "270")]
    _270,
    /// Facility security clearance
    ///
    /// Code list specifying the security clearance assigned to a facility.
    #[strum(serialize = "271")]
    _271,
    /// Individual security clearance
    ///
    /// Code list specifying the security clearance assigned to an individual.
    #[strum(serialize = "272")]
    _272,
    /// Means of communications identifier
    ///
    /// Code list of communication means used to transmit data.
    #[strum(serialize = "273")]
    _273,
    /// Mutually defined
    #[strum(ascii_case_insensitive)]
    _ZZZ,
}

/// Date or time or period function code qualifier
///
/// Code qualifying the function of a date, time or period.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _2005 {
    /// Service completion date/time, actual
    ///
    /// Actual date/time on which the service was completed.
    #[strum(serialize = "1")]
    _1,
    /// Delivery date/time, requested
    ///
    /// Date on which buyer requests goods to be delivered.
    #[strum(serialize = "2")]
    _2,
    /// Invoice date/time
    ///
    /// [2376] Date when a Commercial Invoice is issued.
    #[strum(serialize = "3")]
    _3,
    /// Order date/time
    ///
    /// [2010] Date when an order is issued.
    #[strum(serialize = "4")]
    _4,
    /// Saleable stock demand cover period, expected
    ///
    /// A period of time when saleable stocks are expected to cover demand for a product.
    #[strum(serialize = "5")]
    _5,
    /// Moved from location date
    ///
    /// The date an entity moved from a location.
    #[strum(serialize = "6")]
    _6,
    /// Effective date/time
    ///
    /// Date and/or time at which specified event or document becomes effective.
    #[strum(serialize = "7")]
    _7,
    /// Order received date/time
    ///
    /// Date/time when the purchase order is received by the seller.
    #[strum(serialize = "8")]
    _8,
    /// Processing date/time
    ///
    /// Date/time of processing.
    #[strum(serialize = "9")]
    _9,
    /// Shipment date/time, requested
    ///
    /// Date on which goods should be shipped or despatched by the supplier.
    #[strum(serialize = "10")]
    _10,
    /// Despatch date and or time
    ///
    /// (2170) Date/time on which the goods are or are expected to be despatched or shipped.
    #[strum(serialize = "11")]
    _11,
    /// Terms discount due date/time
    ///
    /// Date by which payment should be made if discount terms are to apply.
    #[strum(serialize = "12")]
    _12,
    /// Terms net due date
    ///
    /// Date by which payment must be made.
    #[strum(serialize = "13")]
    _13,
    /// Payment date/time, deferred
    ///
    /// Date/time when instalments are due.
    #[strum(serialize = "14")]
    _14,
    /// Promotion start date/time
    ///
    /// Date/time when promotion activities begin.
    #[strum(serialize = "15")]
    _15,
    /// Promotion end date/time
    ///
    /// Date/time when promotion activities end.
    #[strum(serialize = "16")]
    _16,
    /// Delivery date/time, estimated
    ///
    /// Date and/or time when the shipper of the goods expects delivery will take place.
    #[strum(serialize = "17")]
    _17,
    /// Installation date/time/period
    ///
    ///
    #[strum(serialize = "18")]
    _18,
    /// Meat ageing period
    ///
    /// Period of time between slaughter and delivery during which meat is ageing.
    #[strum(serialize = "19")]
    _19,
    /// Cheque date/time
    ///
    /// Date/time when cheque is issued.
    #[strum(serialize = "20")]
    _20,
    /// Charge back date/time
    ///
    /// Description to be provided.
    #[strum(serialize = "21")]
    _21,
    /// Freight bill date/time
    ///
    /// Date/time when freight bill is issued.
    #[strum(serialize = "22")]
    _22,
    /// Equipment reconditioning date/time, actual
    ///
    /// Actual date/time of the reconditioning of a piece of equipment.
    #[strum(serialize = "23")]
    _23,
    /// Transfer note acceptance date and time
    ///
    /// Date and time when a transfer note (transfer document for transport exclusively using containers as equipment) is recognised as being valid by the carrier.
    #[strum(serialize = "24")]
    _24,
    /// Delivery date/time, actual
    ///
    /// Date/time on which goods or consignment are delivered at their destination.
    #[strum(serialize = "35")]
    _35,
    /// Expiry date
    ///
    /// Date of expiry of the validity of a referenced document, price information or any other referenced data element with a limited validity period.
    #[strum(serialize = "36")]
    _36,
    /// Ship not before date/time
    ///
    /// Goods should not be shipped before given date/time.
    #[strum(serialize = "37")]
    _37,
    /// Ship not later than date/time
    ///
    /// Date/time by which the goods should have been shipped.
    #[strum(serialize = "38")]
    _38,
    /// Ship week of date
    ///
    /// Date identifying the week during which goods should be shipped.
    #[strum(serialize = "39")]
    _39,
    /// Clinical information issue date and/or time
    ///
    /// Date and/or time when clinical information is issued.
    #[strum(serialize = "40")]
    _40,
    /// Event duration, expected
    ///
    /// The expected duration of an event.
    #[strum(serialize = "41")]
    _41,
    /// Superseded date/time
    ///
    /// Date/time being overlaid by a date given elsewhere.
    #[strum(serialize = "42")]
    _42,
    /// Event duration, intended
    ///
    /// The intended duration of an event.
    #[strum(serialize = "43")]
    _43,
    /// Availability
    ///
    /// Date/time when received item is available.
    #[strum(serialize = "44")]
    _44,
    /// Compilation date and time
    ///
    /// Date and time of the compilation.
    #[strum(serialize = "45")]
    _45,
    /// Cancellation date
    ///
    /// Date on which a document or message has been cancelled.
    #[strum(serialize = "46")]
    _46,
    /// Statistical time series date
    ///
    /// Date for statistical time series purposes.
    #[strum(serialize = "47")]
    _47,
    /// Duration
    ///
    /// Duration.
    #[strum(serialize = "48")]
    _48,
    /// Deliver not before and not after dates
    ///
    /// Deliver not before and not after a specific date range.
    #[strum(serialize = "49")]
    _49,
    /// Goods receipt date/time
    ///
    /// Date/time upon which the goods were received by a given party.
    #[strum(serialize = "50")]
    _50,
    /// Cumulative quantity start date
    ///
    /// First Date for accumulation of delivery quantities.
    #[strum(serialize = "51")]
    _51,
    /// Cumulative quantity end date
    ///
    /// Last Date for accumulation of delivery quantities.
    #[strum(serialize = "52")]
    _52,
    /// Buyer's local time
    ///
    /// Time at the buyer's location.
    #[strum(serialize = "53")]
    _53,
    /// Seller's local time
    ///
    /// Time at the seller's location.
    #[strum(serialize = "54")]
    _54,
    /// Confirmed date/time
    ///
    /// Date/time which has been confirmed.
    #[strum(serialize = "55")]
    _55,
    /// Original authorisation date and/or time
    ///
    /// Date and/or time when original authorisation was issued.
    #[strum(serialize = "56")]
    _56,
    /// Precaution relevant period
    ///
    /// The period when a precaution is relevant.
    #[strum(serialize = "57")]
    _57,
    /// Clearance date (Customs)
    ///
    /// (3080) Date on which Customs formalities necessary to allow goods to be exported, to enter home use, or to be placed under another Customs procedure has been accomplished (CCC).
    #[strum(serialize = "58")]
    _58,
    /// Inbound movement authorization date
    ///
    /// Inland movement authorization date.
    #[strum(serialize = "59")]
    _59,
    /// Engineering change level date
    ///
    /// Date the engineering level of goods is changed.
    #[strum(serialize = "60")]
    _60,
    /// Cancel if not delivered by this date
    ///
    ///
    #[strum(serialize = "61")]
    _61,
    /// Excluded date
    ///
    /// Date excluded from a period of time.
    #[strum(serialize = "62")]
    _62,
    /// Delivery date/time, latest
    ///
    /// Date identifying a point of time after which goods shall not or will not be delivered.
    #[strum(serialize = "63")]
    _63,
    /// Delivery date/time, earliest
    ///
    /// Date identifying a point in time before which the goods shall not be delivered.
    #[strum(serialize = "64")]
    _64,
    /// Delivery date/time, 1st schedule
    ///
    ///
    #[strum(serialize = "65")]
    _65,
    /// Excluded period
    ///
    /// An interval of time excluded from a period of time.
    #[strum(serialize = "66")]
    _66,
    /// Delivery date/time, current schedule
    ///
    /// Delivery Date deriving from actual schedule.
    #[strum(serialize = "67")]
    _67,
    /// Additional period
    ///
    /// An interval of time added to a period of time.
    #[strum(serialize = "68")]
    _68,
    /// Delivery date/time, promised for
    ///
    /// [2138] Date by which, or period within which, the merchandise should be delivered to the buyer, as agreed between the seller and the buyer (generic term).
    #[strum(serialize = "69")]
    _69,
    /// Additional date
    ///
    /// Date added to a period of time.
    #[strum(serialize = "70")]
    _70,
    /// Delivery date/time, requested for (after and including)
    ///
    /// Delivery is requested to happen after or on given date.
    #[strum(serialize = "71")]
    _71,
    /// Delivery date/time, promised for (after and including)
    ///
    /// Delivery might take place earliest at given date.
    #[strum(serialize = "72")]
    _72,
    /// Guarantee period
    ///
    /// The period for which the guarantee is or will be granted.
    #[strum(serialize = "73")]
    _73,
    /// Delivery date/time, requested for (prior to and including)
    ///
    /// Delivery is requested to happen prior to or including the given date.
    #[strum(serialize = "74")]
    _74,
    /// Delivery date/time, promised for (prior to and including)
    ///
    /// Delivery might take place latest at given date.
    #[strum(serialize = "75")]
    _75,
    /// Delivery date/time, scheduled for
    ///
    ///
    #[strum(serialize = "76")]
    _76,
    /// Specification revision date
    ///
    /// Date of revision to a specification.
    #[strum(serialize = "77")]
    _77,
    /// Event date/time/period, actual
    ///
    /// The actual date/time/period an event occurred.
    #[strum(serialize = "78")]
    _78,
    /// Shipment date/time, promised for
    ///
    /// Shipment might happen at given date/time.
    #[strum(serialize = "79")]
    _79,
    /// Planning end date and/or time, actual
    ///
    /// The actual date and/or time the planning ended.
    #[strum(serialize = "80")]
    _80,
    /// Shipment date/time, requested for (after and including)
    ///
    /// Shipment should happen earliest at given date.
    #[strum(serialize = "81")]
    _81,
    /// Medicine administration time
    ///
    /// Designated time of day for the administration of medicine.
    #[strum(serialize = "82")]
    _82,
    /// Dispensing interval, minimum
    ///
    /// The shortest interval allowed between one dispensing of an item and the next dispensing of the same item.
    #[strum(serialize = "83")]
    _83,
    /// Shipment date/time, requested for (prior to and including)
    ///
    /// Shipment should take place latest at given date.
    #[strum(serialize = "84")]
    _84,
    /// Shipment date/time, promised for (prior to and including)
    ///
    /// Shipment might take place latest at given date.
    #[strum(serialize = "85")]
    _85,
    /// Medication date/time, start
    ///
    /// Date and/or time when medication was started.
    #[strum(serialize = "86")]
    _86,
    /// Travel service connection time
    ///
    /// Time elapsing between the arrival of a travel service and the departure of a connecting travel service.
    #[strum(serialize = "87")]
    _87,
    /// Summer time, start
    ///
    /// Date/time at which the summer time starts.
    #[strum(serialize = "88")]
    _88,
    /// Inquiry date
    ///
    ///
    #[strum(serialize = "89")]
    _89,
    /// Report start date
    ///
    ///
    #[strum(serialize = "90")]
    _90,
    /// Report end date
    ///
    ///
    #[strum(serialize = "91")]
    _91,
    /// Contract effective date
    ///
    /// Date when a contract becomes valid.
    #[strum(serialize = "92")]
    _92,
    /// Contract expiry date
    ///
    /// Date when a contract expires.
    #[strum(serialize = "93")]
    _93,
    /// Production/manufacture date
    ///
    /// Date on which goods are produced.
    #[strum(serialize = "94")]
    _94,
    /// Bill of lading date
    ///
    /// Date as specified on the bill of lading.
    #[strum(serialize = "95")]
    _95,
    /// Discharge date/time
    ///
    /// Date/time when goods should, might or have been discharged from the means of transport.
    #[strum(serialize = "96")]
    _96,
    /// Transaction creation date
    ///
    ///
    #[strum(serialize = "97")]
    _97,
    /// Winter time, start
    ///
    /// Date/time at which the winter time starts.
    #[strum(serialize = "98")]
    _98,
    /// Quotation opening date
    ///
    /// The date on which the quotation has been or may be opened.
    #[strum(serialize = "99")]
    _99,
    /// Product ageing period before delivery
    ///
    /// Period of time before delivery during which the product is ageing.
    #[strum(serialize = "100")]
    _100,
    /// Production date, no schedule established as of
    ///
    /// Date as of there is no valid production schedule.
    #[strum(serialize = "101")]
    _101,
    /// Health problem period
    ///
    /// Period of time of health problem.
    #[strum(serialize = "102")]
    _102,
    /// Deposit date/time
    ///
    ///
    #[strum(serialize = "107")]
    _107,
    /// Postmark date/time
    ///
    ///
    #[strum(serialize = "108")]
    _108,
    /// Receive at lockbox date
    ///
    /// The date on which a financial institution, serving as collection agency for a company located in another part of the country, collects an amount of money on behalf of that company.
    #[strum(serialize = "109")]
    _109,
    /// Ship date, originally scheduled
    ///
    /// The date on which the shipment of goods was originally scheduled.
    #[strum(serialize = "110")]
    _110,
    /// Manifest/ship notice date
    ///
    /// The date of issuance of a manifest or ship notice.
    #[strum(serialize = "111")]
    _111,
    /// First interest-bearing date
    ///
    /// The first date from which interest is borne.
    #[strum(serialize = "112")]
    _112,
    /// Sample required date
    ///
    /// Date as of a sample has to be available customer defined.
    #[strum(serialize = "113")]
    _113,
    /// Tooling required date
    ///
    /// Date as of a tool has to be available customer defined.
    #[strum(serialize = "114")]
    _114,
    /// Sample available date
    ///
    /// Date as of a sample will be available seller defined.
    #[strum(serialize = "115")]
    _115,
    /// Equipment return period, expected
    ///
    /// Period until which equipment is expected to be hired.
    #[strum(serialize = "116")]
    _116,
    /// Delivery date/time, first
    ///
    /// First possible date/time for delivery.
    #[strum(serialize = "117")]
    _117,
    /// Cargo booking confirmed date/time
    ///
    /// Date/time at which the cargo booking has been accepted by the carrier.
    #[strum(serialize = "118")]
    _118,
    /// Test completion date
    ///
    /// Date when a test has been completed.
    #[strum(serialize = "119")]
    _119,
    /// Last interest-bearing date
    ///
    /// The last date from which interest is borne.
    #[strum(serialize = "120")]
    _120,
    /// Entry date
    ///
    /// Date of entry.
    #[strum(serialize = "121")]
    _121,
    /// Contract completion date
    ///
    /// The date a contract is completed.
    #[strum(serialize = "122")]
    _122,
    /// Documentary credit expiry date/time
    ///
    /// The latest date/time for presentation of the documents to the bank where the credit expires.
    #[strum(serialize = "123")]
    _123,
    /// Despatch note date
    ///
    /// [2218] Date when a Despatch Note is issued.
    #[strum(serialize = "124")]
    _124,
    /// Import licence date
    ///
    /// [2292] Date when Import Licence is issued.
    #[strum(serialize = "125")]
    _125,
    /// Contract date
    ///
    /// [2326] Date when a Contract is agreed.
    #[strum(serialize = "126")]
    _126,
    /// Previous report date
    ///
    /// Date of the previous report.
    #[strum(serialize = "127")]
    _127,
    /// Delivery date/time, last
    ///
    /// Date when the last delivery should be or has been accomplished.
    #[strum(serialize = "128")]
    _128,
    /// Exportation date
    ///
    /// Date when imported vessel/merchandise last left the country of export for the country of import.
    #[strum(serialize = "129")]
    _129,
    /// Current report date
    ///
    /// Date of the current report.
    #[strum(serialize = "130")]
    _130,
    /// Tax point date
    ///
    /// Date on which tax is due or calculated.
    #[strum(serialize = "131")]
    _131,
    /// Arrival date/time, estimated
    ///
    /// (2348) Date/time when carrier estimates that a means of transport should arrive at the port of discharge or place of destination.
    #[strum(serialize = "132")]
    _132,
    /// Departure date/time, estimated
    ///
    /// Date/time when carrier estimates that a means of transport should depart at the place of departure.
    #[strum(serialize = "133")]
    _133,
    /// Rate of exchange date/time
    ///
    /// Date/time on which the exchange rate was fixed.
    #[strum(serialize = "134")]
    _134,
    /// Telex date
    ///
    /// Date identifying when a telex message was sent.
    #[strum(serialize = "135")]
    _135,
    /// Departure date/time
    ///
    /// [2280] Date (and time) of departure of means of transport.
    #[strum(serialize = "136")]
    _136,
    /// Document/message date/time
    ///
    /// (2006) Date/time when a document/message is issued. This may include authentication.
    #[strum(serialize = "137")]
    _137,
    /// Payment date
    ///
    /// [2034] Date on which an amount due is made available to the creditor, in accordance with the terms of payment.
    #[strum(serialize = "138")]
    _138,
    /// Payment due date
    ///
    /// Date/time at which funds should be made available.
    #[strum(serialize = "140")]
    _140,
    /// Presentation date of Goods declaration (Customs)
    ///
    /// [2032] Date on which a Goods declaration is presented or lodged with Customs.
    #[strum(serialize = "141")]
    _141,
    /// Labour wage determination date
    ///
    /// The date a labour wage is determined.
    #[strum(serialize = "142")]
    _142,
    /// Acceptance date/time of goods
    ///
    /// [2126] Date on which the goods are taken over by the carrier at the place of acceptance (CMR 4).
    #[strum(serialize = "143")]
    _143,
    /// Quota date
    ///
    /// Date that the quota applies to.
    #[strum(serialize = "144")]
    _144,
    /// Event date
    ///
    /// A date specifying an event.
    #[strum(serialize = "145")]
    _145,
    /// Entry date, estimated (Customs)
    ///
    /// Date on which the official date of Customs entry is anticipated.
    #[strum(serialize = "146")]
    _146,
    /// Expiry date of export licence
    ///
    /// [2078] Date of expiry of the validity of an Export Licence.
    #[strum(serialize = "147")]
    _147,
    /// Acceptance date of Goods declaration (Customs)
    ///
    /// [2036] Date on which a Goods declaration is accepted by Customs in accordance with Customs legislation.
    #[strum(serialize = "148")]
    _148,
    /// Invoice date, required
    ///
    /// Date required for invoice issue.
    #[strum(serialize = "149")]
    _149,
    /// Declaration/presentation date
    ///
    /// Date when item has been or has to be declared/presented.
    #[strum(serialize = "150")]
    _150,
    /// Importation date
    ///
    /// Date on which goods are imported, as determined by the governing Customs administration.
    #[strum(serialize = "151")]
    _151,
    /// Exportation date for textiles
    ///
    /// Date when imported textiles last left the country of origin for the country of importation.
    #[strum(serialize = "152")]
    _152,
    /// Cancellation date/time, latest
    ///
    /// The latest date/time on which cancellation of the payment order may be requested.
    #[strum(serialize = "153")]
    _153,
    /// Acceptance date of document
    ///
    /// The date on which a document was accepted.
    #[strum(serialize = "154")]
    _154,
    /// Accounting period start date
    ///
    ///
    #[strum(serialize = "155")]
    _155,
    /// Accounting period end date
    ///
    ///
    #[strum(serialize = "156")]
    _156,
    /// Validity start date
    ///
    ///
    #[strum(serialize = "157")]
    _157,
    /// Horizon start date
    ///
    /// The first date of a period forming a horizon.
    #[strum(serialize = "158")]
    _158,
    /// Horizon end date
    ///
    /// The last date of a period forming a horizon.
    #[strum(serialize = "159")]
    _159,
    /// Authorization date
    ///
    /// Date when an authorization was given.
    #[strum(serialize = "160")]
    _160,
    /// Release date of customer
    ///
    /// Date the customer authorised the goods' release.
    #[strum(serialize = "161")]
    _161,
    /// Release date of supplier
    ///
    /// Date when the supplier released goods.
    #[strum(serialize = "162")]
    _162,
    /// Processing start date/time
    ///
    /// Date/Time when a specific process starts.
    #[strum(serialize = "163")]
    _163,
    /// Processing end date/time
    ///
    /// Date/Time when a specific process ends.
    #[strum(serialize = "164")]
    _164,
    /// Tax period start date
    ///
    /// Date when a tax period begins.
    #[strum(serialize = "165")]
    _165,
    /// Tax period end date
    ///
    /// Date when a tax period ends.
    #[strum(serialize = "166")]
    _166,
    /// Charge period start date
    ///
    /// The charge period's first date.
    #[strum(serialize = "167")]
    _167,
    /// Charge period end date
    ///
    /// The charge period's last date.
    #[strum(serialize = "168")]
    _168,
    /// Lead time
    ///
    /// Time required between order entry till earliest goods delivery.
    #[strum(serialize = "169")]
    _169,
    /// Settlement due date
    ///
    /// More generic than 'payment due date' and therefore more apt for reinsurance/insurance business.
    #[strum(serialize = "170")]
    _170,
    /// Reference date/time
    ///
    /// Date/time on which the reference was issued.
    #[strum(serialize = "171")]
    _171,
    /// Hired from date
    ///
    /// Date from which an item has been or will be hired.
    #[strum(serialize = "172")]
    _172,
    /// Hired until date
    ///
    /// Date until which an item has been or will be hired.
    #[strum(serialize = "173")]
    _173,
    /// Advise after date/time
    ///
    /// The information must be advised after the date/time indicated.
    #[strum(serialize = "174")]
    _174,
    /// Advise before date/time
    ///
    /// The information must be advised before the date/time indicated.
    #[strum(serialize = "175")]
    _175,
    /// Advise completed date/time
    ///
    /// The advise has been completed at the date indicated.
    #[strum(serialize = "176")]
    _176,
    /// Advise on date/time
    ///
    /// The information must be advised on the date/time indicated.
    #[strum(serialize = "177")]
    _177,
    /// Arrival date/time, actual
    ///
    /// [2106] Date (and time) of arrival of means of transport.
    #[strum(serialize = "178")]
    _178,
    /// Booking date/time
    ///
    /// Date at which the booking was made.
    #[strum(serialize = "179")]
    _179,
    /// Closing date/time
    ///
    /// Final date for delivering cargo to a liner ship.
    #[strum(serialize = "180")]
    _180,
    /// Positioning date/time of equipment
    ///
    /// Date/time when equipment is positioned.
    #[strum(serialize = "181")]
    _181,
    /// Issue date
    ///
    /// Date when a document/message has been or will be issued.
    #[strum(serialize = "182")]
    _182,
    /// Date, as at
    ///
    /// Date related to a given context.
    #[strum(serialize = "183")]
    _183,
    /// Notification date/time
    ///
    /// Date/time of notification.
    #[strum(serialize = "184")]
    _184,
    /// Commenced tank cleaning date/time
    ///
    /// The date/and or time tank cleaning was started.
    #[strum(serialize = "185")]
    _185,
    /// Departure date/time, actual
    ///
    /// (2280) Date (and time) of departure of means of transport.
    #[strum(serialize = "186")]
    _186,
    /// Authentication date/time of document
    ///
    /// Date/time when the document is signed or otherwise authenticated.
    #[strum(serialize = "187")]
    _187,
    /// Previous current account date
    ///
    /// Date of the previous current account.
    #[strum(serialize = "188")]
    _188,
    /// Departure date/time, scheduled
    ///
    /// Date (and time) of scheduled departure of means of transport.
    #[strum(serialize = "189")]
    _189,
    /// Transhipment date/time
    ///
    /// Date and time of the transfer of the goods from one means of transport to another.
    #[strum(serialize = "190")]
    _190,
    /// Delivery date/time, expected
    ///
    /// Date/time on which goods are expected to be delivered.
    #[strum(serialize = "191")]
    _191,
    /// Expiration date/time of customs document
    ///
    /// Date on which validity of a customs document expires.
    #[strum(serialize = "192")]
    _192,
    /// Execution date
    ///
    /// The date when ordered bank initiated the transaction.
    #[strum(serialize = "193")]
    _193,
    /// Start date/time
    ///
    /// Date/time on which a period starts.
    #[strum(serialize = "194")]
    _194,
    /// Expiry date of import licence
    ///
    /// [2272] Date of expiry of the validity of an Import Licence.
    #[strum(serialize = "195")]
    _195,
    /// Departure date/time, earliest
    ///
    /// Date/time of earliest departure of means of transport.
    #[strum(serialize = "196")]
    _196,
    /// Lay-time first day
    ///
    /// First of a number of days allowed in a charter party of the loading and discharging of cargo.
    #[strum(serialize = "197")]
    _197,
    /// Lay-time last day
    ///
    /// Last of a number of days allowed in a charter party for the loading and discharging of cargo.
    #[strum(serialize = "198")]
    _198,
    /// Positioning date/time of goods
    ///
    /// The date and/or time the goods have to be or have been positioned.
    #[strum(serialize = "199")]
    _199,
    /// Pick-up/collection date/time of cargo
    ///
    /// Date/time at which the cargo is picked up.
    #[strum(serialize = "200")]
    _200,
    /// Pick-up date/time of equipment
    ///
    /// Date/time at which the equipment is picked up.
    #[strum(serialize = "201")]
    _201,
    /// Posting date
    ///
    /// The date when an entry is posted to an account.
    #[strum(serialize = "202")]
    _202,
    /// Execution date/time, requested
    ///
    /// The date/time on which the ordered bank is requested to initiate the payment order, as specified by the originator (e.g. the date of the debit).
    #[strum(serialize = "203")]
    _203,
    /// Release date (Customs)
    ///
    /// Date on which Customs releases merchandise to the carrier or importer.
    #[strum(serialize = "204")]
    _204,
    /// Settlement date
    ///
    /// Date for settlement of financial transaction e.g. foreign exchange securities.
    #[strum(serialize = "205")]
    _205,
    /// End date/time
    ///
    /// Date/time on which a period (from - to) ends.
    #[strum(serialize = "206")]
    _206,
    /// Commenced pumping ballast date/time
    ///
    /// Date/time on which the intake of materials to be carried to improve the trim and the stability of the means of transport, was commenced.
    #[strum(serialize = "207")]
    _207,
    /// Departure date/time, ultimate
    ///
    /// Date/time at which a means of transport has to depart ultimately.
    #[strum(serialize = "208")]
    _208,
    /// Value date
    ///
    /// Date on which the funds are at the disposal of the beneficiary or cease to be at the disposal of the ordering customer.
    #[strum(serialize = "209")]
    _209,
    /// Reinsurance current account period
    ///
    /// Description to be provided.
    #[strum(serialize = "210")]
    _210,
    /// 360/30
    ///
    /// Calculation is based on year of 360 days, month of 30 days.
    #[strum(serialize = "211")]
    _211,
    /// 360/28-31
    ///
    /// Calculation is based on year of 360 days, month of 28-31 days.
    #[strum(serialize = "212")]
    _212,
    /// 365-6/30
    ///
    /// Calculation is based on year of 365-6 days, month of 30 days.
    #[strum(serialize = "213")]
    _213,
    /// 365-6/28-31
    ///
    /// Calculation is based on year of 365-6 days, month of 28- 31 days.
    #[strum(serialize = "214")]
    _214,
    /// 365/28-31
    ///
    /// Calculation is based on year of 365 days, month of 28-31 days.
    #[strum(serialize = "215")]
    _215,
    /// 365/30
    ///
    /// Calculation is based on year of 365 days, month of 30 days.
    #[strum(serialize = "216")]
    _216,
    /// From date of award to latest delivery
    ///
    /// Lead time to determine the latest date a delivery can be made based on the date an award is made.
    #[strum(serialize = "217")]
    _217,
    /// Authentication/validation date/time
    ///
    ///
    #[strum(serialize = "218")]
    _218,
    /// Crossborder date/time
    ///
    /// Date/time at which goods are transferred across a country border.
    #[strum(serialize = "219")]
    _219,
    /// Interest period
    ///
    /// Number of days used for the calculation of interests.
    #[strum(serialize = "221")]
    _221,
    /// Presentation date, latest
    ///
    /// Latest date for presentation of a document.
    #[strum(serialize = "222")]
    _222,
    /// Delivery date/time, deferred
    ///
    /// New date and time of delivery calculated on basis of a consignee's requirement (chargeable).
    #[strum(serialize = "223")]
    _223,
    /// Permit to admit date
    ///
    /// Date on which permission was granted to move merchandise into a bonded warehouse or free trade zone.
    #[strum(serialize = "224")]
    _224,
    /// Certification of weight date/time
    ///
    /// Date/time at which the carrier proceeds to the weighting of the goods.
    #[strum(serialize = "225")]
    _225,
    /// Discrepancy date/time
    ///
    /// Date/time at which a discrepancy has been found.
    #[strum(serialize = "226")]
    _226,
    /// Beneficiary's banks due date
    ///
    /// Date on which funds should be made available to the beneficiary's bank.
    #[strum(serialize = "227")]
    _227,
    /// Debit value date, requested
    ///
    /// Date on which the account owner wants the debit value to his account.
    #[strum(serialize = "228")]
    _228,
    /// Hoses connected date/time
    ///
    /// The date and/or time hoses were connected.
    #[strum(serialize = "229")]
    _229,
    /// Hoses disconnected date/time
    ///
    /// The date and/or time hoses were disconnected.
    #[strum(serialize = "230")]
    _230,
    /// Arrival date/time, earliest
    ///
    /// Date/time of earliest arrival of means of transport.
    #[strum(serialize = "231")]
    _231,
    /// Arrival date/time, scheduled
    ///
    /// Date (and time) of scheduled arrival of means of transport.
    #[strum(serialize = "232")]
    _232,
    /// Arrival date/time, ultimate
    ///
    /// Date (and time) of ultimate arrival of means of transport.
    #[strum(serialize = "233")]
    _233,
    /// Collection date/time, earliest
    ///
    /// The transport order may be issued before the goods are ready for picking up. This date/time indicates from when on the carrier can have access to the consignment.
    #[strum(serialize = "234")]
    _234,
    /// Collection date/time, latest
    ///
    /// In relation with the arrangements agreed between buyer and seller or between sender and main transport it may be necessary to specify the latest collection date/time.
    #[strum(serialize = "235")]
    _235,
    /// Completed pumping ballast date/time
    ///
    /// Date/time at which the intake of materials, to be carried to improve the trim and the stability of the means of transport, was completed.
    #[strum(serialize = "236")]
    _236,
    /// Completed tank cleaning date/time
    ///
    /// The date and/or time tank cleaning was completed.
    #[strum(serialize = "237")]
    _237,
    /// Tanks accepted date/time
    ///
    /// The date and/or time the tanks are to be or have been accepted.
    #[strum(serialize = "238")]
    _238,
    /// Tanks inspected date/time
    ///
    /// The date and/or time the tanks are to be or have been inspected.
    #[strum(serialize = "239")]
    _239,
    /// Reinsurance accounting period
    ///
    /// To identify a reinsurance account period via start and end dates.
    #[strum(serialize = "240")]
    _240,
    /// From date of award to earliest delivery
    ///
    /// Lead time to determine the earliest date a delivery can be made based on the date an award is made.
    #[strum(serialize = "241")]
    _241,
    /// Preparation date/time of document
    ///
    /// Date and/or time that the document was prepared.
    #[strum(serialize = "242")]
    _242,
    /// Transmission date/time of document
    ///
    ///
    #[strum(serialize = "243")]
    _243,
    /// Settlement date, planned
    ///
    ///
    #[strum(serialize = "244")]
    _244,
    /// Underwriting year
    ///
    /// Year in which the treaty was commenced.
    #[strum(serialize = "245")]
    _245,
    /// Accounting year
    ///
    /// Year considered for accounting of the treaty or portion of the treaty.
    #[strum(serialize = "246")]
    _246,
    /// Year of occurrence
    ///
    /// Year in which a specific event (e.g. a loss) took place.
    #[strum(serialize = "247")]
    _247,
    /// Loss
    ///
    /// Date, time, period on which a referenced loss occurred.
    #[strum(serialize = "248")]
    _248,
    /// Cash call date
    ///
    /// Date on which a cash call was made for a loss suffered and covered.
    #[strum(serialize = "249")]
    _249,
    /// Re-exportation date
    ///
    /// Date of re-exportation.
    #[strum(serialize = "250")]
    _250,
    /// Re-importation date
    ///
    /// Date of re-importation.
    #[strum(serialize = "251")]
    _251,
    /// Arrival date/time at initial port
    ///
    /// Date/time that the conveyance arrives at the initial port in the country of destination.
    #[strum(serialize = "252")]
    _252,
    /// Departure date/time from last port of call
    ///
    /// Date/time that conveyance departed from the last foreign port of call.
    #[strum(serialize = "253")]
    _253,
    /// Registration date of previous Customs declaration
    ///
    /// Registration date of the Customs declaration for the previous Customs procedure either in the same or another country.
    #[strum(serialize = "254")]
    _254,
    /// Availability due date
    ///
    /// Date when ordered items should be available at a specified location.
    #[strum(serialize = "255")]
    _255,
    /// From date of award to completion
    ///
    /// Lead time to determine the completion date of an effort based on the date an award is made.
    #[strum(serialize = "256")]
    _256,
    /// Calculation date/time/period
    ///
    /// The date/time/period on which a calculation will take, or has taken, place.
    #[strum(serialize = "257")]
    _257,
    /// Guarantee date
    ///
    /// Date when a guarantee is placed.
    #[strum(serialize = "258")]
    _258,
    /// Conveyance registration date
    ///
    /// Date when a vessel, vehicle or other means of transport was registered by a competent authority.
    #[strum(serialize = "259")]
    _259,
    /// Valuation date (Customs)
    ///
    /// Date when Customs valuation was made.
    #[strum(serialize = "260")]
    _260,
    /// Release date/time
    ///
    /// Date/time assigned to identify the release of a set of rules, conditions, conventions, productions, etc.
    #[strum(serialize = "261")]
    _261,
    /// Closure date/time/period
    ///
    /// Date/time/period when an enterprise is closed.
    #[strum(serialize = "262")]
    _262,
    /// Invoicing period
    ///
    /// Period for which an invoice is issued.
    #[strum(serialize = "263")]
    _263,
    /// Release frequency
    ///
    /// Frequency of a release.
    #[strum(serialize = "264")]
    _264,
    /// Due date
    ///
    ///
    #[strum(serialize = "265")]
    _265,
    /// Validation date
    ///
    ///
    #[strum(serialize = "266")]
    _266,
    /// Rate/price date/time
    ///
    /// Date/time on which a rate/price is determined.
    #[strum(serialize = "267")]
    _267,
    /// Transit time/limits
    ///
    /// Description to be provided.
    #[strum(serialize = "268")]
    _268,
    /// Ship during date
    ///
    /// The date identifying the period during or in which the goods should be shipped.
    #[strum(serialize = "270")]
    _270,
    /// Ship on or about date
    ///
    /// Date on or about which goods should be shipped.
    #[strum(serialize = "271")]
    _271,
    /// Documentary credit presentation period
    ///
    /// The specification of the period of time, expressed in number of days, after the date of issuance of the transport document(s) within which the documents must be presented.
    #[strum(serialize = "272")]
    _272,
    /// Validity period
    ///
    /// Dates (from/to)/period referenced documents are valid.
    #[strum(serialize = "273")]
    _273,
    /// From date of order receipt to sample ready
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "274")]
    _274,
    /// From date of tooling authorization to sample ready
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "275")]
    _275,
    /// From date of receipt of tooling aids to sample ready
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "276")]
    _276,
    /// From date of sample approval to first product shipment
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "277")]
    _277,
    /// From date of order receipt to shipment
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "278")]
    _278,
    /// From date of order receipt to delivery
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "279")]
    _279,
    /// From last booked order to delivery
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "280")]
    _280,
    /// Date of order lead time
    ///
    /// Lead time is referenced to the date of order.
    #[strum(serialize = "281")]
    _281,
    /// Confirmation date lead time
    ///
    /// Lead time is referenced to the date of confirmation.
    #[strum(serialize = "282")]
    _282,
    /// Arrival date/time of transport lead time
    ///
    /// Lead time is referenced to the date a transport will arrive or has arrived.
    #[strum(serialize = "283")]
    _283,
    /// Before inventory is replenished based on stock check lead time
    ///
    /// Lead time is the defined timespan.
    #[strum(serialize = "284")]
    _284,
    /// Invitation to tender date/time
    ///
    /// Date/time on which the invitation to tender has been made available to relevant parties.
    #[strum(serialize = "285")]
    _285,
    /// Tender submission date/time
    ///
    /// Date/time on which the tender was submitted.
    #[strum(serialize = "286")]
    _286,
    /// Contract award date/time
    ///
    /// Date/time on which the contract is awarded to a tenderer.
    #[strum(serialize = "287")]
    _287,
    /// Price base date/time
    ///
    /// Base date/time of prices.
    #[strum(serialize = "288")]
    _288,
    /// Interest rate validity period
    ///
    /// Validity period of the interest rate.
    #[strum(serialize = "289")]
    _289,
    /// Contractual start date/time
    ///
    /// Date/time on which activities stated in the contract must start.
    #[strum(serialize = "290")]
    _290,
    /// Start date/time, planned
    ///
    ///
    #[strum(serialize = "291")]
    _291,
    /// Works completion date/time, planned
    ///
    ///
    #[strum(serialize = "292")]
    _292,
    /// Works completion date/time, actual
    ///
    ///
    #[strum(serialize = "293")]
    _293,
    /// Hand over date/time, planned
    ///
    /// Date/time on which hand over (i.e. the transfer of responsibility for an object or activity such as documentation, system etc. from one party to another) is planned to take place.
    #[strum(serialize = "294")]
    _294,
    /// Hand over date/time, actual
    ///
    /// Date/time on which hand over (i.e. the transfer of responsibility for an object or activity such as documentation, system etc. from one party to another) actually takes place.
    #[strum(serialize = "295")]
    _295,
    /// Retention release date/time
    ///
    /// Date/time on which the retention is released.
    #[strum(serialize = "296")]
    _296,
    /// Retention release date/time, partial
    ///
    /// Date/time on which the retention is partially released.
    #[strum(serialize = "297")]
    _297,
    /// Escalation start date
    ///
    /// Value date of the indexes appearing as denominators in an escalation formula.
    #[strum(serialize = "298")]
    _298,
    /// Price adjustment start date
    ///
    /// Value date of the indexes appearing as denominators in a price adjustment formula.
    #[strum(serialize = "299")]
    _299,
    /// Price adjustment limit date
    ///
    /// Limit value date of indexes used as numerators in a price adjustment formula.
    #[strum(serialize = "300")]
    _300,
    /// Value date of index
    ///
    /// Date of validity of index values.
    #[strum(serialize = "301")]
    _301,
    /// Publication date
    ///
    ///
    #[strum(serialize = "302")]
    _302,
    /// Escalation date
    ///
    /// Value date of indexes appearing as numerators in an escalation formula.
    #[strum(serialize = "303")]
    _303,
    /// Price adjustment date
    ///
    /// Value date of indexes appearing as numerators in a price adjustment formula.
    #[strum(serialize = "304")]
    _304,
    /// Latest price adjustment date
    ///
    /// Date on which the latest price adjustment took place.
    #[strum(serialize = "305")]
    _305,
    /// Work period
    ///
    /// Period of execution of works.
    #[strum(serialize = "306")]
    _306,
    /// Payment instruction date/time
    ///
    /// Date/time on which a payment instruction was given.
    #[strum(serialize = "307")]
    _307,
    /// Payment valuation presentation date/time
    ///
    /// Date/time on which the payment valuation is presented.
    #[strum(serialize = "308")]
    _308,
    /// Blanks value date
    ///
    /// The date on which the funds are at the disposal of the receiving bank.
    #[strum(serialize = "309")]
    _309,
    /// Received date/time
    ///
    /// Date/time of receipt.
    #[strum(serialize = "310")]
    _310,
    /// On
    ///
    /// Fixed maturity day for deferred payment or time draft(s).
    #[strum(serialize = "311")]
    _311,
    /// Ship not before and not after date/time
    ///
    /// Shipment(s) of goods is/are to be made not before the first specified date/time and not after the second specified date/time.
    #[strum(serialize = "312")]
    _312,
    /// Order to proceed date
    ///
    /// Issue date of an instruction to start work.
    #[strum(serialize = "313")]
    _313,
    /// Planned duration of works
    ///
    ///
    #[strum(serialize = "314")]
    _314,
    /// Agreement to pay date
    ///
    /// Date on which the debtor agreed to pay.
    #[strum(serialize = "315")]
    _315,
    /// Valuation date/time
    ///
    /// Date/time of valuation.
    #[strum(serialize = "316")]
    _316,
    /// Reply date
    ///
    ///
    #[strum(serialize = "317")]
    _317,
    /// Request date
    ///
    /// The date on which something was asked for.
    #[strum(serialize = "318")]
    _318,
    /// Customer value date
    ///
    /// Date at which funds are taken into account for interest calculation (in debit or credit).
    #[strum(serialize = "319")]
    _319,
    /// Declaration reference period
    ///
    /// Reference period of a set of items reported on the same declaration.
    #[strum(serialize = "320")]
    _320,
    /// Promotion date/period
    ///
    /// Date/period relevant for specific promotion activities.
    #[strum(serialize = "321")]
    _321,
    /// Accounting period
    ///
    /// Self-explanatory.
    #[strum(serialize = "322")]
    _322,
    /// Horizon period
    ///
    /// Period forming a (planning) horizon.
    #[strum(serialize = "323")]
    _323,
    /// Processing date/period
    ///
    /// Date/period a specific process happened/will happen.
    #[strum(serialize = "324")]
    _324,
    /// Tax period
    ///
    /// Period a tax rate/tax amount etc. is applicable.
    #[strum(serialize = "325")]
    _325,
    /// Charge period
    ///
    /// Period a specified charge is valid for.
    #[strum(serialize = "326")]
    _326,
    /// Instalment payment due date
    ///
    /// Self-explanatory.
    #[strum(serialize = "327")]
    _327,
    /// Payroll deduction date/time
    ///
    /// Date/time of a monetary deduction made from the salary of a person on a payroll.
    #[strum(serialize = "328")]
    _328,
    /// Birth date/time
    ///
    /// Date/time when a person was born.
    #[strum(serialize = "329")]
    _329,
    /// Joined employer date
    ///
    /// Date when a person joins an employer.
    #[strum(serialize = "330")]
    _330,
    /// Contributions ceasing date/time
    ///
    /// Date/time when contributions cease.
    #[strum(serialize = "331")]
    _331,
    /// Contribution period end date/time
    ///
    /// Date/time when a contribution period ends.
    #[strum(serialize = "332")]
    _332,
    /// Part-time working change date/time
    ///
    /// Date/time when the proportion of part-time work changes.
    #[strum(serialize = "333")]
    _333,
    /// Status change date/time
    ///
    /// Date/time when a status changes.
    #[strum(serialize = "334")]
    _334,
    /// Contribution period start date/time
    ///
    /// Date/time when a contribution period commences.
    #[strum(serialize = "335")]
    _335,
    /// Salary change effective date
    ///
    /// Date when a change in salary becomes effective.
    #[strum(serialize = "336")]
    _336,
    /// Left employer date
    ///
    /// Date when a person leaves an employer.
    #[strum(serialize = "337")]
    _337,
    /// Benefit change date/time
    ///
    /// Date/time when a benefit provided by a service provider is changed.
    #[strum(serialize = "338")]
    _338,
    /// Category change date/time
    ///
    /// Date/time when a change of category is made.
    #[strum(serialize = "339")]
    _339,
    /// Joined fund date/time
    ///
    /// Date/time when a person joins a fund.
    #[strum(serialize = "340")]
    _340,
    /// Waiting time
    ///
    /// The period of time between the moment at which one wants an activity to begin and the moment at which this activity can actually begin.
    #[strum(serialize = "341")]
    _341,
    /// On-board date
    ///
    /// The date goods have been loaded on board of a conveyance.
    #[strum(serialize = "342")]
    _342,
    /// Date/time of discount termination
    ///
    /// Date/time when the deduction from an amount comes to an end.
    #[strum(serialize = "343")]
    _343,
    /// Date/time of interest due
    ///
    /// Date/time when the interest has to be paid.
    #[strum(serialize = "344")]
    _344,
    /// Days of operation
    ///
    /// Week days of operation.
    #[strum(serialize = "345")]
    _345,
    /// Latest check-in time
    ///
    /// Latest time of check-in.
    #[strum(serialize = "346")]
    _346,
    /// Slaughtering start date
    ///
    /// Date on which slaughtering commenced.
    #[strum(serialize = "347")]
    _347,
    /// Packing start date
    ///
    /// Date on which packing commenced.
    #[strum(serialize = "348")]
    _348,
    /// Packing end date
    ///
    /// Date on which packing completed.
    #[strum(serialize = "349")]
    _349,
    /// Test start date
    ///
    /// Date when a test has been started.
    #[strum(serialize = "350")]
    _350,
    /// Inspection date
    ///
    /// Date of inspection.
    #[strum(serialize = "351")]
    _351,
    /// Slaughtering end date
    ///
    /// Date on which slaughtering completed.
    #[strum(serialize = "352")]
    _352,
    /// Accounting transaction date
    ///
    /// Date to which an accounting transaction refers.
    #[strum(serialize = "353")]
    _353,
    /// Activity period date range
    ///
    /// A specific date range associated with an activity.
    #[strum(serialize = "354")]
    _354,
    /// Contractual delivery date
    ///
    /// The date of delivery contractually agreed between parties.
    #[strum(serialize = "355")]
    _355,
    /// Sales date, and or time, and or period
    ///
    /// The date, and or time, and or period on which a sale took place.
    #[strum(serialize = "356")]
    _356,
    /// Cancel if not published by this date
    ///
    /// Cancel if not published by this date.
    #[strum(serialize = "357")]
    _357,
    /// Scheduled for delivery on or after
    ///
    /// Scheduled for delivery on or after the specified date, and or time.
    #[strum(serialize = "358")]
    _358,
    /// Scheduled for delivery on or before
    ///
    /// Scheduled for delivery on or before specified date and or time.
    #[strum(serialize = "359")]
    _359,
    /// Sell by date
    ///
    /// The date by which a product should be sold.
    #[strum(serialize = "360")]
    _360,
    /// Best before date
    ///
    /// The best before date.
    #[strum(serialize = "361")]
    _361,
    /// End availability date
    ///
    /// The end date of availability.
    #[strum(serialize = "362")]
    _362,
    /// Total shelf life period
    ///
    /// A period indicating the total shelf life of a product.
    #[strum(serialize = "363")]
    _363,
    /// Minimum shelf life remaining at time of despatch period
    ///
    /// Period indicating the minimum shelf life remaining for a product at the time of leaving the supplier.
    #[strum(serialize = "364")]
    _364,
    /// Packaging date
    ///
    /// The date on which the packaging of a product took place.
    #[strum(serialize = "365")]
    _365,
    /// Inventory report date
    ///
    /// Date on which a inventory report is made.
    #[strum(serialize = "366")]
    _366,
    /// Previous meter reading date
    ///
    /// Date on which the previous reading of a meter took place.
    #[strum(serialize = "367")]
    _367,
    /// Latest meter reading date
    ///
    /// Date on which the latest reading of a meter took place.
    #[strum(serialize = "368")]
    _368,
    /// Date and or time of handling, estimated
    ///
    /// The date and or time when the handling action is estimated to take place.
    #[strum(serialize = "369")]
    _369,
    /// Date when container equipment becomes domestic
    ///
    /// The date on which foreign-built container equipment has entered into the commerce of another country and has become domestic equipment.
    #[strum(serialize = "370")]
    _370,
    /// Hydrotest date
    ///
    /// The date equipment has been hydrotested.
    #[strum(serialize = "371")]
    _371,
    /// Equipment pre-trip date
    ///
    /// The date on which equipment is pre-tripped.
    #[strum(serialize = "372")]
    _372,
    /// Mooring, date and time
    ///
    /// Date and time of mooring.
    #[strum(serialize = "373")]
    _373,
    /// Road fund tax expiry date
    ///
    /// The date of expiry of the road fund tax.
    #[strum(serialize = "374")]
    _374,
    /// Date of first registration
    ///
    /// Date of first registration.
    #[strum(serialize = "375")]
    _375,
    /// Biannual terminal inspection date
    ///
    /// The date on which a biannual inspection of a terminal has taken or will take place.
    #[strum(serialize = "376")]
    _376,
    /// Federal HighWay Administration (FHWA) inspection date
    ///
    /// The date on which container equipment is to be or has been inspected in accordance with the requirements of the U.S. Federal Highway Administration.
    #[strum(serialize = "377")]
    _377,
    /// Container Safety Convention (CSC) inspection date
    ///
    /// The date on which container equipment is to be or has been inspected as per the Container Safety Convention (CSC).
    #[strum(serialize = "378")]
    _378,
    /// Periodic inspection date
    ///
    /// The date on which a periodic inspection has to take place.
    #[strum(serialize = "379")]
    _379,
    /// Drawing revision date
    ///
    /// Date the drawing revision has been allocated to a design.
    #[strum(serialize = "380")]
    _380,
    /// Product lifespan at time of production
    ///
    /// The total lifespan of a product at the time of its production.
    #[strum(serialize = "381")]
    _381,
    /// Earliest sale date
    ///
    /// The earliest date on which the product may be made available for sale.
    #[strum(serialize = "382")]
    _382,
    /// Cancel if not shipped by this date
    ///
    /// Cancel the order if goods not shipped by this date.
    #[strum(serialize = "383")]
    _383,
    /// Previous invoice date
    ///
    /// Indicates the date which was allocated to a previous invoice.
    #[strum(serialize = "384")]
    _384,
    /// Repair turnaround time
    ///
    /// Provides the period of time necessary to turnaround a given repair.
    #[strum(serialize = "387")]
    _387,
    /// Order amendment binding date
    ///
    /// The date when an order amendment becomes binding for both parties.
    #[strum(serialize = "388")]
    _388,
    /// Cure time
    ///
    /// Specifies the length of time that an article was or should be cured.
    #[strum(serialize = "389")]
    _389,
    /// From date of award to delivery
    ///
    /// Lead time to determine the delivery date based on the date an award is made.
    #[strum(serialize = "390")]
    _390,
    /// From date of receipt of item to approval
    ///
    /// Lead time to determine the date an item will be approved based on the date the item was received.
    #[strum(serialize = "391")]
    _391,
    /// Equipment collection or pick-up date/time, earliest
    ///
    /// Date/time on which equipment can be picked up at the earliest.
    #[strum(serialize = "392")]
    _392,
    /// Equipment collection or pick-up date/time, planned
    ///
    /// Date/time on which equipment can be picked up, either full or empty, according to a planning.
    #[strum(serialize = "393")]
    _393,
    /// Equipment positioning date/time, actual
    ///
    /// Date/time on which equipment was actually positioned (delivered).
    #[strum(serialize = "394")]
    _394,
    /// Equipment positioning date/time, estimated
    ///
    /// Date/time on which equipment is estimated to be positioned (delivered).
    #[strum(serialize = "395")]
    _395,
    /// Equipment positioning date/time, requested
    ///
    /// Date/time on which equipment is requested to be positioned (delivered).
    #[strum(serialize = "396")]
    _396,
    /// Equipment positioning date/time, ultimate
    ///
    /// Date/time on which equipment should be positioned (delivered) at the latest.
    #[strum(serialize = "397")]
    _397,
    /// Goods collection or pick-up date/time, planned
    ///
    /// Date/time at which goods can be picked up, according to a planning.
    #[strum(serialize = "398")]
    _398,
    /// Goods positioning date/time, expected
    ///
    /// Date/time on which goods are expected to be positioned.
    #[strum(serialize = "399")]
    _399,
    /// Cargo release date/time, ultimate
    ///
    /// Ultimate date/time at which goods or equipment should be released.
    #[strum(serialize = "400")]
    _400,
    /// Container Safety Convention (CSC) plate expiration date
    ///
    /// Date on which the validity of a Container Safety Convention (CSC) plate expires.
    #[strum(serialize = "401")]
    _401,
    /// Document received date/time
    ///
    /// Date/time on which the document was actually received.
    #[strum(serialize = "402")]
    _402,
    /// Discharge date/time, actual
    ///
    /// Date/time when the specified goods or transport equipment has or have been discharged from the means of transport.
    #[strum(serialize = "403")]
    _403,
    /// Loading date/time, actual
    ///
    /// Date/time when the specified goods or transport equipment has or have been loaded in or on the means of transport.
    #[strum(serialize = "404")]
    _404,
    /// Equipment collection or pick-up date/time, actual
    ///
    /// Date/time on which the equipment was actually collected.
    #[strum(serialize = "405")]
    _405,
    /// Goods positioning date/time, planned
    ///
    /// The date/time on which the goods will be positioned according to a planning.
    #[strum(serialize = "406")]
    _406,
    /// Document requested date/time
    ///
    /// Date/time on which the document is requested by a party.
    #[strum(serialize = "407")]
    _407,
    /// Expected container hire from date/time
    ///
    /// Estimated date and time when the containers are expected to go on-hire.
    #[strum(serialize = "408")]
    _408,
    /// Order completion date/time, ultimate
    ///
    /// Date/time on which the order should be completed at the latest.
    #[strum(serialize = "409")]
    _409,
    /// Equipment repair ready date/time, ultimate
    ///
    /// Ultimate date/time on which a piece of equipment must be repaired.
    #[strum(serialize = "410")]
    _410,
    /// Container stuffing date/time, ultimate
    ///
    /// Date/time on which the container stuffing should be completed at the latest.
    #[strum(serialize = "411")]
    _411,
    /// Container stripping date/time, ultimate
    ///
    /// Date/time on which the container stripping should be completed at the latest.
    #[strum(serialize = "412")]
    _412,
    /// Discharge and loading completed date/time
    ///
    /// Date/time when all discharge and loading operations on the transport means have been completed.
    #[strum(serialize = "413")]
    _413,
    /// Equipment stock check date/time
    ///
    /// Date/time on which equipment has been ascertained as being in stock.
    #[strum(serialize = "414")]
    _414,
    /// Activity reporting date
    ///
    /// The date applicable to the activity being reported.
    #[strum(serialize = "415")]
    _415,
    /// Submission date
    ///
    /// The date of a submission.
    #[strum(serialize = "416")]
    _416,
    /// Previous booking date/time
    ///
    /// Date/time at which the previous booking was made.
    #[strum(serialize = "417")]
    _417,
    /// Minimum shelf life remaining at time of receipt
    ///
    /// The minimum shelf life remaining at the time of receipt.
    #[strum(serialize = "418")]
    _418,
    /// Forecast period
    ///
    /// A period for which a forecast applies.
    #[strum(serialize = "419")]
    _419,
    /// Unloaded, date and time
    ///
    /// To report the date and time that an unloading action occurred.
    #[strum(serialize = "420")]
    _420,
    /// Estimated acceptance date
    ///
    /// To estimate the date of acceptance.
    #[strum(serialize = "421")]
    _421,
    /// Documentary credit issue date
    ///
    /// The date the documentary credit has been issued.
    #[strum(serialize = "422")]
    _422,
    /// First date of ordering
    ///
    /// The first date on which ordering may take place.
    #[strum(serialize = "423")]
    _423,
    /// Last date of ordering
    ///
    /// The last date on which ordering may take place.
    #[strum(serialize = "424")]
    _424,
    /// Original posting date
    ///
    /// Date when the entry was originally posted.
    #[strum(serialize = "425")]
    _425,
    /// Reinsurance payment frequency
    ///
    /// The frequency of payments of reinsurance premiums.
    #[strum(serialize = "426")]
    _426,
    /// Adjusted age
    ///
    /// The adjusted age used for purposes of calculation.
    #[strum(serialize = "427")]
    _427,
    /// Original issue age
    ///
    /// The original issue age.
    #[strum(serialize = "428")]
    _428,
    /// Coverage duration
    ///
    /// The period coverage has been in force.
    #[strum(serialize = "429")]
    _429,
    /// Coverage issue date
    ///
    /// Date from which the anniversary coverage is measured.
    #[strum(serialize = "430")]
    _430,
    /// Flat extra period
    ///
    /// Period for charging the additional extra.
    #[strum(serialize = "431")]
    _431,
    /// Paid to date
    ///
    /// Date to which payments have been paid.
    #[strum(serialize = "432")]
    _432,
    /// Reinsurance coverage duration
    ///
    /// The period for which reinsurance coverage has been in force.
    #[strum(serialize = "433")]
    _433,
    /// Maturity date
    ///
    /// Date at which maturity occurs.
    #[strum(serialize = "434")]
    _434,
    /// Reinsurance issue age
    ///
    /// The actual or equivalent age at time of issue.
    #[strum(serialize = "435")]
    _435,
    /// Reinsurance paid-up date
    ///
    /// The date up to which the reinsurance has been paid.
    #[strum(serialize = "436")]
    _436,
    /// Benefit period
    ///
    /// The period of time for which benefits are provided.
    #[strum(serialize = "437")]
    _437,
    /// Disability wait period
    ///
    /// The period of time the insured must be disabled before reinsurance coverage becomes effective.
    #[strum(serialize = "438")]
    _438,
    /// Deferred Period
    ///
    /// The period of time for which an activity has been postponed.
    #[strum(serialize = "439")]
    _439,
    /// Documentary credit amendment date
    ///
    /// Date of amendment of a documentary credit.
    #[strum(serialize = "440")]
    _440,
    /// Last on hire date
    ///
    /// Date the item was last placed on hire.
    #[strum(serialize = "441")]
    _441,
    /// Last off hire date
    ///
    /// Date the item was last returned from hire.
    #[strum(serialize = "442")]
    _442,
    /// Direct interchange date
    ///
    /// Date the item was directly interchanged.
    #[strum(serialize = "443")]
    _443,
    /// Approval date
    ///
    /// Date of approval.
    #[strum(serialize = "444")]
    _444,
    /// Original estimate date
    ///
    /// The date of the original estimate.
    #[strum(serialize = "445")]
    _445,
    /// Revised estimate date
    ///
    /// The date the estimate was revised.
    #[strum(serialize = "446")]
    _446,
    /// Creditor's requested value date
    ///
    /// Date on which the creditor requests to be credited.
    #[strum(serialize = "447")]
    _447,
    /// Referenced item creation date
    ///
    /// Creation date of referenced item.
    #[strum(serialize = "448")]
    _448,
    /// Date for the last update
    ///
    /// Date for the last update.
    #[strum(serialize = "449")]
    _449,
    /// Opening date
    ///
    /// Date of opening.
    #[strum(serialize = "450")]
    _450,
    /// Source document capture date
    ///
    /// Date source document data is entered into a business application.
    #[strum(serialize = "451")]
    _451,
    /// Trial balance period
    ///
    /// Period covered by the trial balance.
    #[strum(serialize = "452")]
    _452,
    /// Date of source document
    ///
    /// The date of the source document.
    #[strum(serialize = "453")]
    _453,
    /// Accounting value date
    ///
    /// Date against which the entry has to be legally allocated.
    #[strum(serialize = "454")]
    _454,
    /// Expected value date
    ///
    /// Date on which the funds are expected to be at the disposal of the beneficiary.
    #[strum(serialize = "455")]
    _455,
    /// Chart of account period
    ///
    /// Period covered by the chart of account.
    #[strum(serialize = "456")]
    _456,
    /// Date of separation
    ///
    /// Date of marital separation.
    #[strum(serialize = "457")]
    _457,
    /// Date of divorce
    ///
    /// Date when two married persons are officially divorced.
    #[strum(serialize = "458")]
    _458,
    /// Date of marriage
    ///
    /// Date when two persons are married.
    #[strum(serialize = "459")]
    _459,
    /// Wage period, start date
    ///
    /// Date when a period of wage begins.
    #[strum(serialize = "460")]
    _460,
    /// Wage period, end date
    ///
    /// Date when a period of wage ends.
    #[strum(serialize = "461")]
    _461,
    /// Working period, start date
    ///
    /// Date when a period of work begins.
    #[strum(serialize = "462")]
    _462,
    /// Working period, end date
    ///
    /// Date when a period of work ends.
    #[strum(serialize = "463")]
    _463,
    /// Embarkation date and time
    ///
    /// Date and time at which crew and/or passengers board.
    #[strum(serialize = "464")]
    _464,
    /// Disembarkation date and time
    ///
    /// Date and time at which crew and/or passengers disembark.
    #[strum(serialize = "465")]
    _465,
    /// Time now date
    ///
    /// A time now date used for planning and scheduling purposes.
    #[strum(serialize = "466")]
    _466,
    /// Holiday
    ///
    /// A date or period that is a break from work.
    #[strum(serialize = "467")]
    _467,
    /// Non working
    ///
    /// To specify a non working date or period.
    #[strum(serialize = "468")]
    _468,
    /// Start date or time, earliest
    ///
    /// The earliest date or time for starting.
    #[strum(serialize = "469")]
    _469,
    /// Start date or time, latest
    ///
    /// The latest date or time for starting.
    #[strum(serialize = "470")]
    _470,
    /// Finish date or time, earliest
    ///
    /// The earliest date or time for finishing.
    #[strum(serialize = "471")]
    _471,
    /// Finish date or time, latest
    ///
    /// The latest date or time for finishing.
    #[strum(serialize = "472")]
    _472,
    /// Start date or time, mandatory
    ///
    /// The mandatory date or time for starting.
    #[strum(serialize = "473")]
    _473,
    /// Finish date or time, mandatory
    ///
    /// The mandatory date or time for finishing.
    #[strum(serialize = "474")]
    _474,
    /// Start date or time, actual
    ///
    /// The actual date or time for starting.
    #[strum(serialize = "475")]
    _475,
    /// Start date or time, estimated
    ///
    /// The estimated date or time for starting.
    #[strum(serialize = "476")]
    _476,
    /// Completion date or time, estimated
    ///
    /// The estimated date or time for completion.
    #[strum(serialize = "477")]
    _477,
    /// Start date or time, scheduled
    ///
    /// The scheduled date or time for starting.
    #[strum(serialize = "478")]
    _478,
    /// Completion date or time, scheduled
    ///
    /// The scheduled date or time for completion.
    #[strum(serialize = "479")]
    _479,
    /// Start date or time, not before
    ///
    /// The not before date or time for starting.
    #[strum(serialize = "480")]
    _480,
    /// Start date or time, not after
    ///
    /// The not after date or time for starting.
    #[strum(serialize = "481")]
    _481,
    /// Completion date or time, not before
    ///
    /// The not before date or time for completion.
    #[strum(serialize = "482")]
    _482,
    /// Completion date or time, not after
    ///
    /// The not after date or time for completion.
    #[strum(serialize = "483")]
    _483,
    /// Illness recovery date, expected
    ///
    /// Date when a person is expected to recover from illness.
    #[strum(serialize = "484")]
    _484,
    /// Period of illness, start date
    ///
    /// Date when a period of illness began.
    #[strum(serialize = "485")]
    _485,
    /// Period of illness, end date
    ///
    /// Date when a period of illness ends.
    #[strum(serialize = "486")]
    _486,
    /// Decease date
    ///
    /// Date when a person died.
    #[strum(serialize = "487")]
    _487,
    /// Benefit period, start date
    ///
    /// Date when a period of benefit begins.
    #[strum(serialize = "488")]
    _488,
    /// Benefit period, end date
    ///
    /// Date when a period of benefit ends.
    #[strum(serialize = "489")]
    _489,
    /// Selection period, start date
    ///
    /// Date when a period of selection begins.
    #[strum(serialize = "490")]
    _490,
    /// Selection period, end date
    ///
    /// Date when a period of selection ends.
    #[strum(serialize = "491")]
    _491,
    /// Balance date/time/period
    ///
    /// The date/time/period of a balance.
    #[strum(serialize = "492")]
    _492,
    /// Benefit payments termination date
    ///
    /// To identify the date on which benefit payments have ceased.
    #[strum(serialize = "493")]
    _493,
    /// Covered income period
    ///
    /// To identify the period over which covered income is measured.
    #[strum(serialize = "494")]
    _494,
    /// Current income period
    ///
    /// To identify the period over which current income is measured.
    #[strum(serialize = "495")]
    _495,
    /// Reinstatement date
    ///
    /// Identifies the date of reinstatement.
    #[strum(serialize = "496")]
    _496,
    /// Definition of disability duration
    ///
    /// To identify the period for which the definition of disability applies.
    #[strum(serialize = "497")]
    _497,
    /// Previous termination date
    ///
    /// Identifies the date of the previous termination.
    #[strum(serialize = "498")]
    _498,
    /// Premium change period
    ///
    /// To identify the period of the premium change.
    #[strum(serialize = "499")]
    _499,
    /// Off-hire survey date
    ///
    /// Date on which the equipment was surveyed at the end of the current leasing period.
    #[strum(serialize = "500")]
    _500,
    /// In service survey date
    ///
    /// Date of survey of equipment while in use.
    #[strum(serialize = "501")]
    _501,
    /// On hire survey date
    ///
    /// Date on which the equipment was surveyed at the beginning of the current leasing period.
    #[strum(serialize = "502")]
    _502,
    /// Production inspection date
    ///
    /// Date of production inspection.
    #[strum(serialize = "503")]
    _503,
    /// Overtime, start date
    ///
    /// Date when a period of overtime begins.
    #[strum(serialize = "504")]
    _504,
    /// Overtime, end date
    ///
    /// Date when a period of overtime ends.
    #[strum(serialize = "505")]
    _505,
    /// Back order delivery date/time/period
    ///
    /// The date/time/period during which the delivery of a back order will take, or has taken, place.
    #[strum(serialize = "506")]
    _506,
    /// Negotiations start date
    ///
    /// The date on which negotiations started.
    #[strum(serialize = "507")]
    _507,
    /// Work effective start date
    ///
    /// The date on which work will effectively start.
    #[strum(serialize = "508")]
    _508,
    /// Contract binding date
    ///
    /// The date from which a contract becomes binding on the contracting parties.
    #[strum(serialize = "509")]
    _509,
    /// Notification time limit
    ///
    /// The time limit which has been set for a notification to take place.
    #[strum(serialize = "510")]
    _510,
    /// Time limit
    ///
    /// The time limit in which an event must take place.
    #[strum(serialize = "511")]
    _511,
    /// Attendance date and or time and or period
    ///
    /// Date and or time and or period of attendance.
    #[strum(serialize = "512")]
    _512,
    /// Accident date and or time
    ///
    /// Date and or time when an accident occurred.
    #[strum(serialize = "513")]
    _513,
    /// Adoption date, actual
    ///
    /// Actual date when adoption occurs.
    #[strum(serialize = "514")]
    _514,
    /// Reimbursement claim issue date and or time
    ///
    /// Date and or time when a reimbursement claim is issued.
    #[strum(serialize = "515")]
    _515,
    /// Hospital admission date and or time
    ///
    /// Date and or time of admission to a hospital.
    #[strum(serialize = "516")]
    _516,
    /// Hospital discharge date and or time
    ///
    /// Date and or time of discharge from a hospital.
    #[strum(serialize = "517")]
    _517,
    /// Period of care start date and or time
    ///
    /// Date and or time when a period of care starts.
    #[strum(serialize = "518")]
    _518,
    /// Period of care end date and or time
    ///
    /// Date and or time when a period of care ends.
    #[strum(serialize = "519")]
    _519,
    /// Department admission date and or time
    ///
    /// Date and or time of admission to a department.
    #[strum(serialize = "520")]
    _520,
    /// Department discharge date and or time
    ///
    /// Date and or time of discharge from a department.
    #[strum(serialize = "521")]
    _521,
    /// Childbirth date and or time, actual
    ///
    /// Actual date and or time of childbirth.
    #[strum(serialize = "522")]
    _522,
    /// Prescription issue date and or time
    ///
    /// Date and or time when a prescription was issued.
    #[strum(serialize = "523")]
    _523,
    /// Prescription dispensing date and or time
    ///
    /// Date and or time when a prescription was dispensed.
    #[strum(serialize = "524")]
    _524,
    /// Clinical examination date and or time
    ///
    /// Date and or time of clinical examination.
    #[strum(serialize = "525")]
    _525,
    /// Death date and or time
    ///
    /// Date and or time of death.
    #[strum(serialize = "526")]
    _526,
    /// Childbirth date, estimated
    ///
    /// Estimated date of childbirth.
    #[strum(serialize = "527")]
    _527,
    /// Last menstrual cycle, start date
    ///
    /// Date when the last menstrual cycle started.
    #[strum(serialize = "528")]
    _528,
    /// Pregnancy duration, actual
    ///
    /// Actual duration of pregnancy.
    #[strum(serialize = "529")]
    _529,
    /// Fumigation date and/or time
    ///
    /// The date/or time on which fumigation is to occur or has taken place.
    #[strum(serialize = "530")]
    _530,
    /// Payment period
    ///
    /// A period of time in which a payment has been or will be made.
    #[strum(serialize = "531")]
    _531,
    /// Average delivery delay
    ///
    /// The average delay between deliveries.
    #[strum(serialize = "532")]
    _532,
    /// Budget line application date
    ///
    /// The date on which something has been applied to a budget line.
    #[strum(serialize = "533")]
    _533,
    /// Date of repair or service
    ///
    /// The date of a repair or service.
    #[strum(serialize = "534")]
    _534,
    /// Date of product failure
    ///
    /// The date the product failed.
    #[strum(serialize = "535")]
    _535,
    /// Review date
    ///
    /// Date the item was or will be reviewed.
    #[strum(serialize = "536")]
    _536,
    /// International review cycle start date
    ///
    /// Date the international review cycle starts.
    #[strum(serialize = "537")]
    _537,
    /// International assessment approval for publication date
    ///
    /// Date the Data Maintenance Request (DMR) was approved for publication after completing international review.
    #[strum(serialize = "538")]
    _538,
    /// Status assignment date
    ///
    /// Date a status was assigned.
    #[strum(serialize = "539")]
    _539,
    /// Instruction's original execution date
    ///
    /// Original execution date for the instruction.
    #[strum(serialize = "540")]
    _540,
    /// First published date
    ///
    /// Date when material was first published.
    #[strum(serialize = "541")]
    _541,
    /// Last published date
    ///
    /// Date when material was last published.
    #[strum(serialize = "542")]
    _542,
    /// Balance sheet date, latest
    ///
    /// Date of the latest balance sheet.
    #[strum(serialize = "543")]
    _543,
    /// Security share price as of given date
    ///
    /// Date of the security share price.
    #[strum(serialize = "544")]
    _544,
    /// Assigned date
    ///
    /// Date when assigned.
    #[strum(serialize = "545")]
    _545,
    /// Business opened date
    ///
    /// Date opened for business.
    #[strum(serialize = "546")]
    _546,
    /// Initial financial accounts filed date
    ///
    /// Date when the initial financial accounts were filed.
    #[strum(serialize = "547")]
    _547,
    /// Stop work as of given date
    ///
    /// Date work stopped or will stop.
    #[strum(serialize = "548")]
    _548,
    /// Completion date
    ///
    /// Date of completion.
    #[strum(serialize = "549")]
    _549,
    /// Lease term, start date
    ///
    /// Start date of the lease term.
    #[strum(serialize = "550")]
    _550,
    /// Lease term, end date
    ///
    /// End date of the lease term.
    #[strum(serialize = "551")]
    _551,
    /// Start date, actual
    ///
    /// Actual date of start.
    #[strum(serialize = "552")]
    _552,
    /// Start date, estimated
    ///
    /// Date of estimated start.
    #[strum(serialize = "553")]
    _553,
    /// Filed date
    ///
    /// Date when filed.
    #[strum(serialize = "554")]
    _554,
    /// Return to work date
    ///
    /// Date of return to work.
    #[strum(serialize = "555")]
    _555,
    /// Purchased date
    ///
    /// Date of purchase.
    #[strum(serialize = "556")]
    _556,
    /// Returned date
    ///
    /// Date return takes place.
    #[strum(serialize = "557")]
    _557,
    /// Changed date
    ///
    /// Date change takes place.
    #[strum(serialize = "558")]
    _558,
    /// Terminated date
    ///
    /// Date termination takes place.
    #[strum(serialize = "559")]
    _559,
    /// Evaluation date
    ///
    /// Date evaluation takes place.
    #[strum(serialize = "560")]
    _560,
    /// Business termination date
    ///
    /// Date the business terminates.
    #[strum(serialize = "561")]
    _561,
    /// Release from bankruptcy date
    ///
    /// Date when an entity is released from bankruptcy status.
    #[strum(serialize = "562")]
    _562,
    /// Placement date, initial
    ///
    /// Date of initial placement.
    #[strum(serialize = "563")]
    _563,
    /// Signature date
    ///
    /// Date of signature.
    #[strum(serialize = "564")]
    _564,
    /// Bankruptcy filed date
    ///
    /// Date when bankruptcy was filed.
    #[strum(serialize = "565")]
    _565,
    /// End date, scheduled
    ///
    /// Date when activity is scheduled to end.
    #[strum(serialize = "566")]
    _566,
    /// Report period
    ///
    /// Period covered by the report.
    #[strum(serialize = "567")]
    _567,
    /// Suspended date
    ///
    /// Date of suspension.
    #[strum(serialize = "568")]
    _568,
    /// Renewal date
    ///
    /// Date of renewal.
    #[strum(serialize = "569")]
    _569,
    /// Reported date
    ///
    /// Date when reported.
    #[strum(serialize = "570")]
    _570,
    /// Checked date
    ///
    /// Date when checked.
    #[strum(serialize = "571")]
    _571,
    /// Present residence, start date
    ///
    /// The beginning date of residence at present location.
    #[strum(serialize = "572")]
    _572,
    /// Employment position, start date
    ///
    /// The start date of employment in a particular position.
    #[strum(serialize = "573")]
    _573,
    /// Account closed date
    ///
    /// Date when account was closed.
    #[strum(serialize = "574")]
    _574,
    /// Construction date, actual
    ///
    /// Date of actual construction.
    #[strum(serialize = "575")]
    _575,
    /// Employment profession start date
    ///
    /// Start date of employment in a particular profession.
    #[strum(serialize = "576")]
    _576,
    /// Next review date
    ///
    /// Date of next review.
    #[strum(serialize = "577")]
    _577,
    /// Meeting date
    ///
    /// Date of the meeting.
    #[strum(serialize = "578")]
    _578,
    /// Administrator ordered date
    ///
    /// Date when an administrator is ordered for a company.
    #[strum(serialize = "579")]
    _579,
    /// Last date to file a claim
    ///
    /// Date after which no claim can be filed.
    #[strum(serialize = "580")]
    _580,
    /// Convicted date
    ///
    /// Date when convicted.
    #[strum(serialize = "581")]
    _581,
    /// Interviewed date
    ///
    /// Date of an interview.
    #[strum(serialize = "582")]
    _582,
    /// Last visit date
    ///
    /// Date of last visit.
    #[strum(serialize = "583")]
    _583,
    /// Future period
    ///
    /// Period in the future.
    #[strum(serialize = "584")]
    _584,
    /// Preceding period
    ///
    /// Period preceding current period.
    #[strum(serialize = "585")]
    _585,
    /// Expected problem resolution date
    ///
    /// Date when problem is expected to be resolved.
    #[strum(serialize = "586")]
    _586,
    /// Action date
    ///
    /// Date of action.
    #[strum(serialize = "587")]
    _587,
    /// Accountant's opinion date
    ///
    /// Date of an accountant's opinion.
    #[strum(serialize = "588")]
    _588,
    /// Last activity date
    ///
    /// Date of last activity.
    #[strum(serialize = "589")]
    _589,
    /// Resolved date
    ///
    /// Date when resolved.
    #[strum(serialize = "590")]
    _590,
    /// Recorded date
    ///
    /// Date when recorded.
    #[strum(serialize = "591")]
    _591,
    /// Date of birth, estimated
    ///
    /// The estimated date of birth.
    #[strum(serialize = "592")]
    _592,
    /// Last annual report date
    ///
    /// Date of the last annual report.
    #[strum(serialize = "593")]
    _593,
    /// Net worth date
    ///
    /// Date of net worth.
    #[strum(serialize = "594")]
    _594,
    /// Profit period
    ///
    /// Period over which profit was earned.
    #[strum(serialize = "596")]
    _596,
    /// Registration date
    ///
    /// Date when registered.
    #[strum(serialize = "597")]
    _597,
    /// Consolidation date
    ///
    /// Date when consolidation occurred.
    #[strum(serialize = "598")]
    _598,
    /// Board of directors not authorised as of given date
    ///
    /// As of this date the board of directors is not authorised.
    #[strum(serialize = "599")]
    _599,
    /// Board of directors not complete as of given date
    ///
    /// As of this date the board of directors is not fully filled.
    #[strum(serialize = "600")]
    _600,
    /// Manager not registered as of given date
    ///
    /// As of this date the manager is not registered.
    #[strum(serialize = "601")]
    _601,
    /// Citizenship change date
    ///
    /// Date of citizenship change.
    #[strum(serialize = "602")]
    _602,
    /// Participation date
    ///
    /// Date of participation.
    #[strum(serialize = "603")]
    _603,
    /// Capitalisation date
    ///
    /// Date of capitalisation.
    #[strum(serialize = "604")]
    _604,
    /// Board of directors registration date
    ///
    /// Date when the board of directors was registered.
    #[strum(serialize = "605")]
    _605,
    /// Operations ceased date
    ///
    /// Date when operations ceased.
    #[strum(serialize = "606")]
    _606,
    /// Satisfaction date
    ///
    /// Date when satisfaction was obtained.
    #[strum(serialize = "607")]
    _607,
    /// Legal settlement terms met date
    ///
    /// Date when terms specified in the legal settlement were met.
    #[strum(serialize = "608")]
    _608,
    /// Business control change date
    ///
    /// Date when a new authority took control.
    #[strum(serialize = "609")]
    _609,
    /// Court registration date
    ///
    /// Date of registration in the court.
    #[strum(serialize = "610")]
    _610,
    /// Annual report due date
    ///
    /// Date when annual report is due.
    #[strum(serialize = "611")]
    _611,
    /// Asset and liability schedule date
    ///
    /// Date of the asset and liability schedule.
    #[strum(serialize = "612")]
    _612,
    /// Annual report mailing date
    ///
    /// Date when the annual report was mailed.
    #[strum(serialize = "613")]
    _613,
    /// Annual report filing date
    ///
    /// Date when the annual report was filed.
    #[strum(serialize = "614")]
    _614,
    /// Annual report delinquent on date
    ///
    /// Date when annual report was considered delinquent.
    #[strum(serialize = "615")]
    _615,
    /// Accounting methodology change date
    ///
    /// Date when accounting methodology was changed.
    #[strum(serialize = "616")]
    _616,
    /// Closed until date
    ///
    /// Date when again open.
    #[strum(serialize = "617")]
    _617,
    /// Conversion into holding company date
    ///
    /// Date business was converted into a holding company.
    #[strum(serialize = "618")]
    _618,
    /// Deed not available as of given date
    ///
    /// Date when deed was not available.
    #[strum(serialize = "619")]
    _619,
    /// Detrimental information receipt date
    ///
    /// Date when detrimental information was received.
    #[strum(serialize = "620")]
    _620,
    /// Construction date, estimated
    ///
    /// Estimated date of construction.
    #[strum(serialize = "621")]
    _621,
    /// Financial information date
    ///
    /// Date of the financial information.
    #[strum(serialize = "622")]
    _622,
    /// Graduation date
    ///
    /// Date when graduation occurs.
    #[strum(serialize = "623")]
    _623,
    /// Insolvency discharge granted date
    ///
    /// Date when insolvency discharge was granted.
    #[strum(serialize = "624")]
    _624,
    /// Incorporation date
    ///
    /// Date of incorporation.
    #[strum(serialize = "625")]
    _625,
    /// Inactivity end date
    ///
    /// Date when inactivity ends.
    #[strum(serialize = "626")]
    _626,
    /// Last check for balance sheet update date
    ///
    /// Date balance sheet was last checked to determine if update had taken place.
    #[strum(serialize = "627")]
    _627,
    /// Last capital change date
    ///
    /// Date of last capital change.
    #[strum(serialize = "628")]
    _628,
    /// Letter of agreement date
    ///
    /// Date of a letter of agreement.
    #[strum(serialize = "629")]
    _629,
    /// Letter of liability date
    ///
    /// Date of a letter of liability.
    #[strum(serialize = "630")]
    _630,
    /// Liquidation date
    ///
    /// Date of liquidation.
    #[strum(serialize = "631")]
    _631,
    /// Lowest activity period
    ///
    /// Period of lowest activity.
    #[strum(serialize = "632")]
    _632,
    /// Legal structure change date
    ///
    /// Date when legal structure was changed.
    #[strum(serialize = "633")]
    _633,
    /// Current name effective date
    ///
    /// Date when current name became effective.
    #[strum(serialize = "634")]
    _634,
    /// Not registered as of given date
    ///
    /// Date when not yet registered.
    #[strum(serialize = "635")]
    _635,
    /// Current authority control start date
    ///
    /// Date when current authority took control.
    #[strum(serialize = "636")]
    _636,
    /// Privilege details verification date
    ///
    /// Date when privilege details were verified.
    #[strum(serialize = "637")]
    _637,
    /// Current legal structure effective date
    ///
    /// Date when current legal structure became effective.
    #[strum(serialize = "638")]
    _638,
    /// Peak activity period
    ///
    /// Period of peak activity.
    #[strum(serialize = "639")]
    _639,
    /// Presentation to bankruptcy receivers date
    ///
    /// Date when presented to the bankruptcy receivers.
    #[strum(serialize = "640")]
    _640,
    /// Resignation date
    ///
    /// Date of resignation.
    #[strum(serialize = "641")]
    _641,
    /// Legal action closed date
    ///
    /// Date when the legal action was closed.
    #[strum(serialize = "642")]
    _642,
    /// Mail receipt date
    ///
    /// Date mail was received.
    #[strum(serialize = "643")]
    _643,
    /// Social security claims verification date
    ///
    /// Date when social security claims were verified.
    #[strum(serialize = "644")]
    _644,
    /// Sole directorship registration date
    ///
    /// Date when sole directorship was registered.
    #[strum(serialize = "645")]
    _645,
    /// Trade style registration date
    ///
    /// Date when trade style was registered.
    #[strum(serialize = "646")]
    _646,
    /// Trial start date, scheduled
    ///
    /// Date when a trial is scheduled to begin.
    #[strum(serialize = "647")]
    _647,
    /// Trial start date, actual
    ///
    /// Date when the trial actually started.
    #[strum(serialize = "648")]
    _648,
    /// Value Added Tax (VAT) claims verification date
    ///
    /// Date when the Value Added Tax (VAT) claims were verified.
    #[strum(serialize = "649")]
    _649,
    /// Receivership result date
    ///
    /// Date when the result of the receivership occurs.
    #[strum(serialize = "650")]
    _650,
    /// Investigation end date
    ///
    /// The date when an investigation ended.
    #[strum(serialize = "651")]
    _651,
    /// Employee temporary laid-off period end date
    ///
    /// The ending date of a period in which employees were temporarily placed out of work.
    #[strum(serialize = "652")]
    _652,
    /// Investigation start date
    ///
    /// The date when an investigation began.
    #[strum(serialize = "653")]
    _653,
    /// Income period
    ///
    /// The period of time in which income is earned.
    #[strum(serialize = "654")]
    _654,
    /// Criminal sentence duration
    ///
    /// The period of time over which a criminal sentence applies.
    #[strum(serialize = "655")]
    _655,
    /// Age
    ///
    /// The length of time that a person or thing has existed.
    #[strum(serialize = "656")]
    _656,
    /// Receivables collection period
    ///
    /// The period of time over which receivable accounts are collected.
    #[strum(serialize = "657")]
    _657,
    /// Comparison period
    ///
    /// The time period covered in a comparison.
    #[strum(serialize = "658")]
    _658,
    /// Adjournment
    ///
    /// The period of time over which an adjournment is in effect.
    #[strum(serialize = "659")]
    _659,
    /// Court dismissal date
    ///
    /// The date on which a court refused further hearing of a case.
    #[strum(serialize = "660")]
    _660,
    /// Insufficient assets judgement date
    ///
    /// The date on which assets were judged to be insufficient.
    #[strum(serialize = "661")]
    _661,
    /// Average payment period
    ///
    /// The average period of time over which money has been paid.
    #[strum(serialize = "662")]
    _662,
    /// Forecast period start
    ///
    /// The beginning of a forecast period.
    #[strum(serialize = "663")]
    _663,
    /// Period extended
    ///
    /// Number of time units added to the original end date/time/period.
    #[strum(serialize = "664")]
    _664,
    /// Employee temporary laid-off period start date
    ///
    /// The start date of a period in which employees were temporarily placed out of work.
    #[strum(serialize = "665")]
    _665,
    /// Management available date
    ///
    /// Date when management is available.
    #[strum(serialize = "666")]
    _666,
    /// Withdrawn date
    ///
    /// The date when something was retracted.
    #[strum(serialize = "667")]
    _667,
    /// Claim incurred date
    ///
    /// The date that the claim was incurred.
    #[strum(serialize = "668")]
    _668,
    /// Financial coverage period
    ///
    /// The period of time for which financial coverage applies.
    #[strum(serialize = "669")]
    _669,
    /// Claim made date
    ///
    /// The date on which a claim was made.
    #[strum(serialize = "670")]
    _670,
    /// Stop distribution date
    ///
    /// The date on which distribution is to stop.
    #[strum(serialize = "671")]
    _671,
    /// Period assigned
    ///
    /// The period assigned.
    #[strum(serialize = "672")]
    _672,
    /// Lease period
    ///
    /// The period associated with a lease.
    #[strum(serialize = "673")]
    _673,
    /// Forecast period end date
    ///
    /// The ending date of a forecast period.
    #[strum(serialize = "674")]
    _674,
    /// Judgement date
    ///
    /// The date on which a decision from a court of law was rendered.
    #[strum(serialize = "675")]
    _675,
    /// Period worked for the company
    ///
    /// Period of time that was worked for the company.
    #[strum(serialize = "676")]
    _676,
    /// Transport equipment stuffing date and/or time
    ///
    /// The date and/or time on which the stuffing of transport equipment is to or has taken place.
    #[strum(serialize = "677")]
    _677,
    /// Transport equipment stripping date and/or time
    ///
    /// The date and/or time on which the stripping of a transport equipment is to or has taken place.
    #[strum(serialize = "678")]
    _678,
    /// Initial request date
    ///
    /// Date of an initial request.
    #[strum(serialize = "679")]
    _679,
    /// Period overdue
    ///
    /// The period by which an event is overdue.
    #[strum(serialize = "680")]
    _680,
    /// Implementation date/time/period
    ///
    /// A date/time/period within which an implementation is to take place.
    #[strum(serialize = "681")]
    _681,
    /// Refusal period
    ///
    /// The period within which a refusal can be made.
    #[strum(serialize = "682")]
    _682,
    /// Suspension period
    ///
    /// The period for which something is suspended.
    #[strum(serialize = "683")]
    _683,
    /// Deletion date
    ///
    /// The date on which deletion occurs.
    #[strum(serialize = "684")]
    _684,
    /// First sale date and/or time and/or period
    ///
    /// The first date, and/or time, and/or period a product was sold.
    #[strum(serialize = "685")]
    _685,
    /// Last sale date and/or time and/or period
    ///
    /// The last date, and/or time, and/or period a product was sold.
    #[strum(serialize = "686")]
    _686,
    /// Date ready for collection
    ///
    /// A date on which an object is ready for collection.
    #[strum(serialize = "687")]
    _687,
    /// Shipping date, no schedule established as of
    ///
    /// As at this date no valid shipping schedule has been established.
    #[strum(serialize = "688")]
    _688,
    /// Shipping date and/or time, current schedule
    ///
    /// Shipping date and/or time as currently scheduled.
    #[strum(serialize = "689")]
    _689,
    /// Suppliers' average credit period
    ///
    /// The average period of time that credit is extended by suppliers.
    #[strum(serialize = "690")]
    _690,
    /// Advising date
    ///
    /// Date of advice.
    #[strum(serialize = "691")]
    _691,
    /// Project over target baseline date
    ///
    /// The date an over target baseline was implemented for a project.
    #[strum(serialize = "692")]
    _692,
    /// Established date
    ///
    /// Date when an entity was established or created.
    #[strum(serialize = "693")]
    _693,
    /// Latest filing period
    ///
    /// Latest period for which a filing may be made.
    #[strum(serialize = "694")]
    _694,
    /// Mailing date
    ///
    /// Date when an item may be mailed.
    #[strum(serialize = "695")]
    _695,
    /// Date/time of latest accounts filing at public registry
    ///
    /// The latest date/time when financial accounts were filed at public registry.
    #[strum(serialize = "696")]
    _696,
    /// Date placed in disfavour
    ///
    /// Date when placed in a disfavoured category or status.
    #[strum(serialize = "697")]
    _697,
    /// Employment position start date, estimated
    ///
    /// Estimated start date of employment in a particular position.
    #[strum(serialize = "698")]
    _698,
    /// Registered contractor number assignment date, original
    ///
    /// Date when a registered contractor number was originally assigned.
    #[strum(serialize = "699")]
    _699,
    /// Ownership change date
    ///
    /// Date when ownership changes.
    #[strum(serialize = "700")]
    _700,
    /// Original duration
    ///
    /// Original length of time.
    #[strum(serialize = "701")]
    _701,
    /// Period between changes
    ///
    /// The period of time between changes.
    #[strum(serialize = "702")]
    _702,
    /// From date of notice to proceed to commencement of performance
    ///
    /// Period of time from notice to proceed until performance commencement.
    #[strum(serialize = "703")]
    _703,
    /// From date of notice to proceed to completion
    ///
    /// Period of time from date of notice to proceed until completion.
    #[strum(serialize = "704")]
    _704,
    /// Period an event is late due to customer
    ///
    /// The period of time an event is late due to the actions of a customer.
    #[strum(serialize = "705")]
    _705,
    /// File generation date and/or time
    ///
    /// Date and, or time of file generation.
    #[strum(serialize = "706")]
    _706,
    /// Endorsed certificate issue date
    ///
    /// Date on which a certificate, endorsed by signature or other agreed means, is issued.
    #[strum(serialize = "707")]
    _707,
    /// Patient first visit for condition
    ///
    /// The date of the first visit by a patient to a healthcare provider for this condition.
    #[strum(serialize = "708")]
    _708,
    /// Admission date and/or time, expected
    ///
    /// Expected date and/or time of admission.
    #[strum(serialize = "709")]
    _709,
    /// Symptoms onset, patient alleged
    ///
    /// Date and/or time of onset of symptoms according to the patient.
    #[strum(serialize = "710")]
    _710,
    /// Accident benefit period
    ///
    /// To identify the period of time for which benefits are provided in the event of an accident.
    #[strum(serialize = "711")]
    _711,
    /// Accident benefit age limit
    ///
    /// To identify the age to which benefits are provided to the insured in the event of an accident.
    #[strum(serialize = "712")]
    _712,
    /// Accident lifetime benefit qualification age
    ///
    /// To identify the qualification age for lifetime benefits provided to the insured in the event of an accident.
    #[strum(serialize = "713")]
    _713,
    /// Sickness benefit period
    ///
    /// To identify the period of time for which benefits are provided in the event of sickness.
    #[strum(serialize = "714")]
    _714,
    /// Sickness benefit age limit
    ///
    /// To identify the age to which benefits are provided to the insured in the event of sickness.
    #[strum(serialize = "715")]
    _715,
    /// Sickness lifetime benefit qualification age
    ///
    /// To identify the qualification age for lifetime benefits provided to the insured in the event of sickness.
    #[strum(serialize = "716")]
    _716,
    /// Accident insurance elimination period
    ///
    /// To identify the period of time the insured must be disabled in the event of an accident for benefits to be payable by the ceding company.
    #[strum(serialize = "717")]
    _717,
    /// Sickness insurance elimination period
    ///
    /// The period of time the insured must be disabled in the event of sickness for benefits to be payable by the ceding company.
    #[strum(serialize = "718")]
    _718,
    /// Provider signature date
    ///
    /// Date when the provider signed.
    #[strum(serialize = "719")]
    _719,
    /// Condition initial treatment date
    ///
    /// Date when initially treated for this condition.
    #[strum(serialize = "720")]
    _720,
    /// Information release authorization date
    ///
    /// Date when the information was authorized to be released.
    #[strum(serialize = "721")]
    _721,
    /// Benefit release authorization date
    ///
    /// Date when a benefit is authorized for release.
    #[strum(serialize = "722")]
    _722,
    /// Last seen date
    ///
    /// The date when last seen.
    #[strum(serialize = "723")]
    _723,
    /// Acute manifestation date
    ///
    /// The date the symptoms manifested themselves in an acute form.
    #[strum(serialize = "724")]
    _724,
    /// Similar illness onset date
    ///
    /// The date of the onset of an illness similar to the illness currently being treated.
    #[strum(serialize = "725")]
    _725,
    /// Last X-ray date
    ///
    /// The date the last X-ray was taken.
    #[strum(serialize = "726")]
    _726,
    /// Placement date, previous
    ///
    /// The date something was previously placed.
    #[strum(serialize = "727")]
    _727,
    /// Placement date
    ///
    /// The date something is placed.
    #[strum(serialize = "728")]
    _728,
    /// Temporary prosthesis date
    ///
    /// The date a temporary prosthetic device was provided.
    #[strum(serialize = "729")]
    _729,
    /// Orthodontic treatment period, remaining
    ///
    /// The period of time that the orthodontic treatment has remaining.
    #[strum(serialize = "730")]
    _730,
    /// Orthodontic treatment period, total
    ///
    /// The period of orthodontic treatment from beginning to end.
    #[strum(serialize = "731")]
    _731,
    /// Maximum credit granted date
    ///
    /// Date on which the highest credit was granted.
    #[strum(serialize = "732")]
    _732,
    /// Last date of accounts filed at public register
    ///
    /// Date on which accounts were last filed at the public register.
    #[strum(serialize = "733")]
    _733,
    /// Allowed renewal duration period
    ///
    /// The period of time a company can renew its duration period.
    #[strum(serialize = "734")]
    _734,
    /// Offset from Coordinated Universal Time (UTC)
    ///
    /// Number of hour's offset from Coordinated Universal Time (UTC).
    #[strum(serialize = "735")]
    _735,
    /// Appointment expiry date
    ///
    /// Date when an appointment will expire.
    #[strum(serialize = "736")]
    _736,
    /// Earliest filing period
    ///
    /// Earliest period for which a filing is made.
    #[strum(serialize = "737")]
    _737,
    /// Original name change date
    ///
    /// Date when the original name was changed.
    #[strum(serialize = "738")]
    _738,
    /// Education start date
    ///
    /// Date education begins at an educational institution.
    #[strum(serialize = "739")]
    _739,
    /// Education end date
    ///
    /// Date education is completed at an educational institution.
    #[strum(serialize = "740")]
    _740,
    /// Receivership period
    ///
    /// Period of time a receivership lasts.
    #[strum(serialize = "741")]
    _741,
    /// Financial information submission date/time
    ///
    /// Date/time when financial information is submitted.
    #[strum(serialize = "742")]
    _742,
    /// Purchase order latest possible change date
    ///
    /// Date identifying a point of time after which a purchase order cannot be changed.
    #[strum(serialize = "743")]
    _743,
    /// Investment number allocation date
    ///
    /// The date that an investment number was allocated.
    #[strum(serialize = "744")]
    _744,
    /// Mutually defined
    #[strum(ascii_case_insensitive)]
    ZZZ,
}

/// Date or time or period format code
///
/// Code specifying the representation of a date, time or period.
#[derive(Debug, Serialize, Deserialize, Clone, EnumString, Display)]
pub enum _2379 {
    /// DDMMYY
    ///
    /// Calendar date: D = Day; M = Month; Y = Year.
    #[strum(serialize = "2")]
    _2,

    /// MMDDYY
    ///
    /// Calendar date: M = Month; D = Day; Y = Year.
    #[strum(serialize = "3")]
    _3,

    /// DDMMCCYY
    ///
    /// Calendar date C=Century; Y=Year; M=Month; D=Day.
    #[strum(serialize = "4")]
    _4,

    /// DDMMCCYYHHMM
    ///
    /// Calendar date and time: C=Century; Y=Year; M=Month; D=Day; H=Hour; M=Minute.
    #[strum(serialize = "5")]
    _5,

    /// YYMMDD
    ///
    /// Calendar date: Y = Year; M = Month; D = Day.
    #[strum(serialize = "101")]
    _101,

    /// CCYYMMDD
    ///
    /// Calendar date: C = Century ; Y = Year ; M = Month ; D = Day.
    #[strum(serialize = "102")]
    _102,

    /// YYWWD
    ///
    /// Calendar week day: Y = Year ; W = Week ; D = Day Week number 01 is always first week of January Day number 1 is always Monday.
    #[strum(serialize = "103")]
    _103,

    /// YYDDD
    ///
    /// Calendar day: Y = Year ; D = Day January the first = Day 001 Always start numbering the days of the year from January 1st through December 31st.
    #[strum(serialize = "105")]
    _105,

    /// MMDD
    ///
    /// Day of a month: M = Month; D = Day.
    #[strum(serialize = "106")]
    _106,

    /// DDD
    ///
    /// Day's number within a specific year: D = Day.
    #[strum(serialize = "107")]
    _107,

    /// WW
    ///
    /// Week's number within a specific year: W = Week.
    #[strum(serialize = "108")]
    _108,

    /// MM
    ///
    /// Month's number within a specific year: M = Month.
    #[strum(serialize = "109")]
    _109,

    /// DD
    ///
    /// Day's number within is a specific month: D = Day.
    #[strum(serialize = "110")]
    _110,

    /// YYMMDDHHMM
    ///
    /// Calendar date including time without seconds: Y = Year; M = Month; D = Day; H = Hour; M = Minute.
    #[strum(serialize = "201")]
    _201,

    /// YYMMDDHHMMSS
    ///
    /// Calendar date including time with seconds: Y = Year; M = Month; D = Day; H = Hour; m = Minutes = Seconds.
    #[strum(serialize = "202")]
    _202,

    /// CCYYMMDDHHMM
    ///
    /// Calendar date including time with minutes: C=Century; Y=Year; M=Month; D=Day; H=Hour; M=Minutes.
    #[strum(serialize = "203")]
    _203,

    /// CCYYMMDDHHMMSS
    ///
    /// Calendar date including time with seconds: C=Century;Y=Year; M=Month;D=Day;H=Hour;M=Minute;S=Second.
    #[strum(serialize = "204")]
    _204,

    /// CCYYMMDDHHMMZHHMM
    ///
    /// Calendar date including time and time zone expressed in hours and minutes. ZHHMM = time zone given as offset from Coordinated Universal Time (UTC).
    #[strum(serialize = "205")]
    _205,

    /// YYMMDDHHMMZZZ
    ///
    /// See 201 + Z = Time zone.
    #[strum(serialize = "301")]
    _301,

    /// YYMMDDHHMMSSZZZ
    ///
    /// See 202 + Z = Time zone.
    #[strum(serialize = "302")]
    _302,

    /// CCYYMMDDHHMMZZZ
    ///
    /// See 203 plus Z=Time zone.
    #[strum(serialize = "303")]
    _303,

    /// CCYYMMDDHHMMSSZZZ
    ///
    /// See 204 plus Z=Time zone.
    #[strum(serialize = "304")]
    _304,

    /// MMDDHHMM
    ///
    /// Month, day, hours, minutes; M = Month; D = Day; H = Hour; M = Minute.
    #[strum(serialize = "305")]
    _305,

    /// DDHHMM
    ///
    /// Day, hours, minutes; D = Day; H = Hour; M = Minute.
    #[strum(serialize = "306")]
    _306,

    /// HHMM
    ///
    /// Time without seconds: H = Hour; m = Minute.
    #[strum(serialize = "401")]
    _401,

    /// HHMMSS
    ///
    /// Time with seconds: H = Hour; m = Minute; s = Seconds.
    #[strum(serialize = "402")]
    _402,

    /// HHMMSSZZZ
    ///
    /// See 402 plus Z=Time zone.
    #[strum(serialize = "404")]
    _404,

    /// MMMMSS
    ///
    /// Time without hours: m=minutes, s=seconds.
    #[strum(serialize = "405")]
    _405,

    /// ZHHMM
    ///
    /// Offset from Coordinated Universal Time (UTC) where Z is plus (+) or minus (-).
    #[strum(serialize = "406")]
    _406,

    /// HHMMHHMM
    ///
    /// Time span without seconds: H = Hour; m = Minute;.
    #[strum(serialize = "501")]
    _501,

    /// HHMMSS-HHMMSS
    ///
    /// Format of period to be given without hyphen.
    #[strum(serialize = "502")]
    _502,

    /// HHMMSSZZZ-HHMMSSZZZ
    ///
    /// Format of period to be given without hyphen.
    #[strum(serialize = "503")]
    _503,

    /// CC
    ///
    /// Century.
    #[strum(serialize = "600")]
    _600,

    /// YY
    ///
    /// Calendar year: Y = Year.
    #[strum(serialize = "601")]
    _601,

    /// CCYY
    ///
    /// Calendar year including century: C = Century; Y = Year.
    #[strum(serialize = "602")]
    _602,

    /// YYS
    ///
    /// Semester in a calendar year: Y = Year; S = Semester.
    #[strum(serialize = "603")]
    _603,

    /// CCYYS
    ///
    /// Semester in a calendar year: C = Century; Y = Year; S = Semester.
    #[strum(serialize = "604")]
    _604,

    /// CCYYQ
    ///
    /// Quarter in a calendar year: C = Century; Y = Year; Q = Quarter.
    #[strum(serialize = "608")]
    _608,

    /// YYMM
    ///
    /// Month within a calendar year: Y = Year; M = Month.
    #[strum(serialize = "609")]
    _609,

    /// CCYYMM
    ///
    /// Month within a calendar year: CC = Century; Y = Year; M = Month.
    #[strum(serialize = "610")]
    _610,

    /// YYMMA
    ///
    /// Format of period to be given without hyphen (A = ten days period).
    #[strum(serialize = "613")]
    _613,

    /// CCYYMMA
    ///
    /// Format of period to be given without hyphen (A = ten days period).
    #[strum(serialize = "614")]
    _614,

    /// YYWW
    ///
    /// Week within a calendar year: Y = Year; W = Week 1st week of January = week 01.
    #[strum(serialize = "615")]
    _615,

    /// CCYYWW
    ///
    /// Week within a calendar year: CC = Century; Y = Year; W = Week (1st week of January = week 01).
    #[strum(serialize = "616")]
    _616,

    /// YY-YY
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "701")]
    _701,

    /// CCYY-CCYY
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "702")]
    _702,

    /// YYS-YYS
    ///
    /// Format of period to be given without hyphen.
    #[strum(serialize = "703")]
    _703,

    /// CCYYS-CCYYS
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "704")]
    _704,

    /// YYPYYP
    ///
    /// Format of period to be given without hyphen (P = period of 4 months).
    #[strum(serialize = "705")]
    _705,

    /// CCYYP-CCYYP
    ///
    /// Format of period to be given without hyphen (P = period of 4 months).
    #[strum(serialize = "706")]
    _706,

    /// YYQ-YYQ
    ///
    /// Format of period to be given without hyphen.
    #[strum(serialize = "707")]
    _707,

    /// CCYYQ-CCYYQ
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "708")]
    _708,

    /// YYMM-YYMM
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "709")]
    _709,

    /// CCYYMM-CCYYMM
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "710")]
    _710,

    /// CCYYMMDD-CCYYMMDD
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "711")]
    _711,

    /// YYMMDDHHMM-YYMMDDHHMM
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "713")]
    _713,

    /// YYWW-YYWW
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "715")]
    _715,

    /// CCYYWW-CCYYWW
    ///
    /// Format of period to be given without hyphen.
    #[strum(serialize = "716")]
    _716,

    /// YYMMDD-YYMMDD
    ///
    /// Format of period to be given in actual message without hyphen.
    #[strum(serialize = "717")]
    _717,

    /// CCYYMMDD-CCYYMMDD
    ///
    /// Format of period to be given without hyphen.
    #[strum(serialize = "718")]
    _718,

    /// CCYYMMDDHHMM-CCYYMMDDHHMM
    ///
    /// A period of time which includes the century, year, month, day, hour and minute. Format of period to be given in actual message without hyphen.
    #[strum(serialize = "719")]
    _719,

    /// DHHMM-DHHMM
    ///
    /// Format of period to be given without hyphen (D=day of the week, 1=Monday; 2=Tuesday; ... 7=Sunday).
    #[strum(serialize = "720")]
    _720,

    /// Year
    ///
    /// To indicate a quantity of years.
    #[strum(serialize = "801")]
    _801,

    /// Month
    ///
    /// To indicate a quantity of months.
    #[strum(serialize = "802")]
    _802,

    /// Week
    ///
    /// To indicate a quantity of weeks.
    #[strum(serialize = "803")]
    _803,

    /// Day
    ///
    /// To indicate a quantity of days.
    #[strum(serialize = "804")]
    _804,

    /// Hour
    ///
    /// To indicate a quantity of hours.
    #[strum(serialize = "805")]
    _805,

    /// Minute
    ///
    /// To indicate a quantity of minutes.
    #[strum(serialize = "806")]
    _806,

    /// Second
    ///
    /// To indicate a quantity of seconds.
    #[strum(serialize = "807")]
    _807,

    /// Semester
    ///
    /// To indicate a quantity of semesters (six months).
    #[strum(serialize = "808")]
    _808,

    /// Four months period
    ///
    /// To indicate a quantity of four months periods.
    #[strum(serialize = "809")]
    _809,

    /// Trimester
    ///
    /// To indicate a quantity of trimesters (three months).
    #[strum(serialize = "810")]
    _810,

    /// Half month
    ///
    /// To indicate a quantity of half months.
    #[strum(serialize = "811")]
    _811,

    /// Ten days
    ///
    /// To indicate a quantity of ten days periods.
    #[strum(serialize = "812")]
    _812,

    /// Day of the week
    ///
    /// Numeric representation of the day (Monday = 1).
    #[strum(serialize = "813")]
    _813,

    /// Working days
    ///
    /// Number of working days.
    #[strum(serialize = "814")]
    _814,
}

/// Party function code qualifier
///
/// Code giving specific meaning to a party.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _3035 {
    ///Party to be billed (AAR Accounting rule 11)
    ///
    ///Party to be billed in accordance with AAR Accounting rule 11.
    AA,
    ///Buyer's agent/representative
    ///
    ///Third party who arranged the purchase of merchandise on behalf of the actual buyer.
    AB,
    ///Declarant's agent/representative
    ///
    ///Any natural or legal person who makes a declaration to an official body on behalf of another natural or legal person, where legally permitted (CCC).
    AE,
    ///Transit principal
    ///
    ///Natural or legal person responsible for the satisfactory performance of a Customs transit operation. Source: CCC.
    AF,
    ///Agent/representative
    ///
    ///(3196) Party authorized to act on behalf of another party.
    AG,
    ///Transit principal's agent/representative
    ///
    ///Agent acting on behalf of the transit principal (CCC).
    AH,
    ///Successful job applicant
    ///
    ///Person who has been chosen for a job.
    AI,
    ///Party issuing mutually agreed codes
    ///
    ///The party which has issued all mutually agreed codes used in the message.
    AJ,
    ///Acknowledgement recipient
    ///
    ///Party to whom acknowledgement should be sent.
    AK,
    ///Principal
    ///
    ///(3340) Party accepting liability for goods held or moving (e.g. transit) under a Customs authorization and - when applicable - a guarantee.
    AL,
    ///Authorized official
    ///
    ///Employee of a company or firm authorized to act on behalf of that company or firm e.g. to make a Customs declaration.
    AM,
    ///Approved importer
    ///
    ///Person or company which is authorised by the relevant Customs authority to import goods without payment all taxes or specific taxes at the point of entry into the country.
    AN,
    ///Account of
    ///
    ///Party account is assigned to.
    AO,
    ///Accepting party
    ///
    ///(3352) Party accepting goods, products, services etc.
    AP,
    ///Approved consignor
    ///
    ///Person or company approved by the relevant authority in the country to pack and export specific goods under Customs supervision.
    AQ,
    ///Authorized exporter
    ///
    ///Exporter authorized/approved by Customs for special Customs procedures e.g. simplified procedure.
    AR,
    ///Account servicing financial institution
    ///
    ///Identifies the financial institution servicing the account(s).
    AS,
    ///Authorized importer
    ///
    ///Importer authorized/approved by Customs for special Customs procedures e.g. simplified procedure.
    AT,
    ///Authorized trader (transit)
    ///
    ///Trader authorized/approved by Customs for special transit procedures e.g. simplified procedure.
    AU,
    ///Authorizing official
    ///
    ///Party that has delegated the authority to take a certain action on behalf of a company or agency.
    AV,
    ///Applicant's bank
    ///
    ///Financial institution which is requested to issue the documentary credit.
    AW,
    ///Authenticating party
    ///
    ///Party which certifies that a document is authentic.
    AX,
    ///Animal being investigated
    ///
    ///Animal being investigated.
    AY,
    ///Issuing bank
    ///
    ///Financial institution which issues the documentary credit, if the applicant's bank is not acting as the issuing bank.
    AZ,
    ///Contact bank 1
    ///
    ///Identifies an additional bank which must be informed of certain aspects of the message.
    B1,
    ///Contact bank 2
    ///
    ///Identifies an additional bank which must be informed of certain aspects of the message.
    B2,
    ///Booking agent
    ///
    ///Party acting as a booking office for transport and forwarding services.
    BA,
    ///Buyer's bank
    ///
    ///[3420] Bank employed by the buyer to make payment.
    BB,
    ///Negotiating bank
    ///
    ///Financial institution to whom a negotiable documentary credit is directed.
    BC,
    ///Documentary credit reimbursing bank
    ///
    ///Self-explanatory.
    BD,
    ///Beneficiary
    ///
    ///The ultimate recipient of the funds. Normally the account owner who is reimbursed by the payor.
    BE,
    ///Beneficiary's bank
    ///
    ///Identifies the account servicer for the beneficiary or the payee.
    BF,
    ///Employer
    ///
    ///Self-explanatory.
    BG,
    ///Previous employer
    ///
    ///Previous employer of a person(s).
    BH,
    ///Buyer's financial institution
    ///
    ///Financial institution designated by buyer to make payment.
    BI,
    ///Release to party
    ///
    ///Party to which the goods or container(s) is (are) to be released.
    BJ,
    ///Financial institution
    ///
    ///Party acting as financial institution.
    BK,
    ///Bill of lading recipient
    ///
    ///Party to receive B/L.
    BL,
    ///Insured
    ///
    ///Party which is the object of an insurance contract.
    BM,
    ///Insurance beneficiary
    ///
    ///Party which benefits from insurance coverage.
    BN,
    ///Broker or sales office
    ///
    ///Party acting in the name of the seller as broker or as sales office.
    BO,
    ///Building site purchaser
    ///
    ///Party at the building site responsible for the purchasing of goods and services for that particular site.
    BP,
    ///Cheque drawn bank
    ///
    ///Identifies the bank on which the cheque should be drawn, as instructed by the ordering customer.
    BQ,
    ///Bill and ship to
    ///
    ///Party receiving goods and relevant invoice.
    BS,
    ///Party to be billed for other than freight (bill to)
    ///
    ///Party receiving invoice excluding freight costs.
    BT,
    ///Service bureau
    ///
    ///Party carrying out service bureau processing work, (e.g. a payroll bureau).
    BU,
    ///Member
    ///
    ///Member of a group (e.g. of a group of persons or a service scheme).
    BV,
    /// Borrower
    BW,
    ///Building site engineer
    ///
    ///Party at the building site responsible for engineering matters for that particular site.
    BX,
    ///Buyer
    ///
    ///Party to whom merchandise and/or service is sold.
    BY,
    ///Building site forwarder
    ///
    ///Party at the building site responsible for forwarding the received goods on that particular site.
    BZ,
    ///In care of party no. 1
    ///
    ///Description to be provided.
    C1,
    ///In care of party no. 2
    ///
    ///Description to be provided.
    C2,
    ///Carrier
    ///
    ///(3126) Party undertaking or arranging transport of goods between named points.
    CA,
    ///Customs broker
    ///
    ///Agent or representative or a professional Customs clearing agent who deals directly with Customs on behalf of the importer or exporter (CCC).
    CB,
    ///Claimant
    ///
    ///Party who claims goods or insurance.
    CC,
    ///Agent's bank
    ///
    ///Bank of the agent.
    CD,
    ///Ceding company
    ///
    ///Description to be provided.
    CE,
    ///Container operator/lessee
    ///
    ///Party to whom the possession of specified property (e.g. container) has been conveyed for a period of time in return for rental payments.
    CF,
    ///Carrier's agent
    ///
    ///Party authorized to act for or on behalf of carrier.
    CG,
    ///Connecting carrier
    ///
    ///Owner or operator of a transportation conveyance to which goods in a given transaction will be transferred.
    CH,
    ///Commission processor
    ///
    ///Party who provides extra treatment to goods on commission base.
    CI,
    ///Previous member
    ///
    ///Previous member of a group of persons or a service scheme.
    CJ,
    ///Empty equipment despatch party
    ///
    ///Party from whose premises empty equipment will be or has been despatched.
    CK,
    ///Container location party
    ///
    ///Party from whose premises container will be or has been despatched.
    CL,
    ///Customs
    ///
    ///Identification of customs authority relevant to the transaction or shipment.
    CM,
    ///Consignee
    ///
    ///(3132) Party to which goods are consigned.
    CN,
    ///Cash pool top account servicing financial institution
    ///
    ///Identification of a financial institution servicing the top account of a cash pool.
    CNX,
    ///Cash pool level account servicing financial institution
    ///
    ///Identification of a financial institution servicing the level account of a cash pool.
    CNY,
    ///Cash pool sub-account servicing financial institution
    ///
    ///Identification of a financial institution servicing the sub-account of a cash pool.
    CNZ,
    ///Corporate office
    ///
    ///Identification of the Head Office within a company.
    CO,
    ///Entity in which a financial interest is held
    ///
    ///Business in which a financial interest is held.
    COA,
    ///Intermediate level parent company
    ///
    ///Identifies an intermediate parent company.
    COB,
    ///Transshipment party
    ///
    ///A party responsible for transshipment.
    COC,
    ///Quotation requesting party
    ///
    ///Party sending a request for a quotation.
    COD,
    ///Party maintaining the codes used in the message
    ///
    ///The party which maintains the codes used in the message.
    COE,
    ///Party maintaining the identifiers used in the message
    ///
    ///The party which maintains the identifiers used in the message.
    COF,
    ///Dispatcher
    ///
    ///An individual responsible for sending something to a destination.
    COG,
    ///Submitter of sample
    ///
    ///An entity responsible for the submission of a sample.
    COH,
    ///Institutional provider
    ///
    ///The institution providing the service.
    COI,
    ///Primary health care provider
    ///
    ///Health care provider that has primary responsibility for patient.
    COJ,
    ///Assistant surgeon
    ///
    ///Physician assisting in surgery.
    COK,
    ///Admitting health care provider
    ///
    ///Health care provider that admitted the patient.
    COL,
    ///Referring health care provider
    ///
    ///Health care provider that referred patient to current provider of services.
    COM,
    ///Supervising health care provider
    ///
    ///Health care provider that supervised the rendering of a service.
    CON,
    ///Party providing financing
    ///
    ///Identifies the party providing the financing.
    COO,
    ///Convoying party
    ///
    ///Party designated to escort the transported goods.
    COP,
    ///Nominated bank
    ///
    ///Identifies the nominated bank.
    COQ,
    ///Family member
    ///
    ///Identifies a family member.
    COR,
    ///Co-participant
    ///
    ///Identifies another party who participates in an activity.
    COS,
    ///Involved party
    ///
    ///Party which is involved in an activity.
    COT,
    ///Assigner
    ///
    ///Identifies the entity who assigns.
    COU,
    ///Registered principal
    ///
    ///An individual who is registered as a principal for an entity.
    COV,
    ///Freight payer on behalf of the consignor
    ///
    ///Freight payer is a third party acting on behalf of the consignor.
    COW,
    ///Freight payer on behalf of the consignee
    ///
    ///Freight payer is a third party acting on behalf of the consignee.
    COX,
    ///Party responsible for disinfection
    ///
    ///Party responsible for performing disinfection operations.
    COY,
    ///Party responsible for refueling
    ///
    ///Party responsible for performing refueling operations.
    COZ,
    ///Party to receive certificate of compliance
    ///
    ///Party acting for or on behalf of seller in matters concerning compliance.
    CP,
    ///Advising bank
    ///
    ///Identifies the financial institution used by the issuing bank to advise the documentary credit.
    CPA,
    ///Reimbursing bank
    ///
    ///Identifies the financial institution through which the reimbursement is to be effected.
    CPB,
    ///Advise through bank
    ///
    ///Identifies the financial institution through which the advising bank is to advise.
    CPC,
    ///Charges payer at destination
    ///
    ///Party, other than the ordering party, which has to pay the charges concerning the destination operations.
    CPD,
    ///Vessel master
    ///
    ///Master of the conveyance.
    CPE,
    ///Means of transport charterer
    ///
    ///Charterer of the means of transport.
    CPF,
    ///Excise party
    ///
    ///Party to whom excise must be paid.
    CPG,
    ///Copy report to
    ///
    ///Party receiving a copy of a report.
    CPH,
    ///Related healthcare party
    ///
    ///A healthcare party related to the subject.
    CPI,
    ///Clinical information provider
    ///
    ///Party providing clinical information.
    CPJ,
    ///Service requester
    ///
    ///Party requesting a service.
    CPK,
    ///Patient admitted by
    ///
    ///Party who admitted a patient.
    CPL,
    ///Patient discharged to
    ///
    ///The party who receives the discharged patient.
    CPM,
    ///Patient hosted by
    ///
    ///The party hosting the patient.
    CPN,
    ///Prescriber's contact person
    ///
    ///Contact person for the prescriber.
    CPO,
    ///Cheque order
    ///
    ///Party to which the cheque will be ordered, when different from the beneficiary.
    CQ,
    ///Empty equipment return party
    ///
    ///Party to whose premises empty equipment will be or has been returned.
    CR,
    ///Consolidator
    ///
    ///Party consolidating various consignments, payments etc.
    CS,
    ///Consignee to be specified
    ///
    ///The party to be identified at a later time as the consignee.
    CT,
    ///Container return company
    ///
    ///The company to which containers have to be returned.
    CU,
    ///Consignee of vessel
    ///
    ///Description to be provided.
    CV,
    ///Equipment owner
    ///
    ///Owner of equipment (container, etc.).
    CW,
    ///Consignee's agent
    ///
    ///Party authorized to act on behalf of the consignee.
    CX,
    /// Commissionable agent
    ///
    /// IATA cargo agent entitled to commission.
    CY,
    ///Consignor
    ///
    ///(3336) Party which, by contract with a carrier, consigns or sends goods with the carrier, or has them conveyed by him. Synonym: shipper, sender.
    CZ,
    ///Available with bank (documentary credits)
    ///
    ///Financial institution with whom the documentary credit is available.
    DA,
    ///Distributor branch
    ///
    ///The affiliate of a retailer or distributor.
    DB,
    ///Deconsolidator
    ///
    ///Party that splits up a large consignment composed of separate consignments of goods. The smaller consignments of goods were grouped together into that large consignment for carriage as a larger unit in order to obtain a reduced rate.
    DC,
    ///Despatch charge payer
    ///
    ///Party, other than the ordering party, which has to pay the charges concerning the despatch operations.
    DCP,
    ///Prescription database owner
    ///
    ///Organisation or person owning a prescription database.
    DCQ,
    ///Original prescriber
    ///
    ///The doctor who issued the original prescription.
    DCR,
    ///Temporary employee
    ///
    ///A person employed on a temporary basis.
    DCS,
    ///Designer
    ///
    ///A party who designs.
    DCT,
    ///Quotation delivered to
    ///
    ///Party to whom the quotation is to be or has been delivered.
    DCU,
    ///Developer
    ///
    ///A party who develops.
    DCV,
    ///Test execution party
    ///
    ///The party performing a test.
    DCW,
    ///Party to receive refund
    ///
    ///Party to whom a refund is given.
    DCX,
    ///Authorised issuer of prescription
    ///
    ///Party authorised to issue a prescription.
    DCY,
    ///Authorised dispenser of medicine
    ///
    ///Organisation or person authorised to dispense medicine.
    DCZ,
    ///Documentary credit account party's bank
    ///
    ///Bank of the documentary credit account party.
    DD,
    ///Report responsible party
    ///
    ///The party or person taking responsibility for a report.
    DDA,
    ///Initial sender
    ///
    ///The party who does the initial sending.
    DDB,
    ///The party authorising the original prescription
    ///
    ///The party authorising the issuer of the original prescription.
    DDC,
    ///Depositor
    ///
    ///Party depositing goods, financial payments or documents.
    DE,
    ///Documentary credit applicant
    ///
    ///Party at whose request the applicant's bank/issuing bank is to issue a documentary credit.
    DF,
    ///Documentary credit beneficiary
    ///
    ///Party in whose favour the documentary credit is to be issued and the party that must comply with the credit's terms and conditions.
    DG,
    ///Documentary credit account party
    ///
    ///Party which is responsible for the payment settlement of the documentary credit with the applicant's bank/issuing bank, if different from the documentary credit applicant.
    DH,
    ///Documentary credit second beneficiary
    ///
    ///Party to whom the documentary credit can be transferred.
    DI,
    ///Party according to documentary credit transaction
    ///
    ///Party related to documentary credit transaction.
    DJ,
    ///Documentary credit beneficiary's bank
    ///
    ///Financial institution with which the beneficiary of the documentary credit maintains an account.
    DK,
    ///Factor
    ///
    ///Company offering a financial service whereby a firm sells or transfers title to its accounts receivable to the factoring company.
    DL,
    /// Party to whom documents are to be presented
    DM,
    ///Owner of operation
    ///
    ///Owner of the operation.
    DN,
    ///Document recipient
    ///
    ///(1370) Party which should receive a specified document.
    DO,
    ///Delivery party
    ///
    ///(3144) Party to which goods should be delivered, if not identical with consignee.
    DP,
    ///Owner's agent
    ///
    ///Person acting on delegation of powers of the owner.
    DQ,
    ///Driver
    ///
    ///Person who drives a means of transport.
    DR,
    ///Distributor
    ///
    ///Party distributing goods, financial payments or documents.
    DS,
    ///Declarant
    ///
    ///(3140) Party who makes a declaration to an official body or - where legally permitted - in whose name, or on whose behalf, a declaration to an official body is made.
    DT,
    ///Owner's representative
    ///
    ///Person commissioned by the owner to represent him in certain circumstances.
    DU,
    ///Project management office
    ///
    ///Party commissioned by the owner to follow through the execution of all works.
    DV,
    ///Drawee
    ///
    ///Party on whom drafts must be drawn.
    DW,
    ///Engineer (construction)
    ///
    ///Party representing the contractor to advise and supervise engineering aspects of the works.
    DX,
    ///Engineer, resident (construction)
    ///
    ///Party commissioned by the owner to advise and supervise engineering aspects of the works.
    DY,
    /// Architect
    DZ,
    ///Architect-designer
    ///
    ///Designer of the construction project.
    EA,
    ///Building inspectorate
    ///
    ///Party controlling the conformity of works to legal and regulation rules.
    EB,
    ///Exchanger
    ///
    ///Party exchanging currencies or goods.
    EC,
    ///Engineer, consultant
    ///
    ///Party providing professional engineering services.
    ED,
    /// Location of goods for customs examination before clearance
    ///
    /// SE.
    EE,
    ///Project coordination office
    ///
    ///Party responsible for technical coordination of works.
    EF,
    ///Surveyor, topographical
    ///
    ///Party responsible for topographical measurements.
    EG,
    ///Engineer, measurement
    ///
    ///Party responsible for quantity measurements.
    EH,
    ///Controller, quality
    ///
    ///Party controlling the quality of goods and workmanship for the project.
    EI,
    ///Surveyor, quantity
    ///
    ///Party responsible for the quantification and valuation of the works on behalf of the contractor.
    EJ,
    ///Surveyor (professional), quantity
    ///
    ///Party responsible to the owner for the quantification and valuation of the works.
    EK,
    ///Project
    ///
    ///Party responsible for a project, e.g. a construction project.
    EL,
    ///Party to receive electronic memo of invoice
    ///
    ///Party being informed about invoice issue (via EDI).
    EM,
    ///Tenderer
    ///
    ///Firm answering an invitation to tender.
    EN,
    ///Owner of equipment
    ///
    ///Party who owns equipment.
    EO,
    ///Equipment drop-off party
    ///
    ///The party which drops off equipment.
    EP,
    ///Empty container responsible party
    ///
    ///Party responsible for the empty container.
    EQ,
    ///Empty container return agent
    ///
    ///Party, designated by owner of containers, responsible for their collection as agreed between the owner and customer/ consignee.
    ER,
    ///Contractor, lead
    ///
    ///Leader representing a grouping of co-contractors.
    ES,
    ///Co-contractor
    ///
    ///Member of a grouping of co-contractors.
    ET,
    ///Contractor, general
    ///
    ///Single contractor for the whole construction project, working by his own or with subcontractors.
    EU,
    ///Subcontractor
    ///
    ///Firm carrying out a part of the works for a contractor.
    EV,
    ///Subcontractor with direct payment
    ///
    ///Subcontractor benefiting from direct payments.
    EW,
    ///Exporter
    ///
    ///(3030) Party who makes - or on whose behalf a Customs clearing agent or other authorized person makes - an export declaration. This may include a manufacturer, seller or other person. Within a Customs union, consignor may have the same meaning as exporter.
    EX,
    ///Subcontractor, nominated
    ///
    ///Subcontractor authorized by the owner after having been proposed.
    EY,
    ///Operator, essential services
    ///
    ///Operator of essential services e.g. water, sewerage system, power.
    EZ,
    ///Operator, communication channel
    ///
    ///Operator of a communication channel.
    FA,
    ///Nominated freight company
    ///
    ///Party nominated to act as transport company or carrier for the goods.
    FB,
    ///Contractor, main
    ///
    ///Firm or grouping of co-contractors which has been awarded the contract.
    FC,
    ///Buyer's parent company
    ///
    ///Parent company, e.g. holding company.
    FD,
    /// Credit rating agency
    FE,
    ///Factor, correspondent
    ///
    ///Factoring company engaged by another factoring company to assist the letter with the services provided to the clients (sellers).
    FF,
    ///Buyer as officially registered
    ///
    ///Buying party as officially registered with government.
    FG,
    ///Seller as officially registered
    ///
    ///Selling party as officially registered with government.
    FH,
    ///Copy message to
    ///
    ///Party that is to receive a copy of a message.
    FI,
    ///Trade Union
    ///
    ///Organisation representing employees.
    FJ,
    ///Previous Trade Union
    ///
    ///Employee organisation who previously represented an employee .
    FK,
    ///Passenger
    ///
    ///A person conveyed by a means of transport, other than the crew.
    FL,
    ///Crew member
    ///
    ///A person manning a means of transport.
    FM,
    ///Tariff issuer
    ///
    ///The issuer of a tariff, e.g. a freight tariff.
    FN,
    /// Party performing inspection
    FO,
    ///Freight/charges payer
    ///
    ///Party responsible for the payment of freight.
    FP,
    ///Container survey agent
    ///
    ///The container survey agency that will survey the containers.
    FQ,
    ///Message from
    ///
    ///Party where the message comes from.
    FR,
    ///Party authorized to make definite a contract action
    ///
    ///Party who has the authority to make definite a contract action.
    FS,
    ///Party responsible for financial settlement
    ///
    ///(3450) Party responsible for either the transfer or repatriation of the funds relating to a transaction.
    FT,
    ///Hazardous material office
    ///
    ///The office responsible for providing information regarding hazardous material.
    FU,
    ///Party providing government furnished property
    ///
    ///The party responsible for providing government furnished property.
    FV,
    ///Freight forwarder
    ///
    ///Party arranging forwarding of goods.
    FW,
    ///Current receiver
    ///
    ///Current receiver of the goods in a multi-step transportation process (indirect flow) involving at least one grouping centre.
    FX,
    ///Current sender
    ///
    ///Current sender of the goods in a multi-step transportation process (indirect flow) involving at least one grouping centre.
    FY,
    ///Grouping centre
    ///
    ///A party in charge of groupage, including degroupage and regroupage.
    FZ,
    ///Road carrier
    ///
    ///A road carrier moving cargo.
    GA,
    ///Chamber of commerce
    ///
    ///Name of the Chamber of Commerce of the town where the company is registered.
    GB,
    ///Goods custodian
    ///
    ///(3024) Party responsible for the keeping of goods.
    GC,
    ///Producer
    ///
    ///Party or person who has produced the produce.
    GD,
    ///Registration tribunal
    ///
    ///Name of the tribunal where the company is registered.
    GE,
    ///Slot charter party
    ///
    ///An identification code of a participant or user that books slots (space) on a ship, more likely on a long term basis on a series of sailings. He pays for the space whether he uses it or not.
    GF,
    ///Warehouse
    ///
    ///The name of the warehouse where product is held.
    GG,
    ///Applicant for job
    ///
    ///A person who applied for a job.
    GH,
    ///Spouse
    ///
    ///Person is a spouse.
    GI,
    ///Mother
    ///
    ///Person is a mother.
    GJ,
    ///Father
    ///
    ///Person is a father.
    GK,
    ///Socially insured person
    ///
    ///A person who is registered in a social security scheme.
    GL,
    ///Inventory controller
    ///
    ///To specifically identify the party in charge of inventory control.
    GM,
    ///Processor
    ///
    ///Party or person who has or will apply a process.
    GN,
    ///Goods owner
    ///
    ///The party which owns the goods.
    GO,
    ///Packer
    ///
    ///Party or person who has undertaken or will undertake packing.
    GP,
    ///Slaughterer
    ///
    ///Party or person who has undertaken or will undertake a slaughter.
    GQ,
    ///Goods releaser
    ///
    ///(3026) Party entitled to authorize release of goods from custodian.
    GR,
    ///Consignor's representative
    ///
    ///Party authorised to represent the consignor.
    GS,
    ///Rail carrier
    ///
    ///A carrier moving cargo, including containers, via rail.
    GT,
    ///Originator of article number
    ///
    ///A code identifying the party which created a specific article number.
    GU,
    ///Procurement responsibility for order
    ///
    ///A code used to identify the organization which is responsible for the procurement.
    GV,
    ///Party fulfilling all operations
    ///
    ///Code indicating the fact that the party identified carries out all operations within that company's activities.
    GW,
    ///Central catalogue party
    ///
    ///Party controlling a central catalogue.
    GX,
    ///Inventory reporting party
    ///
    ///Party reporting inventory information.
    GY,
    ///Substitute supplier
    ///
    ///Party which may be in a position to supply products or services should the main usual supplier be unable to do so.
    GZ,
    ///Party which delivers consignments to the terminal
    ///
    ///Party which delivers consignments to a terminal.
    HA,
    ///Party which picks up consignments from the terminal
    ///
    ///Party which picks up consignments from a terminal.
    HB,
    ///Transit freight forwarder
    ///
    ///Freight forwarder to whom transit consignments are addressed, and from whom they are to be on-forwarded.
    HC,
    ///Inspection and acceptance party
    ///
    ///The party who will perform inspection and acceptance.
    HD,
    ///Transportation office
    ///
    ///The office that provides transportation information.
    HE,
    ///Contract administration office
    ///
    ///The office responsible for the administration of a contract.
    HF,
    ///Investigator
    ///
    ///A party who conducts investigations.
    HG,
    ///Audit office
    ///
    ///The office responsible for conducting audits.
    HH,
    ///Requestor
    ///
    ///The party requesting an action.
    HI,
    ///Foreign disclosure information office
    ///
    ///The office that reviews sensitive information for foreign disclosure.
    HJ,
    ///Mark-for party
    ///
    ///The party within an organization for whom the material is marked to be delivered.
    HK,
    ///Party to receive reports
    ///
    ///The party to whom reports are to be submitted.
    HL,
    ///Alternative manufacturer
    ///
    ///Party identification of an alternative manufacturer for a product.
    HM,
    ///Service performer
    ///
    ///The party who is performing a service.
    HN,
    ///Shipper's association
    ///
    ///An association of shippers.
    HO,
    ///Final message recipient
    ///
    ///To identify the final recipient of the message.
    HP,
    ///Account owner
    ///
    ///Identifies the owner of the account.
    HQ,
    ///Shipping line service
    ///
    ///Identifies the shipping line service organization.
    HR,
    ///Creditor
    ///
    ///Party to whom payment is due.
    HS,
    ///Clearing house
    ///
    ///Institution through which funds will be paid.
    HT,
    ///Ordering bank
    ///
    ///Bank which instructed the sender to act on the transaction(s).
    HU,
    ///Receiver of funds
    ///
    ///Identifies the party that receives the funds.
    HV,
    ///Sender of funds
    ///
    ///Identifies the party that sends the funds.
    HW,
    ///Debtor
    ///
    ///Party from whom payment is due.
    HX,
    ///Presenting bank
    ///
    ///The bank which presents documents to the drawee.
    HY,
    ///Work team
    ///
    ///Team responsible for performing work.
    HZ,
    ///Intermediary bank 1
    ///
    ///A financial institution between the ordered bank and the beneficiary's bank.
    I1,
    ///Intermediary bank 2
    ///
    ///A financial institution between the ordered bank and the beneficiary's bank.
    I2,
    ///Intermediary/broker
    ///
    ///Description to be provided.
    IB,
    ///Intermediate consignee
    ///
    ///The intermediate consignee.
    IC,
    ///Replacing manufacturer
    ///
    ///A code used to identify a party who replaces the previous party for the manufacture of an article.
    ID,
    ///Non-resident third party company with whom financial account is held
    ///
    ///Identifies the non-resident third party company with whom the financial account is held.
    IE,
    ///Non-resident group company with whom financial account is held
    ///
    ///Identifies the non-resident group company with whom the financial account is held.
    IF,
    ///Non-resident beneficiary
    ///
    ///The ultimate non-resident recipient of the funds. Normally the account owner who is reimbursed by the payor.
    IG,
    ///Resident beneficiary
    ///
    ///The ultimate resident recipient of the funds. Normally the account owner who is reimbursed by the payor.
    IH,
    ///Issuer of invoice
    ///
    ///(3028) Party issuing an invoice.
    II,
    ///Non-resident instructing party
    ///
    ///Identifies the non-resident party originating the instruction.
    IJ,
    ///Resident instructing party
    ///
    ///Identifies the resident party originating the instruction.
    IL,
    ///Importer
    ///
    ///(3020) Party who makes - or on whose behalf a Customs clearing agent or other authorized person makes - an import declaration. This may include a person who has possession of the goods or to whom the goods are consigned.
    IM,
    ///Insurer
    ///
    ///Description to be provided.
    IN,
    ///Insurance company
    ///
    ///Description to be provided.
    IO,
    ///Insurance claim adjuster
    ///
    ///Description to be provided.
    IP,
    ///Domestic financial institution
    ///
    ///Domestic party acting as financial institution.
    IQ,
    ///Non-domestic financial institution
    ///
    ///Non-domestic party acting as financial institution.
    IR,
    ///Party to receive certified inspection report
    ///
    ///Party (at buyer) to receive certified inspection report.
    IS,
    ///Installation on site
    ///
    ///Description to be provided.
    IT,
    ///Non-resident debtor
    ///
    ///Non-resident party who makes the payment or against whom a claim exists.
    IU,
    ///Invoicee
    ///
    ///(3006) Party to whom an invoice is issued.
    IV,
    ///Non-resident creditor
    ///
    ///Non-resident party receiving the payment or against whom a liability exists.
    IW,
    ///Supplier work team
    ///
    ///The supplier's team responsible for performing the work.
    IX,
    ///Tenant manager
    ///
    ///A code to identify the party who rents the rights to use the goodwill and facilities of an enterprise.
    IY,
    ///Party mandated to liquidate an enterprise
    ///
    ///A code to identify the party who has been legally mandated to sell off an enterprise.
    IZ,
    ///Certified accountant
    ///
    ///Code identifying the party as a certified accountant.
    JA,
    ///Goods collection party
    ///
    ///Party that will collect or has collected the goods.
    JB,
    ///Party at final place of positioning
    ///
    ///Identifies the party at the final place of positioning.
    JC,
    ///Customs office of clearance
    ///
    ///Identifies the office where customs clearance procedures take place.
    JD,
    ///Party from whom customs documents are to be picked up
    ///
    ///Identification of the party from whom customs documents are to be picked up.
    JE,
    ///Party from whom non-customs documents are to be picked up
    ///
    ///Identification of the party from whom non-customs documents are to be picked up.
    JF,
    ///Party to receive customs documents
    ///
    ///Identification of the party to whom customs documents are to be delivered.
    JG,
    ///Party to receive non-customs documents
    ///
    ///Identification of the party to whom non-customs documents are to be delivered.
    JH,
    ///Party designated to provide living animal care
    ///
    ///Party responsible to take care of transported living animals.
    LA,
    ///Co-producer
    ///
    ///A code used to identify a party who participates in production.
    LB,
    ///Party declaring the Value Added Tax (VAT)
    ///
    ///A code to identify the party who is responsible for declaring the Value Added Tax (VAT) on the sale of goods or services.
    LC,
    ///Party recovering the Value Added Tax (VAT)
    ///
    ///A code to identify the party who is eligible to recover the Value Added Tax (VAT) on the sale of goods or services.
    LD,
    ///Person on claim
    ///
    ///To identify the person who is the subject of the claim.
    LE,
    ///Buyer's corporate office
    ///
    ///The identification of the buyer's corporate office.
    LF,
    ///Supplier's corporate office
    ///
    ///The identification of the supplier's corporate office.
    LG,
    ///Liquidator
    ///
    ///The party responsible for settling or paying a debt.
    LH,
    ///Account coordinator
    ///
    ///An individual with coordination responsibilities for a specific account.
    LI,
    ///Inspection leader
    ///
    ///An individual responsible for an inspection team.
    LJ,
    ///Patient
    ///
    ///A person receiving or registered to receive medical treatment.
    LK,
    ///Patient companion
    ///
    ///Person accompanying the patient.
    LL,
    ///Medical treatment executant
    ///
    ///The party who executes a medical treatment.
    LM,
    ///Lender
    ///
    ///Party lending goods or equipment.
    LN,
    ///Medical treatment prescriber
    ///
    ///The party who prescribes a medical treatment.
    LO,
    ///Loading party
    ///
    ///Party responsible for the loading when other than carrier.
    LP,
    ///Debt payment authorisation party
    ///
    ///A party which authorises the payment of a debt.
    LQ,
    ///Administration centre
    ///
    ///Identification of an administration centre.
    LR,
    ///Product services and repairs centre
    ///
    ///A centre which services and repairs products.
    LS,
    ///Secretariat
    ///
    ///Party is a secretariat.
    LT,
    ///Entry point technical assessment group
    ///
    ///Party acts as an entry point for technical assessment.
    LU,
    ///Party assigning a status
    ///
    ///Party responsible for assigning a status.
    LV,
    /// Party for whom item is ultimately intended
    MA,
    ///Manufacturer of goods
    ///
    ///Party who manufactures the goods.
    MF,
    ///Party designated to execute re-icing
    ///
    ///Party designated to execute re-icing, selected in the official list of mandatories competent for this kind of operation.
    MG,
    /// Planning schedule/material release issuer
    MI,
    /// Manufacturing plant
    MP,
    /// Message recipient
    MR,
    ///Document/message issuer/sender
    ///
    ///Issuer of a document and/or sender of a message.
    MS,
    ///Party designated to execute sanitary procedures
    MT,
    /// Notify party no. 1
    ///
    /// The first party which is to be notified.
    N1,
    ///Notify party no. 2
    ///
    ///The second party which is to be notified.
    N2,
    ///Notify party
    ///
    ///(3180) Party to be notified of arrival of goods.
    NI,
    ///Break bulk berth operator
    ///
    ///Party who offers facilities for berthing of vessels, handling and storage of break bulk cargo.
    OA,
    ///Ordered by
    ///
    ///Party who issued an order.
    OB,
    ///Party data responsible party
    ///
    ///The party responsible for all party data.
    OC,
    ///Equipment repair party
    ///
    ///A party making repairs to equipment.
    OD,
    ///Owner of property
    ///
    ///Party owning a property.
    OE,
    ///On behalf of
    ///
    ///Party on behalf of which an action is executed.
    OF,
    ///Owner or lessor's surveyor
    ///
    ///Surveyor hired by the owner or lessor of the item.
    OG,
    ///Lessee's surveyor
    ///
    ///Surveyor hired by the lessee of the item.
    OH,
    ///Outside inspection agency
    ///
    ///Third party inspecting goods or equipment.
    OI,
    ///Third party
    ///
    ///Another party besides the two principals.
    OJ,
    ///Receiver's sub-entity
    ///
    ///Identifies a sub-entity within the receiver's organization.
    OK,
    ///Case of need party
    ///
    ///Party to be approached in case of difficulty.
    OL,
    ///Collecting bank
    ///
    ///Any bank, other than the remitting bank, involved in processing the collection.
    OM,
    ///Remitting bank
    ///
    ///The bank to which the principal has entrusted the handling of a collection.
    ON,
    ///Order of the shipper party
    ///
    ///The owner of goods under consignment which are moving under a negotiable transport document and will only be released upon receipt of the original transport document.
    OO,
    ///Operator of property or equipment
    ///
    ///The party which operates property or a unit of equipment.
    OP,
    ///Collection principal
    ///
    ///The party entrusting the handling of a collection to a bank.
    OQ,
    ///Ordered bank
    ///
    ///Identifies the account servicer for the ordering customer or payor.
    OR,
    ///Original shipper
    ///
    ///The original supplier of the goods.
    OS,
    ///Outside test agency
    ///
    ///Third party testing goods, equipment or services.
    OT,
    ///Account owner's servicing bank on the sending side
    ///
    ///Identifies the financial institution on the sending side which services the account owner's bank account(s).
    OU,
    ///Owner of means of transport
    ///
    ///(3126) Party owning the means of transport. No synonym of carrier = CA.
    OV,
    ///Account owner's servicing bank on the receiving side
    ///
    ///Identifies the financial institution on the receiving side which services the account owner's bank account(s).
    OW,
    ///Sender's correspondent bank
    ///
    ///The account, or branch of the sender, or another financial institution, through which the sender will reimburse the receiver.
    OX,
    ///Ordering customer
    ///
    ///Identifies the originator of the instruction.
    OY,
    ///Receiver's correspondent bank
    ///
    ///The branch of the receiver, or another financial institution, at which the funds will be made available to the receiver.
    OZ,
    ///Contact party 1
    ///
    ///First party to contact.
    P1,
    ///Contact party 2
    ///
    ///Second party to contact.
    P2,
    ///Contact party 3
    ///
    ///Third party to contact.
    P3,
    ///Contact party 4
    ///
    ///Fourth party to contact.
    P4,
    ///Party to receive inspection report
    ///
    ///Party to whom the inspection report should be sent.
    PA,
    ///Paying financial institution
    ///
    ///Financial institution designated to make payment.
    PB,
    ///Actual purchaser's customer
    ///
    ///Party the purchaser within the actual message is selling the ordered goods or services to.
    PC,
    ///Purchaser's department buyer
    ///
    ///Purchasing department of buyer.
    PD,
    ///Payee
    ///
    ///Identifies the credit party when other than the beneficiary.
    PE,
    ///Party to receive freight bill
    ///
    ///Party to whom the freight bill should be sent.
    PF,
    ///Prime contractor
    ///
    ///Party responsible for the whole project if other than the buyer.
    PG,
    /// Payer's financial institution
    PH,
    ///Payee's company name/ID
    ///
    ///Receiving company name/ID (ACH transfers).
    PI,
    ///Party to receive correspondence
    ///
    ///Second party designated by a first party to receive certain correspondence in lieu of it being mailed directly to this first party.
    PJ,
    ///Contact party
    ///
    ///Party to contact.
    PK,
    ///Payor
    ///
    ///Identifies the debit party when other than the ordering customer (for banking purposes).
    PL,
    ///Party to receive paper memo of invoice
    ///
    ///Party being informed about invoice issue (via paper).
    PM,
    ///Party to receive shipping notice
    ///
    ///The party is to be the recipient of the shipping notice.
    PN,
    ///Ordering party
    ///
    ///To be used only if ordering party and buyer are not identical.
    PO,
    /// Certifying party
    PQ,
    ///Payer
    ///
    ///(3308) Party initiating payment.
    PR,
    /// Payer's company name/ID (Check, Draft or Wire)
    PS,
    /// Party to receive test report
    PT,
    ///Despatch party
    ///
    ///(3282) Party where goods are collected or taken over by the carrier (i.e. if other than consignor).
    PW,
    /// Party to receive all documents
    PX,
    ///Checking party
    ///
    ///Party or contact designated on behalf of carrier or his agent to establish the actual figures for quantities, weight, volume and/or (cube) measurements of goods or containers which are to appear in the transport contract and on which charges will be based.
    PY,
    ///Party to print some document
    ///
    ///The party that is to print a specific document.
    PZ,
    ///Central bank or regulatory authority
    ///
    ///Identifies central bank or regulatory authority which must be informed of certain aspects of a message.
    RA,
    ///Receiving financial institution
    ///
    ///Financial institution designated to receive payment.
    RB,
    ///Party to receive commercial invoice remittance
    ///
    ///Party to whom payment for a commercial invoice or bill should be remitted.
    RE,
    ///Received from
    ///
    ///Name of a person or department which actually delivers the goods.
    RF,
    ///Seller's financial institution
    ///
    ///Financial institution designated by seller to receive payment. RDFI (ACH transfers).
    RH,
    ///Reinsurance intermediary/broker
    ///
    ///Intermediary party between ceding company and reinsurance.
    RI,
    ///Reporting carrier (Customs)
    ///
    ///Party who makes the cargo report to Customs.
    RL,
    ///Reporting carrier's nominated agent/representative (Customs)
    ///
    ///Agent who formally makes a cargo report to Customs on behalf of the carrier.
    RM,
    ///Routing party
    ///
    ///Party responsible for the selection of the carrier(s).
    RP,
    ///Party to receive statement of account
    ///
    ///Party to whom the statement of account should be sent.
    RS,
    ///Receiver of cheque
    ///
    ///Identifies the party which is to receive the actual cheque, when different from the receiver of funds.
    RV,
    ///Issuer of waybill
    ///
    ///Party issuing the contract (waybill) for carriage.
    RW,
    ///Sales responsibility
    ///
    ///Description to be provided.
    SB,
    ///Seller
    ///
    ///(3346) Party selling merchandise to a buyer.
    SE,
    ///Ship from
    ///
    ///Identification of the party from where goods will be or have been shipped.
    SF,
    ///Store group
    ///
    ///Description to be provided.
    SG,
    ///Shipping schedule issuer
    ///
    ///The party which issues a shipping schedule.
    SI,
    /// Plant
    SK,
    /// Store keeper
    SN,
    /// Sold to if different than bill to
    SO,
    ///Seller's agent/representative
    ///
    ///(3254) Party representing the seller for the purpose of the trade transaction.
    SR,
    ///Social securities collector's office
    ///
    ///Party collecting social securities premiums.
    SS,
    ///Ship to
    ///
    ///Identification of the party to where goods will be or have been shipped.
    ST,
    ///Supplier
    ///
    ///Party who supplies goods and/or services.
    SU,
    ///Surety for additions
    ///
    ///Natural of legal person (generally a bank of insurance company) who accepts responsibility in due legal form for the financial guarantee to Customs of the payment of additional duties or fees that become due against a particular shipment, which have not previously been covered by surety.
    SX,
    ///Surety
    ///
    ///Natural or legal person (generally a bank or insurance company) who accepts responsibility in due legal form for the financial consequences of non-fulfillment of another's obligations to the Customs (CCC).
    SY,
    ///Surety for antidumping/countervailing duty
    ///
    ///Natural or legal person that has been contracted by the importer to guarantee to Customs the payment of antidumping and/or countervailing duties that become due against a particular shipment.
    SZ,
    ///Legal receiver
    ///
    ///The party responsible for a receivership.
    TA,
    ///Submitter
    ///
    ///To specify that the party is a submitter.
    TB,
    ///Tax collector's office
    ///
    ///Party collecting taxes.
    TC,
    ///Transit charge payer
    ///
    ///Party, other than the ordering party, which has to pay the charges concerning the transit operations.
    TCP,
    ///Party to receive technical documentation
    ///
    ///Party to whom technical documentation should be sent.
    TD,
    ///Bankruptcy referee
    ///
    ///To specify that the party is a referee in a bankruptcy case.
    TE,
    ///Source of information
    ///
    ///To specify that the party is the source of information.
    TF,
    ///Judge
    ///
    ///To specify that the party is a judge.
    TG,
    ///Attorney
    ///
    ///To specify that the party is an attorney.
    TH,
    ///Law firm
    ///
    ///To specify that the party is a law firm.
    TI,
    ///Trustee
    ///
    ///To specify that the party is a trustee.
    TJ,
    ///Signatory
    ///
    ///To specify that the party is a signatory.
    TK,
    ///Occupant
    ///
    ///The party is an occupant.
    TL,
    ///Co-occupant
    ///
    ///The party is a co-occupant.
    TM,
    ///Subject of inquiry
    ///
    ///The party is the subject of an inquiry.
    TN,
    ///Lessor
    ///
    ///The party is a lessor.
    TO,
    ///Owner of residence
    ///
    ///Identifies the owner of a residence.
    TP,
    ///Founder
    ///
    ///Identifies the founder.
    TQ,
    ///Terminal operator
    ///
    ///A party which handles the loading and unloading of marine vessels.
    TR,
    ///Party to receive certified test results
    ///
    ///Party to whom the certified test results should be sent.
    TS,
    ///Transfer to
    ///
    ///The party which is the recipient of a transfer.
    TT,
    ///President
    ///
    ///Identifies the president.
    TU,
    ///Chairperson
    ///
    ///Identifies the chairperson.
    TV,
    ///Legal title holder
    ///
    ///Identifies the legal title holder.
    TW,
    ///Shareholder
    ///
    ///Identifies a shareholder.
    TX,
    ///Provider
    ///
    ///Identifies the provider.
    TY,
    ///Military branch
    ///
    ///Identifies the branch of the military.
    TZ,
    ///Educational institution
    ///
    ///Identifies a university, college or school.
    UA,
    ///Assignor
    ///
    ///Identifies the assignor.
    UB,
    ///Ultimate consignee
    ///
    ///Party who has been designated on the invoice or packing list as the final recipient of the stated merchandise.
    UC,
    ///Ultimate customer
    ///
    ///The final recipient of goods.
    UD,
    ///Advisor
    ///
    ///Identifies the advisor.
    UE,
    ///Co-defendant
    ///
    ///Identifies the co-defendant.
    UF,
    ///Merged company with retained identity
    ///
    ///Company whose identity has been retained from a merger.
    UG,
    ///Party represented
    ///
    ///Identifies the party represented.
    UH,
    ///Unexpected handling party
    ///
    ///Party authorized (during a voyage) to apply unexpected handling procedures or party having applied these procedures.
    UHP,
    ///Assignee
    ///
    ///Identifies the assignee.
    UI,
    ///Key person
    ///
    ///Identifies the key person.
    UJ,
    ///Author
    ///
    ///Identifies the author.
    UK,
    ///Ultimate parent company
    ///
    ///Identifies the ultimate parent company.
    UL,
    ///Party not to be confused with
    ///
    ///Identifies a party not to be confused with another party.
    UM,
    ///Accountant
    ///
    ///Identifies the accountant.
    UN,
    ///Plaintiff
    ///
    ///Identifies the plaintiff.
    UO,
    ///Unloading party
    ///
    ///Description to be provided.
    UP,
    ///Parent company
    ///
    ///Identifies the parent company.
    UQ,
    ///Affiliated company
    ///
    ///Identifies the affiliated company.
    UR,
    ///Bailiff
    ///
    ///Identifies the bailiff.
    US,
    ///Merged company
    ///
    ///Identifies the company involved in a merger.
    UT,
    ///Defendant
    ///
    ///Identifies the defendant.
    UU,
    ///Petitioning creditor
    ///
    ///Identifies the petitioning creditor.
    UV,
    ///Guarantee agency
    ///
    ///Identifies the guarantee agency.
    UW,
    ///Organization group
    ///
    ///Identifies the organization group.
    UX,
    ///Subsidiary
    ///
    ///Identifies the subsidiary.
    UY,
    ///Industry association
    ///
    ///Identifies the industry association.
    UZ,
    ///Joint owner
    ///
    ///Identifies the joint owner.
    VA,
    ///Joint venture
    ///
    ///Identifies the joint venture.
    VB,
    ///Filing office
    ///
    ///Identifies the filing office.
    VC,
    ///Court
    ///
    ///Identifies the court.
    VE,
    ///Liability holder
    ///
    ///Identifies the liability holder.
    VF,
    ///Local government sponsor
    ///
    ///Identifies the local government sponsor.
    VG,
    ///Mortgage company
    ///
    ///Identifies the mortgage company.
    VH,
    ///Notary public
    ///
    ///Identifies the notary public.
    VI,
    ///Officer
    ///
    ///Identifies the officer.
    VJ,
    ///Publisher
    ///
    ///Identifies the publisher.
    VK,
    ///Party manufactured for
    ///
    ///Identifies the party for whom manufacturing of goods is done.
    VL,
    ///Previous owner
    ///
    ///Identifies the previous owner.
    VM,
    ///Vendor
    ///
    ///Party vending goods or services.
    VN,
    ///Purchased company
    ///
    ///Identifies the purchased company.
    VO,
    ///Receiver manager
    ///
    ///Manager of a business which is in receivership status and which will not be liquidated.
    VP,
    ///Responsible government agency
    ///
    ///Identifies the responsible government agency.
    VQ,
    ///Sole proprietor
    ///
    ///Identifies the sole proprietor.
    VR,
    ///Auctioneer
    ///
    ///Identifies the auctioneer.
    VS,
    ///Branch
    ///
    ///Identifies the branch.
    VT,
    ///Business
    ///
    ///Identifies the business.
    VU,
    ///Ultimate same country parent company
    ///
    ///Identifies the highest level parent company in the same country.
    VV,
    ///Responsible party
    ///
    ///Identifies the party that can be called to account.
    VW,
    ///Secured party
    ///
    ///Identifies a party that is guaranteed against loss.
    VX,
    ///Other related party
    ///
    ///Identifies an entity as an unspecified but related party.
    VY,
    ///Co-debtor
    ///
    ///Identifies an entity as a joint or mutual debtor.
    VZ,
    ///Company which holds financial interest
    ///
    ///Identifies a company which holds any financial stake in an undertaking or organization.
    WA,
    ///Rating organization
    ///
    ///Identifies an organization responsible for assigning a classification or rating.
    WB,
    ///Information reference agency
    ///
    ///The agency responsible for the reference of information.
    WC,
    ///Warehouse depositor
    ///
    ///(3004) Party depositing goods in a warehouse.
    WD,
    ///Compilation agency
    ///
    ///The agency responsible for the compilation of information.
    WE,
    ///Information maintenance agency
    ///
    ///The agency responsible for the maintenance of information.
    WF,
    ///Information dissemination agency
    ///
    ///The agency responsible for the dissemination of information.
    WG,
    ///Warehouse keeper
    ///
    ///(3022) Party taking responsibility for goods entered into a warehouse.
    WH,
    ///Inspection address
    ///
    ///Specifies the address for an inspection.
    WI,
    ///Refusal party
    ///
    ///Identification of the party responsible for a refusal.
    WJ,
    ///Value added network provider
    ///
    ///A party that provides telecommunications interconnectivity services in an electronic data interchange environment.
    WK,
    ///Agency
    ///
    ///The business or establishment of an agent.
    WL,
    /// Works manager
    WM,
    ///Party to receive order to supply
    ///
    ///Party designated by the registering party to receive a binding direction to supply something.
    WN,
    ///Party to receive invitation to offer
    ///
    ///An entity to receive an invitation to offer.
    WO,
    ///Sub-entity
    ///
    ///A part into which an entity has been divided.
    WP,
    ///Weighting party
    ///
    ///Party designated (legally accepted) to ascertain the weight.
    WPA,
    ///Doing business as
    ///
    ///The name under which business is conducted.
    WQ,
    ///Party submitting quote
    ///
    ///The party stating the price of something to be purchased.
    WR,
    ///Wholesaler
    ///
    ///Seller of articles, often in large quantities, to be retailed by others.
    WS,
    ///Affiliated party
    ///
    ///A party attached or connected to another party.
    WT,
    ///Previous name
    ///
    ///Name of an entity used before the current name.
    WU,
    ///Party performing task
    ///
    ///An entity responsible for performing a task to be undertaken.
    WV,
    ///Registering party
    ///
    ///Party performing the registration.
    WW,
    ///No heading
    ///
    ///Description to be provided.
    XX,
    ///Mutually defined
    ///
    ///Party specification mutually agreed between interchanging parties.
    ZZZ,
}

/// Code list responsible agency code
///
/// Code specifying the agency responsible for a code list.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _3055 {
    /// CCC (Customs Co-operation Council)
    ///
    #[strum(serialize = "1")]
    _1,
    /// CEC (Commission of the European Communities)
    ///
    /// Generic: see also 140, 141, 142, 162.
    #[strum(serialize = "2")]
    _2,
    /// IATA (International Air Transport Association)
    ///
    /// The airline industry's international organisation.
    #[strum(serialize = "3")]
    _3,
    /// ICC (International Chamber of Commerce)
    ///
    #[strum(serialize = "4")]
    _4,
    /// ISO (International Organization for Standardization)
    ///
    #[strum(serialize = "5")]
    _5,
    /// UN/ECE (United Nations - Economic Commission for Europe)
    ///
    #[strum(serialize = "6")]
    _6,
    /// CEFIC (Conseil Europeen des Federations de l'Industrie Chimique)
    ///
    /// EDI project for chemical industry.
    #[strum(serialize = "7")]
    _7,
    /// EDIFICE
    ///
    /// Standardised electronic commerce forum for companies with interests in computing, electronics and telecommunications.
    #[strum(serialize = "8")]
    _8,
    /// EAN (International Article Numbering association)
    ///
    #[strum(serialize = "9")]
    _9,
    /// ODETTE
    ///
    /// Organization for Data Exchange through Tele-Transmission in Europe (European automotive industry project).
    #[strum(serialize = "10")]
    _10,
    /// Lloyd's register of shipping
    ///
    /// A register of ocean going vessels maintained by Lloyd's of London.
    #[strum(serialize = "11")]
    _11,
    /// UIC (International union of railways)
    ///
    /// International Union of Railways.
    #[strum(serialize = "12")]
    _12,
    /// ICAO (International Civil Aviation Organisation)
    ///
    #[strum(serialize = "13")]
    _13,
    /// ICS (International Chamber of Shipping)
    ///
    #[strum(serialize = "14")]
    _14,
    /// RINET (Reinsurance and Insurance Network)
    ///
    #[strum(serialize = "15")]
    _15,
    /// US, D&B (Dun & Bradstreet Corporation)
    ///
    /// Identifies the Dun & Bradstreet Corporation, United States.
    #[strum(serialize = "16")]
    _16,
    /// S.W.I.F.T.
    ///
    /// Society for Worldwide Interbank Financial Telecommunications s.c.
    #[strum(serialize = "17")]
    _17,
    /// Conventions on SAD and transit (EC and EFTA)
    ///
    /// SAD = Single Administrative Document.
    #[strum(serialize = "18")]
    _18,
    /// FRRC (Federal Reserve Routing Code)
    ///
    #[strum(serialize = "19")]
    _19,
    /// BIC (Bureau International des Containeurs)
    ///
    /// The container industry's international organisation responsible for the issuance of container-related codes.
    #[strum(serialize = "20")]
    _20,
    /// Assigned by transport company
    ///
    /// Codes assigned by a transport company.
    #[strum(serialize = "21")]
    _21,
    /// US, ISA (Information Systems Agreement)
    ///
    /// Codes assigned by the ISA for use by its members.
    #[strum(serialize = "22")]
    _22,
    /// FR, EDITRANSPORT
    ///
    /// French association developing EDI in transport logistics.
    #[strum(serialize = "23")]
    _23,
    /// AU, ROA (Railways of Australia)
    ///
    /// Maintains code lists which are accepted by Australian government railways.
    #[strum(serialize = "24")]
    _24,
    /// EDITEX (Europe)
    ///
    /// EDI group for the textile and clothing industry.
    #[strum(serialize = "25")]
    _25,
    /// NL, Foundation Uniform Transport Code
    ///
    /// Foundation Uniform Transport Code is the EDI organisation for shippers, carriers and other logistic service providers in the Netherlands.
    #[strum(serialize = "26")]
    _26,
    /// US, FDA (Food and Drug Administration)
    ///
    /// U.S. food and drug administration.
    #[strum(serialize = "27")]
    _27,
    /// EDITEUR (European book sector electronic data interchange group)
    ///
    /// Code identifying the pan European user group for the book industry as an organisation responsible for code values in the book industry.
    #[strum(serialize = "28")]
    _28,
    /// GB, FLEETNET
    ///
    /// Association of fleet vehicle hiring and leasing companies in the UK.
    #[strum(serialize = "29")]
    _29,
    /// GB, ABTA (Association of British Travel Agencies)
    ///
    /// ABTA, Association of British Travel Agencies.
    #[strum(serialize = "30")]
    _30,
    /// FI, Finish State Railway
    ///
    /// Finish State Railway.
    #[strum(serialize = "31")]
    _31,
    /// PL, Polish State Railway
    ///
    /// Polish State Railway.
    #[strum(serialize = "32")]
    _32,
    /// BG, Bulgaria State Railway
    ///
    /// Bulgaria State Railway.
    #[strum(serialize = "33")]
    _33,
    /// RO, Rumanian State Railway
    ///
    /// Rumanian State Railway.
    #[strum(serialize = "34")]
    _34,
    /// CZ, Tchechian State Railway
    ///
    /// Tchechian State Railway.
    #[strum(serialize = "35")]
    _35,
    /// HU, Hungarian State Railway
    ///
    /// Hungarian State Railway.
    #[strum(serialize = "36")]
    _36,
    /// GB, British Railways
    ///
    /// British Railways.
    #[strum(serialize = "37")]
    _37,
    /// ES, Spanish National Railway
    ///
    /// Spanish National Railway.
    #[strum(serialize = "38")]
    _38,
    /// SE, Swedish State Railway
    ///
    /// Swedish State Railway.
    #[strum(serialize = "39")]
    _39,
    /// NO, Norwegian State Railway
    ///
    /// Norwegian State Railway.
    #[strum(serialize = "40")]
    _40,
    /// DE, German Railway
    ///
    /// German Railway.
    #[strum(serialize = "41")]
    _41,
    /// AT, Austrian Federal Railways
    ///
    /// Austrian Federal Railways.
    #[strum(serialize = "42")]
    _42,
    /// LU, Luxembourg National Railway Company
    ///
    /// Luxembourg National Railway Company.
    #[strum(serialize = "43")]
    _43,
    /// IT, Italian State Railways
    ///
    /// Italian State Railways.
    #[strum(serialize = "44")]
    _44,
    /// NL, Netherlands Railways
    ///
    /// Netherlands Railways.
    #[strum(serialize = "45")]
    _45,
    /// CH, Swiss Federal Railways
    ///
    /// Swiss Federal Railways.
    #[strum(serialize = "46")]
    _46,
    /// DK, Danish State Railways
    ///
    /// Danish State Railways.
    #[strum(serialize = "47")]
    _47,
    /// FR, French National Railway Company
    ///
    /// French National Railway Company.
    #[strum(serialize = "48")]
    _48,
    /// BE, Belgian National Railway Company
    ///
    /// Belgian National Railway Company.
    #[strum(serialize = "49")]
    _49,
    /// PT, Portuguese Railways
    ///
    /// Portuguese Railways.
    #[strum(serialize = "50")]
    _50,
    /// SK, Slovakian State Railways
    ///
    /// Slovakian State Railways.
    #[strum(serialize = "51")]
    _51,
    /// IE, Irish Transport Company
    ///
    /// Irish Transport Company.
    #[strum(serialize = "52")]
    _52,
    /// FIATA (International Federation of Freight Forwarders Associations)
    ///
    /// International Federation of Freight Forwarders Associations.
    #[strum(serialize = "53")]
    _53,
    /// IMO (International Maritime Organisation)
    ///
    /// International Maritime Organisation.
    #[strum(serialize = "54")]
    _54,
    /// US, DOT (United States Department of Transportation)
    ///
    /// United States Department of Transportation.
    #[strum(serialize = "55")]
    _55,
    /// TW, Trade-van
    ///
    /// Trade-van is an EDI/VAN service centre for customs, transport, and insurance in national and international trade.
    #[strum(serialize = "56")]
    _56,
    /// TW, Chinese Taipei Customs
    ///
    /// Customs authorities of Chinese Taipei responsible for collecting import duties and preventing smuggling.
    #[strum(serialize = "57")]
    _57,
    /// EUROFER
    ///
    /// European steel organisation - EDI project for the European steel industry.
    #[strum(serialize = "58")]
    _58,
    /// DE, EDIBAU
    ///
    /// National body responsible for the German codification in the construction area.
    #[strum(serialize = "59")]
    _59,
    /// Assigned by national trade agency
    ///
    /// The code list is from a national agency.
    #[strum(serialize = "60")]
    _60,
    /// Association Europeenne des Constructeurs de Materiel Aerospatial (AECMA)
    ///
    /// A code to identify the Association Europeenne des Constructeurs de Materiel Aeropsatial (European Association of Aerospace Products Manufacturers) as an authorizing agency for code lists.
    #[strum(serialize = "61")]
    _61,
    /// US, DIstilled Spirits Council of the United States (DISCUS)
    ///
    /// United States DIstilled Spirits Council of the United States (DISCUS).
    #[strum(serialize = "62")]
    _62,
    /// North Atlantic Treaty Organization (NATO)
    ///
    /// A code to identify the North Atlantic Treaty Organization (NATO) as an authorizing agency for code lists.
    #[strum(serialize = "63")]
    _63,
    /// FR, EDIFRANCE
    ///
    /// French association responsible for coordination and promotion of EDI application in France.
    #[strum(serialize = "64")]
    _64,
    /// FR, GENCOD
    ///
    /// French organization responsible for EDI and Barcoding application in the retail sector.
    #[strum(serialize = "65")]
    _65,
    /// MY, Malaysian Customs and Excise
    ///
    /// Malaysia Royal Customs and Excise.
    #[strum(serialize = "66")]
    _66,
    /// MY, Malaysia Central Bank
    ///
    /// Malaysia Central Bank is a regulatory body set up by the government to charge with promoting economic monetary and credit condition favourable to commercial and industrial activity.
    #[strum(serialize = "67")]
    _67,
    /// US, Bureau of Alcohol, Tobacco and Firearms (BATF)
    ///
    /// United States Bureau of Alcohol, Tobacco and Firearms (BATF).
    #[strum(serialize = "68")]
    _68,
    /// US, National Alcohol Beverage Control Association (NABCA)
    ///
    /// United States National Alcohol Beverage Control Association (NABCA).
    #[strum(serialize = "69")]
    _69,
    /// MY, Dagang.Net
    ///
    /// Malaysia, Dagang.Net is a national clearing house which provide EDI/VAN service for customs, transport, retail and financial and other industries in the national and international trade.
    #[strum(serialize = "70")]
    _70,
    /// US, FCC (Federal Communications Commission)
    ///
    /// A code representing the United States Federal Communication Commission (FCC).
    #[strum(serialize = "71")]
    _71,
    /// US, MARAD (Maritime Administration)
    ///
    /// A code representing the United States Maritime Administration (MARAD) under the Department of Transportation (DOT).
    #[strum(serialize = "72")]
    _72,
    /// US, DSAA (Defense Security Assistance Agency)
    ///
    /// A code representing the United States Defense Security Assistance Agency (DSAA) under the Department of Defense (DOD).
    #[strum(serialize = "73")]
    _73,
    /// US, NRC (Nuclear Regulatory Commission)
    ///
    /// A code representing the United States Nuclear Regulatory Commission (NRC).
    #[strum(serialize = "74")]
    _74,
    /// US, ODTC (Office of Defense Trade Controls)
    ///
    /// A code representing the United States Office of Defense Trade Controls (ODTC) under the Department of State.
    #[strum(serialize = "75")]
    _75,
    /// US, ATF (Bureau of Alcohol, Tobacco and Firearms)
    ///
    /// A code representing the United States Bureau of Alcohol, Tobacco and Firearms, Department of Treasury (ATF).
    #[strum(serialize = "76")]
    _76,
    /// US, BXA (Bureau of Export Administration)
    ///
    /// A code representing the United States Bureau of Export Administration (BXA) under the Department of Commerce (DOC) .
    #[strum(serialize = "77")]
    _77,
    /// US, FWS (Fish and Wildlife Service)
    ///
    /// A code depicting the United States Fish and Wildlife Service (FWS).
    #[strum(serialize = "78")]
    _78,
    /// US, OFAC (Office of Foreign Assets Control)
    ///
    /// A code representing the United States Office of Foreign Assets Controls (OFAC).
    #[strum(serialize = "79")]
    _79,
    /// BRMA/RAA - LIMNET - RINET Joint Venture
    ///
    /// Joint venture between BRMA (Brokers & Reinsurance Markets Association) / RAA (Reinsurance Association of America) - LIMNET (London Insurance Market Network) - RINET (Reinsurance and Insurance Network).
    #[strum(serialize = "80")]
    _80,
    /// RU, (SFT) Society for Financial Telecommunications
    ///
    /// Russian company representing the users of the Global Financial Telecommunication Network (GFTN).
    #[strum(serialize = "81")]
    _81,
    /// NO, Enhetsregisteret ved Bronnoysundregisterne
    ///
    /// The co-ordinating register for companies and business units of companies at the Bronnoysund register centre.
    #[strum(serialize = "82")]
    _82,
    /// US, National Retail Federation
    ///
    /// The National Retail Federation is the trade association for the general merchandise retailing industry. In addition to providing support and education services, they also maintain and publish standard colour and size codes for the retail industry.
    #[strum(serialize = "83")]
    _83,
    /// DE, BRD (Gesetzgeber der Bundesrepublik Deutschland)
    ///
    /// German legislature.
    #[strum(serialize = "84")]
    _84,
    /// North America, Telecommunications Industry Forum
    ///
    /// Trade association representing telecommunications service providers, equipment manufacturers, suppliers to the industry and customers.
    #[strum(serialize = "85")]
    _85,
    /// Assigned by party originating the message
    ///
    /// Codes assigned by the party originating the message.
    #[strum(serialize = "86")]
    _86,
    /// Assigned by carrier
    ///
    /// Codes assigned by the carrier.
    #[strum(serialize = "87")]
    _87,
    /// Assigned by owner of operation
    ///
    /// Assigned by owner of operation (e.g. used in construction).
    #[strum(serialize = "88")]
    _88,
    /// Assigned by distributor
    ///
    #[strum(serialize = "89")]
    _89,
    /// Assigned by manufacturer
    ///
    #[strum(serialize = "90")]
    _90,
    /// Assigned by seller or seller's agent
    ///
    #[strum(serialize = "91")]
    _91,
    /// Assigned by buyer or buyer's agent
    ///
    #[strum(serialize = "92")]
    _92,
    /// AT, Austrian Customs
    ///
    #[strum(serialize = "93")]
    _93,
    /// AT, Austrian PTT
    ///
    #[strum(serialize = "94")]
    _94,
    /// AU, Australian Customs Service
    ///
    /// Australian Customs Service.
    #[strum(serialize = "95")]
    _95,
    /// CA, Revenue Canada, Customs and Excise
    ///
    #[strum(serialize = "96")]
    _96,
    /// CH, Administration federale des contributions
    ///
    /// Indirect taxation (e.g. turn-over/sales taxes).
    #[strum(serialize = "97")]
    _97,
    /// CH, Direction generale des douanes
    ///
    /// Customs (incl. ISO alpha 2 country code).
    #[strum(serialize = "98")]
    _98,
    /// CH, Division des importations et exportations, OFAEE
    ///
    /// Import and export licences.
    #[strum(serialize = "99")]
    _99,
    /// CH, Entreprise des PTT
    ///
    /// Telephone (voice/data) + telex numbers, postcodes, postal account numbers.
    #[strum(serialize = "100")]
    _100,
    /// CH, Carbura
    ///
    /// Centrale suisse pour l'importation de carburants et combustibles liquides (Oil products).
    #[strum(serialize = "101")]
    _101,
    /// CH, Centrale suisse pour l'importation du charbon
    ///
    /// Coal.
    #[strum(serialize = "102")]
    _102,
    /// CH, Office fiduciaire des importateurs de denrees alimentaires
    ///
    /// Foodstuff.
    #[strum(serialize = "103")]
    _103,
    /// CH, Association suisse code des articles
    ///
    /// Swiss article numbering association.
    #[strum(serialize = "104")]
    _104,
    /// DK, Ministry of taxation, Central Customs and Tax Administration
    ///
    /// Danish Customs administration.
    #[strum(serialize = "105")]
    _105,
    /// FR, Direction generale des douanes et droits indirects
    ///
    /// French Customs.
    #[strum(serialize = "106")]
    _106,
    /// FR, INSEE
    ///
    /// Institut National de la Statistique et des Etudes Economiques.
    #[strum(serialize = "107")]
    _107,
    /// FR, Banque de France
    ///
    #[strum(serialize = "108")]
    _108,
    /// GB, H.M. Customs & Excise
    ///
    #[strum(serialize = "109")]
    _109,
    /// IE, Revenue Commissioners, Customs AEP project
    ///
    #[strum(serialize = "110")]
    _110,
    /// US, U.S. Customs Service
    ///
    #[strum(serialize = "111")]
    _111,
    /// US, U.S. Census Bureau
    ///
    /// The Bureau of the Census of the U.S. Dept. of Commerce.
    #[strum(serialize = "112")]
    _112,
    /// US, UCC (Uniform Code Council)
    ///
    /// The Uniform Code Council (UCC) is a not-for-profit organization which manages and administers EDI and product bar code standards for the U.S. retail industry. The UCC also maintains U.P.C. manufacturer identifiers, EDI communications identifiers and various EDI code lists specific to retailing. The UCC is located in Dayton, OH, USA.
    #[strum(serialize = "113")]
    _113,
    /// US, ABA (American Bankers Association)
    ///
    #[strum(serialize = "114")]
    _114,
    /// US, DODAAC (Department Of Defense Active Agency Code)
    ///
    #[strum(serialize = "115")]
    _115,
    /// US, ANSI ASC X12
    ///
    /// American National Standards Institute ASC X12.
    #[strum(serialize = "116")]
    _116,
    /// AT, Geldausgabeautomaten-Service Gesellschaft m.b.H.
    ///
    /// Description to be provided.
    #[strum(serialize = "117")]
    _117,
    /// SE, Svenska Bankfoereningen
    ///
    /// Swedish bankers association.
    #[strum(serialize = "118")]
    _118,
    /// IT, Associazione Bancaria Italiana
    ///
    #[strum(serialize = "119")]
    _119,
    /// IT, Socieata' Interbancaria per l'Automazione
    ///
    #[strum(serialize = "120")]
    _120,
    /// CH, Telekurs AG
    ///
    #[strum(serialize = "121")]
    _121,
    /// CH, Swiss Securities Clearing Corporation
    ///
    #[strum(serialize = "122")]
    _122,
    /// NO, Norwegian Interbank Research Organization
    ///
    #[strum(serialize = "123")]
    _123,
    /// NO, Norwegian Bankers' Association
    ///
    #[strum(serialize = "124")]
    _124,
    /// FI, The Finnish Bankers' Association
    ///
    #[strum(serialize = "125")]
    _125,
    /// US, NCCMA (Account Analysis Codes)
    ///
    #[strum(serialize = "126")]
    _126,
    /// DE, ARE (AbRechnungs Einheit)
    ///
    /// A German code for subsidiary unit number.
    #[strum(serialize = "127")]
    _127,
    /// BE, Belgian Bankers' Association
    ///
    #[strum(serialize = "128")]
    _128,
    /// BE, Belgian Ministry of Finance
    ///
    /// VAT numbers.
    #[strum(serialize = "129")]
    _129,
    /// DK, PBS (Pengainstitutternes Betalings Service)
    ///
    #[strum(serialize = "130")]
    _130,
    /// DE, German Bankers Association
    ///
    #[strum(serialize = "131")]
    _131,
    /// GB, BACS Limited
    ///
    #[strum(serialize = "132")]
    _132,
    /// GB, Association for Payment Clearing Services
    ///
    #[strum(serialize = "133")]
    _133,
    /// GB, CHAPS and Town Clearing Company Limited
    ///
    #[strum(serialize = "134")]
    _134,
    /// GB, The Clearing House
    ///
    #[strum(serialize = "135")]
    _135,
    /// GB, Article Number Association (UK) Limited
    ///
    /// EAN bar-coding.
    #[strum(serialize = "136")]
    _136,
    /// AT, Verband oesterreichischer Banken und Bankiers
    ///
    /// Austrian bankers association.
    #[strum(serialize = "137")]
    _137,
    /// FR, CFONB (Comite francais d'organ. et de normalisation bancaires)
    ///
    /// National body responsible for the French codification in banking activity.
    #[strum(serialize = "138")]
    _138,
    /// UPU (Universal Postal Union)
    ///
    /// (a..3 country code).
    #[strum(serialize = "139")]
    _139,
    /// CEC (Commission of the European Communities), DG/XXI-01
    ///
    /// (Computerization within Customs area).
    #[strum(serialize = "140")]
    _140,
    /// CEC (Commission of the European Communities), DG/XXI-B-1
    ///
    /// Description to be provided.
    #[strum(serialize = "141")]
    _141,
    /// CEC (Commission of the European Communities), DG/XXXIV
    ///
    /// Statistical Office of the European Communities: e.g. Geonomenclature.
    #[strum(serialize = "142")]
    _142,
    /// NZ, New Zealand Customs
    ///
    #[strum(serialize = "143")]
    _143,
    /// NL, Netherlands Customs
    ///
    #[strum(serialize = "144")]
    _144,
    /// SE, Swedish Customs
    ///
    #[strum(serialize = "145")]
    _145,
    /// DE, German Customs
    ///
    #[strum(serialize = "146")]
    _146,
    /// BE, Belgian Customs
    ///
    #[strum(serialize = "147")]
    _147,
    /// ES, Spanish Customs
    ///
    #[strum(serialize = "148")]
    _148,
    /// IL, Israel Customs
    ///
    #[strum(serialize = "149")]
    _149,
    /// HK, Hong Kong Customs
    ///
    #[strum(serialize = "150")]
    _150,
    /// JP, Japan Customs
    ///
    #[strum(serialize = "151")]
    _151,
    /// SA, Saudi Arabia Customs
    ///
    #[strum(serialize = "152")]
    _152,
    /// IT, Italian Customs
    ///
    #[strum(serialize = "153")]
    _153,
    /// GR, Greek Customs
    ///
    #[strum(serialize = "154")]
    _154,
    /// PT, Portuguese Customs
    ///
    #[strum(serialize = "155")]
    _155,
    /// LU, Luxembourg Customs
    ///
    #[strum(serialize = "156")]
    _156,
    /// NO, Norwegian Customs
    ///
    #[strum(serialize = "157")]
    _157,
    /// FI, Finnish Customs
    ///
    #[strum(serialize = "158")]
    _158,
    /// IS, Iceland Customs
    ///
    #[strum(serialize = "159")]
    _159,
    /// LI, Liechtenstein authority
    ///
    /// (Identification of relevant responsible agency for e.g. banking/financial matters still pending. For e.g. Customs, currency, post/telephone: see relevant CH entry).
    #[strum(serialize = "160")]
    _160,
    /// UNCTAD (United Nations - Conference on Trade And Development)
    ///
    #[strum(serialize = "161")]
    _161,
    /// CEC (Commission of the European Communities), DG/XIII-D-5
    ///
    /// (TEDIS - incl. CEBIS -, INSIS and CADDIA projects).
    #[strum(serialize = "162")]
    _162,
    /// US, FMC (Federal Maritime Commission)
    ///
    #[strum(serialize = "163")]
    _163,
    /// US, DEA (Drug Enforcement Agency)
    ///
    #[strum(serialize = "164")]
    _164,
    /// US, DCI (Distribution Codes, INC.)
    ///
    #[strum(serialize = "165")]
    _165,
    /// US, National Motor Freight Classification Association
    ///
    /// The organisation in the USA which is responsible for code maintenance in the trucking industry.
    #[strum(serialize = "166")]
    _166,
    /// US, AIAG (Automotive Industry Action Group)
    ///
    #[strum(serialize = "167")]
    _167,
    /// US, FIPS (Federal Information Publishing Standard)
    ///
    #[strum(serialize = "168")]
    _168,
    /// CA, SCC (Standards Council of Canada)
    ///
    #[strum(serialize = "169")]
    _169,
    /// CA, CPA (Canadian Payment Association)
    ///
    #[strum(serialize = "170")]
    _170,
    /// NL, Interpay Girale Services
    ///
    /// Interpay Girale Services.
    #[strum(serialize = "171")]
    _171,
    /// NL, Interpay Debit Card Services
    ///
    /// Interpay Debit Card Services.
    #[strum(serialize = "172")]
    _172,
    /// NO, NORPRO
    ///
    #[strum(serialize = "173")]
    _173,
    /// DE, DIN (Deutsches Institut fuer Normung)
    ///
    /// German standardization institute.
    #[strum(serialize = "174")]
    _174,
    /// FCI (Factors Chain International)
    ///
    #[strum(serialize = "175")]
    _175,
    /// BR, Banco Central do Brazil
    ///
    /// Self-explanatory.
    #[strum(serialize = "176")]
    _176,
    /// AU, LIFA (Life Insurance Federation of Australia)
    ///
    /// Life Insurance Federation of Australia.
    #[strum(serialize = "177")]
    _177,
    /// AU, SAA (Standards Association of Australia)
    ///
    /// Standards Association of Australia.
    #[strum(serialize = "178")]
    _178,
    /// US, Air transport association of America
    ///
    /// U.S. -based trade association representing the major North American scheduled airlines.
    #[strum(serialize = "179")]
    _179,
    /// DE, BIA (Berufsgenossenschaftliches Institut fuer Arbeitssicherheit)
    ///
    /// German institute of the workmen's compensation board.
    #[strum(serialize = "180")]
    _180,
    /// Edibuild
    ///
    /// EDI organization for companies in the construction industry.
    #[strum(serialize = "181")]
    _181,
    /// US, Standard Carrier Alpha Code (Motor)
    ///
    /// Organisation maintaining the SCAC lists and transportation operating in North America.
    #[strum(serialize = "182")]
    _182,
    /// US, American Petroleum Institute
    ///
    /// US-based trade association representing oil and natural gas producers, shippers, refineries, marketers, and major suppliers to the industry.
    #[strum(serialize = "183")]
    _183,
    /// AU, ACOS (Australian Chamber of Shipping)
    ///
    /// The national organisation for the maritime industry in Australia.
    #[strum(serialize = "184")]
    _184,
    /// DE, BDI (Bundesverband der Deutschen Industrie e.V.)
    ///
    /// German industry association.
    #[strum(serialize = "185")]
    _185,
    /// US, GSA (General Services Administration)
    ///
    /// The US General Services Administration.
    #[strum(serialize = "186")]
    _186,
    /// US, DLMSO (Defense Logistics Management Standards Office)
    ///
    /// The Defense Logistics Management Standards Office.
    #[strum(serialize = "187")]
    _187,
    /// US, NIST (National Institute of Standards and Technology)
    ///
    /// The US National Institute of Standards and Technology.
    #[strum(serialize = "188")]
    _188,
    /// US, DoD (Department of Defense)
    ///
    /// The US Department of Defense.
    #[strum(serialize = "189")]
    _189,
    /// US, VA (Department of Veterans Affairs)
    ///
    /// The Department of Veterans Affairs.
    #[strum(serialize = "190")]
    _190,
    /// IAPSO (United Nations Inter-Agency Procurement Services Office)
    ///
    /// United Nations organization responsible for maintaining the United Nations Common Coding System (UNCCS) which is used extensively by UN agencies in procurement and statistical analysis.
    #[strum(serialize = "191")]
    _191,
    /// Shipper's association
    ///
    /// Code assigned by a shipper's association.
    #[strum(serialize = "192")]
    _192,
    /// EU, European Telecommunications Informatics Services (ETIS)
    ///
    /// European Telecommunications Informatics Services is a non-profit cooperative organisation owned by European public network operators, working in the field of information technology.
    #[strum(serialize = "193")]
    _193,
    /// AU, AQIS (Australian Quarantine and Inspection Service)
    ///
    /// Australian Quarantine and Inspection Service.
    #[strum(serialize = "194")]
    _194,
    /// CO, DIAN (Direccion de Impuestos y Aduanas Nacionales)
    ///
    /// The Colombian customs organization.
    #[strum(serialize = "195")]
    _195,
    /// US, COPAS (Council of Petroleum Accounting Society)
    ///
    /// Organization supplying codes of oil field equipment and tubular goods used by joint operators in the petroleum industry.
    #[strum(serialize = "196")]
    _196,
    /// US, DISA (Data Interchange Standards Association)
    ///
    /// The organization maintaining code lists under the administration of the data interchange standards association.
    #[strum(serialize = "197")]
    _197,
    /// CO, Superintendencia Bancaria De Colombia
    ///
    /// The organization which assigns identification numbers to financial institutions conducting business in Colombia.
    #[strum(serialize = "198")]
    _198,
    /// FR, Direction de la Comptabilite Publique
    ///
    /// The French public accounting office.
    #[strum(serialize = "199")]
    _199,
    /// NL, EAN Netherlands
    ///
    /// Netherlands based European Article Numbering association (EAN).
    #[strum(serialize = "200")]
    _200,
    /// US, WSSA(Wine and Spirits Shippers Association)
    ///
    /// United States based Wine and Spirits Shippers association.
    #[strum(serialize = "201")]
    _201,
    /// PT, Banco de Portugal
    ///
    /// Portuguese Central Bank.
    #[strum(serialize = "202")]
    _202,
    /// FR, GALIA (Groupement pour l'Amelioration des Liaisons dans l'Industrie Automobile)
    ///
    /// The national organisation representing France in ODETTE (Organisation for Data Exchanges through Tele- Transmission in Europe).
    #[strum(serialize = "203")]
    _203,
    /// DE, VDA (Verband der Automobilindustrie E.V.)
    ///
    /// The national organisation representing Germany in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[strum(serialize = "204")]
    _204,
    /// IT, ODETTE Italy
    ///
    /// The national organisation representing Italy in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[strum(serialize = "205")]
    _205,
    /// NL, ODETTE Netherlands
    ///
    /// The national organisation representing Netherlands in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[strum(serialize = "206")]
    _206,
    /// ES, ODETTE Spain
    ///
    /// The national organisation representing Spain in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[strum(serialize = "207")]
    _207,
    /// SE, ODETTE Sweden
    ///
    /// The national organisation representing Scandinavian countries in ODETTE (Organisation for Data Exchange through Tele-Transmission in Europe).
    #[strum(serialize = "208")]
    _208,
    /// GB, ODETTE United Kingdom
    ///
    /// The national organisation representing UK in ODETTE (Organisation for Data Exchange through Tele- Transmission in Europe).
    #[strum(serialize = "209")]
    _209,
    /// EU, EDI for financial, informational, cost, accounting, auditing and social areas (EDIFICAS) - Europe
    ///
    /// European association dealing with accounting and auditing.
    #[strum(serialize = "210")]
    _210,
    /// FR, EDI for financial, informational, cost, accounting, auditing and social areas (EDIFICAS) - France
    ///
    /// French association dealing with accounting and auditing.
    #[strum(serialize = "211")]
    _211,
    /// DE, Deutsch Telekom AG
    ///
    /// German telecommunication services agency.
    #[strum(serialize = "212")]
    _212,
    /// JP, NACCS Center (Nippon Automated Cargo Clearance System Operations Organization)
    ///
    /// NACCS (Nippon Automated Cargo Clearance System Operation Organization) Center is the operations organization of the automated cargo clearance system in Japan.
    #[strum(serialize = "213")]
    _213,
    /// US, AISI (American Iron and Steel Institute)
    ///
    /// American iron and steel institute.
    #[strum(serialize = "214")]
    _214,
    /// AU, APCA (Australian Payments Clearing Association)
    ///
    /// Australian association responsible for the management of payment clearing.
    #[strum(serialize = "215")]
    _215,
    /// US, Department of Labor
    ///
    /// To identify the United States department of labour.
    #[strum(serialize = "216")]
    _216,
    /// US, N.A.I.C. (National Association of Insurance Commissioners)
    ///
    /// To identify the United States, National Association of Insurance Commissioners.
    #[strum(serialize = "217")]
    _217,
    /// GB, The Association of British Insurers
    ///
    /// An association that administers code lists on behalf of the UK insurance community.
    #[strum(serialize = "218")]
    _218,
    /// FR, d'ArvA
    ///
    /// Value added network administering insurance code lists on behalf of the French insurance community.
    #[strum(serialize = "219")]
    _219,
    /// FI, Finnish tax board
    ///
    /// Finnish tax board.
    #[strum(serialize = "220")]
    _220,
    /// FR, CNAMTS (Caisse Nationale de l'Assurance Maladie des Travailleurs Salaries)
    ///
    /// The French public institution funding health-care for salaried workers.
    #[strum(serialize = "221")]
    _221,
    /// DK, Danish National Board of Health
    ///
    /// The national authority responsible for the supervision of health activities in Denmark.
    #[strum(serialize = "222")]
    _222,
    /// DK, Danish Ministry of Home Affairs
    ///
    /// The ministry responsible for all interior affairs concerning the Danish people.
    #[strum(serialize = "223")]
    _223,
    /// US, Aluminum Association
    ///
    /// Organization that assigns identification numbers for the aluminum industry.
    #[strum(serialize = "224")]
    _224,
    /// US, CIDX (Chemical Industry Data Exchange)
    ///
    /// Organization that assigns identification numbers for the chemical Industry.
    #[strum(serialize = "225")]
    _225,
    /// US, Carbide Manufacturers
    ///
    /// Organization that assigns identification numbers for the iron and carbide manufacturing industry.
    #[strum(serialize = "226")]
    _226,
    /// US, NWDA (National Wholesale Druggist Association)
    ///
    /// Organization that assigns identification numbers for the wholesale drug industry.
    #[strum(serialize = "227")]
    _227,
    /// US, EIA (Electronic Industry Association)
    ///
    /// Organization that assigns identification numbers for the electronic industry.
    #[strum(serialize = "228")]
    _228,
    /// US, American Paper Institute
    ///
    /// Organization that assigns identification numbers for the American paper industry.
    #[strum(serialize = "229")]
    _229,
    /// US, VICS (Voluntary Inter-Industry Commerce Standards)
    ///
    /// Organization that assigns identification numbers for the retail industry.
    #[strum(serialize = "230")]
    _230,
    /// Copper and Brass Fabricators Council
    ///
    /// Organization that assigns identification numbers for the copper and brass fabricators industry.
    #[strum(serialize = "231")]
    _231,
    /// GB, Inland Revenue
    ///
    /// Code identifying the government department responsible for assessing and collecting revenue consisting of taxes and inland duties in Great Britain.
    #[strum(serialize = "232")]
    _232,
    /// US, OMB (Office of Management and Budget)
    ///
    /// Codes are assigned by the United States Office of Management and Budget.
    #[strum(serialize = "233")]
    _233,
    /// DE, Siemens AG
    ///
    /// Siemens AG, Germany.
    #[strum(serialize = "234")]
    _234,
    /// AU, Tradegate (Electronic Commerce Australia)
    ///
    /// Australian industry body coordinating codes for use in local and international commerce and trade.
    #[strum(serialize = "235")]
    _235,
    /// US, United States Postal Service (USPS)
    ///
    /// Code specifying the official postal service of the United States.
    #[strum(serialize = "236")]
    _236,
    /// US, United States health industry
    ///
    /// Code assigned by the United States health industry.
    #[strum(serialize = "237")]
    _237,
    /// US, TDCC (Transportation Data Coordinating Committee)
    ///
    /// United States Transportation Data Coordinating Committee.
    #[strum(serialize = "238")]
    _238,
    /// US, HL7 (Health Level 7)
    ///
    /// United States, electronic data interchange standards- making organization, Health Level 7.
    #[strum(serialize = "239")]
    _239,
    /// US, CHIPS (Clearing House Interbank Payment Systems)
    ///
    /// United States financial clearing house.
    #[strum(serialize = "240")]
    _240,
    /// PT, SIBS (Sociedade Interbancaria de Servicos)
    ///
    /// Portuguese automated clearing house.
    #[strum(serialize = "241")]
    _241,
    /// NL, Interpay Giraal
    ///
    /// Interpay Giraal.
    #[strum(serialize = "242")]
    _242,
    /// NL, Interpay Cards
    ///
    /// Interpay Cards.
    #[strum(serialize = "243")]
    _243,
    /// US, Department of Health and Human Services
    ///
    /// United States Department of Health and Human Services.
    #[strum(serialize = "244")]
    _244,
    /// DK, EAN (European Article Numbering) Denmark
    ///
    /// Denmark based European Article Numbering (EAN) association.
    #[strum(serialize = "245")]
    _245,
    /// DE, Centrale fuer Coorganisation GMBH
    ///
    /// German representation of European Article Numbering (EAN) International.
    #[strum(serialize = "246")]
    _246,
    /// US, HBICC (Health Industry Business Communication Council)
    ///
    /// Code identifying the United States HIBCC (Health Industry Business Communication Council).
    #[strum(serialize = "247")]
    _247,
    /// US, ASTM (American Society of Testing and Materials)
    ///
    /// A not-for-profit organization that provides a forum for producers, users, ultimate consumers, and those having a general interest (representatives of government and academia) to meet on common ground and write standards for materials, products, systems, and services.
    #[strum(serialize = "248")]
    _248,
    /// IP (Institute of Petroleum)
    ///
    /// An independent European centre for the advancement and dissemination of technical, economic and professional knowledge relating to the international oil and gas industry.
    #[strum(serialize = "249")]
    _249,
    /// US, UOP (Universal Oil Products)
    ///
    /// An United States based organization that provides products, services and technology primarily in the areas of petroleum refining, olefins, aromatics, and gas processing.
    #[strum(serialize = "250")]
    _250,
    /// AU, HIC (Health Insurance Commission)
    ///
    /// Australian agency responsible for administering the Health Insurance Act.
    #[strum(serialize = "251")]
    _251,
    /// AU, AIHW (Australian Institute of Health and Welfare)
    ///
    /// Australian statutory authority responsible for the national collection of health related statistics and health related data definitions.
    #[strum(serialize = "252")]
    _252,
    /// AU, NCCH (National Centre for Classification in Health)
    ///
    /// Australian national authority responsible for healthcare classifications.
    #[strum(serialize = "253")]
    _253,
    /// AU, DOH (Australian Department of Health)
    ///
    /// Australian government department responsible for administration of health policy.
    #[strum(serialize = "254")]
    _254,
    /// AU, ADA (Australian Dental Association)
    ///
    /// Industry association responsible for the classification of dental services in Australia.
    #[strum(serialize = "255")]
    _255,
    /// US, AAR (Association of American Railroads)
    ///
    /// The official United States organization of the railroads in North America.
    #[strum(serialize = "256")]
    _256,
    /// US, UN/SPSC (United Nations Standard Products and Services Classification) association
    ///
    /// The agency responsible for the maintenance of the United Nations standard products and services classification code.
    #[strum(serialize = "257")]
    _257,
    /// JP, Japanese Ministry of Transport
    ///
    /// Japanese Ministry of Transport.
    #[strum(serialize = "258")]
    _258,
    /// JP, Japanese Maritime Safety Agency
    ///
    /// Japanese Maritime Safety Agency.
    #[strum(serialize = "259")]
    _259,
    /// Ediel Nordic forum
    ///
    /// A code to identify Ediel Nordic forum, which is an organization standardizing the use of EDI between the participants in the Nordic power market.
    #[strum(serialize = "260")]
    _260,
    /// EEG7, European Expert Group 7 (Insurance)
    ///
    /// European Expert Group 7 for Insurance.
    #[strum(serialize = "261")]
    _261,
    /// DE, GDV (Gesamtverband der Deutschen Versicherungswirtschaft e.V.)
    ///
    /// Gesamtverband der Deutschen Versicherungswirtschaft e.V. (German Insurance Association).
    #[strum(serialize = "262")]
    _262,
    /// CA, CSIO (Centre for Study of Insurance Operations)
    ///
    /// The Centre for Study of Insurance Operations (CSIO) in Canada.
    #[strum(serialize = "263")]
    _263,
    /// FR, AGF (Assurances Generales de France)
    ///
    /// Code lists are administered by Assurances Generales de France (AGF).
    #[strum(serialize = "264")]
    _264,
    /// SE, Central bank
    ///
    /// Swedish central bank.
    #[strum(serialize = "265")]
    _265,
    /// US, DoA (Department of Agriculture)
    ///
    /// Department of Agriculture, United States federal agency.
    #[strum(serialize = "266")]
    _266,
    /// RU, Russian Bank Identification Code (BIC)
    ///
    /// BIC is used for party identification in the bank of Russia payment system and is a subdivision directory for the bank of Russia.
    #[strum(serialize = "267")]
    _267,
    /// FR, DGI (Direction Generale des Impots)
    ///
    /// French taxation authority.
    #[strum(serialize = "268")]
    _268,
    /// GRE (Reference Group of Experts)
    ///
    /// An international association that administers code lists on behalf of business credit information users and providers.
    #[strum(serialize = "269")]
    _269,
    /// Concord EDI group
    ///
    /// An organisation of international transport equipment leasing companies and transport equipment repair providers responsible for promoting the use of EDI standards and standard business terms.
    #[strum(serialize = "270")]
    _270,
    /// InterContainer InterFrigo
    ///
    /// European railway associated organisation involved in the transport of containers by rail.
    #[strum(serialize = "271")]
    _271,
    /// Joint Automotive Industry agency
    ///
    /// The Joint Automotive Industry (JAI) agency is in charge of code lists that are common to automotive industry groups.
    #[strum(serialize = "272")]
    _272,
    /// CH, SCC (Swiss Chambers of Commerce)
    ///
    /// Swiss Chambers of Commerce.
    #[strum(serialize = "273")]
    _273,
    /// ITIGG (International Transport Implementation Guidelines Group)
    ///
    /// ITIGG is the UN/EDIFACT transport message development group's organisation responsible for the issuance of globally harmonised transport-related codes.
    #[strum(serialize = "274")]
    _274,
    /// ES, Banco de España
    ///
    /// The Spanish central bank.
    #[strum(serialize = "275")]
    _275,
    /// Mutually defined
    #[strum(ascii_case_insensitive)]
    ZZZ,
}

/// Contact function code
///
/// Code specifying the function of a contact (e.g. department or person).
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _3139 {
    /// Insurance contact
    ///
    /// Department/person to contact for matters regarding insurance.
    AA,
    /// Workshop contact
    ///
    /// Department/person to contact for matters regarding the workshop.
    AB,
    /// Accepting contact
    ///
    /// Department/person in charge of accepting incoming goods.
    AC,
    /// Accounting contact
    ///
    /// The contact responsible for accounting matters.
    AD,
    /// Contract contact
    ///
    /// Department/person to contact for matters regarding contracts.
    AE,
    /// Land registry contact
    ///
    /// Department/person to contact for matters regarding land registry.
    AF,
    /// Agent
    ///
    /// Department/person of the agent which acts on behalf of another party.
    AG,
    /// Coordination contact
    ///
    /// Department/person to contact for matters regarding technical coordination of works.
    AH,
    /// Project management contact
    ///
    /// Department/person to contact for matters regarding project management on behalf of the contractor.
    AI,
    /// Investment contact
    ///
    /// Department/person to contact for matters regarding investments.
    AJ,
    /// Works management contact
    ///
    /// Department/person to contact for matters regarding management of works on behalf of the owner.
    AK,
    /// Personnel contact
    ///
    /// Department/person to contact for matters regarding personnel (human resources).
    AL,
    /// Claims contact
    ///
    /// Department/person to contact for matters regarding claims.
    AM,
    /// Laboratory contact
    ///
    /// Department/person to contact for laboratory matters.
    AN,
    /// Plant/equipment contact
    ///
    /// Department/person to contact for matters regarding plant/equipment.
    AO,
    /// Accounts payable contact
    ///
    /// Department/person responsible for the accounts payable function within a corporation.
    AP,
    /// Quantity surveyor contact
    ///
    /// Department/person to contact for matters regarding quantity surveying.
    AQ,
    /// Accounts receivable contact
    ///
    /// Department/person responsible for the accounts receivable within a corporation.
    AR,
    /// Public relations contact
    ///
    /// Department/person to contact for matters regarding public relations.
    AS,
    /// Technical contact
    ///
    /// Department/person to contact for matters regarding technical issues.
    AT,
    /// City works authority contact
    ///
    /// Department/person to contact for matters regarding city works.
    AU,
    /// Maintenance contact
    ///
    /// Department/person to contact for matters regarding maintenance.
    AV,
    /// Town planning contact
    ///
    /// Department/person to contact for matters regarding town ` planning.
    AW,
    /// Traffic authority contact
    ///
    /// Department/person to contact for matters regarding traffic.
    AX,
    /// Electricity supply contact
    ///
    /// Department/person to contact for matters regarding electricity supply.
    AY,
    /// Gas supply contact
    ///
    /// Department/person to contact for matters regarding gas supply.
    AZ,
    /// Water supply contact
    ///
    /// Department/person to contact for matters regarding water supply.
    BA,
    /// Telecommunications network contact
    ///
    /// Department/person to contact for matters regarding telecommunications network.
    BB,
    /// Banking contact
    ///
    /// Contact person for bank.
    BC,
    /// New developments contact
    ///
    /// Department/person to contact for matters regarding new developments (e.g. construction).
    BD,
    /// Transport infrastructure authority
    ///
    /// Department/person to contact for matters regarding transport infrastructure.
    BE,
    /// Service contact
    ///
    /// Department/person to be contacted in service matters.
    BF,
    /// Auditing contact
    ///
    /// Department or person to contact with regard to auditing.
    BG,
    /// Legal auditing contact
    ///
    /// Department or person to contact with regard to legal auditing.
    BH,
    /// Software house contact
    ///
    /// Department or person to contact with regard to software house.
    BI,
    /// Department or person responsible for processing purchase order
    ///
    /// Identification of the department or person responsible for the processing of purchase orders.
    BJ,
    /// Electronic data interchange coordinator
    ///
    /// Code specifying a person responsible for the coordination of matters related to the exchange of information in electronic data interchange format.
    BK,
    /// Waiver contact
    ///
    /// Code specifying a party knowledgeable about a waiver.
    BL,
    /// Automated clearing house (ACH) contact
    ///
    /// Code specifying a person to be contacted at an automated clearing house.
    BM,
    /// Certification contact
    ///
    /// Code specifying a contact with knowledge of a certification action.
    BN,
    /// Ultimate consignee
    ///
    /// Final recipient of the consignment.
    BU,
    /// Carrier
    ///
    /// (3126) Party undertaking or arranging transport of goods between named points.
    CA,
    /// Changed by
    ///
    /// Person who made the change.
    CB,
    /// Responsible person for information production
    ///
    /// Responsible person to contact for matters regarding the production of information.
    CC,
    /// Responsible person for information dissemination
    ///
    /// Responsible person to contact for matters regarding information dissemination.
    CD,
    /// Head of unit for computer data processing
    ///
    /// Head of unit to contact for matters regarding computer data processing.
    CE,
    /// Head of unit for information production
    ///
    /// Head of unit to contact for matters regarding the production of information.
    CF,
    /// Head of unit for information dissemination
    ///
    /// Head of unit to contact for matters regarding dissemination of information.
    CG,
    /// Consignee
    ///
    /// (3132) Party to which goods are consigned.
    CN,
    /// Consignor
    ///
    /// (3336) Party which, by contract with a carrier, consigns or sends goods with the carrier, or has them conveyed by him. Synonym: shipper/sender.
    CO,
    /// Responsible person for computer data processing
    ///
    /// Responsible person to contact for matters regarding computer data processing.
    CP,
    /// Customer relations
    ///
    /// Individual responsible for customer relations.
    CR,
    /// Confirmed with
    ///
    /// Person with whom the contents of the purchase order has been discussed and agreed (e.g. by telephone) prior to the sending of this message.
    CW,
    /// Department/employee to execute export procedures
    DE,
    /// Department/employee to execute import procedures
    DI,
    /// Delivery contact
    ///
    /// Department/person responsible for delivery.
    DL,
    /// Entered by
    ///
    /// Name of an individual who made the entry.
    EB,
    /// Education coordinator
    ///
    /// Person in charge of coordination of education.
    EC,
    /// Engineering contact
    ///
    /// Department/person to contact for matters regarding engineering.
    ED,
    /// Expeditor
    ///
    /// The contact for expediting.
    EX,
    /// Goods receiving contact
    ///
    /// Department/person responsible for receiving the goods at the place of delivery.
    GR,
    /// Emergency dangerous goods contact
    ///
    /// Party who is to be contacted to intervene in case of emergency.
    HE,
    /// Dangerous goods contact
    ///
    /// Department/person to be contacted for details about the transportation of dangerous goods/hazardous material.
    HG,
    /// Hazardous material contact
    ///
    /// Department/person responsible for hazardous material control.
    HM,
    /// Information contact
    ///
    /// Department/person to contact for questions regarding transactions.
    IC,
    /// Insurer contact
    IN,
    /// Place of delivery contact
    LB,
    /// Place of collection contact
    LO,
    /// Material control contact
    ///
    /// Department/person responsible for the controlling/inspection of goods.
    MC,
    /// Material disposition contact
    ///
    /// Department/person responsible for the disposition/scheduling of goods.
    MD,
    /// Material handling contact
    MH,
    /// Message recipient contact
    MR,
    /// Message sender contact
    MS,
    /// Notification contact
    NT,
    /// Order contact
    ///
    /// An individual to contact for questions regarding this order.
    OC,
    /// Prototype coordinator
    ///
    /// Description to be provided.
    PA,
    /// Purchasing contact
    ///
    /// Department/person responsible for issuing this purchase order.
    PD,
    /// Payee contact
    PE,
    /// Product management contact
    ///
    /// Department/person to contact for questions regarding this order.
    PM,
    /// Quality assurance contact
    ///
    /// Quality assurance contact within an organization.
    QA,
    /// Quality coordinator contact
    ///
    /// Quality coordinator contact within an organization.
    QC,
    /// Receiving dock contact
    ///
    /// The receiving dock contact within an organization.
    RD,
    /// Sales administration
    ///
    /// Name of the sales administration contact within a corporation.
    SA,
    /// Schedule contact
    ///
    /// Name of the scheduling contact within a corporation.
    SC,
    /// Shipping contact
    ///
    /// The shipping department contact within an organization.
    SD,
    /// Sales representative or department
    ///
    /// The sales representative or department contact within an organization.
    SR,
    /// Supplier contact
    ///
    /// Department/person to be contacted at the supplier.
    SU,
    /// Traffic administrator
    ///
    /// The traffic administrator contact within an organization.
    TA,
    /// Test contact
    ///
    /// Department/person responsible for testing contact.
    TD,
    /// Technical documentation recipient
    ///
    /// Department/person to receive technical documentation.
    TI,
    /// Transport contact
    ///
    /// Department/person in charge of transportation.
    TR,
    /// Warehouse
    ///
    /// The warehouse contact within an organization.
    WH,
    /// Alternate contact
    ///
    /// Alternate department or person to contact.
    WI,
    /// Office Manager
    ///
    /// An individual responsible for managing the day to day activities of an office.
    WJ,
    /// Mutually defined
    ZZZ,
}

/// Communication address code qualifier
///
/// Code qualifying the communication address.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _3155 {
    /// Circuit switching
    ///
    /// A process that, on demand, connects two or more data terminal equipments and permits the exclusive use of a data circuit between them until the connection is released (ISO).
    AA,
    /// SITA
    ///
    /// Communications number assigned by Societe Internationale de Telecommunications Aeronautiques (SITA).
    AB,
    /// ARINC
    ///
    /// Communications number assigned by Aeronautical Radio Inc.
    AC,
    /// AT&T mailbox
    ///
    /// AT&T mailbox identifier.
    AD,
    /// Peripheral device
    ///
    /// Peripheral device identification.
    AE,
    /// U.S. Defense Switched Network
    ///
    /// The switched telecommunications network of the United States Department of Defense.
    AF,
    /// U.S. federal telecommunications system
    ///
    /// The switched telecommunications network of the United States government.
    AG,
    /// World Wide Web
    ///
    /// Data exchange via the World Wide Web.
    AH,
    /// International calling country code
    ///
    /// Identifies that portion of an international telephone number representing the country code to be used when calling internationally.
    AI,
    /// Alternate telephone
    ///
    /// Identifies the alternate telephone number.
    AJ,
    /// Videotex number
    ///
    /// Code that identifies the communications number for the online videotex service.
    AK,
    /// Cellular phone
    ///
    /// Identifies the cellular phone number.
    AL,
    /// International telephone direct line
    ///
    /// The international telephone direct line number.
    AM,
    /// O.F.T.P. (ODETTE File Transfer Protocol)
    ///
    /// ODETTE File Transfer Protocol.
    AN,
    /// Cable address
    ///
    /// EDI transmission
    CA,
    ///
    /// Number identifying the service and service user.
    /// EI,
    /// Electronic mail
    ///
    /// Exchange of mail by electronic means.
    EM,
    /// Extension
    ///
    /// Telephone extension.
    EX,
    /// File transfer access method
    ///
    /// According to ISO.
    FT,
    /// Telefax
    ///
    /// Device used for transmitting and reproducing fixed graphic material (as printing) by means of signals over telephone lines or other electronic transmission media.
    FX,
    /// GEIS (General Electric Information Service) mailbox
    ///
    /// IBM information exchange
    GM,
    ///
    /// Internal mail
    /// IE,
    ///
    /// Internal mail address/number.
    /// IM,
    /// Mail
    ///
    /// Postal service document delivery.
    MA,
    /// Postbox number
    ///
    /// Packet switching
    PB,
    ///
    /// The process of routing and transferring data by means of addressed packets so that a channel is occupied only during the transmission; upon completion of the transmission the channel is made available for the transfer of other packets (ISO).
    /// PS,
    /// S.W.I.F.T.
    ///
    /// Communications address assigned by Society for Worldwide Interbank Financial Telecommunications s.c.
    SW,
    /// Telephone
    ///
    /// Voice/data transmission by telephone.
    TE,
    /// Telegraph
    ///
    /// Text transmission via telegraph.
    TG,
    /// Telex
    ///
    /// Transmission of text/data via telex.
    TL,
    /// Telemail
    ///
    /// Transmission of text/data via telemail.
    TM,
    /// Teletext
    ///
    /// Transmission of text/data via teletext.
    TT,
    /// TWX
    ///
    /// Communication service involving Teletypewriter machines connected by wire or electronic transmission media. Teletypewriter machines are the devices used to send and receive signals and produce hardcopy from them.
    TX,
    /// X.400 address
    ///
    /// The X.400 address.
    XF,
    /// Pager
    ///
    /// Identifies that the communication number is for a pager.
    XG,
    /// International telephone switchboard
    ///
    /// The international telephone switchboard number.
    XH,
    /// National telephone direct line
    ///
    /// The national telephone direct line number.
    XI,
    /// National telephone switchboard
    ///
    /// The national telephone switchboard number.
    XJ,
}

/// Delivery or transport terms description code
///
/// Code specifying the delivery or transport terms.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _4053 {
    /// Delivery arranged by the supplier
    ///
    /// Indicates that the supplier will arrange delivery of the goods.
    #[strum(serialize = "1")]
    _1,
    /// Delivery arranged by logistic service provider
    ///
    /// Code indicating that the logistic service provider has arranged the delivery of goods.
    #[strum(serialize = "2")]
    _2,
}

/// Delivery or transport terms function code
///
/// Code specifying the function of delivery or transport terms.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _4055 {
    /// Price condition
    ///
    /// Description to be provided.
    #[strum(serialize = "1")]
    _1,
    /// Despatch condition
    ///
    /// Condition requested by the customer under which the supplier shall deliver: Extent of freight costs, means of transport.
    #[strum(serialize = "2")]
    _2,
    /// Price and despatch condition
    ///
    /// Description to be provided.
    #[strum(serialize = "3")]
    _3,
    /// Collected by customer
    ///
    /// Indicates that the customer will pick up the goods at the supplier. He will take care of the means of transport.
    #[strum(serialize = "4")]
    _4,
    /// Transport condition
    ///
    /// Specifies the conditions under which the transport takes place under the responsibility of the carrier.
    #[strum(serialize = "5")]
    _5,
    /// Delivery condition
    ///
    /// Specifies the conditions under which the goods must be delivered to the consignee.
    #[strum(serialize = "6")]
    _6,
}

/// 4065 Contract and carriage condition code
///
/// Code to identify the conditions of contract and carriage.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _4065 {
    /// AVC conditions
    /// General conditions of transport 1983 latest revision laid down by the Stichting Vervoeradres The Hague.
    #[strum(serialize = "1")]
    _1,
    /// Special agreement for parcels transport
    /// Appliance of a non published special agreement signed between a customer and the carrier (mandatory requested by the consignor) for parcels transport.
    #[strum(serialize = "2")]
    _2,
    /// Special agreement for full loading transport
    /// Appliance of a non published special agreement signed between a customer and the carrier (mandatory requested by the consignor) for full load transport.
    #[strum(serialize = "3")]
    _3,
    /// Combined transport
    /// A transport which involves more than one mode of transportation.
    #[strum(serialize = "4")]
    _4,
    /// FIATA combined transport bill of lading
    /// Standard conditions of a combined transport bill of lading issued by FIATA.
    #[strum(serialize = "5")]
    _5,
    /// Freight forwarders national conditions
    /// The contract and carriage conditions as established by freight forwarders on a national basis.
    #[strum(serialize = "6")]
    _6,
    /// Normal tariff, parcels transport
    /// Appliance of the published legal tariff in case of parcels transport (required or not by the consignor.
    #[strum(serialize = "7")]
    _7,
    /// Normal tariff, full loading transport
    /// Appliance of the published legal tariff in case of full load transport (required or not by the consignor).
    #[strum(serialize = "8")]
    _8,
    /// Ordinary
    /// Carrier will choose the cheapest tariff in the legally published tariffs for parcels or full load transports (no tariff required by the consignor).
    #[strum(serialize = "9")]
    _9,
    /// Port to port
    /// The transport will only be port to port, no inland transport would have to be provided under the contract.
    #[strum(serialize = "10")]
    _10,
    /// CMR carnet
    /// Conditions in accordance with the convention of the contract for the international carriage of goods by road.
    #[strum(serialize = "11")]
    _11,
    /// Special tariff, parcels transport
    /// Appliance of the legally published "special" tariff in case or parcels transport (tariff requested by the consignor).
    #[strum(serialize = "12")]
    _12,
    /// Special tariff, full transport
    /// Appliance of the legally published "special tariff" in case of full load transport (tariff requested by the consignor).
    #[strum(serialize = "13")]
    _13,
    /// Through transport
    /// The transport that is contracted not only from port to port, but from one inland location to another inland location.
    #[strum(serialize = "14")]
    _14,
    /// Cancel space allocation
    /// Indication that space previously allocated on a flight is to be cancelled.
    #[strum(serialize = "15")]
    _15,
    /// Report sale of space
    /// Indication that a sale has been made against a space allocation on a specific flight.
    #[strum(serialize = "16")]
    _16,
    /// Alternative space allocation
    /// Indication that space is being requested for a specific flight and that an alternative is acceptable.
    #[strum(serialize = "17")]
    _17,
    /// No alternative space allocation
    /// Indication that space is being requested for a specific flight and that an alternative is not acceptable.
    #[strum(serialize = "18")]
    _18,
    /// Allotment sale
    /// Indication that space is being sold against a space allocation allotment on a specific flight.
    #[strum(serialize = "19")]
    _19,
    /// Confirmation of space
    /// Indication that space requested has been confirmed on a specific flight.
    #[strum(serialize = "20")]
    _20,
    /// Unable to confirm
    /// Indication that airline is unable to confirm the space allocation on a specific flight.
    #[strum(serialize = "21")]
    _21,
    /// Non-operative flight
    /// Indication that airline is unable to confirm space on a specific flight since the flight does not operate.
    #[strum(serialize = "22")]
    _22,
    /// Wait list
    /// Indication that the space allocation request has been assigned to a wait list.
    #[strum(serialize = "23")]
    _23,
    /// Prior space allocation request
    /// Indication that a space allocation on a specific flight has already been requested.
    #[strum(serialize = "24")]
    _24,
    /// Holding confirmed space allocation
    /// Indication that space is being held as confirmed on a specific flight.
    #[strum(serialize = "25")]
    _25,
    /// Holding wait list
    /// Indication that space allocation request on a specific flight has been assigned to a wait list.
    #[strum(serialize = "26")]
    _26,
    /// Door-to-door
    /// The carrier is responsible for the intermodal carriage of cargo including both the pre-carriage and the on- carriage.
    #[strum(serialize = "27")]
    _27,
    /// Door-to-pier
    /// The carrier is responsible for the intermodal carriage of cargo including the pre-carriage, but excluding the on-carriage.
    #[strum(serialize = "28")]
    _28,
    /// Pier-to-door
    /// The carrier is responsible for the intermodal carriage of cargo including the on-carriage, but excluding the pre-carriage.
    #[strum(serialize = "29")]
    _29,
    /// Pier-to-pier
    /// The carrier of intermodal cargo is only responsible for the main carriage.
    #[strum(serialize = "30")]
    _30,
    /// Space cancellation noted
    /// Indication that space previously allocated on a means of transport has been cancelled.
    #[strum(serialize = "31")]
    _31,
    /// Mini landbridge service
    /// Cargo moving from a coastal port for delivery at an inland location or cargo received at an inland location moving to a coastal port for subsequent ocean transportation.
    #[strum(serialize = "32")]
    _32,
    /// Space cancellation noted
    /// Indication that space previously allocated on a flight has been cancelled.
    #[strum(serialize = "33")]
    _33,
    /// Speed level - required
    /// Maximum speed required on an itinerary or part of this itinerary to be able to assume some services.
    #[strum(serialize = "34")]
    _34,
    /// Speed level - adopted
    /// Real speed used on an itinerary or part of this itinerary (for technical reasons, some limitation can be imposed or some higher speed could be used).
    #[strum(serialize = "35")]
    _35,
    /// Normal tariff, less than full load transport
    /// Application of the published legal tariff in case of less than full load transport (required or not by the consignor).
    #[strum(serialize = "36")]
    _36,
    /// Re-expedition special tariff
    /// Indication that a special tariff must be used in the case of a re-expedition.
    #[strum(serialize = "37")]
    _37,
    /// Transport arrangement by the requester
    /// The service requester has the responsibility of making transport arrangement.
    #[strum(serialize = "38")]
    _38,
    /// Transport arrangement by the provider
    /// The service provider has the responsibility of making transport arrangement.
    #[strum(serialize = "39")]
    _39,
    /// Transport arrangement by the patient
    /// The patient has the responsibility of making transport arrangement.
    #[strum(serialize = "40")]
    _40,
}

/// Transport charges payment method code
///
/// Code specifying the payment method for transport charges.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _4215 {
    /// Account
    ///
    /// The charges are to be charged to an account.
    A,
    /// Cash on delivery service charge paid by consignor
    ///
    /// An indication that the consignor is responsible for the payment of the cash on delivery service charge.
    AA,
    /// Cash on delivery service charge paid by consignee
    ///
    /// An indication that the consignee is responsible for the payment of the cash on delivery service charge.
    AB,
    /// Insurance costs paid by consignor
    ///
    /// An indication that the consignor is responsible for the payment of the insurance costs.
    AC,
    /// Insurance costs paid by consignee
    ///
    /// An indication that the consignee is responsible for the payment of the insurance costs.
    AD,
    /// Advance collect
    ///
    /// The amount of freight or other charge on a shipment advanced by one transportation line to another or to the shipper, to be collected from consignee.
    CA,
    /// Collect
    ///
    /// A shipment on which freight charges will be paid by consignee.
    CC,
    /// Collect, freight credited to payment customer
    ///
    /// The freight is collect but has been paid by the shipper and will be credited to that party.
    CF,
    /// Defined by buyer and seller
    ///
    DF,
    /// FOB port of call
    ///
    /// Title and control of goods pass to the buyer at port of call. Responsibility for export taxes and cost of documents for overseas shipments have not been specified.
    FO,
    /// Information copy, no payment due
    ///
    /// Transaction set has been provided for information only.
    IC,
    /// Mixed
    ///
    /// The consignment is partially collect and partially prepaid.
    MX,
    /// Service freight, no charge
    ///
    /// The consignment is shipped on a service basis and there is no freight charge.
    NC,
    /// Not specified
    NS,
    /// Advance prepaid
    ///
    /// Costs have been paid in advance.
    PA,
    /// Customer pick-up/backhaul
    ///
    /// Buyer's private carriage picks up the goods as a return load to the buyer's facility.
    PB,
    /// Prepaid but charged to customer
    ///
    /// shipping charges have been paid in advance of shipment but are charged back to consignee usually as line item on invoice for the purchased goods.
    PC,
    /// Payable elsewhere
    ///
    /// Place of payment not known at the begin of conveyance.
    PE,
    /// Prepaid only
    ///
    /// Payment in advance of freight and/or other charges prior to delivery of shipment at destination, usually by shipper at point of origin.
    PO,
    /// Prepaid (by seller)
    ///
    /// Seller of goods makes payment to carrier for freight charges prior to shipment.
    PP,
    /// Pickup
    ///
    /// Customer is responsible for payment of pickup charges at shipping point.
    PU,
    /// Return container freight paid by customer
    ///
    /// The freight for returning the container is paid by the customer.
    RC,
    /// Return container freight free
    ///
    /// There is no freight charge for returning the container.
    RF,
    /// Return container freight paid by supplier
    ///
    /// The freight charge for returning the container is paid by the supplier.
    RS,
    /// Third party pay
    ///
    /// A third party, someone other than buyer or seller, is identified as responsible for payment of shipping charges.
    TP,
    /// Weight condition
    ///
    /// Description to be provided.
    WC,
    /// Paid by supplier
    ///
    /// Transport charges will be paid by the supplier.
    WD,
    /// Paid by buyer
    ///
    /// Transport charges will be paid by the buyer.
    WE,
    /// Mutually defined
    ZZZ,
}

/// Transport service priority code
///
/// Code specifying the priority of a transport service.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _4219 {
    /// Express
    ///
    /// Express treatment (if by rail, legal express regime for parcels transport).
    #[strum(serialize = "1")]
    _1,
    /// High speed
    ///
    /// Transport under legal international rail convention (CIM) concluded between rail organizations and based on fast routing and specified timetables.
    #[strum(serialize = "2")]
    _2,
    /// Normal speed
    ///
    /// Transport under legal international rail convention (CIM) concluded between rail organizations.
    #[strum(serialize = "3")]
    _3,
    /// Post service
    ///
    /// Transport under conditions specified by UPU (Universal Postal Union) and Rail organizations (parcels transport only).
    #[strum(serialize = "4")]
    _4,
}

/// Payment arrangement code
///
/// Code specifying the arrangements for a payment.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _4237 {
    /// Payable elsewhere
    ///
    /// Responsibility for payment of transport charges unknown at time of departure.
    A,
    /// Third party to pay
    ///
    /// A third party to pay the freight bill is known at the time of shipment.
    B,
    /// Collect
    ///
    /// Charges are (to be) collected from the consignee at the destination.
    C,
    /// Prepaid
    ///
    /// Charges are (to be) prepaid before the transport actually leaves.
    P,
}

/// Response type code
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

/// Price code qualifier
///
/// Code qualifying a price.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _5125 {
    /// Calculation net
    ///
    /// The price stated is the net price including allowances/ charges. Allowances/charges may be stated for information only.
    AAA,
    /// Calculation gross
    ///
    /// The price stated is the gross price to which allowances/ charges must be applied.
    AAB,
    /// Allowances and charges not included, tax included
    ///
    /// The price does not include the allowances and charges, but includes the taxes.
    AAC,
    /// Average selling price
    ///
    /// Average selling price of a product.
    AAD,
    /// Information price, excluding allowances or charges, including taxes
    ///
    /// The price stated is for information purposes only and excludes all allowances and charges. Taxes however are included in the price.
    AAE,
    /// Information price, excluding allowances or charges, and taxes
    ///
    /// The price stated is for information purposes only and excludes all allowances, charges and taxes.
    AAF,
    /// Additive unit price component
    ///
    /// A code to indicate that the price described is an additive component of the total price.
    AAG,
    /// Calculation price
    ///
    /// The price stated is the price for the calculation of the line item amount.
    CAL,
    /// Information
    ///
    /// The price is provided for information.
    INF,
    /// Invoice price
    INV,
}

/// Sub-line item price change operation code
///
/// Code specifying the price change operation for a sub- line item.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _5213 {
    /// Added to the baseline item unit price
    A,
    /// Included in the baseline item unit price
    I,
    /// Subtracted from the baseline item unit price
    S,
}

/// Charge category code
///
/// Code specifying the category of charges.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _5237 {
    /// All charges
    ///
    /// All amounts calculated by the carrier in accordance with tariffs or in case of special events during the voyage (e.g. Rail - freights costs - additional costs).
    #[strum(serialize = "1")]
    _1,
    /// Additional charges
    ///
    /// Charges calculated by the carrier for specific events like re-weighting, re-loading, unexpected operations, services required during the voyage, etc.
    #[strum(serialize = "2")]
    _2,
    /// Transport charges + additional charges
    ///
    /// Transport charges plus Additional charges (e.g. for re- loading, re-weighting or unexpected operations) that must be precised in the payment conditions by the consignor (other charges must be taken in account by the consignee).
    #[strum(serialize = "3")]
    _3,
    /// Basic freight
    ///
    /// The basic freight payable on the cargo as per tariff.
    #[strum(serialize = "4")]
    _4,
    /// Destination haulage charges
    ///
    /// Haulage charges for transporting goods to the destination.
    #[strum(serialize = "5")]
    _5,
    /// Disbursement
    ///
    /// Sums paid out by ship's agent at a port and recovered from the carrier.
    #[strum(serialize = "6")]
    _6,
    /// Destination port charges
    ///
    /// Charges payable at the port of destination.
    #[strum(serialize = "7")]
    _7,
    /// Miscellaneous charges
    ///
    /// Miscellaneous charges not otherwise categorized.
    #[strum(serialize = "8")]
    _8,
    /// Transport charges up to a specified location
    ///
    /// Transport charges to be paid by a specified party for a part of a voyage, i.e. up to a specified location.
    #[strum(serialize = "9")]
    _9,
    /// Origin port charges
    ///
    /// Charges payable at the port of origin.
    #[strum(serialize = "10")]
    _10,
    /// Origin haulage charges
    ///
    /// Haulage charges for the pickup of goods at origin.
    #[strum(serialize = "11")]
    _11,
    /// Other charges
    ///
    ///
    #[strum(serialize = "12")]
    _12,
    /// Specific amount payable
    ///
    /// Amount that the consignor agrees to be invoiced or to pay. This amount is part of the total charges applied to the consignment.
    #[strum(serialize = "13")]
    _13,
    /// Transport costs (carriage charges)
    ///
    /// Monetary amount calculated on the basis of the transport tariffs or contract eventually including charges or other costs.
    #[strum(serialize = "14")]
    _14,
    /// All costs up to a specified location
    ///
    /// All amounts to be paid by the consignor for a part of the voyage, i.e. up to a location that must be precised. (The remaining part of the voyage to be paid by the consignee) The amounts are calculated by the carrier in accordance with tariffs or in case of special events during the voyage (e.g. rail - freight costs - additional costs).
    #[strum(serialize = "15")]
    _15,
    /// Weight/valuation charge
    ///
    /// Code to indicate weight/valuation charges to be either wholly prepaid or wholly collect.
    #[strum(serialize = "16")]
    _16,
    /// All costs
    ///
    /// Description to be provided.
    #[strum(serialize = "17")]
    _17,
    /// Transport costs and supplementary costs
    ///
    /// Description to be provided.
    #[strum(serialize = "18")]
    _18,
    /// Supply of certificate of shipment
    ///
    /// Charges payable for the supply of a certificate of shipment.
    #[strum(serialize = "19")]
    _19,
    /// Supply of consular formalities or certificate of origin
    ///
    /// Charges payable for the supply of consular formalities or certificate of origin.
    #[strum(serialize = "20")]
    _20,
    /// Supply of non-categorised documentation in paper form
    ///
    /// Charges payable for the supply of one or more documents in paper form that are not otherwise categorised.
    #[strum(serialize = "21")]
    _21,
    /// Supply of customs formalities, export
    ///
    /// Charges payable for the supply of export customs formalities.
    #[strum(serialize = "22")]
    _22,
    /// Supply of customs formalities, transit
    ///
    /// Charges payable for the supply of transit customs formalities.
    #[strum(serialize = "23")]
    _23,
    /// Supply of customs formalities, import
    ///
    /// Charges payable for the supply of import customs formalities.
    #[strum(serialize = "24")]
    _24,
}

/// Rate or tariff class description code
///
/// Code specifying an applicable rate or tariff class.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _5243 {
    /// Senior person rate
    ///
    /// Rate class applies to senior persons.
    A,
    /// Basic
    B,
    /// Specific commodity rate
    C,
    /// Teenager rate
    ///
    /// Rate class applies to teenagers.
    D,
    /// Child rate
    /// Rate class applies to children.
    E,
    /// Adult rate
    /// Rate class applies to adults.
    F,
    /// Rate per kilogram
    K,
    /// Minimum charge rate
    M,
    /// Normal rate
    N,
    /// Quantity rate
    Q,
    /// Class rate (Reduction on normal rate)
    ///
    ///  Description to be provided.
    R,
    /// Class rate (Surcharge on normal rate)
    ///
    /// Description to be provided.
    S,
}

/// Price type code
///
/// Code specifying the type of price.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _5375 {
    /// Cancellation price
    ///
    /// Price authorized to be charged in the event of an order being cancelled.
    AA,
    /// Per ton
    ///
    /// To indicate that the price applies per ton.
    AB,
    /// Minimum order price
    ///
    /// A code to identify the price when the minimum number is purchased.
    AC,
    /// Export price
    ///
    /// A code to identify a price for the export market.
    AD,
    /// Range dependent price
    ///
    /// A code identifying the price for a specific range of purchase quantities.
    AE,
    /// Active ingredient
    ///
    /// The price is referring to the active ingredient.
    AI,
    /// As is quantity
    ///
    /// The price is referring to the measured quantity.
    AQ,
    /// Catalogue
    CA,
    /// Contract
    CT,
    /// Consumer unit
    ///
    /// The price is referring to the consumer unit.
    CU,
    /// Distributor
    DI,
    /// ECSC price
    ///
    /// Price registered at European Commission Steel and Carbon office (DG III).
    EC,
    /// Net weight
    NW,
    /// Price catalogue
    PC,
    /// Per each
    PE,
    /// Per kilogram
    PK,
    /// Per litre
    PL,
    /// Per tonne
    PT,
    /// Specified unit
    PU,
    /// Provisional price
    PV,
    /// Gross weight
    PW,
    /// Quoted
    QT,
    /// Suggested retail
    SR,
    /// To be negotiated
    TB,
    /// Traded unit
    ///
    /// The price is referring to the traded unit.
    TU,
    /// Theoretical weight
    ///
    /// Weight calculated on ordered dimension (length, width, thickness) not on final dimension (e.g. steel products).
    TW,
    /// Wholesale
    WH,
    /// Gross volume
    ///
    /// The price is calculated based on gross volume.
    WI,
}

/// Price specification code
///
/// Code identifying pricing specification.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _5387 {
    /// Reference price
    AAA,
    /// Price includes tax
    AAB,
    /// Buyer suggested retail price
    ///
    /// The suggested retail price as suggested or determined by the party purchasing the goods.
    AAC,
    /// Ocean charges rate
    ///
    /// The charges imposed by the ocean transportation industry above and beyond the basic freight.
    AAD,
    /// Not subject to fluctuation
    ///
    /// Not subject to escalation or adjustment.
    AAE,
    /// Subject to escalation
    ///
    /// Subject to increase or development by successive stages.
    AAF,
    /// Subject to price adjustment
    AAG,
    /// Subject to escalation and price adjustment
    ///
    /// Subject to increase or development by successive stages and price adjustment.
    AAH,
    /// Fluctuation conditions not specified
    AAI,
    /// All in price
    ///
    /// Firm price for specified work.
    AAJ,
    /// New price
    ///
    /// A price valid from an effective date/time/period.
    AAK,
    /// Old price
    ///
    /// A price valid prior to an effective date/time/period of a new price.
    AAL,
    /// Per week
    ///
    /// To indicate that the given price applies per week.
    AAM,
    /// Price on application
    ///
    /// Price can be obtained on request from seller.
    AAN,
    /// Unpacked price
    ///
    /// The price given is the price of the item without packaging.
    AAO,
    /// Trade price
    ///
    /// Discount price available to all customers except the retail customer.
    AAP,
    /// Firm price
    ///
    /// Price which will remain unchanged for a given time period.
    AAQ,
    /// Material share of item price
    ///
    /// The per unit cost of referenced material based on a given quotation for that material.
    AAR,
    /// Labour share of item price
    ///
    /// The labour component of the per-unit item price.
    AAS,
    /// Transport share of item price
    ///
    /// The transport component of the per-unit item price.
    AAT,
    /// Packing share of item price
    ///
    /// The packing component of the per-unit item price.
    AAU,
    /// Tooling share of item price
    ///
    /// The tooling component of the per-unit item price.
    AAV,
    /// Temporary vehicle charge
    ///
    /// The component of a price charged for providing a temporary vehicle.
    AAW,
    /// Price component due to interest
    ///
    /// This is the component of the price which is charged due to interest.
    AAX,
    /// Price component due to management services
    ///
    /// This is the component of the price which is charged due to management services rendered.
    AAY,
    /// Price component due to maintenance
    ///
    /// This is the component of the price which is charged due to maintenance.
    AAZ,
    /// Individual buyer price
    ///
    /// A price which is available to an individual buyer as opposed to an institutional buyer.
    ABA,
    /// Group buying price
    ///
    /// A price which is available to a buying group.
    ABB,
    /// Group member buying price
    ///
    /// A special price given to a member of a buying group.
    ABC,
    /// Pre-payment price
    ///
    /// A special price if pre-payment is made for the article ordered.
    ABD,
    /// Retail price - excluding taxes
    ///
    /// Retail price not including any applicable taxes.
    ABE,
    /// Suggested retail price - excluding taxes
    ///
    /// Suggested retail price not including any applicable taxes.
    ABF,
    /// Agreed minimum price
    ///
    /// The minimum price agreed between trading partners.
    ABG,
    /// Statutory minimum retail price
    ///
    /// The legal minimum retail price.
    ABH,
    /// Cost reimbursement price
    ///
    /// A code to indicate that the price represents the reimbursement of the actual costs incurred.
    ABI,
    /// Market price
    ///
    /// A code to indicate that the given price is applicable under normal competitive conditions.
    ABJ,
    /// Open tender price
    ///
    /// A code to indicate that the price mentioned has been submitted in the context of an open tender.
    ABK,
    /// Base price
    ///
    /// The base price of a product or service.
    ABL,
    /// Base price difference
    ///
    /// The difference in price against a base price.
    ABM,
    /// Adjustable price prior to acceptance
    ///
    /// A price which can be adjusted due to economic conditions between the date of offer and the date of acceptance.
    ABN,
    /// Revisable price after acceptance
    ///
    /// A price which can be revised due to economic conditions between the date of acceptance of the order and the date of delivery.
    ABO,
    /// Provisional ceiling price
    ///
    /// A provisional price which cannot be exceeded.
    ABP,
    /// Adjustable provisional ceiling price
    ///
    /// A provisional price which cannot be exceeded but which can be adjusted due to economic conditions between the date of offer and the date of acceptance.
    ABQ,
    /// Revisable provisional ceiling price
    ///
    /// A provisional price which cannot be exceeded but is revisable due to economic conditions between the date of acceptance of the order through to the date of delivery.
    ABR,
    /// Revisable provisional price
    ///
    /// A provisional price which is revisable due to economic conditions between the date of acceptance of the order and the date of delivery.
    ABS,
    /// Adjustable provisional price
    ///
    /// A provisional price which is adjustable due to economic conditions between the date of offer and the date of acceptance.
    ABT,
    /// Area price
    ///
    /// Price connected to a geographical area.
    ABU,
    /// Area system price
    ///
    /// A basis price applied to a geographic area.
    ABV,
    /// Active ingredient
    AI,
    /// Alternate price
    ///
    /// A substitute cost.
    ALT,
    /// Advice price
    AP,
    /// Broker price
    BR,
    /// Catalogue price
    ///
    /// Price per unit of quantity of a product as specified in a catalogue.
    CAT,
    /// Current domestic value
    ///
    /// The present worth of a thing which comes from one's homeland, in terms of money or goods.
    CDV,
    /// Contract price
    ///
    /// Price per unit of quantity of a product/service as agreed in a contract between parties.
    CON,
    /// Current price
    ///
    /// Price at time of transaction, but subject to future change.
    CP,
    /// Consumer unit
    CU,
    /// Confirmed unit price
    ///
    /// The value of a single item that proves to be correct.
    CUP,
    /// Declared customs unit value
    ///
    /// A clearly known duty on a single item which is imposed by law.
    CUS,
    /// Dealer adjusted price
    ///
    /// The necessary or desirable changes that the sales agency makes with respect to the value of the product.
    DAP,
    /// Distributor price
    ///
    /// The cost associated with the agency that markets goods.
    DIS,
    /// Discount price
    ///
    /// A reduction from the usual list value.
    DPR,
    /// Dealer price
    DR,
    /// Discount amount allowed
    ///
    /// A certain price up to which one is able to make reductions from the usual list value.
    DSC,
    /// ECSC price
    ///
    /// Price registered at European Commission Steel and Carbon office (DG III).
    EC,
    /// Estimated price
    ES,
    /// Expected unit price
    ///
    /// The anticipated value of a single item.
    EUP,
    /// Freight/charge rate
    ///
    /// The price that is either a freight rate or a rate on which freight charges are calculated.
    FCR,
    /// Gross unit price
    ///
    /// Unit price to which allowances and charges apply.
    GRP,
    /// Invoice price
    ///
    /// Price per unit of quantity of a product as specified on an invoice.
    INV,
    /// Labelling price
    ///
    /// Retail price of the buyer that should be printed by the producer on the article's label. The labelling price is not necessary the effective retail price.
    LBL,
    /// Maximum order quantity price
    ///
    /// The greatest amount of goods or services which one can buy to receive a certain value.
    MAX,
    /// Minimum order quantity price
    ///
    /// The least amount of goods or services that one can buy to receive a certain value.
    MIN,
    /// Minimum release quantity price
    ///
    /// The least amount of an order one can place in order to receive a certain value.
    MNR,
    /// Manufacturer's suggested retail
    ///
    /// Price that reflects "Sales to other manufacturers" or "Sales for resale".
    MSR,
    /// Maximum release quantity price
    ///
    /// The greatest amount of an order that one can place in order to receive a certain value.
    MXR,
    /// Not-to-exceed price
    NE,
    /// No quote
    ///
    /// No price available.
    NQT,
    /// Net unit price
    ///
    /// Unit price to which no allowances and charges apply.
    NTP,
    /// Net weight
    NW,
    /// Ocean charges rate
    ///
    /// The charges imposed by the ocean transportation industry above and beyond the basic freight.
    OCR,
    /// Ocean freight rate
    ///
    /// The price per pricing unit of ocean transportation services for moving cargo from one location to another.
    OFR,
    /// Price break quantity(s)
    ///
    /// Numerical amounts of goods or services which are associated with different sums of money. As the amount goes up, the price per individual item decreases.
    PAQ,
    /// Unit price beginning quantity
    ///
    /// The starting amount at which you can place a value on a single item.
    PBQ,
    /// Prepaid freight charges
    ///
    /// The cost of shipping is paid before the goods are shipped.
    PPD,
    /// Provisional price
    ///
    /// Price per unit of quantity of a product as provisionally agreed.
    PPR,
    /// Producer's price
    ///
    /// The value that the maker of a good places on an item.
    PRO,
    /// Promotional price
    ///
    /// The value that is placed on an item that is being developed. The idea is to sell this product for less than one normally would, and make up for it by selling a larger quantity.
    PRP,
    /// Gross weight
    PW,
    /// Quote price
    ///
    /// Price per unit of quantity of a product as specified in a quote.
    QTE,
    /// Resale price
    ///
    /// Price per unit of quantity of a product to be used for resale.
    RES,
    /// Retail price
    ///
    /// Price per unit of quantity of a product to be used for retail.
    RTP,
    /// Ship and debit
    ///
    /// To transport goods and be owed money by the customer for the services performed.
    SHD,
    /// Suggested retail price
    ///
    /// Price per unit of quantity of a product suggested for retail.
    SRP,
    /// Gross weight without wooden pallets
    ///
    /// Used in steel industry.
    SW,
    /// To be negotiated
    TB,
    /// Transfer
    ///
    /// To carry or remove from one place, situation, or person to another.
    TRF,
    /// Traded unit
    TU,
    /// Theoretical weight
    TW,
    /// Wholesale price
    ///
    /// Description to be provided.
    WH,
}

/// Quantity type code qualifier
///
/// Code qualifying the type of quantity.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _6063 {
    /// Discrete quantity
    ///
    ///
    #[strum(serialize = "1")]
    _1,
    /// Charge
    ///
    /// Quantity relevant for charge.
    #[strum(serialize = "2")]
    _2,

    /// Cumulative quantity
    ///
    ///
    #[strum(serialize = "3")]
    _3,
    /// Interest for overdrawn account
    ///
    /// Interest for overdrawing the account.
    #[strum(serialize = "4")]
    _4,

    /// Active ingredient dose per unit
    ///
    /// The dosage of active ingredient per unit.
    #[strum(serialize = "5")]
    _5,

    /// Auditor
    ///
    /// The number of entities that audit accounts.
    #[strum(serialize = "6")]
    _6,

    /// Branch locations, leased
    ///
    /// The number of branch locations being leased by an entity.
    #[strum(serialize = "7")]
    _7,

    /// Inventory quantity at supplier's subject to inspection by customer
    ///
    /// Quantity of goods which the customer requires the supplier to have in inventory and which may be inspected by the customer if desired.
    #[strum(serialize = "8")]
    _8,

    /// Branch locations, owned
    ///
    /// The number of branch locations owned by an entity.
    #[strum(serialize = "9")]
    _9,

    /// Judgements registered
    ///
    /// The number of judgements registered against an entity.
    #[strum(serialize = "10")]
    _10,

    /// Split quantity
    ///
    /// Part of the whole quantity.
    #[strum(serialize = "11")]
    _11,

    /// Despatch quantity
    ///
    /// Quantity despatched by the seller.
    #[strum(serialize = "12")]
    _12,

    /// Liens registered
    ///
    /// The number of liens registered against an entity.
    #[strum(serialize = "13")]
    _13,

    /// Livestock
    ///
    /// The number of animals kept for use or profit.
    #[strum(serialize = "14")]
    _14,

    /// Insufficient funds returned cheques
    ///
    /// The number of cheques returned due to insufficient funds.
    #[strum(serialize = "15")]
    _15,

    /// Stolen cheques
    ///
    /// The number of stolen cheques.
    #[strum(serialize = "16")]
    _16,

    /// Quantity on hand
    ///
    /// The total quantity of a product on hand at a location. This includes as well units awaiting return to manufacturer, units unavailable due to inspection procedures and undamaged stock available for despatch, resale or use.
    #[strum(serialize = "17")]
    _17,

    /// Previous quantity
    ///
    /// Quantity previously referenced.
    #[strum(serialize = "18")]
    _18,

    /// Paid-in security shares
    ///
    /// The number of security shares issued and for which full payment has been made.
    #[strum(serialize = "19")]
    _19,

    /// Unusable quantity
    ///
    ///
    #[strum(serialize = "20")]
    _20,
    /// Ordered quantity
    ///
    /// The quantity which has been ordered.
    #[strum(serialize = "21")]
    _21,

    /// Quantity at 100%
    ///
    /// Equivalent quantity at 100% purity.
    #[strum(serialize = "22")]
    _22,

    /// Active ingredient
    ///
    /// Quantity at 100% active agent content.
    #[strum(serialize = "23")]
    _23,

    /// Inventory quantity at supplier's not subject to inspection by customer
    ///
    /// Quantity of goods which the customer requires the supplier to have in inventory but which will not be checked by the customer.
    #[strum(serialize = "24")]
    _24,

    /// Retail sales
    ///
    /// Quantity of retail point of sale activity.
    #[strum(serialize = "25")]
    _25,

    /// Promotion quantity
    ///
    /// A quantity associated with a promotional event.
    #[strum(serialize = "26")]
    _26,

    /// On hold for shipment
    ///
    /// Article received which cannot be shipped in its present form.
    #[strum(serialize = "27")]
    _27,

    /// Military sales quantity
    ///
    /// Quantity of goods or services sold to a military organization.
    #[strum(serialize = "28")]
    _28,

    /// On premises sales
    ///
    /// Sale of product in restaurants or bars.
    #[strum(serialize = "29")]
    _29,

    /// Off premises sales
    ///
    /// Sale of product directly to a store.
    #[strum(serialize = "30")]
    _30,

    /// Estimated annual volume
    ///
    ///
    #[strum(serialize = "31")]
    _31,
    /// Minimum delivery batch
    ///
    ///
    #[strum(serialize = "32")]
    _32,
    /// Maximum delivery batch
    ///
    ///
    #[strum(serialize = "33")]
    _33,
    /// Pipes
    ///
    /// The number of tubes used to convey a substance.
    #[strum(serialize = "34")]
    _34,

    /// Price break from
    ///
    /// The minimum quantity of a quantity range for a specified (unit) price.
    #[strum(serialize = "35")]
    _35,

    /// Price break to
    ///
    /// Description to be provided.
    #[strum(serialize = "36")]
    _36,

    /// Poultry
    ///
    /// The number of domestic fowl.
    #[strum(serialize = "37")]
    _37,

    /// Secured charges registered
    ///
    /// The number of secured charges registered against an entity.
    #[strum(serialize = "38")]
    _38,

    /// Total properties owned
    ///
    /// The total number of properties owned by an entity.
    #[strum(serialize = "39")]
    _39,

    /// Normal delivery
    ///
    /// Quantity normally delivered by the seller.
    #[strum(serialize = "40")]
    _40,

    /// Sales quantity not included in the replenishment calculation
    ///
    /// Sales which will not be included in the calculation of replenishment requirements.
    #[strum(serialize = "41")]
    _41,

    /// Maximum supply quantity, supplier endorsed
    ///
    /// Maximum supply quantity endorsed by a supplier.
    #[strum(serialize = "42")]
    _42,

    /// Buyer
    ///
    /// The number of buyers.
    #[strum(serialize = "43")]
    _43,

    /// Debenture bond
    ///
    /// The number of fixed-interest bonds of an entity backed by general credit rather than specified assets.
    #[strum(serialize = "44")]
    _44,

    /// Debentures filed against directors
    ///
    /// The number of notices of indebtedness filed against an entity's directors.
    #[strum(serialize = "45")]
    _45,

    /// Pieces delivered
    ///
    /// Number of pieces actually received at the final destination.
    #[strum(serialize = "46")]
    _46,

    /// Invoiced quantity
    ///
    /// The quantity as per invoice.
    #[strum(serialize = "47")]
    _47,

    /// Received quantity
    ///
    /// The quantity which has been received.
    #[strum(serialize = "48")]
    _48,

    /// Chargeable distance
    ///
    /// Distance really charged by tariff appliance.
    #[strum(serialize = "49")]
    _49,

    /// Disposition undetermined quantity
    ///
    /// Product quantity that has not yet had its disposition determined.
    #[strum(serialize = "50")]
    _50,

    /// Inventory category transfer
    ///
    /// Inventory that has been moved from one inventory category to another.
    #[strum(serialize = "51")]
    _51,

    /// Quantity per pack
    ///
    ///
    #[strum(serialize = "52")]
    _52,
    /// Minimum order quantity
    ///
    ///
    #[strum(serialize = "53")]
    _53,
    /// Maximum order quantity
    ///
    ///
    #[strum(serialize = "54")]
    _54,
    /// Total sales
    ///
    /// The summation of total quantity sales.
    #[strum(serialize = "55")]
    _55,

    /// Wholesaler to wholesaler sales
    ///
    /// Sale of product to other wholesalers by a wholesaler.
    #[strum(serialize = "56")]
    _56,

    /// In transit quantity
    ///
    /// A quantity that is en route.
    #[strum(serialize = "57")]
    _57,

    /// Quantity withdrawn
    ///
    /// Quantity withdrawn from a location.
    #[strum(serialize = "58")]
    _58,

    /// Numbers of consumer units in the traded unit
    ///
    ///
    #[strum(serialize = "59")]
    _59,
    /// Current inventory quantity available for shipment
    ///
    /// Current inventory quantity available for shipment.
    #[strum(serialize = "60")]
    _60,

    /// Return quantity
    ///
    ///
    #[strum(serialize = "61")]
    _61,
    /// Sorted quantity
    ///
    /// Description to be provided.
    #[strum(serialize = "62")]
    _62,

    /// Sorted quantity rejected
    ///
    /// Description to be provided.
    #[strum(serialize = "63")]
    _63,

    /// Scrap quantity
    ///
    /// Remainder of the total quantity after split deliveries.
    #[strum(serialize = "64")]
    _64,

    /// Destroyed quantity
    ///
    ///
    #[strum(serialize = "65")]
    _65,
    /// Committed quantity
    ///
    /// Quantity a party is committed to.
    #[strum(serialize = "66")]
    _66,

    /// Estimated reading quantity
    ///
    /// The value that is estimated to be the reading of a measuring device (e.g. meter).
    #[strum(serialize = "67")]
    _67,

    /// End quantity
    ///
    /// The quantity recorded at the end of an agreement or period.
    #[strum(serialize = "68")]
    _68,

    /// Start quantity
    ///
    /// The quantity recorded at the start of an agreement or period.
    #[strum(serialize = "69")]
    _69,

    /// Cumulative quantity received
    ///
    /// Cumulative quantity of all deliveries of this article received by the buyer.
    #[strum(serialize = "70")]
    _70,

    /// Cumulative quantity ordered
    ///
    /// Cumulative quantity of all deliveries, outstanding and scheduled orders.
    #[strum(serialize = "71")]
    _71,

    /// Cumulative quantity received end of prior year
    ///
    /// Cumulative quantity of all deliveries of the product received by the buyer till end of prior year.
    #[strum(serialize = "72")]
    _72,

    /// Outstanding quantity
    ///
    /// Difference between quantity ordered and quantity received.
    #[strum(serialize = "73")]
    _73,

    /// Latest cumulative quantity
    ///
    /// Cumulative quantity after complete delivery of all scheduled quantities of the product.
    #[strum(serialize = "74")]
    _74,

    /// Previous highest cumulative quantity
    ///
    /// Cumulative quantity after complete delivery of all scheduled quantities of the product from a prior schedule period.
    #[strum(serialize = "75")]
    _75,

    /// Adjusted corrector reading
    ///
    /// A corrector reading after it has been adjusted.
    #[strum(serialize = "76")]
    _76,

    /// Work days
    ///
    /// Number of work days, e.g. per respective period.
    #[strum(serialize = "77")]
    _77,

    /// Cumulative quantity scheduled
    ///
    /// Adding the quantity actually scheduled to previous cumulative quantity.
    #[strum(serialize = "78")]
    _78,

    /// Previous cumulative quantity
    ///
    /// Cumulative quantity prior the actual order.
    #[strum(serialize = "79")]
    _79,

    /// Unadjusted corrector reading
    ///
    /// A corrector reading before it has been adjusted.
    #[strum(serialize = "80")]
    _80,

    /// Extra unplanned delivery
    ///
    /// Non scheduled additional quantity.
    #[strum(serialize = "81")]
    _81,

    /// Quantity requirement for sample inspection
    ///
    ///
    #[strum(serialize = "82")]
    _82,
    /// Backorder quantity
    ///
    ///
    #[strum(serialize = "83")]
    _83,
    /// Urgent delivery quantity
    ///
    ///
    #[strum(serialize = "84")]
    _84,
    /// Previous order quantity to be cancelled
    ///
    ///
    #[strum(serialize = "85")]
    _85,
    /// Normal reading quantity
    ///
    /// The value recorded or read from a measuring device (e.g. meter) in the normal conditions.
    #[strum(serialize = "86")]
    _86,

    /// Customer reading quantity
    ///
    /// The value recorded or read from a measuring device (e.g. meter) by the customer.
    #[strum(serialize = "87")]
    _87,

    /// Information reading quantity
    ///
    /// The value recorded or read from a measuring device (e.g. meter) for information purposes.
    #[strum(serialize = "88")]
    _88,

    /// Quality control held
    ///
    /// Quantity of goods held pending completion of a quality control assessment.
    #[strum(serialize = "89")]
    _89,

    /// As is quantity
    ///
    ///
    #[strum(serialize = "90")]
    _90,
    /// Open quantity
    ///
    /// Quantity remaining after partial delivery.
    #[strum(serialize = "91")]
    _91,

    /// Final delivery quantity
    ///
    /// Quantity of final delivery to a respective order.
    #[strum(serialize = "92")]
    _92,

    /// Subsequent delivery quantity
    ///
    /// Quantity delivered to a respective order after it's final delivery.
    #[strum(serialize = "93")]
    _93,

    /// Substitutional quantity
    ///
    /// Quantity delivered replacing previous deliveries.
    #[strum(serialize = "94")]
    _94,

    /// Redelivery after post processing
    ///
    ///
    #[strum(serialize = "95")]
    _95,
    /// Quality control failed
    ///
    /// Quantity of goods which have failed quality control.
    #[strum(serialize = "96")]
    _96,

    /// Minimum inventory
    ///
    /// Minimum stock quantity on which replenishment is based.
    #[strum(serialize = "97")]
    _97,

    /// Maximum inventory
    ///
    /// Maximum stock quantity on which replenishment is based.
    #[strum(serialize = "98")]
    _98,

    /// Estimated quantity
    ///
    ///
    #[strum(serialize = "99")]
    _99,
    /// Chargeable weight
    ///
    /// The weight on which charges are based.
    #[strum(serialize = "100")]
    _100,

    /// Chargeable gross weight
    ///
    /// The gross weight on which charges are based.
    #[strum(serialize = "101")]
    _101,

    /// Chargeable tare weight
    ///
    /// The tare weight on which charges are based.
    #[strum(serialize = "102")]
    _102,

    /// Chargeable number of axles
    ///
    /// The number of axles on which charges are based.
    #[strum(serialize = "103")]
    _103,

    /// Chargeable number of containers
    ///
    /// The number of containers on which charges are based.
    #[strum(serialize = "104")]
    _104,

    /// Chargeable number of rail wagons
    ///
    /// The number of rail wagons on which charges are based.
    #[strum(serialize = "105")]
    _105,

    /// Chargeable number of packages
    ///
    /// The number of packages on which charges are based.
    #[strum(serialize = "106")]
    _106,

    /// Chargeable number of units
    ///
    /// The number of units on which charges are based.
    #[strum(serialize = "107")]
    _107,

    /// Chargeable period
    ///
    /// The period of time on which charges are based.
    #[strum(serialize = "108")]
    _108,

    /// Chargeable volume
    ///
    /// The volume on which charges are based.
    #[strum(serialize = "109")]
    _109,

    /// Chargeable cubic measurements
    ///
    /// The cubic measurements on which charges are based.
    #[strum(serialize = "110")]
    _110,

    /// Chargeable surface
    ///
    /// The surface area on which charges are based.
    #[strum(serialize = "111")]
    _111,

    /// Chargeable length
    ///
    /// The length on which charges are based.
    #[strum(serialize = "112")]
    _112,

    /// Quantity to be delivered
    ///
    /// The quantity to be delivered.
    #[strum(serialize = "113")]
    _113,

    /// Number of passengers
    ///
    /// Total number of passengers on the conveyance.
    #[strum(serialize = "114")]
    _114,

    /// Number of crew
    ///
    /// Total number of crew members on the conveyance.
    #[strum(serialize = "115")]
    _115,

    /// Number of transport documents
    ///
    /// Total number of air waybills, bills of lading, etc. being reported for a specific conveyance.
    #[strum(serialize = "116")]
    _116,

    /// Quantity landed
    ///
    /// Quantity of goods actually arrived.
    #[strum(serialize = "117")]
    _117,

    /// Quantity manifested
    ///
    /// Quantity of goods contracted for delivery by the carrier.
    #[strum(serialize = "118")]
    _118,

    /// Short shipped
    ///
    /// Indication that part of the consignment was not shipped.
    #[strum(serialize = "119")]
    _119,

    /// Split shipment
    ///
    /// Indication that the consignment has been split into two or more shipments.
    #[strum(serialize = "120")]
    _120,

    /// Over shipped
    ///
    /// Indication that more goods have been shipped than contracted for delivery.
    #[strum(serialize = "121")]
    _121,

    /// Short-landed goods
    ///
    /// If quantity of goods actually landed is less than the quantity which appears in the documentation. This quantity is the difference between these quantities.
    #[strum(serialize = "122")]
    _122,

    /// Surplus goods
    ///
    /// If quantity of goods actually landed is more than the quantity which appears in the documentation. This quantity is the difference between these quantities.
    #[strum(serialize = "123")]
    _123,

    /// Damaged goods
    ///
    /// Quantity of goods which have deteriorated in transport such that they cannot be used for the purpose for which they were originally intended.
    #[strum(serialize = "124")]
    _124,

    /// Pilferage goods
    ///
    /// Quantity of goods stolen during transport.
    #[strum(serialize = "125")]
    _125,

    /// Lost goods
    ///
    /// Quantity of goods that disappeared in transport.
    #[strum(serialize = "126")]
    _126,

    /// Report difference
    ///
    /// The quantity concerning the same transaction differs between two documents/messages and the source of this difference is a typing error.
    #[strum(serialize = "127")]
    _127,

    /// Quantity loaded
    ///
    /// Quantity of goods loaded onto a means of transport.
    #[strum(serialize = "128")]
    _128,

    /// Units per unit price
    ///
    /// Number of units per unit price.
    #[strum(serialize = "129")]
    _129,

    /// Allowance
    ///
    /// Quantity relevant for allowance.
    #[strum(serialize = "130")]
    _130,

    /// Delivery quantity
    ///
    /// Quantity required by buyer to be delivered.
    #[strum(serialize = "131")]
    _131,

    /// Cumulative quantity, preceding period, planned
    ///
    /// Cumulative quantity originally planned for the preceding period.
    #[strum(serialize = "132")]
    _132,

    /// Cumulative quantity, preceding period, reached
    ///
    /// Cumulative quantity reached in the preceding period.
    #[strum(serialize = "133")]
    _133,

    /// Cumulative quantity, actual planned
    ///
    /// Cumulative quantity planned for now.
    #[strum(serialize = "134")]
    _134,

    /// Period quantity, planned
    ///
    /// Quantity planned for this period.
    #[strum(serialize = "135")]
    _135,

    /// Period quantity, reached
    ///
    /// Quantity reached during this period.
    #[strum(serialize = "136")]
    _136,

    /// Cumulative quantity, preceding period, estimated
    ///
    /// Estimated cumulative quantity reached in the preceding period.
    #[strum(serialize = "137")]
    _137,

    /// Cumulative quantity, actual estimated
    ///
    /// Estimated cumulative quantity reached now.
    #[strum(serialize = "138")]
    _138,

    /// Cumulative quantity, preceding period, measured
    ///
    /// Surveyed cumulative quantity reached in the preceding period.
    #[strum(serialize = "139")]
    _139,

    /// Cumulative quantity, actual measured
    ///
    /// Surveyed cumulative quantity reached now.
    #[strum(serialize = "140")]
    _140,

    /// Period quantity, measured
    ///
    /// Surveyed quantity reached during this period.
    #[strum(serialize = "141")]
    _141,

    /// Total quantity, planned
    ///
    /// Total quantity planned.
    #[strum(serialize = "142")]
    _142,

    /// Quantity, remaining
    ///
    /// Quantity remaining.
    #[strum(serialize = "143")]
    _143,

    /// Tolerance
    ///
    /// Plus or minus tolerance expressed as a monetary amount.
    #[strum(serialize = "144")]
    _144,

    /// Actual stock
    ///
    /// The stock on hand, undamaged, and available for despatch, sale or use.
    #[strum(serialize = "145")]
    _145,

    /// Model or target stock
    ///
    /// The stock quantity required or planned to have on hand, undamaged and available for use.
    #[strum(serialize = "146")]
    _146,

    /// Direct shipment quantity
    ///
    /// Quantity to be shipped directly to a customer from a manufacturing site.
    #[strum(serialize = "147")]
    _147,

    /// Amortization total quantity
    ///
    /// Indication of final quantity for amortization.
    #[strum(serialize = "148")]
    _148,

    /// Amortization order quantity
    ///
    /// Indication of actual share of the order quantity for amortization.
    #[strum(serialize = "149")]
    _149,

    /// Amortization cumulated quantity
    ///
    /// Indication of actual cumulated quantity of previous and actual amortization order quantity.
    #[strum(serialize = "150")]
    _150,

    /// Quantity advised
    ///
    /// Quantity advised by supplier or shipper, in contrast to quantity actually received.
    #[strum(serialize = "151")]
    _151,

    /// Quantity on hand
    ///
    /// The total quantity of a product on hand at a location. This includes as well units awaiting return to manufacturer, units unavailable due to inspection procedures and undamaged stock available for despatch, resale or use.
    #[strum(serialize = "152")]
    _152,

    /// Statistical sales quantity
    ///
    /// Quantity of goods sold in a specified period.
    #[strum(serialize = "153")]
    _153,

    /// Sales quantity planned
    ///
    /// Quantity of goods required to meet future demands. - Market intelligence quantity.
    #[strum(serialize = "154")]
    _154,

    /// Replenishment quantity
    ///
    /// Quantity required to maintain the requisite on-hand stock of goods.
    #[strum(serialize = "155")]
    _155,

    /// Inventory movement quantity
    ///
    /// To specify the quantity of an inventory movement.
    #[strum(serialize = "156")]
    _156,

    /// Opening stock balance quantity
    ///
    /// To specify the quantity of an opening stock balance.
    #[strum(serialize = "157")]
    _157,

    /// Closing stock balance quantity
    ///
    /// To specify the quantity of a closing stock balance.
    #[strum(serialize = "158")]
    _158,

    /// Number of stops
    ///
    /// Number of times a means of transport stops before arriving at destination.
    #[strum(serialize = "159")]
    _159,

    /// Minimum production batch
    ///
    /// The quantity specified is the minimum output from a single production run.
    #[strum(serialize = "160")]
    _160,

    /// Dimensional sample quantity
    ///
    /// The quantity defined is a sample for the purpose of validating dimensions.
    #[strum(serialize = "161")]
    _161,

    /// Functional sample quantity
    ///
    /// The quantity defined is a sample for the purpose of validating function and performance.
    #[strum(serialize = "162")]
    _162,

    /// Pre-production quantity
    ///
    /// Quantity of the referenced item required prior to full production.
    #[strum(serialize = "163")]
    _163,

    /// Delivery batch
    ///
    /// Quantity of the referenced item which constitutes a standard batch for deliver purposes.
    #[strum(serialize = "164")]
    _164,

    /// Delivery batch multiple
    ///
    /// The multiples in which delivery batches can be supplied.
    #[strum(serialize = "165")]
    _165,

    /// All time buy
    ///
    /// The total quantity of the referenced covering all future needs. Further orders of the referenced item are not expected.
    #[strum(serialize = "166")]
    _166,

    /// Total delivery quantity
    ///
    /// The total quantity required by the buyer to be delivered.
    #[strum(serialize = "167")]
    _167,

    /// Single delivery quantity
    ///
    /// The quantity required by the buyer to be delivered in a single shipment.
    #[strum(serialize = "168")]
    _168,

    /// Supplied quantity
    ///
    /// Quantity of the referenced item actually shipped.
    #[strum(serialize = "169")]
    _169,

    /// Allocated quantity
    ///
    /// Quantity of the referenced item allocated from available stock for delivery.
    #[strum(serialize = "170")]
    _170,

    /// Maximum stackability
    ///
    /// The number of pallets/handling units which can be safely stacked one on top of another.
    #[strum(serialize = "171")]
    _171,

    /// Amortisation quantity
    ///
    /// The quantity of the referenced item which has a cost for tooling amortisation included in the item price.
    #[strum(serialize = "172")]
    _172,

    /// Previously amortised quantity
    ///
    /// The cumulative quantity of the referenced item which had a cost for tooling amortisation included in the item price.
    #[strum(serialize = "173")]
    _173,

    /// Total amortisation quantity
    ///
    /// The total quantity of the referenced item which has a cost for tooling amortisation included in the item price.
    #[strum(serialize = "174")]
    _174,

    /// Number of moulds
    ///
    /// The number of pressing moulds contained within a single piece of the referenced tooling.
    #[strum(serialize = "175")]
    _175,

    /// Concurrent item output of tooling
    ///
    /// The number of related items which can be produced simultaneously with a single piece of the referenced tooling.
    #[strum(serialize = "176")]
    _176,

    /// Periodic capacity of tooling
    ///
    /// Maximum production output of the referenced tool over a period of time.
    #[strum(serialize = "177")]
    _177,

    /// Lifetime capacity of tooling
    ///
    /// Maximum production output of the referenced tool over its productive lifetime.
    #[strum(serialize = "178")]
    _178,

    /// Number of deliveries per despatch period
    ///
    /// The number of deliveries normally expected to be despatched within each despatch period.
    #[strum(serialize = "179")]
    _179,

    /// Provided quantity
    ///
    /// The quantity of a referenced component supplied by the buyer for manufacturing of an ordered item.
    #[strum(serialize = "180")]
    _180,

    /// Maximum production batch
    ///
    /// The quantity specified is the maximum output from a single production run.
    #[strum(serialize = "181")]
    _181,

    /// Cancelled quantity
    ///
    /// Quantity of the referenced item which has previously been ordered and is now cancelled.
    #[strum(serialize = "182")]
    _182,

    /// No delivery requirement in this instruction
    ///
    /// This delivery instruction does not contain any delivery requirements.
    #[strum(serialize = "183")]
    _183,

    /// Quantity of material in ordered time
    ///
    /// Quantity of the referenced material within the ordered time.
    #[strum(serialize = "184")]
    _184,

    /// Rejected quantity
    ///
    /// The quantity of received goods rejected for quantity reasons.
    #[strum(serialize = "185")]
    _185,

    /// Cumulative quantity scheduled up to accumulation start date
    ///
    /// The cumulative quantity scheduled up to the accumulation start date.
    #[strum(serialize = "186")]
    _186,

    /// Quantity scheduled
    ///
    /// The quantity scheduled for delivery.
    #[strum(serialize = "187")]
    _187,

    /// Number of identical handling units
    ///
    /// Number of identical handling units in terms of type and contents.
    #[strum(serialize = "188")]
    _188,

    /// Number of packages in handling unit
    ///
    /// The number of packages contained in one handling unit.
    #[strum(serialize = "189")]
    _189,

    /// Despatch note quantity
    ///
    /// The item quantity specified on the despatch note.
    #[strum(serialize = "190")]
    _190,

    /// Adjustment to inventory quantity
    ///
    /// An adjustment to inventory quantity.
    #[strum(serialize = "191")]
    _191,

    /// Free goods quantity
    ///
    /// Quantity of goods which are free of charge.
    #[strum(serialize = "192")]
    _192,

    /// Free quantity included
    ///
    /// Quantity included to which no charge is applicable.
    #[strum(serialize = "193")]
    _193,

    /// Received and accepted
    ///
    /// Quantity which has been received and accepted at a given location.
    #[strum(serialize = "194")]
    _194,

    /// Received, not accepted, to be returned
    ///
    /// Quantity which has been received but not accepted at a given location and which will consequently be returned to the relevant party.
    #[strum(serialize = "195")]
    _195,

    /// Received, not accepted, to be destroyed
    ///
    /// Quantity which has been received but not accepted at a given location and which will consequently be destroyed.
    #[strum(serialize = "196")]
    _196,

    /// Reordering level
    ///
    /// Quantity at which an order may be triggered to replenish.
    #[strum(serialize = "197")]
    _197,

    /// Quantity in transit
    ///
    /// Quantity which is currently in transit.
    #[strum(serialize = "198")]
    _198,

    /// Inventory withdrawal quantity
    ///
    /// Quantity which has been withdrawn from inventory since the last inventory report.
    #[strum(serialize = "199")]
    _199,

    /// Free quantity not included
    ///
    /// Free quantity not included in ordered quantity.
    #[strum(serialize = "200")]
    _200,

    /// Recommended overhaul and repair quantity
    ///
    /// To indicate the recommended quantity of an article required to support overhaul and repair activities.
    #[strum(serialize = "201")]
    _201,

    /// Quantity per next higher assembly
    ///
    /// To indicate the quantity required for the next higher assembly.
    #[strum(serialize = "202")]
    _202,

    /// Quantity per unit of issue
    ///
    /// Provides the standard quantity of an article in which one unit can be issued.
    #[strum(serialize = "203")]
    _203,

    /// Cumulative scrap quantity
    ///
    /// Provides the cumulative quantity of an item which has been identified as scrapped.
    #[strum(serialize = "204")]
    _204,

    /// Publication turn size
    ///
    /// The quantity of magazines or newspapers grouped together with the spine facing alternate directions in a bundle.
    #[strum(serialize = "205")]
    _205,

    /// Recommended maintenance quantity
    ///
    /// Recommended quantity of an article which is required to meet an agreed level of maintenance.
    #[strum(serialize = "206")]
    _206,

    /// Labour hours
    ///
    /// Number of labour hours.
    #[strum(serialize = "207")]
    _207,

    /// Quantity requirement for maintenance and repair of equipment
    ///
    /// Quantity of the material needed to maintain and repair equipment.
    #[strum(serialize = "208")]
    _208,

    /// Additional replenishment demand quantity
    ///
    /// Incremental needs over and above normal replenishment calculations, but not intended to permanently change the model parameters.
    #[strum(serialize = "209")]
    _209,

    /// Returned by consumer quantity
    ///
    /// Quantity returned by a consumer.
    #[strum(serialize = "210")]
    _210,

    /// Replenishment override quantity
    ///
    /// Quantity to override the normal replenishment model calculations, but not intended to permanently change the model parameters.
    #[strum(serialize = "211")]
    _211,

    /// Quantity sold, net
    ///
    /// Net quantity sold which includes returns of saleable inventory and other adjustments.
    #[strum(serialize = "212")]
    _212,

    /// Transferred out quantity
    ///
    /// Quantity which was transferred out of this location.
    #[strum(serialize = "213")]
    _213,

    /// Transferred in quantity
    ///
    /// Quantity which was transferred into this location.
    #[strum(serialize = "214")]
    _214,

    /// Unsaleable quantity
    ///
    /// Quantity of inventory received which cannot be sold in its present condition.
    #[strum(serialize = "215")]
    _215,

    /// Consumer reserved quantity
    ///
    /// Quantity reserved for consumer delivery or pickup and not yet withdrawn from inventory.
    #[strum(serialize = "216")]
    _216,

    /// Out of inventory quantity
    ///
    /// Quantity of inventory which was requested but was not available.
    #[strum(serialize = "217")]
    _217,

    /// Quantity returned, defective or damaged
    ///
    /// Quantity returned in a damaged or defective condition.
    #[strum(serialize = "218")]
    _218,

    /// Taxable quantity
    ///
    /// Quantity subject to taxation.
    #[strum(serialize = "219")]
    _219,

    /// Meter reading
    ///
    /// The numeric value of measure units counted by a meter.
    #[strum(serialize = "220")]
    _220,

    /// Maximum requestable quantity
    ///
    /// The maximum quantity which may be requested.
    #[strum(serialize = "221")]
    _221,

    /// Minimum requestable quantity
    ///
    /// The minimum quantity which may be requested.
    #[strum(serialize = "222")]
    _222,

    /// Daily average quantity
    ///
    /// The quantity for a defined period divided by the number of days of the period.
    #[strum(serialize = "223")]
    _223,

    /// Budgeted hours
    ///
    /// The number of budgeted hours.
    #[strum(serialize = "224")]
    _224,

    /// Actual hours
    ///
    /// The number of actual hours.
    #[strum(serialize = "225")]
    _225,

    /// Earned value hours
    ///
    /// The number of earned value hours.
    #[strum(serialize = "226")]
    _226,

    /// Estimated hours
    ///
    /// The number of estimated hours.
    #[strum(serialize = "227")]
    _227,

    /// Level resource task quantity
    ///
    /// Quantity of a resource that is level for the duration of the task.
    #[strum(serialize = "228")]
    _228,

    /// Available resource task quantity
    ///
    /// Quantity of a resource available to complete a task.
    #[strum(serialize = "229")]
    _229,

    /// Work time units
    ///
    /// Quantity of work units of time.
    #[strum(serialize = "230")]
    _230,

    /// Daily work shifts
    ///
    /// Quantity of work shifts per day.
    #[strum(serialize = "231")]
    _231,

    /// Work time units per shift
    ///
    /// Work units of time per work shift.
    #[strum(serialize = "232")]
    _232,

    /// Work calendar units
    ///
    /// Work calendar units of time.
    #[strum(serialize = "233")]
    _233,

    /// Elapsed duration
    ///
    /// Quantity representing the elapsed duration.
    #[strum(serialize = "234")]
    _234,

    /// Remaining duration
    ///
    /// Quantity representing the remaining duration.
    #[strum(serialize = "235")]
    _235,

    /// Original duration
    ///
    /// Quantity representing the original duration.
    #[strum(serialize = "236")]
    _236,

    /// Current duration
    ///
    /// Quantity representing the current duration.
    #[strum(serialize = "237")]
    _237,

    /// Total float time
    ///
    /// Quantity representing the total float time.
    #[strum(serialize = "238")]
    _238,

    /// Free float time
    ///
    /// Quantity representing the free float time.
    #[strum(serialize = "239")]
    _239,

    /// Lag time
    ///
    /// Quantity representing lag time.
    #[strum(serialize = "240")]
    _240,

    /// Lead time
    ///
    /// Quantity representing lead time.
    #[strum(serialize = "241")]
    _241,

    /// Number of months
    ///
    /// The number of months.
    #[strum(serialize = "242")]
    _242,

    /// Reserved quantity customer direct delivery sales
    ///
    /// Quantity of products reserved for sales delivered direct to the customer.
    #[strum(serialize = "243")]
    _243,

    /// Reserved quantity retail sales
    ///
    /// Quantity of products reserved for retail sales.
    #[strum(serialize = "244")]
    _244,

    /// Consolidated discount inventory
    ///
    /// A quantity of inventory supplied at consolidated discount terms.
    #[strum(serialize = "245")]
    _245,

    /// Returns replacement quantity
    ///
    /// A quantity of goods issued as a replacement for a returned quantity.
    #[strum(serialize = "246")]
    _246,

    /// Additional promotion sales forecast quantity
    ///
    /// A forecast of additional quantity which will be sold during a period of promotional activity.
    #[strum(serialize = "247")]
    _247,

    /// Reserved quantity
    ///
    /// Quantity reserved for specific purposes.
    #[strum(serialize = "248")]
    _248,

    /// Quantity displayed not available for sale
    ///
    /// Quantity displayed within a retail outlet but not available for sale.
    #[strum(serialize = "249")]
    _249,

    /// Inventory discrepancy
    ///
    /// The difference recorded between theoretical and physical inventory.
    #[strum(serialize = "250")]
    _250,

    /// Incremental order quantity
    ///
    /// The incremental quantity by which ordering is carried out.
    #[strum(serialize = "251")]
    _251,

    /// Quantity requiring manipulation before despatch
    ///
    /// A quantity of goods which needs manipulation before despatch.
    #[strum(serialize = "252")]
    _252,

    /// Quantity in quarantine
    ///
    /// A quantity of goods which are held in a restricted area for quarantine purposes.
    #[strum(serialize = "253")]
    _253,

    /// Quantity withheld by owner of goods
    ///
    /// A quantity of goods which has been withheld by the owner of the goods.
    #[strum(serialize = "254")]
    _254,

    /// Quantity not available for despatch
    ///
    /// A quantity of goods not available for despatch.
    #[strum(serialize = "255")]
    _255,

    /// Quantity awaiting delivery
    ///
    /// Quantity of goods which are awaiting delivery.
    #[strum(serialize = "256")]
    _256,

    /// Quantity in physical inventory
    ///
    /// A quantity of goods held in physical inventory.
    #[strum(serialize = "257")]
    _257,

    /// Quantity held by logistic service provider
    ///
    /// Quantity of goods under the control of a logistic service provider.
    #[strum(serialize = "258")]
    _258,

    /// Optimal quantity
    ///
    /// The optimal quantity for a given purpose.
    #[strum(serialize = "259")]
    _259,

    /// Delivery quantity balance
    ///
    /// The difference between the scheduled quantity and the quantity delivered to the consignee at a given date.
    #[strum(serialize = "260")]
    _260,

    /// Cumulative quantity shipped
    ///
    /// Cumulative quantity of all shipments.
    #[strum(serialize = "261")]
    _261,

    /// Quantity suspended
    ///
    /// The quantity of something which is suspended.
    #[strum(serialize = "262")]
    _262,

    /// Control quantity
    ///
    /// The quantity designated for control purposes.
    #[strum(serialize = "263")]
    _263,

    /// Equipment quantity
    ///
    /// A count of a quantity of equipment.
    #[strum(serialize = "264")]
    _264,

    /// Factor
    ///
    /// Number by which the measured unit has to be multiplied to calculate the units used.
    #[strum(serialize = "265")]
    _265,

    /// Unsold quantity held by wholesaler
    ///
    /// Unsold quantity held by the wholesaler.
    #[strum(serialize = "266")]
    _266,

    /// Quantity held by delivery vehicle
    ///
    /// Quantity of goods held by the delivery vehicle.
    #[strum(serialize = "267")]
    _267,

    /// Quantity held by retail outlet
    ///
    /// Quantity held by the retail outlet.
    #[strum(serialize = "268")]
    _268,

    /// Rejected return quantity
    ///
    /// A quantity for return which has been rejected.
    #[strum(serialize = "269")]
    _269,

    /// Accounts
    ///
    /// The number of accounts.
    #[strum(serialize = "270")]
    _270,

    /// Accounts placed for collection
    ///
    /// The number of accounts placed for collection.
    #[strum(serialize = "271")]
    _271,

    /// Activity codes
    ///
    /// The number of activity codes.
    #[strum(serialize = "272")]
    _272,

    /// Agents
    ///
    /// The number of agents.
    #[strum(serialize = "273")]
    _273,

    /// Airline attendants
    ///
    /// The number of airline attendants.
    #[strum(serialize = "274")]
    _274,

    /// Authorised shares
    ///
    /// The number of shares authorised for issue.
    #[strum(serialize = "275")]
    _275,

    /// Employee average
    ///
    /// The average number of employees.
    #[strum(serialize = "276")]
    _276,

    /// Branch locations
    ///
    /// The number of branch locations.
    #[strum(serialize = "277")]
    _277,

    /// Capital changes
    ///
    /// The number of capital changes made.
    #[strum(serialize = "278")]
    _278,

    /// Clerks
    ///
    /// The number of clerks.
    #[strum(serialize = "279")]
    _279,

    /// Companies in same activity
    ///
    /// The number of companies doing business in the same activity category.
    #[strum(serialize = "280")]
    _280,

    /// Companies included in consolidated financial statement
    ///
    /// The number of companies included in a consolidated financial statement.
    #[strum(serialize = "281")]
    _281,

    /// Cooperative shares
    ///
    /// The number of cooperative shares.
    #[strum(serialize = "282")]
    _282,

    /// Creditors
    ///
    /// The number of creditors.
    #[strum(serialize = "283")]
    _283,

    /// Departments
    ///
    /// The number of departments.
    #[strum(serialize = "284")]
    _284,

    /// Design employees
    ///
    /// The number of employees involved in the design process.
    #[strum(serialize = "285")]
    _285,

    /// Physicians
    ///
    /// The number of medical doctors.
    #[strum(serialize = "286")]
    _286,

    /// Domestic affiliated companies
    ///
    /// The number of affiliated companies located within the country.
    #[strum(serialize = "287")]
    _287,

    /// Drivers
    ///
    /// The number of drivers.
    #[strum(serialize = "288")]
    _288,

    /// Employed at location
    ///
    /// The number of employees at the specified location.
    #[strum(serialize = "289")]
    _289,

    /// Employed by this company
    ///
    /// The number of employees at the specified company.
    #[strum(serialize = "290")]
    _290,

    /// Total employees
    ///
    /// The total number of employees.
    #[strum(serialize = "291")]
    _291,

    /// Employees shared
    ///
    /// The number of employees shared among entities.
    #[strum(serialize = "292")]
    _292,

    /// Engineers
    ///
    /// The number of engineers.
    #[strum(serialize = "293")]
    _293,

    /// Estimated accounts
    ///
    /// The estimated number of accounts.
    #[strum(serialize = "294")]
    _294,

    /// Estimated employees at location
    ///
    /// The estimated number of employees at the specified location.
    #[strum(serialize = "295")]
    _295,

    /// Estimated total employees
    ///
    /// The total estimated number of employees.
    #[strum(serialize = "296")]
    _296,

    /// Executives
    ///
    /// The number of executives.
    #[strum(serialize = "297")]
    _297,

    /// Agricultural workers
    ///
    /// The number of agricultural workers.
    #[strum(serialize = "298")]
    _298,

    /// Financial institutions
    ///
    /// The number of financial institutions.
    #[strum(serialize = "299")]
    _299,

    /// Floors occupied
    ///
    /// The number of floors occupied.
    #[strum(serialize = "300")]
    _300,

    /// Foreign related entities
    ///
    /// The number of related entities located outside the country.
    #[strum(serialize = "301")]
    _301,

    /// Group employees
    ///
    /// The number of employees within the group.
    #[strum(serialize = "302")]
    _302,

    /// Indirect employees
    ///
    /// The number of employees not associated with direct production.
    #[strum(serialize = "303")]
    _303,

    /// Installers
    ///
    /// The number of employees involved with the installation process.
    #[strum(serialize = "304")]
    _304,

    /// Invoices
    ///
    /// The number of invoices.
    #[strum(serialize = "305")]
    _305,

    /// Issued shares
    ///
    /// The number of shares actually issued.
    #[strum(serialize = "306")]
    _306,

    /// Labourers
    ///
    /// The number of labourers.
    #[strum(serialize = "307")]
    _307,

    /// Manufactured units
    ///
    /// The number of units manufactured.
    #[strum(serialize = "308")]
    _308,

    /// Maximum number of employees
    ///
    /// The maximum number of people employed.
    #[strum(serialize = "309")]
    _309,

    /// Maximum number of employees at location
    ///
    /// The maximum number of people employed at a location.
    #[strum(serialize = "310")]
    _310,

    /// Members in group
    ///
    /// The number of members within a group.
    #[strum(serialize = "311")]
    _311,

    /// Minimum number of employees at location
    ///
    /// The minimum number of people employed at a location.
    #[strum(serialize = "312")]
    _312,

    /// Minimum number of employees
    ///
    /// The minimum number of people employed.
    #[strum(serialize = "313")]
    _313,

    /// Non-union employees
    ///
    /// The number of employees not belonging to a labour union.
    #[strum(serialize = "314")]
    _314,

    /// Floors
    ///
    /// The number of floors in a building.
    #[strum(serialize = "315")]
    _315,

    /// Nurses
    ///
    /// The number of nurses.
    #[strum(serialize = "316")]
    _316,

    /// Office workers
    ///
    /// The number of workers in an office.
    #[strum(serialize = "317")]
    _317,

    /// Other employees
    ///
    /// The number of employees otherwise categorised.
    #[strum(serialize = "318")]
    _318,

    /// Part time employees
    ///
    /// The number of employees working on a part time basis.
    #[strum(serialize = "319")]
    _319,

    /// Accounts payable average overdue days
    ///
    /// The average number of days accounts payable are overdue.
    #[strum(serialize = "320")]
    _320,

    /// Pilots
    ///
    /// The number of pilots.
    #[strum(serialize = "321")]
    _321,

    /// Plant workers
    ///
    /// The number of workers within a plant.
    #[strum(serialize = "322")]
    _322,

    /// Previous number of accounts
    ///
    /// The number of accounts which preceded the current count.
    #[strum(serialize = "323")]
    _323,

    /// Previous number of branch locations
    ///
    /// The number of branch locations which preceded the current count.
    #[strum(serialize = "324")]
    _324,

    /// Principals included as employees
    ///
    /// The number of principals which are included in the count of employees.
    #[strum(serialize = "325")]
    _325,

    /// Protested bills
    ///
    /// The number of bills which are protested.
    #[strum(serialize = "326")]
    _326,

    /// Registered brands distributed
    ///
    /// The number of registered brands which are being distributed.
    #[strum(serialize = "327")]
    _327,

    /// Registered brands manufactured
    ///
    /// The number of registered brands which are being manufactured.
    #[strum(serialize = "328")]
    _328,

    /// Related business entities
    ///
    /// The number of related business entities.
    #[strum(serialize = "329")]
    _329,

    /// Relatives employed
    ///
    /// The number of relatives which are counted as employees.
    #[strum(serialize = "330")]
    _330,

    /// Rooms
    ///
    /// The number of rooms.
    #[strum(serialize = "331")]
    _331,

    /// Salespersons
    ///
    /// The number of salespersons.
    #[strum(serialize = "332")]
    _332,

    /// Seats
    ///
    /// The number of seats.
    #[strum(serialize = "333")]
    _333,

    /// Shareholders
    ///
    /// The number of shareholders.
    #[strum(serialize = "334")]
    _334,

    /// Shares of common stock
    ///
    /// The number of shares of common stock.
    #[strum(serialize = "335")]
    _335,

    /// Shares of preferred stock
    ///
    /// The number of shares of preferred stock.
    #[strum(serialize = "336")]
    _336,

    /// Silent partners
    ///
    /// The number of silent partners.
    #[strum(serialize = "337")]
    _337,

    /// Subcontractors
    ///
    /// The number of subcontractors.
    #[strum(serialize = "338")]
    _338,

    /// Subsidiaries
    ///
    /// The number of subsidiaries.
    #[strum(serialize = "339")]
    _339,

    /// Law suits
    ///
    /// The number of law suits.
    #[strum(serialize = "340")]
    _340,

    /// Suppliers
    ///
    /// The number of suppliers.
    #[strum(serialize = "341")]
    _341,

    /// Teachers
    ///
    /// The number of teachers.
    #[strum(serialize = "342")]
    _342,

    /// Technicians
    ///
    /// The number of technicians.
    #[strum(serialize = "343")]
    _343,

    /// Trainees
    ///
    /// The number of trainees.
    #[strum(serialize = "344")]
    _344,

    /// Union employees
    ///
    /// The number of employees who are members of a labour union.
    #[strum(serialize = "345")]
    _345,

    /// Number of units
    ///
    /// The quantity of units.
    #[strum(serialize = "346")]
    _346,

    /// Warehouse employees
    ///
    /// The number of employees who work in a warehouse setting.
    #[strum(serialize = "347")]
    _347,

    /// Shareholders holding remainder of shares
    ///
    /// Number of shareholders owning the remainder of shares.
    #[strum(serialize = "348")]
    _348,

    /// Payment orders filed
    ///
    /// Number of payment orders filed.
    #[strum(serialize = "349")]
    _349,

    /// Uncovered cheques
    ///
    /// Number of uncovered cheques.
    #[strum(serialize = "350")]
    _350,

    /// Auctions
    ///
    /// Number of auctions.
    #[strum(serialize = "351")]
    _351,

    /// Units produced
    ///
    /// The number of units produced.
    #[strum(serialize = "352")]
    _352,

    /// Added employees
    ///
    /// Number of employees that were added to the workforce.
    #[strum(serialize = "353")]
    _353,

    /// Number of added locations
    ///
    /// Number of locations that were added.
    #[strum(serialize = "354")]
    _354,

    /// Total number of foreign subsidiaries not included in financial statement
    ///
    /// The total number of foreign subsidiaries not included in the financial statement.
    #[strum(serialize = "355")]
    _355,

    /// Number of closed locations
    ///
    /// Number of locations that were closed.
    #[strum(serialize = "356")]
    _356,

    /// Counter clerks
    ///
    /// The number of clerks that work behind a flat-topped fitment.
    #[strum(serialize = "357")]
    _357,

    /// Payment experiences in the last 3 months
    ///
    /// The number of payment experiences received for an entity over the last 3 months.
    #[strum(serialize = "358")]
    _358,

    /// Payment experiences in the last 12 months
    ///
    /// The number of payment experiences received for an entity over the last 12 months.
    #[strum(serialize = "359")]
    _359,

    /// Total number of subsidiaries not included in the financial statement
    ///
    /// The total number of subsidiaries not included in the financial statement.
    #[strum(serialize = "360")]
    _360,

    /// Paid-in common shares
    ///
    /// The number of paid-in common shares.
    #[strum(serialize = "361")]
    _361,

    /// Total number of domestic subsidiaries not included in financial statement
    ///
    /// The total number of domestic subsidiaries not included in the financial statement.
    #[strum(serialize = "362")]
    _362,

    /// Total number of foreign subsidiaries included in financial statement
    ///
    /// The total number of foreign subsidiaries included in the financial statement.
    #[strum(serialize = "363")]
    _363,

    /// Total number of domestic subsidiaries included in financial statement
    ///
    /// The total number of domestic subsidiaries included in the financial statement.
    #[strum(serialize = "364")]
    _364,

    /// Total transactions
    ///
    /// The total number of transactions.
    #[strum(serialize = "365")]
    _365,

    /// Paid-in preferred shares
    ///
    /// The number of paid-in preferred shares.
    #[strum(serialize = "366")]
    _366,

    /// Employees
    ///
    /// Code specifying the quantity of persons working for a company, whose services are used for pay.
    #[strum(serialize = "367")]
    _367,

    /// Active ingredient dose per unit, dispensed
    ///
    /// The dosage of active ingredient per dispensed unit.
    #[strum(serialize = "368")]
    _368,

    /// Budget
    ///
    /// Budget quantity.
    #[strum(serialize = "369")]
    _369,

    /// Budget, cumulative to date
    ///
    /// Budget quantity, cumulative to date.
    #[strum(serialize = "370")]
    _370,

    /// Actual units
    ///
    /// The number of actual units.
    #[strum(serialize = "371")]
    _371,

    /// Actual units, cumulative to date
    ///
    /// The number of cumulative to date actual units.
    #[strum(serialize = "372")]
    _372,

    /// Earned value
    ///
    /// Earned value quantity.
    #[strum(serialize = "373")]
    _373,

    /// Earned value, cumulative to date
    ///
    /// Earned value quantity accumulated to date.
    #[strum(serialize = "374")]
    _374,

    /// At completion quantity, estimated
    ///
    /// The estimated quantity when a project is complete.
    #[strum(serialize = "375")]
    _375,

    /// To complete quantity, estimated
    ///
    /// The estimated quantity required to complete a project.
    #[strum(serialize = "376")]
    _376,

    /// Adjusted units
    ///
    /// The number of adjusted units.
    #[strum(serialize = "377")]
    _377,

    /// Number of limited partnership shares
    ///
    /// Number of shares held in a limited partnership.
    #[strum(serialize = "378")]
    _378,

    /// National business failure incidences
    ///
    /// Number of firms in a country that discontinued with a loss to creditors.
    #[strum(serialize = "379")]
    _379,

    /// Industry business failure incidences
    ///
    /// Number of firms in a specific industry that discontinued with a loss to creditors.
    #[strum(serialize = "380")]
    _380,

    /// Business class failure incidences
    ///
    /// Number of firms in a specific class that discontinued with a loss to creditors.
    #[strum(serialize = "381")]
    _381,

    /// Mechanics
    ///
    /// Number of mechanics.
    #[strum(serialize = "382")]
    _382,

    /// Messengers
    ///
    /// Number of messengers.
    #[strum(serialize = "383")]
    _383,

    /// Primary managers
    ///
    /// Number of primary managers.
    #[strum(serialize = "384")]
    _384,

    /// Secretaries
    ///
    /// Number of secretaries.
    #[strum(serialize = "385")]
    _385,

    /// Detrimental legal filings
    ///
    /// Number of detrimental legal filings.
    #[strum(serialize = "386")]
    _386,

    /// Branch office locations, estimated
    ///
    /// Estimated number of branch office locations.
    #[strum(serialize = "387")]
    _387,

    /// Previous number of employees
    ///
    /// The number of employees for a previous period.
    #[strum(serialize = "388")]
    _388,

    /// Asset seizers
    ///
    /// Number of entities that seize assets of another entity.
    #[strum(serialize = "389")]
    _389,

    /// Out-turned quantity
    ///
    /// The quantity discharged.
    #[strum(serialize = "390")]
    _390,

    /// Material on-board quantity, prior to loading
    ///
    /// The material in vessel tanks, void spaces, and pipelines prior to loading.
    #[strum(serialize = "391")]
    _391,

    /// Supplier estimated previous meter reading
    ///
    /// Previous meter reading estimated by the supplier.
    #[strum(serialize = "392")]
    _392,

    /// Supplier estimated latest meter reading
    ///
    /// Latest meter reading estimated by the supplier.
    #[strum(serialize = "393")]
    _393,

    /// Customer estimated previous meter reading
    ///
    /// Previous meter reading estimated by the customer.
    #[strum(serialize = "394")]
    _394,

    /// Customer estimated latest meter reading
    ///
    /// Latest meter reading estimated by the customer.
    #[strum(serialize = "395")]
    _395,

    /// Supplier previous meter reading
    ///
    /// Previous meter reading done by the supplier.
    #[strum(serialize = "396")]
    _396,

    /// Supplier latest meter reading
    ///
    /// Latest meter reading recorded by the supplier.
    #[strum(serialize = "397")]
    _397,

    /// Maximum number of purchase orders allowed
    ///
    /// Maximum number of purchase orders that are allowed.
    #[strum(serialize = "398")]
    _398,

    /// File size before compression
    ///
    /// The size of a file before compression.
    #[strum(serialize = "399")]
    _399,

    /// File size after compression
    ///
    /// The size of a file after compression.
    #[strum(serialize = "400")]
    _400,

    /// Securities shares
    ///
    /// Number of shares of securities.
    #[strum(serialize = "401")]
    _401,

    /// Patients
    ///
    /// Number of patients.
    #[strum(serialize = "402")]
    _402,

    /// Completed projects
    ///
    /// Number of completed projects.
    #[strum(serialize = "403")]
    _403,

    /// Promoters
    ///
    /// Number of entities who finance or organize an event or a production.
    #[strum(serialize = "404")]
    _404,

    /// Administrators
    ///
    /// Number of administrators.
    #[strum(serialize = "405")]
    _405,

    /// Supervisors
    ///
    /// Number of supervisors.
    #[strum(serialize = "406")]
    _406,

    /// Professionals
    ///
    /// Number of professionals.
    #[strum(serialize = "407")]
    _407,

    /// Debt collectors
    ///
    /// Number of debt collectors.
    #[strum(serialize = "408")]
    _408,

    /// Inspectors
    ///
    /// Number of individuals who perform inspections.
    #[strum(serialize = "409")]
    _409,

    /// Operators
    ///
    /// Number of operators.
    #[strum(serialize = "410")]
    _410,

    /// Trainers
    ///
    /// Number of trainers.
    #[strum(serialize = "411")]
    _411,

    /// Active accounts
    ///
    /// Number of accounts in a current or active status.
    #[strum(serialize = "412")]
    _412,

    /// Trademarks used
    ///
    /// Number of trademarks used.
    #[strum(serialize = "413")]
    _413,

    /// Machines
    ///
    /// Number of machines.
    #[strum(serialize = "414")]
    _414,

    /// Fuel pumps
    ///
    /// Number of fuel pumps.
    #[strum(serialize = "415")]
    _415,

    /// Tables available
    ///
    /// Number of tables available for use.
    #[strum(serialize = "416")]
    _416,

    /// Directors
    ///
    /// Number of directors.
    #[strum(serialize = "417")]
    _417,

    /// Freelance debt collectors
    ///
    /// Number of debt collectors who work on a freelance basis.
    #[strum(serialize = "418")]
    _418,

    /// Freelance salespersons
    ///
    /// Number of salespersons who work on a freelance basis.
    #[strum(serialize = "419")]
    _419,

    /// Travelling employees
    ///
    /// Number of travelling employees.
    #[strum(serialize = "420")]
    _420,

    /// Foremen
    ///
    /// Number of workers with limited supervisory responsibilities.
    #[strum(serialize = "421")]
    _421,

    /// Production workers
    ///
    /// Number of employees engaged in production.
    #[strum(serialize = "422")]
    _422,

    /// Employees not including owners
    ///
    /// Number of employees excluding business owners.
    #[strum(serialize = "423")]
    _423,

    /// Beds
    ///
    /// Number of beds.
    #[strum(serialize = "424")]
    _424,

    /// Resting quantity
    ///
    /// A quantity of product that is at rest before it can be used.
    #[strum(serialize = "425")]
    _425,

    /// Production requirements
    ///
    /// Quantity needed to meet production requirements.
    #[strum(serialize = "426")]
    _426,

    /// Corrected quantity
    ///
    /// The quantity has been corrected.
    #[strum(serialize = "427")]
    _427,

    /// Operating divisions
    ///
    /// Number of divisions operating.
    #[strum(serialize = "428")]
    _428,

    /// Quantitative incentive scheme base
    ///
    /// Quantity constituting the base for the quantitative incentive scheme.
    #[strum(serialize = "429")]
    _429,

    /// Petitions filed
    ///
    /// Number of petitions that have been filed.
    #[strum(serialize = "430")]
    _430,

    /// Bankruptcy petitions filed
    ///
    /// Number of bankruptcy petitions that have been filed.
    #[strum(serialize = "431")]
    _431,

    /// Projects in process
    ///
    /// Number of projects in process.
    #[strum(serialize = "432")]
    _432,

    /// Changes in capital structure
    ///
    /// Number of modifications made to the capital structure of an entity.
    #[strum(serialize = "433")]
    _433,

    /// Detrimental legal filings against directors
    ///
    /// The number of legal filings that are of a detrimental nature that have been filed against the directors.
    #[strum(serialize = "434")]
    _434,

    /// Number of failed businesses of directors
    ///
    /// The number of failed businesses with which the directors have been associated.
    #[strum(serialize = "435")]
    _435,

    /// Professor
    ///
    /// The number of professors.
    #[strum(serialize = "436")]
    _436,

    /// Seller
    ///
    /// The number of sellers.
    #[strum(serialize = "437")]
    _437,

    /// Skilled worker
    ///
    /// The number of skilled workers.
    #[strum(serialize = "438")]
    _438,

    /// Trademark represented
    ///
    /// The number of trademarks represented.
    #[strum(serialize = "439")]
    _439,

    /// Number of quantitative incentive scheme units
    ///
    /// Number of units allocated to a quantitative incentive scheme.
    #[strum(serialize = "440")]
    _440,

    /// Quantity in manufacturing process
    ///
    /// Quantity currently in the manufacturing process.
    #[strum(serialize = "441")]
    _441,

    /// Number of units in the width of a layer
    ///
    /// Number of units which make up the width of a layer.
    #[strum(serialize = "442")]
    _442,

    /// Number of units in the depth of a layer
    ///
    /// Number of units which make up the depth of a layer.
    #[strum(serialize = "443")]
    _443,

    /// Mutually defined
    ///
    /// As agreed by the trading partners.
    #[strum(ascii_case_insensitive)]
    ZZZ,
}

/// Exchange rate currency market identifier
///
/// To identify an exchange rate currency market.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _6341 {
    /// Paris exchange
    AAA,
    /// Colombian official exchange (Central Bank of Colombia)
    ///
    /// The currency exchange rate is set by the Central Bank of Colombia.
    AAB,
    /// Amsterdam exchange
    AMS,
    /// Bolsa de Comercio de Buenos Aires
    ///
    /// Argentina exchange.
    ARG,
    /// Australian exchange
    AST,
    /// Wien exchange
    ///
    /// Wiener Boersenkammer.
    AUS,
    /// Brussels exchange
    ///
    /// Commission de la Bourse Bruxelles.
    BEL,
    /// Toronto exchange
    CAN,
    /// Contractual agreement exchange rate
    CAR,
    /// US Customs Information Exchange
    ///
    /// Currency rates published by the US Customs Information Exchange, 6 WTC, New York NY 10048-0945, USA.
    CIE,
    /// Copenhagen exchange
    ///
    /// Koebenhavns Fondsboers.
    DEN,
    /// European Community period exchange rate
    ///
    /// Description to be provided.
    ECR,
    /// Helsinki exchange
    FIN,
    /// Frankfurt exchange
    FRA,
    /// International Monetary Fund
    IMF,
    /// London exchange, first closing
    LNF,
    /// London exchange, second closing
    LNS,
    /// Milan exchange
    MIL,
    /// Oslo exchange
    NOR,
    /// New York exchange
    NYC,
    /// Philadelphia exchange
    PHI,
    /// Specific railway exchange currency
    ///
    /// Specific rate of exchange applied to currency exchanges between rail companies and partners.
    SRE,
    /// Stockholm exchange
    SWE,
    /// Zurich exchange
    ZUR,
}

/// Currency type code qualifier
///
/// Code qualifying the type of currency.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _6343 {
    /// Customs valuation currency
    ///
    /// The name or symbol of the monetary unit involved in the transaction for customs valuation.
    #[strum(serialize = "1")]
    _1,
    /// Insurance currency
    ///
    /// The name or symbol of the monetary unit involved in the transaction for insurance purposes.
    #[strum(serialize = "2")]
    _2,
    /// Home currency
    ///
    /// The name or symbol of the local monetary unit.
    #[strum(serialize = "3")]
    _3,
    /// Invoicing currency
    ///
    /// The name or symbol of the monetary unit used for calculation in an invoice.
    #[strum(serialize = "4")]
    _4,
    /// Account currency
    ///
    /// The name or symbol of the monetary unit to be converted from.
    #[strum(serialize = "5")]
    _5,
    /// Reference currency
    ///
    /// The name or symbol of the monetary unit to be converted.
    #[strum(serialize = "6")]
    _6,
    /// Target currency
    ///
    /// The name or symbol of the monetary unit to be converted into.
    #[strum(serialize = "7")]
    _7,
    /// Price list currency
    ///
    /// The name or symbol of the monetary unit used in a price list.
    #[strum(serialize = "8")]
    _8,
    /// Order currency
    ///
    /// The name or symbol of the monetary unit used in an order.
    #[strum(serialize = "9")]
    _9,
    /// Pricing currency
    ///
    /// The name or symbol of the monetary unit used for pricing purposes.
    #[strum(serialize = "10")]
    _10,
    /// Payment currency
    ///
    /// The name or symbol of the monetary unit used for payment.
    #[strum(serialize = "11")]
    _11,
    /// Quotation currency
    ///
    /// The name or symbol of the monetary unit used in a quotation.
    #[strum(serialize = "12")]
    _12,
    /// Recipient local currency
    ///
    /// The name or symbol of the local monetary unit at recipient's location.
    #[strum(serialize = "13")]
    _13,
    /// Supplier currency
    ///
    /// The name or symbol of the monetary unit normally used by the supplier.
    #[strum(serialize = "14")]
    _14,
    /// Sender local currency
    ///
    /// The name or symbol of the local monetary unit at sender's location.
    #[strum(serialize = "15")]
    _15,
    /// Tariff currency
    ///
    /// The currency as per tariff.
    #[strum(serialize = "16")]
    _16,
    /// Charge calculation currency
    ///
    /// The currency in which the charges are calculated.
    #[strum(serialize = "17")]
    _17,
    /// Tax currency
    ///
    /// The currency in which tax amounts are due or have been paid.
    #[strum(serialize = "18")]
    _18,
}

/// Currency usage code qualifier
///
/// Code qualifying the usage of a currency.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _6347 {
    /// Charge payment currency
    ///
    /// The currency in which charges are to be paid.
    #[strum(serialize = "1")]
    _1,
    /// Reference currency
    ///
    /// The currency applicable to amounts stated. It may have to be converted.
    #[strum(serialize = "2")]
    _2,
    /// Target currency
    ///
    /// The currency which should be used to the target destination of the transaction.
    #[strum(serialize = "3")]
    _3,
    /// Transport document currency
    ///
    /// Currency applicable to amounts stated in a transport document/message.
    #[strum(serialize = "4")]
    _4,
    /// Calculation base currency
    ///
    /// Currency on which the calculation is based.
    #[strum(serialize = "5")]
    _5,
    /// Information Currency
    ///
    /// Additional currency the message recipient needs for information purposes. The actual message amount(s) is/are not based upon this currency.
    #[strum(serialize = "6")]
    _6,
    /// Currency of the account
    ///
    /// Currency in which the account is held.
    #[strum(serialize = "7")]
    _7,
}

/// Cargo type classification code
///
/// Code specifying the classification of a type of cargo.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _7085 {
    #[strum(serialize = "1")]
    /// Documents
    ///
    /// Printed, typed or written matter including leaflets, pamphlets, certificates etc., which are not subject to import duties and taxes, restrictions and prohibitions.
    _1,
    #[strum(serialize = "2")]
    /// Low value non-dutiable consignments
    ///
    /// Imported consignments/items/goods in respect of which Customs duties and other taxes are waived as they are below a value determined by the Customs administration.
    _2,
    #[strum(serialize = "3")]
    /// Low value dutiable consignments
    ///
    /// Imported consignments/items/goods in respect of which Customs duties and other taxes are payable are below a certain amount as determined by the Customs administration.
    _3,
    #[strum(serialize = "4")]
    /// High value consignments
    ///
    /// Imported consignments/items/goods which are determined as having a value above a certain amount fixed by the Customs administration, which may or may not attract duties and taxes.
    _4,
    #[strum(serialize = "5")]
    /// Other non-containerized
    ///
    /// Non-containerized cargo which cannot be categorized by any of the other nature of cargo code.
    _5,
    #[strum(serialize = "6")]
    /// Vehicles
    ///
    /// Vehicles which are not stowed in containers.
    _6,
    #[strum(serialize = "7")]
    /// Roll-on roll-off
    ///
    /// Cargo transported or to be transported on roll-on roll- off vessels and which is transportable on its own wheels or stowed on special heavy duty trailers.
    _7,
    #[strum(serialize = "8")]
    /// Palletized
    ///
    /// Non-containerized cargo which is palletized.
    _8,
    #[strum(serialize = "9")]
    /// Containerized
    ///
    /// Cargo stowed or to be stowed in a container.
    _9,
    #[strum(serialize = "10")]
    /// Breakbulk
    ///
    /// Non-containerized cargo stowed in vessels' holds.
    _10,
    #[strum(serialize = "11")]
    /// Hazardous cargo
    ///
    /// Cargo with dangerous properties, according to appropriate dangerous goods regulations.
    _11,
    #[strum(serialize = "12")]
    /// General cargo
    ///
    /// Cargo of a general nature, not otherwise specified.
    _12,
    #[strum(serialize = "13")]
    /// Liquid cargo
    ///
    /// Cargo in liquid form.
    _13,
    #[strum(serialize = "14")]
    /// Temperature controlled cargo
    ///
    /// Cargo transported under specified temperature conditions.
    _14,
    #[strum(serialize = "15")]
    /// Environmental pollutant cargo
    ///
    /// Cargo is an environmental pollutant.
    _15,
    #[strum(serialize = "16")]
    /// Not-hazardous cargo
    ///
    /// Cargo which is not hazardous.
    _16,
    #[strum(serialize = "17")]
    /// Diplomatic
    ///
    /// Cargo transported under diplomatic conditions.
    _17,
    #[strum(serialize = "18")]
    /// Military
    ///
    /// Cargo for military purposes.
    _18,
    #[strum(serialize = "19")]
    /// Obnoxious
    ///
    /// Cargo that is objectionable to human senses.
    _19,
    #[strum(serialize = "20")]
    /// Out of gauge
    ///
    /// Cargo that has at least one non-standard dimension.
    _20,
    #[strum(serialize = "21")]
    /// Household goods and personal effects
    ///
    /// Cargo consisting of household goods and personal effects.
    _21,
    #[strum(serialize = "22")]
    /// Frozen cargo
    ///
    /// Cargo of frozen products.
    _22,
}

/// Service requirement code
///
/// Code specifying a service requirement.
#[derive(Debug, Serialize, Deserialize, EnumString, Display, Clone)]
pub enum _7273 {
    #[strum(serialize = "1")]
    /// Carrier loads
    ///
    /// The cargo is loaded in the equipment by the carrier.
    _1,
    #[strum(serialize = "2")]
    /// Full loads
    ///
    /// Container to be stuffed or stripped under responsibility and for account of the shipper or the consignee.
    _2,
    #[strum(serialize = "3")]
    /// Less than full loads
    ///
    /// Container to be stuffed and stripped for account and risk of the carrier.
    _3,
    #[strum(serialize = "4")]
    /// Shipper loads
    ///
    /// The cargo is loaded in the equipment by the shipper.
    _4,
    #[strum(serialize = "5")]
    /// To be delivered
    ///
    /// The cargo is to be delivered as instructed.
    _5,
    #[strum(serialize = "6")]
    /// To be kept
    ///
    /// The cargo is to be retained awaiting further instructions.
    _6,
    #[strum(serialize = "7")]
    /// Transhipment allowed
    ///
    /// Transhipment of goods is allowed.
    _7,
    #[strum(serialize = "8")]
    /// Transhipment not allowed
    ///
    /// Transhipment of goods is not allowed.
    _8,
    #[strum(serialize = "9")]
    /// Partial shipment allowed
    ///
    /// Partial shipment is allowed.
    _9,
    #[strum(serialize = "10")]
    /// Partial shipment not allowed
    ///
    /// Partial shipment is not allowed.
    _10,
    #[strum(serialize = "11")]
    /// Partial shipment and/or drawing allowed
    ///
    /// Partial shipment and/or drawing is allowed.
    _11,
    #[strum(serialize = "12")]
    /// Partial shipment and/or drawing not allowed
    ///
    /// Partial shipment and/or drawing is not allowed.
    _12,
    #[strum(serialize = "13")]
    /// Carrier unloads
    ///
    /// The cargo is to be unloaded from the equipment by the carrier.
    _13,
    #[strum(serialize = "14")]
    /// Shipper unloads
    ///
    /// The cargo is to be unloaded from the equipment by the shipper.
    _14,
    #[strum(serialize = "15")]
    /// Consignee unloads
    ///
    /// The cargo is to be unloaded from the equipment by the consignee.
    _15,
    #[strum(serialize = "16")]
    /// Consignee loads
    ///
    /// The cargo is to be loaded in the equipment by the consignee.
    _16,
    #[strum(serialize = "17")]
    /// Exclusive usage of equipment
    ///
    /// Usage of the equipment is reserved for exclusive use.
    _17,
    #[strum(serialize = "18")]
    /// Non exclusive usage of equipment
    ///
    /// Usage of the equipment is not reserved for exclusive use.
    _18,
    #[strum(serialize = "19")]
    /// Direct delivery
    ///
    /// Consignment for direct delivery to the consignee.
    _19,
    #[strum(serialize = "20")]
    /// Direct pick-up
    ///
    /// Consignment for direct pick-up from the consignee.
    _20,
    #[strum(serialize = "21")]
    /// Request for delivery advice services
    ///
    /// The service provider is requested to advise about delivery.
    _21,
    #[strum(serialize = "22")]
    /// Do not arrange customs clearance
    ///
    /// Indication that the recipient of the message is not to arrange customs clearance.
    _22,
    #[strum(serialize = "23")]
    /// Arrange customs clearance
    ///
    /// Indication that the recipient of the message is to arrange customs clearance.
    _23,
    #[strum(serialize = "24")]
    /// Check container condition
    ///
    /// Condition of the container is to be checked.
    _24,
    #[strum(serialize = "25")]
    /// Damaged containers allowed
    ///
    /// Damaged containers are allowed.
    _25,
    #[strum(serialize = "26")]
    /// Dirty containers allowed
    ///
    /// Dirty containers are allowed.
    _26,
    #[strum(serialize = "27")]
    /// Fork lift holes not required
    ///
    /// Container needs not to be equipped with pocket holes, but they are allowed.
    _27,
    #[strum(serialize = "28")]
    /// Fork lift holes required
    ///
    /// Container must be equipped with pocket holes.
    _28,
    #[strum(serialize = "29")]
    /// Insure goods during transport
    ///
    /// Indication that the recipient of the message is to insure the goods during transport.
    _29,
    #[strum(serialize = "30")]
    /// Arrange main-carriage
    ///
    /// Indication that the recipient of the message is to arrange the main-carriage.
    _30,
    #[strum(serialize = "31")]
    /// Arrange on-carriage
    ///
    /// Indication that the recipient of the message is to arrange the on-carriage.
    _31,
    #[strum(serialize = "32")]
    /// Arrange pre-carriage
    ///
    /// Indication that the recipient of the message is to arrange the pre-carriage.
    _32,
    #[strum(serialize = "33")]
    /// Report container safety convention information
    ///
    /// Indication that the information on the Container Safety Convention plate (CSC-plate) should be reported.
    _33,
    #[strum(serialize = "34")]
    /// Check seals
    ///
    /// Sealing up of the container is to be checked.
    _34,
    #[strum(serialize = "35")]
    /// Container must be clean
    ///
    /// Container is to be released or delivered clean.
    _35,
    #[strum(serialize = "36")]
    /// Request for proof of delivery
    ///
    /// The service provider is requested to provide proof of delivery.
    _36,
    #[strum(serialize = "37")]
    /// Request for Customs procedure
    ///
    /// The service provider is requested to perform Customs procedure.
    _37,
    #[strum(serialize = "38")]
    /// Request for administration services
    ///
    /// The service provider is requested to perform administration services.
    _38,
    #[strum(serialize = "39")]
    /// Transport insulated under Intercontainer INTERFRIGO conditions
    ///
    /// Insulated transport under Intercontainer INTERFRIGO (joint European railways agreement) conditions.
    _39,
    #[strum(serialize = "40")]
    /// Transport mechanically refrigerated under Intercontainer INTERFRIGO conditions
    ///
    /// Mechanically refrigerated transport under Intercontainer INTERFRIGO (joint European railways agreement) conditions.
    _40,
    #[strum(serialize = "41")]
    /// Cool or freeze service, not under Intercontainer INTERFRIGO conditions
    ///
    /// Cool or freeze service not under Intercontainer INTERFRIGO (joint European railways agreement) conditions.
    _41,
    #[strum(serialize = "42")]
    /// Transhipment overseas
    ///
    /// Transport equipment is to be transferred overseas.
    _42,
    #[strum(serialize = "43")]
    /// Station delivery
    ///
    /// The specified equipment destination station is also the place of delivery of the goods.
    _43,
    #[strum(serialize = "44")]
    /// Non station delivery
    ///
    /// The specified equipment destination station is not the place of delivery of the goods.
    _44,
    #[strum(serialize = "45")]
    /// Cleaning or disinfecting
    ///
    /// The service required is cleaning or disinfection.
    _45,
    #[strum(serialize = "46")]
    /// Close ventilation valve
    ///
    /// The ventilation valve of the equipment must be closed.
    _46,
    #[strum(serialize = "47")]
    /// Consignment held for pick-up
    ///
    /// The consignment is to be held until it is picked up.
    _47,
    #[strum(serialize = "48")]
    /// Refrigeration unit check
    ///
    /// Refrigeration unit has to be checked.
    _48,
    #[strum(serialize = "49")]
    /// Customs clearance at arrival country by carrier
    ///
    /// The carrier is to arrange customs clearance in the arrival country.
    _49,
    #[strum(serialize = "50")]
    /// Customs clearance at departure country by carrier
    ///
    /// The carrier is to arrange customs clearance in the departure country.
    _50,
    #[strum(serialize = "51")]
    /// Heating for live animals
    ///
    /// Heating for live animals has to be provided.
    _51,
    #[strum(serialize = "52")]
    /// Goods humidification
    ///
    /// Humidification of the goods has to be performed.
    _52,
    #[strum(serialize = "53")]
    /// Ensure load is secure
    ///
    /// The load must be checked for correct stowage.
    _53,
    #[strum(serialize = "54")]
    /// Open ventilation valve
    ///
    /// The ventilation valve of the equipment must be opened.
    _54,
    #[strum(serialize = "55")]
    /// Phytosanitary control
    ///
    /// Phytosanitary control to be performed.
    _55,
    #[strum(serialize = "56")]
    /// Tare check by carrier
    ///
    /// Carrier must check the tare of the equipment and attached items.
    _56,
    #[strum(serialize = "57")]
    /// Temperature check
    ///
    /// The temperature must be checked.
    _57,
    #[strum(serialize = "58")]
    /// Weighing of goods
    ///
    /// The goods have to be weighed.
    _58,
    #[strum(serialize = "59")]
    /// Escort
    ///
    /// An escort is required.
    _59,
    #[strum(serialize = "60")]
    /// No escort
    ///
    /// An escort is not required.
    _60,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::clean_num;
    use std::str::FromStr;
    #[test]
    fn test_1001() {
        let vars = ["705", "706", "01"];
        let res = vars.first().map(|x| _1001::from_str(clean_num(x)).unwrap());
        assert_eq!(res, Some(_1001::_705));
        let res = vars.get(1).map(|x| _1001::from_str(clean_num(x)).unwrap());
        assert_eq!(res, Some(_1001::_706));
        let res = vars.get(2).map(|x| _1001::from_str(clean_num(x)).unwrap());
        assert_eq!(res, Some(_1001::_1));
        println!("{}", res.unwrap());
    }
}
