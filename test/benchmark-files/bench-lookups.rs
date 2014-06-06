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

//static HASHMAP_CAPACITY:uint = 32768;
//static HASHMAP_CAPACITY:uint = 4096;
static HASHMAP_CAPACITY:uint = 80000;
static OPERATIONS:u64 = 500;
static AVG_SIZE:u64 = 10;

fn insert_lookup_remove_robin(load:uint)->u64{
    let mut m = HashMap::with_capacity(HASHMAP_CAPACITY);
 	 for i in range(0,load){
        m.insert(i,i+1);
    }
    let start = precise_time_ns();
    for i in range(0,load){
        m.find(&i);
    }
    let end = precise_time_ns();
    end - start
}
fn insert_lookup_remove_hopscotch(load:uint)->u64{
    let mut m = hopscotch::HashMap::with_capacity(HASHMAP_CAPACITY);
   	for i in range(0,load){
        m.insert(i,i+1);
    }    
	let start = precise_time_ns();
    for i in range(0,load){
        m.lookup(i);
    }
    let end = precise_time_ns();
    end - start
}

fn main(){
    // datapoints are the loads 0.3,0.4,0.5,0.6,0.7,0.8,0.9 calculated with 131072
    //let data_point:Vec<uint> = vec!(9830,11469,13107,14746,16384,18022,19660,21299,22937,24576,26214);
    //let data_point:Vec<uint> = vec!(1229,1434,1638,1843,2048,2253,2458,2662,2867,3072,3277);
    let data_point:Vec<uint> = vec!(5000,10000,15000,20000,25000,30000,35000,40000);
    let mut result_hopscotch:Vec<f64> = Vec::with_capacity(12);
    let mut result_robin:Vec<f64> = Vec::with_capacity(12);
    for i in data_point.iter(){
        let mut sum = 0u64;
        for _ in range(0,AVG_SIZE){
            sum += insert_lookup_remove_hopscotch(*i);
        }
        let sum_nano = (sum as f64)/(AVG_SIZE as f64);
        let time_hopscotch = (sum_nano as f64)/(*i as f64);
        result_hopscotch.push(time_hopscotch);
        let mut sum2 = 0u64;
        for _ in range(0,AVG_SIZE){
            sum2 += insert_lookup_remove_robin(*i);
        }
        let sum_nano2 = (sum2 as f64)/(AVG_SIZE as f64);
        let time_robin = (sum_nano2 as f64) / (*i as f64);
        result_robin.push(time_robin);
    }

    let mut file = File::create(&Path::new("output-files/bench-lookups"));
    for i in range(0u,8u){
        let d = data_point.get(i).to_str().append(" ");
        let h = result_hopscotch.get(i).to_str().append(" ");
        let r = result_robin.get(i).to_str().append("\n");
        let line = d.into_bytes()+h.into_bytes()+r.into_bytes();
        file.write(line.as_slice());
    }
}
