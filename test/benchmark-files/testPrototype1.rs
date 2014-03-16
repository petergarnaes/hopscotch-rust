extern crate sync;
extern crate collections;
extern crate test;
extern crate std;

use test::BenchHarness;
use collections::HashMap;
use sync::RWArc;
use std::mem;

fn setup_hashmap(n: uint, m: uint) -> HashMap<uint, uint>{
	let mut retval = HashMap::with_capacity(n);
	let mut b = m.clone();
	for p in range(1u, b){
		retval.insert(p, p);
	};
	
	return retval;
}

fn bob(i: uint, f: uint, p: uint, map: HashMap<uint, uint>){
	//hashmappet skal gives som input
	let map_arc = RWArc::new(map);

		let (sender, rec) = channel();
		sender.send(map_arc.clone());
		spawn(proc() {
		let local_arc = rec.recv();
		 for u in range(1u, i){
			local_arc.write(|m|{
				m.insert(u,2+1)});
			}
		for t in range(1u, f){
			local_arc.read(|m| {
				let b = m.find(&t);});
			}
		for z in range(1u, p){
			local_arc.write(|m|{
				let a = m.pop(&z);});
			}
		});
}


#[bench]
fn bench_m(b: &mut BenchHarness) {
	let mut map = setup_hashmap(100, 60);
	let c = map.clone();
	b.iter(|| bob(20, 60, 20, c));

}


fn main(){
}
