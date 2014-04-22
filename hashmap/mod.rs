#![feature(default_type_params)]
extern crate rand;
//use rand::Rng;
use rand::Rng;
use std::cmp::max;
use std::default::Default;
use std::clone::Clone;
use std::hash::{Hash, Hasher, sip};
use std::mem::replace;
use std::num;
use std::option::{Option, Some, None};
use raw_table::VIRTUAL_BUCKET_CAPACITY;
use raw_table::INITIAL_CAPACITY;
pub mod raw_table;


//ADD_RANGE - 
pub static ADD_RANGE: uint = 256;
pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;

#[deriving(Clone)]
pub struct HashMap<K, V, H = sip::SipHasher>{
    hasher: H,
    raw_table: raw_table::RawTable<K,V>,
	size: uint

}


impl<K: Hash<S> + Eq + Default + Clone, V: Default + Clone, S, H: Hasher<S>> HashMap<K,V,H>{
    //Private help functions

	//hust at decrement size ved remove
    
	fn get_bucket_info(&self, index_addr:uint)->(u32, u64) {
		let b = self.raw_table.get_i_bucket(index_addr);
		let x = b.hop_info.clone();
		let y = b.hash.clone();
		(x,y)
	}
	
	//fn change_bucket_info(&mut self, index_addr:uint)


	fn get_return_value<'a>(&'a self, addr:uint)->&'a V{
		self.raw_table.get_val(addr)
	}

	fn decrement_size(&mut self){
		self.size -= 1;
	}
	
	fn make_hash(&self, key:K)->u64{
		self.hasher.hash(&key)
	}

	fn make_mask(&self)->uint{
		self.raw_table.capacity()-1
	}

	pub fn remove<'a>(&'a mut self, key:K)->Option<&'a V>{
		let new_hash = self.make_hash(key);
		let mask = self.make_mask();
		let index_addr = (new_hash as uint) & mask;
		let (hop_info, _) = self.get_bucket_info(index_addr);

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
		    let mask2 = 1<<i;
		    let mut addr = (index_addr+i) & mask;
			if mask & (hop_info as uint) == 1{
				let (_, check_hash) = self.get_bucket_info(addr);
				if(new_hash == check_hash){
					self.raw_table.get_bucket(index_addr).hop_info =
						self.raw_table.get_bucket(index_addr).hop_info - mask2;
					self.decrement_size();
                    return Some(self.get_return_value(addr));
				}
			}
		}
	    None
    } 

	//lookup - Lookups an item through the key and returns an Option:
	// Some(item) if found, None if not found.
    pub fn lookup<'a>(&'a self, key:K)->Option<&'a V>{
        let new_hash = self.make_hash(key);
        let mask = self.make_mask();
		let index_addr: uint = (new_hash as uint) & mask;
        let (hop_info, bucket_hash) = self.get_bucket_info(index_addr);
		let mut tmp = hop_info;		

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			tmp = tmp >> i;
			let (check_hop, check_hash) = self.get_bucket_info((index_addr + i) & mask);
			if tmp & 1 == 1{
				//Might need some optimization. Might be able to use new_bucket instead which
				//is memory efficient.
				if(new_hash == check_hash){
					return Some(self.get_return_value((index_addr+i) & mask));
				}
			}
			tmp = check_hop;
		}
        None
    }

fn get_swap_vals(&mut self, index_addr:uint, mfd:uint, mask:uint)->V{
	*self.raw_table.get_val(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + mfd) & mask)
}

pub fn swap_vals(&mut self, index_addr:uint, mfd:uint, mask:uint){
	self.raw_table.insert_val(index_addr, self.get_swap_vals(index_addr, mfd, mask));
}

fn get_swap_keys(&mut self, index_addr:uint, mfd:uint, mask:uint)->K{
	*self.raw_table.get_key(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + mfd) & mask)
}

