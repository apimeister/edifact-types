use crate::d00b::Unh;

use super::*;

#[test]
fn build_dtm() {
    let dtm = Dtm {
        _01: "137".to_string(),
        _02: Some("202201010021".to_string()),
        _03: Some(super::_2379::_203),
    };
    println!("{:?}", dtm);
    let str = serde_edifact::to_string(&dtm).unwrap();
    println!("{}", str);
    assert_eq!(str, "DTM+137:202201010021:203'\n");
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
    println!("{:?}", bgm);
    let str = serde_edifact::to_string(&bgm).unwrap();
    println!("{}", str);
    assert_eq!(str, "BGM+23+2BOG129382+9'\n");
}

#[test]
fn build_iftsta() {
    let ifsta = Iftsta {
        _0010: Unh {},
        _0020: Bgm {
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
        _0030: Dtm {
            _01: "137".to_string(),
            _02: Some("202201010021".to_string()),
            _03: Some(crate::d00b::_2379::_203),
        },
        _0040: Tsr {},
        _0050: IftstaSg1 {},
        _0100: IftstaSg3 {},
        _0130: Loc {},
        _0140: Ftx {},
        _0150: Cnt {},
        _0160: IftstaSg4 {},
        _0620: Unt {},
    };
    println!("{:?}", ifsta);
    let str = serde_edifact::to_string(&ifsta).unwrap();
    println!("{}", str);
    let target_str = r#"UNB+UNOC:2+SENDER:ZZZ+RECEIVER:ZZZ+220101:1021+2803570'
UNH+2805567+IFTSTA:D:00B:UN'
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
UNT+31+2805567'
UNZ+1+2805570'
"#;
    assert_eq!(str, target_str);
}