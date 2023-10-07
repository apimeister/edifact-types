use std::fmt;

mod segment;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg};
pub use segment::*;
mod types;
use serde::{Deserialize, Serialize};
pub use types::*;

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
    cnt: Vec<Cnt>,
    sg4: Vec<IftstaSg4>,
    unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg1 {
    nad: NAD,
    sg2: Vec<IftstaSg2>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg2 {
    cta: Cta,
    com: Vec<Com>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg3 {
    rff: RFF,
    dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg4 {
    cni: Cni,
    loc: Vec<LOC>,
    cnt: Vec<Cnt>,
    sg5: Vec<IftstaSg5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg5 {
    sts: STS,
    rff: Vec<RFF>,
    dtm: Vec<DTM>,
    doc: Option<Doc>,
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
