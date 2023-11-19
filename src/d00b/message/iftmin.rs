use crate::d00b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseSg, ParseMsg};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseMsg)]
pub struct IFTMIN {
    pub unh: UNH,
    pub bgm: BGM,
    pub cta: Option<CTA>,
    pub com: Vec<COM>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub cux: Vec<CUX>,
    pub moa: Vec<MOA>,
    pub ftx: Vec<FTX>,
    pub cnt: Vec<CNT>,
    pub doc: Vec<DOC>,
    pub gds: Vec<GDS>,
    pub segment_group_1: Vec<IFTMINSegmentgroup1>,
    pub segment_group_2: Vec<IFTMINSegmentgroup2>,
    pub segment_group_3: Vec<IFTMINSegmentgroup3>,
    pub segment_group_4: Vec<IFTMINSegmentgroup4>,
    pub segment_group_6: Vec<IFTMINSegmentgroup6>,
    pub segment_group_7: Vec<IFTMINSegmentgroup7>,
    pub segment_group_8: Vec<IFTMINSegmentgroup8>,
    pub segment_group_11: Vec<IFTMINSegmentgroup11>,
    pub segment_group_18: Vec<IFTMINSegmentgroup18>,
    pub segment_group_37: Vec<IFTMINSegmentgroup37>,
    pub unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup1 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup10 {
    pub rff: RFF,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup11 {
    pub nad: NAD,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
    pub segment_group_12: Vec<IFTMINSegmentgroup12>,
    pub segment_group_13: Vec<IFTMINSegmentgroup13>,
    pub segment_group_14: Vec<IFTMINSegmentgroup14>,
    pub segment_group_15: Vec<IFTMINSegmentgroup15>,
    pub segment_group_16: Vec<IFTMINSegmentgroup16>,
    pub segment_group_17: Vec<IFTMINSegmentgroup17>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup12 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup13 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup14 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup15 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup16 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup17 {
    pub tsr: TSR,
    pub rff: Option<RFF>,
    pub loc: Option<LOC>,
    pub tpl: Option<TPL>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup18 {
    pub gid: GID,
    pub han: Vec<HAN>,
    pub tmp: Option<TMP>,
    pub rng: Option<RNG>,
    pub tmd: Option<TMD>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
    pub pia: Vec<PIA>,
    pub ftx: Vec<FTX>,
    pub pcd: Vec<PCD>,
    pub segment_group_19: Vec<IFTMINSegmentgroup19>,
    pub gds: Vec<GDS>,
    pub segment_group_20: Vec<IFTMINSegmentgroup20>,
    pub segment_group_21: Vec<IFTMINSegmentgroup21>,
    pub segment_group_22: Vec<IFTMINSegmentgroup22>,
    pub segment_group_23: Vec<IFTMINSegmentgroup23>,
    pub segment_group_24: Vec<IFTMINSegmentgroup24>,
    pub segment_group_25: Vec<IFTMINSegmentgroup25>,
    pub segment_group_27: Vec<IFTMINSegmentgroup27>,
    pub segment_group_29: Vec<IFTMINSegmentgroup29>,
    pub segment_group_31: Vec<IFTMINSegmentgroup31>,
    pub segment_group_32: Vec<IFTMINSegmentgroup32>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup19 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup2 {
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup20 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup21 {
    pub dim: DIM,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup22 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup23 {
    pub pci: PCI,
    pub rff: Option<RFF>,
    pub dtm: Option<DTM>,
    pub gin: Vec<GIN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup24 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup25 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub segment_group_26: Vec<IFTMINSegmentgroup26>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup26 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup27 {
    pub tpl: TPL,
    pub segment_group_28: Vec<IFTMINSegmentgroup28>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup28 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup29 {
    pub sgp: SGP,
    pub segment_group_30: Vec<IFTMINSegmentgroup30>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup3 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup30 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup31 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup32 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub segment_group_33: Vec<IFTMINSegmentgroup33>,
    pub segment_group_34: Vec<IFTMINSegmentgroup34>,
    pub segment_group_35: Vec<IFTMINSegmentgroup35>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup33 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup34 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup35 {
    pub sgp: SGP,
    pub segment_group_36: Vec<IFTMINSegmentgroup36>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup36 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup37 {
    pub eqd: EQD,
    pub eqn: Option<EQN>,
    pub tmd: Option<TMD>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub sel: Vec<SEL>,
    pub tpl: Vec<TPL>,
    pub han: Option<HAN>,
    pub tmp: Option<TMP>,
    pub ftx: Vec<FTX>,
    pub rff: Vec<RFF>,
    pub segment_group_38: Vec<IFTMINSegmentgroup38>,
    pub segment_group_39: Vec<IFTMINSegmentgroup39>,
    pub segment_group_41: Vec<IFTMINSegmentgroup41>,
    pub segment_group_42: Vec<IFTMINSegmentgroup42>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup38 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup39 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub segment_group_40: Vec<IFTMINSegmentgroup40>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup4 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub segment_group_5: Vec<IFTMINSegmentgroup5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup40 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup41 {
    pub eqa: EQA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup42 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub segment_group_43: Vec<IFTMINSegmentgroup43>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup43 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup5 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup6 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup7 {
    pub tcc: TCC,
    pub loc: Option<LOC>,
    pub ftx: Option<FTX>,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup8 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub segment_group_9: Vec<IFTMINSegmentgroup9>,
    pub segment_group_10: Vec<IFTMINSegmentgroup10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IFTMINSegmentgroup9 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}
