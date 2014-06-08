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
    let mut m:HashMap<int,int> = hopscotch::HashMap::with_capacity(60000);
    for i in range(1,50000){
        m.insert(i,i);
        match m.lookup(i){
            //Some(j) => println!("j:{}",*j),
            Some(j) => (),
            None => println!("lookup failed at i:{}",i)
        }
    }
}
