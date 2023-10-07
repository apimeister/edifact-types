use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::newline,
    combinator::opt,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

pub fn parse_line<'a>(input: &'a str, segment_name: &str) -> IResult<&'a str, Vec<&'a str>> {
    let tag_name = format!("{segment_name}+");
    let (rest, vars) = delimited(tag(tag_name.as_str()), take_until("'"), tag("'"))(input)?;
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
