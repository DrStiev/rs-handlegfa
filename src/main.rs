// structure of the program
// GFA2 file as input
// |
// Check if the file it's correct for the format
// |
// Create the HandleGraph
// |
// Allow the user to modify the handlegraph 
// | -> Add nodes, edges and paths
// | -> Modify nodes, edges and paths (WIP)
// | -> Remove nodes, edges and paths (WIP)
// |
// Save the Handlegraph
// | -> as a GFA2 file -> in the same file or in a different one (add a flag to decide)
// | -> as a Handlegraph -> in a different file

pub mod fileoperation;
pub mod graphoperation;

#[macro_use]
extern crate clap;
//use clap::{App, Arg};
use handlegraph2::hashgraph::HashGraph;

const TEXT_MESSAGE: &str = "The operation on a graph are:\n\
1. Add Node(s), Link(s) [or Edge(s)] and Path(s)\n\
2. Modify the value of Node(s), Link(s) [or Edge(s)] and Path(s)\n\
3. Remove Node(s), Link(s) [or Edge(s)] and Path(s)\n\
For now only the first operations are available!"; 

const STOP_MESSAGE: &str = "To STOP modifying the graph, \
or STOP perform a certain operation type [STOP]";

const ADD_MESSAGE: &str = "To ADD an element to the graph (or an operation) type: 
ADD [NODE|LINK|PATH] (case insensitive)";

const ADD_NODE_MESSAGE: &str = "To ADD a NODE into the graph, please type [NODEID] [SEQUENCE|*] where:\n\
[NODEID] is the new id of the node\n\
[SEQUENCE|*] is the new sequence of the node. The character \"*\" represent
that the sequence it's not provided.\n\
The 2 elements MUST BE separated by a SINGLE whitespace.";
const ADD_LINK_MESSAGE: &str = "To ADD a LINK (or EDGE) into the graph, please type [FROM NODEID] [TO NODEID] where:\n\
[FROM NODEID] is the id of the node where the link starts\n\
[TO NODEID] is the id of the node where the link ends.\n\
The 2 elements MUST BE separated by a SINGLE whitespace.";
const ADD_PATH_MESSAGE: &str = "To ADD a PATH into the graph, please type [PATH_ID|*] [NODEID(+-)] where:\n\
[PATH_ID|*] is the id of the new path, the character \"*\" represent
that the id it's not provided \n\
[NODEID(+-)] is the id of the node(s) with explicit orientation.\
This section can contain 1 or more nodeids, every one of them must be \
separated by a WHITESPACE.\n\
The 2 elements MUST BE separated by a SINGLE whitespace.";

/*
const MODIFY_MESSAGE: &str = "To MODIFY an element to the graph (or an operation) type: 
MODIFY [NODE|LINK|PATH] (case insensitive)";

const REMOVE_MESSAGE: &str = "To REMOVE an element to the graph (or an operation) type: 
REMOVE [NODE|LINK|PATH] (case insensitive)";
*/

fn operation(mut graph: HashGraph, file: String) {
    use std::io;
    println!("\n{}\n\n{}\n", TEXT_MESSAGE, STOP_MESSAGE);
    println!("{}\n", ADD_MESSAGE);
    // println!("{}", MODIFY_MESSAGE);
    // println!("{}", REMOVE_MESSAGE);

    let mut stop: bool = false;
    while !stop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        // remember to use .trim()
        match input.to_uppercase().as_str().trim() {
            // TODO: add control for empty or blank id
            "STOP" => stop = true,
            "ADD NODE" => {
                println!("{}", ADD_NODE_MESSAGE);
                let mut stop_: bool = false;
                while !stop_ {
                    let mut operation = String::new();
                    io::stdin().read_line(&mut operation).expect("Failed to read input");
                    match operation.to_uppercase().as_str().trim() {
                        "STOP" => stop_ = true,
                        _ => {
                            let mut iter = operation.split_whitespace();
                            let id: u64 = iter.next().unwrap().parse::<u64>().unwrap();
                            let iter_ = iter.next().unwrap();
                            let sequence: Option<&[u8]> = if iter_ == "*"{
                                None
                            } else {
                                Some(iter_.as_bytes())
                            };

                            graph = graphoperation::add_node(graph.clone(), id, sequence).unwrap();
                            graphoperation::print_simple_graph(&graph);
                        }
                    }
                }
            },
            "ADD LINK" => {
                println!("{}", ADD_LINK_MESSAGE);
                let mut stop_: bool = false;
                while !stop_ {
                    let mut operation = String::new();
                    io::stdin().read_line(&mut operation).expect("Failed to read input");
                    match operation.to_uppercase().as_str().trim() {
                        "STOP" => stop_ = true,
                        _ => {
                            let mut iter = operation.split_whitespace();
                            let id_from: u64 = iter.next().unwrap().parse::<u64>().unwrap();
                            let id_to: u64 = iter.next().unwrap().parse::<u64>().unwrap();

                            graph = graphoperation::add_link_between_nodes(graph.clone(), id_from, id_to).unwrap();
                            graphoperation::print_simple_graph(&graph);
                        }
                    }
                }
            },
            "ADD PATH" => {
                println!("{}", ADD_PATH_MESSAGE);
                let mut stop_: bool = false;
                while !stop_ {
                    let mut operation = String::new();
                    io::stdin().read_line(&mut operation).expect("Failed to read input");
                    match operation.to_uppercase().as_str().trim() {
                        "STOP" => stop_ = true,
                        _ => {
                            let iter: Vec<&str> = operation.split_whitespace().collect();
                            let mut ids: Vec<&[u8]> = vec![];

                            let len: usize = iter.len();
                            let mut x: usize = 1;

                            let path_id: Option<&[u8]> = if iter[0] == "*"{
                                None
                            } else {
                                Some(iter[0].as_bytes())
                            };
                            while x < len {
                                ids.push(iter[x].as_bytes());
                                x += 1;
                            }

                            graph = graphoperation::add_path(graph.clone(), path_id, ids).unwrap();
                            graphoperation::print_simple_graph(&graph);
                        }
                    }
                }
            },
            _ => panic!("No operation with the command: {}", input),
        }
    }

    println!("Do you want to save the changes?");
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("Failed to read input");
    match result.to_uppercase().as_str().trim() {
        "YES"|"Y" => {
                println!("Specify the path where to save the file or it will be overwritten the input file.\n\
                \"*\" is the character to use to not specify any path");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read input");
                match path.trim() {
                    _ => {
                        let path: String = if path == "*" {
                            file
                        } else {
                            path
                        };
                        // FIXME: Error: The filename, directory name, or volume label syntax is incorrect. (os error 123)
                        match fileoperation::save_file(&graph, Some(path)){
                            Ok(_) => println!("File saved!"),
                            Err(why) => println!("Error: {}", why),
                        };
                    }
                }
            }, 
        _ => println!("File not saved!\nProgram terminated correctly!"),
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Matteo Stievano <m.stievano1@campus.unimib.it>")
        (about: "This program allow the user to make various operation on a GFA2 file 
        using instead of a file representation a graph representation.")
        (@arg INPUT: +required "Sets the input file to use")
    ).get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let graph: HashGraph = graphoperation::gfa2_to_handlegraph(file.to_string());
    graphoperation::print_simple_graph(&graph); 

    operation(graph, file.to_string());
}