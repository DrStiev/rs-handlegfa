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

/// function that parses the name field 
fn parse_name(input: &str) -> IResult<&str, String> {
    let (i, name) = re_find!(input, r"^[!-)+-<>-~][!-~]*")?;
    Ok((i, name.to_string()))
}

/// function that parses the first field of the header tag
fn parse_header_tag(input: &str) -> IResult<&str, String> {
    let(i, header) = re_find!(input, r"(VN:Z:1.0)?")?;
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
    let (i, seq) = re_find!(input, r"\*|[A-Za-z=.]+")?;
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
    let (i, overlap) = re_find!(input, r"\*|([0-9]+[MIDNSHPX=])+")?;
    Ok((i, overlap.to_string()))
}

/*
fn parse_optional(input: &str) -> IResult<&str, OptionalField> {
    let col = tag(":");
    let (i, opt_tag) = re_find!(input, r"^[A-Za-Z][A-Za-z0-9]")?;
    let (i, opt_type) = preceded(col, one_of("AifZJHB"))(i)?;

    let (i, opt_val) = match opt_type {
        'A' => ,
        'i' => true,
        'f' => true,
        'Z' => true,
        'J' => true,
        'H' => true,
        'B' => true,
    }

    // let (i, opt_typ) = terminated(one_of("AifZJHB"), col);
    // let (i, opt_tag) = re_find!(input, r"[A-Za-Z][A-Za-z0-9]")?;
    // let (
}
*/

