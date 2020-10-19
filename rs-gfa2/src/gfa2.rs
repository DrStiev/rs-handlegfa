pub mod name_conversion;
pub mod orientation;
pub mod traits;

pub use self::traits::*;
pub use self::orientation::*;

use crate::tag::*;
use bstr::{BStr, BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

// TODO: add an handlegraph to hold the result of the parsing of a GFA2 file
// TODO: add the Display trait to handle the print when gfa2 = GFA2<BString, ()>

/// Returns an Header line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple header 
/// let header = "VN:Z:2.0";
/// let header_ = Header {
///     version: Some("2.0".into()),
///     tag: (),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header<T: OptFields> {
    pub version: Option<BString>,
    pub tag: T,
}

impl<T: OptFields> Header<T> {
    pub fn new(version: Option<BString>) -> Self {
        Header {
            version: version,
            tag: Default::default(),
        }
    }
}

impl<T: OptFields> Default for Header<T> {
    fn default() -> Self {
        Header {
            version: Some("2.0".into()),
            tag: Default::default(),
        }
    }
}

impl fmt::Display for Header<OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "H\t{}\t{}",
            self.version.as_ref().unwrap(),
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Segment line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// // inizialize a simple segment 
/// let segment = "A\t10\tAAAAAAACGT";
/// let segment_: Segment<BString, _> = Segment {
///     id: "A".into(),
///     len: "10".into(),
///     sequence: "AAAAAAACGT".into(),
///     tag:(),
/// };
/// ```
#[derive(
    Default, 
    Debug, 
    Clone, 
    PartialEq, 
    PartialOrd, 
    Serialize, 
    Deserialize, 
    Hash,
)]
pub struct Segment<N, T: OptFields> {
    pub id: N,
    pub len: BString,
    pub sequence: BString,
    pub tag: T,
}

impl<T: OptFields> Segment<BString, T> {
    pub fn new(id: &[u8], len: &[u8], sequence: &[u8]) -> Self {
        Segment {
            id: BString::from(id),
            len: BString::from(len),
            sequence: BString::from(sequence),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId, T: OptFields> Segment<N, T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> Segment<M, T> {
        Segment {
            id: Default::default(),
            len: self.len.clone(),
            sequence: self.sequence.clone(),
            tag: self.tag.clone(),
        }
    }
}

impl fmt::Display for Segment<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}\t{}",
            self.id,
            self.len,
            self.sequence,
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Fragment line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// // inizialize a simple fragment 
/// let fragment = "15\tr1-\t10\t10\t20\t20\t*";
/// let fragment_: Fragment<BString, _> = Fragment {
///     id: "15".into(),
///     ext_ref: "r1-".into(),
///     sbeg: "10".into(),
///     send: "10".into(),
///     fbeg: "20".into(),
///     fend: "20".into(),
///     alignment: "*".into(),
///     tag:(),
/// };
/// ```
#[derive(
    Default, 
    Debug, 
    Clone, 
    PartialEq, 
    PartialOrd, 
    Serialize, 
    Deserialize, 
    Hash,
)]
pub struct Fragment<N, T: OptFields> {
    pub id: N,
    pub ext_ref: BString, // orientation as final char (+-)
    pub sbeg: BString,
    pub send: BString, // dollar character as optional final char
    pub fbeg: BString,
    pub fend: BString,
    pub alignment: BString, // alignment field can be *, trace or CIGAR 
    pub tag: T,
}

impl<T: OptFields> Fragment<BString, T> {
    pub fn new(
        id: &[u8],
        ext_ref: &[u8],
        sbeg: &[u8],
        send: &[u8],
        fbeg: &[u8],
        fend: &[u8],
        alignment: &[u8],
    ) -> Self {
        Fragment {
            id: id.into(),
            ext_ref: ext_ref.into(),
            sbeg: sbeg.into(),
            send: send.into(),
            fbeg: fbeg.into(),
            fend: fend.into(),
            alignment: alignment.into(),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId, T: OptFields> Fragment<N, T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> Fragment<M, T> {
        Fragment {
            id: Default::default(),
            ext_ref: self.ext_ref.clone(),
            sbeg: self.sbeg.clone(),
            send: self.send.clone(),
            fbeg: self.fbeg.clone(),
            fend: self.fend.clone(),
            alignment: self.alignment.clone(),
            tag: self.tag.clone(),
        }
    }
}

impl fmt::Display for Fragment<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "F\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.ext_ref,
            self.sbeg,
            self.send,
            self.fbeg,
            self.fend,
            self.alignment,
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Edge line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// // inizialize a simple edge 
/// let edge = "*\t2+\t45+\t2531\t2591$\t0\t60\t60M";
/// let edge_: Edge<BString, _> = Edge {
///     id: "*".into(),
///     sid1: "2+".into(),
///     sid2: "45+".into(),
///     beg1: "2531".into(),
///     end1: "2591$".into(),
///     beg2: "0".into(),
///     end2: "60".into(),
///     alignment: "60M".into(),
///     tag:(),
/// };
/// ```
#[derive(
    Default, 
    Debug, 
    Clone, 
    PartialEq, 
    PartialOrd, 
    Serialize, 
    Deserialize, 
    Hash,
)]
pub struct Edge<N, T: OptFields> {
    pub id: N, // optional id, can be either * or id tag
    pub sid1: BString, // orientation as final char (+-)
    pub sid2: BString, // orientation as final char (+-)
    pub beg1: BString,
    pub end1: BString, // dollar character as optional final char
    pub beg2: BString,
    pub end2: BString, // dollar character as optional final char
    pub alignment: BString, // alignment field can be *, trace or CIGAR
    pub tag: T,
}

