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
            }
        }
    }

    #[test]
    fn can_parse_big_gfa2_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\big_file\\graph_nicernames.gfa"));

        match gfa {
            None => panic!("Error parsing GFA file"),
            Some(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group = g.groups.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 61);
                assert_eq!(num_fragment, 11);
                assert_eq!(num_edge, 84);
                assert_eq!(num_gap, 2);
                assert_eq!(num_group, 4);
            }
        }
    }

    #[test]
    fn can_parse_gfa1_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\lil.gfa"));

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
                assert_eq!(num_segs, 0);
                assert_eq!(num_fragment, 0);
                assert_eq!(num_edge, 0);
                assert_eq!(num_gap, 0);
                assert_eq!(num_group, 0);
            }
        }
    }

    #[test]
    fn can_parse_very_big_gfa1_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\big_file\\very_big_file.gfa"));

        match gfa {
            None => panic!("Error parsing GFA file"),
            Some(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group = g.groups.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 0);
                assert_eq!(num_fragment, 0);
                assert_eq!(num_edge, 0);
                assert_eq!(num_gap, 0);
                assert_eq!(num_group, 0);
            }
        }
    }

    #[test]
    fn can_parse_blank_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\blankDocument.gfa"));

        match gfa {
            None => panic!("Error parsing GFA file"),
            Some(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group = g.groups.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 0);
                assert_eq!(num_fragment, 0);
                assert_eq!(num_edge, 0);
                assert_eq!(num_gap, 0);
                assert_eq!(num_group, 0);
            }
        }
    }
}