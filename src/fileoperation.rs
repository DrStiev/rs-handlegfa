use gfa2::gfa2::GFA2;

use handlegraph2::hashgraph::*;
use std::io::prelude::*;
use bstr::BString;
use std::fs::File;
use std::path::Path;

/// Function that save a GFA2 object in a file 
/// on a specific or default location 
/// # Example
/// ```ignore
/// use handle_gfa::fileoperation::*;
/// save_file(&graph, Some(String::from("./tests/output_files/gfa2_to_file.gfa")));
/// ```
pub fn save_file(
    graph: &HashGraph,
    path: Option<String>,
) -> std::io::Result<()> {
    use handlegraph2::conversion;

    let path = path.unwrap_or(String::from("./tests/output_files/default_path/file.gfa"));
    let path = Path::new(&path);
    let mut file = File::create(path)?;
    let gfa_file: GFA2<BString, ()> = conversion::to_gfa(&graph);
    file.write_all(format!("{}", gfa_file).as_bytes())?;
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

        // save file on a specific path
        match save_file(&graph, Some(String::from("./tests/output_files/file.gfa"))) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why), 
        };

        // save file on a specific path
        match save_file(&graph, Some(String::from("tests\\output_files\\file.gfa"))) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why), 
        };

        // save file on a default path
        match save_file(&graph, None) {
            Ok(_) => println!("Handlegraph saved correctly!"),
            Err(why) => println!("Error: {}", why), 
        };
    }

    #[test]
    fn can_use_file_saved() {
        use gfa2::{
            parser_gfa2::GFA2Parser,
            tag::OptionalFields,
        };

        let parser: GFA2Parser<bstr::BString, OptionalFields> = GFA2Parser::new();
        let gfa2: GFA2<BString, OptionalFields> = parser.parse_file("./tests/output_files/file.gfa").unwrap();
        
        println!("{}", gfa2);
    }
}