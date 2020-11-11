// manipulate hashgraph
use handlegraph2::{
    handle::{Edge, Handle, NodeId},
    handlegraph::*,
    hashgraph::*,
    mutablehandlegraph::*,
    pathgraph::PathHandleGraph,
};

pub mod error;
pub use self::error::*;

/// Function that reads a ```GFA2``` files passed as input and return its
/// corresponding ```HandleGraph```
pub fn gfa2_to_handlegraph(path: String) -> Result<HashGraph, GraphOperationError> {
    use gfa2::{gfa2::GFA2, parser_gfa2::GFA2Parser};

    let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
    let gfa2: GFA2<usize, ()> = match parser.parse_file(&path) {
        Ok(g) => g,
        Err(why) => return Err(GraphOperationError::FileError(why.to_string())),
    };
    let graph: HashGraph = HashGraph::from_gfa2(&gfa2);

    Ok(graph)
}

/// Function that reads a ```GFA1``` files passed as input and return its
/// corresponding ```HandleGraph```
pub fn gfa1_to_handlegraph(path: String) -> Result<HashGraph, GraphOperationError> {
    use gfa2::{gfa1::GFA, parser_gfa1::GFAParser};

    let parser: GFAParser<usize, ()> = GFAParser::new();
    let gfa: GFA<usize, ()> = match parser.parse_file(&path) {
        Ok(g) => g,
        Err(why) => return Err(GraphOperationError::FileError(why.to_string())),
    };
    let graph: HashGraph = HashGraph::from_gfa(&gfa);

    Ok(graph)
}

/// Function that adds a node in a graph checking if the provided ```NodeId``` already exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.add_node(graph, 14 as u64, Some(b"TEST_NODE_1")).unwrap();
/// ```
pub fn add_node<T: Into<NodeId>>(
    mut graph: HashGraph,
    nodeid: T,
    sequence: Option<&[u8]>,
) -> Result<HashGraph, GraphOperationError> {
    let sequence = sequence.unwrap_or(b"DEFAULT_SEQUENCE");
    let nodeid_temp = nodeid.into();

    for handle in graph.all_handles() {
        let old_seq_id = handle.id();
        if old_seq_id == nodeid_temp {
            return Err(GraphOperationError::IdAlreadyExist(nodeid_temp.to_string()));
        }
    }

    graph.create_handle(sequence, nodeid_temp);
    Ok(graph)
}

/// Function that adds a link between 2 existing ```Nodes``` in a graph.
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.add_link_between_nodes(graph, b"14+", b"15+").unwrap();
/// ```
pub fn add_link_between_nodes(
    mut graph: HashGraph,
    from_node: &[u8],
    to_node: &[u8],
) -> Result<HashGraph, GraphOperationError> {
    use bstr::ByteSlice;
    use gfa2::gfa2::orientation::Orientation;

    let last = from_node.len() - 1;
    let left_id = from_node[..last].to_str().unwrap();

    let sgn: &str = &from_node[last..].to_str().unwrap();
    let left_orient: Orientation = match sgn {
        "+" => Orientation::Forward,
        "-" => Orientation::Backward,
        _ => {
            return Err(GraphOperationError::OrientationNotExists(
                from_node.to_str().unwrap().to_string(),
            ))
        }
    };

    let last = to_node.len() - 1;
    let right_id = to_node[..last].to_str().unwrap();

    let sgn: &str = &to_node[last..].to_str().unwrap();
    let right_orient: Orientation = match sgn {
        "+" => Orientation::Forward,
        "-" => Orientation::Backward,
        _ => {
            return Err(GraphOperationError::OrientationNotExists(
                to_node.to_str().unwrap().to_string(),
            ))
        }
    };

    let right = Handle::new(right_id.parse::<u64>().unwrap(), right_orient);
    let left = Handle::new(left_id.parse::<u64>().unwrap(), left_orient);

    if graph.create_edge(Edge(left, right)) {
        Ok(graph)
    } else {
        Err(GraphOperationError::EdgeNotExist(
            format!("{}{}", from_node.to_str().unwrap(), left_orient),
            format!("{}{}", to_node.to_str().unwrap(), right_orient),
        ))
    }
}

