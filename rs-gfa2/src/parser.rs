use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::map,
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
fn parse_id(input: &str) -> IResult<&str, String> {
    let (i, id) = re_find!(input, r"[!-~]+")?;
    Ok((i, id.to_string()))
}

/// function that parse the id tag (OPTIONAL)
fn parse_opt_id(input: &str) -> IResult<&str, String> {
    let(i, opt_id) = re_find!(input, r"[!-~]+|\*")?;
    Ok((i, opt_id.to_string()))
}

/// function that parse the ref tag
fn parse_ref(input: &str) -> IResult<&str, String> {
    let(i, ref_id) = re_find!(input, r"[!-~]+[+-]")?;
    Ok((i, ref_id.to_string()))
}

/// function that parse the tag element (this field is optional)
// not implemented yet
fn parse_tag(input: &str) -> IResult<&str, String> {
    let (i, seq) = re_find!(input, r"[A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*")?;
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
    // i'm not too sure if this regex will work or not
    let (i, seq) = re_find!(input, r"\*|([0-9]+[MDIP])+|\-?[0-9]+(,\-?[0-9]+)*")?; 
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

// idk if keep this function or not, probably not
fn parse_orient(input: &str) -> IResult<&str, Orientation> {
    let fwd = map(tag("+"), |_| Orientation::Forward);
    let bkw = map(tag("-"), |_| Orientation::Backward);
    alt((fwd, bkw))(input)
}

/// function that parse the header field
fn parse_header(input: &str) -> IResult<&str, Header> {
    let col = tag(":");

    // parse the first field of the header ({VN:Z:2.0})
    let (i, _opt_tag) = terminated(tag("VN"), &col)(input)?;
    let (i, _opt_type) = terminated(tag("Z"), &col)(i)?;
    let (i, version) = re_find!(i, r"2.0")?;

    // parse the second field of the header ({TS:i:<trace spacing>})
    /* 
    let (i, _opt_tag) = terminated(tag("TS"), &col)(input)?;
    let (i, _opt_type) = terminated(tag("i"), &col)(i)?;
    let (i, _trace) = re_find!(i, r"")?;
    */

    Ok((
        i,
        Header {
            version: version.to_string(),
        },
    ))
}

/// function that parse the segment field
fn parse_segment(input: &str) -> IResult<&str, Segment> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_id, &tab)(input)?;
    let (i, len) = terminated(parse_int, &tab)(i)?;
    let (i, seq) = parse_sequence(i)?;

    let result = Segment {
        id: id,
        len: len,
        sequence: seq,
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

    let result = Fragment {
        id: id,
        ext_ref: ref_id,
        sbeg: sbeg,
        send: send,
        fbeg: fbeg,
        fend: fend,
        alignment: alignment,
    };

    Ok((i, result))
}

fn parse_edge(input: &str) -> IResult<&str, Edge> {
    let tab = tag("\t");

    // let (i, _line_type) = terminated(tag("C"), &tab)(input)?;
    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
    
    let (i, sid1) = terminated(parse_ref, &tab)(i)?;
    let (i, sid2) = terminated(parse_ref, &tab)(i)?;

    // probably using a loop is better
    let (i, beg1) = terminated(parse_pos, &tab)(i)?;
    let (i, end1) = terminated(parse_pos, &tab)(i)?;
    let (i, beg2) = terminated(parse_pos, &tab)(i)?;
    let (i, end2) = terminated(parse_pos, &tab)(i)?;

    let (i, alignment) = parse_alignment(i)?;

    let result = Edge {
        id: id,
        sid1: sid1,
        sid2: sid2,
        beg1: beg1,
        end1: end1,
        beg2: beg2,
        end2: end2,
        alignment: alignment,
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
    let (i, var) = parse_int(i)?;

    let result = Gap {
        id: id,
        sid1: sid1,
        sid2: sid2,
        dist: dist,
        var: var,
    };

    Ok((i, result))
}

/// function that parse the group field
fn parse_ogroup(input: &str) -> IResult<&str, Group> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
     // technically the group field has a part with a needed item and then multiple 
    // optional item, I don't think this kind of control can cover all the cases but
    // for now it's ok
    let (i, var_field) = parse_ref(i)?;
    
    let result = Group {
        id: id,
        var_field: var_field,
    };

    Ok((i, result))
}

fn parse_ugroup(input: &str) -> IResult<&str, Group> {
    let tab = tag("\t");

    let (i, id) = terminated(parse_opt_id, &tab)(input)?;
     // technically the group field has a part with a needed item and then multiple 
    // optional item, I don't think this kind of control can cover all the cases but
    // for now it's ok
    let (i, var_field) = parse_id(i)?;
    
    let result = Group {
        id: id,
        var_field: var_field,
    };

    Ok((i, result))
}

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
        _ => Ok((i, Line::Comment)), // ignore unrecognized headers to allow custom record
    }
}

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
    fn can_parse_header() {
        let hdr = "VN:Z:2.0";
        let hdr_ = Header {
            version: "2.0".to_string(),
        };

        match parse_header(hdr) {
            Err(why) => panic!("{:?}", why),
            Ok((res, h)) => assert_eq!(h, hdr_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let seg = "A\t10\tAAAAAAACGT";
        let seg_ = Segment {
            id: "A".to_string(),
            len: "10".to_string(),
            sequence: "AAAAAAACGT".to_string(),
        };
        match parse_segment(seg) {
            Err(why) => panic!("{:?}", why),
            Ok((res, s)) => assert_eq!(s, seg_),
        }
    }

    // the tag element it's ignored but technically it
    // should not being ignored 
    // TODO: FIX DIS
    #[test]
    fn can_parse_tag_segment() {
        let seg = "3\t21\tTGCAACGTATAGACTTGTCAC\tRC:i:4";
        let seg_ = Segment {
            id: "3".to_string(),
            len: "21".to_string(),
            sequence: "TGCAACGTATAGACTTGTCAC".to_string(),
        };
        match parse_segment(seg) {
            Err(why) => panic!("{:?}", why),
            Ok((res, s)) => assert_eq!(s, seg_),
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
        };
        match parse_fragment(fragment) {
            Err(why) => panic!("{:?}", why),
            Ok((res, s)) => assert_eq!(s, fragment_),
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
        };

        match parse_edge(edge) {
            Err(why) => panic!("{:?}", why),
            Ok((res, e)) => assert_eq!(e, edge_),
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
        };

        match parse_gap(gap) {
            Err(why) => panic!("{:?}", why),
            Ok((res, g)) => assert_eq!(g, gap_),
        }
    }

    // the group_test cannot recognize the "vector part" of var field
    // TODO: FIX DIS
    #[test]
    fn can_parse_o_group() {
        let group = "2_to_12\t11+\t11_to_13+\t13+\txx:i:-1";

        let group_ = Group {
            id: "2_to_12".to_string(),
            var_field: "11+\t11_to_13+\t13+\txx:i:-1".to_string(),
        };

        match parse_ogroup(group) {
            Err(why) => panic!("{:?}", why),
            Ok((res, o)) => assert_eq!(o, group_),
        }
    }

    #[test]
    fn can_parse_u_group() {
        let group = "16sub\t2\t3";

        let group_ = Group {
            id: "16sub".to_string(),
            var_field: "2\t3".to_string(),
        };

        match parse_ugroup(group) {
            Err(why) => panic!("{:?}", why),
            Ok((res, u)) => assert_eq!(u, group_),
        }
    }
}