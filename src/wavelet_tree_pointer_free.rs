use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use snafu::{ensure, Snafu};
use std::fmt::Debug;
use std::hash::Hash;
///custom errors for the tree without pointer
#[derive(Debug, Snafu)]
pub enum Error_Pointer_Free {
    #[snafu(display(
        "Es gibt kein 0tes Element, das erste Element wird mit access(1) angesprochen"
    ))]
    Access0,
    #[snafu(display("Eingabe darf bei select nicht kleiner als 1 sein"))]
    SelectSmaller0,
    #[snafu(display("Index ist größer als die Länge der Sequence"))]
    IndexOutOfBound,
    #[snafu(display("Element nicht gefunden"))]
    NoSuchElement,
    #[snafu(display("Element nicht im Alphabet, Fehler bei select"))]
    NotInAlphabet,
    #[snafu(display("Das Symbol kommt nicht oft genug im Wort vor"))]
    NotEnoughElements,

    #[snafu(display("PlatzhalterError"))]
    TempError,
}

pub struct WaveletTreePointerFree<E> {
    alphabet: Vec<E>,
    bitmap: RankSelect,
    wordlength: usize,
}

impl<T> WaveletTreePointerFree<T>
where
    T: Hash + Clone + Ord + Debug + Copy,
{
    pub fn create<S: Clone + Iterator<Item = T>>(sequence: S) -> WaveletTreePointerFree<T> {
        let seqvec = sequence.clone().collect::<Vec<_>>();
        // berecne Alphabet
        let mut alphabet = Vec::new();
        alphabet.extend(sequence.clone().unique());
        alphabet.sort();

        let alphabet_og = alphabet.clone();
        let wordlength = seqvec.len();
        let mut bit_vec = BitVec::new_fill(false, seqvec.len() as u64);
        let min: usize = 0;
        let mid = compute_breakpoint(&min, &(alphabet.len() - 1));
        //Das Alphabet wird geteilt, die 2. Hälfte wird in alphabet2 gespeichert
        let alphabet2 = alphabet.split_off(mid + 1);
        for x in 0..(seqvec.len()) {
            //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
            if alphabet2.contains(&seqvec[x]) {
                bit_vec.set_bit(x as u64, true)
            }
        }

        let mut vec_collect = Vec::new(); // speichere alle Bitvekoren("die Schnipsel") die als Tupel mit ihrer Position created werden
        vec_collect.push((bit_vec, 1));

        //neue Sequenzen erstellen
        let mut sequence1 = Vec::new();
        let mut sequence2 = Vec::new();
        for (key, group) in &sequence
            .into_iter()
            .group_by(|elem| alphabet.contains(&elem))
        {
            //neue Sequencen werden anhand der Keys gebaut
            if key {
                sequence1.extend(group)
            } else {
                sequence2.extend(group)
            }
        }

        vec_collect.extend(create_vec(sequence1, alphabet.clone(), 2)); //TODO funktion splittet alphabet falsch
        vec_collect.extend(create_vec(sequence2, alphabet2, 3));

        // ordne den zurückgegebenen Vec nach ranking
        vec_collect.sort_by(|a, b| a.1.cmp(&b.1));

        //hänge der Reihe nach die Teile aneinander //TODO lücken füllen, falls sich position springt fülle auf
        let mut bitmap = BitVec::new();
        let mut ebene: usize = 0;
        let mut zeichen: usize = 0;

        for i in 0..vec_collect.len() {
            if compute_stage(&vec_collect[i].1) != ebene {
                //wenn lücke vorliegt
                if zeichen != wordlength {
                    // füge zeichen hinzu wordlength -zeichen
                    //println!("zeichen: {}",zeichen);
                    //println!("wordlength: {}",wordlength);
                    for z in 0..(wordlength - zeichen) {
                        bitmap.push(false);
                        //println!("blaaaaaa");
                    }
                }
                // setzte zeichen zurück und erhöhe ebene

                zeichen = 0;
                ebene = ebene + 1;
            }

            zeichen = zeichen + vec_collect[i].0.len() as usize;
            for x in 0..vec_collect[i].0.len() {
                bitmap.push(vec_collect[i].0.get(x));
            }
        }
        //die bitmap in RankSelect umwandeln
        let bitmap = RankSelect::new(bitmap, 1);
        let x = bitmap.bits().len();
        println!("bitmap größe: {}", x);
        println!("{:?}", alphabet_og);
        WaveletTreePointerFree {
            alphabet: alphabet_og,
            bitmap: bitmap,
            wordlength: wordlength,
        }
    }

    pub fn access(&self, index: usize) -> Result<T, Error_Pointer_Free> {
        //Fehlerbehandlung
        ensure!(index > 0, Access0);
        ensure!(index <= self.wordlength, IndexOutOfBound);

        //---------------------------------------------------------------------------------------
        let alphabet_min = 0;
        let alphabet_max = self.alphabet.len() - 1; //TODO alphabet_max ist 0????
        let right = self.wordlength - 1;
        //println!("wordlength{}",alphabet_max);
        let left = 0;
        let position = self.access_bitmap(index - 1, alphabet_min, alphabet_max, left, right);
        return Ok(self.alphabet[position]);
    }

    //min,max sind alphabet grenzen
    //left,right sind bitmap grenzen
    //index bestimmt die stelle an der man sich in der bitmap befindet
    fn access_bitmap(
        &self,
        index: usize,
        min: usize,
        max: usize,
        left: usize,
        right: usize,
    ) -> usize {
        //println!("");
        //println!("index: {}",index);
        //println!("min: {}",min);
        //println!("max: {}",max);
        if min == max {
            return min;
        } else {
            // falls wir nach links gehen

            if !self.bitmap.get(index as u64) {
                //index in nächster Ebene bestimmen

                //erste ebene
                if left == 0 {
                    //println!("erste Ebene, gehe links");
                    let next_index =
                        left + self.wordlength + self.bitmap.rank_0(index as u64).unwrap() as usize
                            - 1;
                    return self.access_bitmap(
                        next_index,
                        min,
                        compute_breakpoint(&min, &max),
                        left + self.wordlength,
                        left + self.wordlength + self.bitmap.rank_0(right as u64).unwrap() as usize
                            - 1,
                    );
                }
                //nicht erste ebene
                else {
                    //println!("gehe links");
                    let next_index = left
                        + self.wordlength
                        + (self.bitmap.rank_0(index as u64).unwrap()
                            - self.bitmap.rank_0(left as u64 - 1).unwrap())
                            as usize
                        - 1;
                    let new_left = left + self.wordlength;
                    let new_right = left - 1
                        + self.wordlength
                        + self.bitmap.rank_0(right as u64).unwrap() as usize
                        - self.bitmap.rank_0(left as u64 - 1).unwrap() as usize;
                    return self.access_bitmap(
                        next_index,
                        min,
                        compute_breakpoint(&min, &max),
                        new_left,
                        new_right,
                    );
                }
            }
            // falls wir nach rechts gehen
            else {
                //index in nächster Ebene bestimmen

                //erste ebene
                if left == 0 {
                    //println!("erste Ebene, gehe rechts");
                    let next_index =
                        left + self.wordlength + self.bitmap.rank_0(right as u64).unwrap() as usize
                            - 1
                            + self.bitmap.rank_1(index as u64).unwrap() as usize;

                    return self.access_bitmap(
                        next_index,
                        compute_breakpoint(&min, &max) + 1,
                        max,
                        left + self.wordlength + self.bitmap.rank_0(right as u64).unwrap() as usize,
                        right + self.wordlength,
                    );
                }
                //nicht erste ebene
                else {
                    //println!("gehe rechts");
                    //println!("linker index: {}",left);
                    //println!("rechter index: {}",right);
                    let next_index = left
                        + self.wordlength as usize
                        + self.bitmap.rank_0(right as u64).unwrap() as usize
                        - self.bitmap.rank_0(left as u64 - 1).unwrap() as usize
                        + (self.bitmap.rank_1(index as u64).unwrap() as usize
                            - self.bitmap.rank_1(left as u64 - 1).unwrap() as usize)
                        - 1;
                    let new_right = right + &self.wordlength;
                    let new_left =
                        left + self.wordlength + self.bitmap.rank_0(right as u64).unwrap() as usize
                            - self.bitmap.rank_0(left as u64 - 1).unwrap() as usize;
                    return self.access_bitmap(
                        next_index,
                        compute_breakpoint(&min, &max) + 1,
                        max,
                        new_left,
                        new_right,
                    );
                }
            }
        }
    }

    /*

    /// Returns the amount of occurences of the charakter in the Intervall [1..index].
        pub fn rank(&self, character: T, index: usize) -> Result<u64, Error> {

        //TODO Fehlerbehandlung


        //Fehlerbehandulung ende

        return Ok(rank(&self,index,min,max,left,right));
        }


        fn rank_bitmap()




    */
}

