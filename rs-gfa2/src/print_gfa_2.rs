use crate::{gfa2::*, tag::*};

use bstr::BString;
use std::fmt::{Display, Write};

/// This entire module will probably be removed, with the functions
/// replaced by Display implementations on GFA and the GFA line types,
/// but I haven't gotten around to it yet

fn write_optional_fields<U: OptFields, T: Write>(opts: &U, stream: &mut T) {
    for field in opts.fields() {
        write!(stream, "\t{}", field).unwrap_or_else(|err| {
            panic!(
                "Error writing optional field '{}' to stream, {}",
                field, err
            )
        })
    }
}

fn write_header<U: OptFields, T: Write>(header: &Header<U>, stream: &mut T) {
    write!(stream, "H").unwrap();
    if let Some(v) = &header.version {
        write!(stream, "\tVN:Z:{}", v).unwrap();
    }
    write_optional_fields(&header.tag, stream);
}

// Write segment
fn write_segment<N: Display, T: Write, U: OptFields>(
    seg: &Segment<N, U>,
    stream: &mut T,
) {
    write!(stream, "S\t{}\t{}\t{}", seg.id, seg.len, seg.sequence)
        .expect("Error writing segment to stream");

    write_optional_fields(&seg.tag, stream);
}

// Write fragment
fn write_fragment<N: Display, T: Write, U: OptFields>(
    fragment: &Fragment<N, U>,
    stream: &mut T,
) {
    write!(
        stream,
        "F\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        fragment.id,
        fragment.ext_ref,
        fragment.sbeg,
        fragment.send,
        fragment.fbeg,
        fragment.fend,
        fragment.alignment,
    )
    .expect("Error writing fragment to stream");

    write_optional_fields(&fragment.tag, stream);
}

// Write edge
fn write_edge<N: Display, T: Write, U: OptFields>(
    edge: &Edge<N, U>,
    stream: &mut T,
) {
    write!(
        stream,
        "E\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        edge.id,
        edge.sid1,
        edge.sid2,
        edge.beg1,
        edge.end1,
        edge.beg2,
        edge.end2,
        edge.alignment,
    )
    .expect("Error writing edge to stream");

    write_optional_fields(&edge.tag, stream);
}

// Write gap
fn write_gap<N: Display, T: Write, U: OptFields>(
    gap: &Gap<N, U>,
    stream: &mut T,
) {
    write!(
        stream,
        "G\t{}\t{}\t{}\t{}\t{}",
        gap.id,
        gap.sid1,
        gap.sid2,
        gap.dist,
        gap.var,
    )
    .expect("Error writing gap to stream");

    write_optional_fields(&gap.tag, stream);
}

// Write o-group
fn write_ogroup<N: Display, T: Write, U: OptFields>(
    ogroup: &GroupO<N, U>,
    stream: &mut T,
) {
    write!(
        stream,
        "O\t{}\t{}",
        ogroup.id,
        ogroup.var_field,
    )
    .expect("Error writing o-group to stream");

    write_optional_fields(&ogroup.tag, stream);
}

// Write u-group
fn write_ugroup<N: Display, T: Write, U: OptFields>(
    ugroup: &GroupU<N, U>,
    stream: &mut T,
) {
    write!(
        stream,
        "U\t{}\t{}",
        ugroup.id,
        ugroup.var_field,
    )
    .expect("Error writing u-group to stream");

    write_optional_fields(&ugroup.tag, stream);
}

/// Write on a [`mutable`][mut] stream the content of a GFA2 Object
/// 
/// [mut]: https://doc.rust-lang.org/std/keyword.mut.html
/// 
/// # Example
/// ```
/// use print_gfa2::*;
/// use gfa2::*;
/// use parser_gfa2::*;
/// 
/// let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
/// let gfa2: GFA2<BString, OptionalFields> =
///     parser.parse_file(&"test\\gfa2_files\\data.gfa").unwrap();
/// 
/// let mut res = String::new();
/// print_gfa2(&gfa2, &mut res);
/// println!("{}", res);
/// ```
pub fn print_gfa2<N: Display, T: Write, U: OptFields>(
    gfa2: &GFA2<N, U>,
    stream: &mut T,
) {
    gfa2.headers.iter().for_each(|h| {
        write_header(h, stream);
        writeln!(stream).unwrap()
    });
    
    gfa2.segments.iter().for_each(|s| {
        write_segment(s, stream);
        writeln!(stream).unwrap();
    });

    gfa2.fragments.iter().for_each(|f| {
        write_fragment(f, stream);
        writeln!(stream).unwrap();
    });

    gfa2.edges.iter().for_each(|e| {
        write_edge(e, stream);
        writeln!(stream).unwrap();
    });

    gfa2.gaps.iter().for_each(|g| {
        write_gap(g, stream);
        writeln!(stream).unwrap();
    });

    gfa2.groups_o.iter().for_each(|o| {
        write_ogroup(o, stream);
        writeln!(stream).unwrap();
    });

    gfa2.groups_u.iter().for_each(|u| {
        write_ugroup(u, stream);
        writeln!(stream).unwrap();
    });
}

pub fn gfa2_to_string(gfa2: &GFA2<BString, OptionalFields>) -> String {
    let mut result = String::new();
    print_gfa2(gfa2, &mut result);
    result
}