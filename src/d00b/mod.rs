use crate::util::Parser;
use edifact_types_macros::DisplayEdifact;
use nom::{combinator::opt, IResult};
use serde::{Deserialize, Serialize};
use std::fmt;

mod types;
mod segment;
mod element;
mod message;

// Re-Export on root level to keep compatibility
pub use segment::*;
pub use element::*;
pub use types::*;
pub use message::coparn::*;
pub use message::coprar::*;
pub use message::coreor::*;
pub use message::iftmbf::*;
pub use message::iftmin::*;
pub use message::iftsta::*;

#[cfg(test)]
mod test_segment;

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
