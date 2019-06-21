use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use super::wavelet_tree_pointer::WaveletTree;


pub struct WaveletGraph{
	adjacency_list: WaveletTree<u64>,
	bitmap: RankSelect,
}


impl WaveletGraph {

	pub fn create_graph<E,N>(graph: Graph<E,N>) -> WaveletGraph{
		let mut i = 0; //Variable für das Setzen der Bits
		let len_bit = graph.node_count() + graph.edge_count(); //Länge des Bitvektors als Summe der Anzahl der nodes und edges
		let mut bit_v = BitVec::new_fill(false,len_bit as u64);
		let len_ad = graph.edge_count();
		let mut adjaceny_vec = Vec::with_capacity(len_ad);
		//Erstellen der benötigten Adjazenzliste und Bitmap
		for node_a in graph.node_indices() {
			bit_v.set_bit(i, true);
			i = i+1;
			for node_b in graph.neighbors(node_a){
				adjaceny_vec.push(node_b.index() as u64);
				i = i+1;
			}
		}
		WaveletGraph{adjacency_list: WaveletTree::create_tree(adjaceny_vec.into_iter()), bitmap: RankSelect::new(bit_v,1)}
	}


	//Gibt den Index des iten Nachbarn vom Knoten v zurück
	pub fn ith_neighor(&self, v: usize, i: usize) -> Result<u64,&'static str> {
		let l = match self.bitmap.select_1(v as u64){
			Some(x) => x,
			None => return Err("Fehler bei ith_neighbor"),
		};
		match self.adjacency_list.access(l as usize +(i-v)){
			Ok(x) => return Ok(x),
			Err(_) => return Err ("Fehler bei access in ith_neighbor"),
		};
	}



	//Gibt den Index des iten vorherigen Nachbarn vom Knoten v zurück
	pub fn ith_reverse_neighbor(&self, v: usize, i: usize) -> Result<u64,&'static str> {
		let p = match self.adjacency_list.select(v as u64,i){
			Ok(x) => x,
			Err(_) => return Err("Fehler bei select in ith_reverse_neighbor"),
		};
		match self.bitmap.rank_1(p){
			Some(x) => return Ok(x),
			None => return Err("Fehler bei ith_neighbor"),
		}
	}

}
