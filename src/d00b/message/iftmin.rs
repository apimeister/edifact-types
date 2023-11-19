use crate::d00b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseSg};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseSg)]
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
    pub segment_group_1: Vec<Segmentgroup1>,
    pub segment_group_2: Vec<Segmentgroup2>,
    pub segment_group_3: Vec<Segmentgroup3>,
    pub segment_group_4: Vec<Segmentgroup4>,
    pub segment_group_6: Vec<Segmentgroup6>,
    pub segment_group_7: Vec<Segmentgroup7>,
    pub segment_group_8: Vec<Segmentgroup8>,
    pub segment_group_11: Vec<Segmentgroup11>,
    pub segment_group_18: Vec<Segmentgroup18>,
    pub segment_group_37: Vec<Segmentgroup37>,
    pub unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup1 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup10 {
    pub rff: RFF,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup11 {
    pub nad: NAD,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
    pub segment_group_12: Vec<Segmentgroup12>,
    pub segment_group_13: Vec<Segmentgroup13>,
    pub segment_group_14: Vec<Segmentgroup14>,
    pub segment_group_15: Vec<Segmentgroup15>,
    pub segment_group_16: Vec<Segmentgroup16>,
    pub segment_group_17: Vec<Segmentgroup17>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup12 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup13 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup14 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup15 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup16 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup17 {
    pub tsr: TSR,
    pub rff: Option<RFF>,
    pub loc: Option<LOC>,
    pub tpl: Option<TPL>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup18 {
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
    pub segment_group_19: Vec<Segmentgroup19>,
    pub gds: Vec<GDS>,
    pub segment_group_20: Vec<Segmentgroup20>,
    pub segment_group_21: Vec<Segmentgroup21>,
    pub segment_group_22: Vec<Segmentgroup22>,
    pub segment_group_23: Vec<Segmentgroup23>,
    pub segment_group_24: Vec<Segmentgroup24>,
    pub segment_group_25: Vec<Segmentgroup25>,
    pub segment_group_27: Vec<Segmentgroup27>,
    pub segment_group_29: Vec<Segmentgroup29>,
    pub segment_group_31: Vec<Segmentgroup31>,
    pub segment_group_32: Vec<Segmentgroup32>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup19 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup2 {
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup20 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup21 {
    pub dim: DIM,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup22 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup23 {
    pub pci: PCI,
    pub rff: Option<RFF>,
    pub dtm: Option<DTM>,
    pub gin: Vec<GIN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup24 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup25 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub segment_group_26: Vec<Segmentgroup26>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup26 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup27 {
    pub tpl: TPL,
    pub segment_group_28: Vec<Segmentgroup28>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup28 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup29 {
    pub sgp: SGP,
    pub segment_group_30: Vec<Segmentgroup30>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup3 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup30 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup31 {
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
pub struct Segmentgroup32 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub segment_group_33: Vec<Segmentgroup33>,
    pub segment_group_34: Vec<Segmentgroup34>,
    pub segment_group_35: Vec<Segmentgroup35>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup33 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup34 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup35 {
    pub sgp: SGP,
    pub segment_group_36: Vec<Segmentgroup36>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup36 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup37 {
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
    pub segment_group_38: Vec<Segmentgroup38>,
    pub segment_group_39: Vec<Segmentgroup39>,
    pub segment_group_41: Vec<Segmentgroup41>,
    pub segment_group_42: Vec<Segmentgroup42>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup38 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup39 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub segment_group_40: Vec<Segmentgroup40>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup4 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub segment_group_5: Vec<Segmentgroup5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup40 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup41 {
    pub eqa: EQA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup42 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub segment_group_43: Vec<Segmentgroup43>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup43 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup5 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup6 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup7 {
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
pub struct Segmentgroup8 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub segment_group_9: Vec<Segmentgroup9>,
    pub segment_group_10: Vec<Segmentgroup10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct Segmentgroup9 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}
