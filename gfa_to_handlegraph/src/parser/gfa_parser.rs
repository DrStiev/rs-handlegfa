// import the libraries to parse a file into the GFA(1) format
use gfa::{
    gfa::GFA,
    parser::{GFAParser, ParseError},
    writer::write_gfa,
};
/*
use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::HandleGraph,
    hashgraph::{HashGraph, PathStep},
    mutablehandlegraph::MutableHandleGraph,
    pathgraph::PathHandleGraph,
}; 
*/
// library to handle the path of a file
use std::path::PathBuf;
use bstring::BString;

/*
pub struct GFA<N, T: OptFields> {
    pub header: Header<T>,
    pub segments: Vec<Segment<N, T>>,
    pub links: Vec<Link<N, T>>,
    pub containments: Vec<Containment<N, T>>,
    pub paths: Vec<Path<N, T>>,
}

pub struct GFAParser<N: SegmentId, T: OptFields> { 
    pub header: Header<T>,
    pub segments: Vec<Segment<N, T>>,
    pub links: Vec<Link<N, T>>,
    pub containments: Vec<Containment<N, T>>,
    pub paths: Vec<Path<N, T>>,
 }
*/

// the parser does not work properly, sometimes catches the Header field
// sometimes not, and the conteinment field is never catched
pub fn print_gfa_file(content: Result<GFA<usize, ()>, ParseError>) {
    match content {
        Ok(result) => {
            let mut out = String::new();
            println!("File content after parsing with the function file_to_gfa:\n");
            write_gfa(&result, &mut out);
            println!("{}", out);
        },
        Err(why) => println!("Error: {}", why),
    }
}

/// function that reads a file and return the result converted to the GFA1 format 
/// or return a ParseError
/// i think the GFA2 format it's not implemented yet
pub fn file_to_gfa(path: &PathBuf) -> Result<GFA<usize, ()>, ParseError> {
    // create a new parser using GFAParser::new()
    let parser = GFAParser::new();

    // try to parse the content of the file passed as input
    // if any kind of error will pop up, they will been caught by
    // the Err(why) statement inside the match branch
     let gfa:GFA<usize, ()> = match parser.parse_file(path) {
        Ok(gfa) => gfa,
        Err(why) => return Err(why),
    };

    Ok(gfa)
}

/*
/// function to convert a gfa graph to an handlegraph
pub fn gfa_to_handlegraph(gfa: GFA<usize, ()>) {
    let _graph: HashGraph = HashGraph::from_gfa(&gfa);
    let iter = _graph.handles_iter();

    for node in iter {
        println!("Node: {:#?}", node);
    }

    // println!("Convert a gfa file to an handlegraph:\n{:#?}", _graph);
}
*/