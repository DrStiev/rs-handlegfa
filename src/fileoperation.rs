use gfa2::{
    gfa2::GFA2,
    tag::OptionalFields,
};

use handlegraph2::hashgraph::*;
use std::io::prelude::*;
use bstr::BString;

/// Function that save an HashGraph object in a file 
/// # Example
/// ```ignore
/// use handle_gfa::fileoperation::*;
/// save_handlegraph(&graph, &"./tests/output_files/handlegraph_to_file.gfa");
/// ```
pub fn save_handlegraph<P: AsRef<std::path::Path>>(
    graph: &HashGraph, 
    filepath: P,
) -> std::io::Result<()> {
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new().write(true).create(true).open(filepath)?;
    file.write_all(format!("{:#?}", graph).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// Function that save a GFA2 object in a file 
/// # Example
/// ```ignore
/// use handle_gfa::fileoperation::*;
/// save_gfa2(&gfa2, &"./tests/output_files/gfa2_to_file.gfa");
/// ```
pub fn save_gfa2<P: AsRef<std::path::Path>>(
    gfa2: &GFA2<BString, OptionalFields>, 
    filepath: P,
) -> std::io::Result<()> {
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new().write(true).create(true).open(filepath)?;
    file.write_all(format!("{}", gfa2).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_save_handlegraph() {
        use handlegraph2::{
            handle::Edge,
            hashgraph::HashGraph,
            mutablehandlegraph::MutableHandleGraph,
            pathgraph::PathHandleGraph,
        };

        let mut graph = HashGraph::new();
        let h1 = graph.create_handle(b"ACCTT", 11);
        let h2 = graph.create_handle(b"TCAAGG", 12);
        let h3 = graph.create_handle(b"CTTGATT", 13);

        // use .flip() to apply reverse complement to the node
        graph.apply_orientation(h2.flip());

        graph.create_edge(Edge(h1, h2));
        graph.create_edge(Edge(h2, h3));
        graph.create_edge(Edge(h1, h3));

        let path = graph.create_path_handle(b"1", false);
        // path: 1 -> 2 -> 3
        graph.append_step(&path, h1);
        graph.append_step(&path, h2);
        graph.append_step(&path, h3);

        match save_handlegraph(&graph, &"./tests/output_files/handlegraph_to_file.gfa") {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why), 
        };
    }

    #[test]
    fn can_save_gfa2() {
        use gfa2::parser_gfa2::GFA2Parser;

        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();

        match save_gfa2(&gfa2, &"./tests/output_files/gfa2_to_file.gfa") {
            Ok(_) => println!("GFA2 saved correctly!"),
            Err(why) => println!("Error: {}", why), 
        };
    }

    #[test]
    fn can_overwritten_output_file() {
        use gfa2::parser_gfa2::GFA2Parser;

        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> = parser.parse_file("./tests/gfa2_files/big.gfa").unwrap();

        match save_gfa2(&gfa2, &"./tests/output_files/gfa2_to_file.gfa") {
            Ok(_) => println!("GFA2 saved correctly!"),
            Err(why) => println!("Error: {}", why), 
        };
    }
}