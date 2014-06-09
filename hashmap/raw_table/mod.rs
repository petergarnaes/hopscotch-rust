#![allow(dead_code)]
extern crate sync;
use sync::RWLock;
use std::clone::Clone;
use std::option::{Option,None,Some};
use std::mem::replace;
use std::cmp::max;
use std::num;
use std::vec::Vec;

pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;
static INITIAL_LOG2_CAP: uint = 4;
pub static INITIAL_CAPACITY: uint = 1 << INITIAL_LOG2_CAP; //2^5

//Is not boxed, like structures are in Rust
pub struct Bucket{
    pub hop_info: u32,
    pub hash:     u64
}

pub struct RawTable<K,V>{
    // Available elements
    capacity: uint,
    // Occupied elements
    //buckets:  Vec<Bucket>, //Contains hop info and hash
    buckets: Vec<RWLock<Bucket>>,
    keys:     Vec<Option<K>>,
    vals:     Vec<Option<V>>
}

impl<K: Clone, V: Clone> RawTable<K,V>{
    pub fn new(cap: uint) -> RawTable<K,V>{
        let capacity = num::next_power_of_two(max(INITIAL_CAPACITY,cap));
        let bucket_vec = Vec::from_fn(capacity,|_|{RWLock::new(Bucket{hop_info:0,hash:0})});
        let keys_vec = Vec::from_elem(capacity,None);
        let vals_vec = Vec::from_elem(capacity,None);
        let ret = RawTable{
                      capacity: capacity,
                      buckets: bucket_vec,
                      keys: keys_vec,
                      vals: vals_vec
                  };
        ret
    }
    pub fn get_bucket_lock<'a>(&'a self,idx:uint)->RWLock<Bucket>{
        self.buckets.get(idx)
    }
    pub fn get_key<'a>(&'a self,idx:uint)->&'a K{
        match *self.keys.get(idx) {
            Some(ref k) => k,
            None => fail!("We suck at rawtable keys:{}",idx)
        }
    }
    pub fn get_val<'a>(&'a self,idx:uint)->&'a V{
        match *self.vals.get(idx) {
            Some(ref v) => v,
            None => fail!("We suck at rawtable values:{}",idx)
        }
    }
    pub fn get_mut_val<'a>(&'a mut self,idx:uint)->&'a mut Option<V>{
        self.vals.get_mut(idx)
    }
    pub fn get_key_option(&self,idx:uint)->bool{
        match *self.keys.get(idx) {
            Some(_) => true,
            None => false
        }
    }
    pub fn insert_key(&mut self,idx:uint,elem:K){
        *self.keys.get_mut(idx) = Some(elem);
        //replace(self.keys.get_mut(idx),elem);
    }
    pub fn delete_key(&mut self,idx:uint){
        *self.keys.get_mut(idx) = None;
    }
    pub fn insert_val(&mut self,idx:uint,elem:V){
        replace(self.vals.get_mut(idx),Some(elem));
    }
    
    pub fn capacity(&self)->uint{
        self.capacity
    }
}
