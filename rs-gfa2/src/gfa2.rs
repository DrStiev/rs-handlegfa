pub mod name_conversion;
pub mod orientation;
pub mod traits;

pub use self::orientation::*;
pub use self::traits::*;

use crate::{
    cigar::CIGAR,
    optfields::*
};

use bstr::BString;
use serde::{
    Deserialize, 
    Serialize
};

/// This module defines the various GFA2 line types, the GFA2 object,
/// and some utility functions and types.
 
/// Simple representation of a parsed GFA2 file, using a Vec<T> to
/// store each separate GFA line type
/// BACKWARD COMPATIBILITY WITH GFA1 from 
/// [https://github.com/DrStiev/GFA-spec/blob/master/GFA2.md#backward-compatibility-with-gfa-1]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA2<N, T:OptFields> {
    // the Header field it's represented as a vector 'cause even if 
    // the most of the case it's only one string, sometimes it can be  
    // multiple strings
    pub header: Vec<Header<T>>, 
    // new integer length field in S-lines
    pub segments: Vec<Segment<N, T>>,
    // new F-lines for describing multi-alignments
    pub fragments: Vec<Fragment<N, T>>,
    // new E-lines to replace L- and C-lines
    pub edges: Vec<Edge<N, T>>,
    // new G-lines for scaffolds
    pub gaps: Vec<Gap<N, T>>,
    // new U- and O-lines to replace P-lines that encode subgraphs and paths,
    // and can take edge id's, obviating the need for orientation signs and 
    // alignments between segments
    pub groups: Vec<Group<N, T>>,

    // Alignmens can be trace length [https://dazzlerblog.wordpress.com/2015/11/05/trace-points/]
    // as well as CIGAR [https://samtools.github.io/hts-specs/SAMv1.pdf] strings
    // IMPORTANT! right now it's only possible to parse CIGAR alignment
    // trace length alignment will be added asap

    // Positions have been extended to include postfix $ symbol 
    // for positions representing the end of a read

    // Segments, edges and paths all have an orientation that is specified with a postfix + or -
    // symbol in contexts where the orientation is needed
}

/// Enum contains the different kinds of GFA lines
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line<N, T:OptFields> {
    Header(Header<T>),
    Segment(Segment<N, T>),
    Fragment(Fragment<N, T>), 
    Edge(Edge<N, T>),
    Gap(Gap<N, T>),
    Group(Group<N, T>),
}

