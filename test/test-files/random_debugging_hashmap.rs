#![feature(default_type_params)]
extern crate rand;
use hopscotch::{HashMap};

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;

fn main(){
    	let mut m:HashMap<int,int> = HashMap::with_capacity(31);
		for i in range(0, 10){
			let bob = m.insert(i,i);
			if !bob{
				println!("Ur insert is fkd")
			}
		}
			let rawtable = m.getRawTable();
			for j in range(0u ,32){
				let bucket = rawtable.get_i_bucket(j);
				println!("Hash:{}    Hopinfo:{}", bucket.hash, bucket.hop_info);
			}	

	}
