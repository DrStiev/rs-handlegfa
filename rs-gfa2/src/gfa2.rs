
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OptionalFieldValue {
    PrintableChar(char),
    SignedInt(i64),
    Float(f32),
    PrintableString(String),
    JSON(String),
    ByteArray(Vec<u8>),
    IntArray(Vec<i32>),
    FloatArray(Vec<f32>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OptionalField {
    pub tag: String,
    pub content: OptionalFieldValue,
}

/// The header contains an optional 'VN' SAM-tag version number, 2.0, 
/// and an optional 'TS' SAM-tag specifying the default the trace point spacing for any Dazzler traces 
/// specified to accelerate alignment computation. Any number of header lines containing SAM-tags may occur. 
/// A 'TS' tag can occur after the fixed arguments on any E-, G-, or F-line in which case 
/// it specifies the trace spacing to use with the trace on that specific line, 
/// otherwise the default spacing is used.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header {
    pub version: String,
    // the trace spacing field and the tag field are optional so u can use 
    // the OptionalFieldValue to handle them
}

/// A segment is specified by an S-line giving a user-specified ID for the sequence, 
/// its length in bases, and the string denoted by the segment or * if absent. 
/// The sequence is typically expected to be bases or IUPAC characters, 
/// but GFA2 places no restriction other than that they be printable characters other than space. 
/// The length does not need to be the actual length of the sequence, if the sequence is given, 
/// but rather an indication to a drawing program of how long to draw the representation of the segment. 
/// The segment sequences and any CIGAR strings referring to them if present follow the unpadded SAM convention.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Segment {
    pub id: String,
    pub len: String,
    pub sequence: String,
}

impl Segment {
    pub fn new(name: &str, len: &str, sequence: &str) -> Self {
        Segment {
            id: name.to_string(),
            len: len.to_string(),
            sequence: sequence.to_string(),
        }
    }
}

// idk if keep this function or not, probably not
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

/// Fragments, if present, are encoded in F-lines that give 
/// (a) the segment they belong to, 
/// (b) an oriented external ID that references a sequence in an external collection 
/// (e.g. a database of reads or segments in another GFA2 or SAM file), 
/// (c) the interval of the vertex segment that the external string contributes to, and 
/// (d) the interval of the fragment that contributes to the segment. 
/// One concludes with either a trace or CIGAR string detailing the alignment, or a * if absent.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Fragment {
    pub id: String,
    pub ext_ref: String, // orientation as final char (+-)
    pub sbeg: String,
    pub send: String, // dollar character as optional final char
    pub fbeg: String,
    pub fend: String,
    // alignment field can be *, trace or CIGAR
    pub alignment: String, 
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
    ) -> Fragment {
        Fragment {
            id: id.to_string(),
            ext_ref: ext_ref.to_string(),
            sbeg: sbeg.to_string(),
            send: send.to_string(),
            fbeg: fbeg.to_string(),
            fend: fend.to_string(),
            alignment: alignment.to_string(),
        }
    }
}