/// function that parses the segment tag
fn parse_segment(input: &str) -> IResult<&str, Segment> {
    let tab = tag("\t");

    let (i, name) = terminated(parse_name, &tab)(input)?;
    let (i, seq) = parse_sequence(i)?;

    // TODO branch on the length of the remaining input to read the rest

    let result = Segment {
        name: name,
        sequence: seq,

        segment_len: None,
        read_count: None,
        fragment_count: None,
        kmer_count: None,
        checksum: None,
        uri: None,
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

    let result = Link {
        from_segment,
        from_orient,
        to_segment,
        to_orient,
        overlap,

        map_quality: None,
        num_mismatches: None,
        read_count: None,
        fragment_count: None,
        kmer_count: None,
        edge_id: None,
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

    let (i, overlap) = terminated(parse_overlap, &tab)(i)?;

    let result = Containment {
        container_name,
        container_orient,
        contained_name,
        contained_orient,
        overlap,
        pos: pos.parse::<usize>().unwrap(),
    
        read_coverage: None,
        num_mismatches: None,
        edge_id: None,
    };

    Ok((i, result))
}

/// function that parses the path tag
fn parse_path(input: &str) -> IResult<&str, Path> {
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

/// function that parses the line of a GFA file
fn parse_line(line: &str) -> IResult<&str, Line> {
    let (i, line_type) = terminated(one_of("HSLCP#"), tab)(line)?;

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
        'L' => {
            let (i, l) = parse_link(i)?;
            Ok((i, Line::Link(l)))
        }
        'C' => {
            let (i, c) = parse_containment(i)?;
            Ok((i, Line::Containment(c)))
        }
        'P' => {
            let (i, p) = parse_path(i)?;
            Ok((i, Line::Path(p)))
        }
        _ => Ok((i, Line::Comment)), // ignore unrecognized headers for now
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
/// use rs_gfa2::parser::*;
/// use std::path::PathBuf;
/// 
/// // initialize the parser object
/// let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\lil.gfa"));
///         match gfa {
///             None => panic!("Error parsing GFA file"),
///             Some(g) => {
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
///```
pub fn parse_gfa(path: &PathBuf) -> Option<GFA> {
    let file = File::open(path).expect(&format!("Error opening file {:?}", path));

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut gfa = GFA::new();

    for line in lines {
        let l = line.expect("Error parsing file");
        let p = parse_line(&l);

        if let Ok((_, Line::Header(h))) = p {
            gfa.headers.push(h);
        } else if let Ok((_, Line::Segment(s))) = p {
            gfa.segments.push(s);
        } else if let Ok((_, Line::Link(l))) = p {
            gfa.links.push(l);
        } else if let Ok((_, Line::Containment(c))) = p {
            gfa.containments.push(c);
        } else if let Ok((_, Line::Path(pt))) = p {
            gfa.paths.push(pt);
        }
    }

    Some(gfa)
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
            Err(err) => {
                panic!("{:?}", err);
            }
            Ok((_res, h)) => assert_eq!(h, hdr_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let seg = "11	ACCTT	";
        let seg_ = Segment {
            name: "11".to_string(),
            sequence: "ACCTT".to_string(),

            segment_len: None,
            read_count: None,
            fragment_count: None,
            kmer_count: None,
            checksum: None,
            uri: None,
        };
        match parse_segment(seg) {
            Err(err) => {
                panic!("{:?}", err);
            }
            Ok((_res, s)) => assert_eq!(s, seg_),
        }
    }

    #[test]
    fn can_parse_link() {
        let link = "11	+	12	-	4M	";
        let link_ = Link {
            from_segment: "11".to_string(),
            from_orient: Orientation::Forward,
            to_segment: "12".to_string(),
            to_orient: Orientation::Backward,
            overlap: "4M".to_string(),

            map_quality: None,
            num_mismatches: None,
            read_count: None,
            fragment_count: None,
            kmer_count: None,
            edge_id: None,
        };
        match parse_link(link) {
            Err(err) => {
                panic!("{:?}", err);
            }
            Ok((_res, l)) => assert_eq!(l, link_),
        }
    }

    #[test]
    fn can_parse_containment() {
        let cont = "1\t-\t2\t+\t110\t100M	";

        let cont_ = Containment {
            container_name: "1".to_string(),
            container_orient: Orientation::Backward,
            contained_name: "2".to_string(),
            contained_orient: Orientation::Forward,
            overlap: "100M".to_string(),

            pos: 110,
            read_coverage: None,
            num_mismatches: None,
            edge_id: None,
        };

        match parse_containment(cont) {
            Err(err) => {
                panic!("{:?}", err);
            }
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
            Err(err) => {
                panic!("{:?}", err);
            }
            Ok((_res, p)) => assert_eq!(p, path_),
        }
    }

    #[test]
    fn can_parse_lines() {
        let input = "H	VN:Z:1.0
S	1	CAAATAAG
S	2	A
S	3	G
S	4	T
S	5	C
L	1	+	2	+	0M
L	1	+	3	+	0M
P	x	1+,3+,5+,6+,8+,9+,11+,12+,14+,15+	8M,1M,1M,3M,1M,19M,1M,4M,1M,11M";

        let lines = input.lines();
        let mut gfa = GFA::new();

        let gfa_correct = GFA {
            headers: vec![
                Header::new("VN:Z:1.0"),
            ],
            segments: vec![
                Segment::new("1", "CAAATAAG"),
                Segment::new("2", "A"),
                Segment::new("3", "G"),
                Segment::new("4", "T"),
                Segment::new("5", "C"),
            ],
            links: vec![
                Link::new("1", Orientation::Forward, "2", Orientation::Forward, "0M"),
                Link::new("1", Orientation::Forward, "3", Orientation::Forward, "0M"),
            ],
            paths: vec![Path::new(
                "x",
                vec![
                    "1+", "3+", "5+", "6+", "8+", "9+", "11+", "12+", "14+", "15+",
                ],
                vec!["8M", "1M", "1M", "3M", "1M", "19M", "1M", "4M", "1M", "11M"],
            )],
            containments: vec![],
        };

        for l in lines {
            let p = parse_line(l);

            if let Ok((_, Line::Header(h))) = p {
                gfa.headers.push(h);
            } else if let Ok((_, Line::Segment(s))) = p {
                gfa.segments.push(s);
            } else if let Ok((_, Line::Link(l))) = p {
                gfa.links.push(l);
            } else if let Ok((_, Line::Path(pt))) = p {
                gfa.paths.push(pt);
            }
        }

        assert_eq!(gfa_correct, gfa);
    }
}
