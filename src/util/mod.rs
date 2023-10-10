use nom::{
    bytes::complete::{escaped, is_not, tag},
    character::complete::{newline, one_of},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

pub fn parse_line<'a>(input: &'a str, segment_name: &str) -> IResult<&'a str, Vec<&'a str>> {
    let tag_name = format!("{segment_name}+");
    let (rest, vars) = delimited(
        tag(tag_name.as_str()),
        escaped(is_not("?'"), '?', one_of(r#":?+'"#)),
        tag("'"),
    )(input)?;
    let (_, vars) = separated_list0(
        tag("+"),
        preceded(opt(tag("+")), escaped(is_not("?+"), '?', one_of(r#":?+'"#))),
    )(vars)?;
    // look for trailing newline
    let (rest, _) = opt(newline)(rest)?;
    Ok((rest, vars))
}

pub trait Parser<I, O, E> {
    fn parse(str: I) -> IResult<I, O>;
}

pub fn unborrow_string(input: &&str) -> String {
    input.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_test() {
        let input_str = r#"UNH+3757?'651?+IFTSTA?:D:0??0B:UN'"#;
        println!("### input ##\n {input_str:?}");
        let (rest, line_vars) = parse_line(input_str, "UNH").unwrap();
        println!("### vars ##\n {line_vars:?}");
        println!("### rest ##\n {rest:?}");
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_line_vars_test() {
        let input_str = r#"3757?'651?+IFTSTA?:D:0??0B:UN+123+hello?+??world+goodbye"#;
        println!("### input ##\n {input_str:?}");
        let result: IResult<&str, Vec<&str>> = separated_list0(
            tag("+"),
            preceded(opt(tag("+")), escaped(is_not("?+"), '?', one_of(r#":?+'"#))),
        )(input_str);
        let (rest, vars) = match result {
            Ok((a, b)) => (a, b),
            Err(e) => {
                println!("{e}");
                ("", vec![""])
            }
        };
        println!("### vars ##\n {vars:?}");
        println!("### rest ##\n {rest:?}");
        assert!(rest.is_empty());
    }
}
