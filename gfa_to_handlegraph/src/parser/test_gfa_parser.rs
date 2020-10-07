use super::*;

    #[test]
    fn gfa1_file_to_gfa() {
        // seems that certain elements of the GFA1 format (H, C) are not parsed
        let mut file = PathBuf::new();
        file.push("test\\gfas\\gfa1_files\\compression_test.gfa");
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