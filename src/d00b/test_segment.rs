use crate::{d00b::*, util::Parser};

#[test]
fn test_bgm() {
    let str = "BGM+23+2BOG129382+9'\n";
    let (rest, obj) = BGM::parse(str).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
    assert_eq!(obj._030, Some(_1225::_9));
    println!("{}", obj);
    assert_eq!(str, format!("{obj}"));
}

#[test]
fn test_unh() {
    let str = "UNH+2805567+IFTSTA:D:00B:UN'\n";
    let (rest, obj) = UNH::parse(str).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
    println!("{}", obj);
}