/// Function that adds a path to read the node of a graph
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// let ids: Vec<&[u8]> = vec![b"11+", b"13+"];
///
/// match add_path(graph, Some(b"TEST_PATH_1"), ids) {
///     Ok(g) => {
///         let mut x = 0;
///         while !g.get_path(&x).is_none() {
///             g.print_path(&x);
///             x += 1;
///         }
///     },
///     Err(why) => println!("Error: {}", why),
/// };
/// ```
pub fn add_path(
    mut graph: HashGraph,
    path_id: Option<&[u8]>,
    sequence_of_id: Vec<&[u8]>,
) -> Result<HashGraph, GraphOperationError> {
    use bstr::ByteSlice;
    use gfa2::gfa2::orientation::Orientation;

    let path_id = path_id.unwrap_or(b"default_path_id");
    // check if the path it's circular
    let last = sequence_of_id.len() - 1;
    let is_circular: bool = sequence_of_id[0] == sequence_of_id[last];

    // create the path
    let path = graph.create_path_handle(path_id, is_circular);
    for seq in sequence_of_id.iter() {
        let last = seq.len() - 1;
        let seq_id = seq[..last].to_str().unwrap();

        let sgn: &str = &seq[last..].to_str().unwrap();
        let orient: Orientation = match sgn {
            "+" => Orientation::Forward,
            "-" => Orientation::Backward,
            _ => {
                return Err(GraphOperationError::OrientationNotExists(
                    seq.to_str().unwrap().to_string(),
                ))
            }
        };

        let handle = Handle::new(seq_id.parse::<u64>().unwrap(), orient);
        graph.append_step(&path, handle);
    }

    Ok(graph)
}

/// Function that removes a node in a graph checking if the provided ```NodeId``` exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.remove_node(graph, 14 as u64).unwrap();
/// ```
pub fn remove_node<T: Into<NodeId>>(
    mut graph: HashGraph,
    nodeid: T,
) -> Result<HashGraph, GraphOperationError> {
    let node = nodeid.into();
    if graph.remove_handle(node) {
        Ok(graph)
    } else {
        Err(GraphOperationError::NodesNotExist(
            node.to_string(),
            "".to_string(),
        ))
    }
}

/// Function that removes a link in a graph checking if the provided ```NodeId``` exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.remove_link(graph, b"14+", b"15+").unwrap();
/// ```
pub fn remove_link(
    mut graph: HashGraph,
    from_node: &[u8],
    to_node: &[u8],
) -> Result<HashGraph, GraphOperationError> {
    use bstr::ByteSlice;
    use gfa2::gfa2::orientation::Orientation;

    let last = from_node.len() - 1;
    let left_id = from_node[..last].to_str().unwrap();

    let sgn: &str = &from_node[last..].to_str().unwrap();
    let left_orient: Orientation = match sgn {
        "+" => Orientation::Forward,
        "-" => Orientation::Backward,
        _ => {
            return Err(GraphOperationError::OrientationNotExists(
                from_node.to_str().unwrap().to_string(),
            ))
        }
    };

    let last = to_node.len() - 1;
    let right_id = to_node[..last].to_str().unwrap();

    let sgn: &str = &to_node[last..].to_str().unwrap();
    let right_orient: Orientation = match sgn {
        "+" => Orientation::Forward,
        "-" => Orientation::Backward,
        _ => {
            return Err(GraphOperationError::OrientationNotExists(
                to_node.to_str().unwrap().to_string(),
            ))
        }
    };

    let right = Handle::new(right_id.parse::<u64>().unwrap(), right_orient);
    let left = Handle::new(left_id.parse::<u64>().unwrap(), left_orient);

    if graph.create_edge(Edge(left, right)) {
        Ok(graph)
    } else {
        Err(GraphOperationError::EdgeNotExist(
            format!("{}{}", from_node.to_str().unwrap(), left_orient),
            format!("{}{}", to_node.to_str().unwrap(), right_orient),
        ))
    }
}

/// Function that removes a path in a graph checking if the provided ```PathName``` exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.remove_path(graph, Some(&BString::from("14")).unwrap();
/// ```
pub fn remove_path(
    mut graph: HashGraph,
    path_name: Option<&[u8]>,
) -> Result<HashGraph, GraphOperationError> {
    let path_name = path_name.unwrap_or(b"default_path_id");
    if graph.remove_path(path_name) {
        Ok(graph)
    } else {
        Err(GraphOperationError::PathNotExist(
            String::from_utf8(path_name.to_vec()).expect("Invalid UTF-8 character"),
        ))
    }
}

