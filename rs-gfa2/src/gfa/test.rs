/// insert the REAL tests here
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::parser_gfa::*;

    #[test]
    fn can_parse_gfa_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\lil.gfa"));

        match gfa {
            None => panic!("Error parsing GFA file"),
            Some(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_links = g.links.len();
                let num_paths = g.paths.len();
                let num_conts = g.containments.len();

                assert_eq!(num_head, 1);
                assert_eq!(num_segs, 15);
                assert_eq!(num_links, 20);
                assert_eq!(num_conts, 0);
                assert_eq!(num_paths, 3);

                println!("{}", g);
            }
        }
    }

    #[test]
    fn can_parse_very_big_gfa1_file() {
        let gfa = parse_gfa(&PathBuf::from("test\\gfas\\big_file\\A-3105.sort.gfa"));

        match gfa {
            None => panic!("Error parsing GFA file"),
            Some(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_links = g.links.len();
                let num_paths = g.paths.len();
                let num_conts = g.containments.len();

                assert_eq!(num_head, 1);
                assert_eq!(num_segs, 6880);
                assert_eq!(num_links, 10774);
                assert_eq!(num_conts, 0);
                assert_eq!(num_paths, 11);
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
                let num_links = g.links.len();
                let num_paths = g.paths.len();
                let num_conts = g.containments.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 0);
                assert_eq!(num_links, 0);
                assert_eq!(num_conts, 0);
                assert_eq!(num_paths, 0);
            }
        }
    }
}