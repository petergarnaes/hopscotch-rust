#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;

fn main(){
	let mut x = HashMap::new();
	x.insert(123, 11);
	let a = x.lookup(123);
	let b = x.remove(123);
	print("{}", a == b);
}