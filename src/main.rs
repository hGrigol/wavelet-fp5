use std::mem::size_of_val;
use std::time::{Duration, Instant};
fn main() {
    let mut tempvec = Vec::new();
    for x in 0..1000000 {
        tempvec.push(x % 30);
    }
    let tree = wavelet5::WaveletTree::create(tempvec.into_iter());
    println!("Size of tree: {:?}", size_of_val(&tree));
    let mut vec = Vec::new();
    for x in 0..1000000 {
        vec.push(x % 30);
    }
    println!("Size of vec: {:?}", size_of_val(&vec));
    let mut average_tree = 0;
    let mut average_linear = 0;
    let mut i = 0;
    for p in 0..100 {
        let now = Instant::now();
        i = tree.access(1274).unwrap();
        average_tree += now.elapsed().as_nanos();

        let now = Instant::now();

        i = vec[1274];
        average_linear += now.elapsed().as_nanos();
    }
    println!(
        "Average over 100 Tests for access: Tree: {:?}, linear: {:?}",
        average_tree / 100,
        average_linear / 100
    );

    let mut average_tree = 0;
    let mut average_linear = 0;
    for p in 0..100 {
        let now = Instant::now();
        i = tree.select(9, 1274).unwrap();
        average_tree += now.elapsed().as_nanos();

        let now = Instant::now();
        let mut x = 0;
        for z in 0..vec.len() {
            if vec[z] == 9 {
                x += 1
            };
            if x == 1274 {
                break;
            }
        }
        average_linear += now.elapsed().as_nanos();
    }
    println!(
        "Average over 100 Tests for select: Tree: {:?}, linear: {:?}",
        average_tree / 100,
        average_linear / 100
    );
}
