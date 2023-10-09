use std::vec;

use super::*;

#[test]
fn build_dtm() {
    let dtm = DTM {
        _010: C507 {
            _010: "137".to_string(),
            _020: Some("202201010021".to_string()),
            _030: Some(super::_2379::_203),
        },
    };
    println!("{dtm:?}");
    let str = format!("{dtm}");
    println!("{}", str);
    assert_eq!(str, "DTM+137:202201010021:203");
}

#[test]
fn build_bgm() {
    let bgm = BGM {
        _010: Some(C002 {
            _010: Some("23".to_string()),
            _020: None,
            _030: None,
            _040: None,
        }),
        _020: Some(C106 {
            _010: Some("2BOG129382".to_string()),
            _020: None,
            _030: None,
        }),
        _030: Some(_1225::_9),
        _040: None,
    };
    println!("{bgm:?}");
    let str = format!("{bgm}");
    println!("{}", str);
    assert_eq!(str, "BGM+23+2BOG129382+9'\n");
}

#[test]
fn build_iftsta() {
    // TODO complete test case
    let ifsta = Iftsta {
        unh: UNH {
            _010: "2805567".to_string(),
            _020: S009 {
                _010: "IFTSTA".to_string(),
                _020: "D".to_string(),
                _030: "00B".to_string(),
                _040: "UN".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
        bgm: BGM {
            _010: Some(C002 {
                _010: Some("23".to_string()),
                _020: None,
                _030: None,
                _040: None,
            }),
            _020: Some(C106 {
                _010: Some("2BOG129382".to_string()),
                _020: None,
                _030: None,
            }),
            _030: Some(_1225::_9),
            _040: None,
        },
        dtm: vec![DTM {
            _010: C507 {
                _010: "137".to_string(),
                _020: Some("202201010021".to_string()),
                _030: Some(crate::d00b::_2379::_203),
            },
        }],
        segment_group_1: vec![IftstaSg1 {
            nad: NAD {
                _010: "CA".to_string(),
                _020: Some(C082 {
                    _010: "ABCD".to_string(),
                    ..Default::default()
                }),
                _040: Some(C080 {
                    _010: "SENDER-COMP".to_string(),
                    ..Default::default()
                }),
                _050: Some(C059 {
                    _010: Some("STREET 1".to_string()),
                    ..Default::default()
                }),
                _060: Some("CITY1".to_string()),
                _080: Some("99999".to_string()),
                _090: Some("DE".to_string()),
                ..Default::default()
            },
            segment_group_2: vec![],
        }],
        segment_group_3: vec![
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "4F".to_string(),
                        _020: Some("100113938".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "AAZ".to_string(),
                        _020: Some("ABCD".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "BN".to_string(),
                        _020: Some("24O4023815".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "BM".to_string(),
                        _020: Some("ABCD224O4023815X".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "EQ".to_string(),
                        _020: Some("TRHU4307252".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "CO".to_string(),
                        _020: Some("2023521591".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
            IftstaSg3 {
                rff: RFF {
                    _010: C506 {
                        _010: "FF".to_string(),
                        _020: Some("31075687".to_string()),
                        ..Default::default()
                    },
                },
                dtm: None,
            },
        ],
        segment_group_4: vec![IftstaSg4 {
            cni: CNI {
                _010: Some("1".to_string()),
                ..Default::default()
            },
            loc: vec![],
            cnt: vec![],
            // STS+1+P::HS:Rail Departed from In-Transit Locat'
            segment_group_5: vec![IftstaSg5 {
                sts: STS {
                    _010: Some(C601 {
                        _010: "1".to_string(),
                        ..Default::default()
                    }),
                    _020: Some(C555 {
                        _010: "P".to_string(),
                        _020: None,
                        _030: Some("HS".to_string()),
                        _040: Some("Rail Departed from In-Transit Locat".to_string()),
                    }),
                    ..Default::default()
                },
                rff: vec![
                    RFF {
                        _010: C506 {
                            _010: "BM".to_string(),
                            _020: Some("ABCD224O4023815X".to_string()),
                            ..Default::default()
                        },
                    },
                    RFF {
                        _010: C506 {
                            _010: "BN".to_string(),
                            _020: Some("24O4023815".to_string()),
                            ..Default::default()
                        },
                    },
                    RFF {
                        _010: C506 {
                            _010: "EQ".to_string(),
                            _020: Some("TRHU4561222".to_string()),
                            ..Default::default()
                        },
                    },
                ],
                dtm: vec![DTM {
                    _010: C507 {
                        _010: "334".to_string(),
                        _020: Some("202211190710".to_string()),
                        _030: Some(_2379::_203),
                    },
                }],
                doc: None,
                ftx: vec![],
                nad: vec![],
                loc: Some(LOC {
                    _010: "175".to_string(),
                    _020: Some(C517 {
                        _010: Some("USSYR".to_string()),
                        _020: Some("227".to_string()),
                        _030: None,
                        _040: Some("SYRACUSE".to_string()),
                    }),
                    _030: Some(C519 {
                        _010: Some("US".to_string()),
                        _020: Some("162".to_string()),
                        _030: None,
                        _040: None,
                    }),
                    _040: None,
                    _050: None,
                }),
                pci: vec![],
                segment_group_6: vec![],
                segment_group_8: vec![],
                segment_group_10: vec![],
            }],
        }],
        ..Default::default()
    };
    println!("{ifsta:?}");
    let str = format!("{ifsta}");
    println!("{}", str);
    let target_str = r#"UNH+2805567+IFTSTA:D:00B:UN'
BGM+23+2BOG129382+9'
DTM+137:202201010021:203'
NAD+CA+ABCD:160++SENDER-COMP+STREET 1+CITY1++99999+DE'
RFF+4F:100113938'
RFF+AAZ:ABCD'
RFF+BN:24O4023815'
RFF+BM:ABCD224O4023815X'
RFF+EQ:TRHU4307252'
RFF+CO:2023521591'
RFF+FF:31075687'
CNI+1'
STS+1+P::HS:Rail Departed from In-Transit Locat'
RFF+BM:ABCD224O4023815X'
RFF+BN:24O4023815'
RFF+EQ:TRHU4561222'
DTM+334:202211190710:203'
LOC+175+USSYR:227::SYRACUSE+US:162'
TDT+1++2+25+ABCD:172::SENDER-COMP+++:::CSX TRANSPORTATION'
LOC+9+COCTG:139::SOCIEDAD PORTUARIA DE CARTAGEN, , CO'
DTM+186:202211091415:203'
LOC+11+USNYC:139::APM TERMINALS ELIZABETH, , US'
DTM+178:202211170505:203'
DTM+133:202211181630:203'
LOC+29+USCHI:139::CSX 59TH STREET RAMP, , US'
DTM+132:202211210600:203'
DTM+133:202211210600:203'
LOC+7+USWOQ:139::WOODRIDGE, IL, US'
DTM+132:202211210700:203'
EQD+CN+TRHU4561222+45G1:102:5+++5'
UNT+31+2805567'"#;
    assert_eq!(str, target_str);
}

#[test]
fn parse_iftsta() {
    let input_str = r#"UNH+2805567+IFTSTA:D:00B:UN'
BGM+23+2BOG129382+9'
DTM+137:202201010021:203'
NAD+CA+ABCD:160++SENDER-COMP+STREET 1+CITY1++99999+DE'
RFF+4F:100113938'
RFF+AAZ:ABCD'
RFF+BN:24O4023815'
RFF+BM:ABCD224O4023815X'
RFF+EQ:TRHU4307252'
RFF+CO:2023521591'
RFF+FF:31075687'
CNI+1'
STS+1+P::HS:Rail Departed from In-Transit Locat'
RFF+BM:ABCD224O4023815X'
RFF+BN:24O4023815'
RFF+EQ:TRHU4561222'
DTM+334:202211190710:203'
LOC+175+USSYR:227::SYRACUSE+US:162'
TDT+1++2+25+ABCD:172::SENDER-COMP+++:::CSX TRANSPORTATION'
LOC+9+COCTG:139::SOCIEDAD PORTUARIA DE CARTAGEN, , CO'
DTM+186:202211091415:203'
LOC+11+USNYC:139::APM TERMINALS ELIZABETH, , US'
DTM+178:202211170505:203'
DTM+133:202211181630:203'
LOC+29+USCHI:139::CSX 59TH STREET RAMP, , US'
DTM+132:202211210600:203'
DTM+133:202211210600:203'
LOC+7+USWOQ:139::WOODRIDGE, IL, US'
DTM+132:202211210700:203'
EQD+CN+TRHU4561222+45G1:102:5+++5'
UNT+31+2805567'"#;
    let (rest, obj) = Iftsta::parse(input_str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
}