/// defined the macro_rules! some_line_fn
macro_rules! some_line_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<N, T:OptFields> Line<N, T> {
            pub fn $name(self) -> Option<$tgt> {
                if let $variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

some_line_fn!(some_header, Header<T>, Line::Header);
some_line_fn!(some_segment, Segment<N, T>, Line::Segment);
some_line_fn!(some_fragment, Fragment<N, T>, Line::Fragment);
some_line_fn!(some_edge, Edge<N, T>, Line::Edge);
some_line_fn!(some_gap, Gap<N, T>, Line::Gap);
some_line_fn!(some_group, Group<N, T>, Line::Group);

/// Enum contains the different kinds of GFA lines
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a, N, T:OptFields> {
    Header(&'a Header<T>),
    Segment(&'a Segment<N, T>),
    Fragment(&'a Fragment<N, T>), 
    Edge(&'a Edge<N, T>),
    Gap(&'a Gap<N, T>),
    Group(&'a Group<N, T>),
}

/// defined the macro_rules! some_line_ref_fn
macro_rules! some_line_ref_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<'a, N, T: OptFields> LineRef<'a, N, T> {
            pub fn $name(self) -> Option<&'a $tgt> {
                if let $variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

some_line_ref_fn!(some_header, Header<T>, LineRef::Header);
some_line_ref_fn!(some_segment, Segment<N, T>, LineRef::Segment);
some_line_ref_fn!(some_fragment, Fragment<N, T>, LineRef::Fragment);
some_line_ref_fn!(some_edge, Edge<N, T>, LineRef::Edge);
some_line_ref_fn!(some_gap, Gap<N, T>, LineRef::Gap);
some_line_ref_fn!(some_group, Group<N, T>, LineRef::Group);

impl<N, T: OptFields> GFA2<N, T> {

    /// insert a GFA line (wrapped in the line enum) into an existing GFA.
    /// Symply pushes it into the corresponding Vec in the GFA,
    /// or replaces the header, so there's no deduplication or sorting
    /// taking place
    pub fn insert_line(&mut self, line: Line<N, T>) {
        use Line::*;
        match line {
            // use .push(h) instead of = h 'cause the header field now it's a Vec
            Header(h) => self.header.push(h), 
            Segment(s) => self.segments.push(s),
            Fragment(f) => self.fragments.push(f),
            Edge(e) => self.edges.push(e),
            Gap(g) => self.gaps.push(g),
            Group(ou) => self.groups.push(ou),
        }
    }

    /// Consume a GFA ogject to produce an iterator over all the lines contined within.
    /// The iterator first produces all segments, then fragments, then edgesm then gap
    /// and finally group
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line<N, T>> {
        use Line::*;
        let segs = self.segments.into_iter().map(Segment);
        let fragments = self.fragments.into_iter().map(Fragment);
        let edges = self.edges.into_iter().map(Edge);
        let gaps = self.gaps.into_iter().map(Gap);
        let groups = self.groups.into_iter().map(Group);

        segs.chain(fragments).chain(edges).chain(gaps).chain(groups)
    }

    /// Return an iterator over references to the lines in the GFA2
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_, N, T>> {
        use LineRef::*;
        let segs = self.segments.iter().map(Segment);
        let fragments = self.fragments.iter().map(Fragment);
        let edges = self.edges.iter().map(Edge);
        let gaps = self.gaps.iter().map(Gap);
        let groups = self.groups.iter().map(Group);

        segs.chain(fragments).chain(edges).chain(gaps).chain(groups)
    }
}

impl<N: SegmentId, T: OptFields> GFA2<N, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The header line of a GFA2 graph
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header<T: OptFields> {
    pub version: Option<BString>,
    pub trace_spacing: Option<BString>,
    pub tag: Vec<T>,
}

impl<T: OptFields> Default for Header<T> {
    fn default() -> Self {
        Header {
            version: Some("2.0".into()),
            trace_spacing: Default::default(),
            tag: Default::default(),
        }
    }
}

/// A segment in a GFA2 graph. Generic over the name type, but
/// currently the parser is only defined for N = BString
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Segment<N, T:OptFields> {
    pub id: N,
    pub len: N,
    pub sequence: BString,
    pub tag: Vec<T>,
}

impl<T: OptFields> Segment<BString, T> {
    pub fn new(id: &[u8], len: &[u8], sequence: BString) -> Self {
        Segment {
            id: BString::from(id),
            len: BString::from(len),
            sequence,
            tag: Default::default(),
        }
    }
}

/// A fragment in a GFA2 graph. Generic over the name type, but
/// currently the parser is only defined for N = BString
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Fragment<N, T:OptFields> {
    pub id: N,
    pub external_ref: BString,
    pub s_beg: N,
    pub s_end: BString, 
    pub f_beg: N,
    pub f_end: BString, 
    pub alignment: CIGAR,
    pub tag: Vec<T>,
}

impl<T: OptFields> Fragment<BString, T> {
    pub fn new(
        id: &[u8], 
        external_ref: BString, 
        s_beg: &[u8], 
        s_end: BString, 
        f_beg: &[u8], 
        f_end: BString, 
        alignment: CIGAR) -> Self {
            Fragment {
                id: BString::from(id),
                external_ref,
                s_beg: BString::from(s_beg),
                s_end,
                f_beg: BString::from(f_beg),
                f_end,
                alignment,
                tag: Default::default(),
            }
        }
}

/// An edge in a GFA2 graph. Generic over the name type, but
/// currently the parser is only defined for N = BString
/// The new E-lines replace the L- and C-lines
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Edge<N, T:OptFields> {
    pub id: BString,
    pub s_id1: BString, // orientation (+-) needed
    pub s_id2: BString, // orientation (+-) needed
    pub beg1: N,
    pub end1: BString,
    pub beg2: N,
    pub end2: BString,
    pub alignment: CIGAR,
    pub tag: Vec<T>,
}

impl<T: OptFields> Edge<BString, T> {
    pub fn new(
        id: BString,
        s_id1: BString,
        s_id2: BString,
        beg1: &[u8],
        end1: BString,
        beg2: &[u8],
        end2: BString,
        alignment: CIGAR) -> Self {
            Edge {
                id,
                s_id1,
                s_id2,
                beg1: BString::from(beg1),
                end1,
                beg2: BString::from(beg2),
                end2,
                alignment,
                tag: Default::default(),
            }
        }
}

/// A gap in a GFA2 graph. Generic over the name type, but
/// currently the parser is only defined for N = BString
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Gap<N, T:OptFields> {
    pub id: N,
    pub s_id1: BString, // orientation (+-) needed
    pub s_id2: BString, // orientation (+-) needed
    pub dist: BString,
    pub var: BString, // i'm not so sure about this field
    pub tag: Vec<T>,
}

impl<T: OptFields> Gap<BString, T> {
    pub fn new(
        id: BString, 
        s_id1: BString, 
        s_id2: BString, 
        dist: BString, 
        var: BString) -> Self {
        Gap {
            id: BString::from(id),
            s_id1,
            s_id2,
            dist,
            var,
            tag: Default::default(),
        }
    }
}

/// A group in a GFA2 graph. Generic over the name type, but
/// currently the parser is only defined for N = BString
/// The new U- and O-lines replace the P-lines
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Group<N, T: OptFields> {
    // nested structs are NOT supported by rust so, it's better to define the common
    // parts as part of the Group struct and then define the "specialized" part of the 
    // o- and U-groups as separate structs
    pub id: N,
    pub ref_id: BString, // orientation (+-) needed only in O-Group
    pub ref_id_content: Vec<Option<BString>>, // orientation (+-) needed only in O-Group
    pub tag: Vec<T>,
}

impl<T: OptFields> Group<BString, T> {
    pub fn new(
        id: BString, 
        ref_id: BString, 
        ref_id_content: Vec<Option<BString>>) -> Self {
        Group{
            id: BString::from(id),
            ref_id,
            ref_id_content,
            tag: Default::default(),
        }
    }
}