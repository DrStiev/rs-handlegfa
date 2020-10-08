use super::*;

    #[test]
    fn gfa1_file_to_gfa() {
        // works perfectly smooth
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa1_files\\prova.gfa");
        print_gfa_file(file_to_gfa(&file));
    }

    #[test]
    fn gfa1_file_with_parsing_mismatch_to_gfa() {
        // the parser fails to match the Header field and the Containment field
        // the optional field in the segment seems to be left behind and not parsed
        // seems that if there are more than 1 header field this is completely ignored
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa1_files\\check_overlap_test.gfa");
        print_gfa_file(file_to_gfa(&file));
    }

    #[test]
    fn gfa2_file_to_gfa() {
        // the version 2 of the format GFA (GFA2) does not work with the 
        // current library of "gfa"
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa2_files\\example2.gfa");
        print_gfa_file(file_to_gfa(&file));
    }

    #[test]
    fn empty_file_to_gfa() {
        // using a blank document the parser will parse an header 
        // (H    VN:Z:1.0), like if it's the only information present in the file
        // idk if this is the correct behaviour of this version of the library or
        // i've done some mistake in the implementation 
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa2_files\\blankDocument.gfa");
        print_gfa_file(file_to_gfa(&file));
    }

    #[test]
    fn handlegraph_from_right_parsed_file() {
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa1_files\\prova.gfa");
        gfa_to_handlegraph((file_to_gfa(&file)).unwrap());
    }

    #[test]
    fn handlegraph_from_not_correct_parsed_file() {
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa1_files\\compression_test.gfa");
        gfa_to_handlegraph((file_to_gfa(&file)).unwrap());
    }

