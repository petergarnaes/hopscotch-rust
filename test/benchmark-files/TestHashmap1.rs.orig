extern crate sync;
extern crate collections;
extern crate test;

use test::BenchHarness;
use collections::HashMap;
use sync::RWArc;

//fn bob(){
//	let map = HashMap::new();
//	let map_arc = RWArc::new(map);
//
//	for n in range(1u, 10){
//		let (port, chan) = Chan::new();
//		chan.send(map_arc.clone());
//		spawn(proc() {
//			let local_arc = port.recv();
//		for u in range(1u, 10){
//			local_arc.write(|m|{
//				m.insert(3,n+1)});
//			local_arc.read(|m| {
//				let b = m.find(&3);});
//			}
//		});
//	};
//}


//#[bench]
//fn bench_m(b: &mut BenchHarness) {
//	b.iter(|| bob());
//
//}


fn main(){
    // This will make sure hashmap allocates fitting size for the data types
    // it holds
    let mut map: HashMap<~str,~str> = HashMap::new();
    map.find_or_insert(~"Peter",~"Is a good boy");
    println!("Running normally");
}
