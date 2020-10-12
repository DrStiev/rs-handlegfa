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
/// use rs_gfa2::gfa2::*;
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
pub struct Header {
    pub version: String,
    pub tag: Vec<String>,
}

impl Header {
    pub fn new(version: &str, tag: Vec<&str>) -> Header {
        Header {
            version: version.to_string(),
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "H\t{}\t{}",
            self.version,
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
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
/// use rs_gfa2::gfa2::*;
/// 
/// // inizialize a simple segment 
/// let simple_segment = Segment {
///     id: "3".to_string(),
///     len: "21".to_string(),
///     sequence: "TGCAACGTATAGACTTGTCAC".to_string(),
///     tag: vec![],
/// };
/// 
/// // inizialize a richer segment
/// let richer_segment = Segment {
///     id: "61".to_string(),
///     len: "61".to_string(),
///     sequence: "GACAAAGTCATCGGGCATTATCTGAACATAAAACACTATCAATAAGTTGGAGTCATTACCT".to_string(),
///     tag: vec!["LN:i:61".to_string(), "KC:i:9455".to_string()],
/// };
/// 
/// // inizialize an empty segment
/// // this is allowed but the segment line will be  
/// // considered not part of the GFA2 format
/// let empty_segment = Segment {
///     id: "".to_string(),
///     len: "".to_string(),
///     sequence: "".to_string(),
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Segment {
    pub id: String,
    pub len: String,
    pub sequence: String,
    pub tag: Vec<String>,
}

impl Segment {
    pub fn new(name: &str, len: &str, sequence: &str, tag: Vec<&str>) -> Segment {
        Segment {
            id: name.to_string(),
            len: len.to_string(),
            sequence: sequence.to_string(),
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}\t{}",
            self.id,
            self.len,
            self.sequence,
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
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
/// use rs_gfa2::gfa2::*;
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
/// 
/// // inizialize a different fragment
/// let different_fragment = Fragment {
///     id: "2".to_string(),
///     ext_ref: "read2+".to_string(),
///     sbeg: "45".to_string(),
///     send: "62".to_string(),
///     fbeg: "0".to_string(),
///     fend: "18".to_string(),
///     alignment: "*".to_string(),
///     tag: vec!["id:Z:read2_in_2".to_string()],
/// };
/// 
/// // inizialize an empty fragment
/// // this is allowed but the fragment line will be  
/// // considered not part of the GFA2 format
/// let empty_fragment = Fragment {
///     id: "".to_string(),
///     ext_ref: "".to_string(),
///     sbeg: "".to_string(),
///     send: "".to_string(),
///     fbeg: "".to_string(),
///     fend: "".to_string(),
///     alignment: "".to_string(),
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Fragment {
    pub id: String,
    pub ext_ref: String, // orientation as final char (+-)
    pub sbeg: String,
    pub send: String, // dollar character as optional final char
    pub fbeg: String,
    pub fend: String,
    pub alignment: String, // alignment field can be *, trace or CIGAR 
    pub tag: Vec<String>,
}

impl Fragment {
    pub fn new(
        id: &str,
        ext_ref: &str,
        sbeg: &str,
        send: &str,
        fbeg: &str,
        fend: &str,
        alignment: &str,
        tag: Vec<&str>,
    ) -> Fragment {
        Fragment {
            id: id.to_string(),
            ext_ref: ext_ref.to_string(),
            sbeg: sbeg.to_string(),
            send: send.to_string(),
            fbeg: fbeg.to_string(),
            fend: fend.to_string(),
            alignment: alignment.to_string(),
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Fragment {
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
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
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
/// use rs_gfa2::gfa2::*;
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
/// 
/// // inizialize a different edge
/// let different_edge = Edge {
///     id: "1_to_5".to_string(),
///     sid1: "1+".to_string(),
///     sid2: "5+".to_string(),
///     beg1: "0".to_string(),
///     end1: "122$".to_string(),
///     beg2: "2".to_string(),
///     end2: "124".to_string(),
///     alignment: "*".to_string(),
///     tag: vec!["zz:Z:tag".to_string()],
/// };
/// 
/// // inizialize an empty edge
/// // this is allowed but the edge line will be  
/// // considered not part of the GFA2 format
/// let empty_edge = Edge {
///     id: "".to_string(),
///     sid1: "".to_string(),
///     sid2: "".to_string(),
///     beg1: "".to_string(),
///     end1: "".to_string(),
///     beg2: "".to_string(),
///     end2: "".to_string(),
///     alignment: "".to_string(),
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Edge {
    pub id: String, // optional id, can be either * or id tag
    pub sid1: String, // orientation as final char (+-)
    pub sid2: String, // orientation as final char (+-)
    pub beg1: String,
    pub end1: String, // dollar character as optional final char
    pub beg2: String,
    pub end2: String, // dollar character as optional final char
    pub alignment: String, // alignment field can be *, trace or CIGAR
    pub tag: Vec<String>,
}

impl Edge {
    pub fn new(
        id: &str,
        sid1: &str,
        sid2: &str,
        beg1: &str,
        end1: &str,
        beg2: &str,
        end2: &str,
        alignment: &str,
        tag: Vec<&str>,
    ) -> Edge {
        Edge {
            id: id.to_string(),
            sid1: sid1.to_string(),
            sid2: sid2.to_string(),
            beg1: beg1.to_string(),
            end1: end1.to_string(),
            beg2: beg2.to_string(),
            end2: end2.to_string(),
            alignment: alignment.to_string(),
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Edge {
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
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
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
/// use rs_gfa2::gfa2::*;
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
/// 
/// // inizialize a different gap
/// let different_gap = Gap {
///     id: "1_to_11".to_string(),
///     sid1: "1+".to_string(),
///     sid2: "11-".to_string(),
///     dist: "120".to_string(),
///     var: "*".to_string(),
///     tag: vec![],
/// };
/// 
/// // inizialize an empty gap
/// // this is allowed but the gap line will be  
/// // considered not part of the GFA2 format
/// let empty_gap = Gap {
///     id: "".to_string(),
///     sid1: "".to_string(),
///     sid2: "".to_string(),
///     dist: "".to_string(),
///     var: "".to_string(),
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Gap {
    pub id: String, // optional id, can be either * or id tag
    pub sid1: String, // orientation as final char (+-)
    pub sid2: String, // orientation as final char (+-)
    pub dist: String,
    pub var: String,
    pub tag: Vec<String>,
}

impl Gap {
    pub fn new(
        id: &str,
        sid1: &str,
        sid2: &str,
        dist: &str,
        var: &str,
        tag: Vec<&str>,
    ) -> Gap {
        Gap {
            id: id.to_string(),
            sid1: sid1.to_string(),
            sid2: sid2.to_string(),
            dist: dist.to_string(),
            var: var.to_string(),
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "G\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            self.sid1,
            self.sid2,
            self.dist,
            self.var,
            // this inline method is useful but add a tabspace at the end of the tag 
            // creating so an incorrect string 
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
/// use rs_gfa2::gfa2::*;
/// 
/// // inizialize a simple o-group 
/// let simple_o_group = GroupO {
///     id: "2_to_12".to_string(),
///     var_field: vec!["11+".to_string(), "11_to_13+".to_string(), "13+".to_string()],
///     tag: vec!["xx:i:-1".to_string()],
/// };
/// 
/// // inizialize an empty o-group 
/// // this is allowed but the o-group line will be  
/// // considered not part of the GFA2 format
/// let empty_o_group = GroupO {
///     id: "".to_string(),
///     var_field: vec![],
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GroupO {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: String, // optional id, can be either * or id tag
    pub var_field: Vec<String>, // variable field, O-Group have this as optional tag
                                // instead U-Group have dis as normal tag   
    pub tag: Vec<String>,  
}

impl GroupO {
    pub fn new(id: &str, var_field: Vec<&str>, tag: Vec<&str>) -> GroupO {
        GroupO {
            id: id.to_string(),
            var_field: var_field.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
            // convert a Vec<T> to Vec<String>
            // this conversion is used to convert Vec<&str> to Vec<String>
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for GroupO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "O\t{}\t{}\t{}",
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
/// use rs_gfa2::gfa2::*;
/// 
/// // inizialize a simple u-group 
/// let simple_u_group = GroupU {
///     id: "16sub".to_string(),
///     var_field: vec!["2".to_string(), "3".to_string()],
///     tag: vec![],
/// };
/// 
/// // inizialize an empty u-group 
/// // this is allowed but the u-group line will be  
/// // considered not part of the GFA2 format
/// let empty_u_group = GroupU {
///     id: "".to_string(),
///     var_field: vec![],
///     tag: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GroupU {
    // O-Group and U-Group are different only for one field
    // this field can implment or not an optional tag (using * char)
    pub id: String, // optional id, can be either * or id tag
    pub var_field: Vec<String>, // variable field, O-Group have this as optional tag
                                // instead U-Group have dis as normal tag   
    pub tag: Vec<String>,  
}

impl GroupU {
    pub fn new(id: &str, var_field: Vec<&str>, tag: Vec<&str>) -> GroupU {
        GroupU {
            id: id.to_string(),
            var_field: var_field.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
            // convert a Vec<T> to Vec<String>
            // this conversion is used to convert Vec<&str> to Vec<String>
            tag: tag.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for GroupU {
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line {
    Header(Header),
    Segment(Segment),
    Fragment(Fragment),
    Edge(Edge),
    Gap(Gap),
    GroupO(GroupO),
    GroupU(GroupU),
    Comment,
    CustomRecord,
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
/// use rs_gfa2::gfa2::*;
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
/// 
/// // inizialize an empty GFA2 object 
/// let empty_gfa2 = GFA2::new();
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA2 {
    // struct to hold the results of parsing a file; not actually a graph
    // TODO: implement an handlegraph to hold the result of the parsing of a GFA2 file
    pub headers: Vec<Header>,
    pub segments: Vec<Segment>,
    pub fragments: Vec<Fragment>,
    pub edges: Vec<Edge>,
    pub gaps: Vec<Gap>,
    pub groups_o: Vec<GroupO>,
    pub groups_u: Vec<GroupU>,
}

impl GFA2 {
    pub fn new() -> Self {
        GFA2 {
            headers: vec![],
            segments: vec![],
            fragments: vec![],
            edges: vec![],
            gaps: vec![],
            groups_o: vec![],
            groups_u: vec![],
        }
    }
}

impl fmt::Display for GFA2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
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

#[cfg(test)] 
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::parser_gfa2::*;

    #[test]
    fn print_empty_gfa2_file() {
        let gfa2 = GFA2::new();
        println!("{}", gfa2);
    }
        
    #[test]
    fn print_gfa2_file() {
        let parser = parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\irl.gfa"));

        match parser {
            None => panic!("Error parsing the file"),
            Some(parser) => {
                let mut gfa2 = GFA2::new();
                
                gfa2.headers = parser.headers;
                gfa2.segments = parser.segments;
                gfa2.fragments = parser.fragments;
                gfa2.edges = parser.edges;
                gfa2.gaps = parser.gaps;
                gfa2.groups_o = parser.groups_o;
                gfa2.groups_u = parser.groups_u;

                println!("{}", gfa2);
            }
        }
    }

    #[test]
    fn print_gfa2_file_alternative() {
        let mut gfa2 = GFA2::new();
        
        gfa2.headers = vec![
            Header{
                version: "VN:Z:2.0".to_string(),
                tag: vec![],
            }
        ];
        gfa2.segments = vec![
            Segment {
                id: "1".to_string(),
                len: "8".to_string(),
                sequence: "CGATGCAA".to_string(),
                tag: vec![],
            },
            Segment {
                id: "2".to_string(),
                len: "10".to_string(),
                sequence: "TGCAAAGTAC".to_string(),
                tag: vec![],
            },
            Segment {
                id: "3".to_string(),
                len: "21".to_string(),
                sequence: "TGCAACGTATAGACTTGTCAC".to_string(),
                tag: vec!["RC:i:4".to_string()],
            },
            Segment {
                id: "4".to_string(),
                len: "7".to_string(),
                sequence: "TATATGC".to_string(),
                tag: vec![],
            },
            Segment {
                id: "5".to_string(),
                len: "8".to_string(),
                sequence: "CGATGATA".to_string(),
                tag: vec![],
            },
            Segment {
                id: "6".to_string(),
                len: "4".to_string(),
                sequence: "ATGA".to_string(),
                tag: vec![],
            },
        ];
        gfa2.fragments = vec![];
        gfa2.edges = vec![
            Edge {
                id: "*".to_string(),
                sid1: "1+".to_string(),
                sid2: "2+".to_string(),
                beg1: "3".to_string(),
                end1: "8$".to_string(),
                beg2: "0".to_string(),
                end2: "5".to_string(),
                alignment: "0,2,4".to_string(),
                tag: vec!["TS:i:2".to_string()],
            },
            Edge {
                id: "*".to_string(),
                sid1: "3+".to_string(),
                sid2: "2+".to_string(),
                beg1: "21$".to_string(),
                end1: "21$".to_string(),
                beg2: "0".to_string(),
                end2: "0".to_string(),
                alignment: "0M".to_string(),
                tag: vec![],
            },
            Edge {
                id: "*".to_string(),
                sid1: "3+".to_string(),
                sid2: "4-".to_string(),
                beg1: "16".to_string(),
                end1: "21$".to_string(),
                beg2: "3".to_string(),
                end2: "7$".to_string(),
                alignment: "1M1D3M".to_string(),
                tag: vec![],
            },
            Edge {
                id: "*".to_string(),
                sid1: "4-".to_string(),
                sid2: "5+".to_string(),
                beg1: "0".to_string(),
                end1: "0".to_string(),
                beg2: "0".to_string(),
                end2: "0".to_string(),
                alignment: "0M".to_string(),
                tag: vec![],
            },
        ];
        gfa2.gaps = vec![];
        gfa2.groups_o = vec![];
        gfa2.groups_u = vec![];

        println!("{}", gfa2);
    }
}