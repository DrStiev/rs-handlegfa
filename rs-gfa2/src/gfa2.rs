pub mod name_conversion;
pub mod traits;

pub use self::traits::*;

use crate::{alignment::CIGAR, tag::*};
use bstr::{BStr, BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

/// implement the Display trait for all the struct in gfa2.rs
use std::fmt;

/// Returns an Header line which is composed of:\
///     * [`version`][string] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `version` - A [`string`][string] slice.
/// * `tag` - A [`vector of string`][vec].
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple header 
/// let simple_header = Header {
///     version: "VN:Z:2.0".to_string(),
///     tag: vec![],
/// };
/// 
/// // inizialize a richer header
/// let richer_header = Header {
///     version: "VN:Z:2.0".to_string(),
///     tag: vec!["RC:i:4".to_string()],
/// };
/// 
/// // inizialize an empty header
/// // this is allowed because all the fields 
/// // of an Header line is either optional or
/// // with zero-or-more counts 
/// let empty_header = Header {
///     version: "".to_string(),
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header<T: OptFields> {
    pub version: Option<BString>,
    pub tag: T,
}

impl<T: OptFields> Default for Header<T> {
    fn default() -> Self {
        Header {
            version: Some("2.0".into()),
            tag: Default::default(),
        }
    }
}

impl<T: OptFields> Header<T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> Header<T> {
        Header {
            version: Default::default(),
            tag: self.tag.clone(),
        }
    }
}

impl fmt::Display for Header<OptionalFields> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "H\t{}\t{}",
            self.version.unwrap(),
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Segment line which is composed of:\
///     * [`id`][string] field,\
///     * [`len`][string] field,\
///     * [`sequence`][string] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `id` - A [`string`][string] slice,\
///     representing the id associated to the current segment.
/// * `len` - A [`string`][string] slice, \
///     representing the length of the following `sequence` string \
///     the 'len' element does not need to be the actual length of the `sequence`.
/// * `sequence` - A [`string`][string] slice, \
///     this `string` is  typically expected to be bases or IUPAC characters, \
///     but there's no restriction other than that the characters must be printable.
/// * `tag` - A [`vector of string`][vec]. 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple segment 
/// let simple_segment = Segment {
///     id: "3".to_string(),
///     len: "21".to_string(),
///     sequence: "TGCAACGTATAGACTTGTCAC".to_string(),
///     tag: vec![],
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
    pub fn new(id: BString, len: BString, sequence: BString) -> Self {
        Segment {
            id: id,
            len: len,
            sequence: sequence,
            tag: Default::default(),
        }
    }
}

impl<N, T: OptFields> Segment<N, T> {
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

/// Returns a Fragment line which is composed of:\
///     * [`id`][string] field,\
///     * [`external reference`][string] field,\
///     * [`segment begin`][string] field,\
///     * [`segment end`][string] field,\
///     * [`fragment begin`][string] field,\
///     * [`fragment end`][string] field,\
///     * [`alignment`][string] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// [trace]: https://dazzlerblog.wordpress.com/2015/11/05/trace-points/
/// 
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// 
/// # Arguments
/// 
/// * `id` - A [`string`][string] slice,\
///     representing the id associated to the current fragment.
/// * `external reference` - A [`string`][string] slice, \
///     representing an 'id' that references a sequence in an external collection.
/// * `segment begin` and `segment end` - A pair of [`string`][string] slices, \
///     representing the interval of the vertex segment that the `external reference` string \
///     contributes to.
/// * `fragment begin` and `fragment end` - A pair of [`string`][string] slices, \
///     representing the interval of the fragment that contributes to the segment.
/// * `alignment` - A [`string`][string] slice, \
///     representing the conclusion of the edge line and can be either a [`trace`][trace] or \
///     [`CIGAR`][cigar] string detailing the alignment, or * if absent.
/// * `tag` - A [`vector of string`][vec]. 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple fragment 
/// let simple_fragment = Fragment {
///     id: "12".to_string(),
///     ext_ref: "1-".to_string(),
///     sbeg: "0".to_string(),
///     send: "140$".to_string(),
///     fbeg: "0".to_string(),
///     fend: "140".to_string(),
///     alignment: "11M".to_string(),
///     tag: vec![],
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
    pub alignment: Option<CIGAR>, // alignment field can be *, trace or CIGAR 
    pub tag: T,
}