impl<T: OptFields> Edge<BString, T> {
    pub fn new(
        id: &[u8],
        sid1: &[u8],
        sid2: &[u8],
        beg1: &[u8],
        end1: &[u8],
        beg2: &[u8],
        end2: &[u8],
        alignment: &[u8],
    ) -> Self {
        Edge {
            id: id.into(),
            sid1: sid1.into(),
            sid2: sid2.into(),
            beg1: beg1.into(),
            end1: end1.into(),
            beg2: beg2.into(),
            end2: end2.into(),
            alignment: alignment.into(),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId, T: OptFields> Edge<N, T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> Edge<M, T> {
        Edge {
            id: Default::default(),
            sid1: self.sid1.clone(),
            sid2: self.sid2.clone(),
            beg1: self.beg1.clone(),
            end1: self.end1.clone(),
            beg2: self.beg2.clone(),
            end2: self.end2.clone(),
            alignment: self.alignment.clone(),
            tag: self.tag.clone(),
        }
    }
}

impl fmt::Display for Edge<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "E\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.sid1,
            self.sid2,
            self.beg1,
            self.end1,
            self.beg2,
            self.end2,
            self.alignment,
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Gap line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// // inizialize a simple gap 
/// let gap = "g1\t7+\t22+\t10\t*";
/// let gap_: Gap<BString, _> = Gap {
///     id: "g1".into(),
///     sid1: "7+".into(),
///     sid2: "22+".into(),
///     dist: "10".into(),
///     var: "*".into(),
///     tag:(),
/// };
/// ```
#[derive(
    Default, 
    Debug, 
    Clone, 
    PartialEq, 
    PartialOrd, 
    Serialize, 
    Deserialize, 
    Hash,
)]
pub struct Gap<N, T: OptFields> {
    pub id: N, // optional id, can be either * or id tag
    pub sid1: BString, // orientation as final char (+-)
    pub sid2: BString, // orientation as final char (+-)
    pub dist: BString,
    pub var: BString,
    pub tag: T,
}

impl<T: OptFields> Gap<BString, T> {
    pub fn new(
        id: &[u8],
        sid1: &[u8],
        sid2: &[u8],
        dist: &[u8],
        var: &[u8],
    ) -> Self {
        Gap {
            id: id.into(),
            sid1: sid1.into(),
            sid2: sid2.into(),
            dist: dist.into(),
            var: var.into(),
            tag: Default::default(),
        }
    }
}

impl<N: SegmentId, T: OptFields> Gap<N, T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> Gap<M, T> {
        Gap {
            id: Default::default(),
            sid1: self.sid1.clone(),
            sid2: self.sid2.clone(),
            dist: self.dist.clone(),
            var: self.var.clone(),
            tag: self.tag.clone(),
        }
    }
}

impl fmt::Display for Gap<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "G\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.sid1,
            self.sid2,
            self.dist,
            self.var,
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a O-Group line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// // inizialize a simple o-group 
/// let ogroup = "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-";
/// let ogroup_: GroupO<BString, _> =
/// GroupO::new( 
///     "P1".into(),
///     "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
///     (),
/// );
/// ```
#[derive(
    Default, 
    Debug, 
    Clone, 
    PartialEq, 
    PartialOrd, 
    Serialize, 
    Deserialize, 
    Hash,
)]
pub struct GroupO<N, T: OptFields> {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString, // optional id, can be either * or id tag
    pub var_field: BString,  
    pub tag: T,  
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId, T: OptFields> GroupO<N, T> {
    pub fn new(id: BString, var_field: BString, tag: T) -> Self {
        GroupO {
            id: id,
            var_field: var_field,
            tag: tag,
            _segment_names: std::marker::PhantomData,
        }
    }
}

impl<N: SegmentId, T:OptFields> GroupO<N, T> {
    /// parses (and copies) a segment ID in the group segment list
    fn parse_segment_id(input: &[u8]) -> Option<(N, Orientation)> {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Group O segment did not include orientation"),
        };
        let seg = &input[..last];
        let id = N::parse_opt_id(seg)?;
        Some((id, orient))
    }
}

