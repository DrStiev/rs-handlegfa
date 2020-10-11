use nom::{
    bytes::complete::*,
    character::complete::*,
    sequence::terminated,
    IResult,
};
use std::{
    fs::File,
    io::prelude::*,
    io::BufReader,
    path::PathBuf,
};

use crate::gfa2::*;

/// function that parse the id tag (NOT OPTIONAL)
/// add the vector part
fn parse_id(input: &str) -> IResult<&str, String> {
    let (i, id) = re_find!(input, r"[!-~]+([ ][!-~]+)*")?;
    Ok((i, id.to_string()))
}

/// function that parse the id tag (OPTIONAL)
fn parse_opt_id(input: &str) -> IResult<&str, String> {
    let(i, opt_id) = re_find!(input, r"[!-~]+|\*")?;
    Ok((i, opt_id.to_string()))
}

/// function that parse the ref tag
/// /// add the vector part
fn parse_ref(input: &str) -> IResult<&str, String> {
    let(i, ref_id) = re_find!(input, r"[!-~]+[+-]([ ][!-~]+[+-])*")?;
    Ok((i, ref_id.to_string()))
}

/// function that parse the tag element (this field is optional)
// not implemented yet
fn parse_tag(input: &str) -> IResult<&str, String> {
    let (i, seq) = re_find!(input, r"(\t[A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*")?;
    Ok((i, seq.to_string()))
}

//function that parse the sequence element
fn parse_sequence(input: &str) -> IResult<&str, String> {
    let (i, seq) = re_find!(input, r"\*|[!-~]+")?;
    Ok((i, seq.to_string()))
}

fn parse_alignment(input: &str) -> IResult<&str, String> {
    // the alignment is composed of 3 choices: 
    // * "empty"
    // ([0-9]+[MDIP])+ CIGAR alignment
    // \-?[0-9]+(,\-?[0-9]+)* trace alignment
    let (i, seq) = re_find!(input, r"\*|([0-9]+[MDIP])+|(\-?[0-9]+(,\-?[0-9]+)*)")?; 
    Ok((i, seq.to_string()))
}

/// function that parse the pos tag
fn parse_pos(input: &str) -> IResult<&str, String> {
    let(i, pos) = re_find!(input, r"[!-~]+\$?")?;
    Ok((i, pos.to_string()))
}

/// function that parse the int tag
fn parse_int(input: &str) -> IResult<&str, String> {
    let(i, int) = re_find!(input, r"\-?[0-9]+")?;
    Ok((i, int.to_string()))
}

/// function that parse the var tag (similar to the int tag)
fn parse_var(input: &str) -> IResult<&str, String> {
    let(i, int) = re_find!(input, r"\*|\-?[0-9]+")?;
    Ok((i, int.to_string()))
}

/// function that parse the first (and second) field of the header tag
fn parse_header_tag(input: &str) -> IResult<&str, String> {
    let(i, header) = re_find!(input, r"(VN:Z:2.0)?(\tTS:i:(\*|[!-~]+))?")?;
    Ok((i, header.to_string()))
}

/// function that parse the header field
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

/// function that parse the segment field
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

/// function that parse the fragment field
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

/// function that parse the gap field
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

/// function that parse the group field
fn parse_ogroup(input: &str) -> IResult<&str, Group> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
    let (i, var_field) = parse_id(i)?;
    let value_var = var_field.split_terminator(" ").map(String::from).collect();    

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());
    
    let result = Group {
        id: id,
        var_field: value_var,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parse the group field
fn parse_ugroup(input: &str) -> IResult<&str, Group> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
    let (i, var_field) = parse_id(i)?;
    let value_var = var_field.split_terminator(" ").map(String::from).collect();

    let (i, tag) = parse_tag(i)?;
    let mut tag_value: Vec<String> = tag.split_terminator("\t").map(String::from).collect();
    tag_value.retain(|tag| !tag.is_empty());
    
    let result = Group {
        id: id,
        var_field: value_var,
        tag: tag_value,
    };

    Ok((i, result))
}

/// function that parses all the lines based on their prefix 
fn parse_line(line: &str) -> IResult<&str, Line> {
    let (i, line_type) = terminated(one_of("HSFEGOU#"), tab)(line)?;

    match line_type {
        'H' => {
            let (i, h) = parse_header(i)?;
            Ok((i, Line::Header(h)))
        }
        '#' => Ok((i, Line::Comment)),
        'S' => {
            let (i, s) = parse_segment(i)?;
            Ok((i, Line::Segment(s)))
        }
        'F' => {
            let (i, f) = parse_fragment(i)?;
            Ok((i, Line::Fragment(f)))
        }
        'E' => {
            let (i, e) = parse_edge(i)?;
            Ok((i, Line::Edge(e)))
        }
        'G' => {
            let (i, g) = parse_gap(i)?;
            Ok((i, Line::Gap(g)))
        }
        'O' => {
            let (i, o) = parse_ogroup(i)?;
            Ok((i, Line::Group(o)))
        }
        'U' => {
            let (i, u) = parse_ugroup(i)?;
            Ok((i, Line::Group(u)))
        }
        _ => Ok((i, Line::CustomRecord)), // ignore unrecognized headers to allow custom record
    }
}

