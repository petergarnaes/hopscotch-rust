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
    priv hop_info: uint,
    //Because rust implementation does it
    hash: uint,
    key: *K,
    value: *V,
    //Lock for multithreaded implementation
}

pub struct HashMap<K,V,H = SipHasher>{
    priv hasher: H,
    priv size: uint,
    //maybe implement a Mask, which denotes the allocated size of the hashmap.
    // Option is needed because we have no null value
    priv buckets: Vec<Option<Bucket<K,V>>>
}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> HashMap<K,V,H>{
    //Private help functions
    fn resize(&mut self){
    }
    fn insert_bucket(&mut self,)
}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> Container for HashMap<K,V,H>{
    fn len(&self) -> uint {self.size}
}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> Map<K,V> for HashMap<K,V,H>{
    fn find<'a>(&'a self, k: &K) -> Option<&'a V>{

    }
}
impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> MutableMap<K,V> for HashMap<K,V,H>{
    fn find_mut<'a>(&'a mut self,k: &K) -> Option<&'a mut V>{

    }
    fn swap(&mut self, k: K, v: V) -> Option<V> {

    }
    fn pop(&mut self, k: &K) -> Option<V>{

    }
}
impl<K: Hash + Eq, V> HashMap<K, V>{
    pub fn new() -> HashMap<K, V, SipHasher>{

    }
    pub fn with_capacity(capacity: uint) -> HashMap<K, V>{

    }
}
