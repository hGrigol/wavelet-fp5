#[macro_use] extern crate derive_builder;
mod wavelet_tree_pointer;
fn main() {
    let tree = wavelet_tree_pointer::WaveletTree::create_tree("Hallo Ich bin niklas".chars());;
   // match tree.access()
}