/// function that parses a GFA2 file
pub fn parse_gfa(path: &PathBuf) -> Option<GFA2> {
    let file = File::open(path).expect(&format!("Error opening file {:?}", path));

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut gfa = GFA2::new();

    for line in lines {
        let l = line.expect("Error parsing file");
        let p = parse_line(&l);

        if let Ok((_, Line::Header(h))) = p {
            gfa.headers.push(h);
        } else if let Ok((_, Line::Segment(s))) = p {
            gfa.segments.push(s);
        } else if let Ok((_, Line::Fragment(f))) = p {
            gfa.fragments.push(f);
        } else if let Ok((_, Line::Edge(e))) = p {
            gfa.edges.push(e);
        } else if let Ok((_, Line::Gap(g))) = p {
            gfa.gaps.push(g);
        } else if let Ok((_, Line::Group(ou))) = p {
            gfa.groups.push(ou)
        }
    }

    Some(gfa)
}

#[cfg(test)]
mod test {
    use crate::parser::*;

    #[test]
    fn can_parse_blank_header() {
        let hdr = "";
        let hdr_ = Header {
            version: "".to_string(),
            tag: vec![],
        };

        match parse_header(hdr) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, h)) => assert_eq!(h, hdr_),
        }
    }

    #[test]
    fn can_parse_header() {
        let hdr = "VN:Z:2.0";
        let hdr_ = Header {
            version: "VN:Z:2.0".to_string(),
            tag: vec![],
        };

        match parse_header(hdr) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, h)) => assert_eq!(h, hdr_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let seg = "3\t21\tTGCAACGTATAGACTTGTCAC\tRC:i:4";
        let seg_ = Segment {
            id: "3".to_string(),
            len: "21".to_string(),
            sequence: "TGCAACGTATAGACTTGTCAC".to_string(),
            tag: vec!["RC:i:4".to_string()],
        };
        match parse_segment(seg) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, s)) => assert_eq!(s, seg_),
        }
    }

    #[test]
    fn can_parse_double_tag_segment() {
        let seg = "61\t61\tGACAAAGTCATCGGGCATTATCTGAACATAAAACACTATCAATAAGTTGGAGTCATTACCT\tLN:i:61\tKC:i:9455";
        let seg_ = Segment {
            id: "61".to_string(),
            len: "61".to_string(),
            sequence: "GACAAAGTCATCGGGCATTATCTGAACATAAAACACTATCAATAAGTTGGAGTCATTACCT".to_string(),
            tag: vec!["LN:i:61".to_string(), "KC:i:9455".to_string()],
        };
        match parse_segment(seg) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, s)) => assert_eq!(s, seg_),
        }
    }
    
    #[test]
    fn can_parse_fragment() {
        let fragment = "12\t1-\t0\t140$\t0\t140\t11M";
        let fragment_ = Fragment {
            id: "12".to_string(),
            ext_ref: "1-".to_string(),
            sbeg: "0".to_string(),
            send: "140$".to_string(),
            fbeg: "0".to_string(),
            fend: "140".to_string(),
            alignment: "11M".to_string(),
            tag: vec![],
        };
        match parse_fragment(fragment) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, f)) => assert_eq!(f, fragment_),
        }
    }

    #[test]
    fn can_parse_edge() {
        let edge = "*\t3+\t65-\t5329\t5376$\t20\t67$\t47M";

        let edge_ = Edge {
            id: "*".to_string(),
            sid1: "3+".to_string(),
            sid2: "65-".to_string(),
            beg1: "5329".to_string(),
            end1: "5376$".to_string(),
            beg2: "20".to_string(),
            end2: "67$".to_string(),
            alignment: "47M".to_string(),
            tag: vec![],
        };

        match parse_edge(edge) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, e)) => assert_eq!(e, edge_),
        }
    }

    #[test]
    fn can_parse_gap() {
        let gap = "2_to_12\t2-\t12+\t500\t50";

        let gap_ = Gap {
            id: "2_to_12".to_string(),
            sid1: "2-".to_string(),
            sid2: "12+".to_string(),
            dist: "500".to_string(),
            var: "50".to_string(),
            tag: vec![],
        };

        match parse_gap(gap) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, g)) => assert_eq!(g, gap_),
        }
    }

    #[test]
    fn can_parse_gap2() {
        let gap = "g1\t7+\t22+\t10\t*";

        let gap_ = Gap {
            id: "g1".to_string(),
            sid1: "7+".to_string(),
            sid2: "22+".to_string(),
            dist: "10".to_string(),
            var: "*".to_string(),
            tag: vec![],
        };

        match parse_gap(gap) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, g)) => assert_eq!(g, gap_),
        }
    }

    #[test]
    fn can_parse_o_group() {
        let group = "2_to_12\t11+ 11_to_13+ 13+\txx:i:-1";

        let group_ = Group {
            id: "2_to_12".to_string(),
            var_field: vec!["11+".to_string(), "11_to_13+".to_string(), "13+".to_string()],
            tag: vec!["xx:i:-1".to_string()],
        };

        match parse_ogroup(group) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, o)) => assert_eq!(o, group_),
        }
    }

    #[test]
    fn can_parse_u_group() {
        let group = "16sub\t2 3";

        let group_ = Group {
            id: "16sub".to_string(),
            var_field: vec!["2".to_string(), "3".to_string()],
            tag: vec![],
        };

        match parse_ugroup(group) {
            Err(why) => panic!("{:?}", why),
            Ok((_res, u)) => assert_eq!(u, group_),
        }
    }
}