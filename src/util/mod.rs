use nom::{
    bytes::complete::{escaped, is_not, tag, take_while},
    character::complete::{newline, one_of},
    combinator::opt,
    multi::separated_list0,
    sequence::delimited,
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
        take_while(|x: char| {
            x != '+' && (x.is_alphanumeric() || x.is_whitespace() || x.is_ascii_punctuation())
        }),
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
}