pub fn swap_keys(&mut self, index_addr:uint, mfd:uint, mask:uint){
	self.raw_table.insert_key(index_addr, self.get_swap_keys(index_addr, mfd, mask));
}

	//used to displace a bucket nearer to the start_bucket of insert()
	pub fn find_closer_bucket(&mut self, free_distance:uint, index_addr:uint, val:int, mask:uint)->(uint, int){
		let mut move_bucket = self.raw_table.get_bucket((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) & mask);
		let mut free_dist = VIRTUAL_BUCKET_CAPACITY-1;
		while(0 < free_dist){
			let mut start_hop_info = move_bucket.hop_info;
			let mut move_free_distance = -1;
			let mask = 1u;
			let iter = 0;
			for i in range(0, free_dist){
				if mask & (start_hop_info as uint) == 1{
					move_free_distance = i;
					break;
				}
			}
		if(move_free_distance != -1){
			if(start_hop_info == move_bucket.hop_info){
				move_bucket.hop_info = (move_bucket.hop_info | (1<< free_dist));

				// Vi har et problem med pointers her. raw_table.get_val 
                // returnere en &data og ikke en data. For at kunne gøre dette 
                // skal dette derefereres. dette gælder for insert af key og 
                // value.

				//inserts the data of the newly found bucket into the old one
				{
				self.swap_vals(index_addr, move_free_distance, mask);
				}
				//inserts the key of the newly found bucket into the old one
				{
				self.swap_keys(index_addr, move_free_distance, mask);
				}
				
				move_bucket.hop_info = move_bucket.hop_info & -(1<<move_free_distance);
				return ((free_distance - free_dist), val);
				}
			}
		}
		return (free_distance, val+1);
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
		let new_hash = self.hasher.hash(&key);
		let mask = self.raw_table.capacity()-1;
		let index_addr = mask & (new_hash as uint);
		let mut start_bucket = self.raw_table.get_bucket(index_addr);
		if(self.check_key(key) == true){
			return false
		}

		let mut free_distance = 0u;
		let mut val = 1;
        let mut info = 0;
		for i in range(0,  ADD_RANGE){
			let b = self.raw_table.get_bucket((index_addr+i) & mask);
            info = info | b.hop_info;
			if info & 1 == 0 {
				break;
			}
			free_distance += 1;
		}

		if free_distance < ADD_RANGE {
			while(val != 0){
				if(free_distance < VIRTUAL_BUCKET_CAPACITY){
					start_bucket.hop_info = start_bucket.hop_info | 
                                                            (1<<free_distance);
					self.raw_table.insert_key((index_addr + free_distance) & 
                                                                    mask, key.clone());
					self.raw_table.insert_val((index_addr + free_distance) & 
                                                                    mask, data.clone());
					self.size += 1;
					return true
				}
			let (free_distance, val) = self.find_closer_bucket(free_distance, 
                                                        index_addr, val, mask);
			}
		}
		self.raw_table.resize();
		self.insert(key.clone(), data.clone())
    }
  
    pub fn getRawTable<'a>(&'a mut self)->&'a mut raw_table::RawTable<K,V>{
        &mut self.raw_table
    }
    pub fn getSipHasher<'a>(&'a self)->&'a H{
        &self.hasher
    }

    pub fn with_hasher(hasher:H)->HashMap<K,V,H>{
        HashMap::with_capacity_and_hasher(INITIAL_CAPACITY,hasher)
    }
    pub fn with_capacity_and_hasher(capacity:uint,hasher:H)->HashMap<K,V,H>{
        let cap = num::next_power_of_two(max(INITIAL_CAPACITY, capacity));
        HashMap{
            hasher: hasher,
            raw_table: raw_table::RawTable::new(cap),
            size: 0
        }
    }
}

impl<K:Hash+Eq+Default+Clone,V:Default+Clone> HashMap<K,V,sip::SipHasher>{
    pub fn new() -> HashMap<K,V,sip::SipHasher>{
        HashMap::with_capacity(INITIAL_CAPACITY)
    }

    pub fn with_capacity(capacity: uint) -> HashMap<K,V,sip::SipHasher>{
        let mut r = rand::task_rng();
        let r0 = r.gen();
        let r1 = r.gen();
        let hasher = sip::SipHasher::new_with_keys(r0, r1);
        HashMap::with_capacity_and_hasher(capacity,hasher)
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
