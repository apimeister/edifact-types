use super::Coprar;
use crate::d95b::*;

struct X {
    a: String,
    b: Option<String>,
    c: Option<String>,
    d: Vec<String>,
}
#[test]
fn test_str() {
    let x = X {
        a: "a".to_string(),
        b: None,
        c: None,
        d: vec![],
    };
    let mut v = vec![];
    v.push(x.a);
    v.push(x.b.as_ref().map_or("".to_string(), |x| x.to_string()));
    v.push(x.c.as_ref().map_or("".to_string(), |x| x.to_string()));
    if x.d.is_empty() {
        v.push("".to_string());
    } else {
        x.d.iter().for_each(|x| v.push(x.to_string()));
    }
    println!("{v:?}");
}

#[test]
fn render_bgm() {
    let x = Bgm {
        _010: Some(C002 {
            _010: Some("45".to_string()),
            ..Default::default()
        }),
        _020: Some("20121121084145".to_string()),
        _030: Some(_1225::_9),
        _040: Some(_4343::AB),
    };
    let expected = "BGM+45+20121121084145+9+AB";
    let str = format!("{x}");
    println!("{str}");
    assert_eq!(str, expected)
}

#[test]
fn render_eqd() {
    let x = Eqd {
        _010: "CN".to_string(),
        _020: Some(C237 {
            _010: Some("MSTI6415664".to_string()),
            ..Default::default()
        }),
        _030: Some(C224 {
            _010: Some("45G1".to_string()),
            _020: Some("102".to_string()),
            _030: Some("5".to_string()),
            ..Default::default()
        }),
        _040: None,
        _050: Some("6".to_string()),
        _060: Some("5".to_string()),
    };
    let expected = "EQD+CN+MSTI6415664+45G1:102:5++6+5";
    let str = format!("{x}");
    println!("{str}");
    assert_eq!(str, expected)
}
#[test]
fn render_coprar() {
    let obj = Coprar {
        unh: Unh {
            _010: "638".to_string(),
            _020: Some(S009 {
                _010: "COPRAR".to_string(),
                _020: "D".to_string(),
                _030: "95B".to_string(),
                _040: "UN".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        },
        bgm: Bgm {
            _010: Some(C002 {
                _010: Some("45".to_string()),
                ..Default::default()
            }),
            _020: Some("20121121084145".to_string()),
            _030: Some(_1225::_9),
            _040: Some(_4343::AB),
        },
        ftx: vec![Ftx {
            _010: "OSI".to_string(),
            _030: Some(C107 {
                _010: "L".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }],
        rff: vec![Rff {
            _010: C506 {
                _010: "XXX".to_string(),
                _020: Some("1".to_string()),
                ..Default::default()
            },
        }],
        sg1: CoprarSg1 {
            tdt: Tdt {
                _010: "20".to_string(),
                _020: "123W".to_string(),
                _030: C220 {
                    _010: Some("1".to_string()),
                    _020: None,
                },
                _050: C040 {
                    _010: Some("MSK".to_string()),
                    _020: Some("172".to_string()),
                    _030: Some("20".to_string()),
                    ..Default::default()
                },
                _080: C222 {
                    _010: Some("D5EP4".to_string()),
                    _020: Some("103".to_string()),
                    _040: Some("HELLO WORLD".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            },
            loc: vec![Loc {
                _010: "9".to_string(),
                _020: C517 {
                    _010: Some("AEJEA".to_string()),
                    _020: Some("139".to_string()),
                    _030: Some("6".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            dtm: vec![
                Dtm {
                    _010: C507 {
                        _010: "132".to_string(),
                        _020: Some("20121124".to_string()),
                        _030: Some("203".to_string()),
                    },
                },
                Dtm {
                    _010: C507 {
                        _010: "133".to_string(),
                        _020: Some("20121125".to_string()),
                        _030: Some("203".to_string()),
                    },
                },
            ],
            ..Default::default()
        },
        sg2: vec![CoprarSg2 {
            nad: Nad {
                _010: "CA".to_string(),
                _020: Some(C082 {
                    _010: "MSK".to_string(),
                    _020: Some("160".to_string()),
                    _030: Some("20".to_string()),
                }),
                ..Default::default()
            },
            ..Default::default()
        }],
        sg3: vec![
            CoprarSg3 {
                eqd: Eqd {
                    _010: "CN".to_string(),
                    _020: Some(C237 {
                        _010: Some("MSTI6415664".to_string()),
                        ..Default::default()
                    }),
                    _030: Some(C224 {
                        _010: Some("45G1".to_string()),
                        _020: Some("102".to_string()),
                        _030: Some("5".to_string()),
                        ..Default::default()
                    }),
                    _050: Some("6".to_string()),
                    _060: Some("5".to_string()),
                    ..Default::default()
                },
                rff: vec![Rff {
                    _010: C506 {
                        _010: "BN".to_string(),
                        _020: Some("2GO2005174".to_string()),
                        ..Default::default()
                    },
                }],
                loc: vec![
                    Loc {
                        _010: "11".to_string(),
                        _020: C517 {
                            _010: Some("INNSA".to_string()),
                            _020: Some("139".to_string()),
                            _030: Some("6".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Loc {
                        _010: "7".to_string(),
                        _020: C517 {
                            _010: Some("INNSA".to_string()),
                            _020: Some("139".to_string()),
                            _030: Some("6".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                mea: vec![
                    Mea {
                        _010: "AAE".to_string(),
                        _020: Some(C502 {
                            _010: Some("G".to_string()),
                            ..Default::default()
                        }),
                        _030: Some(C174 {
                            _010: "KGM".to_string(),
                            _020: Some("11740".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Mea {
                        _010: "AAE".to_string(),
                        _020: Some(C502 {
                            _010: Some("VGM".to_string()),
                            ..Default::default()
                        }),
                        _030: Some(C174 {
                            _010: "KGM".to_string(),
                            _020: Some("11740".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ftx: vec![Ftx {
                    _010: "AAA".to_string(),
                    _040: Some(C108 {
                        _010: "PLASTICS?".to_string(),
                        _020: Some("ARTICLES".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                sg4: Some(CoprarSg3Sg4 {
                    tdt: Tdt {
                        _010: "10".to_string(),
                        _020: "142E".to_string(),
                        _030: C220 {
                            _010: Some("1".to_string()),
                            ..Default::default()
                        },
                        _040: C228 {
                            _010: Some("13".to_string()),
                            ..Default::default()
                        },
                        _080: C222 {
                            _010: Some("D5MG2".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                nad: Some(Nad {
                    _010: "CF".to_string(),
                    _020: Some(C082 {
                        _010: "MSK".to_string(),
                        _020: Some("160".to_string()),
                        _030: Some("20".to_string()),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            CoprarSg3 {
                eqd: Eqd {
                    _010: "CN".to_string(),
                    _020: Some(C237 {
                        _010: Some("MSTI6415664".to_string()),
                        ..Default::default()
                    }),
                    _030: Some(C224 {
                        _010: Some("45G1".to_string()),
                        _020: Some("102".to_string()),
                        _030: Some("5".to_string()),
                        ..Default::default()
                    }),
                    _050: Some("6".to_string()),
                    _060: Some("5".to_string()),
                    ..Default::default()
                },
                rff: vec![Rff {
                    _010: C506 {
                        _010: "BN".to_string(),
                        _020: Some("2GO2005174".to_string()),
                        ..Default::default()
                    },
                }],
                loc: vec![
                    Loc {
                        _010: "11".to_string(),
                        _020: C517 {
                            _010: Some("INNSA".to_string()),
                            _020: Some("139".to_string()),
                            _030: Some("6".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Loc {
                        _010: "7".to_string(),
                        _020: C517 {
                            _010: Some("INNSA".to_string()),
                            _020: Some("139".to_string()),
                            _030: Some("6".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                mea: vec![
                    Mea {
                        _010: "AAE".to_string(),
                        _020: Some(C502 {
                            _010: Some("G".to_string()),
                            ..Default::default()
                        }),
                        _030: Some(C174 {
                            _010: "KGM".to_string(),
                            _020: Some("11740".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Mea {
                        _010: "AAE".to_string(),
                        _020: Some(C502 {
                            _010: Some("VGM".to_string()),
                            ..Default::default()
                        }),
                        _030: Some(C174 {
                            _010: "KGM".to_string(),
                            _020: Some("11740".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ftx: vec![Ftx {
                    _010: "AAA".to_string(),
                    _040: Some(C108 {
                        _010: "PLASTICS?".to_string(),
                        _020: Some("ARTICLES".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                sg4: Some(CoprarSg3Sg4 {
                    tdt: Tdt {
                        _010: "10".to_string(),
                        _020: "142E".to_string(),
                        _030: C220 {
                            _010: Some("1".to_string()),
                            ..Default::default()
                        },
                        _040: C228 {
                            _010: Some("13".to_string()),
                            ..Default::default()
                        },
                        _080: C222 {
                            _010: Some("D5MG2".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                nad: Some(Nad {
                    _010: "CF".to_string(),
                    _020: Some(C082 {
                        _010: "MSK".to_string(),
                        _020: Some("160".to_string()),
                        _030: Some("20".to_string()),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ],
        cnt: Cnt {
            _010: C270 {
                _010: "16".to_string(),
                _020: "5".to_string(),
                ..Default::default()
            },
        },
        unt: Unt {
            _010: "56".to_string(),
            _020: "638".to_string(),
        },
    };
    let str = format!("{obj}");
    println!("{}", str);
    let edi = std::fs::read_to_string("./test-data/d95b_coprar.edi").unwrap();
    assert_eq!(edi, str.trim());
}
