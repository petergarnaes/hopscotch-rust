use std::clone::Clone;
use std::option::{Option,None,Some};
use std::cmp::Eq;
use std::mem::replace;
use std::cmp::max;
use std::num;
use std::default::Default;
use std::vec::Vec;

pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;
static INITIAL_LOG2_CAP: uint = 4;
pub static INITIAL_CAPACITY: uint = 1 << INITIAL_LOG2_CAP; //2^5

//Is not boxed, like structures are in Rust
#[deriving(Show,Clone)]
pub struct Bucket{
    pub hop_info: u32,
    pub hash:     u64
}

#[deriving(Show,Clone)]
pub struct RawTable<K,V>{
    // Available elements
    capacity: uint,
    // Occupied elements
    //buckets:  Vec<Bucket>, //Contains hop info and hash
    buckets: Vec<Bucket>,
    keys:     Vec<Option<K>>,
    vals:     Vec<Option<V>>
}

impl<K: Clone, V: Clone> RawTable<K,V>{
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
    pub fn get_i_bucket<'a>(&'a self,idx:uint)->&'a Bucket{
        self.buckets.get(idx)
    }
    pub fn get_bucket<'a>(&'a mut self,idx:uint)->&'a mut Bucket{
        self.buckets.get_mut(idx)
    }
    pub fn get_key<'a>(&'a self,idx:uint)->&'a K{
        match *self.keys.get(idx) {
            Some(ref k) => k,
            None => fail!("We suck at rawtable keys")
        }
    }
    pub fn get_val<'a>(&'a self,idx:uint)->&'a V{
        match *self.vals.get(idx) {
            Some(ref v) => v,
            None => fail!("We suck at rawtable values")
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
    pub fn insert_val(&mut self,idx:uint,elem:V){
        replace(self.vals.get_mut(idx),Some(elem));
    }
    /*
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
                    Vec::from_elem(self.capacity,Bucket{hop_info:0,hash:0}));
        let a:K = Default::default();
        let old_keys = replace(&mut self.keys,Vec::from_elem(self.capacity,a));
        let b:V = Default::default();
        let old_vals = replace(&mut self.vals,Vec::from_elem(self.capacity,b));
        // Use old values to repopulate table
        
        // Holds which of the next virtual_bucket_size elements are full
        let mut info:u32 = 0;
        let old_mask = old_capacity - 1;
        let new_mask = self.capacity - 1;
        for bucket in old_buckets.iter(){
            info = info | bucket.hop_info;
            println!("Info:{}",info);
            if info & 1 == 1 {
                let old_address = (bucket.hash as uint) & old_mask;
                println!("Old address:{}",old_address);
                let new_address = (bucket.hash as uint) & new_mask;
                println!("New address:{}",new_address);
                replace(self.buckets.get_mut(new_address),
                                                *old_buckets.get(old_address));
                replace(self.keys.get_mut(new_address),
                                            old_keys.get(old_address).clone());
                replace(self.vals.get_mut(new_address),
                                            old_vals.get(old_address).clone());
            }
            info = info >> 1;
        }
        true
    }*/
    pub fn capacity(&self)->uint{
        self.capacity
    }
}
