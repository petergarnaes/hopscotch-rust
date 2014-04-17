use std::cmp::max;
use std::default::Default;
use std::clone::Clone;
use std::hash::{Hash, Hasher, sip};
use std::mem::replace;
use std::num;
use std::option::{Option, Some, None};
mod raw_table;

//ADD_RANGE - 
static ADD_RANGE: uint = 256;

#[deriving(Clone)]
pub struct HashMap<K, V, H>{
    hasher: sip::SipHasher,
    raw_table: raw_table::RawTable<K,V>,
	size: uint

// Lets begin with an H=32 and find out later how we determine the bit size
// and cache size of the system later

}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> HashMap<K,V,H>{
    //Private help functions

	//hust at decrement size ved remove
    fn remove(&mut self, K: key)->Option<V>{
		let new_hash = self.hasher.hash(key);
		let mask = self.raw_table.capacity()-1;
		let mut index_addr = new_hash & mask;
		let mut new_bucket = self.raw_table.get_bucket(index_addr);
		let mut hop_info = new_bucket.hopinfo;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
		let mask2 = 1<<i;
		let mut addr = (index_addr+i) & mask;
			if(mask & hop_info){
				check_bucket = self.raw_table.get_bucket(addr);
				if(new_hash == check_bucket.hash){
					let ret = Some(check_bucket.get_val(addr));
					check_bucket.remove_key(addr);
					check_bucket.remove_val(addr);
					new_bucket.hopinfo = new_bucket.hopinfo - mask2;
					self.raw_table.size -= 1;
				}
			}
		}
    }

	//lookup
    fn lookup(&self, K: key)->Option<V>{
        let new_hash = self.hasher.hash(key);
        let mask = self.raw_table.capacity()-1;
        let mut index_addr = new_hash & mask;
        let mut new_bucket = self.raw_table.get_bucket(index_addr);
		let mut hop_info = new_bucket.hopinfo;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			let mut tmp = hop_info;
			tmp = tmp >> i;

			if(tmp & 1){
				//Might need some optimization. Might be able to use new_bucket instead which
				//is memory efficient.
				let mut check_bucket = self.raw_table.get_bucket((index_addr + i) & mask);
				if(new_hash == check_bucket.hash){
					check_bucket.get_val((index_addr+i) & mask);
				}
			}
			check_bucket = self.raw_table.get_bucket((index_addr + i) & mask);
			hop_info = check_bucket.hopinfo;
		}
        NONE;
    }

	//resize
	fn resize(&self){
		self.raw_table.resize();
	}

	//used to displace a bucket nearer to the start_bucket of insert()
	fn find_closer_bucket(&mut self, free_distance:int, index_addr:uint, val:int, mask:uint)->int{
		let mut move_bucket = self.raw_table.get_bucket((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) & mask);
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
				let new_free_bucket = self.raw_table.get_bucket();

				move_bucket.hopinfo = (move_bucket.hopinfo | (1<< free_dist));
				//inserts the data of the newly found bucket into the old one
				self.raw_table.insert_val(index_addr,
self.raw_table.get_val(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + move_free_distance)) & mask);
				//inserts the key of the newly found bucket into the old one
				self.raw_table.insert_key(index_addr,
self.raw_table.get_key(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + move_free_distance)) & mask);

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

	fn check_key(&mut self, K: key)->bool{
		match lookup(key){
			Some(_) => return true,
			None => return false
		}
	}
    fn insert(&mut self, K: key, V: data)-> bool{
		let new_hash = self.hasher.hask(key);
		let mask = self.raw_table.capacity()-1;
		let mut index_addr = new_hash & mask;
		let mut start_bucket = self.raw_table.get_bucket(index_addr);

		if(check_key(key)){
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
					self.raw_table.size += 1;
					return true
				}
			(free_distance, val) = find_close_bucket(free_distance, index_addr, val, mask);
			}
		}
		resize();
		return insert(key, data);
    }

}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> Map<K,V> for HashMap<K,V,H>{
    fn find<'a>(&'a self, k: &K) -> Option<&'a V>{

    }
}

impl<K: Hash<S> + Eq, V, S, H: Hasher<S>> MutableMap<K,V> for HashMap<K,V,H>{
    fn find_mut<'a>(&'a mut self,k: &K) -> Option<&'a mut V>{

    }
  
}
impl<K: Hash + Eq, V> HashMap<K, V>{
    pub fn new() -> HashMap<K, V, SipHasher>{

    }
    pub fn with_capacity(capacity: uint) -> HashMap<K, V>{

    }
}
