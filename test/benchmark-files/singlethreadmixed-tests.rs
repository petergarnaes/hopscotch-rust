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
fn bench_dens60(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 1228);
	b.iter(|| single_thread_test(50, 100, 50, map.clone()));
	}

#[bench]
fn bench_dens70(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 1433);
	b.iter(|| single_thread_test(50, 100, 50, map.clone()));
	}

#[bench]
fn bench_dens80(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 1638);
	b.iter(|| single_thread_test(50, 100, 50, map.clone()));
	}

#[bench]
fn bench_dens90(b: &mut BenchHarness) {
	let map = setup_hashmap(2047, 1842);
	b.iter(|| single_thread_test(50, 100, 50, map.clone()));
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