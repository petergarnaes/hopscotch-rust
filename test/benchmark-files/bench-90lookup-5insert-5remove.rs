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

static HASHMAP_CAPACITY:uint = 32768;
static OPERATIONS:uint = 500;
static AVG_SIZE:u64 = 10;

fn insert_lookup_remove_robin(load:uint,ops:uint)->u64{
    let mut m = HashMap::with_capacity(HASHMAP_CAPACITY);
    for i in range(1,load){
        m.insert(i,i+1);
    }
    let start = precise_time_ns();
    for i in range(load,load+ops){
        m.insert(i,i+1);
        for _ in range(0,18){
            m.find(&i);
        }
        m.remove(&i);
    }
    let end = precise_time_ns();
    end - start
}
fn insert_lookup_remove_hopscotch(load:uint,ops:uint)->u64{
    let mut m = hopscotch::HashMap::with_capacity(HASHMAP_CAPACITY);
    for i in range(1,load){
        m.insert(i,i+1);
    }
    let start = precise_time_ns();
    for i in range(load,load+ops){
        m.insert(i,i+1);
        for _ in range(0,18){
            m.lookup(i);
        }
        m.remove(i);
    }
    let end = precise_time_ns();
    end - start
}

fn main(){
    // datapoints are the loads 0.3,0.4,0.5,0.6,0.7,0.8,0.9 calculated with 131072
    let data_point:Vec<uint> = vec!(9830,11469,13107,14746,16384,18022,19660,21299,22937,24576,26214);
    let mut result_hopscotch:Vec<u64> = Vec::with_capacity(12);
    let mut result_robin:Vec<u64> = Vec::with_capacity(12);
    let load_factor = vec!(0.3,0.35,0.4,0.45,0.5,0.55,0.6,0.65,0.7,0.75,0.8);
    let mut it = 0;
    for i in data_point.iter(){
        println!("load factor:{}",*load_factor.get(it));
        let mut sum = 0u64;
        for _ in range(0,AVG_SIZE){
            sum += insert_lookup_remove_hopscotch(*i,OPERATIONS);
        }
        let time_hopscotch = sum/AVG_SIZE;
        result_hopscotch.push(time_hopscotch);
        let mut sum2 = 0u64;
        for _ in range(0,AVG_SIZE){
            sum2 += insert_lookup_remove_robin(*i,OPERATIONS);
        }
        let time_robin = sum2/AVG_SIZE;
        result_robin.push(time_robin);
        it += 1;
    }
    let mut file = File::create(&Path::new("output-files/bench-90lookup-5insert-5remove"));
    for i in range(0u,11u){
        let d = load_factor.get(i).to_str().append(" ");
        let h = result_hopscotch.get(i).to_str().append(" ");
        let r = result_robin.get(i).to_str().append("\n");
        let line = d.into_bytes()+h.into_bytes()+r.into_bytes();
        file.write(line.as_slice());
    }
}
