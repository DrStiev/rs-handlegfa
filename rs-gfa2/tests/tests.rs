#[cfg(test)]
mod tests {
    use gfa2::gfa2::*;
    use gfa2::parser_gfa2::GFA2Parser;
    use gfa2::tag::OptionalFields;
    use bstr::BString;
    
    #[test]
    fn can_parse_gfa2_file_with_tag() {
        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> =
            parser.parse_file(&"./tests/gfa2_files/sample2.gfa").unwrap();
        
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
            parser.parse_file(&"./tests/gfa2_files/data.gfa").unwrap();
    
        println!("{}", gfa2);
    }

    #[test]
    fn can_parse_multiple_tag() {
        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> =
            parser.parse_file(&"./tests/gfa2_files/sample.gfa").unwrap();
    
        println!("{}", gfa2);
    }
}