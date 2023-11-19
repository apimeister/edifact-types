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
        // println!("internal_line: {i}");
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
            new_lines.push(format!("{}: {right_side},", tag.to_lowercase()));
        }
        new_lines.first().cloned()
    }

    fn internal_group(i: &str, re: Regex) -> Option<(String, String)> {
        let mut new_group = vec![];
        // println!("internal_group: {i}");
        for (_, [name, req, repeat]) in re.captures_iter(i).map(|c| c.extract()) {
            let struct_name = format!("{}{}", MSG_TYPE, name.replace(' ', ""));
            let handle = name.replace(' ', "_").to_lowercase();
            let group_open = if repeat == "1" {
                if req == "M" {
                    format!("{handle}: {struct_name},")
                } else {
                    format!("{handle}: Option<{struct_name}>,")
                }
            } else {
                format!("{handle}: Vec<{struct_name}>,")
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
        println!("Input:\n\n");
        let mut final_string: String = format!(
            "#[derive(Default, Debug, Serialize, Deserialize, DisplayEdifact, ParseMsg)]\npub struct {MSG_TYPE} {{"
        );

        let mut lines: Vec<&str> = vec![];
        while !i.is_empty() {
            let Ok((rest, line)) = line_parser(i) else {
                panic!("paniced while reading file")
            };
            lines.push(line);
            i = rest;
        }

        let mut groups: HashMap<String, String> = HashMap::new();
        let mut group_level: Vec<bool> = vec![];
        let mut current_group: Vec<String> = vec![];
        for line in &lines {
            // println!("line: {line}");

            // kick out non-parsable lines
            let parsed_line = internal_line(line, re_line.clone());
            let parsed_group = internal_group(line, re_group.clone());
            if parsed_line.is_none() && parsed_group.is_none() {
                continue;
            }
            // figure out how deeply nested we are
            // println!("group_level: {group_level:?} // current_group: {current_group:?}");
            let outer = if group_level.is_empty() {
                0
            } else {
                group_level.len() - 1
            };
            let (outer_line, _ol_rest) = line.split_at(line.len() - outer);
            let (inner_line, il_rest) = line.split_at(line.len() - group_level.len());
            // println!("line_stripped_outer_{outer}: {outer_line}");
            // println!("line_stripped_outer_{outer}: {ol_rest}");
            // println!("line_stripped_inner_{}: {inner_line}", group_level.len());
            // println!("line_stripped_inner_{}: {il_rest}", group_level.len());

            if let Some(inside_group) = group_level.last() {
                if *inside_group {
                    // we are in a group and are starting another
                    if inner_line.ends_with('+') {
                        // start group recording
                        println!("group _start: {inner_line}");
                        if let Some((group_handle, name)) = parsed_group {
                            let cg = current_group.last().unwrap();
                            if let Some(g) = groups.get_mut(cg) {
                                *g = format!("{g}\n    pub {group_handle}");
                            };
                            groups.insert(name.clone(), format!("#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]\npub struct {name} {{"));
                            current_group.push(name);
                        };
                        group_level.push(true);
                    } else if outer_line.ends_with('+') {
                        if let Some(res) = parsed_line {
                            println!("group ___end: {line}");
                            let cg = current_group.last().unwrap();
                            if let Some(g) = groups.get_mut(cg) {
                                *g = format!("{g}\n    pub {res}");
                            };
                        };
                        // end group recording, can be more than one group
                        let mut loopy = il_rest;
                        // println!("loopy_start: {loopy}");
                        while loopy.starts_with('+') {
                            let cg = current_group.last().unwrap();
                            if let Some(g) = groups.get_mut(cg) {
                                *g = format!("{g}\n}}\n");
                            };
                            group_level.pop();
                            current_group.pop();
                            loopy = loopy.strip_prefix('+').unwrap_or(loopy);

                            // println!("loopy: {loopy}");
                            // println!("loopy group_level: {group_level:?} // current_group: {current_group:?}");
                        }
                    } else {
                        // inside group
                        println!("group middle: {line}");
                        if let Some(res) = parsed_line {
                            if let Some(cg) = current_group.last() {
                                if let Some(g) = groups.get_mut(cg) {
                                    *g = format!("{g}\n    pub {res}");
                                };
                            } else {
                                println!("{line} -> this is inside a group, but not parsed")
                            };
                        };
                    }
                }
            } else {
                // we are starting off with a new group
                if outer_line.ends_with('+') {
                    // start group recording
                    println!("group ___new: {outer_line}");
                    if let Some((group_handle, name)) = parsed_group {
                        final_string = format!("{final_string}\n    pub {group_handle}");
                        groups.insert(name.clone(), format!("#[derive(Debug, Serialize, Deserialize, DisplayEdifactSg, ParseSg)]\npub struct {name} {{"));
                        current_group.push(name);
                    };
                    group_level.push(true);
                    continue;
                }
                // outside group
                if let Some(res) = parsed_line {
                    println!("normal _____: {line}");
                    final_string = format!("{final_string}\n    pub {res}");
                }
            }
        }
        let u = format!(
            "use crate::{VERSION}::*;
use edifact_types_macros::{{DisplayEdifact, DisplayEdifactSg, ParseSg, ParseMsg}};
use serde::{{Deserialize, Serialize}};
use std::fmt;"
        );
        final_string = format!("{u}\n\n{final_string}\n}}\n");
        let mut sorted: Vec<_> = groups.iter().collect();
        sorted.sort_by_key(|a| a.0);
        sorted
            .iter()
            .for_each(|g| final_string = format!("{final_string}\n{}", g.1));
        // fs::write(
        //     format!("src/{VERSION}/message/{}.rs", MSG_TYPE.to_lowercase()),
        //     final_string.clone(),
        // )
        // .unwrap();
        println!("Output:\n\n{final_string}");
    }
}
