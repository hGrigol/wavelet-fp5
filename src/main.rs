#[macro_use] extern crate derive_builder;
mod wavelet_tree_pointer;
fn main() {
    let tree = wavelet_tree_pointer::WaveletTree::create_tree("Ha".chars());;
	//let c = tree.access(5);
	//let z= tree.rank('a',5);
	//match z{
	//Ok(x) => println!("a an Stelle {}",x),
	//Err(_)=> println!("a kommt nicht vor"),
	//}
   // match tree.access()
}
