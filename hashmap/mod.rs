#![feature(default_type_params)]
extern crate rand;
use std::cmp::max;
use std::default::Default;
use std::clone::Clone;
use std::hash::{Hash, Hasher, sip};
use std::mem::replace;
use std::num::NumCast;
use std::option::{Option, Some, None};
use raw_table::VIRTUAL_BUCKET_CAPACITY;
use raw_table::INITIAL_CAPACITY;
mod raw_table;


//ADD_RANGE - 
pub static ADD_RANGE: uint = 256;
pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;

#[deriving(Clone)]
pub struct HashMap<K, V, H>{
    hasher: sip::SipHasher,
    raw_table: raw_table::RawTable<K,V>,
	size: uint

}


impl<K: Hash<S> + Eq + Default + Clone, V: Default + Clone, S, H: Hasher<S>> HashMap<K,V,H>{
    //Private help functions

	//hust at decrement size ved remove
    pub fn remove(&mut self, key:&K)->Option<&V>{
		let new_hash = self.hasher.hash(key);
		let mask = self.raw_table.capacity()-1;
		let index_addr: uint = 0;
		match new_hash.to_uint(){
			Some(x) => index_addr = x & mask,
			None => return None
		}	
		let &mut new_bucket = self.raw_table.get_bucket(index_addr);
		let hop_info = new_bucket.hopinfo;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
		let mask2 = 1<<i;
		let mut addr = (index_addr+i) & mask;
			if((mask & hop_info)){
				let &mut check_bucket = self.raw_table.get_bucket(addr);
				if(new_hash == check_bucket.hash){
					let ret = Some(self.raw_table.get_val(addr));
					self.raw_table.remove_key(addr);
					self.raw_table.remove_val(addr);
					new_bucket.hopinfo = new_bucket.hopinfo - mask2;
					self.size -= 1;
					return ret;
				}
			}
		}
	None
    } 

	//lookup - Lookups an item through the key and returns an Option:
	// Some(item) if found, None if not found.
    pub fn lookup(&self, key:K)->Option<&V>{
        let new_hash = self.hasher.hash(key);
        let mask = self.raw_table.capacity()-1;
		let index_addr: uint = 0;
		match new_hash.to_uint(){
			Some(x) => index_addr = x & mask,
			None => return None
		}	
        let &mut new_bucket = self.raw_table.get_bucket(index_addr);
		let hop_info = new_bucket.hopinfo;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			let mut tmp = hop_info;
			tmp = tmp >> i;
			let &mut check_bucket = self.raw_table.get_bucket((index_addr + i) & mask);
			if(tmp & 1){
				//Might need some optimization. Might be able to use new_bucket instead which
				//is memory efficient.
				if(new_hash == check_bucket.hash){
					return Some(self.raw_table.get_val((index_addr+i) & mask));
				}
			}
			hop_info = check_bucket.hopinfo;
		}
        None;
    }



	//used to displace a bucket nearer to the start_bucket of insert()
	pub fn find_closer_bucket(&mut self, free_distance:int, index_addr:uint, val:int, mask:uint)->(int, int){
		let mut move_bucket = self.raw_table.get_bucket((index_addr - (self.VIRTUAL_BUCKET_CAPACITY-1)) & mask);
		let mut free_dist = VIRTUAL_BUCKET_CAPACITY-1;
		while(0 < free_dist){
			let mut start_hop_info = move_bucket.hopinfo;
			let move_free_distance = -1;
			let mask = 1u;
			let iter = 0;
			for i in range(0, free_dist){
				if(mask & start_hop_info){
					move_free_distance = i;
					break;
				}
			}
		if(move_free_distance != -1){
			if(start_hop_info == move_bucket.hopinfo){
				move_bucket.hopinfo = (move_bucket.hopinfo | (1<< free_dist));
				//inserts the data of the newly found bucket into the old one
				self.raw_table.insert_val(index_addr,
self.raw_table.get_val(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + move_free_distance) & mask));
				//inserts the key of the newly found bucket into the old one
				self.raw_table.insert_key(index_addr,
self.raw_table.get_key(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + move_free_distance) & mask));

				move_bucket.hopinfo = move_bucket.hopinfo & -(1<<move_free_distance);
				return (free_distance - free_dist, val);
				}
			}
		}
		val = 1;
		return (free_distance, val);
	}


	//Supporting function used in insert. Used to lookup whether or not
	//the current key exist or not.

	pub fn check_key(&mut self, key:K)->bool{
		match self.lookup(key) {
			Some(_) => return true,
			None => return false
		}
	}
    pub fn insert(&mut self, key:K, data:V)-> bool{
		let new_hash = self.hasher.hash(key);
		let mask = self.raw_table.capacity()-1;
		let index_addr: uint = 0;
		match new_hash.to_uint(){
			Some(x) => index_addr = x & mask,
			None => return false
		}	
		let mut start_bucket = self.raw_table.get_bucket(index_addr);

		if(self.check_key(key) == true){
			return false
		}

		let mut free_distance = 0;
		let mut val = 1;
		for i in range(0,  ADD_RANGE){
			let check_key = self.get_key((index_addr+i) & mask);
			if(check_key = Default::default()){
				break;
			}
			free_distance += 1;
		}

		if (free_distance < ADD_RANGE){
			while(val != 0){
				if(free_distance < VIRTUAL_BUCKET_CAPACITY){
					start_bucket.hopinfo = start_bucket.hopinfo | (1<<free_distance);
					self.raw_table.insert_key((index_addr + free_distance, key) & mask);
					self.raw_table.insert_val((index_addr + free_distance, data) & mask);
					self.size += 1;
					return true
				}
			(free_distance, val) = self.find_closer_bucket(free_distance, index_addr, val, mask);
			}
		}
		self.raw_table.resize();
		return self.insert(key, data);
    } 
  
    pub fn getRawTable(&mut self)->&mut raw_table::RawTable<K,V>{
        &mut self.raw_table
    }
    pub fn getSipHasher(&self)->&sip::SipHasher{
        &self.hasher
    }

    pub fn new() -> HashMap<K, V,sip::SipHasher>{
        let rng = rand::task_rng();
        let r0 = rng.gen();
        let r1 = rng.gen();
        let hasher = sip::SipHasher::new_with_keys(r0,r1);
        let hashmap = HashMap{
            hasher: hasher,
            raw_table: raw_table::RawTable::new(INITIAL_CAPACITY),
	        size: 0
        };
	    hashmap
    }

    pub fn with_capacity(capacity: uint) -> HashMap<K, V,sip::SipHasher>{
		let rng = rand::task_rng();
        let r0 = rng.gen();
        let r1 = rng.gen();
        let hasher = sip::SipHasher::new_with_keys(r0,r1);
        let hashmap = HashMap{
            hasher: hasher,
            raw_table: raw_table::RawTable::new(capacity),
	        size: 0
        };
	    hashmap

    }

}

//impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> Map<K,V> for HashMap<K,V,H>{
//    fn find<'a>(&'a self, k: &K) -> Option<&'a V>{
//
//    }
//}

//impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> MutableMap<K,V> for HashMap<K,V,H>{
//    fn find_mut<'a>(&'a mut self,k: &K) -> Option<&'a mut V>{
//
//    }  
//}
