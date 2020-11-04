pub mod fileoperation;
pub mod graphoperation;

pub use fileoperation::*;
pub use graphoperation::*;

#[macro_use]
extern crate clap;
use handlegraph2::hashgraph::HashGraph;

const TEXT_MESSAGE: &str = "The possible operation on a graph are:\n\
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
[SEQUENCE|*] is the new sequence of the node. The character \"*\" represent that the sequence it's not provided.\n\
The 2 elements MUST BE separated by a SINGLE whitespace.\n";
const ADD_LINK_MESSAGE: &str = "To ADD a LINK (or EDGE) into the graph, please type [FROM NODEID] [TO NODEID] where:\n\
[FROM NODEID] is the id of the node where the link starts\n\
[TO NODEID] is the id of the node where the link ends.\n\
The 2 elements MUST BE separated by a SINGLE whitespace.\n";
const ADD_PATH_MESSAGE: &str = "To ADD a PATH into the graph, please type [PATH_ID|*] [NODEID(+-)] where:\n\
[PATH_ID|*] is the id of the new path, the character \"*\" represent that the id it's not provided \n\
[NODEID(+-)] is the id of the node(s) with explicit orientation.\
This section can contain 1 or more nodeids, every one of them must be separated by a WHITESPACE.\n\
The 2 elements MUST BE separated by a SINGLE whitespace.\n";

/*
const MODIFY_MESSAGE: &str = "To MODIFY an element to the graph (or an operation) type: 
MODIFY [NODE|LINK|PATH] (case insensitive)";

const REMOVE_MESSAGE: &str = "To REMOVE an element to the graph (or an operation) type: 
REMOVE [NODE|LINK|PATH] (case insensitive)";
*/

fn operation(mut graph: HashGraph, display_file: bool) -> HashGraph {
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
            "STOP" => stop = true,
            "ADD NODE" => {
                // TODO: add control for empty or blank id
                println!("\n{}", ADD_NODE_MESSAGE);
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
                            let sequence: Option<&[u8]> = if iter_ == "*" {
                                None
                            } else {
                                Some(iter_.as_bytes())
                            };

                            match add_node(graph.clone(), id, sequence) {
                                Ok(g) => {
                                    graph = g.clone();
                                    if display_file {
                                        println!();
                                        print_simple_graph(&g);
                                    } else { 
                                        println!("The file it's too big to being displayed");
                                    }
                                },
                                Err(why) => println!("Error: {}", why),
                            }
                        }
                    }
                }
            },
            "ADD LINK" => {
                // TODO: add control for empty or blank id
                println!("\n{}", ADD_LINK_MESSAGE);
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

                            match add_link_between_nodes(graph.clone(), id_from, id_to) {
                                Ok(g) => {
                                    graph = g.clone();
                                    if display_file {
                                        println!();
                                        print_simple_graph(&g);
                                    } else { 
                                        println!("The file it's too big to being displayed");
                                    }
                                },
                                Err(why) => println!("Error: {}", why),
                            }
                        }
                    }
                }
            },
            "ADD PATH" => {
                // TODO: add control for empty or blank id
                println!("\n{}", ADD_PATH_MESSAGE);
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

                            let path_id: Option<&[u8]> = if iter[0] == "*" {
                                None
                            } else {
                                Some(iter[0].as_bytes())
                            };
                            while x < len {
                                ids.push(iter[x].as_bytes());
                                x += 1;
                            }

                            match add_path(graph.clone(), path_id, ids) {
                                Ok(g) => {
                                    graph = g.clone();
                                    if display_file {
                                        println!();
                                        print_simple_graph(&g);
                                    } else { 
                                        println!("The file it's too big to being displayed");
                                    }
                                },
                                Err(why) => println!("Error: {}", why),
                            }
                        }
                    }
                }
            },
            _ => println!("No operation with the command: {}", input),
        }
    }
    graph
}

fn save(graph: HashGraph, format: &str, file: &str) {
    use std::io;

    println!("Do you want to save the changes?");
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("Failed to read input");
    match result.to_uppercase().as_str().trim() {
        "YES"|"Y" => {
                println!("Specify the path where to save the file or the input file will be overwritten.\n\
                \"*\" is the character to use to not specify any path");
                let mut path = String::new();
                io::stdin().read_line(&mut path).expect("Failed to read input");
                match path.trim() {
                    "*" => {
                        if format == "GFA1" {
                            match save_as_gfa1_file(&graph, Some(String::from(file))){
                                Ok(_) => println!("File saved!"),
                                Err(why) => println!("Error: {}", why),
                            };
                        } else {
                            match save_as_gfa2_file(&graph, Some(String::from(file))){
                                Ok(_) => println!("File saved!"),
                                Err(why) => println!("Error: {}", why),
                            };
                        }  
                    }
                    " " => {
                        if format == "GFA1" {
                            match save_as_gfa1_file(&graph, None){
                                Ok(_) => println!("File saved!"),
                                Err(why) => println!("Error: {}", why),
                            };
                        } else {
                            match save_as_gfa2_file(&graph, None){
                                Ok(_) => println!("File saved!"),
                                Err(why) => println!("Error: {}", why),
                            };
                        }
                    },
                    _ => {
                        if format == "GFA1" {
                            match save_as_gfa1_file(&graph, Some(String::from(path.trim()))){
                                Ok(_) => println!("File saved!"),
                                Err(why) => println!("Error: {}", why),
                            };
                        } else {
                            match save_as_gfa2_file(&graph, Some(String::from(path.trim()))){
                                Ok(_) => println!("File saved!"),
                                Err(why) => println!("Error: {}", why),
                            };
                        }
                    }
                }
            }, 
        "NO"|"N" => println!("File not saved!\nProgram terminated correctly!"),
        _ => println!("Command not recognized!\nProgram terminated and file not saved!"),
    }
}

fn main() {
    use std::fs;

    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Matteo Stievano <m.stievano1@campus.unimib.it>")
        (about: "This program allow the user to make various operation on a GFA2 file 
        using instead of a file representation a graph representation.")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg FORMAT: +required "Sets the the format for the input file to use")
    ).get_matches();

    let file = matches.value_of("INPUT").unwrap();
    let display_file: bool = if fs::metadata(file.clone()).unwrap().len() < 10_000 {
        true
    } else {
        false
    };
    match matches.value_of("FORMAT").unwrap().to_string().to_uppercase().as_str(){
        "GFA1" => {
            match gfa1_to_handlegraph(file.to_string()) {
                Ok(g) => {
                    let mut graph: HashGraph = g;
                    if display_file {
                        println!();
                        print_simple_graph(&graph); 
                    } else { 
                        println!("The file it's too big to being displayed");
                    }
                    graph = operation(graph, display_file);
                    save(graph, "GFA1", file)
                },
                Err(why) => println!("Error: {}", why),
            }
        },
        "GFA2" => {
            match gfa2_to_handlegraph(file.to_string()) {
                Ok(g) => {
                    let mut graph: HashGraph = g;
                    if display_file {
                        println!();
                        print_simple_graph(&graph); 
                    } else { 
                        println!("The file it's too big to being displayed");
                    }
                    graph = operation(graph, display_file);
                    save(graph, "GFA2", file)
                },
                Err(why) => println!("Error: {}", why),
            }
        },
        _ => println!("Error! Format not recognized!"),
    };  
}