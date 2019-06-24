use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use snafu::{ensure, Snafu};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(
        "Es gibt kein 0tes Element, das erste Element wird mit access(1) angesprochen"
    ))]
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
pub struct WaveletTree<E> {
    alphabet: Vec<E>,
    root: Option<Box<BinNode>>,
}
#[derive(Serialize, Deserialize)]
pub struct BinNode {
    value: RankSelect,
    left: Option<Box<BinNode>>,
    right: Option<Box<BinNode>>,
}
pub struct Iterhelper<E> {
    position: usize,
    tree: WaveletTree<E>,
}

impl<'de, T> WaveletTree<T>
where
    T: Hash + Clone + Ord + Debug + Copy + Serialize + Deserialize<'de>,
{
    pub fn create_tree<S: Clone + Iterator<Item = T>>(sequence: S) -> WaveletTree<T> {
        let seqvec = sequence.clone().collect::<Vec<_>>();
        let mut alphabet: Vec<T> = Vec::new();
        alphabet.extend(sequence.unique());
        alphabet.sort();
        let alphslice = &alphabet[..];
        WaveletTree {
            root: Some(Box::new(BinNode::create_node(alphslice, seqvec))),
            alphabet: alphabet,
        }
    }

    pub fn access(&self, index: usize) -> Result<T, Error> {
        ensure!(index > 0, Access0);
        // Abfangen von fehlerhafter Eingabe, Index ist größer als Sequenz
        let z = match &self.root {
            Some(x) => x,
            None => return Err(Error::RootUnwrapError),
        };
        ensure!(z.len() >= index as u64, IndexOutOfBound);

        //-------------------------------------------

        let z = match &self.root {
            Some(x) => x.access((index - 1) as u64, 0, self.alphabet.len() - 1),
            None => return Err(Error::RootUnwrapError), //TODO snafu Fehler implementieren
        };
        match z {
            Some(x) => Ok(self.alphabet[x]),
            None => return Err(Error::NoSuchElement),
        }
    }

    pub fn select(&self, character: T, index: usize) -> Result<u64, Error> {
        // Abfangen von fehlerhafter Eingabe, Index darf hier nicht 0 sein
        ensure!(index > 0, SelectSmaller0);

        //------------------------
        let character_index1 = &self.alphabet.binary_search(&character); // speichere an welchem index steht das gesuchte zeichen im alphabet steht
        let character_index = match character_index1 {
            Ok(x) => x,
            Err(_) => return Err(Error::NotInAlphabet), //TODO  element nicht in alphabet
        };

        //Abfangen dass der Buchstabe nicht index oft vorkommt
        let z = match &self.root {
            Some(x) => x,
            None => return Err(Error::RootUnwrapError),
        };

        if &self.rank(character, z.len() as usize).unwrap() < &(index as u64) {
            return Err(Error::NotEnough);
        }

        let result = match &self.root {
            Some(x) => x.select(index as u64, character_index, 0, self.alphabet.len() - 1),
            None => return Err(Error::TempError), //Err("Fehler"),
        };
        match result {
            Some(x) => return Ok(x + 1),
            None => return Err(Error::TempError),
        }
    }

    pub fn rank(&self, character: T, index: usize) -> Result<u64, Error> {
        if index < 1 {
            return Ok(0);
        }
        let index = index - 1;
        let z = match &self.root {
            Some(x) => x,
            None => return Err(Error::RootUnwrapError),
        };
        // Abfangen von fehlerhafter Eingabe, Index ist größer als Sequenz
        ensure!(z.len() > index as u64, IndexOutOfBound);

        //---------------------------------
        let character_index1 = &self.alphabet.binary_search(&character); // speichere an welchem index das gesuchte zeichen im alphabet steht
        let character_index = match character_index1 {
            Ok(x) => x,
            Err(_) => return Ok(0), //element nicht in alphabet => gib 0 zurück
        };
        let result = match &self.root {
            Some(x) => (*x).rank(index as u64, character_index, 0, &self.alphabet.len() - 1),
            None => return Err(Error::NoSuchElement),
        };
        match result {
            Some(x) => return Ok(x),
            None => return Err(Error::NoSuchElement),
        }
    }

    pub fn rebuild(&self) -> Result<Vec<T>, Error> {
        let mut result: Vec<T> = Vec::new();
        let root = match &self.root {
            Some(x) => x,
            None => return Err(Error::RootUnwrapError),
        };
        let len = root.len();
        for x in 1..(len + 1) {
            match self.access(x as usize) {
                Ok(z) => result.push(z),
                Err(_) => return Err(Error::NoSuchElement),
            };
        }
        Ok(result)
    }

    pub fn len(&self) -> Result<u64, Error> {
        let root = match &self.root {
            Some(x) => x,
            None => return Err(Error::RootUnwrapError),
        };
        Ok(root.len())
    }
}

impl<'de, T> IntoIterator for WaveletTree<T>
where
    T: Hash + Clone + Ord + Debug + Copy + Serialize + Deserialize<'de>,
{
    type Item = T;
    type IntoIter = Iterhelper<T>;
    fn into_iter(self) -> Self::IntoIter {
        Iterhelper::new(self)
    }
}

