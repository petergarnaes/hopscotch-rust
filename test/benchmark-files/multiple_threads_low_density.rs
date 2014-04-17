extern crate test;
extern crate collections;
extern crate sync;

use output_benchmark::write_output;
mod output_benchmark;
mod Hashmaptest_mod;

#[cfg(test)]
mod tests{
	use sync::RWArc;
	use test::BenchHarness;
	use collections::HashMap;

use super::Hashmaptest_mod::setup_hashmap;
use super::Hashmaptest_mod::single_thread_test;

#[bench]
fn bench_dens1(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 20);
	b.iter(|| single_thread_test(1448, 0, 0, map.clone()));
	}

#[bench]
fn bench_dens3(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 60);
	b.iter(|| single_thread_test(1448, 0, 0, map.clone()));
	}

#[bench]
fn bench_dens10(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 200);
	b.iter(|| single_thread_test(1448, 0, 0, map.clone()));
	}

#[bench]
fn bench_dens30(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 600);
	b.iter(|| single_thread_test(1448, 0, 0, map.clone()));
	}
}

fn main(){
   	let filename = ~"singlethreadmixed-tests";
	let title = ~"Hashmap single thread tests";
	let x_axis = ~"Hashmap density";
	let y_axis = ~"ns/iteration";
	let data_points = ~[60,70,80,90];
	write_output(filename, title, x_axis, y_axis, data_points);
}
