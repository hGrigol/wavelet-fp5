#[macro_use] extern crate derive_builder;
mod wavelet_tree_pointer;
fn main() {
    let tree = wavelet_tree_pointer::WaveletTree::create_tree("aaaakkkeeeaallleemdaat".chars());;
//	let z = tree.access(21);
//	let z= tree.rank('a',15);
	let z= tree.select('l',0);
	match z{
	Ok(x) => println!("a kommt {} mal vor",x),
	Err(_)=> println!("a kommt nicht vor"),
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
