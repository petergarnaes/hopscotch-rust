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
    //Private help functions

	//hust at decrement size ved remove
    
	/*fn get_bucket_info(&self, index_addr:uint)->(u32, u64) {
		let b = self.raw_table.get_i_bucket(index_addr);
		let x = b.hop_info.clone();
		let y = b.hash.clone();
		(x,y)
	}*/
	
	//fn change_bucket_info(&mut self, index_addr:uint)


    #[inline(always)]
	fn get_return_value<'a>(&'a self, addr:uint)->&'a V{
		self.raw_table.get_val(addr)
	}

    #[inline(always)]
	fn decrement_size(&mut self){
		self.size -= 1;
	}
	
	pub fn remove<'a>(&'a mut self, key:K)->Option<V>{
		let new_hash = self.hasher.hash(&key);
		let mask = self.raw_table.capacity()-1u;
		let index_addr = (new_hash as uint) & mask;
		let hop_info = self.raw_table.get_bucket(index_addr).hop_info.clone();
		let mut info = 1u32;

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
			if info & hop_info >= 1u32{
		        let addr = (index_addr+i) & mask;
				let check_hash = self.raw_table.get_bucket(addr).hash.clone();
				if new_hash == check_hash {
                    self.raw_table.get_bucket(index_addr).hop_info -= info;
                    self.raw_table.delete_key(index_addr);
                    let ret_val = self.raw_table.get_val(addr).clone();
                    //*self.raw_table.get_mut_val(addr) = None;
                    self.decrement_size();
                    return Some(ret_val);
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
        let mut hop_info = self.raw_table.get_i_bucket(index_addr).hop_info.clone();

		for i in range(0u, VIRTUAL_BUCKET_CAPACITY){
            //println!("hop info:{}",hop_info);
			if (hop_info & 1) == 1{
				//Might need some optimization. Might be able to use new_bucket instead which
				//is memory efficient.
                //println!("i:{}",i);
			    let check_hash = self.raw_table.get_i_bucket((index_addr + i) & mask).hash.clone();
				if new_hash == check_hash {
					return Some(self.get_return_value((index_addr+i) & mask));
				}
			}
			hop_info = hop_info >> 1;
		}
        //println!("Debug101");
        None
    }

	//used to displace a bucket nearer to the start_bucket of insert()
    #[inline(always)]
	pub fn find_closer_bucket(&mut self, free_distance:&mut uint, index_addr:uint, val:&mut int, mask:uint){
        //println!("free{}",*free_distance);
        let free_bucket_index = (index_addr + *free_distance) & mask;
        let mut move_bucket_index = ((index_addr + *free_distance) - (VIRTUAL_BUCKET_CAPACITY-1)) & mask; 
        // move_bucket
        //println!("Bobby!");
        //println!("free distance in closer bucket:{}",*free_distance);
		let mut free_dist = VIRTUAL_BUCKET_CAPACITY-1u;
		while 0 < free_dist {
		    let mut move_bucket_hop_info = self.raw_table.get_bucket(move_bucket_index).hop_info.clone();
			// This is for checking the hop info is not changed later, not usable when we are singlethreaded
            //let start_hop_info = move_info;
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
            //println!("Bobby plz!");
            // This is the check we do, to see if the hop info is changed, 
            // because this bucket is the virtual bucket we insert into.
            // Lock()
			//if start_hop_info == self.raw_table.get_bucket((closer_virtual_bucket+iter) & mask).hop_info {
                let new_free_bucket_index = (move_bucket_index + move_free_distance) & mask;

                self.raw_table.get_bucket(move_bucket_index).hop_info |= (1<<free_dist);
                let new_free_bucket_val = self.raw_table.get_val(new_free_bucket_index).clone();
                self.raw_table.insert_val(free_bucket_index,new_free_bucket_val);
                let new_free_bucket_key = self.raw_table.get_key(new_free_bucket_index).clone();
                self.raw_table.insert_key(free_bucket_index,new_free_bucket_key);

                self.raw_table.get_bucket(move_bucket_index).hop_info &= !(1u32<<move_free_distance);

                let new_free_bucket = self.raw_table.get_bucket(new_free_bucket_index).clone();
                self.raw_table.get_bucket(free_bucket_index).hop_info = new_free_bucket.hop_info;
                self.raw_table.get_bucket(free_bucket_index).hash = new_free_bucket.hash;
			    *free_distance -= free_dist;
			    return;
		    }
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
        //println!("Insert hashes to: {}",new_hash)
		let mask = self.raw_table.capacity()-1;
		let index_addr = mask & (new_hash as uint);
		//let mut info = self.get_insert_bucket_info(index_addr,mask);
        //let mut info = self.raw_table.get_bucket(index_addr).hop_info.clone();
        let mut free_distance = 0u;
		let mut val = 1;
		//for i in range(1,  ADD_RANGE){
		//	if (info & 1) == 0 {
		//		break;
		//	}
		//	info = info >> 1;
		//	let b_info = self.raw_table.get_bucket((index_addr+i) & mask).hop_info.clone();
        //    info = info | b_info;
        //    //println!("info in insert: {}",info);
		//	free_distance += 1;
		//}
        for i in range(0,ADD_RANGE){
            if !self.raw_table.get_key_option((index_addr+i) & mask) {
                break;
            }
            free_distance += 1;
        }
        //println!("free_distance in insert: {}",free_distance);
		if free_distance < ADD_RANGE {
			while val != 0 {
				if free_distance < VIRTUAL_BUCKET_CAPACITY {
                    //assert!(start_info & (1<<free_distance) != 0); 
                    //println!("info:{}",info);
                    //println!("index address:{}",index_addr);
                    //println!("free distance at insert:{}",free_distance);
                    self.raw_table.get_bucket(index_addr).hop_info |= 1<<free_distance;
					//println!("address:{}",(index_addr + free_distance) & mask);
					self.raw_table.get_bucket((index_addr + free_distance) & 
                                                        mask).hash = new_hash;
					self.raw_table.insert_key((index_addr + free_distance) & 
                                                                    mask, key.clone());
					self.raw_table.insert_val((index_addr + free_distance) & 
                                                                    mask, data.clone());
					self.size += 1;
					return true;
                    
				}
                //println!("free distance before closer bucket:{}",free_distance);
				self.find_closer_bucket(&mut free_distance, index_addr, &mut val, mask);
                //println!("free distance after closer bucket:{}",free_distance);
			}
		}
        println!("free_distance:{}",free_distance);
	    self.resize();
		self.insert(key.clone(), data.clone())
    }
    #[allow(dead_code)]
    pub fn insert_resize(&mut self, key:K, data:V,add_range:uint)-> bool{
		if self.check_key(&key) {
			return false;
		}
		let new_hash = self.hasher.hash(&key);
        //println!("Insert hashes to: {}",new_hash)
		let mask = self.raw_table.capacity()-1;
		let index_addr = mask & (new_hash as uint);
		//let mut info = self.get_insert_bucket_info(index_addr,mask);
        //let mut info = self.raw_table.get_bucket(index_addr).hop_info.clone();
        let mut free_distance = 0u;
		let mut val = 1;
		//for i in range(1,  ADD_RANGE){
		//	if (info & 1) == 0 {
		//		break;
		//	}
		//	info = info >> 1;
		//	let b_info = self.raw_table.get_bucket((index_addr+i) & mask).hop_info.clone();
        //    info = info | b_info;
        //    //println!("info in insert: {}",info);
		//	free_distance += 1;
		//}
        for i in range(0,add_range){
            if !self.raw_table.get_key_option((index_addr+i) & mask) {
                break;
            }
            free_distance += 1;
        }
        //println!("free_distance in insert: {}",free_distance);
		if free_distance < add_range {
			while val != 0 {
				if free_distance < VIRTUAL_BUCKET_CAPACITY {
                    //assert!(start_info & (1<<free_distance) != 0); 
                    //println!("info:{}",info);
                    //println!("index address:{}",index_addr);
                    //println!("free distance at insert:{}",free_distance);
                    self.raw_table.get_bucket(index_addr).hop_info |= 1<<free_distance;
					//println!("address:{}",(index_addr + free_distance) & mask);
					self.raw_table.get_bucket((index_addr + free_distance) & 
                                                        mask).hash = new_hash;
					self.raw_table.insert_key((index_addr + free_distance) & 
                                                                    mask, key.clone());
					self.raw_table.insert_val((index_addr + free_distance) & 
                                                                    mask, data.clone());
					self.size += 1;
					return true;
                    
				}
                //println!("free distance before closer bucket:{}",free_distance);
				self.find_closer_bucket(&mut free_distance, index_addr, &mut val, mask);
                //println!("free distance after closer bucket:{}",free_distance);
			}
		}
        false
    }
    pub fn resize(&mut self){
        println!("Resize!!!");
        let new_capacity = self.raw_table.capacity() << 1;
        //println!("new capacity:{}",new_capacity);
        let old_table = replace(&mut self.raw_table,raw_table::RawTable::new(new_capacity));
        let old_capacity = old_table.capacity();
        let mut info = 0;
        for i in range(0,old_capacity){
            /*info = info | old_table.get_i_bucket(i).hop_info;
            //println!("info:{}",info);
            if info & 1 == 1 {
                self.insert(old_table.get_key(i).clone(),old_table.get_val(i).clone());
            }
            info = info >> 1;*/
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
