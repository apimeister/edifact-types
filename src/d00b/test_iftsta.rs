use super::*;

#[test]
fn build_dtm() {
    let dtm = Dtm {
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
    let bgm = Bgm {
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
    assert_eq!(str, "BGM+23+2BOG129382+9");
}

#[test]
#[should_panic]
fn build_iftsta() {
    // TODO complete test case
    let ifsta = Iftsta {
        bgm: Bgm {
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
        dtm: vec![Dtm {
            _010: C507 {
                _010: "137".to_string(),
                _020: Some("202201010021".to_string()),
                _030: Some(crate::d00b::_2379::_203),
            },
        }],
        sg1: vec![IftstaSg1 {
            nad: Nad {
                _010: "CA".to_string(),
                _020: Some(C082{ _010:"ABCD".to_string(), ..Default::default() }),
                _040: Some(C080{ _010:"SENDER-COMP".to_string(), ..Default::default() }),
                _050: Some(C059 {
                    _010: Some("STREET 1".to_string()),
                    ..Default::default()
                }),
                _060: Some("CITY1".to_string()),
                _080: Some("99999".to_string()),
                _090: Some("DE".to_string()),
                ..Default::default()
            },
            sg2: vec![],
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
