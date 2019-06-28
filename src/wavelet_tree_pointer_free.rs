use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;


pub struct WaveletTreePointerFree<E>{
    alphabet:Vec<E>,
    bitmap : RankSelect,
	wordlength : usize, 
}



impl<T> WaveletTreePointerFree<T> where T: Hash+Clone+Ord+Debug+Copy {

  pub fn create<S:Clone+Iterator<Item=T>>(sequence: S) -> WaveletTreePointerFree<T>{
	
	let seqvec = sequence.clone().collect::<Vec<_>>();
	// berecne Alphabet    
	let mut alphabet = Vec::new();
    alphabet.extend(sequence.clone().unique());
    alphabet.sort();
    println!("{:?}",alphabet);
	let wordlength = seqvec.len();
	let mut bit_vec = BitVec::new_fill(false,seqvec.len() as u64);
    
	let mid = (alphabet.len()+1)/2;//TODO richtig splitten
    //Das Alphabet wird geteilt, die 2. Hälfte wird in alphabet2 gespeichert
    let alphabet2 = alphabet.split_off(mid);
	for x in 0..(seqvec.len()){
                //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
                if alphabet2.contains(&seqvec[x]){
                bit_vec.set_bit(x as u64,true)}
	}	

	let mut vec_collect = Vec::new(); // speichere alle Bitvekoren("die Schnipsel") die created werden als Tupel mit ihrer Position
	vec_collect.push((bit_vec,1));
	
	//neue Sequenzen erstellen
	let mut sequence1 = Vec::new();
    let mut sequence2 = Vec::new();
	for (key,group) in &sequence.into_iter().group_by(|elem| alphabet.contains(&elem)){ 
    	//neue Sequencen werden anhand der Keys gebaut
    	if key {sequence1.extend(group)} else {sequence2.extend(group)}
    }
	vec_collect.extend(create_vec(sequence1,alphabet.clone(),2));
	vec_collect.extend(create_vec(sequence2,alphabet2,3));
	

	// ordne den zurückgegebenen Vec nach ranking
	vec_collect.sort_by(|a, b| a.1.cmp(&b.1));

	//hänge der Reihe nach die Teile aneinander
	let mut bitmap = BitVec::new();
	for i in 0..vec_collect.len(){
		
		for x in 0..vec_collect[i].0.len(){		
			bitmap.push(vec_collect[i].0.get(x));
		}
	}
	//die bitmap in RankSelect umwandeln
	let bitmap =RankSelect::new(bitmap,1); 		
	
	WaveletTreePointerFree{alphabet, bitmap, wordlength}
  }

 pub fn access (&self,index : usize) -> Result<T, &'static str>{
	//Fehlerbehandlung	
	


