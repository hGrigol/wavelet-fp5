use wavelet5::{Error, WaveletTree, WaveletGraph};
#[macro_use(matches)]
extern crate matches;
mod common;


/*#[test]
fn testing_create(){
	let graph = common::setup_graph();
}*/

#[test]
fn testing_ith_neighbor(){
	let graph = common::setup_graph();
	let neighbor = graph.ith_neighbor(0,2);
	assert_eq!(neighbor.unwrap(), 2);
}
