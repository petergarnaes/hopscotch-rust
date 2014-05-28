#![feature(default_type_params)]
extern crate rand;
extern crate time;
extern crate collections;

use collections::HashMap;
use time::precise_time_ns;

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;


fn main(){
    let mut m = hopscotch::HashMap::with_capacity(100);
    let start_hopscotch = precise_time_ns();
    for i in range(1,10){
        m.insert(i,i+1);
    }
    let end_hopscotch = precise_time_ns();
    let time_hopscotch = end_hopscotch-start_hopscotch;
    println!("time elapsed hopscotch:{}",time_hopscotch);
    let mut rh = HashMap::with_capacity(100);
    let start_robin = precise_time_ns();
    for i in range(1,10){
        rh.find_or_insert(i,i+1);
    }
    let end_robin = precise_time_ns();
    let time_robin = end_robin-start_robin;
    println!("time elapsed robin hood:{}",time_robin);
}
