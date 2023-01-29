use std::fmt;

mod segment;
use macros::{DisplayEdifact, DisplayEdifactSg};
pub use segment::*;
mod types;
use serde::{Deserialize, Serialize};
pub use types::*;

#[cfg(test)]
mod test_iftsta;

/// A message to report the transport status and/or a change in the
/// transport status (i.e. event) between agreed parties.
///
/// https://service.unece.org/trade/untdid/d00b/trmd/iftsta_c.htm
#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Iftsta {
    unh: Unh,
    bgm: Bgm,
    dtm: Vec<Dtm>,
    tsr: Option<Tsr>,
    sg1: Vec<IftstaSg1>,
    sg3: Vec<IftstaSg3>,
    loc: Vec<Loc>,
    ftx: Vec<Ftx>,
    cnt: Vec<Cnt>,
    sg4: Vec<IftstaSg4>,
    unt: Unt,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg1 {
    nad: Nad,
    sg2: Vec<IftstaSg2>,
}


#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg2 {
    cta: Cta,
    com: Vec<Com>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg3 {
    rff: Rff,
    dtm: Option<Dtm>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg4 {
    cni: Cni,
    loc: Vec<Loc>,
    cnt: Vec<Cnt>,
    sg5: Vec<IftstaSg5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg5 {
    sts: Sts,
    rff: Vec<Rff>,
    dtm: Vec<Dtm>,
    doc: Option<Doc>,
    ftx: Vec<Ftx>,
    nad: Vec<Nad>,
    loc: Option<Loc>,
    pci: Vec<Pci>,
    sg6: Vec<IftstaSg6>,
    sg8: Vec<IftstaSg8>,
    sg10: Vec<IftstaSg10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg6 {
    tdt: Tdt,
    dtm: Vec<Dtm>,
    rff: Vec<Rff>,
    sg7: Vec<IftstaSg7>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg7 {
    loc: Loc,
    dtm: Vec<Dtm>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg8 {
    eqd: Eqd,
    mea: Vec<Mea>,
    dim: Vec<Dim>,
    sel: Vec<Sel>,
    rff: Vec<Rff>,
    tpl: Vec<Tpl>,
    tmd: Option<Tmd>,
    sg9: Vec<IftstaSg9>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg9 {
    eqa: Eqa,
    sel: Vec<Sel>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg10 {
    gid: Gid,
    han: Vec<Han>,
    sgp: Vec<Sgp>,
    dgs: Vec<Dgs>,
    ftx: Vec<Ftx>,
    sg11: Vec<IftstaSg11>,
    sg12: Vec<IftstaSg12>,
    sg13: Vec<IftstaSg13>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg11 {
    mea: Mea,
    eqn: Option<Eqn>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg12 {
    dim: Dim,
    eqn: Option<Eqn>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg13 {
    pci: Pci,
    gin: Vec<Gin>,
}