impl<T: OptFields> GroupO<BString, T> {
    /// Produces an iterator over the segments of the given group,
    /// parsing the orientation and producing a slice to each segment
    /// name
    pub fn iter(&self) -> impl Iterator<Item = (&'_ BStr, Orientation)> {
        self.var_field.split_str(b" ").map(Self::segment_id_ref)
    }

    fn segment_id_ref(input: &[u8]) -> (&'_ BStr, Orientation) {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Group O segment did not include orientation"),
        };
        let seg = &input[..last];
        (seg.as_ref(), orient)
    }
}

impl<T: OptFields> GroupO<usize, T> {
    /// Produces an iterator over the usize segments of the given group
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    } 
}

impl fmt::Display for GroupO<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "O\t{}\t{}\t{}",
            self.id,
            // this inline method is useful but add a whitespace at the end of the var_field 
            // creating so an incorrect string 
            self.var_field.to_string() + " ",
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a U-Group line 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// // inizialize a simple u-group 
/// let ugroup = "SG1\t16 24 SG2 51_24 16_24";
/// let ugroup_: GroupU<BString, _> =
/// GroupU::new( 
///     "SG1".into(),
///     "16 24 SG2 51_24 16_24".into(),
///     (),
/// );
/// ```
#[derive(
    Default, 
    Debug, 
    Clone, 
    PartialEq, 
    PartialOrd, 
    Serialize, 
    Deserialize, 
    Hash,
)]
pub struct GroupU<N, T: OptFields> {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: BString, // optional id, can be either * or id tag
    pub var_field: BString,  
    pub tag: T,  
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId, T: OptFields> GroupU<N, T> {
    pub fn new(id: BString, var_field: BString, tag: T) -> Self {
        GroupU {
            id: id,
            var_field: var_field,
            tag: tag,
            _segment_names: std::marker::PhantomData,
        }
    }
}

impl<N: SegmentId, T:OptFields> GroupU<N, T> {
    /// parses (and copies) a segment ID in the group segment list
    fn parse_segment_id(input: &[u8]) -> Option<N> {
        let last = input.len() - 1;
        let seg = &input[..last];
        let id = N::parse_opt_id(seg)?;
        Some(id)
    }
}

impl<T: OptFields> GroupU<BString, T> {
    /// Produces an iterator over the segments of the given group,
    /// parsing the orientation and producing a slice to each segment
    /// name
    pub fn iter(&self) -> impl Iterator<Item = &'_ BStr> {
        self.var_field.split_str(b" ").map(Self::segment_id_ref)
    }

    fn segment_id_ref(input: &[u8]) -> &'_ BStr {
        let last = input.len() - 1;
        let seg = &input[..last];
        seg.as_ref()
    }
}

impl<T: OptFields> GroupU<usize, T> {
    /// Produces an iterator over the usize segments of the given group
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.var_field
            .split_str(b" ")
            .filter_map(Self::parse_segment_id)
    } 
}

impl fmt::Display for GroupU<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "U\t{}\t{}\t{}",
            self.id,
            // this inline method is useful but add a whitespace at the end of the var_field 
            // creating so an incorrect string 
            self.var_field.to_string() + " ",
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a GFA2 object 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// use bstr::BString;
/// 
/// let gfa2: GFA2<BString, OptionalFields> = GFA2 {
///     headers: vec![
///         Header::new(Some("2.0".into())),
///     ],
///     segments: vec![
///         Segment::new(b"A", b"10", b"AAAAAAACGT"),
///     ],
///     fragments: vec![
///         Fragment::new(b"15", b"r1-", b"10", b"10", b"20", b"20", b"*"),
///     ],
///     edges: vec![
///         Edge::new(b"*", b"2+", b"45+", b"2531", b"2591$", b"0", b"60", b"60M"),
///     ],
///     gaps: vec![
///         Gap::new(b"g1", b"7+", b"22+", b"10", b"*"),
///     ],
///     groups_o: vec![
///         GroupO::new("P1".into(), "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(), vec![]),
///     ],
///     groups_u: vec![
///         GroupU::new("SG1".into(), "16 24 SG2 51_24 16_24".into(), vec![]),
///     ]
/// };
/// // inizialize a simple gfa2 object 
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA2<N, T:OptFields> { // OptFields is used to encode the <tag>* item
    // struct to hold the results of parsing a file; not actually a graph
    pub headers: Vec<Header<T>>,
    pub segments: Vec<Segment<N, T>>,
    pub fragments: Vec<Fragment<N, T>>,
    pub edges: Vec<Edge<N, T>>,
    pub gaps: Vec<Gap<N, T>>,
    pub groups_o: Vec<GroupO<N, T>>,
    pub groups_u: Vec<GroupU<N, T>>,
}

