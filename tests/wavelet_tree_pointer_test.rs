use serde_json;
use wavelet5::{Error, WaveletTree};
use std::iter;
#[macro_use(matches)]
extern crate matches;
mod common;

#[test]
fn testing_string_rebuild() {
    let tree = common::setup_string1();
    let vec = tree.rebuild();
    let vec2: Vec<char> = "Hallo Ich bin ein Test für einen Satz".chars().collect();
    let b = &vec2[1..4];
    // println!("{:?}", b);
    // println!("{:?}", vec);
    assert!(matches!(vec, vec2));
}
#[test]
fn testin_string_iterator_in_tree() {
    let tree = common::setup_string1();
    let mut vec = Vec::new();
    for x in tree.into_iter() {
        vec.push(x);
    }
    assert_eq!(tree.select('a', 1).unwrap(), 2);
    println!("{:?}", vec);
}
#[test]
fn testing_string_serialize() {
    let tree = common::setup_string1();
    let serialized = serde_json::to_string(&tree).unwrap();
    let _tree2: WaveletTree<char> = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(tree, _tree2));
}

#[test]
fn testing_string_select_in_alpabet_lower_case() {
    let tree = common::setup_string1();
    assert_eq!(tree.select('a', 1).unwrap(), 2);
}

#[test]
fn testing_string_select_not_in_alphabet() {
    let tree = common::setup_string1();

    assert!(matches!(tree.select('Z', 1), Err(Error::NotInAlphabet)));
}

#[test]
fn testing_string_select_white_space() {
    let tree = common::setup_string1();
    assert_eq!(tree.select(' ', 3).unwrap(), 14);
    assert_eq!(tree.access(14).unwrap(), ' ');
}

#[test]
fn testing_string_select_in_alphabet_upper_case() {
    let tree = common::setup_string1();
    assert_eq!(tree.select('S', 1).unwrap(), 34);
}

#[test]
fn testing_string_select_index_to_high() {
    let tree = common::setup_string1();
    assert!(matches!(
        tree.select('a', 20),
        Err(Error::NotEnoughElements)
    ));
}

#[test]
fn testing_string_access_in_sequence() {
    let tree = common::setup_string1();
    assert_eq!(tree.access(1).unwrap(), 'H');
}

#[test]
fn testing_string_access_not_in_sequence() {
    let tree = common::setup_string1();

    assert!(matches!(tree.access(10000), Err(Error::IndexOutOfBound)));
}

#[test]
fn testing_string_access_0() {
    let tree = common::setup_string1();
    assert!(matches!(tree.access(0), Err(Error::Access0)));
}
#[test]
fn testing_string_access_index() {
    let tree = common::setup_string1();
    assert!(matches!(tree[1], 'H'));
}

#[test]
fn testing_string_rank() {
    let tree = common::setup_string1();
    assert_eq!(tree.rank('l', 4).unwrap(), 2);
}

#[test]
fn testing_int_rebuild() {
    let tree = common::setup_ints();
    let vec = tree.rebuild();
    let mut vec2 = Vec::new();
    for x in 0..100 {
        vec2.push(x);
    }
    assert!(matches!(vec, vec2));
}
#[test]
fn testin_int_iterator_in_tree() {
    let tree = common::setup_ints();
    let mut vec = Vec::new();
    for x in tree.into_iter() {
        vec.push(x);
    }
    assert_eq!(tree.select(1, 1).unwrap(), 2);
    println!("{:?}", vec);
}

#[test]
fn testing_int_serialize() {
    let tree = common::setup_ints();
    let serialized = serde_json::to_string(&tree).unwrap();
    let _tree2: WaveletTree<u64> = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(tree, _tree2));
}

#[test]
fn testing_int_select_in_alpabet() {
    let tree = common::setup_ints();
    assert_eq!(tree.select(5, 1).unwrap(), 6);
}

#[test]
fn testing_int_select_not_in_alphabet() {
    let tree = common::setup_ints();

    assert!(matches!(
        tree.select(14500000075, 1),
        Err(Error::NotInAlphabet)
    ));
}

#[test]
fn testing_int_select_in_alphabet() {
    let tree = common::setup_ints();
    assert_eq!(tree.select(9, 1).unwrap(), 10);
}

#[test]
fn testing_int_select_index_to_high() {
    let tree = common::setup_ints();
    assert!(matches!(
        tree.select(3, 1000000000),
        Err(Error::NotEnoughElements)
    ));
}

#[test]
fn testing_int_access_in_sequence() {
    let tree = common::setup_ints();
    assert_eq!(tree.access(9).unwrap(), 8);
}

#[test]
fn testing_int_access_not_in_sequence() {
    let tree = common::setup_ints();

    assert!(matches!(
        tree.access(100000000),
        Err(Error::IndexOutOfBound)
    ));
}

#[test]
fn testing_int_access_0() {
    let tree = common::setup_ints();
    assert!(matches!(tree.access(0), Err(Error::Access0)));
}

#[test]
fn testing_int_access_index() {
    let tree = common::setup_ints();
    assert!(matches!(tree[1], 0));
}

#[test]
fn testing_int_rank() {
    let tree = common::setup_ints();
    assert_eq!(tree.rank(4, 10).unwrap(), 1);
}

#[test]
#[should_panic]
fn testing_empty_iterator(){
  let mut nope = iter::empty::<i32>();
  let tree = wavelet5::WaveletTree::create_tree(nope);
}