	//---------------------------------------------------------------------------------------
	let alphabet_min = 0;
	let alphabet_max = self.alphabet.len()-1;
	let right = self.wordlength-1;
	let left = 0;
	let position = self.access_bitmap(index,alphabet_min,alphabet_max,left,right);
	return Ok(self.alphabet[position])

  }
	
	
	


  //min,max sind alphabet grenzen
  //left,right sind bitmap grenzen 
  //index bestimmt die stelle an der man sich in der bitmap befindet 		
 fn access_bitmap(&self,index :usize, min : usize, max : usize, left :usize , right :usize) -> usize{
	if min == max{return min;}
	else{
		// falls wir nach links gehen 
		if self.bitmap.get(index as u64) {
		
			//index in nächster Ebene bestimmen

			//erste ebene
			if left == 0 {
				let  next_index = left + self.wordlength + self.bitmap.rank_0(index as u64).unwrap() as usize-1;
				return self.access_bitmap(next_index,min,compute_breakpoint(&min,&max)-1,left+ self.wordlength,left+ self.wordlength+self.bitmap.rank_0(right as u64).unwrap() as usize)
			}
			//nicht erste ebene
			else{
				let  next_index = left + self.wordlength  + (self.bitmap.rank_0(index as u64).unwrap()-self.bitmap.rank_0(left as u64-1).unwrap()) as usize-1;
				return self.access_bitmap(next_index,min,compute_breakpoint(&min,&max)-1,left+self.wordlength,left-1+self.wordlength+self.bitmap.rank_0(right as u64).unwrap() as usize - self.bitmap.rank_0(left as u64-1).unwrap()  as usize);
			}
			
					

		}
		// falls wir nach rechts gehen //TODO returns überarbeiten
		else{
			//index in nächster Ebene bestimmen

			//erste ebene
			if left == 0 {
				let  next_index =  self.wordlength + self.bitmap.rank_0(right as u64).unwrap() as usize-1 +self.bitmap.rank_1(index  as u64).unwrap() as usize;
				return self.access_bitmap(next_index,compute_breakpoint(&min,&max),max,left+ self.wordlength+self.bitmap.rank_0(right as u64).unwrap() as usize,right+self.wordlength)}
			//nicht erste ebene
			else{
				let next_index = left + self.wordlength  as usize + self.bitmap.rank_0(right as u64).unwrap() as usize+(self.bitmap.rank_1(index as u64).unwrap() as usize-self.bitmap.rank_1(left as u64-1).unwrap() as usize)-1;
				return self.access_bitmap(next_index,compute_breakpoint(&min,&max),max,left+&self.wordlength,left-1+self.wordlength+self.bitmap.rank_0(right as u64).unwrap() as usize-self.bitmap.rank_0(left as u64-1).unwrap() as usize)}	
		}
		
	 		

	}

  }











}


// formel : 2^(log(r-l+1)) -1 soll berechnet werden
  fn compute_breakpoint(l : &usize , r: &usize)-> usize{
	let potenz :f64 = (r-l+1) as f64;
	let log : u32= potenz.log2()as u32 -1 ;
	let base : u64 = 2;	
	let result = base.pow(log) -1;
	return result as usize
  }












//die Idee ist am Ende eine großen Vektor mit Tuplen aus Bitvekoren und ihrer Stelle im Baum zu haben. Die Wurzel hat dabei das ranking = 1. 
  fn create_vec<E:Hash+Clone+Ord+Debug>(sequence : Vec<E>,mut alphabet : Vec<E>, ranking : usize) -> Vec<(BitVec,usize)>{
	if alphabet.len()==1{  // im "Blatt" angekommen
		let bit_vec = BitVec::new_fill(true,sequence.len() as u64);
		let mut result = Vec::new();		
		result.push((bit_vec,ranking));
		return result
	}
	else{	
		let mut bit_vec = BitVec::new_fill(false,sequence.len() as u64);
    	let mid = (alphabet.len()+1)/2;//TODO richtig splitten //Das Alphabet wird geteilt, die 2. Hälfte wird in alphabet2 gespeichert
    	let alphabet2 = alphabet.split_off(mid);
		
		//fülle bitmap aus
		for x in 0..(sequence.len()){
                //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
                if alphabet2.contains(&sequence[x]){
                bit_vec.set_bit(x as u64,true)}
		}

		let mut sequence1 = Vec::new();
    	let mut sequence2 = Vec::new();
		for (key,group) in &sequence.into_iter().group_by(|elem| alphabet.contains(&elem)){ 
    		//neue Sequencen werden anhand der Keys gebaut
    		if key {sequence1.extend(group)} else {sequence2.extend(group)}
    	}
				
		
		let left_rank  =  ranking*2;
		let right_rank = (ranking*2)+1;
		let mut result = Vec::new(); //rückgabe;
		result.push((bit_vec,ranking));
		//füge untere Vektoren hinzu     
		result.extend(create_vec(sequence1,alphabet,left_rank));
		result.extend(create_vec(sequence2,alphabet2,right_rank));
		return result
	}
  }


