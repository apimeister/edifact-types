use crate::d95b::*;

#[test]
fn test_baplie_struct_creation() {
    let baplie = BAPLIE::default();
    assert_eq!(baplie.dtm.len(), 0);
    assert_eq!(baplie.rff.len(), 0);
    assert_eq!(baplie.nad.len(), 0);
    assert_eq!(baplie.segment_group_1.len(), 0);
    assert_eq!(baplie.segment_group_2.len(), 0);
}

#[test]
fn test_baplie_segment_group_1_creation() {
    let sg1 = BAPLIESegmentgroup1::default();
    assert_eq!(sg1.loc.len(), 0);
    assert_eq!(sg1.dtm.len(), 0);
    assert_eq!(sg1.rff.len(), 0);
    assert_eq!(sg1.ftx.len(), 0);
}

#[test]
fn test_baplie_segment_group_2_creation() {
    let sg2 = BAPLIESegmentgroup2 {
        loc: LOC {
            _010: "TEST".to_string(),
            _020: None,
            _030: None,
            _040: None,
            _050: None,
        },
        gid: vec![],
        gds: vec![],
        ftx: vec![],
        mea: vec![],
        dim: vec![],
        tmp: vec![],
        rng: vec![],
        loc2: vec![],
        rff: vec![],
        segment_group_3: vec![],
        segment_group_4: vec![],
    };
    assert_eq!(sg2.loc._010, "TEST");
    assert_eq!(sg2.gid.len(), 0);
    assert_eq!(sg2.gds.len(), 0);
}

#[test]
fn test_baplie_segment_group_3_creation() {
    let sg3 = BAPLIESegmentgroup3 {
        eqd: EQD {
            _010: "TEST".to_string(),
            _020: None,
            _030: None,
            _040: None,
            _050: None,
            _060: None,
        },
        eqa: vec![],
        nad: None,
    };
    assert_eq!(sg3.eqd._010, "TEST");
    assert_eq!(sg3.eqa.len(), 0);
    assert!(sg3.nad.is_none());
}

#[test]
fn test_baplie_segment_group_4_creation() {
    let sg4 = BAPLIESegmentgroup4 {
        dgs: DGS {
            _010: None,
            _020: None,
            _030: None,
            _040: None,
            _050: None,
            _060: None,
            _070: None,
            _080: None,
            _090: None,
            _100: None,
            _110: None,
            _120: None,
            _130: None,
        },
        ftx: vec![],
    };
    assert!(sg4.dgs._010.is_none());
    assert_eq!(sg4.ftx.len(), 0);
}

#[test]
fn test_gds_struct_creation() {
    let gds = GDS {
        _010: "CARGO".to_string(),
    };
    assert_eq!(gds._010, "CARGO");
}

#[test]
fn test_gid_struct_creation() {
    let gid = GID {
        _010: "1".to_string(),
        _020: None,
        _030: None,
    };
    assert_eq!(gid._010, "1");
    assert!(gid._020.is_none());
    assert!(gid._030.is_none());
}

#[test]
fn test_c213_struct_creation() {
    let c213 = C213 {
        _010: Some("10".to_string()),
        _020: Some("CT".to_string()),
        _030: None,
        _040: None,
        _050: Some("CARTON".to_string()),
        _060: None,
    };
    assert_eq!(c213._010, Some("10".to_string()));
    assert_eq!(c213._020, Some("CT".to_string()));
    assert_eq!(c213._050, Some("CARTON".to_string()));
} 