use serde_json;
use std::time::{Duration, Instant};
use wavelet5::{Error, WaveletTree};
#[macro_use(matches)]
extern crate matches;
mod common;
/*
#[test]
fn testing_int_rank() {
    let tree = common::setup_ints();
    let mut vec = Vec::new();
    for x in 0..1000000 {
        vec.push(x % 30);
    }

    let mut average_tree= 0;
    let mut average_linear = 0;
   for p in 0..100{

    let now = Instant::now();
    let i =tree.access(1274).unwrap();
    average_tree += now.elapsed().as_nanos();
    let now = Instant::now();
     let i = vec[1274];
    average_linear += now.elapsed().as_nanos();
  }
  println!("Tree: {:?}, linear: {:?}", average_tree/100, average_linear/100);
}
*/
