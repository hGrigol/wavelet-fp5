use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;
use petgraph::graph::Graph;
use super::wavelet_tree_pointer::WaveletTree;


pub struct WaveletGraph<T>{
	adjacency_list: WaveletTree<T>,
	bitmap: RankSelect,
}


impl<T> WaveletGraph<T> {

	pub fn create_graph<E,N>(&self, graph: Graph<E,N>) -> WaveletGraph<T>{
		let mut i = 0; //Variable für das Setzen der Bits
		let lenBit = graph.node_count() + graph.edge_count(); //Länge des Bitvektors als Summe der Anzahl der nodes und edges
		let mut bitV = BitVec::new_fill(false,lenBit as u64);
		let lenAd = graph.edge_count();
		let mut adjacenyVec = Vec::with_capacity(lenAd);
		//Erstellen der benötigten Adjazenzliste und Bitmap
		for nodeA in graph.node_indices() {
			bitV.set_bit(i, true);
			i = i+1;
			for nodeB in graph.neighbors(nodeA){
				adjacenyVec.push(nodeB);
				i = i+1;
			}
		}
		WaveletGraph{adjacency_list: WaveletTree::create_tree(adjacenyVec.into_iter()), bitmap: RankSelect::new(bitV,1)}
	}

}
