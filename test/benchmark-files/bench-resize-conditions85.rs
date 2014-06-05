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

fn resize_test(load:uint,table_size:uint,add_range:uint)->bool{
    let mut m = hopscotch::HashMap::with_capacity(table_size);
    for i in range(1,load){
        if !m.insert_resize(i,i+1,add_range) {
            return true;
        }
    }
    false
}

fn main(){
    let mut size_4096 = Vec::with_capacity(4);
    let mut size_8192 = Vec::with_capacity(4);
    let mut size_16384 = Vec::with_capacity(4);
    let mut size_32768 = Vec::with_capacity(4);
    let add_ranges = vec!(128,256,512);

    for ar in add_ranges.iter(){
        let mut res = 0;
        for i in range(0,100){
            if resize_test(3482,4096,*ar){
                res += 1;
            }
        }
        size_4096.push(res);
    }
    for ar in add_ranges.iter(){
        let mut res = 0;
        for i in range(0,100){
            if resize_test(6963,8192,*ar){
                res += 1;
            }
        }
        size_8192.push(res);
    }
    for ar in add_ranges.iter(){
        let mut res = 0;
        for i in range(0,100){
            if resize_test(13926,16384,*ar){
                res += 1;
            }
        }
        size_16384.push(res);
    }
    for ar in add_ranges.iter(){
        let mut res = 0;
        for i in range(0,100){
            if resize_test(27853,32768,*ar){
                res += 1;
            }
        }
        size_32768.push(res);
    }
    let mut file = File::create(&Path::new("output-files/bench-resize-conditions85"));
    for i in range(0u,3u){
        let label = add_ranges.get(i).to_str().append(" ");
        let s4096 = size_4096.get(i).to_str().append(" ");
        let s8192 = size_8192.get(i).to_str().append(" ");
        let s16384 = size_16384.get(i).to_str().append(" ");
        let s32768 = size_32768.get(i).to_str().append("\n");
        let line = label.into_bytes()+s4096.into_bytes()+s8192.into_bytes()+s16384.into_bytes()+s32768.into_bytes();
        file.write(line.as_slice());
    }
}
