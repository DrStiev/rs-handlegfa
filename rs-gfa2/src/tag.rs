/// file that tries to mimic the behaviour of the file optfields.rs 
/// optfields.rs find, parse and store all the different types of
/// optional fields associated to each kind of lines.
/// with the format GFA2 the optional field tag is been replaced by a 
/// simple tag element with 0 or N occurencies.
/// So, I don't think this file could be useful as the original.
use bstr::BString;
use lazy_static::lazy_static;
use regex::bytes::Regex;

/// These type aliases are useful for configuring the parsers, as the
/// type of the optional field container must be given when creating a
/// GFAParser or GFA object.
pub type OptionalFields = Vec<OptField>;
pub type NoOptionalFields = ();

/// An optional field a la SAM. Identified by its tag, which is any
/// two characters matching [A-Za-z][A-Za-z0-9].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OptField {
    pub tag: [u8; 2],
    pub value: OptFieldVal,
}

/// enum for representing each of the SAM optional field types. The
/// `B` type, which denotes either an integer or float array, is split
/// in two variants, and they ignore the size modifiers in the spec,
/// instead always holding i64 or f32.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OptFieldVal {
    Z(BString),
    I(BString),
    F(BString),
    A(BString),
    J(BString),
    H(BString),
    B(BString),
}

impl OptField {
    /// Panics if the provided tag doesn't match the regex
    /// [A-Za-z0-9][A-Za-z0-9].
    pub fn tag(t: &[u8]) -> [u8; 2] {
        assert_eq!(t.len(), 2);
        assert!(t[0].is_ascii_alphanumeric());
        assert!(t[1].is_ascii_alphanumeric());
        [t[0], t[1]]
    }

    /// Create a new OptField from a tag name and a value, panicking
    /// if the provided tag doesn't fulfill the requirements of
    /// OptField::tag().
    pub fn new(tag: &[u8], value: OptFieldVal) -> Self {
        let tag = OptField::tag(tag);
        OptField { tag, value }
    }

    /// Parses the header and optional fields from a bytestring in the format\ 
    /// ```<Header> <- {VN:Z:2.0}\t{TS:i:[-+]?[0-9]+}\t<tag>*```
    /// ```<tag> <- <TAG>:<TYPE>:<VALUE> <- [A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*```
    pub fn parse(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = 
                Regex::new(r"(?-u)([A-Za-z0-9][A-Za-z0-9]:[ABHJZif]:[ -~]*)*").unwrap();
        }

        use OptFieldVal::*;

        let o_tag = input.get(0..=1)?;
        let o_type = input.get(3)?;

        let o_val = match o_type {
            b'A' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(A),
            b'i' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(I),
            b'f' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(F),
            b'Z' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(Z),
            b'J' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(J),
            b'H' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(H),
            b'B' => RE
                .find(input)
                .map(|s| s.as_bytes().into())
                .map(B),
            _ => None,
        }?;

        Some(Self::new(o_tag, o_val))
    }
}

/// The Display implementation produces spec-compliant strings in the
/// ```<TAG>:<TYPE>:<VALUE>``` format, and can be parsed back using
/// OptField::parse().
impl std::fmt::Display for OptField {
    fn fmt(&self, form: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OptFieldVal::*;

        match &self.value {
            A(x) => write!(form, "{}", x),
            I(x) => write!(form, "{}", x),
            F(x) => write!(form, "{}", x),
            Z(x) => write!(form, "{}", x),
            J(x) => write!(form, "{}", x),
            H(x) => write!(form, "{}", x),
            B(x) => write!(form, "{}", x),
        }
    }
}

/// The OptFields trait describes how to parse, store, and query
/// optional fields. Each of the GFA line types and the GFA struct
/// itself are generic over the optional fields, so the choice of
/// OptFields implementor can impact memory usage, which optional
/// fields are parsed, and possibly more in the future
pub trait OptFields: Sized + Default + Clone {
    /// Return the optional field with the given tag, if it exists.
    fn get_field(&self, tag: &[u8]) -> Option<&OptField>;

    /// Return a slice over all optional fields. NB: This may be
    /// replaced by an iterator or something else in the future
    fn fields(&self) -> &[OptField];

    /// Given an iterator over bytestrings, each expected to hold one
    /// optional field (in the <TAG>:<TYPE>:<VALUE> format), parse
    /// them as optional fields to create a collection. Returns `Self`
    /// rather than `Option<Self>` for now, but this may be changed to
    /// become fallible in the future.
    fn parse<T>(input: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>;
}

/// This implementation is useful for performance if we don't actually
/// need any optional fields. () takes up zero space, and all
/// methods are no-ops.
impl OptFields for () {
    fn get_field(&self, _: &[u8]) -> Option<&OptField> {
        None
    }

    fn fields(&self) -> &[OptField] {
        &[]
    }

    fn parse<T>(_input: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>,
    {
    }
}

/// Stores all the optional fields in a vector. `get_field` simply
/// uses std::iter::Iterator::find(), but as there are only a
/// relatively small number of optional fields in practice, it should
/// be efficient enough.
impl OptFields for Vec<OptField> {
    fn get_field(&self, tag: &[u8]) -> Option<&OptField> {
        self.iter().find(|o| o.tag == tag)
    }

    fn fields(&self) -> &[OptField] {
        self.as_slice()
    }

    fn parse<T>(input: T) -> Self
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>,
    {
        input
            .into_iter()
            .filter_map(|f| OptField::parse(f.as_ref()))
            .collect()
    }
}