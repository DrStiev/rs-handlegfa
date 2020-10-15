/// implement the Display trait for all the struct in gfa2.rs
use std::fmt;

/// Returns an Header line which is composed of:\
///     * [`version`][string] field, 
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `version` - A [`string`][string] slice.
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::gfa::*;
/// 
/// // inizialize a simple header 
/// let simple_header = Header {
///     version: "VN:Z:1.0".to_string(),
/// };
/// 
/// // inizialize an empty header
/// // this is allowed because all the fields 
/// // of an Header line are optional 
/// let empty_header = Header {
///     version: "".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header {
    pub version: String,
}

impl Header {
    pub fn new(version: &str) -> Header {
        Header{
            version: version.to_string(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "H\t{}", self.version)
    }
}

/// Returns a Segment line which is composed of:\
///     * [`name`][string] field,\
///     * [`sequence`][string] field,\
///     * [`optional fields`][vec] fields
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `name` - A [`string`][string] slice,\
///     representing the name associated to the current segment.
/// * `sequence` - A [`string`][string] slice, \
///     this `string` is  typically expected to be bases or IUPAC characters, \
///     but there's no restriction other than that the characters must be printable.
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::gfa::*;
/// 
/// // inizialize a simple segment 
/// let simple_segment = Segment {
///     name: "3".to_string(),
///     sequence: "TGCAACGTATAGACTTGTCAC".to_string(),
///     optional_fields: vec![],
/// };
/// 
/// // inizialize an empty segment
/// // this is allowed but the segment line will be  
/// // considered not part of the GFA format
/// let empty_segment = Segment {
///     name: "".to_string(),
///     sequence: "".to_string(),
///     optional_fields: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Segment {
    pub name: String,
    pub sequence: String,

    pub optional_fields: Vec<String>,
}

