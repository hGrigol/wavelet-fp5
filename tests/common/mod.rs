use petgraph::Graph;
use wavelet5::WaveletGraph;
use wavelet5::WaveletTree;
use wavelet5::WaveletTreePointerFree;

pub fn setup_string1() -> WaveletTree<char> {
    wavelet5::WaveletTree::create("Hallo Ich bin ein Test für einen Satz".chars().into_iter())
}

pub fn setup_string() -> WaveletTree<char> {
    wavelet5::WaveletTree::create("AsWDaaaaa aGDW!/(%§".chars().into_iter())
}

pub fn setup_ints() -> WaveletTree<u64> {
    let mut vec = Vec::new();
    for x in 0..1000000 {
        vec.push(x % 30);
    }
    wavelet5::WaveletTree::create(vec.into_iter())
}
pub fn setup_string_2_2() -> WaveletTreePointerFree<char> {
    wavelet5::WaveletTreePointerFree::create(
        "Hallo Ich bin ein Test für einen Satz".chars().into_iter(),
    )
}

pub fn setup_string_2_1() -> WaveletTreePointerFree<char> {
    wavelet5::WaveletTreePointerFree::create("AsWDaaaaa aGDW!/(%§".chars().into_iter())
}

pub fn setup_graph() -> WaveletGraph {
    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    let fb = deps.add_node("fixedbitset");
    let qc = deps.add_node("quickcheck");
    let rand = deps.add_node("rand");
    let libc = deps.add_node("libc");
    deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);
    wavelet5::WaveletGraph::create_graph(deps)
}

pub fn setup_single_node() -> WaveletGraph {
    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    wavelet5::WaveletGraph::create_graph(deps)
}

pub fn setup_single_graph() -> WaveletGraph {
    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    deps.extend_with_edges(&[(pg, pg)]);
    wavelet5::WaveletGraph::create_graph(deps)
}