impl BinNode {
    fn create_node<E: Hash + Clone + Ord + Debug>(alphabet: &[E], sequence: Vec<E>) -> BinNode {
        let count = sequence.len();
        if alphabet.len() <= 1 {
            let value = BitVec::new_fill(true, count as u64);
            BinNode {
                value: RankSelect::new(value, 1),
                left: None,
                right: None,
            }
        } else {
            let mut value = BitVec::new_fill(false, count as u64);
            let mid = (alphabet.len() + 1) / 2;
            //Das Alphabet wird geteilt, die 2. Hälfte wird in alphabet2 gespeichert
            let (alphabet1, alphabet2) = alphabet.split_at(mid); //TODO eigentlich mid+1,aber dann stack overflow?
                                                                 //Die Sequenzen für den nächsten Schritt

            let mut sequence1 = Vec::new();
            let mut sequence2 = Vec::new();
            //Es werden alle Elemente der Sequenz durchegangen
            for x in 0..(sequence.len()) {
                //wenn sie in der 2. Hälfte des Alphabets sind wird ihr Eintrag in der Bitmap auf 1 gesetzt
                if alphabet2.contains(&sequence[x]) {
                    value.set_bit(x as u64, true)
                }
            }
            //Group_by teilt in Gruppen key ist true wenn Zeichen in alphabet1, sonst false
            for (key, group) in &sequence
                .into_iter()
                .group_by(|elem| alphabet1.contains(&elem))
            {
                //neue Sequencen werden anhand der Keys gebaut
                if key {
                    sequence1.extend(group)
                } else {
                    sequence2.extend(group)
                }
            }
            BinNode {
                value: RankSelect::new(value, 1),
                left: Some(Box::new(BinNode::create_node(alphabet1, sequence1))),
                right: Some(Box::new(BinNode::create_node(alphabet2, sequence2))),
            }
        }
    }

    fn access(&self, index: u64, min: usize, max: usize) -> Option<usize> {
        if min == max {
            return Some(min);
        } else {
            if self.value.get((index) as u64) {
                let next_index = self.value.rank((index) as u64).unwrap();
                match &self.right {
                    Some(x) => return (*x).access(next_index - 1, 1 + (min + max) / 2, max),
                    None => return None,
                }
            } else {
                let next_index = self.value.rank_0((index) as u64).unwrap();
                match &self.left {
                    Some(x) => return (*x).access(next_index - 1, min, (min + max) / 2),
                    None => return None,
                }
            }
        }
    }

    fn select(&self, index: u64, character: &usize, min: usize, max: usize) -> Option<(u64)> {
        //Blatt erreicht
        if min == max {
            return Some(index - 1);
        }
        // Position wird in Index umgerechnet, da Eingabe mit Position erfolgt
        else {
            if character <= &((max + min) / 2) {
                let result = match &self.left {
                    Some(x) => (*x).select(index, character, min, (min + max) / 2),
                    None => return None,
                };
                let new_index = match result {
                    Some(x) => x,
                    None => return None,
                };
                return self.value.select_0(new_index + 1); //+1 da Index in Position umgerechnet wird
            } else {
                let result = match &self.right {
                    Some(x) => (*x).select(index, character, (min + max) / 2 + 1, max),
                    None => return None,
                };
                let new_index = match result {
                    Some(x) => x,
                    None => return None,
                };
                return self.value.select_1(new_index + 1); //+1 da Index in Position umgerechnet wird
            }
        }
    }

    fn rank(&self, index: u64, character: &usize, min: usize, max: usize) -> Option<u64> {
        if min == max {
            return Some(index + 1);
        }
        //Wenn nicht im blatt
        else {
            if character <= &((max + min) / 2) {
                let next_index = self.value.rank_0((index) as u64).unwrap();
                match &self.left {
                    Some(x) => return (*x).rank(next_index - 1, character, min, (min + max) / 2),
                    None => return None,
                }
            } else {
                let next_index = self.value.rank((index) as u64).unwrap();
                match &self.right {
                    Some(x) => {
                        return (*x).rank(next_index - 1, character, ((min + max) / 2) + 1, max);
                    }
                    None => return None,
                }
            }
        }
    }

    fn len(&self) -> u64 {
        self.value.bits().len()
    }
}

impl<'de, E> Iterhelper<E>
where
    E: Hash + Clone + Ord + Debug + Copy + Serialize + Deserialize<'de>,
{
    fn new(tree: WaveletTree<E>) -> Iterhelper<E> {
        Iterhelper {
            position: 0,
            tree: tree,
        }
    }
}

impl<'de, E> Iterator for Iterhelper<E>
where
    E: Hash + Clone + Ord + Debug + Copy + Serialize + Deserialize<'de>,
{
    type Item = E;
    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        let len = match self.tree.len() {
            Ok(x) => x,
            Err(_) => return None,
        };

        if self.position <= len as usize {
            match self.tree.access(self.position) {
                Ok(x) => return Some(x),
                Err(_) => return None,
            };
        } else {
            None
        }
    }
}
