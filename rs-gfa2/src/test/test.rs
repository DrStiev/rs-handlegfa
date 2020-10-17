/// insert the REAL tests here
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn can_parse_gfa2_file() {
        let gfa = crate::parser_gfa2::parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\irl.gfa"));

        match gfa {
            Err(why) => println!("{}", why), 
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group_o = g.groups_o.len();
                let num_group_u = g.groups_u.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 3);
                assert_eq!(num_fragment, 0);
                assert_eq!(num_edge, 4);
                assert_eq!(num_gap, 0);
                assert_eq!(num_group_o, 4);
                assert_eq!(num_group_u, 0);
            }
        }
    }

    #[test]
    fn can_parse_gfa2_file_with_comments() {
        let gfa = crate::parser_gfa2::parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\sample2.gfa"));

        match gfa {
            Err(why) => println!("{}", why),
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group_o = g.groups_o.len();
                let num_group_u = g.groups_u.len();
                let num_comments = g.comments.len();

                assert_eq!(num_head, 4);
                assert_eq!(num_segs, 9);
                assert_eq!(num_fragment, 2);
                assert_eq!(num_edge, 6);
                assert_eq!(num_gap, 2);
                assert_eq!(num_group_o, 2);
                assert_eq!(num_group_u, 2);
                assert_eq!(num_comments, 3);
            }
        }
    }

    #[test]
    fn can_parse_gfa2_file_with_custom_records() {
        let gfa = crate::parser_gfa2::parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\sample.gfa"));

        match gfa {
            Err(why) => println!("{}", why),
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group_o = g.groups_o.len();
                let num_group_u = g.groups_u.len();
                let num_custom_records = g.custom_record.len();

                assert_eq!(num_head, 1);
                assert_eq!(num_segs, 6);
                assert_eq!(num_fragment, 0);
                assert_eq!(num_edge, 4);
                assert_eq!(num_gap, 0);
                assert_eq!(num_group_o, 0);
                assert_eq!(num_group_u, 0);
                assert_eq!(num_custom_records, 1);
            }
        }
    }


    #[test]
    fn can_parse_big_gfa2_file() {
        let gfa = crate::parser_gfa2::parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\graph_nicernames.gfa"));

        match gfa {
            Err(why) => println!("{}", why),
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_fragment = g.fragments.len();
                let num_edge = g.edges.len();
                let num_gap = g.gaps.len();
                let num_group_o = g.groups_o.len();
                let num_group_u = g.groups_u.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 61);
                assert_eq!(num_fragment, 11);
                assert_eq!(num_edge, 84);
                assert_eq!(num_gap, 2);
                assert_eq!(num_group_o, 2);
                assert_eq!(num_group_u, 2);
            }
        }
    }

    /*
    #[test]
    fn can_parse_gfa1_file() {
        let gfa = crate::parser_gfa::parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\prova.gfa"));

        match gfa {
            Err(why) => println!("{}", why), 
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_links = g.links.len();
                let num_paths = g.paths.len();
                let num_conts = g.containments.len();

                assert_eq!(num_head, 1);
                assert_eq!(num_segs, 19);
                assert_eq!(num_links, 26);
                assert_eq!(num_conts, 0);
                assert_eq!(num_paths, 3);
            }
        }
    }

    #[test]
    fn can_parse_gfa1_file_with_comments() {
        let gfa = crate::parser_gfa::parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\lil.gfa"));

        match gfa {
            Err(why) => println!("{}", why),
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_links = g.links.len();
                let num_paths = g.paths.len();
                let num_conts = g.containments.len();
                let num_comments = g.comments.len();

                assert_eq!(num_head, 1);
                assert_eq!(num_segs, 15);
                assert_eq!(num_links, 20);
                assert_eq!(num_conts, 0);
                assert_eq!(num_paths, 3);
                assert_eq!(num_comments, 1);
            }
        }
    }

    #[test]
    fn can_parse_big_gfa1_file() {
        let gfa = crate::parser_gfa::parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\very_large_file.gfa"));

        match gfa {
            Err(why) => println!("{}", why),
            Ok(g) => {
                let num_head = g.headers.len();
                let num_segs = g.segments.len();
                let num_links = g.links.len();
                let num_paths = g.paths.len();
                let num_conts = g.containments.len();

                assert_eq!(num_head, 0);
                assert_eq!(num_segs, 160);
                assert_eq!(num_links, 0);
                assert_eq!(num_conts, 0);
                assert_eq!(num_paths, 0);
            }
        }
    }

    #[test]
    fn error_parse_gfa2_file() {
        let res = crate::parser_gfa::parse_gfa(&PathBuf::from("test\\gfas\\gfa2_files\\big.gfa"));
        
        match res {
            Err(why) => println!("{}", why),
            Ok(res) => println!("{}", res),
        }
    }
    */

    #[test]
    fn error_parse_gfa1_file() {
        let res = crate::parser_gfa2::parse_gfa(&PathBuf::from("test\\gfas\\gfa1_files\\check_overlap_test_no_fasta.gfa"));
        match res {
            Err(why) => println!("{}", why),
            Ok(res) => println!("{}", res),
        }
    }
}
