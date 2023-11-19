use crate::d00b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseMsg, ParseSg};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseMsg)]
pub struct COPARN {
    pub unh: UNH,
    pub bgm: BGM,
    pub tmd: Option<TMD>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub ftx: Vec<FTX>,
    pub loc: Vec<LOC>,
    pub segment_group_1: Vec<COPARNSegmentgroup1>,
    pub segment_group_2: Vec<COPARNSegmentgroup2>,
    pub segment_group_4: Vec<COPARNSegmentgroup4>,
    pub segment_group_6: Vec<COPARNSegmentgroup6>,
    pub segment_group_13: Vec<COPARNSegmentgroup13>,
    pub cnt: Option<CNT>,
    pub unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup1 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup10 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub segment_group_11: Vec<COPARNSegmentgroup11>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup11 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup12 {
    pub tmp: TMP,
    pub rng: Option<RNG>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup13 {
    pub eqd: EQD,
    pub rff: Vec<RFF>,
    pub eqn: Option<EQN>,
    pub tmd: Vec<TMD>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub loc: Vec<LOC>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub segment_group_14: Vec<COPARNSegmentgroup14>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub pcd: Vec<PCD>,
    pub segment_group_15: Vec<COPARNSegmentgroup15>,
    pub moa: Vec<MOA>,
    pub gor: Vec<GOR>,
    pub eqa: Option<EQA>,
    pub cod: Option<COD>,
    pub han: Vec<HAN>,
    pub segment_group_17: Vec<COPARNSegmentgroup17>,
    pub segment_group_18: Vec<COPARNSegmentgroup18>,
    pub segment_group_20: Vec<COPARNSegmentgroup20>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup14 {
    pub tmp: TMP,
    pub rng: Option<RNG>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup15 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub segment_group_16: Vec<COPARNSegmentgroup16>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup16 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup17 {
    pub dam: DAM,
    pub cod: Option<COD>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup18 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub segment_group_19: Vec<COPARNSegmentgroup19>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup19 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup2 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub segment_group_3: Vec<COPARNSegmentgroup3>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup20 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub cta: Option<CTA>,
    pub com: Option<COM>,
    pub rff: Vec<RFF>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup3 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup4 {
    pub nad: NAD,
    pub segment_group_5: Vec<COPARNSegmentgroup5>,
    pub rff: Vec<RFF>,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup5 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup6 {
    pub gid: GID,
    pub han: Vec<HAN>,
    pub ftx: Vec<FTX>,
    pub rff: Vec<RFF>,
    pub pia: Vec<PIA>,
    pub segment_group_7: Vec<COPARNSegmentgroup7>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub segment_group_8: Vec<COPARNSegmentgroup8>,
    pub segment_group_9: Vec<COPARNSegmentgroup9>,
    pub segment_group_10: Vec<COPARNSegmentgroup10>,
    pub segment_group_12: Vec<COPARNSegmentgroup12>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup7 {
    pub nad: NAD,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup8 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct COPARNSegmentgroup9 {
    pub sgp: SGP,
    pub mea: Vec<MEA>,
}
