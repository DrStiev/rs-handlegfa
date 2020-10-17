use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    combinator::map,
    multi::separated_list,
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

use crate::gfa::*;
use crate::error::GFAError;

/// function that parses the name field 
fn parse_name(input: &str) -> IResult<&str, String> {
    let (i, name) = re_find!(input, r"^([!-)+-<>-~][!-~]*)")?;
    Ok((i, name.to_string()))
}

/// function that parses the first field of the header tag
fn parse_header_tag(input: &str) -> IResult<&str, String> {
    let(i, header) = re_find!(input, r"^(VN:Z:1.0)?")?;
    Ok((i, header.to_string()))
}

/// function that parses the header tag
fn parse_header(input: &str) -> IResult<&str, Header> {
    let (i, version) = parse_header_tag(input)?;

    let result = Header {
        version: version,
    };

    Ok((i, result))
}

/// function that parses the sequence field
fn parse_sequence(input: &str) -> IResult<&str, String> {
    let (i, seq) = re_find!(input, r"^(\*|[A-Za-z=.]+)")?;
    Ok((i, seq.to_string()))
}

/// function that parses the orientation character
fn parse_orient(input: &str) -> IResult<&str, Orientation> {
    let fwd = map(tag("+"), |_| Orientation::Forward);
    let bkw = map(tag("-"), |_| Orientation::Backward);
    alt((fwd, bkw))(input)
}

/// function that parses the overlap field
fn parse_overlap(input: &str) -> IResult<&str, String> {
    let (i, overlap) = re_find!(input, r"^(\*|([0-9]+[MIDNSHPX=])+)")?;
    Ok((i, overlap.to_string()))
}

/// function that parses the optional fields of the segment tag
fn parse_optional_fields_segment(input: &str) -> IResult<&str, String> {
    let (i, opt) = re_find!(input, r"^(\t(((LN|RC|FC|KC):i:[-+]?[0-9]+)|(SH:H:[0-9A-F]+)|(UR:Z:[ -~]+)))*")?;
    Ok((i, opt.to_string()))
}

/// function that parses the optional fields of the link tag
fn parse_optional_fields_link(input: &str) -> IResult<&str, String> {
    let (i, opt) = re_find!(input, r"^(\t(((MQ|NM|RC|FC|KC):i:[-+]?[0-9]+)|(ID:Z:[ -~]+)))*")?;
    Ok((i, opt.to_string()))
}

/// function that parses the optional fields of the containment tag
fn parse_optional_fields_contaiment(input: &str) -> IResult<&str, String> {
    let (i, opt) = re_find!(input, r"^(\t(((NM|RC):i:[-+]?[0-9]+)|(ID:Z:[ -~]+)))*")?;
    Ok((i, opt.to_string()))
}

/// function that parses the segment tag
fn parse_segment(input: &str) -> IResult<&str, Segment> {
    let tab = tag("\t");

    let (i, name) = terminated(parse_name, &tab)(input)?;
    let (i, seq) = parse_sequence(i)?;

    let (i, opt) = parse_optional_fields_segment(i)?;
    let mut opt_value: Vec<String> = opt.split_terminator("\t").map(String::from).collect();
    opt_value.retain(|opt| !opt.is_empty());

    let result = Segment {
        name: name,
        sequence: seq,
        optional_fields: opt_value,
    };

    Ok((i, result))
}

/// function that parses the link tag
fn parse_link(input: &str) -> IResult<&str, Link> {
    let tab = tag("\t");

    let seg = terminated(parse_name, &tab);
    let orient = terminated(parse_orient, &tab);

    let (i, from_segment) = seg(input)?;
    let (i, from_orient) = orient(i)?;
    let (i, to_segment) = seg(i)?;
    let (i, to_orient) = orient(i)?;
    let (i, overlap) = parse_overlap(i)?;

    let (i, opt) = parse_optional_fields_link(i)?;
    let mut opt_value: Vec<String> = opt.split_terminator("\t").map(String::from).collect();
    opt_value.retain(|opt| !opt.is_empty());

    let result = Link {
        from_segment,
        from_orient,
        to_segment,
        to_orient,
        overlap,
        optional_fields: opt_value,
    };

    Ok((i, result))
}

