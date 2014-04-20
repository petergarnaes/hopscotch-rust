use std::clone::Clone;
use std::cmp::Eq;
use std::mem::replace;
use std::cmp::max;
use std::num;
use std::default::Default;
use std::slice::{from_elem};

pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;
static INITIAL_LOG2_CAP: uint = 5;
pub static INITIAL_CAPACITY: uint = 1 << INITIAL_LOG2_CAP; //2^5

//Is not boxed, like structures are in Rust
#[deriving(Show,Clone,Eq)]
pub struct Bucket{
    pub hop_info: u32,
    pub hash:     u64
}

#[deriving(Show,Clone,Eq)]
pub struct RawTable<K,V>{
    // Available elements
    capacity: uint,
    // Occupied elements
    buckets:  ~[Bucket], //Contains hop info and hash
    keys:     ~[K],
    vals:     ~[V]
}

impl<K: Default + Clone, V: Default + Clone> RawTable<K,V>{
    pub fn new(cap: uint) -> RawTable<K,V>{
        let capacity = num::next_power_of_two(max(INITIAL_CAPACITY,cap));
        let bucket_vec = from_elem(capacity,Bucket{hop_info:0,hash:0});
        let a:K = Default::default();
        let keys_vec = from_elem(capacity,a);
        let b:V = Default::default();
        let vals_vec = from_elem(capacity,b);
        let ret = RawTable{
                      capacity: capacity,
                      buckets: bucket_vec,
                      keys: keys_vec,
                      vals: vals_vec
                  };
        ret
    }
    pub fn get_bucket<'a>(&'a mut self,idx:uint)->&'a mut Bucket{
        &mut self.buckets[idx]
    }
    pub fn get_key<'a>(&'a self,idx:uint)->&'a K{
        &self.keys[idx]
    }
    pub fn get_val<'a>(&'a self,idx:uint)->&'a V{
        &self.vals[idx]
    }
    pub fn remove_key<'a>(&mut self,idx:uint)->&'a K{
		let x = self.keys[idx];
        self.keys[idx] = Default::default();
		&x
    }
    pub fn remove_val<'a>(&mut self,idx:uint)->&'a V{
		let x = self.keys[idx];
        self.vals[idx] = Default::default();
		&x
    }
    pub fn insert_key(&mut self,idx:uint,elem:K){
        self.keys[idx] = elem
    }
    pub fn insert_val(&mut self,idx:uint,elem:V){
        self.vals[idx] = elem
    }
    pub fn resize(&mut self)->bool{
        // Check if table can be resized, return false if it can't
        let new_capacity = self.capacity << 1;
        // Assert the shift doesn't overflow, aka the 1 has moved 'over the 
        // edge'. We know this is alright, because it is initialized as a power
        // of two.
        if new_capacity == 0 {
            return false;
        }
        // Replace old table, replaces the value at a mutable location with a 
        // new one, returning the old value, without deinitializing or copying 
        // either one
        let old_capacity = replace(&mut self.capacity,new_capacity);
        let old_buckets = replace(&mut self.buckets,
                    from_elem(self.capacity,Bucket{hop_info:0,hash:0}));
        let a:K = Default::default();
        let old_keys = replace(&mut self.keys,from_elem(self.capacity,a));
        let b:V = Default::default();
        let old_vals = replace(&mut self.vals,from_elem(self.capacity,b));
        // Use old values to repopulate table
        
        // Holds which of the next virtual_bucket_size elements are full
        let mut info:u32 = 0;
        let old_mask = old_capacity - 1;
        let new_mask = self.capacity - 1;
        for bucket in old_buckets.iter(){
            info = info | bucket.hop_info;
            if info & 1 == 1 {
                let old_address = (bucket.hash as uint) & old_mask;
                let new_address = (bucket.hash as uint) & new_mask;
                replace(&mut self.buckets[new_address],
                                                old_buckets[old_address]);
                replace(&mut self.keys[new_address],
                                                old_keys[old_address].clone());
                replace(&mut self.vals[new_address],
                                                old_vals[old_address].clone());
            }
            info = info >> 1;
        }
        true
    }
    pub fn capacity(&self)->uint{
        self.capacity
    }
}
