use crate::d00b::*;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, ParseSg};
use nom::{combinator::opt, multi::many0};
use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact, ParseSg)]
pub struct Iftmin {
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
    pub segment_group_1: Vec<IftminSg1>,
    pub segment_group_2: Vec<IftminSg2>,
    pub segment_group_3: Vec<IftminSg3>,
    pub segment_group_4: Vec<IftminSg4>,
    pub segment_group_6: Vec<IftminSg6>,
    pub segment_group_7: Vec<IftminSg7>,
    pub segment_group_8: Vec<IftminSg8>,
    pub segment_group_11: Vec<IftminSg11>,
    pub segment_group_18: Vec<IftminSg18>,
    pub segment_group_37: Vec<IftminSg37>,
    pub unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg1 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg2 {
    pub tod: TOD,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg3 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg4 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub iftmin_sg5: Vec<IftminSg5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg5 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}
#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg6 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg7 {
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
pub struct IftminSg8 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub iftmin_sg9: Vec<IftminSg9>,
    pub iftmin_sg10: Vec<IftminSg10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg9 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg10 {
    pub rff: RFF,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg11 {
    pub nad: NAD,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
    pub iftmin_sg12: Vec<IftminSg12>,
    pub iftmin_sg13: Vec<IftminSg13>,
    pub iftmin_sg14: Vec<IftminSg14>,
    pub iftmin_sg15: Vec<IftminSg15>,
    pub iftmin_sg16: Vec<IftminSg16>,
    pub iftmin_sg17: Vec<IftminSg17>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg12 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg13 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg14 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg15 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg16 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg17 {
    pub tsr: TSR,
    pub rff: Option<RFF>,
    pub loc: Option<LOC>,
    pub tpl: Option<TPL>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg18 {
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
    pub iftmin_sg19: Vec<IftminSg19>,
    pub gds: Vec<GDS>,
    pub iftmin_sg20: Vec<IftminSg20>,
    pub iftmin_sg21: Vec<IftminSg21>,
    pub iftmin_sg22: Vec<IftminSg22>,
    pub iftmin_sg23: Vec<IftminSg23>,
    pub iftmin_sg24: Vec<IftminSg24>,
    pub iftmin_sg25: Vec<IftminSg25>,
    pub iftmin_sg27: Vec<IftminSg27>,
    pub iftmin_sg29: Vec<IftminSg29>,
    pub iftmin_sg31: Vec<IftminSg31>,
    pub iftmin_sg32: Vec<IftminSg32>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg19 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg20 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg21 {
    pub dim: DIM,
    pub eqn: Option<EQN>,
}
#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg22 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg23 {
    pub pci: PCI,
    pub rff: Option<RFF>,
    pub dtm: Option<DTM>,
    pub gin: Vec<GIN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg24 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg25 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub iftmin_sg26: Vec<IftminSg26>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg26 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg27 {
    pub tpl: TPL,
    pub iftmin_sg28: Vec<IftminSg28>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg28 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg29 {
    pub sgp: SGP,
    pub iftmin_sg30: Vec<IftminSg30>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg30 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg31 {
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
pub struct IftminSg32 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub iftmin_sg33: Vec<IftminSg33>,
    pub iftmin_sg34: Vec<IftminSg34>,
    pub iftmin_sg35: Vec<IftminSg35>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg33 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg34 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg35 {
    pub sgp: SGP,
    pub iftmin_sg36: Vec<IftminSg36>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg36 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg37 {
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
    pub iftmin_sg38: Vec<IftminSg38>,
    pub iftmin_sg39: Vec<IftminSg39>,
    pub iftmin_sg41: Vec<IftminSg41>,
    pub iftmin_sg42: Vec<IftminSg42>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg38 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg39 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub iftmin_sg40: Vec<IftminSg40>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg40 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg41 {
    pub eqa: EQA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg42 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub iftmin_sg43: Vec<IftminSg43>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]
pub struct IftminSg43 {
    pub cta: CTA,
    pub com: Vec<COM>,
}
