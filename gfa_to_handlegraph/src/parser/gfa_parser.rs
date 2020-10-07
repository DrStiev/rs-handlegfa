// import the libraries to parse a file into the GFA(1) format
use gfa::gfa::*;
use gfa::parser::*;
use gfa::writer::*;

// library to handle the path of a file
use std::path::PathBuf;

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

pub fn print_gfa_file(content: Result<String, ParseError>) {
    match content {
        Ok(result) => println!("File content:\n{}", result),
        Err(why) => println!("Error: {}", why),
    }
}

/// function that reads a file and return the result converted to the GFA1 format 
/// or return a ParseError
/// i think the GFA2 format it's not implemented yet
pub fn file_to_gfa(path: &PathBuf) -> Result<String, ParseError> {
    // create a new parser using GFAParser::new()
    let parser = GFAParser::new();

    // try to parse the content of the file passed as input
    // if any kind of error will pop up, they will been caught by
    // the Err(why) statement inside the match branch
     let result = match parser.parse_file(path) {
        Ok(result) => result,
        Err(why) => return Err(why),
    };
   Ok(gfa_string(&result))
}