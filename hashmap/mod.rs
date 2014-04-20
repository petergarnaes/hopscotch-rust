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
    pub fn remove(&mut self, key:K)->Option<V>{
		let new_hash = self.hasher.hash(key);
		let mask = self.raw_table.capacity()-1;
		let mut index_addr = new_hash & mask;
		let mut new_bucket = self.raw_table.get_bucket(index_addr);
		let mut hop_info = new_bucket.hopinfo;

		for i in range(0u, self.VIRTUAL_BUCKET_CAPACITY){
		let mask2 = 1<<i;
		let mut addr = (index_addr+i) & mask;
			if(mask & hop_info){
				let mut check_bucket = self.raw_table.get_bucket(addr);
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

	//lookup - Lookups an item through the key and returns an Option:
	// Some(item) if found, None if not found.
    pub fn lookup(&self, key:K)->Option<V>{
        let new_hash = self.hasher.hash(key);
        let mask = self.raw_table.capacity()-1;
        let mut index_addr = new_hash & mask;
        let mut new_bucket = self.raw_table.get_bucket(index_addr);
		let mut hop_info = new_bucket.hopinfo;

		for i in range(0u, self.VIRTUAL_BUCKET_CAPACITY){
			let mut tmp = hop_info;
			tmp = tmp >> i;
			let mut check_bucket = self.raw_table.get_bucket((index_addr + i) & mask);
			if(tmp & 1){
				//Might need some optimization. Might be able to use new_bucket instead which
				//is memory efficient.
				if(new_hash == check_bucket.hash){
					Some(check_bucket.get_val((index_addr+i) & mask));
				}
			}
			check_bucket = self.raw_table.get_bucket((index_addr + i) & mask);
			hop_info = check_bucket.hopinfo;
		}
        None;
    }



	//used to displace a bucket nearer to the start_bucket of insert()
	pub fn find_closer_bucket(&mut self, free_distance:int, index_addr:uint, val:int, mask:uint)->(int, int){
		let mut move_bucket = self.raw_table.get_bucket((index_addr - (self.VIRTUAL_BUCKET_CAPACITY-1)) & mask);
		let mut free_dist = self.VIRTUAL_BUCKET_CAPACITY-1;
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
self.raw_table.get_val(((index_addr - (self.VIRTUAL_BUCKET_CAPACITY-1)) + move_free_distance) & mask));
				//inserts the key of the newly found bucket into the old one
				self.raw_table.insert_key(index_addr,
self.raw_table.get_key(((index_addr - (self.VIRTUAL_BUCKET_CAPACITY-1)) + move_free_distance) & mask));

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
		let mut index_addr = new_hash & mask;
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
				if(free_distance < self.VIRTUAL_BUCKET_CAPACITY){
					start_bucket.hopinfo = start_bucket.hopinfo | (1<<free_distance);
					self.raw_table.insert_key((index_addr + free_distance, key) & mask);
					self.raw_table.insert_val((index_addr + free_distance, data) & mask);
					self.raw_table.size += 1;
					return true
				}
			(free_distance, val) = self.find_closer_bucket(free_distance, index_addr, val, mask);
			}
		}
		self.raw_table.resize();
		return self.insert(key, data);
    } 
  
    pub fn getRawTable(&mut self)->&mut raw_table::RawTable{
        &mut self.raw_table
    }
    pub fn getSipHasher(&self)->&sip::SipHasher{
        &self.hasher
    }

    pub fn new(&mut self) -> HashMap<K, V>{
		self.raw_table.new(100);	
		return self;
    }

    pub fn with_capacity(&mut self, capacity: uint) -> HashMap<K, V>{
		self.raw_table.new(capacity);
		return self;
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

}