// formel : 2^(log(r-l+1)) -1 soll berechnet werden
// index des letzten elements
fn compute_breakpoint(l: &usize, r: &usize) -> usize {
    let potenz: f64 = (r - l + 1) as f64;
    let mut log: f64 = potenz.log2() as f64; // wieder in u32 umwandeln
    let two: f64 = 2.0;
    if two.powf(log) == potenz {
        log = log - 1.0;
    }
    let base: u64 = 2;
    let result = base.pow(log as u32) - 1;
    return l + result as usize;
}

//berechnet anhand des ranks die ebene in der man sich befindet//TODO ändern !!!!
fn compute_stage(p: &usize) -> usize {
    let potenz: f64 = *p as f64;
    let stage: u32 = potenz.log2() as u32;
    return stage as usize;
}

//die Idee ist am Ende eine großen Vektor mit Tuplen aus Bitvekoren und ihrer Stelle im Baum zu haben. Die Wurzel hat dabei das ranking = 1.
fn create_vec<E: Hash + Clone + Ord + Debug>(
    sequence: Vec<E>,
    mut alphabet: Vec<E>,
    ranking: usize,
) -> Vec<(BitVec, usize)> {
    if alphabet.len() <= 1 {
        // im "Blatt" angekommen
        let bit_vec = BitVec::new_fill(true, sequence.len() as u64);
        let mut result = Vec::new();
        result.push((bit_vec, ranking));
        return result;
    } else {
        let mut bit_vec = BitVec::new_fill(false, sequence.len() as u64);
        let min: usize = 0;
        let alphabet_length = alphabet.len();
        println!("alphabet länge: {}", alphabet_length);
        let mid = compute_breakpoint(&min, &(alphabet.len() - 1));
        let alphabet2 = alphabet.split_off(mid + 1);

        //fülle bitmap aus
        for x in 0..(sequence.len()) {
            //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
            if alphabet2.contains(&sequence[x]) {
                bit_vec.set_bit(x as u64, true)
            }
        }

        let mut sequence1 = Vec::new();
        let mut sequence2 = Vec::new();
        for (key, group) in &sequence
            .into_iter()
            .group_by(|elem| alphabet.contains(&elem))
        {
            //neue Sequencen werden anhand der Keys gebaut
            if key {
                sequence1.extend(group)
            } else {
                sequence2.extend(group)
            }
        }

        let left_rank = ranking * 2;
        let right_rank = (ranking * 2) + 1;
        let mut result = Vec::new(); //rückgabe;
        result.push((bit_vec, ranking));
        //füge untere Vektoren hinzu
        result.extend(create_vec(sequence1, alphabet, left_rank));
        result.extend(create_vec(sequence2, alphabet2, right_rank));
        return result;
    }
}