/// function that parses the containment tag
fn parse_containment(input: &str) -> IResult<&str, Containment> {
    let tab = tag("\t");

    let seg = terminated(parse_name, &tab);
    let orient = terminated(parse_orient, &tab);

    let (i, container_name) = seg(input)?;
    let (i, container_orient) = orient(i)?;
    let (i, contained_name) = seg(i)?;
    let (i, contained_orient) = orient(i)?;
    let (i, pos) = terminated(digit0, &tab)(i)?;

    let (i, overlap) = parse_overlap(i)?;

    let (i, opt) = parse_optional_fields_contaiment(i)?;
    let mut opt_value: Vec<String> = opt.split_terminator("\t").map(String::from).collect();
    opt_value.retain(|opt| !opt.is_empty());

    let result = Containment {
        container_name,
        container_orient,
        contained_name,
        contained_orient,
        overlap,
        pos: pos.parse::<usize>().unwrap(),
        optional_fields: opt_value,
    };

    Ok((i, result))
}

/// function that parses the path tag
fn parse_path(input: &str) -> IResult<&str, Path> {
    let tab = tag("\t");

    let (i, path_name) = terminated(parse_name, &tab)(input)?;
    let (i, segs) = terminated(parse_name, &tab)(i)?;
    let segment_names = segs.split_terminator(",").map(String::from).collect();
    let (i, overlaps) = separated_list(tag(","), parse_overlap)(i)?;

    let result = Path {
        path_name,
        segment_names,
        overlaps,
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

/// function that parses the line of a GFA file
fn parse_line(line: &str) -> IResult<&str, Line> {
    let tab = tag("\t");
    let line_type = line.chars().nth(0).unwrap(); //&line[0..1];
    // let (i, line_type) = terminated(one_of("HSLCP#"), &tab)(line)?;

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
        'L' => {
            let (i, _l) = terminated(one_of("L"), &tab)(line)?;
            let(i, l) = parse_link(i)?;
            Ok((i, Line::Link(l)))
        }
        'C' => {
            let (i, _c) = terminated(one_of("C"), &tab)(line)?;
            let(i, c) = parse_containment(i)?;
            Ok((i, Line::Containment(c)))
        }
        'P' => {
            let (i, _p) = terminated(one_of("P"), &tab)(line)?;
            let(i, p) = parse_path(i)?;
            Ok((i, Line::Path(p)))
        }
        '#' => {
            let (i, _com) = terminated(one_of("#"), tag(" "))(line)?;
            let(i, com) = parse_comment(i)?;
            Ok((i, Line::Comment(com)))
        }
        // found error
        _ => panic!("Error parsing the file"), 
    }
}

/// Read a file and tries to parse as a GFA file.\
/// Returns an [`Option<GFA>`][option] object
/// 
/// [option]: https://doc.rust-lang.org/std/option/enum.Option.html
/// 
/// [gfa]: https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md
/// 
/// [pathbuf]: https://doc.rust-lang.org/std/path/struct.PathBuf.html 
/// 
/// # Argument
/// 
///  * `file path` - A [`reference`][pathbuf] to a relative (or absolute) path to a file \
///     
/// # Output
/// 
/// * `GFA file` - a [`option<GFA>`][option] object, in which is stored the result if \ 
///     the parsing function has run smoothly
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::parser_gfa::*;
/// use std::path::PathBuf;
/// 
/// // initialize the parser object
/// let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\lil.gfa"));
///         match gfa {
///             Err(why) => println!("{}", why),
///             Ok(g) => {
///                 let num_head = g.headers.len();
///                 let num_segs = g.segments.len();
///                 let num_links = g.links.len();
///                 let num_paths = g.paths.len();
///                 let num_conts = g.containments.len();
///
///                 assert_eq!(num_head, 1);
///                 assert_eq!(num_segs, 15);
///                 assert_eq!(num_links, 20);
///                 assert_eq!(num_conts, 0);
///                 assert_eq!(num_paths, 3);
///
///                 println!("{}", g);
///             }
///         }
/// ```
pub fn parse_gfa(path: &PathBuf) -> Result<GFA, GFAError> {
    let file = File::open(path).expect(&format!("Error opening file {:?}", path));

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut gfa = GFA::new();

    for line in lines {
        let l = line.expect("Error parsing the file");
        let p = parse_line(&l)?;

        if let (_, Line::Header(h)) = p {
            gfa.headers.push(h);
        } else if let (_, Line::Segment(s)) = p {
            gfa.segments.push(s);
        } else if let (_, Line::Link(l)) = p {
            gfa.links.push(l);
        } else if let (_, Line::Containment(c)) = p {
            gfa.containments.push(c);
        } else if let (_, Line::Path(pt)) = p {
            gfa.paths.push(pt);
        } else if let (_, Line::Comment(comment)) = p {
            gfa.comments.push(comment)
        }
    }

    Ok(gfa)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_header() {
        let hdr = "VN:Z:1.0";
        let hdr_ = Header {
            version: "VN:Z:1.0".to_string(),
        };

        match parse_header(hdr) {
            Err(err) => println!("{:?}", err),
            Ok((_res, h)) => assert_eq!(h, hdr_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let seg = "11\tACCTT";
        let seg_ = Segment {
            name: "11".to_string(),
            sequence: "ACCTT".to_string(),
            optional_fields: vec![],
        };
        match parse_segment(seg) {
            Err(err) => println!("{:?}", err),
            Ok((_res, s)) => assert_eq!(s, seg_),
        }
    }

    #[test]
    fn can_parse_segment_with_optional_fields() {
        let seg = "2643\tGCAAATCGCCAGCGCCAGCAACGGATAGTTAATTTTCATGCCTTATCTCCACC\tLN:i:53\tKC:i:6";
        let seg_ = Segment {
            name: "2643".to_string(),
            sequence: "GCAAATCGCCAGCGCCAGCAACGGATAGTTAATTTTCATGCCTTATCTCCACC".to_string(),
            optional_fields: vec!["LN:i:53".to_string(), "KC:i:6".to_string()],
        };
        match parse_segment(seg) {
            Err(err) => println!("{:?}", err),
            Ok((_res, s)) => assert_eq!(s, seg_),
        }
    }

    #[test]
    fn can_parse_link() {
        let link = "11\t+\t12\t-\t4M";
        let link_ = Link {
            from_segment: "11".to_string(),
            from_orient: Orientation::Forward,
            to_segment: "12".to_string(),
            to_orient: Orientation::Backward,
            overlap: "4M".to_string(),
            optional_fields: vec![],
        };
        match parse_link(link) {
            Err(err) => println!("{:?}", err),
            Ok((_res, l)) => assert_eq!(l, link_),
        }
    }

    #[test]
    fn can_parse_link_with_optional_fields() {
        let link = "1\t+\t2\t+\t12M\tID:Z:1_to_2";
        let link_ = Link {
            from_segment: "1".to_string(),
            from_orient: Orientation::Forward,
            to_segment: "2".to_string(),
            to_orient: Orientation::Forward,
            overlap: "12M".to_string(),
            optional_fields: vec!["ID:Z:1_to_2".to_string()],
        };
        match parse_link(link) {
            Err(err) => println!("{:?}", err),
            Ok((_res, l)) => assert_eq!(l, link_),
        }
    }

    #[test]
    fn can_parse_containment() {
        let cont = "1\t-\t2\t+\t110\t100M";

        let cont_ = Containment {
            container_name: "1".to_string(),
            container_orient: Orientation::Backward,
            contained_name: "2".to_string(),
            contained_orient: Orientation::Forward,
            pos: 110,
            overlap: "100M".to_string(),
            optional_fields: vec![],
        };

        match parse_containment(cont) {
            Err(err) => println!("{:?}", err),
            Ok((_res, c)) => assert_eq!(c, cont_),
        }
    }

    #[test]
    fn can_parse_containment_with_optional_fields() {
        let cont = "1\t+\t5\t+\t12\t120M\tID:Z:1_to_5";

        let cont_ = Containment {
            container_name: "1".to_string(),
            container_orient: Orientation::Forward,
            contained_name: "5".to_string(),
            contained_orient: Orientation::Forward,
            pos: 12,
            overlap: "120M".to_string(),
            optional_fields: vec!["ID:Z:1_to_5".to_string()],
        };

        match parse_containment(cont) {
            Err(err) => println!("{:?}", err),
            Ok((_res, c)) => assert_eq!(c, cont_),
        }
    }

    #[test]
    fn can_parse_path() {
        let path = "14\t11+,12-,13+\t4M,5M";

        let path_ = Path {
            path_name: "14".to_string(),
            segment_names: vec!["11+".to_string(), "12-".to_string(), "13+".to_string()],
            overlaps: vec!["4M".to_string(), "5M".to_string()],
        };

        match parse_path(path) {
            Err(err) => println!("{:?}", err),
            Ok((_res, p)) => assert_eq!(p, path_),
        }
    }
}
