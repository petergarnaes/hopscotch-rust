#![feature(default_type_params)]
extern crate rand;
use hopscotch::{HashMap};
use std::hash::{Hasher,Hash};
use std::io::IoResult;

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;

struct HasherZeroAndUp;

impl Hasher<HashZeroAndUp> for HasherZeroAndUp{
    fn hash<T:Hash<HashZeroAndUp>>(&self,value:&T)->u64{
        let mut state = HashZeroAndUp{hash:0};
        value.hash(&mut state);
        state.hash
    }
}

struct HashZeroAndUp{
    hash: u64,
}

impl Writer for HashZeroAndUp {
    fn write(&mut self, buf: &[u8]) -> IoResult<()>{
        for byte in buf.iter(){
            self.hash += *byte as u64
        }
        //self.hash = 1u64;
        Ok(())
    }
}

struct HasherSameBucket;

impl Hasher<HashSameBucket> for HasherSameBucket{
    fn hash<T:Hash<HashSameBucket>>(&self,value:&T)->u64{
        let mut state = HashSameBucket{hash:0};
        value.hash(&mut state);
        state.hash
    }
}

struct HashSameBucket{
    hash: u64,
}

impl Writer for HashSameBucket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()>{
        let mut a = 0u64;
        for byte in buf.iter(){
            a += *byte as u64
        }
        let b = 32u64 * a;
        self.hash = 33u64 + b;
        //println!("hash:{}",self.hash);
        //self.hash = 1u64;
        Ok(())
    }
}

fn main(){
    	let mut m = HashMap::with_hasher(HasherSameBucket);
		for i in range(1, 11){
			let bob = m.insert(i,i);
			if !bob{
				println!("Ur insert is fkd")
			}
            let bib = m.lookup(i);
            match bib {
                Some(x) => println!("val:{}",x),
                None => fail!("This sucks!")
            };
		}
			let rawtable = m.getRawTable();
			for j in range(0u ,16){
				let bucket = rawtable.get_i_bucket(j);
				println!("Hash:{}    Hopinfo:{}", bucket.hash, bucket.hop_info);
			}	
            println!("raw table size: {}",rawtable.capacity());

	}
