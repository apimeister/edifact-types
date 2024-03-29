use crate::{d95b::*, util::Parser};

#[test]
fn test_bgm() {
    _ = env_logger::builder().is_test(true).try_init();
    let str = r#"NAD+SZ++PO BOX 6 CHIQUEDA:ESTRADA DE CHEQUEDA+RAIMUNDO & MAIA, SA++ALCOBACA+10+2461-601+PT'NAD+CA+HAM:160:20'"#;
    let (rest, _obj) = NAD::parse(str).unwrap();
    let (rest, obj) = NAD::parse(rest).unwrap();
    assert!(rest.is_empty());
    println!("{obj:?}");
    println!("{obj}");
    assert_eq!(format!("{obj}"), r#"NAD+CA+HAM:160:20"#);
}