/// Edges are encoded in E-lines that in general represent a local alignment between arbitrary intervals 
/// of the sequences of the two vertices in question. 
/// One gives first an edge ID or * and then the oriented segment ID’s of the two vertices involved.
/// One then gives the intervals of each segment that align, each as a pair of positions. 
/// A position is an integer optionally followed by a $-sign. 
/// Positions are conceptually tick-marks between symbols starting a 0 to the left of the first symbol and ending at L 
/// to the right of the last symbol where L is the length of the segment. 
/// A $-sign must follow an integer x if and only if it is the last position in the segment it refers to, 
/// i.e. x = L. It is an error to do otherwise.
/// Position intervals are always intervals in the segment in its normal orientation before being oriented by 
/// the orientation signs. 
/// If a minus sign is specified, then the interval of the second segment is reverse complemented in order to 
/// align with the interval of the first segment. 
/// That is, E * s1+ s2- b1 e1 b2 e2 aligns s1[b1,e1] to the reverse complement of s2[b2,e2].
/// A field for a CIGAR string or Dazzler-trace describing the alignment is last, but may be absent by giving a *. 
/// One gives a CIGAR string to describe an exact alignment relationship between the two segments. 
/// A trace string by contrast is given when one simply wants an accelerated method for computing an alignment 
/// between the two intervals. A trace is a list of integers separated by commas, 
/// each integer giving the # of characters in the second segment to align to the next TS characters 
/// in the first segment where the TS is either the default trace spacing given in a header line with the TS SAM-tag, 
/// or the spacing given in a TS SAM-tag on the line of the edge. 
/// If a * is given as the alignment note that it is still possible to compute the implied alignment from the sequences.
/// The GFA2 concept of edge generalizes the link and containment lines of GFA. 
/// For example a GFA edge which encodes what is called a dovetail overlap (because two ends overlap) 
/// is a GFA2 edge where:
/// beg1 = 0 and end2 = y$ or beg2 = 0 and end1 = x$ (if the aligned segments are in the same orientation)
/// beg1 = 0 and beg2 = 0 or end1 = x$ and end2 = y$ (if the aligned segments are in opposite orientation)
/// while GFA containment is modeled by the case where either beg1 = 0 and end1 = x$ or beg2 = 0 and end2 = x$.
/// ![Edge representation](https://github.com/GFA-spec/GFA-spec/blob/master/images/GFA2.Fig1.png)
/// Special codes could be adopted for dovetail and containment relationships but the thought is 
/// there is no particular reason to do so, the use of the $ sentinel for terminal positions makes their identification 
/// simple both algorithmically and visually, and the more general scenario allows interesting possibilities. 
/// For example, one might have two haplotype bubbles shown in the “Before” picture below, 
/// and then in a next phase choose a path through the bubbles as the primary “contig”, 
/// and then capture the two bubble alternatives as a vertex linked with generalized edges shown in the “After” picture. 
/// Note carefully that you need a generalized edge to capture the attachment of the two haplotype bubbles in the “After” picture.
/// ![Edge representation 2](https://github.com/GFA-spec/GFA-spec/blob/master/images/GFA2.Fig2.png)
/// While one has graphs in which vertex sequences actually overlap as above, one also frequently encounters models 
/// in which there is no overlap (basically edge-labelled models captured in a vertex-labelled form). 
/// This is captured by edges for which
/// beg1 = end1 = x$ and beg2 = end2 = 0 (i.e. 0-length overlap of the end of segment 1 and the beginning of segment 2)!
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Edge {
    pub id: String, // optional id, can be either * or id tag
    pub sid1: String, // orientation as final char (+-)
    pub sid2: String, // orientation as final char (+-)
    pub beg1: String,
    pub end1: String, // dollar character as optional final char
    pub beg2: String,
    pub end2: String, // dollar character as optional final char
    pub alignment: String,
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
        }
    }
}

/// While not a concept for pure DeBrujin or long-read assemblers, 
/// it is the case that paired end data and external maps often order and orient contigs/vertices into scaffolds 
/// with intervening gaps. 
/// To this end we introduce a gap edge described in G-lines that give the estimated gap distance between the two segment sequences 
/// and the variance of that estimate. 
/// The gap is between the first segment at left and the second segment at right where the segments are oriented 
/// according to their sign indicators. 
/// The next integer gives the expected distance between the first and second segment in their respective orientations, 
/// and the final field is either an integer giving the variance in this estimate or a * indicating the variance is unknown. 
/// Relationships in E-lines are fixed and known, where as in a G-line, 
/// the distance is an estimate and the line type is intended to allow one to define assembly scaffolds.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Gap {
    pub id: String, // optional id, can be either * or id tag
    pub sid1: String, // orientation as final char (+-)
    pub sid2: String, // orientation as final char (+-)
    pub dist: String,
    pub var: String,
}

impl Gap {
    pub fn new(
        id: &str,
        sid1: &str,
        sid2: &str,
        dist: &str,
        var: &str,
    ) -> Gap {
        Gap {
            id: id.to_string(),
            sid1: sid1.to_string(),
            sid2: sid2.to_string(),
            dist: dist.to_string(),
            var: var.to_string(),
        }
    }
}

