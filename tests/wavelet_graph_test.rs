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
fn testing_which_neighbor(){
	let graph = common::setup_graph();
	let single_edge = common::setup_single_graph();
	assert_eq!(graph.which_neighbor(0,2).unwrap(),2);
	assert_eq!(graph.which_neighbor(2,3).unwrap(),1);
	assert_eq!(graph.which_neighbor(3,4).unwrap(),1);
	assert_eq!(single_edge.which_neighbor(0,0).unwrap(),1);
	assert!(matches!(graph.which_neighbor(1,3), Err(ErrorGraph::IsNoNeighbor)));
	assert!(matches!(single_edge.which_neighbor(2,3), Err(ErrorGraph::ErrorIndexOutOfBounds)));
}
#[test]
fn testing_get_weights(){
	let graph_unweighted = common::setup_graph();
	let graph = common::setup_graph_weighted();
	assert_eq!(graph.get_weight(0,1).unwrap(),1);
	assert_eq!(graph.get_weight(3,4).unwrap(),5);
	assert_eq!(graph.get_weight(2,4).unwrap(),4);
	assert!(matches!(graph.get_weight(0,0), Err(ErrorGraph::IsNoNeighbor)));
	assert_eq!(graph_unweighted.get_weight(0,1).unwrap(),0);
	assert_eq!(graph_unweighted.get_weight(3,4).unwrap(),0);
	assert_eq!(graph_unweighted.get_weight(2,4).unwrap(),0);
	assert!(matches!(graph_unweighted.get_weight(0,0), Err(ErrorGraph::IsNoNeighbor)));
}

#[test]
fn testing_serialize() {
    let graph_unweighted = common::setup_graph();
	let graph = common::setup_graph_weighted();
    let serialized_unweighted = serde_json::to_string(&graph_unweighted).unwrap();
	let serialized = serde_json::to_string(&graph).unwrap();
    let _graph2: WaveletGraph<u64> = serde_json::from_str(&serialized_unweighted).unwrap();
	let _graph3: WaveletGraph<u64> = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(graph_unweighted, _graph2));
	assert!(matches!(graph, _graph3));

}

#[test]
fn testing_get_neighbors() {
	let graph_unweighted = common::setup_graph();
	let graph = common::setup_graph_weighted();
	let mut test_vec = Vec::new();
	test_vec.push(1);
	test_vec.push(2);
	let mut test_vec2 = Vec::new();
	test_vec2.push(4);
	let mut empty_vec: Vec<u64> = Vec::new();
	assert!(matches!(graph_unweighted.get_neighbors(0).unwrap(), test_vec));
	assert!(matches!(graph.get_neighbors(0).unwrap(), test_vec));
	assert!(matches!(graph_unweighted.get_neighbors(3).unwrap(), test_vec2));
	assert!(matches!(graph.get_neighbors(3).unwrap(), test_vec2));
	assert!(matches!(graph_unweighted.get_neighbors(1).unwrap(), empty_vec));
	assert!(matches!(graph.get_neighbors(1).unwrap(), empty_vec));
	assert!(matches!(graph.get_neighbors(8),Err(ErrorGraph::ErrorIndexOutOfBounds)));
}
