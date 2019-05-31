use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;



pub struct WaveletTree<E>{
    alphabet:Vec<E>,
    root: Option<Box<BinNode>>,
}

pub struct BinNode {
    value:RankSelect ,
    left: Option<Box<BinNode>>,
    right:Option<Box<BinNode>>,
}




impl<T> WaveletTree <T>
 where T: Hash+Clone+Ord+Debug+Copy{

  pub fn create_tree<S:Clone+IntoIterator<Item=T>>(sequence: S) -> WaveletTree<T>{
    let seqvec = sequence.clone().into_iter().collect::<Vec<_>>();
    let mut vec = Vec::new();
    vec.extend(sequence.into_iter().unique());
    vec.sort();
    println!("{:?}",vec);
    WaveletTree{alphabet:vec.clone(), root: Some(Box::new(BinNode::create_node(vec,seqvec)))}
  }

 fn access (&self,index : usize) -> Result<T, &'static str>{
      let z = match &self.root{

		      Some(x) => x.access(index as u64,&self.alphabet,0,self.alphabet.len()-1),
		      None =>return Err("Element nicht gefunden"),   //TODO snafu Fehler implementieren

		      };
     match z {
     
          Some(x)=> Ok(x),
          None => return Err("Element nicht gefunden"),
     }
         
 }
 //fn select (symbol:<T as std::iter::IntoIterator>::Item, index : usize) -> usize{}
 
  

}




impl BinNode{

    fn create_node<E:Hash+Clone+Ord+Debug>(mut alphabet: Vec<E>,sequence: Vec<E>,) -> BinNode{
    
        let count = sequence.len(); 
        //Wenn wir nur ein Zeichen im Alphabet haben sind wir in einem leaf -> keine Kinder
        if alphabet.len()<=1 {
             let value = BitVec::new_fill(true,count as u64);
             BinNode{value: RankSelect::new(value,1), left: None, right: None}
        }
        else{
            let mut value = BitVec::new_fill(false,count as u64);
            let mid = alphabet.len()/2;
            //Das Alphabet wird geteilt, die 2. Hälfte wird in alphabet2 gespeichert
            let alphabet2 = alphabet.split_off(mid+1);
            //Die Sequenzen für den nächsten Schritt
            let mut sequence1 = Vec::new();
            let mut sequence2 = Vec::new();
            //Es werden alle Elemente der Sequenz durchegangen
            for x in 0..sequence.len()-1{
                //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
                if alphabet2.contains(&sequence[x]){
                value.set_bit(x as u64,true)} 
            }
            //Group_by teilt in Gruppen key ist true wenn Zeichen in alphabet1, sonst false
            for (key,group) in &sequence.into_iter().group_by(|elem| alphabet.contains(&elem)){
                //neue Sequencen werden anhand der Keys gebaut
                if key {sequence1.extend(group)} else {sequence2.extend(group)}
            }
            BinNode{value: RankSelect::new(value,1),left: Some(Box::new(BinNode::create_node(alphabet,sequence1))), right:Some(Box::new(BinNode::create_node(alphabet2,sequence2)))}
        }
    }
  
  fn access<E:Hash+Clone+Ord+Debug+Copy> (&self,index : u64,alphabet: &Vec<E>,min: usize,max : usize) -> Option<E>{
      if min == max { return Some(alphabet[min]);}
      else{
		  
		  if self.value.get((index-1) as u64) { 
		      let next_index = self.value.rank((index-1) as u64).unwrap();
		      match &self.right{
		      Some(x) => return (*x).access(next_index,alphabet,1+(min+max)/2,max),
		      None =>return None,   //TODO snafu Fehler implementieren
		      }
		  }
		  else {
		      let next_index = self.value.rank_0((index-1) as u64).unwrap();
		      match &self.left{
		      Some(x) =>return (*x).access(next_index,alphabet,min,(min+max)/2),
		      None => return None,   //TODO snafu Fehler implementieren
		      }
		  }
    }
  }

  //fn select (symbol:<T as std::iter::IntoIterator>::Item, index : usize) -> usize{}
  
  
}



