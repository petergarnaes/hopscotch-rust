use std::rand::{task_rng,Rng};
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
pub static ADD_RANGE: uint = 512;
//pub static VIRTUAL_BUCKET_CAPACITY: uint = 32;

#[deriving(Clone)]
pub struct HashMap<K, V, H = sip::SipHasher>{
    hasher: H,
    raw_table: raw_table::RawTable<K,V>,
	size: uint
}


impl<K: Hash<S> + Default + Clone, V: Default + Clone, S, H: Hasher<S>> HashMap<K,V,H>{
    #[inline(always)]
	fn decrement_size(&mut self){
		self.size -= 1;
	}
	
	pub fn remove<'a>(&'a mut self, key:K)->Option<V>{
		let new_hash = self.hasher.hash(&key);
		let mask = self.raw_table.capacity()-1u;
		let index_addr = (new_hash as uint) & mask;
        let lock_virtual_bucket = self.raw_table.get_bucket_lock(index_addr);
        let bucket = lock_virtual_bucket.write();
		let hop_info = bucket.hop_info.clone();
		let mut info = 1u32;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			if info & hop_info >= 1u32{
		        let addr = (index_addr+i) & mask;
                let lock_remove_bucket = self.raw_table.get_bucket_lock(addr);
                let remove_bucket = lock_remove_bucket.read();
				let check_hash = remove_bucket.hash.clone();
                drop(remove_bucket);
				if new_hash == check_hash {
                    bucket.hop_info -= info;
                    self.raw_table.delete_key(index_addr);
                    let ret_val = self.raw_table.get_val(addr).clone();
                    self.decrement_size();
                    return Some(ret_val);
				}
                //Unlocks the lock
			}
            info = info << 1;
		}
	    None
        // Because drop is called on all variables the function returns,
        // the lock we took on the virual bucket is automatically unlocked
    }

	//lookup - Lookups an item through the key and returns an Option:
	// Some(item) if found, None if not found.
    pub fn lookup<'a>(&'a self, key:K)->Option<&'a V>{
        let new_hash = self.hasher.hash(&key);
        let mask = self.raw_table.capacity()-1u;
		let index_addr: uint = (new_hash as uint) & mask;
        let virtual_bucket_lock = self.raw_table.get_bucket_lock(index_addr);
        let virtual_bucket = virtual_bucket_lock.read();
        let mut hop_info = virtual_bucket.hop_info.clone();

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			if (hop_info & 1) == 1{
                let check_bucket_lock = self.raw_table.get_bucket_lock(
                                                        (index_addr+i) & mask);
                let check_bucket = check_bucket_lock.read();
			    let check_hash = check_bucket.hash.clone();
				if new_hash == check_hash {
					return self.get_val((index_addr+i) & mask);
				}
                drop(check_bucket);
			}
			hop_info = hop_info >> 1;
		}
        None
    }

	//used to displace a bucket nearer to the start_bucket of insert()
    #[inline(always)]
	pub fn find_closer_bucket(&mut self, free_distance:&mut uint, index_addr:uint, val:&mut int, mask:uint){
        let free_bucket_index = (index_addr + *free_distance) & mask;
        let mut move_bucket_index = ((index_addr + *free_distance) - (VIRTUAL_BUCKET_CAPACITY-1)) & mask; 
		let mut free_dist = VIRTUAL_BUCKET_CAPACITY-1u;
		while 0 < free_dist {
            let move_bucket_lock = self.raw_table.get_bucket_lock(move_bucket_index);
            let mut move_bucket = move_bucket_lock.write();
		    let mut move_bucket_hop_info = move_bucket.hop_info.clone();
			let mut move_free_distance = -1;
			let mut mask2 = 1u32;
			for i in range(0, free_dist){
				if mask2 & move_bucket_hop_info == 1{
					move_free_distance = i;
					break;
				}
                mask2 = mask2 << 1;
			}
		    if move_free_distance != -1 {
                let new_free_bucket_index = (move_bucket_index + move_free_distance) & mask;

                move_bucket.hop_info |= (1<<free_dist);
                // The Rust way would be if this method only required an
                // immutable self, because the locks can return the guarded
                // data as mutable. If this way of doing things should be 
                // complete, keys, and values should be locked as well,
                // but that would be ridiculous!
                let new_free_bucket_val = self.raw_table.get_val(new_free_bucket_index).clone();
                self.raw_table.insert_val(free_bucket_index,new_free_bucket_val);
                let new_free_bucket_key = self.raw_table.get_key(new_free_bucket_index).clone();
                self.raw_table.insert_key(free_bucket_index,new_free_bucket_key);

                move_bucket.hop_info &= !(1u32<<move_free_distance);

                let new_free_bucket = self.raw_table.get_bucket_lock(new_free_bucket_index);
                let free_bucket_lock = self.raw_table.get_bucket_lock(free_bucket_index);
                let free_bucket = free_bucket_lock.write();
                free_bucket.hop_info = new_free_bucket.hop_info;
                free_bucket.hash = new_free_bucket.hash;
			    *free_distance -= free_dist;
			    return;
                // Again, buckets automatically unlock when returning
		    }
            drop(move_bucket);
            move_bucket_index = (move_bucket_index + 1) & mask;
            free_dist -= 1;
	    }
	    *val = 0;
    }

    #[inline(always)]
	fn check_key(&self, key:&K)->bool{
		match self.lookup(key.clone()) {
			Some(_) => return true,
			None => return false
		}
	}
	
    pub fn insert(&mut self, key:K, data:V)-> bool{
		if self.check_key(&key) {
			return false;
		}
		let new_hash = self.hasher.hash(&key);
		let mask = self.raw_table.capacity()-1;
		let index_addr = mask & (new_hash as uint);
        let mut free_distance = 0u;
		let mut val = 1;
        for i in range(0,ADD_RANGE){
            if !self.raw_table.get_key_option((index_addr+i) & mask) {
                break;
            }
            free_distance += 1;
        }
		if free_distance < ADD_RANGE {
			while val != 0 {
				if free_distance < VIRTUAL_BUCKET_CAPACITY {
                    let virtual_bucket_lock = self.raw_table.get_bucket_lock(index_addr);
                    let virtual_bucket = virtual_bucket_lock.write();
                    virtual_bucket.hop_info |= 1<<free_distance;
					self.raw_table.get_bucket((index_addr + free_distance) & 
                                                        mask).hash = new_hash;
					self.raw_table.insert_key((index_addr + free_distance) & 
                                                                    mask, key.clone());
					self.raw_table.insert_val((index_addr + free_distance) & 
                                                                    mask, data.clone());
					self.size += 1;
					return true;
                    
				}
				self.find_closer_bucket(&mut free_distance, index_addr, &mut val, mask);
			}
		}
        println!("free_distance:{}",free_distance);
	    self.resize();
		self.insert(key.clone(), data.clone())
    }
    
    pub fn resize(&mut self){
        println!("Resize!!!");
        let new_capacity = self.raw_table.capacity() << 1;
        //println!("new capacity:{}",new_capacity);
        let old_table = replace(&mut self.raw_table,raw_table::RawTable::new(new_capacity));
        let old_capacity = old_table.capacity();
        let mut info = 0;
        for i in range(0,old_capacity){
            //quick linear probing til resize
            if old_table.get_key_option(i){
                self.insert(old_table.get_key(i).clone(), old_table.get_val(i).clone());
            }
        }
    }
  
    pub fn get_rawtable<'a>(&'a mut self)->&'a mut raw_table::RawTable<K,V>{
        &mut self.raw_table
    }
    pub fn get_siphasher<'a>(&'a self)->&'a H{
        &self.hasher
    }

    #[allow(dead_code)]
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

impl<K:Hash+Default+Clone,V:Default+Clone> HashMap<K,V,sip::SipHasher>{
    pub fn new() -> HashMap<K,V,sip::SipHasher>{
        HashMap::with_capacity(INITIAL_CAPACITY)
    }

    pub fn with_capacity(capacity: uint) -> HashMap<K,V,sip::SipHasher>{
        let mut r = task_rng();
        let r0 = r.gen();
        let r1 = r.gen();
        let hasher = sip::SipHasher::new_with_keys(r0, r1);
        HashMap::with_capacity_and_hasher(capacity,hasher)
    }
}
