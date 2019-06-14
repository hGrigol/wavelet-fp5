mod common;
#[test]
fn testing_string_select_in_alpabet_lower_case(){
    let tree =common::setup_string1();
    assert_eq!(tree.select('a',1).unwrap(),2);


}

#[test]
fn testing_string_select_not_in_alphabet(){
    let tree =common::setup_string1();

    assert_eq!(tree.select('Z',1),Err("Element nicht im Alphabet, Fehler bei select"));


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
fn testing_string_select5(){
    let tree =common::setup_string1();
    assert_eq!(tree.select('a',20),Err("Das Symbol kommt nicht oft genug im Wort vor"));

}
