use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};
use super::wavelet_tree_pointer::WaveletTree;



#[derive(Debug, Snafu)]
pub enum Error_Graph {
	#[snafu(display("Fehler bei select_1 auf der bitmap in ith_neighbor aufgetreten"))]
    ErrorIthNeighbor1,
	#[snafu(display("Fehler bei access Operation auf der adjaceny_list in ith_neighbor aufgetreten"))]
    ErrorIthNeighbor2,
	#[snafu(display("Der übergebene Index ist größer als die Anzahl der Knoten"))]
    ErrorIndexOutOfBounds,
	#[snafu(display("Fehler bei select auf der adjaceny in ith_reverse_neighbor aufgetreten"))]
    ErrorIthReverseNeighbor1,
	#[snafu(display("Fehler bei rank auf der bitmap in ith_reverse_neighbor aufgetreten"))]
    ErrorIthReverseNeighbor2,
}



pub struct WaveletGraph{
	adjacency_list: WaveletTree<u64>,
	bitmap: RankSelect,
	node_count: u64,
}


impl WaveletGraph {

	pub fn create_graph<E,N>(graph: Graph<E,N>) -> WaveletGraph{
		let mut i = 0; //Variable für das Setzen der Bits
		let nodes = graph.node_count();
		let len_bit = nodes + graph.edge_count(); //Länge des Bitvektors als Summe der Anzahl der nodes und edges
		let mut bit_v = BitVec::new_fill(false,len_bit as u64);
		let len_ad = graph.edge_count();
		let mut adjaceny_vec = Vec::with_capacity(len_ad);
		//Erstellen der benötigten Adjazenzliste und Bitmap
		for node_a in graph.node_indices() {
			bit_v.set_bit(i, true);
			i = i+1;
			//let anfang = i;
			println!("NodeIndizes: {} and neighbors:", node_a.index());
			for node_b in graph.neighbors(node_a){
				adjaceny_vec.push(node_b.index() as u64);
				println!("{}", node_b.index());
				i = i+1;
			}
			//adjaceny_vec.sort();
		}
		WaveletGraph{adjacency_list: WaveletTree::create_tree(adjaceny_vec.into_iter()), bitmap: RankSelect::new(bit_v,1), node_count: nodes as u64}
	}


	//Gibt den Index des iten Nachbarn vom Knoten v zurück
	pub fn ith_neighbor(&self, v: usize, i: usize) -> Result<u64,Error_Graph> {
		ensure!(self.node_count >= v as u64, ErrorIndexOutOfBounds);
		let l = match self.bitmap.select_1((v+1) as u64){
			Some(x) => x,
			None => return Err(Error_Graph::ErrorIthNeighbor1),
		};
		match self.adjacency_list.access(l as usize +(i-v)){
			Ok(x) => return Ok(x),
			Err(_) => return Err(Error_Graph::ErrorIthNeighbor2),
		};
	}



	//Gibt den Index des iten vorherigen Nachbarn vom Knoten v zurück
	pub fn ith_reverse_neighbor(&self, v: usize, i: usize) -> Result<u64,Error_Graph> {
		ensure!(self.node_count >= v as u64, ErrorIndexOutOfBounds);
		let p = match self.adjacency_list.select(v as u64,i){
			Ok(x) => x,
			Err(_) => return Err(Error_Graph::ErrorIthReverseNeighbor1),
		};
		match self.bitmap.rank_1(p){
			Some(x) => return Ok(x),
			None => return Err(Error_Graph::ErrorIthReverseNeighbor2),
		}
	}

}
