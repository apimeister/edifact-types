use serde::{Deserialize, Serialize};

use super::{_1225, _4343};

#[derive(Debug, Serialize, Deserialize)]
pub struct Unh {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unt {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bgm {
    pub _010: Option<C002>,
    pub _020: Option<C106>,
    pub _030: Option<_1225>,
    pub _040: Option<_4343>,
}

/// C002 DOCUMENT/MESSAGE NAME
#[derive(Debug, Serialize, Deserialize)]
pub struct C002 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
    pub _040: Option<String>,
}

/// C106 DOCUMENT/MESSAGE IDENTIFICATION
#[derive(Debug, Serialize, Deserialize)]
pub struct C106 {
    pub _010: Option<String>,
    pub _020: Option<String>,
    pub _030: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dtm {
    pub _01: String,
    pub _02: Option<String>,
    pub _03: Option<super::_2379>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tsr {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loc {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ftx {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cnt {}
