use wavelet5::WaveletTree;
use wavelet5::WaveletGraph;
use petgraph::Graph;

pub fn setup_string1() -> WaveletTree<char>{
 wavelet5::WaveletTree::create_tree("Hallo Ich bin ein Test für einen Satz".chars().into_iter())   
}

pub fn setup_string() -> WaveletTree<char> {
    wavelet5::WaveletTree::create_tree("AsWDaaaaa aGDW!/(%§".chars().into_iter())
}


pub fn setup_graph() -> WaveletGraph<u64>{
	let mut deps = Graph::<&str, u64>::new();
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


pub fn setup_ints() -> WaveletTree<u64> {
    let mut vec = Vec::new();
    for x in 0..1000000 {
        vec.push(x % 30);
    }
    wavelet5::WaveletTree::create_tree(vec.into_iter())
}

pub fn setup_single_node() -> WaveletGraph<u64>{
	let mut deps = Graph::<&str, u64>::new();
	let pg = deps.add_node("petgraph");
	wavelet5::WaveletGraph::create_graph(deps)
}

pub fn setup_single_graph() -> WaveletGraph<u64>{
	let mut deps = Graph::<&str, u64>::new();
	let pg = deps.add_node("petgraph");
	deps.extend_with_edges(&[(pg,pg)]);
	wavelet5::WaveletGraph::create_graph(deps)
}

pub fn setup_graph_weighted() -> WaveletGraph<u64>{
	let mut deps = Graph::<&str, u64>::new();
	let pg = deps.add_node("petgraph");
	let fb = deps.add_node("fixedbitset");
	let qc = deps.add_node("quickcheck");
	let rand = deps.add_node("rand");
	let libc = deps.add_node("libc");
	deps.extend_with_edges(&[
    	(pg, fb, 1), (pg, qc, 2),
    	(qc, rand, 3), (rand, libc, 5), (qc, libc, 4)
	]);
	wavelet5::WaveletGraph::create_graph(deps)
}