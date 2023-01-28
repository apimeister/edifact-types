mod segment;
pub use segment::*;
mod types;
use serde::{Serialize, Deserialize};
pub use types::*;

#[cfg(test)]
mod test_coprar;

#[cfg(test)]
mod test_segment;

/// Container discharge/loading order message
/// 
/// https://service.unece.org/trade/untdid/d95b/trmd/coprar_d.htm#MESDEF
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Coprar {
    pub unh: Unh,
    pub bgm: Bgm,
    pub ftx: Vec<Ftx>,
    pub rff: Vec<Rff>,
    pub sg1: CoprarSg1,
    pub sg2: Vec<CoprarSg2>,
    pub sg3: Vec<CoprarSg3>,
    pub cnt: Cnt,
    pub unt: Unt,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CoprarSg1{
    pub tdt: Tdt,
    pub rff: Vec<Rff>,
    pub loc: Vec<Loc>,
    pub dtm: Vec<Dtm>,
    pub ftx: Vec<Ftx>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CoprarSg2{
    pub nad: Nad,
    pub cta: Cta,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CoprarSg3{
    pub eqd: Eqd,
    pub rff: Vec<Rff>,
    pub eqn: Vec<Eqn>,
    pub tmd: Vec<Tmd>,
    pub dtm: Vec<Dtm>,
    pub loc: Vec<Loc>,
    pub mea: Vec<Mea>,
    pub dim: Vec<Dim>,
    pub tmp: Vec<Tmp>,
    pub rng: Vec<Rng>,
    pub sel: Vec<Sel>,
    pub ftx: Vec<Ftx>,
    pub dgs: Vec<Dgs>,
    pub eqa: Vec<Eqa>,
    pub sg4: Option<CoprarSg3Sg4>,
    pub nad: Option<Nad>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CoprarSg3Sg4{
    pub tdt: Tdt,
    pub rff: Option<Rff>,
    pub loc: Option<Loc>,
    pub dtm: Option<Dtm>,
}