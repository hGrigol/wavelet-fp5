use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use super::wavelet_tree_pointer::WaveletTree;


pub struct WaveletGraph<T>{
	adjacency_list: WaveletTree<T>,
	bitmap: RankSelect,
}


impl<T> WaveletGraph<T> {

	pub fn create_graph<E,N>(graph: Graph<E,N>) -> WaveletGraph<T>{
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


	//Gibt den Index des iten Nachbarn vom Knoten v zurück
	pub fn ith_neighor(&self, v: NodeIndex, i: u64) -> Result<u64,&'static str> {
		let l = match &self.bitmap.select_1(v.index() as u64){
			Some(x) => x,
			None => return Err("Fehler bei ith_neighbor"),
		};
		let result = &self.adjacency_list.access(l+i-v.index() as u64);
		
		result.self() as u64

	}

	//Gibt den Index des iten vorherigen Nachbarn vom Knoten v zurück
	pub fn ith_reverse_neighbor(&self, v: NodeIndex, i: usize) -> Result<u64,&'static str> {
		let p = &self.adjacency_list.select(v,i);
		let l = match &self.bitmap.rank_1(p){
			Some(x) => return Ok(*x),
			None => return Err("Fehler bei ith_neighbor"),
		};
	}

}
