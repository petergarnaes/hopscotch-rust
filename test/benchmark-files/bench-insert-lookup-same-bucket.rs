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

fn insert_time_robin(nr_of_inserts:int)->u64{
    let h = hashers::HasherSameBucket::new(10,256);
    let mut m = HashMap::with_capacity_and_hasher(250,h);
    let start = precise_time_ns();
    for i in range(1,nr_of_inserts+1){
        m.insert(i,i+1);
    }
    let end = precise_time_ns();
    end - start
}
fn insert_time_hopscotch(nr_of_inserts:int)->u64{
    let h = hashers::HasherSameBucket::new(10,256);
    let mut m = hopscotch::HashMap::with_capacity_and_hasher(250,h);
    let start = precise_time_ns();
    for i in range(1,nr_of_inserts+1){
        m.insert(i,i+1);
    }
    let end = precise_time_ns();
    end - start
}
fn lookup_time_hopscotch(nr_of_inserts:int)->u64{
    let h = hashers::HasherSameBucket::new(10,256);
    let mut m = hopscotch::HashMap::with_capacity_and_hasher(250,h);
    for i in range(1,nr_of_inserts+1){
        m.insert(i,i+1);
    }
    let start = precise_time_ns();
    for i in range(1,nr_of_inserts+1){
        m.lookup(i);
    }
    let end = precise_time_ns();
    end - start
}
fn lookup_time_robin(nr_of_inserts:int)->u64{
    let h = hashers::HasherSameBucket::new(10,256);
    let mut m = HashMap::with_capacity_and_hasher(250,h);
    for i in range(1,nr_of_inserts+1){
        m.insert(i,i+1);
    }
    let start = precise_time_ns();
    for i in range(1,nr_of_inserts+1){
        m.find(&i);
    }
    let end = precise_time_ns();
    end - start
}
fn main(){
    let data_point:Vec<int> = vec!(1,5,10,15,20,25,30);
    let mut result_hopscotch:Vec<u64> = Vec::with_capacity(7);
    let mut result_robin:Vec<u64> = Vec::with_capacity(7);
    let mut result_hopscotch2:Vec<u64> = Vec::with_capacity(7);
    let mut result_robin2:Vec<u64> = Vec::with_capacity(7);
    // Datapoint for 1 iteration
    let nr_of_inserts = 500u64;
    for i in data_point.iter(){
        let mut sum = 0;
        for _ in range(1,nr_of_inserts){
            sum += insert_time_hopscotch(*i);
        }
        let time_hopscotch = sum/nr_of_inserts;
        result_hopscotch.push(time_hopscotch);
        let mut sum2 = 0;
        for _ in range(1,nr_of_inserts){
            sum2 += insert_time_robin(*i);
        }
        let time_robin = sum2/nr_of_inserts;
        result_robin.push(time_robin);
        let mut sum3 = 0;
        for _ in range(1,nr_of_inserts){
            sum3 += lookup_time_hopscotch(*i);
        }
        let time_hopscotch2 = sum3/nr_of_inserts;
        result_hopscotch2.push(time_hopscotch2);
        let mut sum4 = 0;
        for _ in range(1,nr_of_inserts){
            sum4 += lookup_time_robin(*i);
        }
        let time_robin2 = sum4/nr_of_inserts;
        result_robin2.push(time_robin2);
    }
    let mut file = File::create(&Path::new("output-files/bench-insert-lookup-same-bucket"));
    // 7 times
    for i in range(0u,7u){
        let d = data_point.get(i).to_str().append(" ");
        let h = result_hopscotch.get(i).to_str().append(" ");
        let r = result_robin.get(i).to_str().append(" ");
        let h2 = result_hopscotch2.get(i).to_str().append(" ");
        let r2 = result_robin2.get(i).to_str().append("\n");

        let line = d.into_bytes()+h.into_bytes()+r.into_bytes()+h2.into_bytes()+r2.into_bytes();
        file.write(line.as_slice());
    }
}