/// Function that modifiws a node in a graph checking if the provided ```NodeId``` exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.modify_node(graph, 14 as u64, b"NEW_SEQUENCE").unwrap();
/// ```
pub fn modify_node<T: Into<NodeId>>(
    mut graph: HashGraph,
    nodeid: T,
    sequence: &[u8],
) -> Result<HashGraph, GraphOperationError> {
    let node = nodeid.into();

    if graph.modify_handle(node, sequence) {
        Ok(graph)
    } else {
        Err(GraphOperationError::NodesNotExist(
            node.to_string(),
            "".to_string(),
        ))
    }
}

/// Function that modifies a link in a graph checking if the provided ```NodeId``` exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.modify_link(graph, b"14+", b"15+", b"14+", b"17-").unwrap();
/// ```
pub fn modify_link(
    mut graph: HashGraph,
    from_node: &[u8],
    to_node: &[u8],
    new_from_node: Option<&[u8]>,
    new_to_node: Option<&[u8]>,
) -> Result<HashGraph, GraphOperationError> {
    use bstr::ByteSlice;
    use gfa2::gfa2::orientation::Orientation;

    let last = from_node.len() - 1;
    let old_left_id = from_node[..last].to_str().unwrap();

    let sgn: &str = &from_node[last..].to_str().unwrap();
    let old_left_orient: Orientation = match sgn {
        "+" => Orientation::Forward,
        "-" => Orientation::Backward,
        _ => {
            return Err(GraphOperationError::OrientationNotExists(
                from_node.to_str().unwrap().to_string(),
            ))
        }
    };

    let last = to_node.len() - 1;
    let old_right_id = to_node[..last].to_str().unwrap();

    let sgn: &str = &to_node[last..].to_str().unwrap();
    let old_right_orient: Orientation = match sgn {
        "+" => Orientation::Forward,
        "-" => Orientation::Backward,
        _ => {
            return Err(GraphOperationError::OrientationNotExists(
                to_node.to_str().unwrap().to_string(),
            ))
        }
    };

    let old_right = Handle::new(old_right_id.parse::<u64>().unwrap(), old_right_orient);
    let old_left = Handle::new(old_left_id.parse::<u64>().unwrap(), old_left_orient);

    let new_right = match new_to_node {
        Some(id) => {
            let last = id.len() - 1;
            let new_right_id = id[..last].to_str().unwrap();

            let sgn: &str = &id[last..].to_str().unwrap();
            let new_right_orient: Orientation = match sgn {
                "+" => Orientation::Forward,
                "-" => Orientation::Backward,
                _ => {
                    return Err(GraphOperationError::OrientationNotExists(
                        id.to_str().unwrap().to_string(),
                    ))
                }
            };
            Some(Handle::new(
                new_right_id.parse::<u64>().unwrap(),
                new_right_orient,
            ))
        }
        None => Some(old_right),
    };

    let new_left = match new_from_node {
        Some(id) => {
            let last = id.len() - 1;
            let new_left_id = id[..last].to_str().unwrap();

            let sgn: &str = &id[last..].to_str().unwrap();
            let new_left_orient: Orientation = match sgn {
                "+" => Orientation::Forward,
                "-" => Orientation::Backward,
                _ => {
                    return Err(GraphOperationError::OrientationNotExists(
                        id.to_str().unwrap().to_string(),
                    ))
                }
            };
            Some(Handle::new(
                new_left_id.parse::<u64>().unwrap(),
                new_left_orient,
            ))
        }
        None => Some(old_left),
    };

    if graph.modify_edge(Edge(old_left, old_right), new_left, new_right) {
        Ok(graph)
    } else {
        Err(GraphOperationError::EdgeNotExist(
            format!("{}{}", old_left.id().to_string(), old_left_orient),
            format!("{}{}", old_right.id().to_string(), old_right_orient),
        ))
    }
}

