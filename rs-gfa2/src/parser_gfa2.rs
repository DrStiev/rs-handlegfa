pub mod error;
pub use self::error::{GFA2FieldResult, GFA2Result, ParseError, ParseFieldError};

use bstr::{BStr, BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;
use crate::{gfa2::*, tag::*};

use crate::parser_gfa2::error::ParserTolerance;


/// Builder struct for GFAParsers
pub struct GFA2ParserBuilder {
    pub headers: bool,
    pub segments: bool,
    pub fragments: bool,
    pub edges: bool,
    pub gaps: bool,
    pub groups_o: bool,
    pub groups_u: bool,
    pub tolerance: ParserTolerance,
}

impl GFA2ParserBuilder {
    /// Parse no GFA lines, useful if you only want to parse one line type.
    pub fn none() -> Self {
        GFA2ParserBuilder {
            headers: false,
            segments: false,
            fragments: false,
            edges: false,
            gaps: false,
            groups_o: false,
            groups_u: false,
            tolerance: Default::default(),
        }
    }

    /// Parse all GFA lines.
    pub fn all() -> Self {
        GFA2ParserBuilder {
            headers: true,
            segments: true,
            fragments: true,
            edges: true,
            gaps: true,
            groups_o: true,
            groups_u: true,
            tolerance: Default::default(),
        }
    }

    pub fn ignore_errors(mut self) -> Self {
        self.tolerance = ParserTolerance::IgnoreAll;
        self
    }

    pub fn ignore_safe_errors(mut self) -> Self {
        self.tolerance = ParserTolerance::Safe;
        self
    }

    pub fn pedantic_errors(mut self) -> Self {
        self.tolerance = ParserTolerance::Pedantic;
        self
    }

    pub fn build<N: SegmentId, T: OptFields>(self) -> GFA2Parser<N, T> {
        GFA2Parser {
            headers: self.headers,
            segments: self.segments,
            fragments: self.fragments,
            edges: self.edges,
            gaps: self.gaps,
            groups_o: self.groups_o,
            groups_u: self.groups_u,
            tolerance: self.tolerance,
            _optional_fields: std::marker::PhantomData,
            _segment_names: std::marker::PhantomData,
        }
    }

    pub fn build_usize_id<T: OptFields>(self) -> GFA2Parser<usize, T> {
        self.build()
    }

    pub fn build_bstr_id<T: OptFields>(self) -> GFA2Parser<BString, T> {
        self.build()
    }
}

/// return a GFA2Parser object
/// 
/// # Examples
/// 
/// ```ignore
/// use gfa2::*;
/// use bstr::BString;
/// use parser_gfa2::GFA2Parser;
/// 
/// // create a parser
/// let parser: GFA2Parser<bstr::BString, ()> = GFA2Parser::new();
/// // create a gfa2 object to store the result of the parsing
/// let gfa2: GFA2<BString, ()> = parser.parse_file(&"test\\gfa2_files\\sample2.gfa"). unwrap();
/// ```
#[derive(Clone)]
pub struct GFA2Parser<N: SegmentId, T: OptFields> {
    headers: bool,
    segments: bool,
    fragments: bool,
    edges: bool,
    gaps: bool,
    groups_o: bool,
    groups_u: bool,
    tolerance: ParserTolerance,
    _optional_fields: std::marker::PhantomData<T>,
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId, T: OptFields> Default for GFA2Parser<N, T> {
    fn default() -> Self {
        let config = GFA2ParserBuilder::all();
        config.build()
    }
}

impl<N: SegmentId, T: OptFields> GFA2Parser<N, T> {
    /// Create a new GFAParser that will parse all four GFA line
    /// types, and use the optional fields parser and storage `T`.
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse_gfa_line(&self, bytes: &[u8]) -> GFA2Result<Line<N, T>> {
        let line: &BStr = bytes.trim().as_ref();

        let mut fields = line.split_str(b"\t");
        let hdr = fields.next().ok_or(ParseError::EmptyLine)?;

        let invalid_line =
            |e: ParseFieldError| ParseError::invalid_line(e, bytes);

        let line = match hdr {
            b"H" if self.headers => Header::parse_line(fields).map(Header::wrap),
            b"S" if self.segments => Segment::parse_line(fields).map(Segment::wrap),
            b"F" if self.fragments => Fragment::parse_line(fields).map(Fragment::wrap),
            b"E" if self.edges => Edge::parse_line(fields).map(Edge::wrap),
            b"G" if self.gaps => Gap::parse_line(fields).map(Gap::wrap),
            b"O" if self.groups_o => GroupO::parse_line(fields).map(GroupO::wrap),
            b"U" if self.groups_u => GroupU::parse_line(fields).map(GroupU::wrap),
            _ => return Err(ParseError::UnknownLineType),
        }
        .map_err(invalid_line)?;
        Ok(line)
    }

    pub fn parse_lines<I>(&self, lines: I) -> GFA2Result<GFA2<N, T>>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let mut gfa2 = GFA2::new();

        for line in lines {
            match self.parse_gfa_line(line.as_ref()) {
                Ok(parsed) => gfa2.insert_line(parsed),
                Err(err) if err.can_safely_continue(&self.tolerance) => (),
                Err(err) => return Err(err),
            };
        }

        Ok(gfa2)
    }

    pub fn parse_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<GFA2<N, T>, ParseError> {
        use {
            bstr::io::BufReadExt,
            std::{fs::File, io::BufReader},
        };

        let file = File::open(path)?;
        let lines = BufReader::new(file).byte_lines();

        let mut gfa2 = GFA2::new();

        for line in lines {
            let line = line?;
            match self.parse_gfa_line(line.as_ref()) {
                Ok(parsed) => gfa2.insert_line(parsed),
                Err(err) if err.can_safely_continue(&self.tolerance) => (),
                Err(err) => return Err(err),
            };
        }

        Ok(gfa2)
    }
}

pub struct GFA2ParserLineIter<I, N, T>
where
    N: SegmentId,
    T: OptFields,
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    parser: GFA2Parser<N, T>,
    iter: I,
}

