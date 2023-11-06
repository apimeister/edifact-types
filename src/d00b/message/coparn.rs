use crate::d00b::*;
use crate::util::Parser;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg};
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Coparn {
    pub unh: UNH,
    pub bgm: BGM,
    pub tmd: Option<TMD>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub ftx: Vec<FTX>,
    pub loc: Vec<LOC>,
    pub segment_group_1: Vec<CoparnSg1>,
    pub segment_group_2: Vec<CoparnSg2>,
    pub segment_group_4: Vec<CoparnSg4>,
    pub segment_group_6: Vec<CoparnSg6>,
    pub segment_group_13: Vec<CoparnSg13>,
    pub cnt: CNT,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Coparn, nom::error::Error<&'a str>> for Coparn {
    fn parse(input: &'a str) -> IResult<&'a str, Coparn> {
        let mut output = Coparn::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg1 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg2 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub segment_group_3: Vec<CoparnSg3>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg3 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg4 {
    pub nad: NAD,
    pub segment_group_5: Vec<CoparnSg5>,
    pub rff: Vec<RFF>,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg5 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg6 {
    pub gid: GID,
    pub han: Vec<HAN>,
    pub ftx: Vec<FTX>,
    pub rff: Vec<RFF>,
    pub pia: Vec<PIA>,
    pub segment_group_7: Vec<CoparnSg7>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub segment_group_8: Vec<CoparnSg8>,
    pub segment_group_9: Vec<CoparnSg9>,
    pub segment_group_10: Vec<CoparnSg10>,
    pub segment_group_12: Vec<CoparnSg12>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg7 {
    pub nad: NAD,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg8 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg9 {
    pub sgp: SGP,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg10 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub segment_group_11: Vec<CoparnSg11>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg11 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg12 {
    pub tmp: TMP,
    pub rng: RNG,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg13 {
    pub eqd: EQD,
    pub rff: Vec<RFF>,
    pub eqn: EQN,
    pub tmd: Vec<TMD>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub loc: Vec<LOC>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub segment_group_14: Vec<CoparnSg14>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub pcd: Vec<PCD>,
    pub segment_group_15: Vec<CoparnSg15>,
    pub moa: Vec<MOA>,
    pub gor: Vec<GOR>,
    pub eqa: EQA,
    pub cod: COD,
    pub han: Vec<HAN>,
    pub segment_group_17: Vec<CoparnSg17>,
    pub segment_group_18: Vec<CoparnSg18>,
    pub segment_group_20: Vec<CoparnSg20>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg14 {
    pub tmp: TMP,
    pub rng: Option<RNG>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg15 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub segment_group_16: Vec<CoparnSg16>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg16 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg17 {
    pub dam: DAM,
    pub cod: Option<COD>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg18 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub segment_group_19: Vec<CoparnSg19>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg19 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg20 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub cta: Option<CTA>,
    pub com: Option<COM>,
    pub rff: Vec<RFF>,
}
