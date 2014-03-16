extern crate sync;
extern crate collections;
extern crate test;

use test::BenchHarness;
use collections::HashMap;
use sync::RWArc;

fn setup_hashmap(n: uint, m: uint) -> HashMap<uint, uint>{
	let mut retval = HashMap::with_capacity(n);
	let mut b = m.clone();
	for p in (1u, b){
		retval.insert(12, 10);
	};
	
	return retval;
}

fn bob(){
	//hashmappet skal gives som input
	let map = HashMap::with_capacity(100);
	let map_arc = RWArc::new(map);

	for n in range(1u, 10){
		let (sender, rec) = channel();
		sender.send(map_arc.clone());
		spawn(proc() {
		let local_arc = rec.recv();
		   for u in range(1u, 10){
			local_arc.write(|m|{
				m.insert(u,2+1)});
			local_arc.read(|m| {
				let b = m.find(&u);});
			local_arc.write(|m|{
				let a = m.pop(&u);});
			}
		});
	};
}


#[bench]
fn bench_m(b: &mut BenchHarness) {
	b.iter(|| bob());

}


fn main(){
}
