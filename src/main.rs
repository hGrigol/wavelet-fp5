#[macro_use] extern crate derive_builder;
mod wavelet_tree_pointer;
mod wavelet_graph;
fn main() {

    let tree = wavelet_tree_pointer::WaveletTree::create_tree("aba cdef".chars().into_iter());;
	//let z = tree.access(6);
 	//let z= tree.rank('a',2);
	let z = tree.select(' ',1);

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
