use std::clone::Clone;
use std::mem::replace;
use std::cmp::max;
use std::num;
use std::default::Default;
use std::vec::Vec;

pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;
static INITIAL_LOG2_CAP: uint = 5;
static INITIAL_CAPACITY: uint = 1 << INITIAL_LOG2_CAP; //2^5

//Is not boxed, like structures are in Rust
#[deriving(Show,Clone)]
pub struct Bucket{
    hop_info: u32,
    hash:     u64
}

pub struct RawTable<K,V>{
    // Available elements
    capacity: uint,
    // Occupied elements
    size:     uint,
    buckets:  Vec<Bucket>, //Contains hop info and hash
    keys:     Vec<K>,
    vals:     Vec<V>
}

impl<K: Default + Clone, V: Default + Clone> RawTable<K,V>{
    pub fn new(cap: uint) -> RawTable<K,V>{
        let capacity = num::next_power_of_two(max(INITIAL_CAPACITY,cap));
        let bucket_vec = Vec::from_elem(capacity,Bucket{hop_info:0,hash:0});
        let a:K = Default::default();
        let keys_vec = Vec::from_elem(capacity,a);
        let b:V = Default::default();
        let vals_vec = Vec::from_elem(capacity,b);
        let ret = RawTable{
                      capacity: capacity,
                      size: 0,
                      buckets: bucket_vec,
                      keys: keys_vec,
                      vals: vals_vec
                  };
        ret
    }
    pub fn get_virt_bucket<'a>(&'a mut self,idx:uint)->&'a mut [Bucket]{
        self.buckets.mut_slice(idx,idx+VIRTUAL_BUCKET_CAPACITY)
    }
    pub fn get_key<'a>(&'a self,idx:uint)->&'a K{
        self.keys.get(idx)
    }
    pub fn get_val<'a>(&'a self,idx:uint)->&'a V{
        self.vals.get(idx)
    }
    pub fn remove_key(&mut self,idx:uint)->Option<K>{
        self.keys.remove(idx)
    }
    pub fn remove_val(&mut self,idx:uint)->Option<V>{
        self.vals.remove(idx)
    }
    pub fn insert_key(&mut self,idx:uint,elem:K){
        self.keys.insert(idx,elem)
    }
    pub fn insert_val(&mut self,idx:uint,elem:V){
        self.vals.insert(idx,elem)
    }
    pub fn resize(&mut self){
        // Check if table can be resized, return false if it can't
        let new_capacity = self.capacity << 1;
        // Assert the shift doesn't overflow, aka the 1 has moved 'over the 
        // edge'. We know this is alright, because it is initialized as a power
        // of two.
        assert!(new_capacity != 0);
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
            if info & 1 == 1 {
                let old_address = (bucket.hash as uint) & old_mask;
                let new_address = (bucket.hash as uint) & new_mask;
                replace(&mut self.buckets.get(new_address),
                                                old_buckets.get(old_address));
                replace(&mut self.keys.get(new_address),
                                                    old_keys.get(old_address));
                replace(&mut self.vals.get(new_address),
                                                    old_vals.get(old_address));
            }
            info = info >> 1;
        }
    }
    pub fn capacity(&self)->uint{
        self.capacity
    }
    pub fn size(&self)->uint{
        self.size
    }

}

fn main(){
    let mut v:Vec<Bucket> = Vec::with_capacity(10);
    v.insert(0,Bucket{hop_info:12,hash:10});
    let b = v.get(0);
    println!("Sup?{}",b);
}
