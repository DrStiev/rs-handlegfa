// FIXME SENPAI!

use nom::{
    bytes::complete::*,
    character::complete::*,
    sequence::terminated,
    IResult,
    re_find,
};
use std::{
    fs::File,
    io::prelude::*,
    io::BufReader,
    path::PathBuf,
};

#[path = "error/error.rs"]
pub mod error;
use error::GFAError;

use crate::gfa2::*;

/// function that parses the id tag (added the optional vector part)
fn parse_id(input: &str) -> IResult<&str, String> {
    let (i, id) = re_find!(input, r"^([!-~]+([ ][!-~]+)*)")?;
    Ok((i, id.to_string()))
}

/// function that parses the optional id tag
fn parse_opt_id(input: &str) -> IResult<&str, String> {
    let(i, opt_id) = re_find!(input, r"^([!-~]+|\*)")?;
    Ok((i, opt_id.to_string()))
}

/// function that parses the ref tag (added the optional vector part)
fn parse_ref(input: &str) -> IResult<&str, String> {
    let(i, ref_id) = re_find!(input, r"^([!-~]+[+-]([ ][!-~]+[+-])*)")?;
    Ok((i, ref_id.to_string()))
}

/// function that parses the tag element (this field is optional)
fn parse_tag(input: &str) -> IResult<&str, String> {
    let (i, seq) = re_find!(input, r"^(\t[A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*")?;
    Ok((i, seq.to_string()))
}

/// function that parses the sequence element
fn parse_sequence(input: &str) -> IResult<&str, String> {
    let (i, seq) = re_find!(input, r"^(\*|[!-~]+)")?;
    Ok((i, seq.to_string()))
}

/// funtion that parses the alignment element
fn parse_alignment(input: &str) -> IResult<&str, String> {
    // the alignment is composed of 3 choices: 
    // * "empty"
    // ([0-9]+[MDIP])+ CIGAR alignment
    // \-?[0-9]+(,\-?[0-9]+)* trace alignment
    let (i, seq) = re_find!(input, r"^(\*|([0-9]+[MDIP])+|(\-?[0-9]+(,\-?[0-9]+)*))")?; 
    Ok((i, seq.to_string()))
}

/// function that parses the pos tag
fn parse_pos(input: &str) -> IResult<&str, String> {
    let(i, pos) = re_find!(input, r"^([!-~]+\$?)")?;
    Ok((i, pos.to_string()))
}

/// function that parses the int tag
fn parse_int(input: &str) -> IResult<&str, String> {
    let(i, int) = re_find!(input, r"^(\-?[0-9]+)")?;
    Ok((i, int.to_string()))
}

/// function that parses the var tag (similar to the int tag)
fn parse_var(input: &str) -> IResult<&str, String> {
    let(i, int) = re_find!(input, r"^(\*|\-?[0-9]+)")?;
    Ok((i, int.to_string()))
}

/// function that parses the first (and second) field of the header tag
fn parse_header_tag(input: &str) -> IResult<&str, String> {
    let(i, header) = re_find!(input, r"^((VN:Z:2.0)?(\tTS:i:(\*|[!-~]+))?)")?;
    Ok((i, header.to_string()))
}

