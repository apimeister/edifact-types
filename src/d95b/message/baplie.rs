use crate::d95b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseMsg, ParseSg};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseMsg)]
pub struct BAPLIE {
    pub unh: UNH,
    pub bgm: BGM,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub nad: Vec<NAD>,
    pub segment_group_1: Vec<BAPLIESegmentgroup1>,
    pub segment_group_2: Vec<BAPLIESegmentgroup2>,
    pub unt: UNT,
}

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct BAPLIESegmentgroup1 {
    pub tdt: TDT,
    pub loc: Vec<LOC>,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct BAPLIESegmentgroup2 {
    pub loc: LOC,
    pub gid: Vec<GID>,
    pub gds: Vec<GDS>,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub tmp: Vec<TMP>,
    pub rng: Vec<RNG>,
    pub loc2: Vec<LOC>,
    pub rff: Vec<RFF>,
    pub segment_group_3: Vec<BAPLIESegmentgroup3>,
    pub segment_group_4: Vec<BAPLIESegmentgroup4>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct BAPLIESegmentgroup3 {
    pub eqd: EQD,
    pub eqa: Vec<EQA>,
    pub nad: Option<NAD>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct BAPLIESegmentgroup4 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
} 