impl<T: OptFields> Fragment<BString, T> {
    pub fn new(
        id:BString,
        ext_ref: BString,
        sbeg: BString,
        send: BString,
        fbeg: BString,
        fend: BString,
        alignment: Option<CIGAR>,
    ) -> Self {
        Fragment {
            id: id,
            ext_ref: ext_ref,
            sbeg: sbeg,
            send: send,
            fbeg: fbeg,
            fend: fend,
            alignment: alignment,
            tag: Default::default(),
        }
    }
}

impl<N, T: OptFields> Fragment<N, T> {
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
            self.alignment.unwrap(),
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Edge line which is composed of:\
///     * [`id`][string] field,\
///     * [`segment 1 id`][string] field,\
///     * [`segment 2 id`][string] field,\
///     * [`begin segment 1`][string] field,\
///     * [`end segment 1`][string] field,\
///     * [`begin segment 2`][string] field,\
///     * [`end segment 2`][string] field,\
///     * [`alignment`][string] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// [trace]: https://dazzlerblog.wordpress.com/2015/11/05/trace-points/
/// 
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// 
/// # Arguments
/// 
/// * `id` - A [`string`][string] slice,\
///     representing the id associated to the current edge.
/// * `segment 1 id` and `segment 2 id` - A pair of [`string`][string] slice, \
///     representing an `id` that references to a pair of segments \
///     in the local collection.
/// * `begin segment 1` and `end segment 1` - A pair of [`string`][string] slices, \
///     representing the interval of the first segment that align as a pair of positions. \
///     Position is an integer optionally followed by a $-sign.
/// * `begin segment 2` and `end segment 2` - A pair of [`string`][string] slices, \
///     representing the interval of the first segment that align as a pair of positions. \
///     Position is an integer optionally followed by a $-sign.
/// * `alignment` - A [`string`][string] slice, \
///     representing the conclusion of the edge line and can be either a [`trace`][trace] or \
///     [`CIGAR`][cigar] string detailing the alignment, or * if absent.
/// * `tag` - A [`vector of string`][vec]. 
/// 
/// ## Note
/// 
/// The GFA2 concept of edge generalizes the link and containment lines of GFA. 
/// For example a GFA edge which encodes what is called a dovetail overlap (because two ends overlap) 
/// is a GFA2 edge where:
/// 
/// * `begin segment 1` = 0 and `end segment 2` = y$ \
///     or `begin segment 2` = 0 and `end segment 1` = x$ \
///     (if the aligned segments are in the same orientation)
/// *`begin segment 1` = 0 and `begin segment 2` = 0 \
///     or `end segment 1` = x$ and `end segment 2` = y$ \
///     (if the aligned segments are in opposite orientation)
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple edge 
/// let simple_edge = Edge {
///     id: "*".to_string(),
///     sid1: "3+".to_string(),
///     sid2: "65-".to_string(),
///     beg1: "5329".to_string(),
///     end1: "5376$".to_string(),
///     beg2: "20".to_string(),
///     end2: "67$".to_string(),
///     alignment: "47M".to_string(),
///     tag: vec![],
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
    pub alignment: Option<CIGAR>, // alignment field can be *, trace or CIGAR
    pub tag: T,
}

impl<T: OptFields> Edge<BString, T> {
    pub fn new(
        id: BString,
        sid1: BString,
        sid2: BString,
        beg1: BString,
        end1: BString,
        beg2: BString,
        end2: BString,
        alignment: Option<CIGAR>,
    ) -> Self {
        Edge {
            id: id,
            sid1: sid1,
            sid2: sid2,
            beg1: beg1,
            end1: end1,
            beg2: beg2,
            end2: end2,
            alignment: alignment,
            tag: Default::default(),
        }
    }
}

impl<N, T: OptFields> Edge<N, T> {
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
            self.alignment.unwrap(),
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Gap line which is composed of:\
///     * [`id`][string] field,\
///     * [`segment 1 id`][string] field,\
///     * [`segment 2 id`][string] field,\
///     * [`distance`][string] field,\
///     * [`variance`][string] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `id` - A [`string`][string] slice,\
///     representing the id associated to the current edge.
/// * `segment 1 id` and `segment 2 id` - A pair of [`string`][string] slices, \
///     representing an `id` that references to a pair of segments \
///     in the local collection.
/// * `distance` - A [`string`][string] slice, \
///     representing the estimated gap distance between `segment 1` and `segment 2`. \
/// * `variance` - A [`string`][string] slice, \
///     representing the variance of the estimation about the gap distance.
/// * `tag` - A [`vector of string`][vec]. 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple gap 
/// let simple_gap = Gap {
///     id: "2_to_12".to_string(),
///     sid1: "2-".to_string(),
///     sid2: "12+".to_string(),
///     dist: "500".to_string(),
///     var: "50".to_string(),
///     tag: vec![],
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
        id: BString,
        sid1: BString,
        sid2: BString,
        dist: BString,
        var: BString,
    ) -> Self {
        Gap {
            id: id,
            sid1: sid1,
            sid2: sid2,
            dist: dist,
            var: var,
            tag: Default::default(),
        }
    }
}

