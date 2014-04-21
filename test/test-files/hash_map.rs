#![feature(default_type_params)]
extern crate rand;

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
//extern crate collections;

#[cfg(test)]
mod test_hopscotch{
    //use collections::hashmap::HashMap;
    use super::rand;
    use super::rand::Rng;
    use std::hash::{Hash,Hasher,sip};
    use super::hopscotch::{HashMap};
    use super::raw_table::VIRTUAL_BUCKET_CAPACITY;

    #[test]
    fn test_lookup_without_insert(){
        let m:HashMap<int,int> = HashMap::new();
        let key = 5;
        let val = 6;
        // Configure raw table
        let r = m.getRawTable();
        let hasher = m.getSipHasher();
        let raw_address = hasher.hash(&key) & ((r.capacity()-1u) as u64);
        r.insert_key((raw_address as uint),key);
        r.insert_val((raw_address as uint),val);
        let op = m.lookup(key);
        match op{
            Some(var) => assert!(*var == val),
            None => fail!("lookup doesn't work!")
        }
    }
    #[test]
    fn test_lookup_with_insert(){
        let m:HashMap<uint,uint> = HashMap::with_capacity(500);
        for i in range(0u,256u){
            m.insert(i,i+1);
        }
        for i in range(0u,256u){
            let op = m.lookup(i);
            match op{
                Some(var) => assert!(*var == i+1),
                None => fail!("lookup doesn't work!")
            }
        }
    }
    #[test]
    fn test_insert_without_lookup(){
        let m:HashMap<int,int> = HashMap::new();
        let key = 60;
        let val = 567;
        m.insert(key,val);
        let r = m.getRawTable();
        let hasher = m.getSipHasher();
        let hash = hasher.hash(&key);
        let raw_address = hash & ((r.capacity()-1u) as u64);
        assert!(*r.get_key((raw_address as uint)) == key);
        assert!(*r.get_val((raw_address as uint)) == val);
        // Try with many randomly generated numbers
        let m2:HashMap<uint,uint> = HashMap::with_capacity(500);
        let mut rng = rand::task_rng();
        for i in range(0u,200u){
            let key2 = rng.gen();
            let val2 = rng.gen();
            m2.insert(key2,val2);
            let r2 = m2.getRawTable();
            let hasher2 = m2.getSipHasher();
            let hash2 = hasher2.hash(&key2);
            let raw_address2 = hash2 & (r2.capacity() as u64);
            // Check if bucket has only one element placed at the raw address
            if r2.get_bucket((raw_address2 as uint)).hop_info == 1{
                assert!(*r2.get_key((raw_address2 as uint)) == key2);
                assert!(*r2.get_val((raw_address2 as uint)) == val2);
            // Looking through virtual bucket to assert it really is put here
            } else {
                let mask = r2.capacity()-1;
                let hit = false;
                let info = r2.get_bucket((raw_address2 as uint)).hop_info;
                for i in range(1u,VIRTUAL_BUCKET_CAPACITY-1){
                    if info & 1 == 1{
                        if *r2.get_key((raw_address2 as uint)+i & mask) == key2{
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
        let m:HashMap<uint,uint> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        let op = m.remove(upper+1);
        assert!(op == None);

    }
    #[test]
    fn test_remove_of_valid_key(){
        let m:HashMap<uint,uint> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        for i in range(1u,upper){
            let op = m.remove(i);
            match op {
                Some(var) => assert!(*var == i+2),
                None => fail!("remove doesn't work!")
            }
        }
    }
    #[test]
    fn test_remove_with_lookup(){
        let m:HashMap<uint,uint> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        for i in range(1u,upper){
            m.remove(i);
            assert!(m.lookup(i) == None);
        } 
    }
    #[test]
    fn test_resize(){

    }
    #[test]
    fn test_insert_after_resize(){

    }
    #[test]
    fn test_lookup_after_resize(){

    }
    #[test]
    fn test_remove_after_resize(){

    }
}
