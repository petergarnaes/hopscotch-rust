#![allow(dead_code)]
use std::clone::Clone;
use std::option::{Option,None,Some};
use std::mem::replace;
use std::cmp::max;
use std::num;
use std::vec::Vec;
//Our H-range
pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;
//Sets our default size when creating a hash table
static INITIAL_LOG2_CAP: uint = 4;
pub static INITIAL_CAPACITY: uint = 1 << INITIAL_LOG2_CAP;

//Bucket structure, containt hop information and hash
#[deriving(Show,Clone)]
pub struct Bucket{
    pub hop_info: u32,
    pub hash:     u64
}

//Rawtable, contains the 3 arrays: Array of buckets, keys and values.
#[deriving(Show,Clone)]
pub struct RawTable<K,V>{
    capacity: uint,
    buckets: Vec<Bucket>,
    keys:     Vec<Option<K>>,
    vals:     Vec<Option<V>>
}

//Rawtable functions
impl<K: Clone, V: Clone> RawTable<K,V>{
	//Initiates a new hash table.
    pub fn new(cap: uint) -> RawTable<K,V>{
        let capacity = num::next_power_of_two(max(INITIAL_CAPACITY,cap));
        let bucket_vec = Vec::from_elem(capacity,Bucket{hop_info:0,hash:0});
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
	//returns an immutable bucket at the given index
    pub fn get_i_bucket<'a>(&'a self,idx:uint)->&'a Bucket{
        self.buckets.get(idx)
    }
	//returns a mutable bucket at the given index
    pub fn get_bucket<'a>(&'a mut self,idx:uint)->&'a mut Bucket{
        self.buckets.get_mut(idx)
    }
	//returns an immutable key at the given index
    pub fn get_key<'a>(&'a self,idx:uint)->&'a K{
        match *self.keys.get(idx) {
            Some(ref k) => k,
            None => fail!("Getting the key at: {} fails",idx)
        }
    }
	//returns an immutable value at the given index
    pub fn get_val<'a>(&'a self,idx:uint)->&'a V{
        match *self.vals.get(idx) {
            Some(ref v) => v,
            None => fail!("Getting the value at: {} fails",idx)
        }
    }
	//return a mutable value at the given index
    pub fn get_mut_val<'a>(&'a mut self,idx:uint)->&'a mut Option<V>{
        self.vals.get_mut(idx)
    }
	//returns a key wrapped in an option.
    pub fn get_key_option(&self,idx:uint)->bool{
        match *self.keys.get(idx) {
            Some(_) => true,
            None => false
        }
    }
	//inserts a key at the given index
    pub fn insert_key(&mut self,idx:uint,elem:K){
        *self.keys.get_mut(idx) = Some(elem);
        //replace(self.keys.get_mut(idx),elem);
    }
	//deletes a key at the given index
    pub fn delete_key(&mut self,idx:uint){
        *self.keys.get_mut(idx) = None;
    }
	//inserts a value at the given index
    pub fn insert_val(&mut self,idx:uint,elem:V){
        replace(self.vals.get_mut(idx),Some(elem));
    }
	//returns the max capacity of the hash table.
    pub fn capacity(&self)->uint{
        self.capacity
    }
}
