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
use std::iter::range_step;
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
	
	pub fn remove<'a>(&'a mut self, key:K)->Option<&'a V>{
		let new_hash = self.hasher.hash(&key);
		let mask = self.raw_table.capacity()-1u;
		let index_addr = (new_hash as uint) & mask;
		let (hop_info,_) = self.get_bucket_info(index_addr);
		let mut info = 1u32;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			if info & hop_info >= 1u32{
		        let mut addr = (index_addr+i) & mask;
				let (_, check_hash) = self.get_bucket_info(addr);
				if(new_hash == check_hash){
                    {
                    let remove_bucket = self.raw_table.get_bucket(index_addr);
                    remove_bucket.hop_info = remove_bucket.hop_info - info;
					}
                    self.decrement_size();
                    return Some(self.get_return_value(addr));
				}
			}
            info = info << 1;
		}
	    None
    } 

	//lookup - Lookups an item through the key and returns an Option:
	// Some(item) if found, None if not found.
    pub fn lookup<'a>(&'a self, key:K)->Option<&'a V>{
        let new_hash = self.hasher.hash(&key);
        //println!("Lookup hashes to: {}",new_hash)
        let mask = self.raw_table.capacity()-1u;
		let index_addr: uint = (new_hash as uint) & mask;
        let (mut hop_info,_) = self.get_bucket_info(index_addr);

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
            //println!("hop info:{}",hop_info);
			if (hop_info & 1) == 1{
				//Might need some optimization. Might be able to use new_bucket instead which
				//is memory efficient.
                //println!("i:{}",i);
			    let (_,check_hash) = self.get_bucket_info((index_addr + i) & mask);
				if(new_hash == check_hash){
					return Some(self.get_return_value((index_addr+i) & mask));
				}
			}
			hop_info = hop_info >> 1;
		}
        println!("Debug101");
        None
    }

//used to get the value of the displacing bucket
fn get_sec_vals(&mut self, index_addr:uint, mfd:uint, mask:uint)->V{
	self.raw_table.get_val(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + mfd) & mask).clone()
}

//used to get the key of the displacing bucket
fn get_sec_keys(&mut self, index_addr:uint, mfd:uint, mask:uint)->K{
	self.raw_table.get_key(((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) + mfd) & mask).clone()
}

	//used to displace a bucket nearer to the start_bucket of insert()
	pub fn find_closer_bucket(&mut self, free_distance:&mut uint, index_addr:uint, val:&mut int, mask:uint)->uint{
		let ( mut move_info, _) = self.get_bucket_info(((index_addr + *free_distance) - (VIRTUAL_BUCKET_CAPACITY-1)) & mask);
        println!("move info:{}",move_info);
		let mut free_dist = VIRTUAL_BUCKET_CAPACITY-1u;
		while(0 < free_dist){
			let start_hop_info = move_info;
			let mut move_free_distance = -1;
			let mut mask = 1u;
			for i in range(0, free_dist){
				if mask & (start_hop_info as uint) == 1{
					move_free_distance = i;
					break;
				}
                mask = mask << 1;
			}
		if(move_free_distance != -1){
            println!("Bobby plz!");
			if(start_hop_info == move_info){
				self.raw_table.get_bucket(((index_addr+*free_distance) - (VIRTUAL_BUCKET_CAPACITY-1)) & mask).hop_info = (move_info | (1<< free_dist));

				// Vi har et problem med pointers her. raw_table.get_val 
                // returnere en &data og ikke en data. For at kunne gøre dette 
                // skal dette derefereres. dette gælder for insert af key og 
                // value.

				//inserts the keys of the newly found bucket into the old one
				{
				swap(self.get_sec_keys(index_addr, move_free_distance, mask), self.get_key(((index_addr + *free_distance) - (VIRTUAL_BUCKET_CAPACITY-1)) & mask));
				}
				//inserts the data of the newly found bucket into the old one
				{
				let b = self.get_sec_vals(index_addr, move_free_distance, mask);
				self.raw_table.insert_val(index_addr, b);
				}
				
				self.raw_table.get_bucket((index_addr - (VIRTUAL_BUCKET_CAPACITY-1)) & mask).hop_info = move_info & -(1<<move_free_distance);
				println!("free dist:{}",free_dist);
				*free_distance = *free_distance - free_dist;
				return (move_free_distance as uint);
				}
			}
            free_dist = free_dist - 1;
		}
		*val = 0;
		return 0u;
	}

	//Insert skal anvende move_free_distance når den skal finde sine buckets. I starten af funktionen skal denne
	//sættes til 0, hvorefter den vil blive opdateret når find_closer_buckets kaldes.

	//Supporting function used in insert. Used to lookup whether or not
	//the current key exist or not.

    fn get_insert_bucket_info(&mut self,addr:uint,mask:uint) -> u32{
        let start_addr = (addr-(VIRTUAL_BUCKET_CAPACITY-1)) & mask;
        let mut info = self.raw_table.get_bucket(start_addr).hop_info.clone();
        let mut i = 1;
        while i < VIRTUAL_BUCKET_CAPACITY {
            info = info >> 1;
            info = info | self.raw_table.get_bucket((start_addr+i) & mask).hop_info;
            i += 1;
        }
        info
    }

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
        //println!("Insert hashes to: {}",new_hash)
		let mask = self.raw_table.capacity()-1;
		let index_addr = mask & (new_hash as uint);
		let mut info = self.get_insert_bucket_info(index_addr,mask);
		//let (mut info,_) = self.get_bucket_info(index_addr);
        let mut free_distance = 0u;
		let mut val = 1;
		for i in range(1,  ADD_RANGE){
			if (info & 1) == 0 {
				break;
			}
			info = info >> 1;
			let (b_info, _) = self.get_bucket_info((index_addr+i) & mask);
            info = info | b_info;
            //println!("info in insert: {}",info);
			free_distance += 1;
		}
        //println!("free_distance in insert: {}",free_distance);
		if free_distance < ADD_RANGE {
			while val != 0 {
				if free_distance < VIRTUAL_BUCKET_CAPACITY {
                    //assert!(start_info & (1<<free_distance) != 0); 
                    println!("info:{}",info);
                    println!("index address:{}",index_addr);
                    println!("free distance at insert:{}",free_distance);
                    self.raw_table.get_bucket(index_addr).hop_info |= (1<<free_distance);
					println!("address:{}",(index_addr + free_distance) & mask);
					self.raw_table.get_bucket((index_addr + free_distance) & 
                                                        mask).hash = new_hash;
					self.raw_table.insert_key((index_addr + free_distance) & 
                                                                    mask, key.clone());
					self.raw_table.insert_val((index_addr + free_distance) & 
                                                                    mask, data.clone());
					self.size += 1;
					return true;
                    
				}
                println!("free distance before closer bucket:{}",free_distance);
				self.find_closer_bucket(&mut free_distance, index_addr, &mut val, mask);
                println!("free distance after closer bucket:{}",free_distance);
			}
		}
        println!("Blob");
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