/// Function that modifies a path in a graph checking if the provided ```PathName``` exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
///
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = graph.modify_path(graph, b"14", vec![b"11+", b"12-"]).unwrap();
/// ```
pub fn modify_path(
    mut graph: HashGraph,
    path_name: &[u8],
    sequence_of_id: Vec<&[u8]>,
) -> Result<HashGraph, GraphOperationError> {
    use bstr::ByteSlice;
    use gfa2::gfa2::orientation::Orientation;

    let path_name = path_name;
    let mut handles: Vec<Handle> = vec![];

    for seq in sequence_of_id.iter() {
        let last = seq.len() - 1;
        let seq_id = seq[..last].to_str().unwrap();

        let sgn: &str = &seq[last..].to_str().unwrap();
        let orient: Orientation = match sgn {
            "+" => Orientation::Forward,
            "-" => Orientation::Backward,
            _ => {
                return Err(GraphOperationError::OrientationNotExists(
                    seq.to_str().unwrap().to_string(),
                ))
            }
        };

        let handle = Handle::new(seq_id.parse::<u64>().unwrap(), orient);
        handles.push(handle);
    }
    if graph.modify_path(path_name, handles) {
        Ok(graph)
    } else {
        Err(GraphOperationError::PathNotExist(
            String::from_utf8(path_name.to_vec()).expect("Invalid UTF-8 character"),
        ))
    }
}

/// Print an HashGraph object in a simplified way
/// # Example
/// ```ignore
/// print_simple_graph(&hashgraph);
/// /*
/// Graph: {
///     Nodes: {
///         13: CTTGATT
///         12: TCAAGG
///         11: ACCTT
///     }
///     Edges: {
///         12- --> 13+
///         11+ --> 12-
///         11+ --> 13+
///     }
///     Paths: {
///         14: ACCTT -> CTTGATT
///         15: ACCTT -> CCTTGA -(TCAAGG) -> CTTGATT
///     }
/// }
/// */
/// ```
pub fn print_simple_graph(graph: &HashGraph) {
    graph.print_graph();
}

// TODO: print the graph as a DeBrujin one in a graphical way
/// Print an HashGraph object as a DeBrujin graph (more or less)
/// () -> (1) -> AATTCG -> (2) -> CTTGGA -> (3) -> GAACTG -> ()
///         \                               ^    
///          -------------> AGGTCAG -------/
pub fn print_debrujin_graph(_graph: &HashGraph) {}

#[cfg(test)]
mod tests {
    use super::*;
    use gfa2::{gfa2::GFA2, parser_gfa2::GFA2Parser};

