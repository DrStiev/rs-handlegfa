/// insert the REAL tests here
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::parser::*;

    #[test]
    fn can_parse_gfa2_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\big.gfa"));

        match gfa {
            None => panic!("Error parsing GFA file"),
            Some(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group = g.groups.len();

                assert_eq!(num_head, 1);
                assert_eq!(num_segs, 64);
                assert_eq!(num_fragment, 0);
                assert_eq!(num_edge, 71);
                assert_eq!(num_gap, 0);
                assert_eq!(num_group, 0);

                println!("{:#?}", g);
            }
        }
    }
}