impl<I, N, T> GFA2ParserLineIter<I, N, T>
where
    N: SegmentId,
    T: OptFields,
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    pub fn from_parser(parser: GFA2Parser<N, T>, iter: I) -> Self {
        Self { parser, iter }
    }
}

impl<I, N, T> Iterator for GFA2ParserLineIter<I, N, T>
where
    N: SegmentId,
    T: OptFields,
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    type Item = GFA2Result<Line<N, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_line = self.iter.next()?;
        let result = self.parser.parse_gfa_line(next_line.as_ref());
        Some(result)
    }
}

impl<I, N, T> std::iter::FusedIterator for GFA2ParserLineIter<I, N, T>
where
    N: SegmentId,
    T: OptFields,
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
}

fn next_field<I, P>(mut input: I) -> GFA2FieldResult<P>
where
    I: Iterator<Item = P>,
    P: AsRef<[u8]>,
{
    input.next().ok_or(ParseFieldError::MissingFields)
}

/// function that parses the ref tag
/// ```<ref> <- [!-~]+[+-]```
fn parse_ref<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Reference"))
}

/// function that parses the HEADER field
/// ```H {VN:Z:2.0} {TS:i:<trace spacing>} <tag>*```
impl<T: OptFields> Header<T> {
    #[inline]
    fn wrap<N: SegmentId>(self) -> Line<N, T> {
        Line::Header(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = next_field(&mut input)?;
        let version = OptField::parse(next.as_ref());
        let version2 = version.clone();
        let version =
            if let Some(OptFieldVal::Z(version)) = version.map(|v| v.value) {
                Some(version)
            } else if let Some(OptFieldVal::I(version2)) = version2.map(|v| v.value) {
                Some(version2)
            } else {
                None
            };
        let tag = T::parse(input);

        Ok(Header { 
            version, 
            tag,
         })
    }
}

/// function that parses the sequence tag of the segment element
/// ```<sequence> <- * | [!-~]+```
fn parse_sequence<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|[!-~]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Sequence"))
}

/// function that parses the slen tag of the segment element 
/// ```<int> <- {-}[0-9]+```
fn parse_slen<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\-?[0-9]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Lenght"))
}

/// function that parses the SEGMENT element
/// ```<segment> <- S <sid:id> <slen:int> <sequence> <tag>*```
impl<N: SegmentId, T: OptFields> Segment<N, T> {
    #[inline]
    fn wrap(self) -> Line<N, T> {
        Line::Segment(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next(&mut input)?;
        let len = parse_slen(&mut input)?;
        let sequence = parse_sequence(&mut input)?;
        let tag = T::parse(input);
        Ok(Segment {
            id,
            len,
            sequence,
            tag,
        })
    }
}

/// function that parses the pos tag of the fragment element
/// ```<pos> <- {-}[0-9]+{$}```
fn parse_pos<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\-?[0-9]+\$?").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Position"))
}

