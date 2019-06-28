use wavelet5::{Error, WaveletTree, WaveletGraph, ErrorGraph};
#[macro_use(matches)]
extern crate matches;
mod common;


#[test]
fn testing_ith_neighbor(){
	let graph = common::setup_graph();
	assert_eq!(graph.ith_neighbor(0,2).unwrap(),2);
	assert_eq!(graph.ith_neighbor(2,1).unwrap(),3);
}

#[test]
fn testing_ith_reverse_neighbor(){
	let graph = common::setup_graph();
	assert_eq!(graph.ith_reverse_neighbor(4,2).unwrap(),3);
	assert_eq!(graph.ith_reverse_neighbor(4,1).unwrap(),2);
	assert_eq!(graph.ith_reverse_neighbor(1,1).unwrap(),0);
}

#[test]
fn testing_fail_ith_neighbor(){
	let graph = common::setup_graph();
	assert!(matches! (graph.ith_neighbor(0,4), Err(ErrorGraph::NeighborDoesnotExist)));
	assert!(matches! (graph.ith_neighbor(6,2), Err(ErrorGraph::ErrorIndexOutOfBounds)));
	assert!(matches! (graph.ith_neighbor(4,0), Err(ErrorGraph::ErrorIndexOutOfBounds)));
}

#[test]
fn testing_fail_ith_reverse_neighbor(){
	let graph = common::setup_graph();
	assert!(matches! (graph.ith_reverse_neighbor(8,2), Err(ErrorGraph::ErrorIndexOutOfBounds)));
	assert!(matches! (graph.ith_reverse_neighbor(4,0), Err(ErrorGraph::ErrorIndexOutOfBounds)));
	assert!(matches! (graph.ith_reverse_neighbor(4,3), Err(ErrorGraph::ReverseNeighborDoesnotExist)));
}

