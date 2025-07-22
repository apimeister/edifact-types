use edifact_types::d95b::*;

fn main() {
    // Create a simple BAPLIE message
    let baplie = BAPLIE {
        unh: UNH {
            _010: "0001".to_string(),
            _020: Some(S009 {
                _010: "BAPLIE".to_string(),
                _020: "D".to_string(),
                _030: "95B".to_string(),
                _040: "UN".to_string(),
                _050: None,
            }),
            _030: None,
            _040: None,
        },
        bgm: BGM {
            _010: Some(C002 {
                _010: Some("BAPLIE".to_string()),
                _020: Some("D".to_string()),
                _030: Some("95B".to_string()),
                _040: Some("UN".to_string()),
            }),
            _020: Some("BAPLIE001".to_string()),
            _030: None,
            _040: None,
        },
        dtm: vec![DTM {
            _010: C507 {
                _010: "137".to_string(),
                _020: Some("20231201".to_string()),
                _030: Some("102".to_string()),
            },
        }],
        rff: vec![RFF {
            _010: C506 {
                _010: "BM".to_string(),
                _020: Some("REF001".to_string()),
                _030: None,
                _040: None,
            },
        }],
        nad: vec![NAD {
            _010: "CA".to_string(),
            _020: Some(C082 {
                _010: "CARRIER001".to_string(),
                _020: None,
                _030: None,
            }),
            _030: None,
            _040: None,
            _050: None,
            _060: None,
            _070: None,
            _080: None,
            _090: None,
        }],
        segment_group_1: vec![BAPLIESegmentgroup1 {
            tdt: TDT {
                _010: "20".to_string(),
                _020: Some("VOY001".to_string()),
                _030: Some(C220 {
                    _010: Some("1".to_string()),
                    _020: Some("MARITIME".to_string()),
                }),
                _040: Some(C228 {
                    _010: Some("VESSEL".to_string()),
                    _020: Some("CONTAINER VESSEL".to_string()),
                }),
                _050: Some(C040 {
                    _010: Some("CARRIER001".to_string()),
                    _020: None,
                    _030: None,
                    _040: None,
                }),
                _060: None,
                _070: None,
                _080: None,
                _090: None,
            },
            loc: vec![LOC {
                _010: "5".to_string(),
                _020: Some(C517 {
                    _010: Some("PORT001".to_string()),
                    _020: None,
                    _030: None,
                    _040: None,
                }),
                _030: None,
                _040: None,
                _050: None,
            }],
            dtm: vec![DTM {
                _010: C507 {
                    _010: "132".to_string(),
                    _020: Some("20231201".to_string()),
                    _030: Some("102".to_string()),
                },
            }],
            rff: vec![RFF {
                _010: C506 {
                    _010: "VON".to_string(),
                    _020: Some("LOAD001".to_string()),
                    _030: None,
                    _040: None,
                },
            }],
            ftx: vec![FTX {
                _010: "AAA".to_string(),
                _020: None,
                _030: None,
                _040: None,
                _050: None,
            }],
        }],
        segment_group_2: vec![BAPLIESegmentgroup2 {
            loc: LOC {
                _010: "20".to_string(),
                _020: Some(C517 {
                    _010: Some("BAY01".to_string()),
                    _020: None,
                    _030: None,
                    _040: None,
                }),
                _030: None,
                _040: None,
                _050: None,
            },
            gid: vec![GID {
                _010: "1".to_string(),
                _020: Some(C213 {
                    _010: Some("10".to_string()),
                    _020: Some("CT".to_string()),
                    _030: None,
                    _040: None,
                    _050: Some("CARTON".to_string()),
                    _060: None,
                }),
                _030: None,
            }],
            gds: vec![GDS {
                _010: "GENERAL CARGO".to_string(),
            }],
            ftx: vec![FTX {
                _010: "AAA".to_string(),
                _020: None,
                _030: None,
                _040: None,
                _050: None,
            }],
            mea: vec![MEA {
                _010: "GID".to_string(),
                _020: Some(C502 {
                    _010: Some("WT".to_string()),
                    _020: None,
                    _030: None,
                    _040: None,
                }),
                _030: Some(C174 {
                    _010: "KGM".to_string(),
                    _020: Some("1000".to_string()),
                    _030: None,
                    _040: None,
                    _050: None,
                }),
                _040: None,
            }],
            dim: vec![],
            tmp: vec![],
            rng: vec![],
            loc2: vec![],
            rff: vec![],
            segment_group_3: vec![BAPLIESegmentgroup3 {
                eqd: EQD {
                    _010: "CN".to_string(),
                    _020: Some(C237 {
                        _010: Some("CONT001".to_string()),
                        _020: None,
                        _030: None,
                        _040: None,
                    }),
                    _030: Some(C224 {
                        _010: Some("45G1".to_string()),
                        _020: None,
                        _030: None,
                        _040: None,
                    }),
                    _040: None,
                    _050: None,
                    _060: None,
                },
                eqa: vec![],
                nad: None,
            }],
            segment_group_4: vec![],
        }],
        unt: UNT {
            _010: "25".to_string(),
            _020: "0001".to_string(),
        },
    };

    // Print the BAPLIE message
    println!("BAPLIE Message:");
    println!("{}", baplie);
    
    // Access some fields
    println!("\nMessage Reference: {}", baplie.unh._010);
    println!("Document Number: {}", baplie.bgm._020.as_ref().unwrap());
    println!("Number of Transport Details: {}", baplie.segment_group_1.len());
    println!("Number of Location Details: {}", baplie.segment_group_2.len());
} 