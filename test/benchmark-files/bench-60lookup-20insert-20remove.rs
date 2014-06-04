#![feature(default_type_params)]
extern crate rand;
extern crate time;
extern crate collections;

use collections::HashMap;
use time::precise_time_ns;
use std::io::File;

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
#[path="../hashers.rs"]
mod hashers;

static HASHMAP_CAPACITY:uint = 40000;

fn insert_lookup_remove_robin(load:int)->u64{
    let mut m = HashMap::with_capacity(HASHMAP_CAPACITY);
    // The inserts will fill the table to the desired loadfactor, and the 
    // other operations will scale accordingly
    let lookups = load*3;
    let inserts = load;
    let start = precise_time_ns();
    for i in range(1,inserts){
        m.insert(i,i+1);
    }
    for _ in range(0,3){
        for j in range(1,inserts){
            m.find(&j);
        }
    }
    for k in range(1,inserts){
        m.remove(&k);
    }
    let end = precise_time_ns();
    end - start
}
fn insert_lookup_remove_hopscotch(load:int)->u64{
    let mut m = hopscotch::HashMap::with_capacity(HASHMAP_CAPACITY);
    // The inserts will fill the table to the desired loadfactor, and the 
    // other operations will scale accordingly
    let lookups = load*3;
    let inserts = load;
    let start = precise_time_ns();
    for i in range(1,inserts){
        m.insert(i,i+1);
    }
    for _ in range(0,3){
        for j in range(1,inserts){
            m.lookup(j);
        }
    }
    for k in range(1,inserts){
        m.remove(k);
    }
    let end = precise_time_ns();
    end - start
}

fn main(){
    // datapoints are the loads 0.3,0.4,0.5,0.6,0.7,0.8,0.9 calculated with 40000
    let data_point:Vec<int> = vec!(12000,16000,20000,24000,28000,32000,36000);
    let mut result_hopscotch:Vec<u64> = Vec::with_capacity(7);
    let mut result_robin:Vec<u64> = Vec::with_capacity(7);
    for i in data_point.iter(){
        let time_hopscotch = insert_lookup_remove_hopscotch(*i);
        result_hopscotch.push(time_hopscotch);
        let time_robin = insert_lookup_remove_robin(*i);
        result_robin.push(time_robin);
    }
    let mut file = File::create(&Path::new("output-files/bench-60lookup-20insert-20remove"));
    let load_factor = vec!(0.3,0.4,0.5,0.6,0.7,0.8,0.9);
    for i in range(0u,7u){
        let d = load_factor.get(i).to_str().append(" ");
        let h = result_hopscotch.get(i).to_str().append(" ");
        let r = result_robin.get(i).to_str().append("\n");
        let line = d.into_bytes()+h.into_bytes()+r.into_bytes();
        file.write(line.as_slice());
    }
}
