use crate::d95b::*;
use crate::util::Parser;

#[test]
#[should_panic]
fn bgm_parse_1() {
    let str = "ABC+abc123";
    let res = BGM::parse(str);
    assert!(res.is_err());
}

#[test]
#[should_panic]
fn bgm_parse_2() {
    let str = "BGM++++++";
    let (rest, _) = BGM::parse(str).unwrap();
    assert!(rest.is_empty());
}

#[test]
fn bgm_parse_3() {
    let str = "BGM+45+20121121084145+9+AB'";
    let (rest, obj) = BGM::parse(str).unwrap();
    assert!(rest.is_empty());
    let expect_01 = Some(C002 {
        _010: Some("45".to_string()),
        ..Default::default()
    });
    assert_eq!(obj._010, expect_01);
    let expect_02 = Some("20121121084145".to_string());
    assert_eq!(obj._020, expect_02);
    let expect_03 = Some(_1225::_9);
    assert_eq!(obj._030, expect_03);
    let expect_04 = Some(_4343::AB);
    assert_eq!(obj._040, expect_04);
    println!("{obj:?}");
}

#[test]
fn unh_parse_1() {
    let str = "UNH+111505+COPRAR:D:95B:UN";
    let (rest, _) = UNH::parse(str).unwrap();
    assert!(rest.is_empty());
}

#[test]
#[should_panic]
fn rff_parse_1() {
    let str = "UNH+111505+COPRAR:D:95B:UN";
    let res = UNH::parse(str);
    assert!(res.is_err());
}

#[test]
fn rff_parse_2() {
    let str = "RFF+XX:1";
    let (rest, _) = RFF::parse(str).unwrap();
    assert!(rest.is_empty());
}

#[test]
fn tdt_parse_1() {
    let str = "TDT+20+111N+1++ABC:172:20+++9283227:146:11:MONTE SARMIENTO'";
    let (rest, obj) = TDT::parse(str).unwrap();
    assert!(rest.is_empty());
    assert_eq!(obj._010, "20");
    assert_eq!(
        obj._030,
        C220 {
            _010: Some("1".to_string()),
            _020: None
        }
    );
    assert_eq!(obj._080._040, Some("MONTE SARMIENTO".to_string()));
    println!("{obj:?}");
}