/// A group encoding on a U- or O-line allows one to name and specify a subgraph of the overall graph. 
/// Such a collection could for example be highlighted by a drawing program on command, 
/// or might specify decisions about tours through the graph. 
/// U-lines encode unordered collections and O-lines encode ordered collections (defined in the next paragraph), 
/// which we alternatively call sets and paths, respectively. 
/// The remainder of the line then consists of an optional ID for the collection followed by a non-empty list of ID's 
/// referring to segments, edges, or other groups that are separated by single spaces 
/// (i.e. the list is in a single column of the tab-delimited format). 
/// In the case of paths every reference must be oriented, and not so in a set. 
/// A group list may refer to another group recursively. 
/// It is an error for a U-line and an O-line to have the same name.
/// An unordered collection or set defined in a U-line refers to the subgraph induced by the vertices and edges in the collection 
/// (i.e. one adds all edges between a pair of segments in the list and one adds all segments adjacent to edges in the list.) 
/// An ordered collection defined in an O-line captures paths in the graph consisting of the listed objects 
/// and the implied adjacent objects between consecutive objects in the list where the orientation of the objects 
/// matters (e.g. the edge between two consecutive segments, the segment between two consecutive edges, etc.) 
/// A set can contain a reference to a path, but not vice versa, 
/// in which case the orientation of the objects in the path become irrelevant.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
// O-Group and U-Group are different only for one field
// this field can implment or not an optional tag (using * char)
pub struct Group {
    pub id: String, // optional id, can be either * or id tag
    pub var_field: String, // variable field, O-Group have this as optional tag
                    // instead U-Group have dis as normal tag    
    // vec_var_field: Vec<String>, 
}

impl Group {
    pub fn new(id: &str, var_field: &str) -> Group {
        Group {
            id: id.to_string(),
            var_field: var_field.to_string(),
            // convert a Vec<T> to Vec<String>
            // this conversion is used to convert Vec<&str> to Vec<String>
            // vec_var_field: vec_var_field.iter().map(|&s| s.to_string()).collect::<Vec<String>>(),
        }
    }
}

/// Like GFA, GFA2 is tab-delimited in that every lexical token is separated from the next by a single tab.
/// Each record line must begin with a letter and lies on a single line with no white space before the first symbol. 
/// The tokens that generate record lines are <header>, <segment>, <fragment>, <edge>, <gap>, and <group>. 
/// Any line that does not begin with a recognized code (i.e. H, S, F, E, G, O, or U) can be ignored. 
/// This will allow users to have additional record lines specific to their special processes. 
/// Moreover, the suffix of any GFA2 record line may contain any number of user-specific SAM tags 
/// which may be ignored by software designed to support the core standard. 
/// Tags with lower-case letters are reserved for end-users.
/// There is one name space for all identifiers for segments, edges, gaps, and groups. 
/// External fragment ID's are assumed to be in a distinct name space. 
/// It is an error for any identifier to be used twice in a defining context. 
/// Note carefully that instead of an identifier, one can use a * for edges, gaps, and groups, 
/// implying that an id is not needed as the item will not be referred to elsewhere in the file. 
/// Moreover, almost all references to identifiers are oriented, by virtue of a post-fix + or - sign. 
/// A +-sign indicates the object is in the orientation it was defined, and a --sign indicates it should be reverse-complemented.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line {
    Header(Header),
    Segment(Segment),
    Fragment(Fragment),
    Edge(Edge),
    Gap(Gap),
    Group(Group),
    Comment,
    CustomRecord,
}

// struct to hold the results of parsing a file; not actually a graph
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA2 {
    pub headers: Vec<Header>,
    pub segments: Vec<Segment>,
    pub fragments: Vec<Fragment>,
    pub edges: Vec<Edge>,
    pub gaps: Vec<Gap>,
    pub groups: Vec<Group>,
}

impl GFA2 {
    pub fn new() -> Self {
        GFA2 {
            headers: vec![],
            segments: vec![],
            fragments: vec![],
            edges: vec![],
            gaps: vec![],
            groups: vec![],
        }
    }
}