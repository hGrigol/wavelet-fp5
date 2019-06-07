#[macro_use] extern crate derive_builder;
mod wavelet_tree_pointer;
fn main() {
    let tree = wavelet_tree_pointer::WaveletTree::create_tree("aba cdef".chars());;
	let z = tree.access(10);
//	let z= tree.rank('a',3);
//	let z = tree.select('a',3);
	match z{
	Ok(x) => println!("a kommt {} mal vor",x),
	Err(z)=> println!("{}", z),
	}

//	let z= tree.rank('b',5);
//	match z{
//	Ok(x) => println!("b kommt {} mal vor",x),
//	Err(_)=> println!("a kommt nicht vor"),
//	}
//	let z= tree.rank('c',5);
//	match z{
//	Ok(x) => println!("c kommt {} mal vor",x),
//	Err(_)=> println!("a kommt nicht vor"),
//	}
//	let z= tree.rank('d',5);
//	match z{
//	Ok(x) => println!("d kommt {} mal vor",x),
//	Err(_)=> println!("a kommt nicht vor"),
//	}
//	let z= tree.rank('e',5);
//	match z{
//	Ok(x) => println!("e kommt {} mal vor",x),
//	Err(_)=> println!("g kommt nicht vor"),
//	}
//	let z= tree.rank('f',5);
//  match z{
//	Ok(x) => println!("f kommt {} mal vor",x),
//	Err(_)=> println!("a kommt nicht vor"),
//	}

}
