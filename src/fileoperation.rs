use gfa2::gfa1::GFA;
use gfa2::gfa2::GFA2;
use handlegraph2::{
    hashgraph::HashGraph,
    mutablehandlegraph::*,
};

use bstr::BString;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Function that save a GFA2 object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// use handle_gfa::fileoperation::*;
/// save_as_gfa2_file(&graph, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_as_gfa2_file(
    graph: &HashGraph,
    path: Option<String>,
) -> Result<(), std::io::Error> {
    use handlegraph2::conversion;

    let path = path.unwrap_or_else(|| String::from(
        "./tests/output_files/default_path/file_gfa2.gfa",
    ));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    let gfa_file: GFA2<BString, ()> = conversion::to_gfa2(&graph);
    file.write_all(format!("{}", gfa_file).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// Function that save a GFA1 object in a file
/// on a specific or default location
/// # Example
/// ```ignore
/// use handle_gfa::fileoperation::*;
/// save_as_gfa1_file(&graph, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_as_gfa1_file(
    graph: &HashGraph,
    path: Option<String>,
) -> Result<(), std::io::Error> {
    use handlegraph2::conversion;

    let path = path.unwrap_or_else(|| String::from(
        "./tests/output_files/default_path/file_gfa1.gfa",
    ));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    let gfa_file: GFA<BString, ()> = conversion::to_gfa(&graph);
    file.write_all(format!("{}", gfa_file).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_save_handlegraph_as_gfa2_file() {
        use handlegraph2::{
            handle::Edge, hashgraph::HashGraph,
            mutablehandlegraph::MutableHandleGraph, pathgraph::PathHandleGraph,
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

        // save file on a specific path
        match save_as_gfa2_file(
            &graph,
            Some(String::from("./tests/output_files/file_gfa2.gfa")),
        ) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_save_handlegraph_as_gfa2_file_default_path() {
        use handlegraph2::{
            handle::Edge, hashgraph::HashGraph,
            mutablehandlegraph::MutableHandleGraph, pathgraph::PathHandleGraph,
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

        // save file on a default path
        match save_as_gfa2_file(&graph, None) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_save_handlegraph_as_gfa1_file() {
        use handlegraph2::{
            handle::Edge, hashgraph::HashGraph,
            mutablehandlegraph::MutableHandleGraph, pathgraph::PathHandleGraph,
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

        // save file on a specific path
        match save_as_gfa1_file(
            &graph,
            Some(String::from("./tests/output_files/file_gfa1.gfa")),
        ) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_save_handlegraph_as_gfa1_file_default_path() {
        use handlegraph2::{
            handle::Edge, hashgraph::HashGraph,
            mutablehandlegraph::MutableHandleGraph, pathgraph::PathHandleGraph,
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

        // save file on a default path
        match save_as_gfa1_file(&graph, None) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_use_file_gfa2_saved() {
        use gfa2::{parser_gfa2::GFA2Parser, tag::OptionalFields};

        let parser: GFA2Parser<bstr::BString, OptionalFields> =
            GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> = parser
            .parse_file("./tests/output_files/file_gfa2.gfa")
            .unwrap();

        println!("{}", gfa2);
    }

    #[test]
    fn can_use_file_gfa1_saved() {
        use gfa2::{parser_gfa1::GFAParser, tag::OptionalFields};

        let parser: GFAParser<bstr::BString, OptionalFields> = GFAParser::new();
        let gfa: GFA<BString, OptionalFields> = parser
            .parse_file("./tests/output_files/file_gfa1.gfa")
            .unwrap();

        println!("{}", gfa);
    }
}
