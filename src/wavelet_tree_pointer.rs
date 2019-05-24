use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;
#[derive(Debug)]
pub struct WaveletTree<E>{
    alphabet:Vec<E>,
    root: Option<Box<BinNode>>,
}

#[derive(Debug)]
pub struct BinNode {
    value:BitVec<u64> ,
    left: Option<Box<BinNode>>,
    right: Option<Box<BinNode>>,
}




impl<T> WaveletTree<T>
 where T: IntoIterator+Clone+Debug,
      <T as std::iter::IntoIterator>::Item : Hash+Clone+Ord+Debug{

  fn create(sequence: &T) -> WaveletTree<<T as std::iter::IntoIterator>::Item>{
    let mut seqvec = sequence.clone().into_iter().collect::<Vec<_>>();
    let mut vec = Vec::new();
    vec.extend(sequence.clone().into_iter().unique());
    vec.sort();
    WaveletTree{alphabet:vec, //root: BinNode::createNode(vec,seqvec)
    root: Some(Box::new(BinNode  {value: BitVec::new_fill(false,64),left:None,right:None}))}
  }

}





impl BinNode {

   // fn createNode(&alphabet: Vec<E>,&sequence: T){
    
   //let count = 
  //  let value: BitVec::new_fill(false,)

   // }
}