/// function that parses the header field
fn parse_header(input: &str) -> IResult<&str, Header> {
    let (i, version) = parse_header_tag(input)?;

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());

    let result = Header {
        version: version,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses the segment field
fn parse_segment(input: &str) -> IResult<&str, Segment> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_id, &tab)(input)?;
    let (i, len) = terminated(parse_int, &tab)(i)?;
    let (i, seq) = parse_sequence(i)?;

   let (i, tag) = parse_tag(i)?;
   let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
   tag_value.retain(|tag| !tag.is_empty());

    let result = Segment {
        id: id,
        len: len,
        sequence: seq,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses the fragment field
fn parse_fragment(input: &str) -> IResult<&str, Fragment> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_id, &tab)(input)?;
    let (i, ref_id) = terminated(parse_ref, &tab)(i)?;

    // probably using a loop is better
    let (i, sbeg) = terminated(parse_pos, &tab)(i)?;
    let (i, send) = terminated(parse_pos, &tab)(i)?;
    let (i, fbeg) = terminated(parse_pos, &tab)(i)?;
    let (i, fend) = terminated(parse_pos, &tab)(i)?;

    let (i, alignment) = parse_alignment(i)?;

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());

    let result = Fragment {
        id: id,
        ext_ref: ref_id,
        sbeg: sbeg,
        send: send,
        fbeg: fbeg,
        fend: fend,
        alignment: alignment,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses the edge field
fn parse_edge(input: &str) -> IResult<&str, Edge> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
    
    let (i, sid1) = terminated(parse_ref, &tab)(i)?;
    let (i, sid2) = terminated(parse_ref, &tab)(i)?;

    // probably using a loop is better
    let (i, beg1) = terminated(parse_pos, &tab)(i)?;
    let (i, end1) = terminated(parse_pos, &tab)(i)?;
    let (i, beg2) = terminated(parse_pos, &tab)(i)?;
    let (i, end2) = terminated(parse_pos, &tab)(i)?;

    let (i, alignment) = parse_alignment(i)?;

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());

    let result = Edge {
        id: id,
        sid1: sid1,
        sid2: sid2,
        beg1: beg1,
        end1: end1,
        beg2: beg2,
        end2: end2,
        alignment: alignment,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses the gap field
fn parse_gap(input: &str) -> IResult<&str, Gap> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;

    let (i, sid1) = terminated(parse_ref, &tab)(i)?;
    let (i, sid2) = terminated(parse_ref, &tab)(i)?;

    let (i, dist) = terminated(parse_int, &tab)(i)?;
    let (i, var) = parse_var(i)?;

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());

    let result = Gap {
        id: id,
        sid1: sid1,
        sid2: sid2,
        dist: dist,
        var: var,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses the group field
fn parse_ogroup(input: &str) -> IResult<&str, GroupO> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
    let (i, var_field) = parse_id(i)?;
    let value_var = var_field.split_terminator(" ").map(String::from).collect();    

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());
    
    let result = GroupO {
        id: id,
        var_field: value_var,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses the group field
fn parse_ugroup(input: &str) -> IResult<&str, GroupU> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
    let (i, var_field) = parse_id(i)?;
    let value_var = var_field.split_terminator(" ").map(String::from).collect();

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());
    
    let result = GroupU {
        id: id,
        var_field: value_var,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses a comment line
fn parse_comment(input: &str) -> IResult<&str, Comment> {
    let(i, comment) = re_find!(input, r"^([ -~]*)")?;

    let result = Comment {
        comment: comment.to_string(),
    };

    Ok((i, result))
}

fn insert_custom_record(input: &str) -> IResult<&str, CustomRecord> {
    let result = CustomRecord {
        record: input.to_string(),
    };

    Ok(("", result))
}

/// function that parses all the lines based on their prefix 
fn parse_line(line: &str) -> IResult<&str, Line> {
    let tab = tag("\t");
    let line_type = line.chars().nth(0).unwrap(); //&line[0..1];
    // let (i, line_type) = terminated(one_of("HSFEGOU#"), &tab)(line)?;

    match line_type {
        'H' => {
            let (i, _h) = terminated(one_of("H"), &tab)(line)?;
            let(i, h) = parse_header(i)?;
            Ok((i, Line::Header(h)))
        }
        'S' => {
            let (i, _s) = terminated(one_of("S"), &tab)(line)?;
            let(i, s) = parse_segment(i)?;
            Ok((i, Line::Segment(s)))
        }
        'F' => {
            let (i, _f) = terminated(one_of("F"), &tab)(line)?;
            let(i, f) = parse_fragment(i)?;
            Ok((i, Line::Fragment(f)))
        }
        'E' => {
            let (i, _e) = terminated(one_of("E"), &tab)(line)?;
            let(i, e) = parse_edge(i)?;
            Ok((i, Line::Edge(e)))
        }
        'G' => {
            let (i, _g) = terminated(one_of("G"), &tab)(line)?;
            let(i, g) = parse_gap(i)?;
            Ok((i, Line::Gap(g)))
        }
        'O' => {
            let (i, _o) = terminated(one_of("O"), &tab)(line)?;
            let(i, o) = parse_ogroup(i)?;
            Ok((i, Line::GroupO(o)))
        }
        'U' => {
            let (i, _u) = terminated(one_of("U"), &tab)(line)?;
            let(i, u) = parse_ugroup(i)?;
            Ok((i, Line::GroupU(u)))
        }
        '#' => {
            let (i, _com) = terminated(one_of("#"), tag(" "))(line)?;
            let(i, com) = parse_comment(i)?;
            Ok((i, Line::Comment(com)))
        }
        // ignore unrecognized prefix to allow custom record
        _ => {
            let(i, rec) = insert_custom_record(line)?;
            Ok((i, Line::CustomRecord(rec)))
        }
    }
}

/// Read a file and tries to parse as a GFA2 file.\
/// Returns an [`Option<GFA>`][option] object
/// 
/// [option]: https://doc.rust-lang.org/std/option/enum.Option.html
/// 
/// [gfa]: https://github.com/GFA-spec/GFA-spec/blob/master/GFA2.md
/// 
/// [pathbuf]: https://doc.rust-lang.org/std/path/struct.PathBuf.html 
/// 
/// # Argument
/// 
///  * `file path` - A [`reference`][pathbuf] to a relative (or absolute) path to a file \
///     
/// # Output
/// 
/// * `GFA2 file` - a [`option<GFA>`][option] object, in which is stored the result if \ 
///     the parsing function has run smoothly
/// 
/// # Examples
/// 
/// ```
/// use gfa2::parser_gfa2::*;
/// use std::path::PathBuf;
/// 
/// // initialize the parser object
/// let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\graph_nicernames.gfa"));
///     match gfa {
///     // check the results of the parsing function
///         Err(why) => println!("{}", why),
///         Ok(g) => {
///         // use the result as you want
///             let num_head = g.headers.len();
///             let num_segs = g.segments.len();
///             let num_fragment = g.fragments.len();
///             let num_edge = g.edges.len();
///             let num_gap = g.gaps.len();
///             let num_group_o = g.groups_o.len();
///             let num_group_u = g.groups_u.len();
///             // control if the result it's correct
///             assert_eq!(num_head, 0);
///             assert_eq!(num_segs, 61);
///             assert_eq!(num_fragment, 11);
///             assert_eq!(num_edge, 84);
///             assert_eq!(num_gap, 2);
///             assert_eq!(num_group_o, 2);
///             assert_eq!(num_group_u, 2);
///     }
/// }
/// ```
pub fn parse_gfa(path: &PathBuf) -> Result<GFA2, GFAError> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut gfa = GFA2::new();

    for line in lines {
        let l = line.expect("Error parsing the file");
        let p = parse_line(&l)?;

        if let (_, Line::Header(h)) = p {
            gfa.headers.push(h);
        } else if let (_, Line::Segment(s)) = p {
            gfa.segments.push(s);
        } else if let (_, Line::Fragment(f)) = p {
            gfa.fragments.push(f);
        } else if let (_, Line::Edge(e)) = p {
            gfa.edges.push(e);
        } else if let (_, Line::Gap(g)) = p {
            gfa.gaps.push(g);
        } else if let (_, Line::GroupO(o)) = p {
            gfa.groups_o.push(o)
        } else if let (_, Line::GroupU(u)) = p {
            gfa.groups_u.push(u)
        } else if let (_, Line::Comment(comment)) = p {
            gfa.comments.push(comment)
        } else if let (_, Line::CustomRecord(custom)) = p {
            gfa.custom_record.push(custom)
        }
    }

    Ok(gfa)
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: ADD NEW TESTS
}