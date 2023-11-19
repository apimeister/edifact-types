use crate::d00b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseMsg, ParseSg};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseMsg)]
pub struct IFTSTA {
    pub unh: UNH,
    pub bgm: BGM,
    pub dtm: Vec<DTM>,
    pub tsr: Option<TSR>,
    pub segment_group_1: Vec<IFTSTASegmentgroup1>,
    pub segment_group_3: Vec<IFTSTASegmentgroup3>,
    pub loc: Vec<LOC>,
    pub ftx: Vec<FTX>,
    pub cnt: Vec<CNT>,
    pub segment_group_4: Vec<IFTSTASegmentgroup4>,
    pub unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup1 {
    pub nad: NAD,
    pub segment_group_2: Vec<IFTSTASegmentgroup2>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup10 {
    pub gid: GID,
    pub han: Vec<HAN>,
    pub sgp: Vec<SGP>,
    pub dgs: Vec<DGS>,
    pub ftx: Vec<FTX>,
    pub segment_group_11: Vec<IFTSTASegmentgroup11>,
    pub segment_group_12: Vec<IFTSTASegmentgroup12>,
    pub segment_group_13: Vec<IFTSTASegmentgroup13>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup11 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup12 {
    pub dim: DIM,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup13 {
    pub pci: PCI,
    pub gin: Vec<GIN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup2 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup3 {
    pub rff: RFF,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup4 {
    pub cni: CNI,
    pub loc: Vec<LOC>,
    pub cnt: Vec<CNT>,
    pub segment_group_5: Vec<IFTSTASegmentgroup5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup5 {
    pub sts: STS,
    pub rff: Vec<RFF>,
    pub dtm: Vec<DTM>,
    pub doc: Option<DOC>,
    pub ftx: Vec<FTX>,
    pub nad: Vec<NAD>,
    pub loc: Option<LOC>,
    pub pci: Vec<PCI>,
    pub segment_group_6: Vec<IFTSTASegmentgroup6>,
    pub segment_group_8: Vec<IFTSTASegmentgroup8>,
    pub segment_group_10: Vec<IFTSTASegmentgroup10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup6 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub segment_group_7: Vec<IFTSTASegmentgroup7>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup7 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup8 {
    pub eqd: EQD,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub sel: Vec<SEL>,
    pub rff: Vec<RFF>,
    pub tpl: Vec<TPL>,
    pub tmd: Option<TMD>,
    pub segment_group_9: Vec<IFTSTASegmentgroup9>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTSTASegmentgroup9 {
    pub eqa: EQA,
    pub sel: Vec<SEL>,
}
