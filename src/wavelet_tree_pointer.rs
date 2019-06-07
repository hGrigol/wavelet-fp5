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
	let vec2 =vec.clone();
    WaveletTree{alphabet: vec2, root: Some(Box::new(BinNode::create_node(vec,seqvec)))}
  }

 pub fn access (&self,index : usize) -> Result<T, &'static str>{
      let z = match &self.root{

		      Some(x) => x.access(index as u64,&self.alphabet,0,self.alphabet.len()-1),
		      None =>return Err("Element nicht gefunden"),   //TODO snafu Fehler implementieren

		      };
     match z {
     
          Some(x)=> Ok(x),
          None => return Err("Element nicht gefunden"),
     }
         
 }
 pub fn select (&self,character : T,index : usize) -> Result<u64,&'static str>{
	let character_index1 = &self.alphabet.binary_search(&character); // speichere an welchem index steht das gesuchte zeichen im alphabet steht 
	let character_index = match character_index1  {
		Ok(x)  => x ,
		Err(_) => return Ok(100000), //TODO  element nicht in alphabet 
	};	
	let result = match &self.root {
		Some(x) => x.select(index as u64,&self.alphabet,character_index,0,self.alphabet.len()-1),
		None => return Err("Fehler"),
	};
	match result {
		Some(x)=> return Ok(x),
		None => return Err("Fehler bei select"),
	} 	

}
 
  pub fn rank (&self,character : T,index : usize) -> Result<u64,&'static str>{	
	let character_index1 = &self.alphabet.binary_search(&character); // speichere an welchem index steht das gesuchte zeichen im alphabet steht 
	let character_index = match character_index1  {
		Ok(x)  => x ,
		Err(_) => return Ok(0),  //element nicht in alphabet => gib 0 zurück 
	};
	let result = match &self.root {

		Some(x) => (*x).rank(index as u64,&self.alphabet,character_index,0,&self.alphabet.len()-1),
		None => return Err("Element nicht gefunden"),
	};		
	match result {
				
		Some(x) => return Ok(x),
		None => return Err("Element nicht gefunden"),	
	}


  }  

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
            let mid = (alphabet.len()+1)/2;
            //Das Alphabet wird geteilt, die 2. Hälfte wird in alphabet2 gespeichert
            let alphabet2 = alphabet.split_off(mid);//TODO eigentlich mid+1,aber dann stack overflow?
            //Die Sequenzen für den nächsten Schritt
            let mut sequence1 = Vec::new();
            let mut sequence2 = Vec::new();
            //Es werden alle Elemente der Sequenz durchegangen
            for x in 0..(sequence.len()){
                //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
                if alphabet2.contains(&sequence[x]){
                value.set_bit(x as u64,true)} 
            }
            //Group_by teilt in Gruppen key ist true wenn Zeichen in alphabet1, sonst false
            for (key,group) in &sequence.into_iter().group_by(|elem| alphabet.contains(&elem)){
                //neue Sequencen werden anhand der Keys gebaut
                if key {sequence1.extend(group)} else {sequence2.extend(group)}
            }
			/*println!("alphabet1 : {}",alphabet.len());
			println!("alphabet2 : {}",alphabet2.len());
			println!("{:?}",value);
			println!("-------------------------");
			*/

            BinNode{value: RankSelect::new(value,1),left: Some(Box::new(BinNode::create_node(alphabet,sequence1))), right:Some(Box::new(BinNode::create_node(alphabet2,sequence2)))}
        }
    }
  
  fn access<E:Hash+Clone+Ord+Debug+Copy> (&self,index : u64,alphabet: &Vec<E>,min: usize,max : usize) -> Option<E>{
      if min == max { return Some(alphabet[min]);}
      else{
		  
		  if self.value.get((index) as u64) { 
		      let next_index = self.value.rank((index) as u64).unwrap();
		      match &self.right{
		      Some(x) => return (*x).access(next_index-1,alphabet,1+(min+max)/2,max),
		      None =>return None,   //TODO snafu Fehler implementieren
		      }
		  }
		  else {
		      let next_index = self.value.rank_0((index) as u64).unwrap();
		      match &self.left{
		      Some(x) =>return (*x).access(next_index-1,alphabet,min,(min+max)/2),
		      None => return None,   //TODO snafu Fehler implementieren
		      }
		  }
    }
  }

  fn select <E:Hash+Clone+Ord+Debug+Copy> (&self,index : u64,alphabet: &Vec<E>,character : &usize, min : usize ,max : usize) -> Option<(u64)>{
	if min==max{return Some(index-1);}
	else{
		if character <= &((max+min)/2){
			
			let result = match &self.left {
				Some(x) => (*x).select(index,alphabet,character,min,(min+max)/2),
				None => return None,
			};
			let new_index = match result {
				Some(x) => x,
				None => return None,
			};
			return self.value.select_0(new_index+1);
			

		}
		else{
		
			let result = match &self.right{
				Some(x) => (*x).select(index,alphabet,character,(min+max)/2 +1,max),
				None => return None,
			};
			let new_index = match result {
				Some(x) => x,
				None => return None,
			};
			return self.value.select_1(new_index+1);
		
		}
			    				
		
		

		
	}
	}



  fn rank<E:Hash+Clone+Ord+Debug+Copy> (&self,index : u64,alphabet: &Vec<E>,character : &usize, min : usize ,max : usize) -> Option<u64>{
	//println!("index:{}",index);
	if min == max { return Some(index+1)} //Wenn im blatt 
	else{
		if character <= &((max+min)/2)	    		
		{
			
			let next_index=self.value.rank_0((index) as u64).unwrap();
			match &self.left{
				Some(x)=> return (*x).rank(next_index-1,alphabet,character,min,(min+max)/2),
				None => return None
			}
		}
		else{
			let next_index=self.value.rank((index) as u64).unwrap();
			match &self.right{
				Some(x)=> return (*x).rank(next_index-1,alphabet,character,((min+max)/2)+1,max),
				None => return None
			}
	
		}
	}    
	
  }
  
}


























