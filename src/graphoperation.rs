// manipulate hashgraph
use handlegraph2::{
    handle::{Handle, NodeId, Edge},
    hashgraph::HashGraph,
    mutablehandlegraph::*,
    pathgraph::PathHandleGraph,
    handlegraph::*,
};

pub mod error;
pub use self::error::*;

/// Function that reads a ```GFA2``` files passed as input and return its
/// corresponding ```HandleGraph```
pub fn gfa2_to_handlegraph(path: String) -> Result<HashGraph, GraphOperationError> {
    use gfa2::{
        parser_gfa2::GFA2Parser,  
        gfa2::GFA2,
    };

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
    use gfa2::{
        parser_gfa1::GFAParser,  
        gfa1::GFA,
    };

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
/// graph = add_node(graph, 14 as u64, Some(b"TEST_NODE_1")).unwrap();
/// ```
pub fn add_node<T: Into<NodeId>>(
    mut graph: HashGraph, 
    nodeid: T,
    sequence: Option<&[u8]>, 
) -> Result<HashGraph, GraphOperationError> {
    let sequence = sequence.unwrap_or(b"");
    let nodeid_temp = nodeid.into();

    for handle in graph.all_handles() {
        let old_seq_id = handle.id();
        if old_seq_id == nodeid_temp {
            return Err(GraphOperationError::IdAlreadyExist(nodeid_temp.to_string()))
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
/// graph = add_link_between_nodes(graph, 14 as u64, 15 as u64).unwrap();
/// ```
pub fn add_link_between_nodes<T: Into<NodeId>>(
    mut graph: HashGraph, 
    from_node: T,
    to_node: T,
) -> Result<HashGraph, GraphOperationError> {
    use gfa2::gfa2::orientation::Orientation;

    let orient = |rev: bool| {
        if rev {
            Orientation::Backward
        } else {
            Orientation::Forward
        }
    }; 

    let mut find_left: bool = false;
    let mut find_right: bool = false;

    let mut left_orient: Orientation = Orientation::default();
    let mut right_orient: Orientation = Orientation::default();

    let from_node: NodeId = from_node.into();
    let to_node: NodeId = to_node.into();

    // check if the segmentId associated to from_node and to_node exists
    for handle in graph.all_handles() {
        let seq_id = handle.id();
        let rev = handle;

        if from_node == seq_id {
            find_left = true;
            left_orient = orient(rev.is_reverse());
        }
        if to_node == seq_id {
            find_right = true;
            right_orient = orient(rev.is_reverse());
        } 
        if find_left && find_right {
            break;
        }
    }

    // panic even if one segment id did not exist
    if !find_left && !find_right {
        return Err(GraphOperationError::NodesNotExist(vec![from_node.to_string(), to_node.to_string()]))
    } else if !find_left {
        return Err(GraphOperationError::NodesNotExist(vec![from_node.to_string(), "".to_string()]))
    } else if !find_right {
        return Err(GraphOperationError::NodesNotExist(vec!["".to_string(), to_node.to_string()]))
    }

    // if everything ru smooth then create 2 new Handle object
    // and then, add a new edge to the graph
    let left = Handle::new(from_node, left_orient);
    let right = Handle::new(to_node, right_orient);

    graph.create_edge(Edge(left, right));
    Ok(graph)
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
    use gfa2::gfa2::orientation::Orientation;
    use bstr::ByteSlice;

    let path_id = path_id.unwrap_or(b"default_path_id");
    // check if the path it's circular
    let last = sequence_of_id.len()-1;
    let is_circular: bool = if sequence_of_id[0] == sequence_of_id[last] {
        true
    } else {
        false
    };

    // create the path
    let path = graph.create_path_handle(path_id, is_circular);
    for seq in sequence_of_id.iter() {
        let last = seq.len()-1;
        let seq_id = seq[..last].to_str().unwrap(); 

        let sgn: &str = &seq[last..].to_str().unwrap();
        let orient: Orientation = match sgn {
            "+" => Orientation::Forward,
            "-" => Orientation::Backward,
            _ => return Err(GraphOperationError::OrientationNotExists(seq.to_str().unwrap().to_string()))
        };

        let handle = Handle::new(seq_id.parse::<u64>().unwrap(), orient);
        graph.append_step(&path, handle);
    }

    Ok(graph)
}

/// Print an HashGraph object in a simplified way
pub fn print_simple_graph(graph: &HashGraph) {
    use bstr::BString;

    println!("Graph : {{");
    // get all the nodeid and sequence associated with them
    for handle in graph.all_handles() {
        let node_id: String = handle.id().to_string();
        let sequence: BString = graph.sequence_iter(handle.forward()).collect();

        println!("\t{} [sequence = {}]", node_id, sequence);
    }

    println!();
    // get all the link (edge) between nodes
    for edge in graph.all_edges() {
        let Edge(left, right) = edge;

        let from_node: String = if !left.id().to_string().is_empty(){
            left.id().to_string()
        } else {
            "NUL".to_string()
        };
        let to_node: String = if !right.id().to_string().is_empty(){
            right.id().to_string()
        } else {
            "NUL".to_string()
        };

        println!("\t{} --> {}", from_node, to_node);
    }

    println!();
    // get all the path
    let mut x :i64 = 0;
    while !graph.get_path(&x).is_none() {
        let path = graph.paths.get(&x).unwrap();
        let mut first: bool = true;

        for (ix, handle) in path.nodes.iter().enumerate() {
            let node = graph.get_node(&handle.id()).unwrap();
            if first {
                first = false;
                print!("\t");
            }
            if ix != 0 {
                print!(" -> ");
            }
            print!("{}", node.sequence);
        }
        println!();
        x += 1;
    } 

    println!("}}");
}

// TODO: print the graph as a DeBrujin one in a graphical way
/// Print an HashGraph object as a DeBrujin graph (more or less)
/// () -> (1) -> AATTCG -> (2) -> CTTGGA -> (3) -> GAACTG -> ()
///         \                               ^    
///          -------------> AGGTCAG -------/
pub fn print_debrujin_graph(graph: &HashGraph) {}

#[cfg(test)]
mod tests {
    use super::*;

    use gfa2::{
        parser_gfa2::GFA2Parser,
        parser_gfa1::GFAParser,  
        gfa2::GFA2,
        gfa1::GFA,
    };

    #[test]
    fn can_print_graph_from_gfa2() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let graph = HashGraph::from_gfa2(&gfa2);

        print_simple_graph(&graph);
    }

    #[test]
    fn can_gfa2_to_handlegraph() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string()){
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph)
            },
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_print_graph_from_gfa1() {
        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let graph = HashGraph::from_gfa(&gfa);

        print_simple_graph(&graph);
    }

    #[test]
    fn can_gfa1_to_handlegraph() {
        match gfa1_to_handlegraph("./tests/gfa1_files/lil.gfa".to_string()){
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph)
            },
            Err(why) => println!("Error: {}", why),
        };
    }


    #[test]
    fn can_gfa2_to_handlegraph_error() {
        match gfa2_to_handlegraph("./tests/gfa2_files/spec_q8.gfa".to_string()){
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph)
            },
            Err(why) => println!("Error: {}", why),
        };

        match gfa2_to_handlegraph("./tests/gfa2_files/GFA1_TEST.gfa".to_string()){
            Ok(g) => {
                let graph: HashGraph = g;
                print_simple_graph(&graph)
            },
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_node() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let graph2 = HashGraph::from_gfa2(&gfa2);

        match add_node(graph2, 14 as u64, Some(b"TEST_NODE_1")) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa1: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let graph1 = HashGraph::from_gfa(&gfa1);

        match add_node(graph1, 20 as u64, Some(b"TEST_NODE_1")) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn error_can_add_node() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let graph = HashGraph::from_gfa2(&gfa2);

        match add_node(graph, 12 as u64, Some(b"TEST_NODE_1")) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa1: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let graph1 = HashGraph::from_gfa(&gfa1);

        match add_node(graph1, 5 as u64, Some(b"TEST_NODE_1")) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_link() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let mut graph = HashGraph::from_gfa2(&gfa2);

        graph = add_node(graph, 14 as u64, Some(b"TEST_NODE_1")).unwrap();
        graph = add_node(graph, 15 as u64, Some(b"TEST_NODE_2")).unwrap();

        match add_link_between_nodes(graph, 14 as u64, 15 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let mut graph1 = HashGraph::from_gfa(&gfa);

        graph1 = add_node(graph1, 20 as u64, Some(b"TEST_NODE_1")).unwrap();
        graph1 = add_node(graph1, 21 as u64, Some(b"TEST_NODE_2")).unwrap();

        match add_link_between_nodes(graph1, 20 as u64, 21 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn error_can_add_link() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let mut graph = HashGraph::from_gfa2(&gfa2);

        graph = add_node(graph, 14 as u64, Some(b"TEST_NODE_1")).unwrap();
        graph = add_node(graph, 15 as u64, Some(b"TEST_NODE_2")).unwrap();

        match add_link_between_nodes(graph.clone(), 17 as u64, 16 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        match add_link_between_nodes(graph.clone(), 18 as u64, 15 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        match add_link_between_nodes(graph.clone(), 14 as u64, 20 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let mut graph1 = HashGraph::from_gfa(&gfa);

        graph1 = add_node(graph1, 20 as u64, Some(b"TEST_NODE_1")).unwrap();
        graph1 = add_node(graph1, 21 as u64, Some(b"TEST_NODE_2")).unwrap();

        match add_link_between_nodes(graph1.clone(), 50 as u64, 51 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        match add_link_between_nodes(graph1.clone(), 51 as u64, 21 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };

        match add_link_between_nodes(graph1.clone(), 20 as u64, 50 as u64) {
            Ok(g) => print_simple_graph(&g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_path() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let graph = HashGraph::from_gfa2(&gfa2);
        let ids: Vec<&[u8]> = vec![b"11+", b"13+"];

        match add_path(graph, Some(b"TEST_PATH_1"), ids) {
            Ok(g) => {
                let mut x = 0;
                while !g.get_path(&x).is_none() {
                    g.print_path(&x);
                    x += 1;
                }
            }, 
            Err(why) => println!("Error: {}", why),
        };

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let graph1 = HashGraph::from_gfa(&gfa);
        let ids: Vec<&[u8]> = vec![b"11+", b"13+"];

        match add_path(graph1, Some(b"TEST_PATH_1"), ids) {
            Ok(g) => {
                let mut x = 0;
                while !g.get_path(&x).is_none() {
                    g.print_path(&x);
                    x += 1;
                }
            }, 
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn error_can_add_path() {
        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();
        let graph = HashGraph::from_gfa2(&gfa2);
        let ids: Vec<&[u8]> = vec![b"11+", b"13"];

        match add_path(graph, Some(b"TEST_PATH_1"), ids) {
            Ok(g) => {
                let mut x = 0;
                while !g.get_path(&x).is_none() {
                    g.print_path(&x);
                    x += 1;
                }
            }, 
            Err(why) => println!("Error: {}", why),
        };

        let parser: GFAParser<usize, ()> = GFAParser::new();
        let gfa: GFA<usize, ()> = parser.parse_file("./tests/gfa1_files/lil.gfa").unwrap();
        let graph1 = HashGraph::from_gfa(&gfa);
        let ids: Vec<&[u8]> = vec![b"11", b"13+"];

        match add_path(graph1, Some(b"TEST_PATH_1"), ids) {
            Ok(g) => {
                let mut x = 0;
                while !g.get_path(&x).is_none() {
                    g.print_path(&x);
                    x += 1;
                }
            }, 
            Err(why) => println!("Error: {}", why),
        };
    }
}