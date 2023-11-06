use crate::d00b::*;
use crate::util::Parser;
use edifact_types_macros::DisplayEdifact;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Iftmbf {
    pub unh: UNH,
    pub bgm: BGM,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Iftmbf, nom::error::Error<&'a str>> for Iftmbf {
    fn parse(input: &'a str) -> IResult<&'a str, Iftmbf> {
        let mut output = Iftmbf::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}
