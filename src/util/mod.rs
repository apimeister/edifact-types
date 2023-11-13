use nom::{
    bytes::complete::{escaped, is_not, tag},
    character::complete::{newline, one_of},
    combinator::opt,
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

pub fn clean_num(mut input: &str) -> &str {
    // make sure whitespace is removed
    input = input.trim();
    // make sure leading zeros are removed, up to 1 digit
    while input.starts_with('0') && input.len() > 1 {
        input = input.strip_prefix('0').unwrap();
    }

    input
}

pub fn parse_line<'a>(input: &'a str, segment_name: &str) -> IResult<&'a str, Vec<&'a str>> {
    let tag_name = format!("{segment_name}+");
    let (rest, vars) = delimited(
        tag(tag_name.as_str()),
        escaped(is_not("?'"), '?', one_of(r#":?+'"#)),
        tag("'"),
    )(input)?;
    let (_, vars) = crate::util::parse_plus_section(vars)?;
    // look for trailing newline
    let (rest, _) = opt(newline)(rest)?;
    Ok((rest, vars))
}

pub fn parse_plus_section(input: &str) -> IResult<&str, Vec<&str>> {
    let (rest, vars) = separated_list0(
        tag("+"),
        preceded(opt(tag("+")), escaped(is_not("?+"), '?', one_of(r#":?+'"#))),
    )(input)?;
    Ok((rest, vars))
}

pub fn parse_colon_section(input: &str) -> IResult<&str, Vec<&str>> {
    let (rest, vars) = separated_list0(
        tag(":"),
        preceded(opt(tag(":")), escaped(is_not("?:"), '?', one_of(r#":?+'"#))),
    )(input)?;
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
    use nom::character::complete::not_line_ending;
    use regex::Regex;
    use std::{collections::HashMap, fs};

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
    fn parse_plus_section_test() {
        let input_str = r#"3757?'651?+IFTSTA?:D:0??0B:UN+123+hello?+??world+goodbye"#;
        println!("### input ##\n {input_str:?}");
        let (rest, vars) = parse_plus_section(input_str).unwrap();
        println!("### vars ##\n {vars:?}");
        println!("### rest ##\n {rest:?}");
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_colon_section_test() {
        let input_str = r#"IFTSTA?:D:0??0B:UN"#;
        println!("### input ##\n {input_str:?}");
        let (rest, vars) = parse_colon_section(input_str).unwrap();
        println!("### vars ##\n {vars:?}");
        println!("### rest ##\n {rest:?}");
        assert!(rest.is_empty());
    }

    fn line_parser(i: &str) -> IResult<&str, &str> {
        let (rest, _) = opt(newline)(i)?;
        let (rest, line) = not_line_ending(rest)?;
        Ok((rest, line))
    }

    fn internal_line(i: &str, re: Regex) -> Option<String> {
        let mut new_lines = vec![];
        println!("internal_line: {i}");
        for (_, [tag, _name, req, repeat]) in re.captures_iter(i).map(|c| c.extract()) {
            let right_side = if repeat == "1" {
                if req == "M" {
                    tag.to_string()
                } else {
                    format!("Option<{tag}>")
                }
            } else {
                format!("Vec<{tag}>")
            };
            new_lines.push(format!("{tag}: {right_side}"));
        }
        new_lines.first().cloned()
    }

    fn internal_group(i: &str, re: Regex) -> Option<(String, String)> {
        let mut new_group = vec![];
        println!("internal_group: {i}");
        for (_, [name, req, repeat]) in re.captures_iter(i).map(|c| c.extract()) {
            let struct_name = name.replace(' ', "");
            let handle = name.replace(' ', "_").to_lowercase();
            let group_open = if repeat == "1" {
                if req == "M" {
                    format!("{handle}: {struct_name}")
                } else {
                    format!("{handle}: Option<{struct_name}>")
                }
            } else {
                format!("{handle}: Vec<{struct_name}>")
            };
            new_group.push((group_open, struct_name))
        }
        new_group.first().cloned()
    }

    /////////////////
    /// Change this for your messagetype
    /// ////////////
    const VERSION: &str = "d00b";
    const MSG_TYPE: &str = "COPARN";
    #[test]
    fn parse_edifact_descr_from_file() {
        let contents = fs::read_to_string(format!("edi_desc/{VERSION}/{MSG_TYPE}"))
            .expect("Should have been able to read the file");
        let mut i = contents.as_str();
        let re_line =
            Regex::new(r"^\d{4}.+([A-Z]{3})\s+(\S+ ?\S+ ?\S+?)\s+(M|C)\s+(\d{1,4})").unwrap();
        let re_group = Regex::new(r".*-+ (\S+ ?\S+ ?\S+?)\s+-+ (C|M)\s+(\d{1,4}).*").unwrap();

        let mut final_string: String = format!("#[derive(Debug, Serialize, Deserialize, Default, DisplayEdifact)]\npub struct {MSG_TYPE} {{");

        let mut lines: Vec<&str> = vec![];
        while !i.is_empty() {
            let Ok((rest, line)) = line_parser(i) else {
                panic!("paniced while reading file")
            };
            lines.push(line);
            i = rest;
        }

        let mut groups: HashMap<String, String> = HashMap::new();
        let mut inside_group = false;
        let mut current_group = String::from("");
        for line in &lines {
            if line.strip_suffix('+').is_some() {
                if !inside_group {
                    // start group recording
                    println!("group start: {line}");
                    if let Some((group_handle, name)) = internal_group(line, re_group.clone()) {
                        final_string = format!("{final_string}\n    {group_handle}");
                        groups.insert(name.clone(), format!("pub struct {name} {{"));
                        current_group = name;
                    };
                    inside_group = true;
                } else {
                    // end group recording
                    println!("group end: {line}");
                    if let Some(res) = internal_line(line, re_line.clone()) {
                        if let Some(g) = groups.get_mut(&current_group) {
                            *g = format!("{g}\n    {res}\n}}\n");
                        };
                    };
                    inside_group = false;
                }
                continue;
            } else if line.strip_suffix('|').is_some() {
                // inside group
                if let Some(res) = internal_line(line, re_line.clone()) {
                    if let Some(g) = groups.get_mut(&current_group) {
                        *g = format!("{g}\n    {res}");
                    };
                };
                continue;
            }
            // outside group
            let res = internal_line(line, re_line.clone());
            println!("normal: {res:?}");
            if res.is_some() {
                final_string = format!("{final_string}\n    {},", res.unwrap());
            }
        }
        final_string = format!("{final_string}\n}}\n");

        println!("Output:\n\n\n{final_string}");
        let mut sorted: Vec<_> = groups.iter().collect();
        sorted.sort_by_key(|a| a.0);
        sorted.iter().for_each(|g| println!("{}", g.1));
    }
}
