use std::cmp::max;
use std::default::Default;
use std::hash::{Hash,Hasher};
use std::hash::sip::{SipState, SipHasher};
use std::mem::replace;
use std::num;
use std::rand::Rng;
use std::vec::{Items, MutItems};

// Lets begin with an H=32 and find out later how we determine the bit size
// and cache size of the system later

struct Bucket<K,V>{
    hop_info: uint,
    //Because rust implementation does it
    hash: uint,
    key: K,
    value: V,
    //Lock for multithreaded implementation
}

pub struct HashMap<K,V,H = SipHasher>{
    priv hasher: H,
    priv size: uint,
    // Option is needed because we have no null value
    priv buckets: Vec<Option<Bucket<K,V>>>
}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> HashMap<K,V,H>{
    //Private help functions
    fn resize(&mut self){
        // Test if we shift correctly
        let new_size = self.size << 1;
        
        let old_buckets = replace(&mut self.buckets,Vec::from_fn(new_size,
                                    |_| None));
        //Will be incremented when we insert new values
        self.size = 0;
        for buckets in old_buckets.move_iter(){
            match bucket {
                Some(Bucket{hash: hash,key: key,value: value}) => {
                    self.insert_bucket(hash,key,value)
                },
                None => {}
            }
            self.insert_bucket(bucket);
        }
    }
    fn insert_bucket(&mut self,)
}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> Container for HashMap<K,V,H>{
    fn len(&self) -> uint {
        // We lock the whole map
    }
}
