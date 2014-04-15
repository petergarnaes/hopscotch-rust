extern crate sync;
extern crate collections;

use collections::HashMap;
use sync::RWArc;
//This file uses 2 functions, setup_hashmap and test_running.
//This is used to test how fast a hashmap runs with a certain
//density executing lookups, inserts and deletes.
//It all happens in the function bench_m.
//
//To first decide the size and density of the hashmap, you will
//need to change the values in the first line of bench_m.
//The first input value decides the size of the hashmap, and the
//second input value decides the density of the hashmap.
//
//For the second line, is the iterator which runs the test.
//It is here the user can decide how many times the user wants
//to run a lookup, insert or delete in the test.


//setup_hashmap creates a hashmap with a density set by the user.
//the input variable n decides the size of the hashmap, and the 
//input variable m decides the density of the hashmap that the
//function returns.
 
pub fn setup_hashmap(n: uint, m: uint) -> HashMap<uint, uint>{
	let mut retval = HashMap::with_capacity(n);
	for p in range(1u, m){
		retval.insert(p, p);
	};
	
	return retval;
}

//test_running executes lookups, inserts and deletes in a hashmap given by
//the user. The amount of inserts and lookups are defined by the user as well.
//The function takes 4 inputs:
// 		i: is the amount of inserts the test executes
//		f: is the amount of lookups the test executes
//		p: is the amount of deletes the test executes
//		a: is the amount of process' spawned.
//		map: is the Hashmap which it executes on.

pub fn multi_threaded_test(i: uint, f: uint, p: uint, a: uint ,map: HashMap<uint, uint>){
	let map_arc = RWArc::new(map);
		for q in range(0, a){
			let (sender, rec) = channel();
			sender.send(map_arc.clone());
				spawn(proc() {
				let local_arc = rec.recv();
				 for u in range(0, i){
					local_arc.write(|m|{
						m.insert(u,2+1)});
					}
				for t in range(0, f){
					local_arc.read(|m| {
						m.find(&t);});
					}
				for z in range(0, p){
					local_arc.write(|m|{
						 m.pop(&z);});
					}
				});
		}
}

pub fn single_thread_test(i: uint, f: uint, p: uint, mut map: HashMap<uint, uint>){
		for a in range(0, i){
			map.insert(a, 2+1);
		};
		for b in range(0, f){
			map.find(&b);
		};
		for c in range(0, p){
			map.pop(&p);
		};
}