    #[test]
    fn readme_file_test() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser
            .parse_file("./tests/gfa2_files/irl.gfa2")
            .unwrap();
        println!("{:#?}", gfa2);
        println!("{}", gfa2);
        let graph = HashGraph::from_gfa2(&gfa2);
        println!("{:#?}", graph);
        print_simple_graph(&graph);
    }

    #[test]
    fn can_print_graph() {
        use gfa2::{gfa1::GFA, parser_gfa1::GFAParser};
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser
            .parse_file("./tests/gfa2_files/spec_q7.gfa2")
            .unwrap();
        let graph = HashGraph::from_gfa2(&gfa2);
        print_simple_graph(&graph);

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa: GFA<usize, ()> = parser
            .parse_file("./tests/gfa1_files/lil.gfa")
            .unwrap();
        let graph = HashGraph::from_gfa(&gfa);
        print_simple_graph(&graph);
    }

    // about 20 seconds
    #[test]
    fn moddable_medium_graph() {
        println!("Parse and create graph");
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/big_files/test.gfa2").unwrap();
        let mut graph = HashGraph::from_gfa2(&gfa2);

        // remove nodes, edges and paths
        println!("Remove 1000 nodes");
        for i in 1..1001 {
            match remove_node(graph.clone(), i as u64) {
                Ok(g) => graph = g,
                Err(why) => println!("Error: {}", why),
            };
        }
        const PATHS: [&[u8]; 3] = [
            b"gi|568815592:32578768-32589835",
            b"gi|568815529:3998044-4011446",
            b"gi|568815551:3814534-3830133",
        ];
        println!("Remove 3 paths");
        for i in 1..PATHS.len() {
            let path_name: &[u8] = PATHS.get(i as usize).unwrap();
            match remove_path(graph.clone(), Some(path_name)) {
                Ok(g) => graph = g,
                Err(why) => println!("Error: {}", why),
            };
        }
        println!("Remove 5 edges");
        match remove_link(graph.clone(), b"2138-", b"2137-") {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };
        match remove_link(graph.clone(), b"2139+", b"2140+") {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };
        match remove_link(graph.clone(), b"2139+", b"3090+") {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };
        match remove_link(graph.clone(), b"2139-", b"2138-") {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };
        match remove_link(graph.clone(), b"2140+", b"2141+") {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };

        println!("Add 10 paths and edges");
        // add nodes, edges and paths
        let paths: Vec<&[u8]> = vec![
            b"5001+", b"5002+", b"5003-", b"5004+", b"5005-", b"5006-", b"5007+", b"5008+",
            b"5009+", b"5010-",
        ];
        for i in 1..11 {
            match add_node(graph.clone(), 5000 + i as u64, None) {
                Ok(g) => graph = g,
                Err(why) => println!("Error: {}", why),
            };
            if i > 1 {
                match add_link_between_nodes(
                    graph.clone(),
                    format!("{}{}", 4000 + i - 1 as u64, "+".to_string()).as_bytes(),
                    format!("{}{}", 4000 + i as u64, "+".to_string()).as_bytes(),
                ) {
                    Ok(g) => graph = g,
                    Err(why) => println!("Error: {}", why),
                };
            }
        }
        match add_path(graph.clone(), None, paths.clone()) {
            Ok(_) => (),
            Err(why) => println!("Error: {}", why),
        };
        // modify nodes, edges and paths

        //print_simple_graph(&graph);
    }

    #[test]
    fn can_modify_node() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph);
                match modify_node(graph, 11 as u64, b"NEW_TEST_SEQUENCE") {
                    Ok(g) => print_simple_graph(&g),
                    Err(why) => println!("Error: {}", why),
                };
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_modify_edge() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph);
                match modify_link(graph, b"11+", b"13+", Some(b"13+"), Some(b"11+")) {
                    Ok(g) => print_simple_graph(&g),
                    Err(why) => println!("Error: {}", why),
                };
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_modify_path() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph);
                //let smaller path = "11+ 12-";
                match modify_path(graph, b"14", vec![b"11+", b"12-"]) {
                    Ok(g) => print_simple_graph(&g),
                    Err(why) => println!("Error: {}", why),
                };
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_remove_node() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph);
                match remove_node(graph, 11 as u64) {
                    Ok(g) => print_simple_graph(&g),
                    Err(why) => println!("Error: {}", why),
                };
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_remove_edge() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph);
                match remove_link(graph, b"12-", b"13+") {
                    Ok(g) => print_simple_graph(&g),
                    Err(why) => println!("Error: {}", why),
                };
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_remove_path() {
        use bstr::BString;

        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph);
                match remove_path(graph, Some(&BString::from("14"))) {
                    Ok(g) => print_simple_graph(&g),
                    Err(why) => println!("Error: {}", why),
                };
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_convert_file_to_handlegraph() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()) {
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph)
            }
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_node() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser
            .parse_file("./tests/gfa2_files/spec_q7.gfa2")
            .unwrap();
        let graph2 = HashGraph::from_gfa2(&gfa2);
        print_simple_graph(&graph2);
        match add_node(graph2, 14 as u64, Some(b"TEST_NODE_1")) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_link() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser
            .parse_file("./tests/gfa2_files/spec_q7.gfa2")
            .unwrap();
        let mut graph = HashGraph::from_gfa2(&gfa2);

        graph = add_node(graph, 14 as u64, Some(b"TEST_NODE_1")).unwrap();
        graph = add_node(graph, 15 as u64, Some(b"TEST_NODE_2")).unwrap();
        print_simple_graph(&graph);
        match add_link_between_nodes(graph, b"14+", b"15+") {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_path() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser
            .parse_file("./tests/gfa2_files/spec_q7.gfa2")
            .unwrap();
        let graph = HashGraph::from_gfa2(&gfa2);
        let ids: Vec<&[u8]> = vec![b"11+", b"13+"];
        print_simple_graph(&graph);
        match add_path(graph, Some(b"TEST_PATH_1"), ids) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }
}
