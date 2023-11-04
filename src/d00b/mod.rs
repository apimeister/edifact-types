use crate::util::Parser;
use edifact_types_macros::{DisplayEdifact, DisplayEdifactSg};
use nom::{
    combinator::{opt, peek},
    multi::many0,
    IResult,
};
use serde::{Deserialize, Serialize};
use std::fmt;

mod types;
pub use types::*;
mod segment;
pub use segment::*;
mod element;
pub use element::*;

#[cfg(test)]
mod test_coparn;
#[cfg(test)]
mod test_iftmin;
#[cfg(test)]
mod test_iftsta;
#[cfg(test)]
mod test_segment;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, DisplayEdifact)]
pub struct Interchange<T>
where
    T: std::fmt::Display,
{
    pub una: Option<UNA>,
    pub unb: UNB,
    pub segment: T,
    pub unz: UNZ,
}

impl<'a, T: Default + Parser<&'a str, T, nom::error::Error<&'a str>> + std::fmt::Display>
    Parser<&'a str, Interchange<T>, nom::error::Error<&'a str>> for Interchange<T>
{
    fn parse(input: &'a str) -> IResult<&'a str, Interchange<T>> {
        let mut output = Interchange::default();
        let (input, obj) = opt(UNA::parse)(input)?;
        output.una = obj;
        let (input, obj) = UNB::parse(input)?;
        output.unb = obj;
        let (input, t_obj) = T::parse(input)?;
        output.segment = t_obj;
        let (input, obj) = UNZ::parse(input)?;
        output.unz = obj;
        Ok((input, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Coreor {
    pub unh: UNH,
    pub bgm: BGM,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Coreor, nom::error::Error<&'a str>> for Coreor {
    fn parse(input: &'a str) -> IResult<&'a str, Coreor> {
        let mut output = Coreor::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Coparn {
    pub unh: UNH,
    pub bgm: BGM,
    pub tmd: Option<TMD>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub ftx: Vec<FTX>,
    pub loc: Vec<LOC>,
    pub segment_group_1: Vec<CoparnSg1>,
    pub segment_group_2: Vec<CoparnSg2>,
    pub segment_group_4: Vec<CoparnSg4>,
    pub segment_group_6: Vec<CoparnSg6>,
    pub segment_group_13: Vec<CoparnSg13>,
    pub cnt: CNT,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Coparn, nom::error::Error<&'a str>> for Coparn {
    fn parse(input: &'a str) -> IResult<&'a str, Coparn> {
        let mut output = Coparn::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg1 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg2 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub segment_group_3: Vec<CoparnSg3>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg3 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg4 {
    pub nad: NAD,
    pub segment_group_5: Vec<CoparnSg5>,
    pub rff: Vec<RFF>,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg5 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg6 {
    pub gid: GID,
    pub han: Vec<HAN>,
    pub ftx: Vec<FTX>,
    pub rff: Vec<RFF>,
    pub pia: Vec<PIA>,
    pub segment_group_7: Vec<CoparnSg7>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub segment_group_8: Vec<CoparnSg8>,
    pub segment_group_9: Vec<CoparnSg9>,
    pub segment_group_10: Vec<CoparnSg10>,
    pub segment_group_12: Vec<CoparnSg12>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg7 {
    pub nad: NAD,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg8 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg9 {
    pub sgp: SGP,
    pub mea: Vec<MEA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg10 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub segment_group_11: Vec<CoparnSg11>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg11 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg12 {
    pub tmp: TMP,
    pub rng: RNG,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg13 {
    pub eqd: EQD,
    pub rff: Vec<RFF>,
    pub eqn: EQN,
    pub tmd: Vec<TMD>,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub loc: Vec<LOC>,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub segment_group_14: Vec<CoparnSg14>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub pcd: Vec<PCD>,
    pub segment_group_15: Vec<CoparnSg15>,
    pub moa: Vec<MOA>,
    pub gor: Vec<GOR>,
    pub eqa: EQA,
    pub cod: COD,
    pub han: Vec<HAN>,
    pub segment_group_17: Vec<CoparnSg17>,
    pub segment_group_18: Vec<CoparnSg18>,
    pub segment_group_20: Vec<CoparnSg20>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg14 {
    pub tmp: TMP,
    pub rng: Option<RNG>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg15 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub mea: Vec<MEA>,
    pub segment_group_16: Vec<CoparnSg16>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg16 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg17 {
    pub dam: DAM,
    pub cod: Option<COD>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg18 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub segment_group_19: Vec<CoparnSg19>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg19 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct CoparnSg20 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub cta: Option<CTA>,
    pub com: Option<COM>,
    pub rff: Vec<RFF>,
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Coprar {
    pub unh: UNH,
    pub bgm: BGM,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Coprar, nom::error::Error<&'a str>> for Coprar {
    fn parse(input: &'a str) -> IResult<&'a str, Coprar> {
        let mut output = Coprar::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Iftmbf {
    pub unh: UNH,
    pub bgm: BGM,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Iftmbf, nom::error::Error<&'a str>> for Iftmbf {
    fn parse(input: &'a str) -> IResult<&'a str, Iftmbf> {
        let mut output = Iftmbf::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Iftmcs {
    pub unh: UNH,
    pub bgm: BGM,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Iftmcs, nom::error::Error<&'a str>> for Iftmcs {
    fn parse(input: &'a str) -> IResult<&'a str, Iftmcs> {
        let mut output = Iftmcs::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
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

impl<'a> Parser<&'a str, Iftmin, nom::error::Error<&'a str>> for Iftmin {
    fn parse(mut input: &'a str) -> IResult<&'a str, Iftmin> {
        let mut output = Iftmin::default();
        (input, output.unh) = UNH::parse(input)?;
        (input, output.bgm) = BGM::parse(input)?;
        (input, output.cta) = opt(CTA::parse)(input)?;
        (input, output.com) = many0(COM::parse)(input)?;
        (input, output.dtm) = many0(DTM::parse)(input)?;
        (input, output.tsr) = many0(TSR::parse)(input)?;
        (input, output.cux) = many0(CUX::parse)(input)?;
        (input, output.moa) = many0(MOA::parse)(input)?;
        (input, output.ftx) = many0(FTX::parse)(input)?;
        (input, output.cnt) = many0(CNT::parse)(input)?;
        (input, output.doc) = many0(DOC::parse)(input)?;
        (input, output.gds) = many0(GDS::parse)(input)?;

        // Segment Group 1
        let mut loop_sg1 = vec![];
        while peek(opt(LOC::parse))(input)?.1.is_some() {
            let (outer_rest, loc) = LOC::parse(input)?;
            let (outer_rest, dtm) = many0(DTM::parse)(outer_rest)?;
            input = outer_rest;
            loop_sg1.push(IftminSg1 { loc, dtm });
        }
        output.segment_group_1 = loop_sg1;

        // Segment Group 2
        let mut loop_sg2 = vec![];
        while peek(opt(TOD::parse))(input)?.1.is_some() {
            let (outer_rest, tod) = TOD::parse(input)?;
            let (outer_rest, loc) = many0(LOC::parse)(outer_rest)?;
            input = outer_rest;
            loop_sg2.push(IftminSg2 { tod, loc });
        }
        output.segment_group_2 = loop_sg2;

        // Segment Group 3
        let mut loop_sg3 = vec![];
        while peek(opt(RFF::parse))(input)?.1.is_some() {
            let (outer_rest, rff) = RFF::parse(input)?;
            let (outer_rest, dtm) = many0(DTM::parse)(outer_rest)?;
            input = outer_rest;
            loop_sg3.push(IftminSg3 { rff, dtm });
        }
        output.segment_group_3 = loop_sg3;

        // Segment Group 4
        let mut loop_sg4 = vec![];
        while peek(opt(GOR::parse))(input)?.1.is_some() {
            let (outer_rest, gor) = GOR::parse(input)?;
            let (outer_rest, dtm) = many0(DTM::parse)(outer_rest)?;
            let (outer_rest, loc) = many0(LOC::parse)(outer_rest)?;
            let (outer_rest, sel) = many0(SEL::parse)(outer_rest)?;
            let (outer_rest, ftx) = many0(FTX::parse)(outer_rest)?;
            input = outer_rest;

            // Segment Group 5
            let mut iftmin_sg5 = vec![];
            while peek(opt(DOC::parse))(input)?.1.is_some() {
                let (inner_rest, inner_doc) = DOC::parse(input)?;
                let (inner_rest, inner_dtm) = opt(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg5.push(IftminSg5 {
                    doc: inner_doc,
                    dtm: inner_dtm,
                });
            }

            loop_sg4.push(IftminSg4 {
                gor,
                dtm,
                loc,
                sel,
                ftx,
                iftmin_sg5,
            });
        }
        output.segment_group_4 = loop_sg4;

        // Segment Group 6
        let mut loop_sg6 = vec![];
        while peek(opt(CPI::parse))(input)?.1.is_some() {
            let (outer_rest, cpi) = CPI::parse(input)?;
            let (outer_rest, rff) = many0(RFF::parse)(outer_rest)?;
            let (outer_rest, cux) = opt(CUX::parse)(outer_rest)?;
            let (outer_rest, loc) = many0(LOC::parse)(outer_rest)?;
            let (outer_rest, moa) = many0(MOA::parse)(outer_rest)?;
            input = outer_rest;
            loop_sg6.push(IftminSg6 {
                cpi,
                rff,
                cux,
                loc,
                moa,
            });
        }
        output.segment_group_6 = loop_sg6;

        // Segment Group 7
        let mut loop_sg7 = vec![];
        while peek(opt(TCC::parse))(input)?.1.is_some() {
            let (outer_rest, tcc) = TCC::parse(input)?;
            let (outer_rest, loc) = opt(LOC::parse)(outer_rest)?;
            let (outer_rest, ftx) = opt(FTX::parse)(outer_rest)?;
            let (outer_rest, cux) = opt(CUX::parse)(outer_rest)?;
            let (outer_rest, pri) = opt(PRI::parse)(outer_rest)?;
            let (outer_rest, eqn) = opt(EQN::parse)(outer_rest)?;
            let (outer_rest, pcd) = opt(PCD::parse)(outer_rest)?;
            let (outer_rest, moa) = many0(MOA::parse)(outer_rest)?;
            let (outer_rest, qty) = many0(QTY::parse)(outer_rest)?;
            input = outer_rest;
            loop_sg7.push(IftminSg7 {
                tcc,
                loc,
                ftx,
                cux,
                pri,
                eqn,
                pcd,
                moa,
                qty,
            });
        }
        output.segment_group_7 = loop_sg7;

        // Segment Group 8
        let mut loop_sg8 = vec![];
        while peek(opt(TDT::parse))(input)?.1.is_some() {
            let (outer_rest, tdt) = TDT::parse(input)?;
            let (outer_rest, dtm) = many0(DTM::parse)(outer_rest)?;
            let (outer_rest, tsr) = many0(TSR::parse)(outer_rest)?;
            input = outer_rest;

            // Segment Group 9
            let mut iftmin_sg9 = vec![];
            while peek(opt(LOC::parse))(input)?.1.is_some() {
                let (inner_rest, sg9_loc) = LOC::parse(input)?;
                let (inner_rest, sg9_dtm) = many0(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg9.push(IftminSg9 {
                    loc: sg9_loc,
                    dtm: sg9_dtm,
                });
            }

            // Segment Group 10
            let mut iftmin_sg10 = vec![];
            while peek(opt(RFF::parse))(input)?.1.is_some() {
                let (inner_rest, sg10_rff) = RFF::parse(input)?;
                let (inner_rest, sg10_dtm) = opt(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg10.push(IftminSg10 {
                    rff: sg10_rff,
                    dtm: sg10_dtm,
                });
            }

            loop_sg8.push(IftminSg8 {
                tdt,
                dtm,
                tsr,
                iftmin_sg9,
                iftmin_sg10,
            });
        }
        output.segment_group_8 = loop_sg8;

        // Segment Group 11
        let mut loop_sg11 = vec![];
        while peek(opt(NAD::parse))(input)?.1.is_some() {
            let (outer_rest, nad) = NAD::parse(input)?;
            let (outer_rest, loc) = many0(LOC::parse)(outer_rest)?;
            let (outer_rest, moa) = many0(MOA::parse)(outer_rest)?;
            input = outer_rest;
            // Segment Group 12
            let mut iftmin_sg12 = vec![];
            while peek(opt(CTA::parse))(input)?.1.is_some() {
                let (inner_rest, sg12_cta) = CTA::parse(input)?;
                let (inner_rest, sg12_com) = many0(COM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg12.push(IftminSg12 {
                    cta: sg12_cta,
                    com: sg12_com,
                });
            }
            // Segment Group 13
            let mut iftmin_sg13 = vec![];
            while peek(opt(DOC::parse))(input)?.1.is_some() {
                let (inner_rest, sg13_doc) = DOC::parse(input)?;
                let (inner_rest, sg13_dtm) = opt(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg13.push(IftminSg13 {
                    doc: sg13_doc,
                    dtm: sg13_dtm,
                });
            }
            // Segment Group 14
            let mut iftmin_sg14 = vec![];
            while peek(opt(TCC::parse))(input)?.1.is_some() {
                let (inner_rest, sg14_tcc) = TCC::parse(input)?;
                let (inner_rest, sg14_cux) = opt(CUX::parse)(inner_rest)?;
                let (inner_rest, sg14_pri) = opt(PRI::parse)(inner_rest)?;
                let (inner_rest, sg14_eqn) = opt(EQN::parse)(inner_rest)?;
                let (inner_rest, sg14_pcd) = opt(PCD::parse)(inner_rest)?;
                let (inner_rest, sg14_moa) = many0(MOA::parse)(inner_rest)?;
                let (inner_rest, sg14_qty) = many0(QTY::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg14.push(IftminSg14 {
                    tcc: sg14_tcc,
                    cux: sg14_cux,
                    pri: sg14_pri,
                    eqn: sg14_eqn,
                    pcd: sg14_pcd,
                    moa: sg14_moa,
                    qty: sg14_qty,
                });
            }
            // Segment Group 15
            let mut iftmin_sg15 = vec![];
            while peek(opt(RFF::parse))(input)?.1.is_some() {
                let (inner_rest, sg15_rff) = RFF::parse(input)?;
                let (inner_rest, sg15_dtm) = many0(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg15.push(IftminSg15 {
                    rff: sg15_rff,
                    dtm: sg15_dtm,
                });
            }
            // Segment Group 16
            let mut iftmin_sg16 = vec![];
            while peek(opt(CPI::parse))(input)?.1.is_some() {
                let (inner_rest, sg16_cpi) = CPI::parse(input)?;
                let (inner_rest, sg16_rff) = many0(RFF::parse)(inner_rest)?;
                let (inner_rest, sg16_cux) = opt(CUX::parse)(inner_rest)?;
                let (inner_rest, sg16_loc) = many0(LOC::parse)(inner_rest)?;
                let (inner_rest, sg16_moa) = many0(MOA::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg16.push(IftminSg16 {
                    cpi: sg16_cpi,
                    rff: sg16_rff,
                    cux: sg16_cux,
                    loc: sg16_loc,
                    moa: sg16_moa,
                });
            }
            // Segment Group 17
            let mut iftmin_sg17 = vec![];
            while peek(opt(TSR::parse))(input)?.1.is_some() {
                let (inner_rest, sg17_tsr) = TSR::parse(input)?;
                let (inner_rest, sg17_rff) = opt(RFF::parse)(inner_rest)?;
                let (inner_rest, sg17_loc) = opt(LOC::parse)(inner_rest)?;
                let (inner_rest, sg17_tpl) = opt(TPL::parse)(inner_rest)?;
                let (inner_rest, sg17_ftx) = many0(FTX::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg17.push(IftminSg17 {
                    tsr: sg17_tsr,
                    rff: sg17_rff,
                    loc: sg17_loc,
                    tpl: sg17_tpl,
                    ftx: sg17_ftx,
                });
            }

            loop_sg11.push(IftminSg11 {
                nad,
                loc,
                moa,
                iftmin_sg12,
                iftmin_sg13,
                iftmin_sg14,
                iftmin_sg15,
                iftmin_sg16,
                iftmin_sg17,
            });
        }
        output.segment_group_11 = loop_sg11;

        // Segment Group 18
        let mut loop_sg18 = vec![];
        while peek(opt(GID::parse))(input)?.1.is_some() {
            let (outer_rest, gid) = GID::parse(input)?;
            let (outer_rest, han) = many0(HAN::parse)(outer_rest)?;
            let (outer_rest, tmp) = opt(TMP::parse)(outer_rest)?;
            let (outer_rest, rng) = opt(RNG::parse)(outer_rest)?;
            let (outer_rest, tmd) = opt(TMD::parse)(outer_rest)?;
            let (outer_rest, loc) = many0(LOC::parse)(outer_rest)?;
            let (outer_rest, moa) = many0(MOA::parse)(outer_rest)?;
            let (outer_rest, pia) = many0(PIA::parse)(outer_rest)?;
            let (outer_rest, ftx) = many0(FTX::parse)(outer_rest)?;
            let (outer_rest, pcd) = many0(PCD::parse)(outer_rest)?;
            input = outer_rest;
            // Segment Group 19
            let mut iftmin_sg19 = vec![];
            while peek(opt(NAD::parse))(input)?.1.is_some() {
                let (inner_rest, inner_nad) = NAD::parse(input)?;
                let (inner_rest, inner_dtm) = opt(DTM::parse)(inner_rest)?;
                let (inner_rest, inner_loc) = many0(LOC::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg19.push(IftminSg19 {
                    nad: inner_nad,
                    dtm: inner_dtm,
                    loc: inner_loc,
                });
            }
            let (outer_rest, gds) = many0(GDS::parse)(input)?;
            input = outer_rest;
            // Segment Group 20
            let mut iftmin_sg20 = vec![];
            while peek(opt(MEA::parse))(input)?.1.is_some() {
                let (inner_rest, inner_mea) = MEA::parse(input)?;
                let (inner_rest, inner_eqn) = opt(EQN::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg20.push(IftminSg20 {
                    mea: inner_mea,
                    eqn: inner_eqn,
                });
            }
            // Segment Group 21
            let mut iftmin_sg21 = vec![];
            while peek(opt(DIM::parse))(input)?.1.is_some() {
                let (inner_rest, inner_dim) = DIM::parse(input)?;
                let (inner_rest, inner_eqn) = opt(EQN::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg21.push(IftminSg21 {
                    dim: inner_dim,
                    eqn: inner_eqn,
                });
            }
            // Segment Group 22
            let mut iftmin_sg22 = vec![];
            while peek(opt(RFF::parse))(input)?.1.is_some() {
                let (inner_rest, inner_rff) = RFF::parse(input)?;
                let (inner_rest, inner_dtm) = many0(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg22.push(IftminSg22 {
                    rff: inner_rff,
                    dtm: inner_dtm,
                });
            }
            // Segment Group 23
            let mut iftmin_sg23 = vec![];
            while peek(opt(PCI::parse))(input)?.1.is_some() {
                let (inner_rest, inner_pci) = PCI::parse(input)?;
                let (inner_rest, inner_rff) = opt(RFF::parse)(inner_rest)?;
                let (inner_rest, inner_dtm) = opt(DTM::parse)(inner_rest)?;
                let (inner_rest, inner_gin) = many0(GIN::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg23.push(IftminSg23 {
                    pci: inner_pci,
                    rff: inner_rff,
                    dtm: inner_dtm,
                    gin: inner_gin,
                });
            }
            // Segment Group 24
            let mut iftmin_sg24 = vec![];
            while peek(opt(DOC::parse))(input)?.1.is_some() {
                let (inner_rest, inner_doc) = DOC::parse(input)?;
                let (inner_rest, inner_dtm) = many0(DTM::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg24.push(IftminSg24 {
                    doc: inner_doc,
                    dtm: inner_dtm,
                });
            }
            // Segment Group 25
            let mut iftmin_sg25 = vec![];
            while peek(opt(GOR::parse))(input)?.1.is_some() {
                let (inner_rest, sg25_gor) = GOR::parse(input)?;
                let (inner_rest, sg25_dtm) = many0(DTM::parse)(inner_rest)?;
                let (inner_rest, sg25_loc) = many0(LOC::parse)(inner_rest)?;
                let (inner_rest, sg25_sel) = many0(SEL::parse)(inner_rest)?;
                let (inner_rest, sg25_ftx) = many0(FTX::parse)(inner_rest)?;
                input = inner_rest;
                // Segment Group 26
                let mut iftmin_sg26 = vec![];
                while peek(opt(DOC::parse))(input)?.1.is_some() {
                    let (inner_rest, sg26_doc) = DOC::parse(input)?;
                    let (inner_rest, sg26_dtm) = opt(DTM::parse)(inner_rest)?;
                    input = inner_rest;
                    iftmin_sg26.push(IftminSg26 {
                        doc: sg26_doc,
                        dtm: sg26_dtm,
                    });
                }
                iftmin_sg25.push(IftminSg25 {
                    gor: sg25_gor,
                    dtm: sg25_dtm,
                    loc: sg25_loc,
                    sel: sg25_sel,
                    ftx: sg25_ftx,
                    iftmin_sg26,
                });
            }

            // Segment Group 27
            let mut iftmin_sg27 = vec![];
            while peek(opt(TPL::parse))(input)?.1.is_some() {
                let (inner_rest, inner_tpl) = TPL::parse(input)?;
                input = inner_rest;
                // Segment Group 28
                let mut iftmin_sg28 = vec![];
                while peek(opt(DOC::parse))(input)?.1.is_some() {
                    let (sg28_inner_rest, sg28_inner_mea) = MEA::parse(input)?;
                    let (sg28_inner_rest, sg28_inner_eqn) = opt(EQN::parse)(sg28_inner_rest)?;
                    input = sg28_inner_rest;
                    iftmin_sg28.push(IftminSg28 {
                        mea: sg28_inner_mea,
                        eqn: sg28_inner_eqn,
                    });
                }
                iftmin_sg27.push(IftminSg27 {
                    tpl: inner_tpl,
                    iftmin_sg28,
                });
            }

            // Segment Group 29
            let mut iftmin_sg29 = vec![];
            while peek(opt(SGP::parse))(input)?.1.is_some() {
                let (inner_rest, inner_sgp) = SGP::parse(input)?;
                input = inner_rest;
                // Segment Group 30
                let mut iftmin_sg30 = vec![];
                while peek(opt(MEA::parse))(input)?.1.is_some() {
                    let (sg30_inner_rest, sg30_inner_mea) = MEA::parse(input)?;
                    let (sg30_inner_rest, sg30_inner_eqn) = opt(EQN::parse)(sg30_inner_rest)?;
                    input = sg30_inner_rest;
                    iftmin_sg30.push(IftminSg30 {
                        mea: sg30_inner_mea,
                        eqn: sg30_inner_eqn,
                    });
                }
                iftmin_sg29.push(IftminSg29 {
                    sgp: inner_sgp,
                    iftmin_sg30,
                });
            }

            // Segment Group 31
            let mut iftmin_sg31 = vec![];
            while peek(opt(TCC::parse))(input)?.1.is_some() {
                let (inner_rest, inner_tcc) = TCC::parse(input)?;
                let (inner_rest, inner_cux) = opt(CUX::parse)(inner_rest)?;
                let (inner_rest, inner_pri) = opt(PRI::parse)(inner_rest)?;
                let (inner_rest, inner_eqn) = opt(EQN::parse)(inner_rest)?;
                let (inner_rest, inner_pcd) = opt(PCD::parse)(inner_rest)?;
                let (inner_rest, inner_moa) = many0(MOA::parse)(inner_rest)?;
                let (inner_rest, inner_qty) = many0(QTY::parse)(inner_rest)?;
                let (inner_rest, inner_loc) = many0(LOC::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg31.push(IftminSg31 {
                    tcc: inner_tcc,
                    cux: inner_cux,
                    pri: inner_pri,
                    eqn: inner_eqn,
                    pcd: inner_pcd,
                    moa: inner_moa,
                    qty: inner_qty,
                    loc: inner_loc,
                });
            }

            // Segment Group 32
            let mut iftmin_sg32 = vec![];
            while peek(opt(DGS::parse))(input)?.1.is_some() {
                let (inner_rest, inner_dgs) = DGS::parse(input)?;
                let (inner_rest, inner_ftx) = many0(FTX::parse)(inner_rest)?;
                input = inner_rest;
                // Segment Group 33
                let mut iftmin_sg33 = vec![];
                while peek(opt(CTA::parse))(input)?.1.is_some() {
                    let (sg33_inner_rest, sg33_inner_cta) = CTA::parse(input)?;
                    let (sg33_inner_rest, sg33_inner_com) = many0(COM::parse)(sg33_inner_rest)?;
                    input = sg33_inner_rest;
                    iftmin_sg33.push(IftminSg33 {
                        cta: sg33_inner_cta,
                        com: sg33_inner_com,
                    });
                }
                // Segment Group 34
                let mut iftmin_sg34 = vec![];
                while peek(opt(MEA::parse))(input)?.1.is_some() {
                    let (sg34_inner_rest, sg34_inner_mea) = MEA::parse(input)?;
                    let (sg34_inner_rest, sg34_inner_eqn) = opt(EQN::parse)(sg34_inner_rest)?;
                    input = sg34_inner_rest;
                    iftmin_sg34.push(IftminSg34 {
                        mea: sg34_inner_mea,
                        eqn: sg34_inner_eqn,
                    });
                }
                // Segment Group 35
                let mut iftmin_sg35 = vec![];
                while peek(opt(SGP::parse))(input)?.1.is_some() {
                    let (sg35_inner_rest, sg35_inner_sgp) = SGP::parse(input)?;
                    input = sg35_inner_rest;
                    // Segment Group 36
                    let mut iftmin_sg36 = vec![];
                    while peek(opt(MEA::parse))(input)?.1.is_some() {
                        let (sg36_inner_rest, sg36_inner_mea) = MEA::parse(input)?;
                        let (sg36_inner_rest, sg36_inner_eqn) = opt(EQN::parse)(sg36_inner_rest)?;
                        input = sg36_inner_rest;
                        iftmin_sg36.push(IftminSg36 {
                            mea: sg36_inner_mea,
                            eqn: sg36_inner_eqn,
                        });
                    }
                    iftmin_sg35.push(IftminSg35 {
                        sgp: sg35_inner_sgp,
                        iftmin_sg36,
                    });
                }

                iftmin_sg32.push(IftminSg32 {
                    dgs: inner_dgs,
                    ftx: inner_ftx,
                    iftmin_sg33,
                    iftmin_sg34,
                    iftmin_sg35,
                });
            }

            loop_sg18.push(IftminSg18 {
                gid,
                han,
                tmp,
                rng,
                tmd,
                loc,
                moa,
                pia,
                ftx,
                pcd,
                iftmin_sg19,
                gds,
                iftmin_sg20,
                iftmin_sg21,
                iftmin_sg22,
                iftmin_sg23,
                iftmin_sg24,
                iftmin_sg25,
                iftmin_sg27,
                iftmin_sg29,
                iftmin_sg31,
                iftmin_sg32,
            });
        }
        output.segment_group_18 = loop_sg18;

        // Segment Group 37
        let mut loop_sg37 = vec![];
        while peek(opt(EQD::parse))(input)?.1.is_some() {
            let (outer_rest, eqd) = EQD::parse(input)?;
            let (outer_rest, eqn) = opt(EQN::parse)(outer_rest)?;
            let (outer_rest, tmd) = opt(TMD::parse)(outer_rest)?;
            let (outer_rest, mea) = many0(MEA::parse)(outer_rest)?;
            let (outer_rest, dim) = many0(DIM::parse)(outer_rest)?;
            let (outer_rest, sel) = many0(SEL::parse)(outer_rest)?;
            let (outer_rest, tpl) = many0(TPL::parse)(outer_rest)?;
            let (outer_rest, han) = opt(HAN::parse)(outer_rest)?;
            let (outer_rest, tmp) = opt(TMP::parse)(outer_rest)?;
            let (outer_rest, ftx) = many0(FTX::parse)(outer_rest)?;
            let (outer_rest, rff) = many0(RFF::parse)(outer_rest)?;
            input = outer_rest;
            // Segment Group 38
            let mut iftmin_sg38 = vec![];
            while peek(opt(TCC::parse))(input)?.1.is_some() {
                let (inner_rest, sg38_tcc) = TCC::parse(input)?;
                let (inner_rest, sg38_cux) = opt(CUX::parse)(inner_rest)?;
                let (inner_rest, sg38_pri) = opt(PRI::parse)(inner_rest)?;
                let (inner_rest, sg38_eqn) = opt(EQN::parse)(inner_rest)?;
                let (inner_rest, sg38_pcd) = opt(PCD::parse)(inner_rest)?;
                let (inner_rest, sg38_moa) = many0(MOA::parse)(inner_rest)?;
                let (inner_rest, sg38_qty) = many0(QTY::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg38.push(IftminSg38 {
                    tcc: sg38_tcc,
                    cux: sg38_cux,
                    pri: sg38_pri,
                    eqn: sg38_eqn,
                    pcd: sg38_pcd,
                    moa: sg38_moa,
                    qty: sg38_qty,
                });
            }
            // Segment Group 39
            let mut iftmin_sg39 = vec![];
            while peek(opt(NAD::parse))(input)?.1.is_some() {
                let (inner_rest, sg39_nad) = NAD::parse(input)?;
                let (inner_rest, sg39_dtm) = opt(DTM::parse)(inner_rest)?;
                input = inner_rest;
                // Segment Group 40
                let mut iftmin_sg40 = vec![];
                while peek(opt(CTA::parse))(input)?.1.is_some() {
                    let (inner_rest, sg40_cta) = CTA::parse(input)?;
                    let (inner_rest, sg40_com) = many0(COM::parse)(inner_rest)?;
                    input = inner_rest;

                    iftmin_sg40.push(IftminSg40 {
                        cta: sg40_cta,
                        com: sg40_com,
                    });
                }

                iftmin_sg39.push(IftminSg39 {
                    nad: sg39_nad,
                    dtm: sg39_dtm,
                    iftmin_sg40,
                });
            }

            // Segment Group 41
            let mut iftmin_sg41 = vec![];
            while peek(opt(EQA::parse))(input)?.1.is_some() {
                let (inner_rest, sg41_eqa) = EQA::parse(input)?;
                let (inner_rest, sg41_eqn) = opt(EQN::parse)(inner_rest)?;
                input = inner_rest;
                iftmin_sg41.push(IftminSg41 {
                    eqa: sg41_eqa,
                    eqn: sg41_eqn,
                });
            }

            // Segment Group 42
            let mut iftmin_sg42 = vec![];
            while peek(opt(DGS::parse))(input)?.1.is_some() {
                let (inner_rest, sg42_dgs) = DGS::parse(input)?;
                let (inner_rest, sg42_ftx) = many0(FTX::parse)(inner_rest)?;
                input = inner_rest;
                // Segment Group 43
                let mut iftmin_sg43 = vec![];
                while peek(opt(CTA::parse))(input)?.1.is_some() {
                    let (inner_rest, sg43_cta) = CTA::parse(input)?;
                    let (inner_rest, sg43_com) = many0(COM::parse)(inner_rest)?;
                    input = inner_rest;

                    iftmin_sg43.push(IftminSg43 {
                        cta: sg43_cta,
                        com: sg43_com,
                    });
                }

                iftmin_sg42.push(IftminSg42 {
                    dgs: sg42_dgs,
                    ftx: sg42_ftx,
                    iftmin_sg43,
                });
            }
            loop_sg37.push(IftminSg37 {
                eqd,
                eqn,
                tmd,
                mea,
                dim,
                sel,
                tpl,
                han,
                tmp,
                ftx,
                rff,
                iftmin_sg38,
                iftmin_sg39,
                iftmin_sg41,
                iftmin_sg42,
            });
        }
        output.segment_group_37 = loop_sg37;

        let (input, obj) = UNT::parse(input)?;
        output.unt = obj;
        Ok((input, output))
    }
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg1 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg2 {
    pub tod: TOD,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg3 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg4 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub iftmin_sg5: Vec<IftminSg5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg5 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg6 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
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

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg8 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub tsr: Vec<TSR>,
    pub iftmin_sg9: Vec<IftminSg9>,
    pub iftmin_sg10: Vec<IftminSg10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg9 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg10 {
    pub rff: RFF,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
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

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg12 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg13 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg14 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg15 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg16 {
    pub cpi: CPI,
    pub rff: Vec<RFF>,
    pub cux: Option<CUX>,
    pub loc: Vec<LOC>,
    pub moa: Vec<MOA>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg17 {
    pub tsr: TSR,
    pub rff: Option<RFF>,
    pub loc: Option<LOC>,
    pub tpl: Option<TPL>,
    pub ftx: Vec<FTX>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
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

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg19 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub loc: Vec<LOC>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg20 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg21 {
    pub dim: DIM,
    pub eqn: Option<EQN>,
}
#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg22 {
    pub rff: RFF,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg23 {
    pub pci: PCI,
    pub rff: Option<RFF>,
    pub dtm: Option<DTM>,
    pub gin: Vec<GIN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg24 {
    pub doc: DOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg25 {
    pub gor: GOR,
    pub dtm: Vec<DTM>,
    pub loc: Vec<LOC>,
    pub sel: Vec<SEL>,
    pub ftx: Vec<FTX>,
    pub iftmin_sg26: Vec<IftminSg26>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg26 {
    pub doc: DOC,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg27 {
    pub tpl: TPL,
    pub iftmin_sg28: Vec<IftminSg28>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg28 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg29 {
    pub sgp: SGP,
    pub iftmin_sg30: Vec<IftminSg30>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg30 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
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

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg32 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub iftmin_sg33: Vec<IftminSg33>,
    pub iftmin_sg34: Vec<IftminSg34>,
    pub iftmin_sg35: Vec<IftminSg35>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg33 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg34 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg35 {
    pub sgp: SGP,
    pub iftmin_sg36: Vec<IftminSg36>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg36 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
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

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg38 {
    pub tcc: TCC,
    pub cux: Option<CUX>,
    pub pri: Option<PRI>,
    pub eqn: Option<EQN>,
    pub pcd: Option<PCD>,
    pub moa: Vec<MOA>,
    pub qty: Vec<QTY>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg39 {
    pub nad: NAD,
    pub dtm: Option<DTM>,
    pub iftmin_sg40: Vec<IftminSg40>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg40 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg41 {
    pub eqa: EQA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg42 {
    pub dgs: DGS,
    pub ftx: Vec<FTX>,
    pub iftmin_sg43: Vec<IftminSg43>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftminSg43 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

/// A message to report the transport status and/or a change in the
/// transport status (i.e. event) between agreed parties.
///
/// https://service.unece.org/trade/untdid/d00b/trmd/iftsta_c.htm
#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]
pub struct Iftsta {
    pub unh: UNH,
    pub bgm: BGM,
    pub dtm: Vec<DTM>,
    pub tsr: Option<TSR>,
    pub segment_group_1: Vec<IftstaSg1>,
    pub segment_group_3: Vec<IftstaSg3>,
    pub loc: Vec<LOC>,
    pub ftx: Vec<FTX>,
    pub cnt: Vec<CNT>,
    pub segment_group_4: Vec<IftstaSg4>,
    pub unt: UNT,
}

impl<'a> Parser<&'a str, Iftsta, nom::error::Error<&'a str>> for Iftsta {
    fn parse(input: &'a str) -> IResult<&'a str, Iftsta> {
        let mut output = Iftsta::default();
        let (rest, obj) = UNH::parse(input)?;
        output.unh = obj;
        let (rest, obj) = BGM::parse(rest)?;
        output.bgm = obj;
        let (rest, obj) = many0(DTM::parse)(rest)?;
        output.dtm = obj;
        let (rest, obj) = opt(TSR::parse)(rest)?;
        output.tsr = obj;
        // segment group 1
        let mut loop_sg1 = vec![];
        let mut loop_rest = rest;
        while peek(opt(NAD::parse))(loop_rest)?.1.is_some() {
            let (rest, nad) = NAD::parse(loop_rest)?;
            loop_rest = rest;
            loop_sg1.push(IftstaSg1 {
                nad,
                segment_group_2: vec![],
            });
        }
        let rest = loop_rest;
        output.segment_group_1 = loop_sg1;
        // segment group 3
        let mut loop_sg3 = vec![];
        let mut loop_rest = rest;
        while peek(opt(RFF::parse))(loop_rest)?.1.is_some() {
            let (rest, rff) = RFF::parse(loop_rest)?;
            let (rest, dtm) = opt(DTM::parse)(rest)?;
            loop_rest = rest;
            loop_sg3.push(IftstaSg3 { rff, dtm });
        }
        let rest = loop_rest;
        output.segment_group_3 = loop_sg3;
        let (rest, obj) = many0(LOC::parse)(rest)?;
        output.loc = obj;
        let (rest, obj) = many0(FTX::parse)(rest)?;
        output.ftx = obj;
        let (rest, obj) = many0(CNT::parse)(rest)?;
        output.cnt = obj;
        // segment group 4
        let mut loop_sg4 = vec![];
        let mut loop_rest = rest;
        while peek(opt(CNI::parse))(loop_rest)?.1.is_some() {
            let (rest, cni) = CNI::parse(loop_rest)?;
            let (rest, loc) = many0(LOC::parse)(rest)?;
            let (rest, cnt) = many0(CNT::parse)(rest)?;
            loop_rest = rest;
            // segment group 5
            let mut loop_sg5 = vec![];
            while peek(opt(STS::parse))(loop_rest)?.1.is_some() {
                let (rest, sts) = STS::parse(loop_rest)?;
                let (rest, rff) = many0(RFF::parse)(rest)?;
                let (rest, dtm) = many0(DTM::parse)(rest)?;
                let (rest, doc) = opt(DOC::parse)(rest)?;
                let (rest, ftx) = many0(FTX::parse)(rest)?;
                let (rest, nad) = many0(NAD::parse)(rest)?;
                let (rest, loc) = opt(LOC::parse)(rest)?;
                let (rest, pci) = many0(PCI::parse)(rest)?;
                loop_rest = rest;
                // segment group 6
                let mut loop_sg6 = vec![];
                while peek(opt(TDT::parse))(loop_rest)?.1.is_some() {
                    let (rest, tdt) = TDT::parse(rest)?;
                    let (rest, dtm) = many0(DTM::parse)(rest)?;
                    let (rest, rff) = many0(RFF::parse)(rest)?;
                    loop_rest = rest;
                    // segment group 7
                    let mut loop_sg7 = vec![];
                    while peek(opt(LOC::parse))(loop_rest)?.1.is_some() {
                        let (rest, loc) = LOC::parse(loop_rest)?;
                        let (rest, dtm) = many0(DTM::parse)(rest)?;
                        loop_rest = rest;
                        loop_sg7.push(IftstaSg7 { loc, dtm });
                    }
                    loop_sg6.push(IftstaSg6 {
                        tdt,
                        dtm,
                        rff,
                        segment_group_7: loop_sg7,
                    });
                }
                // segment group 8
                let mut loop_sg8 = vec![];
                while peek(opt(EQD::parse))(loop_rest)?.1.is_some() {
                    let (rest, eqd) = EQD::parse(loop_rest)?;
                    let (rest, mea) = many0(MEA::parse)(rest)?;
                    let (rest, dim) = many0(DIM::parse)(rest)?;
                    let (rest, sel) = many0(SEL::parse)(rest)?;
                    let (rest, rff) = many0(RFF::parse)(rest)?;
                    let (rest, tpl) = many0(TPL::parse)(rest)?;
                    let (rest, tmd) = opt(TMD::parse)(rest)?;
                    loop_rest = rest;
                    loop_sg8.push(IftstaSg8 {
                        eqd,
                        mea,
                        dim,
                        sel,
                        rff,
                        tpl,
                        tmd,
                        segment_group_9: vec![],
                    });
                }
                loop_sg5.push(IftstaSg5 {
                    sts,
                    rff,
                    dtm,
                    doc,
                    ftx,
                    nad,
                    loc,
                    pci,
                    segment_group_6: loop_sg6,
                    segment_group_8: loop_sg8,
                    segment_group_10: vec![],
                });
            }

            loop_sg4.push(IftstaSg4 {
                cni,
                loc,
                cnt,
                segment_group_5: loop_sg5,
            });
        }
        let rest = loop_rest;
        output.segment_group_4 = loop_sg4;
        let (rest, obj) = UNT::parse(rest)?;
        output.unt = obj;
        Ok((rest, output))
    }
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg1 {
    pub nad: NAD,
    pub segment_group_2: Vec<IftstaSg2>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg2 {
    pub cta: CTA,
    pub com: Vec<COM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg3 {
    pub rff: RFF,
    pub dtm: Option<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg4 {
    pub cni: CNI,
    pub loc: Vec<LOC>,
    pub cnt: Vec<CNT>,
    pub segment_group_5: Vec<IftstaSg5>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg5 {
    pub sts: STS,
    pub rff: Vec<RFF>,
    pub dtm: Vec<DTM>,
    pub doc: Option<DOC>,
    pub ftx: Vec<FTX>,
    pub nad: Vec<NAD>,
    pub loc: Option<LOC>,
    pub pci: Vec<PCI>,
    pub segment_group_6: Vec<IftstaSg6>,
    pub segment_group_8: Vec<IftstaSg8>,
    pub segment_group_10: Vec<IftstaSg10>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg6 {
    pub tdt: TDT,
    pub dtm: Vec<DTM>,
    pub rff: Vec<RFF>,
    pub segment_group_7: Vec<IftstaSg7>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg7 {
    pub loc: LOC,
    pub dtm: Vec<DTM>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg8 {
    pub eqd: EQD,
    pub mea: Vec<MEA>,
    pub dim: Vec<DIM>,
    pub sel: Vec<SEL>,
    pub rff: Vec<RFF>,
    pub tpl: Vec<TPL>,
    pub tmd: Option<TMD>,
    pub segment_group_9: Vec<IftstaSg9>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg9 {
    pub eqa: EQA,
    pub sel: Vec<SEL>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg10 {
    pub gid: GID,
    pub han: Vec<HAN>,
    pub sgp: Vec<SGP>,
    pub dgs: Vec<DGS>,
    pub ftx: Vec<FTX>,
    pub segment_group_11: Vec<IftstaSg11>,
    pub segment_group_12: Vec<IftstaSg12>,
    pub segment_group_13: Vec<IftstaSg13>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg11 {
    pub mea: MEA,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg12 {
    pub dim: DIM,
    pub eqn: Option<EQN>,
}

#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg)]
pub struct IftstaSg13 {
    pub pci: PCI,
    pub gin: Vec<GIN>,
}