/// function that parses the FRAGMENT element
/// ```<fragment> <- F <sid:id> <external:ref> <sbeg:pos> <send:pos> <fbeg:pos> <fend:pos> <alignment> <tag>*```
impl<N: SegmentId, T: OptFields> Fragment<N, T> {
    #[inline]
    fn wrap(self) -> Line<N, T> {
        Line::Fragment(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next(&mut input)?;
        let ext_ref = parse_ref(&mut input)?;
        let sbeg = parse_pos(&mut input)?;
        let send = parse_pos(&mut input)?;
        let fbeg = parse_pos(&mut input)?;
        let fend = parse_pos(&mut input)?;
        let alignment = next_field(&mut input)?.as_ref().into();
        let tag = T::parse(input);
        Ok(Fragment {
            id,
            ext_ref,
            sbeg,
            send,
            fbeg,
            fend,
            alignment,
            tag,
        })
    }
}

/// function that parses the EDGE element
/// ```<edge> <- E <eid:opt_id> <sid1:ref> <sid2:ref> <beg1:pos> <end1:pos> <beg2:pos> <end2:pos> <alignment> <tag>*```
impl<N: SegmentId, T: OptFields> Edge<N, T> {
    #[inline]
    fn wrap(self) -> Line<N, T> {
        Line::Edge(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next_opt(&mut input)?;
        let sid1 = parse_ref(&mut input)?;
        let sid2 = parse_ref(&mut input)?;
        let beg1 = parse_pos(&mut input)?;
        let end1 = parse_pos(&mut input)?;
        let beg2 = parse_pos(&mut input)?;
        let end2 = parse_pos(&mut input)?;
        let alignment = next_field(&mut input)?.as_ref().into();
        let tag = T::parse(input);
        Ok(Edge {
            id,
            sid1,
            sid2,
            beg1,
            end1,
            beg2,
            end2,
            alignment,
            tag,
        })
    }
}

/// function that parses the (dist)int tag of the gap element
/// ```<int> <- {-}[0-9]+```
fn parse_dist<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\-?[0-9]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Distance"))
}

/// function that parses the (var)int tag of the gap element
/// ```<int> <- {-}[0-9]+```
fn parse_var<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)\*|\-?[0-9]+").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Variance"))
}

/// function that parses the GAP element
/// ```<gap> <- G <gid:opt_id> <sid1:ref> <sid2:ref> <dist:int> (* | <var:int>) <tag>*```
impl<N: SegmentId, T: OptFields> Gap<N, T> {
    #[inline]
    fn wrap(self) -> Line<N, T> {
        Line::Gap(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = N::parse_next_opt(&mut input)?;
        let sid1 = parse_ref(&mut input)?;
        let sid2 = parse_ref(&mut input)?;
        let dist = parse_dist(&mut input)?;
        let var = parse_var(&mut input)?;
        let tag = T::parse(input);
        Ok(Gap {
            id,
            sid1,
            sid2,
            dist,
            var,
            tag,
        })
    }
}

/// function that parses the ref tag og the o group element
/// ```<ref> <- [!-~]+[+-]```
fn parse_group_ref<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)[!-~]+[+-]([ ][!-~]+[+-])*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Reference"))
}

/// function that parses the id tag og the o group element
/// ```<id> <- [!-~]+```
fn parse_group_id<I>(input: &mut I) -> GFA2FieldResult<BString>
where
    I: Iterator,
    I::Item: AsRef<[u8]>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?-u)[!-~]+([ ][!-~]+)*").unwrap();
    }

    let next = next_field(input)?;
    RE.find(next.as_ref())
        .map(|s| BString::from(s.as_bytes()))
        .ok_or(ParseFieldError::InvalidField("Id"))
}

/// function that parses the GROUPO element
/// ```<o_group> <- O <oid:opt_id> <ref>([ ]<ref>)* <tag>*```
impl<N: SegmentId, T: OptFields> GroupO<N, T> {
    #[inline]
    fn wrap(self) -> Line<N, T> {
        Line::GroupO(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = BString::parse_next_opt(&mut input)?;
        let var_field = parse_group_ref(&mut input)?;
        let tag = T::parse(input);
        Ok(GroupO::new(id, var_field, tag))
    }
}

/// function that parses the GROUPO element
/// ```<u_group> <- U <uid:opt_id>  <id>([ ]<id>)*  <tag>*```
impl<N: SegmentId, T: OptFields> GroupU<N, T> {
    #[inline]
    fn wrap(self) -> Line<N, T> {
        Line::GroupU(self)
    }

    #[inline]
    fn parse_line<I>(mut input: I) -> GFA2FieldResult<Self>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let id = BString::parse_next_opt(&mut input)?;
        let var_field = parse_group_id(&mut input)?;
        let tag = T::parse(input);
        Ok(GroupU::new(id, var_field, tag))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_header() {
        let header = "VN:Z:2.0";
        let header_ = Header {
            // the version field of the header object is used to store
            // the entire header tag without the optional (tag)*
            version: Some("VN:Z:2.0".into()), 
            tag: (),
        };

        let result: GFA2FieldResult<Header<()>> = Header::parse_line([header].iter());

        match result {
            Err(why) => println!("Error: {}", why),
            Ok(h) => assert_eq!(h, header_),
        }
    }

