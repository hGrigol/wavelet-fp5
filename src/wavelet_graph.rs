use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use petgraph::graph::Graph;
use snafu::{Snafu, ensure};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use super::wavelet_tree_pointer::WaveletTree;


#[derive(Debug, Snafu)]
pub enum ErrorGraph {
    #[snafu(display("Error occured when calling select_1 in ith_neighbor"))]
    ErrorIthNeighbor1,
    #[snafu(display("Error occured when calling access on adjaceny_list in ith_neighbor"))]
    ErrorIthNeighbor2,
    #[snafu(display("v > number of nodes or i = 0"))]
    ErrorIndexOutOfBounds,
    #[snafu(display(
        "Error occured when calling select on adjaceny_list in ith_reverse_neighbor"
    ))]
    ReverseNeighborDoesnotExist,
    #[snafu(display("Error occured when calling rank_1 in ith_reverse_neighbor"))]
    ErrorIthReverseNeighbor2,
	#[snafu(display("This neighbor does not exist, number of neighbors < i"))]
	NeighborDoesnotExist,
	#[snafu(display("Try to access placeholder"))]
	AccessPlaceholder,	
	#[snafu(display("Error occured when calling select_1 in which_neighbor"))]
	WhichNeighborError,
	#[snafu(display("The given i is no neighbor of v"))]
	IsNoNeighbor,
	#[snafu(display("Error while trying to get the weight"))]
	WeightError,
}

/// Representation of a directed or undirected graph.
/// All adjacency lists are concatenated and saved as a WaveletTree.  
/// Indices are saved as `Option<u64>` starting with 0.
/// None is added as a Placeholder when a new adjaceny list is concatenated.
/// In the bitmap a true marks the beginning of a new adjaceny list
/// (e.g. the fifth true bit marks the beginning of the adjaceny list of the node with index 5)
#[derive(Serialize, Deserialize)]
pub struct WaveletGraph<E>{
	adjacency_list: WaveletTree<Option<u64>>,
	bitmap: RankSelect,
	edge_weights: Vec<Option<E>>
}


impl<'de, E> WaveletGraph<E>
where
    E: Hash + Clone + Ord + Debug + Copy + Serialize + Deserialize<'de>,

{

	/// Creates a representation of a given petgraph as a WaveletTree and a bitmap.
	pub fn create_graph<N>(graph: Graph<N,E>) -> WaveletGraph<E>{
		let mut i = 0; //Variable for setting the bits
		let nodes = graph.node_count();
		let len_vec = nodes + graph.edge_count();
		let mut bit_v = BitVec::new_fill(false,len_vec as u64);
		let mut adjaceny_vec = Vec::with_capacity(len_vec);
		let mut weight_vec = Vec::with_capacity(len_vec);
		//Creating adjacency_list and bitmap
		for node_a in graph.node_indices() {
			bit_v.set_bit(i, true);
			weight_vec.push(None);
			let mut weights = Vec::new();
			for edge_a in graph.edges(node_a) {
				let single_weight = edge_a.weight();
				weights.push(Some(*single_weight));
			}
			weights.reverse();
			weight_vec.append(&mut weights);
			adjaceny_vec.push(None);
			i = i+1;
			let mut neighbors = Vec::new();
			for node_b in graph.neighbors(node_a){
				neighbors.push(Some(node_b.index() as u64));
				i = i+1;
			}
			neighbors.reverse();
			adjaceny_vec.append(&mut neighbors);
		}
		//println!("adjacency_list: {:?}", adjaceny_vec);
		//println!("bitmap: {:?}", bit_v);
		//println!("weights: {:?}", weight_vec);
		WaveletGraph{adjacency_list: WaveletTree::create(adjaceny_vec.into_iter()), bitmap: RankSelect::new(bit_v,1), edge_weights: weight_vec}
	}


	/// Returns the index of the ith neighbor of node v.
	/// Node indices start with 0
	pub fn ith_neighbor(&self, v: usize, i: usize) -> Result<u64,ErrorGraph> {
		ensure!(self.adjacency_list.alphabet_len() > v && i > 0, ErrorIndexOutOfBounds);
		let l = match self.bitmap.select_1((v+1) as u64){
			Some(x) => x,
			None => return Err(ErrorGraph::ErrorIthNeighbor1),
		};
		let cutoff = match self.bitmap.select_1((v+2) as u64){
			Some(x) => x,
			None => self.bitmap.bits().len(),
		};
		ensure!(cutoff >= l + i as u64 - v as u64, NeighborDoesnotExist);
		match self.adjacency_list.access(l as usize + i + 1){
			Ok(x) => match x{
				Some(x) => return Ok(x),
				None => return Err(ErrorGraph::AccessPlaceholder),
			},
			Err(_) => return Err(ErrorGraph::ErrorIthNeighbor2),
		};
	}



	/// Returns the index of the ith reverse neighbor of node v.
	/// Node indices start with 0
	pub fn ith_reverse_neighbor(&self, v: usize, i: usize) -> Result<u64,ErrorGraph> {
		ensure!(self.adjacency_list.alphabet_len() > v && i > 0, ErrorIndexOutOfBounds);
		let p = match self.adjacency_list.select(Some(v as u64),i){
			Ok(x) => x,
			Err(_) => return Err(ErrorGraph::ReverseNeighborDoesnotExist),
		};
		match self.bitmap.rank_1(p-1){
			Some(x) => return Ok(x-1),
			None => return Err(ErrorGraph::ErrorIthReverseNeighbor2),
		}
	}
	
	/// Returns which neighbor of v the node with index i is
	pub fn which_neighbor(&self, v: usize, i: usize) -> Result<u64, ErrorGraph>{
		ensure!(self.adjacency_list.alphabet_len() > v && self.adjacency_list.alphabet_len() > i,
		ErrorIndexOutOfBounds);
		let postion_v = match self.bitmap.select_1((v+1) as u64) {
			Some(x) => x,
			None => return Err(ErrorGraph::WhichNeighborError),
		};
		let cutoff = match self.bitmap.select_1((v + 2) as u64) {
            Some(x) => x,
            None => self.bitmap.bits().len(),
        };
		for number in postion_v+2..cutoff+1 {
			if self.adjacency_list[number as usize].unwrap() == i as u64 {
				return Ok(number-(postion_v+1))
			}
		}
		Err(ErrorGraph::IsNoNeighbor)		
	}
	
	/// Returns the weight of the edge between node v and i 
	pub fn get_weight(&self, v: usize, i: usize) -> Result<E, ErrorGraph>{
		ensure!(self.adjacency_list.alphabet_len() > v && self.adjacency_list.alphabet_len() > i,
		ErrorIndexOutOfBounds);
		let postion_v = match self.bitmap.select_1((v+1) as u64) {
			Some(x) => x,
			None => return Err(ErrorGraph::WhichNeighborError),
		};
		let neighbor = match self.which_neighbor(v,i){
			Ok(x) => x,
			Err(_) => return Err(ErrorGraph::IsNoNeighbor),
		};
		match self.edge_weights[(postion_v + neighbor) as usize]{
			Some(x) => return Ok(x),
			None => return Err(ErrorGraph::WeightError),
		}		
	}
}
