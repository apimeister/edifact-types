use crate::{d00b::*, util::Parser};

#[test]
fn test_bgm() {
    let str = "BGM+23+2BOG129382+9'";
    let (rest, obj) = BGM::parse(str).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
    assert_eq!(obj._030, Some(_1225::_9));
    println!("{obj}");
    assert_eq!(format!("{obj}"), "BGM+23+2BOG129382+9");
}

#[test]
fn test_unh() {
    let str = "UNH+2805567+IFTSTA:D:00B:UN'";
    let (rest, obj) = UNH::parse(str).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
    println!("{obj}");
    assert_eq!(format!("{obj}"), "UNH+2805567+IFTSTA:D:00B:UN");
}
