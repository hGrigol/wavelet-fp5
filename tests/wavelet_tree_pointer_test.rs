use wavelet5::{Error, WaveletTree};
use ::serde;
use serde_json;
#[macro_use(matches)]
extern crate matches;
mod common;

#[test]
fn testing_string_serialize(){
    let tree =common::setup_string1();
    let serialized = serde_json::to_string(&tree).unwrap();
    let tree2: WaveletTree<char> = serde_json::from_str(&serialized).unwrap();
    assert!(matches!(tree,tree2));
}

#[test]
fn testing_string_select_in_alpabet_lower_case(){
    let tree =common::setup_string1();
    assert_eq!(tree.select('a',1).unwrap(),2);


}

#[test]
fn testing_string_select_not_in_alphabet(){
    let tree =common::setup_string1();

   assert!(matches!(tree.select('Z',1),Err(Error::NotInAlphabet)));
}


#[test]
fn testing_string_select_white_space(){
    let tree =common::setup_string1();
    assert_eq!(tree.select(' ',3).unwrap(),14);

}


#[test]
fn testing_string_select_in_alphabet_upper_case(){
    let tree =common::setup_string1();
    assert_eq!(tree.select('S',1).unwrap(),34);

}

#[test]
fn testing_string_select_index_to_high(){
    let tree =common::setup_string1();
    assert!(matches!(tree.select('a',20),Err(Error::NotEnough)));

}


#[test]
fn testing_string_access_in_sequence(){
    let tree =common::setup_string1();
    assert_eq!(tree.access(1).unwrap(),'H');


}

#[test]
fn testing_string_access_not_in_sequence(){
    let tree =common::setup_string1();

    assert!(matches!(tree.access(10000),Err(Error::IndexOutOfBound)));
}


#[test]
fn testing_string_access_0(){
    let tree =common::setup_string1();

    assert!(matches!(tree.access(0),Err(Error::Access0)));
}
