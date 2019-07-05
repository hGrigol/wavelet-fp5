use wavelet5::{ErrorGraph, WaveletGraph};
#[macro_use(matches)]
extern crate matches;
mod common;

#[test]
fn testing_ith_neighbor() {
    let graph = common::setup_graph();
    let single_edge = common::setup_single_graph();
    assert_eq!(graph.ith_neighbor(0, 2).unwrap(), 2);
    assert_eq!(graph.ith_neighbor(2, 1).unwrap(), 3);
    assert_eq!(single_edge.ith_reverse_neighbor(0, 1).unwrap(), 0);
}

#[test]
fn testing_ith_reverse_neighbor() {
    let graph = common::setup_graph();
    let single_edge = common::setup_single_graph();
    assert_eq!(graph.ith_reverse_neighbor(4, 2).unwrap(), 3);
    assert_eq!(graph.ith_reverse_neighbor(4, 1).unwrap(), 2);
    assert_eq!(graph.ith_reverse_neighbor(1, 1).unwrap(), 0);
    assert_eq!(single_edge.ith_reverse_neighbor(0, 1).unwrap(), 0);
}

#[test]
fn testing_fail_ith_neighbor() {
    let graph = common::setup_graph();
    let single_node = common::setup_single_node();
    let single_edge = common::setup_single_graph();
    assert!(matches!(
        graph.ith_neighbor(0, 4),
        Err(ErrorGraph::NeighborDoesnotExist)
    ));
    assert!(matches!(
        graph.ith_neighbor(6, 2),
        Err(ErrorGraph::ErrorIndexOutOfBounds)
    ));
    assert!(matches!(
        graph.ith_neighbor(4, 0),
        Err(ErrorGraph::ErrorIndexOutOfBounds)
    ));
    assert!(matches!(
        single_node.ith_neighbor(0, 4),
        Err(ErrorGraph::NeighborDoesnotExist)
    ));
    assert!(matches!(
        single_edge.ith_neighbor(0, 4),
        Err(ErrorGraph::NeighborDoesnotExist)
    ));
}

#[test]
fn testing_fail_ith_reverse_neighbor() {
    let graph = common::setup_graph();
    let single_edge = common::setup_single_graph();
    assert!(matches!(
        graph.ith_reverse_neighbor(8, 2),
        Err(ErrorGraph::ErrorIndexOutOfBounds)
    ));
    assert!(matches!(
        graph.ith_reverse_neighbor(4, 0),
        Err(ErrorGraph::ErrorIndexOutOfBounds)
    ));
    assert!(matches!(
        graph.ith_reverse_neighbor(4, 3),
        Err(ErrorGraph::ReverseNeighborDoesnotExist)
    ));
    assert!(matches!(
        single_edge.ith_reverse_neighbor(0, 3),
        Err(ErrorGraph::ReverseNeighborDoesnotExist)
    ));
}

#[test]
fn testing_serialize(){
	let graph = common::setup_graph();
	let serialized = serde_json::to_string(&graph).unwrap();
	let _graph2: WaveletGraph = serde_json::from_str(&serialized).unwrap();
	assert!(matches!(graph, _graph2));
}
