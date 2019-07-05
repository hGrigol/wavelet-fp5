use serde_json;
use wavelet5::{Error_Pointer_Free, WaveletTreePointerFree};
#[macro_use(matches)]
extern crate matches;
mod common;

#[test]
fn testing_string_access_in_sequence() {
    let tree = common::setup_string_2_2();
    assert_eq!(tree.access(1).unwrap(), 'H');
}
#[test]
fn testing_string_access_in_sequence2() {
    let tree = common::setup_string_2_2();
    assert_eq!(tree.access(37).unwrap(), 'z');
}
#[test]
fn testing_string_access_in_sequence3() {
    let tree = common::setup_string_2_2();
    assert_eq!(tree.access(5).unwrap(), 'o');
}

#[test]
fn testing_string_access_not_in_sequence() {
    let tree = common::setup_string_2_1();

    assert!(matches!(
        tree.access(10000),
        Err(Error_Pointer_Free::IndexOutOfBound)
    ));
}

#[test]
fn testing_string_access_0() {
    let tree = common::setup_string_2_1();

    assert!(matches!(tree.access(0), Err(Error_Pointer_Free::Access0)));
}