/// Enum containing the different kinds of GFA2 lines.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line<N, T:OptFields> {
    Header(Header<T>),
    Segment(Segment<N, T>),
    Fragment(Fragment<N, T>),
    Edge(Edge<N, T>),
    Gap(Gap<N, T>),
    GroupO(GroupO<N, T>),
    GroupU(GroupU<N, T>),
}

macro_rules! some_line_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<N, T: OptFields> Line<N, T> {
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
some_line_fn!(some_ogroup, GroupO<N, T>, Line::GroupO);
some_line_fn!(some_ugroup, GroupU<N, T>, Line::GroupU);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a, N, T:OptFields> {
    Header(&'a Header<T>),
    Segment(&'a Segment<N, T>),
    Fragment(&'a Fragment<N, T>),
    Edge(&'a Edge<N, T>),
    Gap(&'a Gap<N, T>),
    GroupO(&'a GroupO<N, T>),
    GroupU(&'a GroupU<N, T>),
}

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
some_line_ref_fn!(some_ogroup, GroupO<N, T>, LineRef::GroupO);
some_line_ref_fn!(some_ugroup, GroupU<N, T>, LineRef::GroupU);

/// Insert a GFA line (wrapped in the Line enum) into an existing
/// GFA. Simply pushes it into the corresponding Vec in the GFA,
/// or replaces the header, so there's no deduplication or sorting
/// taking place.
impl<N, T: OptFields> GFA2<N, T> {
    /// Insert a GFA line (wrapped in the Line enum) into an existing
    /// GFA. Simply pushes it into the corresponding Vec in the GFA,
    /// or replaces the header, so there's no deduplication or sorting
    /// taking place.
    pub fn insert_line(&mut self, line: Line<N, T>) {
        use Line::*;
        match line {
            Header(h) => self.headers.push(h),
            Segment(s) => self.segments.push(s),
            Fragment(f) => self.fragments.push(f),
            Edge(e) => self.edges.push(e),
            Gap(g) => self.gaps.push(g),
            GroupO(o) => self.groups_o.push(o),
            GroupU(u) => self.groups_u.push(u),
        }
    }

    /// Consume a GFA2 object to produce an iterator over all the lines
    /// contained within. The iterator first produces all headers then segments,
    /// fragments, edges, gaps, groups, comments and finally custom records
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line<N, T>> {
        use Line::*;
        let heads = self.headers.into_iter().map(Header);
        let segs = self.segments.into_iter().map(Segment);
        let frags = self.fragments.into_iter().map(Fragment);
        let edges = self.edges.into_iter().map(Edge);
        let gaps = self.gaps.into_iter().map(Gap);
        let ogroups = self.groups_o.into_iter().map(GroupO);
        let ugroups = self.groups_u.into_iter().map(GroupU);

        heads
            .chain(segs)
            .chain(frags)
            .chain(edges)
            .chain(gaps)
            .chain(ogroups)
            .chain(ugroups)
    }

    /// Return an iterator over references to the lines in the GFA2
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_, N, T>> {
        use LineRef::*;
        let heads = self.headers.iter().map(Header);
        let segs = self.segments.iter().map(Segment);
        let frags = self.fragments.iter().map(Fragment);
        let edges = self.edges.iter().map(Edge);
        let gaps = self.gaps.iter().map(Gap);
        let ogroups = self.groups_o.iter().map(GroupO);
        let ugroups = self.groups_u.iter().map(GroupU);

        heads
            .chain(segs)
            .chain(frags)
            .chain(edges)
            .chain(gaps)
            .chain(ogroups)
            .chain(ugroups)
    }
}

impl<N: SegmentId, T:OptFields> GFA2<N, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl fmt::Display for GFA2<BString, OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}{}{}{}{}{}{}",
            self.headers.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.fragments.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.edges.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.gaps.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_o.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_u.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}