impl Segment {
    pub fn new(name: &str, sequence: &str, optional_fields: Vec<&str>) -> Segment {
        Segment {
            name: name.to_string(),
            sequence: sequence.to_string(),

            optional_fields: optional_fields.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "S\t{}\t{}\t{}",
            self.name,
            self.sequence,
            
            self.optional_fields.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Orientation {
    Forward,
    Backward,
}

impl Orientation {
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Forward => true,
            Self::Backward => false,
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let orient = match self {
            Self::Backward => "-",
            Self::Forward => "+",
        };
        write!(f, "{}",orient)
    }
}

/// Returns a Link line which is composed of:\
///     * [`from segment`][string] field,\
///     * [`from orient`][string] field,\
///     * [`to segment`][string] field,\
///     * [`to orient`][string] field,\
///     * [`overlap`][string] field,\
///     * [`optional fields][vec] fields,
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// 
/// # Arguments
/// 
/// * `from segment` and `to segment` - A pair of [`string`][string] slices,\
///     representing the name of the segment that is used to express the link.\
///     the link starts from the 'from segment' and end to the 'to segment'
/// * `from orient` and `to orient` - A pair of [`string`][string] slices, \
///     representing how to overlap of string A on the string B.\
///     orientation marked as `-` replace the sequence of the segment with its reverse complement\
///     whereas a `+` mark indicates the segment sequence is used as-is.
/// * `overlap` - A [`string`][string] slice,\
///     representing the length of the ovelap describing as [`cigar`][cigar]
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::gfa::*;
/// 
/// // inizialize a simple link 
/// let simple_link = Link {
///     from_segment: "1".to_string(),
///     from_orient: "+".to_string(),
///     to_segment: "2".to_string(),
///     to_orient: "+".to_orient(),
///     overlap: "5M".to_string(),
///     optional_fields: vec![], 
/// };
/// 
/// // inizialize an empty link
/// // this is allowed but the link line will be  
/// // considered not part of the GFA format
/// let empty_link = Link {
///     from_segment: "".to_string(),
///     from_orient: "".to_string(),
///     to_segment: "".to_string(),
///     to_orient: "".to_orient(),
///     overlap: "".to_string(),
///     optional_fields: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Link {
    pub from_segment: String,
    pub from_orient: Orientation,
    pub to_segment: String,
    pub to_orient: Orientation,
    pub overlap: String,

    pub optional_fields: Vec<String>,
}

impl Link {
    pub fn new(
        from_segment: &str,
        from_orient: Orientation,
        to_segment: &str,
        to_orient: Orientation,
        overlap: &str,
        optional_fields: Vec<&str>,
    ) -> Link {
        Link {
            from_segment: from_segment.to_string(),
            from_orient: from_orient,
            to_segment: to_segment.to_string(),
            to_orient: to_orient,
            overlap: overlap.to_string(),

            optional_fields: optional_fields.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "L\t{}\t{}\t{}\t{}\t{}\t{}",
            self.from_segment,
            self.from_orient,
            self.to_segment,
            self.to_orient,
            self.overlap,

            self.optional_fields.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Containment line which is composed of:\
///     * [`container name`][string] field,\
///     * [`container orient`][string] field,\
///     * [`contained name`][string] field,\
///     * [`contained orient`][string] field,\
///     * [`position`][string] field,\
///     * [`overlap`][string] field,\
///     * [`optional fields`][vec] fields,
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// 
/// # Arguments
/// 
/// * `container name` and `contained name` - A pair of [`string`][string] slices,\
///     representing the name of the overlapping segment where the `contained name` one \
///     is contained in the `container name` one.
/// * `cantainer orient` and `contained orient` - A pair of [`string`][string] slices, \
///     representing the orientation of the segments associated to them.
/// * `position` - A [`string`][string] slice, \
///     representing the leftmost position of the `contained name` segment in the \
///     `container name` segment in its forward orientation.
/// * `overlap` - A [`string`][string] slice,\
///     representing the length of the ovelap describing as [`cigar`][cigar]
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::gfa::*;
/// 
/// // inizialize a simple Containment 
/// let simple_containment = Containment {
///     container_name: "1".to_string(),
///     container_orientation: "+".to_string(),
///     contained_name: "5".to_string(),
///     contained_orientation: "+".to_orient(),
///     pos: "12". to_string(),
///     overlap: "120M".to_string(),
///     optional_fields> vec![],
/// };
/// 
/// // inizialize an empty containment
/// // this is allowed but the containment line will be  
/// // considered not part of the GFA format
/// let empty_containment = Containment {
///     container_name: "".to_string(),
///     container_orientation: "".to_string(),
///     contained_name: "".to_string(),
///     contained_orientation: "".to_orient(),
///     pos: "". to_string(),
///     overlap: "".to_string()
///     optional_fields> vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Containment {
    pub container_name: String,
    pub container_orient: Orientation,
    pub contained_name: String,
    pub contained_orient: Orientation,
    pub pos: usize,
    pub overlap: String,

    pub optional_fields: Vec<String>,
}

impl Containment {
    fn new(
        container_name: &str,
        container_orient: Orientation,
        contained_name: &str,
        contained_orient: Orientation,
        pos: usize,
        overlap: &str,
        optional_fields: Vec<&str>,
    ) -> Containment {
        Containment {
            container_name: container_name.to_string(),
            container_orient: container_orient,
            contained_name: contained_name.to_string(),
            contained_orient: contained_orient,
            pos: pos,
            overlap: overlap.to_string(),
            optional_fields: optional_fields.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

impl fmt::Display for Containment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "C\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.container_name,
            self.container_orient,
            self.contained_name,
            self.contained_orient,
            self.pos,
            self.overlap,

            self.optional_fields.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Path line which is composed of:\
///     * [`path name`][string] field,\
///     * [`segments names`][vec] fields,\
///     * [`overlaps`][vec] fields
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// [cigar]: https://samtools.github.io/hts-specs/SAMv1.pdf
/// 
/// # Arguments
/// 
/// * `path name` - A [`string`][string] slice,\
///     representing the name of the path.
/// * `segment names` - A [`vector of string`][vec] slices, \
///     representing a comma-separated list of segment names and orientations.
/// * `overlaps` -  A [`vector of string`][vec] slices,\
///     representing optional comma-separated list of [`CIGAR`][cigar] strings.
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::gfa::*;
/// 
/// // inizialize a simple Path 
/// let simple_path = Path {
///     path_name: "14".to_string(),
///     segment_names: vec!["11+".to_string(), "12+".to_string()],
///     overlaps: vec!["122M".to_string()],
/// };
/// 
/// // inizialize an empty path
/// // this is allowed but the path line will be  
/// // considered not part of the GFA format
/// let empty_path = Path {
///     path_name: "".to_string(),
///     segment_names: vec![],
///     overlaps: vec![],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Path {
    pub path_name: String,
    pub segment_names: Vec<String>,
    pub overlaps: Vec<String>,
}

impl Path {
    pub fn new(path_name: &str, seg_names: Vec<&str>, overlaps: Vec<&str>) -> Path {
        let segment_names = seg_names.iter().map(|s| s.to_string()).collect();
        let overlaps = overlaps.iter().map(|s| s.to_string()).collect();
        Path {
            path_name: path_name.to_string(),
            segment_names,
            overlaps,
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P\t{}\t{}\t{}",
            self.path_name,
            self.segment_names.iter().fold(String::new(), |acc, str| acc + &str.to_string() + ","),
            self.overlaps.iter().fold(String::new(), |acc, str| acc + &str.to_string() + ","),
        )
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line {
    Header(Header),
    Segment(Segment),
    Link(Link),
    Containment(Containment),
    Path(Path),
    Comment,
}

/// Returns a GFA object which is composed of:\
///     * [`headers`][vec] field,\
///     * [`segments`][vec] field,\
///     * [`links`][vec] field,\
///     * [`containmnets`][vec] field,\
///     * [`paths`][vec] field,\
/// 
/// [string]: https://doc.rust-lang.org/std/string/struct.String.html
/// 
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// 
/// # Arguments
/// 
/// * `headers` - A [`vector of Header`][vec].
/// * `segments` - A [`vector of Segment`][vec]. 
/// * `links` - A [`vector of Link`][vec].
/// * `containments` - A [`vector of Containment`][vec].
/// * `paths` - A [`vector of Path`][vec].
/// 
/// # Examples
/// 
/// ```
/// use rs_gfa2::gfa::*;
/// 
/// // inizialize a simple gfa object 
/// let gfa_correct = GFA {
///     headers: vec![
///         Header::new("VN:Z:1.0"),
///     ],
///     segments: vec![
///         Segment::new("1", "CAAATAAG"),
///         Segment::new("2", "A"),
///         Segment::new("3", "G"),
///         Segment::new("4", "T"),
///         Segment::new("5", "C"),
///     ],
///     links: vec![
///         Link::new("1", Orientation::Forward, "2", Orientation::Forward, "0M"),
///         Link::new("1", Orientation::Forward, "3", Orientation::Forward, "0M"),
///     ],
///     paths: vec![Path::new(
///         "x",
///         vec![
///             "1+", "3+", "5+", "6+", "8+", "9+", "11+", "12+", "14+", "15+",
///         ],
///         vec!["8M", "1M", "1M", "3M", "1M", "19M", "1M", "4M", "1M", "11M"],
///     )],
///     containments: vec![],
/// };
/// // inizialize an empty GFA object 
/// let empty_gfa = GFA::new();
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA {
    // struct to hold the results of parsing a file; not actually a graph
    pub headers: Vec<Header>,
    pub segments: Vec<Segment>,
    pub links: Vec<Link>,
    pub containments: Vec<Containment>,
    pub paths: Vec<Path>,
}

impl GFA {
    pub fn new() -> Self {
        GFA {
            headers: vec![],
            segments: vec![],
            links: vec![],
            containments: vec![],
            paths: vec![],
        }
    }
}

impl fmt::Display for GFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}{}{}{}{}",
            self.headers.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.links.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.containments.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.paths.iter().fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_empty_gfa_file() {
        let gfa = GFA::new();
        println!("{}", gfa);
    }

    #[test]
    fn print_gfa_file() {
        let gfa_correct = GFA {
            headers: vec![
                Header::new("VN:Z:1.0"),
            ],
            segments: vec![
                Segment::new("1", "CAAATAAG", vec![]),
                Segment::new("2", "A", vec![]),
                Segment::new("3", "G", vec![]),
                Segment::new("4", "T", vec![]),
                Segment::new("5", "C", vec![]),
            ],
            links: vec![
                Link::new("1", Orientation::Forward, "2", Orientation::Forward, "0M", vec![]),
                Link::new("1", Orientation::Forward, "3", Orientation::Forward, "0M", vec![]),
            ],
            paths: vec![Path::new(
                "x",
                vec!["1+", "3+", "5+", "6+", "8+", "9+", "11+", "12+", "14+", "15+"],
                vec!["8M", "1M", "1M", "3M", "1M", "19M", "1M", "4M", "1M", "11M"],
            )],
            containments: vec![],
        };

        println!("{}", gfa_correct);
    }
}