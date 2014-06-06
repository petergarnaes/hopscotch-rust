extern crate time;

use std::rand::{task_rng,Rng};
use std::hash::{Hash,Hasher,sip};
use time::precise_time_ns;

fn main(){
    let mut r = task_rng();
    let r0 = r.gen();
    let r1 = r.gen();
    let h = sip::SipHasher::new_with_keys(r0,r1);
    let start = precise_time_ns();
    for i in range(0,10000){
        h.hash(&i);
    }
    let end = precise_time_ns();
    println!("Hasher time:{}",(end-start)/10000);
}
