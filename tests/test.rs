// manipulate hashgraph
use handlegraph2::hashgraph::*;

// manipulate files
use gfa2::{gfa1::GFA, gfa2::GFA2, parser_gfa1::GFAParser, parser_gfa2::GFA2Parser};

use handlegfa::{fileoperation::*, graphoperation::*};

#[test]
fn readme_file_test() {
    let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
    let gfa2: GFA2<usize, ()> = parser.parse_file("./tests/gfa2_files/irl.gfa2").unwrap();
    println!("{:#?}", gfa2);
    println!("{}", gfa2);
    let graph = HashGraph::from_gfa2(&gfa2);
    println!("{:#?}", graph);
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
        b"5001+", b"5002+", b"5003-", b"5004+", b"5005-", b"5006-", b"5007+", b"5008+", b"5009+",
        b"5010-",
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
}

#[test]
fn big_graph_with_big_operation() {
    // about 8 minutes
    println!("Parse and create graph");
    let parser: GFAParser<usize, ()> = GFAParser::new();
    let gfa: GFA<usize, ()> = parser
        .parse_file("./tests/big_files/ape-4-0.10b.gfa")
        .unwrap();
    let mut graph = HashGraph::from_gfa(&gfa);

    // about x minutes
    // remove nodes
    println!("Remove 1_000 nodes");
    for i in 1..1_001 {
        match remove_node(
            graph.clone(),
            format!("{}{}", 115, i).parse::<u64>().unwrap(),
        ) {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };
    }
    /*
    const PATHS: [&[u8]; 10] = [
        b"path-1", b"path-2", b"path-3", b"path-4", b"path-5", b"path-6", b"path-7", b"path-8",
        b"path-9", b"path-10",
    ];
    // about x minutes
    println!("Add 10 paths containing 100 segment ids each");
    // add paths
    let mut x = 10_000;
    for i in 1..PATHS.len() {
        let mut ids: Vec<&[u8]> = vec![];
        let path_name: &[u8] = PATHS.get(i as usize).unwrap();
        for n in 1..101 {
            ids.push(format!("{}{}{}", 115, x + n, "+").as_bytes());  
        }
        match add_path(graph.clone(), Some(path_name), ids) {
            Ok(g) => graph = g,
            Err(why) => println!("Error: {}", why),
        };
        x += 10_000;
    }
    */

    // about x minutes
    println!("Save modified file");
    match save_as_gfa1_file(
        &graph,
        Some(String::from("./tests/output_files/new_ape-4-0.10b.gfa")),
    ) {
        Ok(_) => println!("File saved!"),
        Err(why) => println!("Error: {}", why),
    }
}

#[test]
fn extension_error() {
    match gfa2_to_handlegraph("./tests/gfa2_files/error_extension.txt".to_string()) {
        Ok(g) => {
            let graph: HashGraph = g;
            print_simple_graph(&graph)
        }
        Err(why) => println!("Error: {}", why),
    };
}
