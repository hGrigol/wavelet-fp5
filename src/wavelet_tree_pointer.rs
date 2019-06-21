use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;
use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Es gibt kein 0tes Element, das erste Element wird mit access(1) angesprochen"))]
    Access0,
    #[snafu(display("Eingabe darf bei select nicht kleiner als 1 sein"))]
    SelectSmaller0,
    #[snafu(display("Fehler bei root unwrap in access"))]
    RootUnwrapError,
    #[snafu(display("Index ist größer als die Länge der Sequence"))]
    IndexOutOfBound,
    #[snafu(display("Element nicht gefunden"))]
    NoSuchElement,
    #[snafu(display("Element nicht im Alphabet, Fehler bei select"))]
    NotInAlphabet,
    #[snafu(display("Das Symbol kommt nicht oft genug im Wort vor"))]
    NotEnough,
    
   #[snafu(display("PlatzhalterError"))]
   TempError,
}

#[derive(Serialize, Deserialize)]
pub struct WaveletTree<E>{
    alphabet:Vec<E>,
    root: Option<Box<BinNode>>,
}
#[derive(Serialize, Deserialize)]
pub struct BinNode {
    value:RankSelect ,
    left: Option<Box<BinNode>>,
    right:Option<Box<BinNode>>,
}




impl<'de,T> WaveletTree <T>
 where T: Hash+Clone+Ord+Debug+Copy+Serialize+Deserialize<'de>{

  pub fn create_tree<S:Clone+Iterator<Item=T>>(sequence: S) -> WaveletTree<T>{
    let seqvec = sequence.clone().collect::<Vec<_>>();
    let mut vec = Vec::new();
    vec.extend(sequence.unique());
    vec.sort();

    let serialized = serde_json::to_string(&vec).unwrap();

    println!("{:?}",vec);
	let vec2 =vec.clone();
    WaveletTree{alphabet: vec2, root: Some(Box::new(BinNode::create_node(vec,seqvec)))}
  }

 pub fn access (&self,index : usize) -> Result<T, Error>{
    ensure!(index >0,Access0);
	// Abfangen von fehlerhafter Eingabe, Index ist größer als Sequenz
	let z = match &self.root{
		Some(x) => x,
		None => return Err(Error::RootUnwrapError),
	};
	ensure!(z.value.bits().len() >index as u64,IndexOutOfBound);
	

	//-------------------------------------------	
    let z = match &self.root{

		      Some(x) => x.access((index-1) as u64,&self.alphabet,0,self.alphabet.len()-1),
		      None =>return Err(Error::NoSuchElement),   //TODO snafu Fehler implementieren

		      };
     match z {
     
          Some(x)=> Ok(x),
          None => return Err(Error::NoSuchElement),
     }
         
 }
 pub fn select (&self,character : T,index : usize) -> Result<u64,Error>{

	// Abfangen von fehlerhafter Eingabe, Index darf hier nicht 0 sein
	ensure!(index > 0,SelectSmaller0); 

	//------------------------
	let character_index1 = &self.alphabet.binary_search(&character); // speichere an welchem index steht das gesuchte zeichen im alphabet steht 
	let character_index = match character_index1  {
		Ok(x)  => x ,
		Err(_) => return Err(Error::NotInAlphabet), //TODO  element nicht in alphabet 
	};

	//Abfangen dass der Buchstabe nicht index oft vorkommt
    let z = match &self.root{
		Some(x) => x,
		None => return Err(Error::RootUnwrapError),
	};
	if &self.rank(character,z.value.bits().len() as usize).unwrap() < &(index as u64){
		return Err(Error::NotEnough);
	}


	let result = match &self.root {
		Some(x) => x.select(index as u64,character_index,0,self.alphabet.len()-1),
		None => return Err(Error::TempError), //Err("Fehler"),
	};
	match result {
		Some(x)=> return Ok(x+1),
		None => return Err(Error::TempError),
	} 	

}
 
  pub fn rank (&self,character : T,index : usize) -> Result<u64,Error>{
	
    if index<1 {
        return Ok(0);
    }
    let index = index-1;
	let z = match &self.root{
		Some(x) => x,
		None => return Err(Error::RootUnwrapError),
	};
    // Abfangen von fehlerhafter Eingabe, Index ist größer als Sequenz
	if z.value.bits().len() <= index as u64{
		return Err(Error::IndexOutOfBound);
	}	

	//---------------------------------
	let character_index1 = &self.alphabet.binary_search(&character); // speichere an welchem index das gesuchte zeichen im alphabet steht 
	let character_index = match character_index1  {
		Ok(x)  => x ,
		Err(_) => return Ok(0),  //element nicht in alphabet => gib 0 zurück 
	};
	let result = match &self.root {

		Some(x) => (*x).rank(index as u64,character_index,0,&self.alphabet.len()-1),
		None => return Err(Error::NoSuchElement),
	};		
	match result {
				
		Some(x) => return Ok(x),
		None => return Err(Error::NoSuchElement),	
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

  fn select(&self,index : u64,character : &usize, min : usize ,max : usize) -> Option<(u64)>{
	//Blatt erreicht	
	if min==max{return Some(index-1);} // Position wird in Index umgerechnet, da Eingabe mit Position erfolgt

	else{
		if character <= &((max+min)/2){
			
			let result = match &self.left {
				Some(x) => (*x).select(index,character,min,(min+max)/2),
				None => return None,
			};
			let new_index = match result {
				Some(x) => x,
				None => return None,
			};
			return self.value.select_0(new_index+1); //+1 da Index in Position umgerechnet wird
			

		}
		else{
		
			let result = match &self.right{
				Some(x) => (*x).select(index,character,(min+max)/2 +1,max),
				None => return None,
			};
			let new_index = match result {
				Some(x) => x,
				None => return None,
			};
			return self.value.select_1(new_index+1); //+1 da Index in Position umgerechnet wird
		
		}
			    				
		
		

		
	}
	}



  fn rank(&self,index : u64,character : &usize, min : usize ,max : usize) -> Option<u64>{
	//println!("index:{}",index);
	if min == max { return Some(index+1)} //Wenn im blatt 
	else{
		if character <= &((max+min)/2)	    		
		{
			
			let next_index=self.value.rank_0((index) as u64).unwrap();
			match &self.left{
				Some(x)=> return (*x).rank(next_index-1,character,min,(min+max)/2),
				None => return None
			}
		}
		else{
			let next_index=self.value.rank((index) as u64).unwrap();
			match &self.right{
				Some(x)=> return (*x).rank(next_index-1,character,((min+max)/2)+1,max),
				None => return None
			}
	
		}
	}    
	
  }
  
}


























