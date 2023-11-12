mod segment;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg, DisplayOuterSegment};
pub use segment::*;
mod types;
use serde::{Deserialize, Serialize};
use std::fmt;
pub use types::*;

#[cfg(test)]
mod test_coprar;

#[cfg(test)]
mod test_segment;

/// Container discharge/loading order message
///
/// https://service.unece.org/trade/untdid/d95b/trmd/coprar_d.htm#MESDEF
#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayEdifact)]
pub struct Coprar {
    pub unh: UNH,
    pub bgm: BGM,
    pub ftx: Vec<FTX>,
    pub rff: Vec<RFF>,
    pub sg1: CoprarSg1,
    pub sg2: Vec<CoprarSg2>,
    pub sg3: Vec<CoprarSg3>,
    pub cnt: CNT,
    pub unt: UNT,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayEdifactSg)]
pub struct CoprarSg1 {
    pub tdt: TDT,
    pub rff: Vec<RFF>,
    pub loc: Vec<LOC>,
    pub dtm: Vec<DTM>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayEdifactSg)]
pub struct CoprarSg2 {
    pub nad: NAD,
    pub cta: CTA,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayEdifactSg)]
pub struct CoprarSg3 {
    pub eqd: EQD,
    pub rff: Vec<RFF>,
    pub eqn: Vec<EQN>,
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
    pub sg4: Option<CoprarSg3Sg4>,
    pub nad: Option<NAD>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, DisplayEdifactSg)]
pub struct CoprarSg3Sg4 {
    pub tdt: TDT,
    pub rff: Option<RFF>,
    pub loc: Option<LOC>,
    pub dtm: Option<DTM>,
}
