#![feature(default_type_params)]
extern crate rand;
#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
#[path="../hashers.rs"]
mod hashers;

fn main(){
    let h = hashers::HasherFindCloserBucket::new(10,64);
	let mut x = hopscotch::HashMap::new();
	x.insert(123, 11);
    let a = match x.lookup(123){
        Some(var) => var.clone(),
        None => fail!("Shait!")
    };
	let b = match x.remove(123) {
        Some(var) => var.clone(),
        None => fail!("Shait2!")
    };
	println!("{}", a == b);
    let mut m = hopscotch::HashMap::with_capacity_and_hasher(64,h);
    for i in range(1,35){
        m.insert(i,i+1);
        for j in range(0u,m.get_rawtable().capacity()){
            let rawtable = m.get_rawtable();
            let bucket = rawtable.get_i_bucket(j);
            let key = rawtable.get_key(j);
            let val = rawtable.get_val(j);
            println!("{} Hash:{} hop _info:{}",j,bucket.hash,bucket.hop_info);
            println!("key:{} value:{}",key,val);
        }
        if i != 1 {
            for k in range(1,i){
                println!("k:{}",k);
                match m.lookup(k){
                    Some(var) => assert!(*var == k+1),
                    None => fail!("Fak!")
                }
            }
        }
    }
    let rawtable = m.get_rawtable();
    for j in range(1u,64u){
        let bucket = rawtable.get_i_bucket(j);
        let key = rawtable.get_key(j);
        let val = rawtable.get_val(j);
        println!("{} Hash:{} hop _info:{}",j,bucket.hash,bucket.hop_info);
        println!("key:{} value:{}",key,val);
    }
}
