use crate::d95b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseMsg, ParseSg};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseMsg)]
pub struct COPRAR {
    pub unh: UNH,
    pub bgm: BGM,
    pub ftx: Vec<FTX>,
    pub rff: Vec<RFF>,
    pub segment_group_1: COPRARSegmentgroup1,
    pub segment_group_2: Vec<COPRARSegmentgroup2>,
    pub segment_group_3: Vec<COPRARSegmentgroup3>,
    pub cnt: CNT,
    pub unt: UNT,
}

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPRARSegmentgroup1 {
    pub tdt: TDT,
    pub rff: Vec<RFF>,
    pub loc: Vec<LOC>,
    pub dtm: Vec<DTM>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPRARSegmentgroup2 {
    pub nad: NAD,
    pub cta: Vec<CTA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPRARSegmentgroup3 {
    pub eqd: EQD,
    pub rff: Vec<RFF>,
    pub eqn: Option<EQN>,
    pub tmd: Vec<TMD>,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub tmp: Vec<TMP>,
    pub rng: Vec<RNG>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub dgs: Vec<DGS>,
    pub eqa: Vec<EQA>,
    pub segment_group_4: Option<COPRARSegmentgroup4>,
    pub nad: Option<NAD>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPRARSegmentgroup4 {
    pub tdt: TDT,
    pub rff: Vec<RFF>,
    pub loc: Vec<LOC>,
    pub dtm: Vec<DTM>,
}
