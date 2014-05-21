#![feature(default_type_params)]
extern crate rand;
#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
#[path="../hashers.rs"]
mod hashers;

fn main(){
    let h = hashers::HasherSameBucket::new(12,128);
    let mut map = hopscotch::HashMap::with_capacity_and_hasher(64,h);
    for i in range(1,21){
        map.insert(i,i+1);
    }
    for j in range(0u,map.getRawTable().capacity()){
        let raw_table = map.getRawTable();
        let bucket = raw_table.get_i_bucket(j);
        let key = raw_table.get_key(j);
        let val = raw_table.get_val(j);
        println!("{} Hash:{} Hop info:{}",j,bucket.hash,bucket.hop_info);
        println!("Key:{} Val:{}",key,val);
    }
    map.resize();
    for j in range(0u,map.getRawTable().capacity()){
        let raw_table = map.getRawTable();
        let bucket = raw_table.get_i_bucket(j);
        let key = raw_table.get_key(j);
        let val = raw_table.get_val(j);
        println!("{} Hash:{} Hop info:{}",j,bucket.hash,bucket.hop_info);
        println!("Key:{} Val:{}",key,val);
    }
}
