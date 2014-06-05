#![feature(default_type_params)]
#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
//extern crate collections;

#[cfg(test)]
mod test_hopscotch{
    //use collections::hashmap::HashMap;
    use std::rand::{task_rng,Rng};
    use std::hash::{Hash,Hasher,sip};
    use std::clone::Clone;
    use std::default::Default;
    use super::hopscotch;
    use super::hopscotch::{HashMap};
    use super::raw_table::RawTable;
    use super::raw_table::VIRTUAL_BUCKET_CAPACITY;

    fn hash_with_hasher<T:Hash>(hasher:&sip::SipHasher,key:&T)->u64{
        hasher.hash(key)
    }

    fn insert_key_val_in_raw_table<K:Default+Clone,V:Default+Clone>
            (r:&mut hopscotch::raw_table::RawTable<K,V>,key:K,val:V,hash:u64){
        let raw_address = hash & ((r.capacity()-1u) as u64);
        {
            let mut b = r.get_bucket(raw_address as uint);
            b.hash = hash;
            b.hop_info = 1;
        }
        r.insert_key((raw_address as uint),key);
        r.insert_val((raw_address as uint),val);

    }


    #[test]
    fn test_lookup_without_insert(){
        let mut m:HashMap<int,int> = HashMap::new();
        let key = 5;
        let val = 6;
        // Configure raw table
        let hash = hash_with_hasher(m.getSipHasher(),&key);
        insert_key_val_in_raw_table(m.getRawTable(),key,val,hash);
        let op = m.lookup(key);
        match op{
            Some(var) => assert!(*var == val),
            None => fail!("lookup doesn't work!")
        }
    }
    #[test]
    fn test_lookup_with_insert(){
        let mut m:HashMap<uint,uint> = HashMap::with_capacity(500);
        for i in range(1u,256u){
            m.insert(i,i+1);
        }
        for i in range(1u,256u){
            let op = m.lookup(i);
            match op{
                Some(var) => assert!(*var == i+1),
                None => fail!("lookup doesn't work!")
            }
        }
    }
    #[test]
    fn test_insert_without_lookup(){
        let mut m:HashMap<int,int> = HashMap::new();
        let key = 60;
        let val = 567;
        m.insert(key,val);
        let hash = m.getSipHasher().hash(&key);
        let r = m.getRawTable();
        let raw_address = hash & ((r.capacity()-1u) as u64);
        assert!(*r.get_key((raw_address as uint)) == key);
        assert!(*r.get_val((raw_address as uint)) == val);
        // Try with many randomly generated numbers
        let mut m2:HashMap<uint,uint> = HashMap::with_capacity(500);
        let mut rng = task_rng();
        for i in range(0u,200u){
            let key2 = rng.gen();
            let val2 = rng.gen();
            m2.insert(key2,val2);
            let hash2 = m2.getSipHasher().hash(&key2); 
            let r2 = m2.getRawTable();
            let raw_address2 = hash2 & ((r2.capacity()-1) as u64);
            // Check if bucket has only one element placed at the raw address
            if r2.get_bucket((raw_address2 as uint)).hop_info == 1{
                assert!(*r2.get_key((raw_address2 as uint)) == key2);
                assert!(*r2.get_val((raw_address2 as uint)) == val2);
            // Looking through virtual bucket to assert it really is put here
            } else {
                let mask = r2.capacity()-1;
                let mut hit = false;
                let mut info = r2.get_bucket((raw_address2 as uint)).hop_info;
                for j in range(0u,VIRTUAL_BUCKET_CAPACITY-1){
                    if info & 1 == 1{
                        if *r2.get_key((raw_address2 as uint)+j & mask) == key2{
                            hit = true;
                        }
                    }
                    info = info >> 1;
                }
                assert!(hit);
            }
        }
    }
    //#[test]
    //fn test_insert_with_lookup_rng_max_load_factor(){
    //
    //}
    #[test]
    fn test_remove_of_an_invalid_key(){
        let mut m:HashMap<uint,uint> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        let op = m.remove(upper+1);
        assert!(op == None);

    }
    #[test]
    fn test_remove_of_valid_key(){
        let mut m:HashMap<uint,uint> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        for i in range(1u,upper){
            let op = m.remove(i);
            match op {
                Some(var) => assert!(var == i+2),
                None => fail!("remove doesn't work!")
            }
        }
    }
    #[test]
    fn test_remove_with_lookup(){
        let mut m:HashMap<uint,uint> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        for i in range(1u,upper){
            m.remove(i);
            assert!(m.lookup(i) == None);
        } 
    }
    //#[test]
    //fn test_resize(){
//
  //  }
    //#[test]
    //fn test_insert_after_resize(){
//
    //}
    //#[test]
    //fn test_lookup_after_resize(){
//
    //}
    //#[test]
    //fn test_remove_after_resize(){
//
    //}
}
