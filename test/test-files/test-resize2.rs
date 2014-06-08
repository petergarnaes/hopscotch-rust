#![feature(default_type_params)]
#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;

fn main(){
    let mut m = hopscotch::HashMap::with_capacity(16000);
    for i in range(0,17000){
        m.insert(i,i);
    }
    for i in range(0,17000){
        match m.lookup(i) {
            Some(_) => (),
            None => fail!("Bob...")
        }
    }
}
