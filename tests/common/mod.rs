use wavelet5::WaveletTree;
use wavelet5::WaveletGraph;
use petgraph::Graph;

pub fn setup_string1() -> WaveletTree<char>{
 wavelet5::WaveletTree::create_tree("Hallo Ich bin ein Test für einen Satz".chars().into_iter())   
}

pub fn setup_string() -> WaveletTree<char> {
    wavelet5::WaveletTree::create_tree("AsWDaaaaa aGDW!/(%§".chars().into_iter())
}

pub fn setup_graph() -> WaveletGraph{
	let mut deps = Graph::<&str, &str>::new();
	let pg = deps.add_node("petgraph");
	let fb = deps.add_node("fixedbitset");
	let qc = deps.add_node("quickcheck");
	let rand = deps.add_node("rand");
	let libc = deps.add_node("libc");
	deps.extend_with_edges(&[
    	(pg, fb), (pg, qc),
    	(qc, rand), (rand, libc), (qc, libc)
	]);
	wavelet5::WaveletGraph::create_graph(deps)
}

