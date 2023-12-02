use crate::util::Parser;
use edifact_types_macros::DisplayEdifact;
use nom::{combinator::opt, IResult};
use serde::{Deserialize, Serialize};
use std::fmt;

const VERSION: &str = "D95B";

mod element;
mod message;
mod segment;
mod types;

// Re-Export on root level to keep compatibility
pub use element::*;
// pub use message::baplie::*;
pub use message::coprar::*;
pub use segment::*;
pub use types::*;

#[cfg(test)]
mod test_segment;

/// from: [official info](https://unece.org/fileadmin/DAM/trade/edifact/untdid/d422_s.htm)
/// 6.1 Interchange structure
///
/// The Service String Advice, UNA, and the service segments UNB
/// to UNZ shall appear in the below stated order in an
/// interchange. There may be several functional groups or
/// messages within an interchange and several messages in a
/// functional group. A message consists of segments. The
/// structures for segments and for data elements therein are
/// shown in 6.2 and 6.3. The contents of the service segments
/// are shown annex B. See also figure 1.
///
/// An interchange consists of:
///
/// x | x | x | x | Name | Abbr. | Req.
/// --- | --- | --- | --- | --- | --- | ---
/// o | o | o | o | Service String Advice | UNA | Conditional
/// _ | _ | _ | _ | Interchange Header | UNB | Mandatory
/// \| | _ | _ | _ | Functional Group Header | UNG | Conditional
/// \| | \| | _ | _ | Message Header | UNH | Mandatory
/// \| | \| | \| |   | User Data Segments |   | As required
/// \| | \| | \| | _ | Message Trailer | UNT | Mandatory
/// \| | \| | _ | _ | Functional Group Trailer | UNE | Conditional
/// \| | _ | _ | _ | Interchange Trailer | UNZ | Mandatory
///
/// In addition to the above service segments, the service
/// segment UNS can, when required, be used to divide a message
/// into sections. See annex B (NOT IMPLEMENTED).
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayEdifact)]
pub struct Interchange<T>
where
    T: std::fmt::Display,
{
    pub una: Option<UNA>,
    pub unb: UNB,
    pub segment: T,
    pub unz: UNZ,
}

impl<'a, T: Default + Parser<&'a str, T, nom::error::Error<&'a str>> + std::fmt::Display>
    Parser<&'a str, Interchange<T>, nom::error::Error<&'a str>> for Interchange<T>
{
    fn parse(input: &'a str) -> IResult<&'a str, Interchange<T>> {
        let mut output = Interchange::default();
        let (input, obj) = opt(UNA::parse)(input)?;
        output.una = obj;
        let (input, obj) = UNB::parse(input)?;
        output.unb = obj;
        let (input, t_obj) = T::parse(input)?;
        output.segment = t_obj;
        let (input, obj) = UNZ::parse(input)?;
        output.unz = obj;
        Ok((input, output))
    }
}
