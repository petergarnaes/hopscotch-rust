use std::clone::Clone;
use std::cmp::Eq;
use std::default::Default;
use std::vec::Vec;

pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;

//Is not boxed, like structures are in Rust
#[deriving(Show,Clone)]
pub struct Bucket{
    hop_info: u32,
    hash:     u64
}

pub struct RawTable<K,V>{
    capacity: uint,
    size:     uint,
    buckets:  Vec<Bucket>, //Contains hop info and hash
    keys:     Vec<K>,
    vals:     Vec<V>
}

impl<K: Default + Clone, V: Default + Clone> RawTable<K,V>{
    pub fn new(capacity: uint) -> RawTable<K,V>{
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
    pub fn remove_key(&mut self,idx:uint)->bool{
        let var = self.keys.remove(idx);
        match var {
            Some(_) => true,
            None    => false
        }
    }
    pub fn remove_val(&mut self,idx:uint)->bool{
        let var = self.vals.remove(idx);
        match var {
            Some(_) => true,
            None    => false
        } 
    }
    pub fn insert_key(&mut self,idx:uint,elem:K){
        self.keys.insert(idx,elem)
    }
    pub fn insert_val(&mut self,idx:uint,elem:V){
        self.vals.insert(idx,elem)
    }
    pub fn resize(&self,size:uint)->bool{
        true
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
