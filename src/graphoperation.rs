// manipulate hashgraph
use handlegraph2::{
    handle::{Handle, NodeId, Edge},
    hashgraph::HashGraph,
    mutablehandlegraph::MutableHandleGraph,
    pathgraph::PathHandleGraph,
    handlegraph::iter::{
        AllHandles,
        HandleSequences,
        AllEdges,
    },
};

//TODO: make a better error handling, now sucks

/// Function that reads a ```GFA2``` files passed as input and return its
/// corresponding ```HandleGraph```
pub fn gfa2_to_handlegraph(path: String) -> HashGraph {
    use gfa2::{
        parser_gfa2::GFA2Parser,  
        gfa2::GFA2,
    };

    let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
    let gfa2: GFA2<usize, ()> = parser.parse_file(&path).unwrap();
    let graph: HashGraph = HashGraph::from_gfa(&gfa2);

    graph
}

/// Function that adds a node in a graph checking if the provided ```NodeId``` already exists
/// # Example
/// ```ignore
/// use handle_gfa::graphoperation::*;
/// 
/// let mut graph = HashGraph::from_gfa(&gfa2);
/// graph = add_node(graph, Some(b"TEST_NODE_1"), 14 as u64).unwrap();
/// ```
pub fn add_node<T: Into<NodeId>>(
    mut graph: HashGraph, 
    sequence: Option<&[u8]>, 
    nodeid: T,
) -> std::result::Result<HashGraph, std::io::Error> {
    // get the last nodeid and then add a new node with 
    //  graph.create_handle(sequence, retrieved_nodeid)
    // or use  graph.append_handle(sequence) <- but first i need to understand better the behaviour

    let sequence = sequence.unwrap_or(b"NNNNNNNNNNNNNNNNNNNNN");
    let nodeid_temp = nodeid.into();

    for handle in graph.all_handles() {
        let old_seq_id = handle.id();
        if old_seq_id == nodeid_temp {
            panic!("The Id provided ({}) already exists in the graph!", nodeid_temp)
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
) -> std::result::Result<HashGraph, std::io::Error> {
    // it's better use Handle or NodeId?
    // get the left node and right node, fuse them togheter 
    // and create a new edge with
    //  graph.create_edge(Edge(from_node, to_node))
    use gfa2::gfa2::orientation::Orientation;

    let orient = |rev: bool| {
        if rev {
            Orientation::Backward
        } else {
            Orientation::Forward
        }
    }; 
    // println!("{:#?}", graph);
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
    if (!find_left) || (!find_right) {
        panic!("Cannot find the node(s)!")
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
) -> std::result::Result<HashGraph, std::io::Error> {
    // control if the path is circular? (first sequence_id equal to last sequence_id)
    // path = create_path_handle(path_id, is_circular)
    // iterate over sequence_of_id to do
    // graph.append_step(&path, sequence_of_id[i])
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
            _ => panic!("Error! segment ref id did not include orientation"),
        };
        let handle = Handle::new(seq_id.parse::<u64>().unwrap(), orient);
        graph.append_step(&path, handle);
    }

    Ok(graph)
}

/// print an HashGraph object as a DeBrujin Graph (VERY OPTIONAL)
pub fn print_pretty_graph(graph: &HashGraph) {
    // get all the segment with:
    // for handle in graph.all_handles()
    // get the id and sequence and form a tuple(id, sequence) 
    // order the tuples

    // get all the edge with:
    // for edge in graph.all_edges()
    // let Edge(left, right) = edge
    // get the id and orientation
    // apply reverse and complement the sequence if orientation = '-'
    // form tuple merging 2 segment tuple ((idleft, sequence)(idright, sequence))

    // add start and end tuple to the top and bottom of the "list"
    // (START) (END)
    // to obtain something like: 
    // (START)
    // ((1, A)(2, T))
    // ...
    // ((n-1, A)(n, T))
    // (END)

    // handle then the graphic part (VERY OPTIONAL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_gfa2_to_handlegraph() {
        let graph: HashGraph = gfa2_to_handlegraph("./tests/gfa2_files/spec_q7.gfa".to_string());
        println!("{:#?}", graph);
    }

    #[test]
    fn can_add_node() {
        use gfa2::{
            parser_gfa2::GFA2Parser,  
            gfa2::GFA2,
        };

        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();

        let graph = HashGraph::from_gfa(&gfa2);

        match add_node(graph, Some(b"TEST_NODE_1"), 14 as u64) {
            Ok(g) => println!("{:#?}", g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_link() {
        use gfa2::{
            parser_gfa2::GFA2Parser,  
            gfa2::GFA2,
        };

        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();

        let mut graph = HashGraph::from_gfa(&gfa2);

        graph = add_node(graph, Some(b"TEST_NODE_1"), 14 as u64).unwrap();
        graph = add_node(graph, Some(b"TEST_NODE_2"), 15 as u64).unwrap();

        match add_link_between_nodes(graph, 14 as u64, 15 as u64) {
            Ok(g) => println!("{:#?}", g),
            Err(why) => println!("Error: {}", why),
        };
    }

    #[test]
    fn can_add_path() {
        use gfa2::{
            parser_gfa2::GFA2Parser,  
            gfa2::GFA2,
        };

        let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
        let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/spec_q7.gfa").unwrap();

        let graph = HashGraph::from_gfa(&gfa2);
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
    }
}