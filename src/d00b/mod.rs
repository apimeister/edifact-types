mod segment;
pub use segment::*;
mod types;
use serde::{Serialize, Deserialize};
pub use types::*;

#[cfg(test)]
mod test_iftsta;

/// A message to report the transport status and/or a change in the
/// transport status (i.e. event) between agreed parties.
///
/// https://service.unece.org/trade/untdid/d00b/trmd/iftsta_c.htm
#[derive(Debug, Serialize, Deserialize)]
pub struct Iftsta {
    _0010: Unh,
    _0020: Bgm,
    _0030: Dtm,
    _0040: Tsr,
    _0050: IftstaSg1,
    _0100: IftstaSg3,
    _0130: Loc,
    _0140: Ftx,
    _0150: Cnt,
    _0160: IftstaSg4,
    _0620: Unt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IftstaSg1 {
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IftstaSg3 {
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IftstaSg4 {
    
}