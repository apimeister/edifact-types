use std::fmt;

mod segment;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg};
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult,
};
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
    segment_group_1: Vec<IftstaSg1>,
    segment_group_3: Vec<IftstaSg3>,
    loc: Vec<LOC>,
    ftx: Vec<FTX>,
    cnt: Vec<CNT>,
    segment_group_4: Vec<IftstaSg4>,
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
        // segment group 1
        let mut loop_sg1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(NAD::parse))(loop_rest)?.1.is_some() {
            let (rest, nad) = NAD::parse(loop_rest)?;
            loop_rest = rest;
            loop_sg1.push(IftstaSg1 {
                nad,
                segment_group_2: vec![],
            });
        }
        let rest = loop_rest;
        output.segment_group_1 = loop_sg1;
        // segment group 3
        let mut loop_sg3 = vec![];
        let mut loop_rest = rest;
        while peek(opt(RFF::parse))(loop_rest)?.1.is_some() {
            let (rest, rff) = RFF::parse(loop_rest)?;
            let (rest, dtm) = opt(DTM::parse)(rest)?;
            loop_rest = rest;
            loop_sg3.push(IftstaSg3 { rff, dtm });
        }
        let rest = loop_rest;
        output.segment_group_3 = loop_sg3;
        let (rest, obj) = many0(LOC::parse)(rest)?;
        output.loc = obj;
        let (rest, obj) = many0(FTX::parse)(rest)?;
        output.ftx = obj;
        let (rest, obj) = many0(CNT::parse)(rest)?;
        output.cnt = obj;
        // segment group 4
        let mut loop_sg4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(CNI::parse))(loop_rest)?.1.is_some() {
            let (rest, cni) = CNI::parse(loop_rest)?;
            let (rest, loc) = many0(LOC::parse)(rest)?;
            let (rest, cnt) = many0(CNT::parse)(rest)?;
            loop_rest = rest;
            let mut loop_sg5 = vec![];
            while peek(opt(STS::parse))(loop_rest)?.1.is_some() {
                let (rest, sts) = STS::parse(loop_rest)?;
                let (rest, rff) = many0(RFF::parse)(rest)?;
                let (rest, dtm) = many0(DTM::parse)(rest)?;
                let (rest, doc) = opt(DOC::parse)(rest)?;
                let (rest, ftx) = many0(FTX::parse)(rest)?;
                let (rest, nad) = many0(NAD::parse)(rest)?;
                let (rest, loc) = opt(LOC::parse)(rest)?;
                let (rest, pci) = many0(PCI::parse)(rest)?;
                loop_rest = rest;
                let mut loop_sg6 = vec![];
                while peek(opt(TDT::parse))(loop_rest)?.1.is_some() {
                    let (rest, tdt) = TDT::parse(rest)?;
                    let (rest, dtm) = many0(DTM::parse)(rest)?;
                    let (rest, rff) = many0(RFF::parse)(rest)?;
                    loop_rest = rest;
                    let mut loop_sg7 = vec![];
                    while peek(opt(LOC::parse))(loop_rest)?.1.is_some() {
                        let (rest, loc) = LOC::parse(loop_rest)?;
                        let (rest, dtm) = many0(DTM::parse)(rest)?;
                        loop_rest = rest;
                        loop_sg7.push(IftstaSg7 { loc, dtm });
                    }
                    loop_sg6.push(IftstaSg6 {
                        tdt,
                        dtm,
                        rff,
                        segment_group_7: loop_sg7,
                    });
                }
                // segment group 8
                let mut loop_sg8 = vec![];
                while peek(opt(EQD::parse))(loop_rest)?.1.is_some() {
                    let (rest, eqd) = EQD::parse(loop_rest)?;
                    let (rest, mea) = many0(MEA::parse)(rest)?;
                    let (rest, dim) = many0(DIM::parse)(rest)?;
                    let (rest, sel) = many0(SEL::parse)(rest)?;
                    let (rest, rff) = many0(RFF::parse)(rest)?;
                    let (rest, tpl) = many0(TPL::parse)(rest)?;
                    let (rest, tmd) = opt(TMD::parse)(rest)?;
                    loop_rest = rest;
                    loop_sg8.push(IftstaSg8 {
                        eqd,
                        mea,
                        dim,
                        sel,
                        rff,
                        tpl,
                        tmd,
                        segment_group_9: vec![],
                    });
                }
                loop_sg5.push(IftstaSg5 {
                    sts,
                    rff,
                    dtm,
                    doc,
                    ftx,
                    nad,
                    loc,
                    pci,
                    segment_group_6: loop_sg6,
                    segment_group_8: loop_sg8,
                    segment_group_10: vec![],
                });
            }

            loop_sg4.push(IftstaSg4 {
                cni,
                loc,
                cnt,
                segment_group_5: loop_sg5,
            });
        }
        let rest = loop_rest;
        output.segment_group_4 = loop_sg4;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg1 {
    nad: NAD,
    segment_group_2: Vec<IftstaSg2>,
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
    segment_group_5: Vec<IftstaSg5>,
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
    segment_group_6: Vec<IftstaSg6>,
    segment_group_8: Vec<IftstaSg8>,
    segment_group_10: Vec<IftstaSg10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg6 {
    tdt: TDT,
    dtm: Vec<DTM>,
    rff: Vec<RFF>,
    segment_group_7: Vec<IftstaSg7>,
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
    segment_group_9: Vec<IftstaSg9>,
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
    segment_group_11: Vec<IftstaSg11>,
    segment_group_12: Vec<IftstaSg12>,
    segment_group_13: Vec<IftstaSg13>,
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
