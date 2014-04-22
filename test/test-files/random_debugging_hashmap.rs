#![feature(default_type_params)]
extern crate rand;
use hopscotch::{HashMap};
use std::hash::{Hasher,Hash};
use std::io::IoResult;
use hashers::HasherSameBucket;

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
#[path="../hashers.rs"]
mod hashers;

fn main(){
    	let mut m = HashMap::with_hasher(HasherSameBucket);
		for i in range(1, 11){
			let bob = m.insert(i,i);
			if !bob{
				println!("Ur insert is fkd")
			}
            //let bib = m.lookup(i);
            //match bib {
            //    Some(x) => println!("val:{}",x),
            //    None => fail!("This sucks!")
            //};
		}
			let rawtable = m.getRawTable();
			for j in range(0u ,16){
				let bucket = rawtable.get_i_bucket(j);
                let key = rawtable.get_key(j);
                let val = rawtable.get_val(j);
				println!("Hash:{}    Hopinfo:{}", bucket.hash, bucket.hop_info);
                println!("Key:{}     Value:{}", key,val);
			}	
            println!("raw table size: {}",rawtable.capacity());

	}