    #[test]
    fn can_parse_segment() {
        let segment = "A\t10\tAAAAAAACGT";
        let segment_: Segment<BString, _> = Segment {
            id: "A".into(),
            len: "10".into(),
            sequence: "AAAAAAACGT".into(),
            tag:(),
        };

        let fields = segment.split_terminator('\t');
        let result = Segment::parse_line(fields);

        match result{
            Err(why) => println!("Error: {}", why),
            Ok(s) => assert_eq!(s, segment_),
        }
    }

    #[test]
    fn can_parse_fragment() {
        let fragment = "15\tr1-\t10\t10\t20\t20\t*";
        let fragment_: Fragment<BString, _> = Fragment {
            id: "15".into(),
            ext_ref: "r1-".into(),
            sbeg: "10".into(),
            send: "10".into(),
            fbeg: "20".into(),
            fend: "20".into(),
            alignment: "*".into(),
            tag:(),
        };

        let fields = fragment.split_terminator('\t');
        let result = Fragment::parse_line(fields);

        match result{
            Err(why) => println!("Error: {}", why),
            Ok(f) => assert_eq!(f, fragment_),
        }
    }

    #[test]
    fn can_parse_edge() {
        let edge = "*\t2+\t45+\t2531\t2591$\t0\t60\t60M";
        let edge_: Edge<BString, _> = Edge {
            id: "*".into(),
            sid1: "2+".into(),
            sid2: "45+".into(),
            beg1: "2531".into(),
            end1: "2591$".into(),
            beg2: "0".into(),
            end2: "60".into(),
            alignment: "60M".into(),
            tag:(),
        };

        let fields = edge.split_terminator('\t');
        let result = Edge::parse_line(fields);

        match result{
            Err(why) => println!("Error: {}", why),
            Ok(e) => assert_eq!(e, edge_),
        }
    }
    #[test]
    fn can_parse_gap() {
        let gap = "g1\t7+\t22+\t10\t*";
        let gap_: Gap<BString, _> = Gap {
            id: "g1".into(),
            sid1: "7+".into(),
            sid2: "22+".into(),
            dist: "10".into(),
            var: "*".into(),
            tag:(),
        };

        let fields = gap.split_terminator('\t');
        let result = Gap::parse_line(fields);

        match result{
            Err(why) => println!("Error: {}", why),
            Ok(g) => assert_eq!(g, gap_),
        }
    }

    #[test]
    fn can_parse_ogroup() {
        let ogroup = "P1\t36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-";
        let ogroup_: GroupO<BString, _> =
            GroupO::new( 
                "P1".into(),
                "36+ 53+ 53_38+ 38_13+ 13+ 14+ 50-".into(),
                (),
            );

        let fields = ogroup.split_terminator('\t');
        let result = GroupO::parse_line(fields);

        match result{
            Err(why) => println!("Error {}", why),
            Ok(o) => assert_eq!(o, ogroup_),
        }
    }

    #[test]
    fn can_parse_ugroup() {
        let ugroup = "SG1\t16 24 SG2 51_24 16_24";
        let ugroup_: GroupU<BString, _> =
            GroupU::new( 
                "SG1".into(),
                "16 24 SG2 51_24 16_24".into(),
                (),
            );

        let fields = ugroup.split_terminator('\t');
        let result = GroupU::parse_line(fields);

        match result{
            Err(why) => println!("Error: {}", why),
            Ok(u) => assert_eq!(u, ugroup_),
        }
    }

    #[test]
    fn can_parse_gfa2_file_with_tag() {
        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> =
            parser.parse_file(&"./src/tests/gfa2_files/sample2.gfa").unwrap();
        
        let head = gfa2.headers.len();
        let seg = gfa2.segments.len();
        let frag = gfa2.fragments.len();
        let edge = gfa2.edges.len();
        let gap = gfa2.gaps.len();
        let ogroup = gfa2.groups_o.len();
        let ugroup = gfa2.groups_u.len();

        assert_eq!(head, 4);
        assert_eq!(seg, 9);
        assert_eq!(frag, 2);
        assert_eq!(edge, 6);
        assert_eq!(gap, 2);
        assert_eq!(ogroup, 2);
        assert_eq!(ugroup, 2);

        println!("{}", gfa2);
    }

    #[test]
    fn can_parse_gfa2_file_with_no_tag() {
        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> =
            parser.parse_file(&"./src/tests/gfa2_files/data.gfa").unwrap();
    
        println!("{}", gfa2);
    }

    #[test]
    fn can_parse_multiple_tag() {
        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> =
            parser.parse_file(&"./src/tests/gfa2_files/sample.gfa").unwrap();
    
        println!("{}", gfa2);
    }
}