impl<N, T: OptFields> Gap<N, T> {
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

/// Returns a O-Group line which is composed of:\
///     * [`id`][string] field,\
///     * [`reference field`][vec] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `id` - A [`string`][string] slice,\
///     representing the id associated to the current edge.
/// * `reference field` - A [`vector of string`][vec] slice, \
///     representing an `id`  followed by a non-empty list of 'id' \
///     referring to `segments`, `edges` or other groups that are \
///     separated by single spaces.
/// * `tag` - A [`vector of string`][vec]. 
/// 
/// ## Note
/// 
/// `O-Groups` encode ordered collections 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple o-group 
/// let simple_o_group = GroupO {
///     id: "2_to_12".to_string(),
///     var_field: vec!["11+".to_string(), "11_to_13+".to_string(), "13+".to_string()],
///     tag: vec!["xx:i:-1".to_string()],
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
pub struct GroupO<N, T: OptFields> {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: N, // optional id, can be either * or id tag
    pub var_field: Vec<BString>, // variable field, O-Group have this as optional tag
                                // instead U-Group have dis as normal tag   
    pub tag: T,  
}

impl<T: OptFields> GroupO<BString, T> {
    pub fn new(id: BString, var_field: Vec<BString>) -> Self {
        GroupO {
            id: id,
            var_field: var_field,
            tag: Default::default(),
        }
    }
}

impl<N, T: OptFields> GroupO<N, T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> GroupO<M, T> {
        GroupO {
            id: Default::default(),
            var_field: self.var_field.clone(),
            tag: self.tag.clone(),
        }
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
            self.var_field.iter().fold(String::new(), |acc, str| acc + &str.to_string() + " "),
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a U-Group line which is composed of:\
///     * [`id`][string] field,\
///     * [`id field`][vec] field,\
///     * and a [`tag`][vec] field
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `id` - A [`string`][string] slice,\
///     representing the id associated to the current edge.
/// * `id field` - A [`vector of string`][vec] slice, \
///     representing an `id`  followed by a non-empty list of 'id' \
///     referring to `segments`, `edges` or other groups that are \
///     separated by single spaces.
/// * `tag` - A [`vector of string`][vec]. 
/// 
/// ## Note
/// 
/// `U-Groups` encode unordered collections 
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple u-group 
/// let simple_u_group = GroupU {
///     id: "16sub".to_string(),
///     var_field: vec!["2".to_string(), "3".to_string()],
///     tag: vec![],
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
pub struct GroupU<N, T: OptFields> {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: N, // optional id, can be either * or id tag
    pub var_field: Vec<BString>, // variable field, O-Group have this as optional tag
                                // instead U-Group have dis as normal tag   
    pub tag: T,  
}

impl<T: OptFields> GroupU<BString, T> {
    pub fn new(id: BString, var_field: Vec<BString>) -> Self {
        GroupU {
            id: id,
            var_field: var_field,
            tag: Default::default(),
        }
    }
}

impl<N, T: OptFields> GroupU<N, T> {
    pub(crate) fn nameless_clone<M: Default>(&self) -> GroupU<M, T> {
        GroupU {
            id: Default::default(),
            var_field: self.var_field.clone(),
            tag: self.tag.clone(),
        }
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
            self.var_field.iter().fold(String::new(), |acc, str| acc + &str.to_string() + " "),
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
            self.tag.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// define a struct to holds the comment lines of a file
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Comment {
    pub comment: String,
}

impl Comment {
    pub fn new(comment: &str) -> Comment {
        Comment {
            comment: comment.to_string(),
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# {}", self.comment)
    }
}

/// define a struct to holds the custom lines of a file
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CustomRecord {
    pub record: String,
}

impl CustomRecord {
    pub fn new(record: &str) -> CustomRecord {
        CustomRecord {
            record: record.to_string(),
        }
    }
}

impl fmt::Display for CustomRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# {}", self.record)
    }
}

/// Returns a GFA2 object which is composed of:\
///     * [`headers`][vec] field,\
///     * [`segments`][vec] field,\
///     * [`fragments`][vec] field,\
///     * [`edges`][vec] field,\
///     * [`gaps`][vec] field,\
///     * [`o groups`][vec] field,\
///     * [`u groups`][vec] field,
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `headers` - A [`vector of Header`][vec].
/// * `segments` - A [`vector of Segment`][vec]. 
/// * `fragments` - A [`vector of Fragment`][vec].
/// * `edges` - A [`vector of Edge`][vec].
/// * `gaps` - A [`vector of Gap`][vec].
/// * `o groups` - A [`vector of OGroup`][vec].
/// * `u groups` - A [`vector of UGroup`][vec].
/// 
/// # Examples
/// 
/// ```
/// use gfa2::gfa2::*;
/// 
/// // inizialize a simple gfa2 object 
/// let mut gfa2 = GFA2::new();    
/// gfa2.headers = vec![
///     Header{
///         version: "VN:Z:2.0".to_string(),
///         tag: vec![],
///     }
/// ];
/// gfa2.segments = vec![
///     Segment {
///         id: "1".to_string(),
///         len: "8".to_string(),
///         sequence: "CGATGCAA".to_string(),
///         tag: vec![],
///     }
/// ];
/// gfa2.fragments = vec![];
/// gfa2.edges = vec![
///     Edge {
///         id: "*".to_string(),
///         sid1: "1+".to_string(),
///         sid2: "2+".to_string(),
///         beg1: "3".to_string(),
///         end1: "8$".to_string(),
///         beg2: "0".to_string(),
///         end2: "5".to_string(),
///         alignment: "0,2,4".to_string(),
///         tag: vec!["TS:i:2".to_string()],
///     }
/// ];
/// gfa2.gaps = vec![];
/// gfa2.groups_o = vec![];
/// gfa2.groups_u = vec![];
/// gfa2.comments = vec![];
/// gfa2.custom_record = vec![];
/// 
/// // inizialize an empty GFA2 object 
/// let empty_gfa2 = GFA2::new();
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA2<N, T:OptFields> { // OptFields is used to encode the <tag>* item
    // struct to hold the results of parsing a file; not actually a graph
    // TODO: implement an handlegraph to hold the result of the parsing of a GFA2 file
    pub headers: Vec<Header<T>>,
    pub segments: Vec<Segment<N, T>>,
    pub fragments: Vec<Fragment<N, T>>,
    pub edges: Vec<Edge<N, T>>,
    pub gaps: Vec<Gap<N, T>>,
    pub groups_o: Vec<GroupO<N, T>>,
    pub groups_u: Vec<GroupU<N, T>>,
    pub comments: Vec<Comment>,
    pub custom_record: Vec<CustomRecord>,
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
    Comment(Comment),
    CustomRecord(CustomRecord),
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
// TODO: maybe I should delete dis lines?
some_line_fn!(some_comment, Comment, Line::Comment);
some_line_fn!(some_custom, CustomRecord, Line::CustomRecord);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a, N, T:OptFields> {
    Header(&'a Header<T>),
    Segment(&'a Segment<N, T>),
    Fragment(&'a Fragment<N, T>),
    Edge(&'a Edge<N, T>),
    Gap(&'a Gap<N, T>),
    GroupO(&'a GroupO<N, T>),
    GroupU(&'a GroupU<N, T>),
    Comment(&'a Comment),
    CustomRecord(&'a CustomRecord),
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
// TODO: maybe I should delete dis lines?
some_line_ref_fn!(some_comment, Comment, LineRef::Comment);
some_line_ref_fn!(some_custom, CustomRecord, LineRef::CustomRecord);

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
            Comment(com) => self.comments.push(com),
            CustomRecord(rec) => self.custom_record.push(rec),
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
        
        let comments = self.comments.into_iter().map(Comment);
        let custom_records = self.custom_record.into_iter().map(CustomRecord);

        heads
            .chain(segs)
            .chain(frags)
            .chain(edges)
            .chain(gaps)
            .chain(ogroups)
            .chain(ugroups)
            .chain(comments)
            .chain(custom_records)
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
        
        let comments = self.comments.iter().map(Comment);
        let custom_records = self.custom_record.iter().map(CustomRecord);

        heads
            .chain(segs)
            .chain(frags)
            .chain(edges)
            .chain(gaps)
            .chain(ogroups)
            .chain(ugroups)
            .chain(comments)
            .chain(custom_records)
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
            "{}{}{}{}{}{}{}{}",
            self.headers.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.fragments.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.edges.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.gaps.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_o.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.groups_u.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.custom_record.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}

#[cfg(test)] 
mod tests {
    use super::*;
    // TODO: ADD NEW TESTS 
    // TODO: FIX THE DOC TESTS!
}