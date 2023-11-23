use std::str::FromStr;

use crate::d00b::*;

#[test]
fn parse_sg1() {
    let input_str = r#"LOC+2+NIMGA:::MANAGUA'DTM+95:20230519:102'DTM+10:20230519:102'"#;
    let (rest, sg1) = IFTMINSegmentgroup1::parse(input_str).unwrap();
    println!("{sg1:?}");
    assert!(rest.is_empty())
}

#[test]
fn unb_test() {
    env_logger::init();
    let input_str =
        r#"UNB+UNOC:3+NO0987654321:30+7080003248381:14+101201:1105+1234567890++CONSIGNOR++++1'"#;
    let (_rest, unb) = UNB::parse(input_str).unwrap();
    println!("{unb:?}");
    // assert_eq!(input_str, format!("{unb}"));
}

#[test]
fn t_0029_test() {
    env_logger::init();
    let input_str = r#"A"#;
    let rest = _0029::from_str(input_str).unwrap();
    println!("{rest:#?}");
}

#[test]
fn test_double_end() {
    env_logger::init();
    let input_str = r#"MEA+WT+G+KGM:1.5''MEA+WT+G+KGM:1.5'"#;
    let (rest, _) = MEA::parse(input_str).unwrap();
    let (rest, _) = MEA::parse(rest).unwrap();
    println!("{rest:#?}");
    assert!(rest.is_empty());
}

#[test]
fn parse_iftmin() {
    // https://developer.bring.com/files/IG_BIG14_1_5_2017-08-09.pdf
    // queried 20.11.23 - 08:00
    env_logger::init();
    let input_str = r#"UNA:+,? '
UNB+UNOC:3+NO0987654321:30+7080003248381:14+101201:1105+1234567890++CONSIGNOR+A+++1'
UNH+1234+IFTMIN:D:00B:UN:BIG14'
BGM+610+70123451234567898+1'
CTA+BK+:EDI dep./ Eva HW'
COM+eva.hw@bring.com:EM'
DTM+137:20100810:102'
DTM+234:20100810:102'
MOA+22:4530:NOK'
FTX+PRD+++1020:X'
FTX+SSR+++1076:ME01NO'
FTX+TRR+++DAVA'
FTX+DEL+++Call 911 before delivery'
FTX+AAR+++0022'
CNT+11:9:PCE'
GDS+11'
TOD+5+CC'
TOD+6++:::EXW:INCOTERMS2010'
LOC+1+3000::139:Drammen+NO'
RFF+CMR:70701020000000034'
RFF+IV:867'
DTM+171:20100923:102'
TDT+20++3++BCS::87'
TSR+1'
LOC+202+1000::87:Bring Logistics, Oslo'
NAD+CZ+10009999999::87++Bring IKT:X+Biskop Gunnerusgate 14 A:18 etg.+OSLO++0001+NO'
LOC+202+20000::87'
CTA+IC+:Kalle'
COM+?+4723249852:TE'
CTA+AA'
COM+email@frigo.com:EM'
RFF+ACD:23456'
NAD+FP+10009999999::87++Bring IKT+Biskop Gunnerusgate 14 A:18 etg.+OSLO++0001+NO'
NAD+FP+10000089989::87'
NAD+FP+0583516:5:87'
LOC+202+20000::87'
CTA+IC'
COM+?+4723249852:TE'
NAD+PW+++Backer?'s Corner+Ravnkroken 12:1 etg.+OSLO++1254+NO'
CTA+IC'
COM+?+4798765432:TE'
NAD+PE+++Pan Audit Co+Hallingkastet 6:hus 3+BERGEN++5000+NO'
RFF+ADE:52020599227'
GID+1+1:201:::Pallet'
TMP+1+-15:CEL'
RNG+12+CEL:4:8'
MOA+114:99:NOK'
PIA+5+1011:MF'
PIA+1+7411220000:HS'
FTX+AAA+4++Dekk på felg'
FTX+INS+4++Y'
MEA+WT+AAW+KGM:40'
MEA+VOL+AAW+DMQ:96'
MEA+WT+G+GRM:525'
MEA+WT+G+KGM:1.5''
MEA+ABT+SQ+PCS:44'
MEA+VOL+AAW+LTR:05'
DIM+2+CMT:60:50:100'
RFF+AAQ:NC006D'
RFF+AQW:H109'
PCI+30+370404600000010750'
RFF+CW:123456'
GIN+ZON+4502633099+4502633021+X+X+X'
DGS+ADR+5.1:(6.1?+8)+1463++2+(E)'
FTX+AAC+++PNG:720'
FTX+AAD+++Dangerous Goods Technical Name N.O.S.'
MEA+WT+AAL+KGM:40'
MEA+WT+AAL+GRM:525'
MEA+VOL++LTR:5.5'
EQD+CN+NC044D+44+1'
EQN+9'
UNT+61+1234'
UNZ+1+1234567890'"#;
    let (rest, obj): (&str, Interchange<IFTMIN>) = Interchange::parse(input_str).unwrap();
    println!("{obj:?}");
    assert!(rest.is_empty());
    // assert_eq!(input_str, format!("{obj}"));
}
