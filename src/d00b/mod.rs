use std::fmt;

mod segment;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg};
use nom::{IResult, multi::many0, combinator::opt};
pub use segment::*;
mod types;
use serde::{Deserialize, Serialize};
pub use types::*;

use crate::util::Parser;

#[cfg(test)]
mod test_iftsta;
#[cfg(test)]
mod test_segment;

/// A message to report the transport status and/or a change in the
/// transport status (i.e. event) between agreed parties.
///
/// https://service.unece.org/trade/untdid/d00b/trmd/iftsta_c.htm
#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Iftsta {
    unh: UNH,
    bgm: BGM,
    dtm: Vec<DTM>,
    tsr: Option<TSR>,
    sg1: Vec<IftstaSg1>,
    sg3: Vec<IftstaSg3>,
    loc: Vec<LOC>,
    ftx: Vec<FTX>,
    cnt: Vec<CNT>,
    sg4: Vec<IftstaSg4>,
    unt: UNT,
}

impl<'a> Parser<&'a str, Iftsta, nom::error::Error<&'a str>> for Iftsta {
    fn parse(input: &'a str) -> IResult<&'a str, Iftsta> {
        let mut output = Iftsta::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = many0(DTM::parse)(rest)?;
        output.dtm = obj;
        let (rest, obj) = opt(TSR::parse)(rest)?;
        output.tsr = obj;
        //TODO
        let (rest, obj) = many0(LOC::parse)(rest)?;
        output.loc = obj;
        let (rest, obj) = many0(FTX::parse)(rest)?;
        output.ftx = obj;
        let (rest, obj) = many0(CNT::parse)(rest)?;
        output.cnt = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg1 {
    nad: NAD,
    sg2: Vec<IftstaSg2>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg2 {
    cta: CTA,
    com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg3 {
    rff: RFF,
    dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg4 {
    cni: CNI,
    loc: Vec<LOC>,
    cnt: Vec<CNT>,
    sg5: Vec<IftstaSg5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg5 {
    sts: STS,
    rff: Vec<RFF>,
    dtm: Vec<DTM>,
    doc: Option<DOC>,
    ftx: Vec<FTX>,
    nad: Vec<NAD>,
    loc: Option<LOC>,
    pci: Vec<PCI>,
    sg6: Vec<IftstaSg6>,
    sg8: Vec<IftstaSg8>,
    sg10: Vec<IftstaSg10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg6 {
    tdt: TDT,
    dtm: Vec<DTM>,
    rff: Vec<RFF>,
    sg7: Vec<IftstaSg7>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg7 {
    loc: LOC,
    dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg8 {
    eqd: EQD,
    mea: Vec<MEA>,
    dim: Vec<DIM>,
    sel: Vec<SEL>,
    rff: Vec<RFF>,
    tpl: Vec<TPL>,
    tmd: Option<TMD>,
    sg9: Vec<IftstaSg9>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg9 {
    eqa: EQA,
    sel: Vec<SEL>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg10 {
    gid: GID,
    han: Vec<HAN>,
    sgp: Vec<SGP>,
    dgs: Vec<DGS>,
    ftx: Vec<FTX>,
    sg11: Vec<IftstaSg11>,
    sg12: Vec<IftstaSg12>,
    sg13: Vec<IftstaSg13>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg11 {
    mea: MEA,
    eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg12 {
    dim: DIM,
    eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg13 {
    pci: PCI,
    gin: Vec<GIN>,
}
