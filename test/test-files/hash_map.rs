#![feature(default_type_params)]

#[path="../../hashmap/mod.rs"]
mod hopscotch;
#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;
//extern crate collections;

#[cfg(test)]
mod test_hopscotch{
    //use std::hash::{Hash,Hasher,sip};
    //use collections::hashmap::HashMap;
    extern crate rand;
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
        let raw_address = hasher.hash(key) & r.capacity();
        r.insert_key(raw_address,key);
        r.insert_val(raw_address,val);
        assert!(m.lookup(key) == val);
    }
    #[test]
    fn test_lookup_with_insert(){
        let m:HashMap<int,int> = HashMap::with_capacity(500);
        for i in range(0u,256u){
            m.insert(i,i+1);
        }
        for i in range(0u,256u){
            assert!(m.lookup(i) == i+1);
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
        let hash = hasher.hash(key);
        let raw_address = hash & r.capacity();
        assert!(r.get_key(raw_address) == key);
        assert!(r.get_val(raw_address) == val);
        // Try with many randomly generated numbers
        let m2:HashMap<int,int> = HashMap::with_capacity(500);
        let mut rng = rand::task_rng();
        for i in range(0u,200u){
            let key = rng.gen();
            let val = rng.gen();
            m2.insert(key,val);
            let r = m2.getRawTable();
            let hasher = m2.getSipHasher();
            let hash = hasher.hash(key);
            let raw_address = hash & r.capacity();
            // Check if bucket has only one element placed at the raw address
            if r.get_bucket(raw_address).hop_info == 1{
                assert!(r.get_key(raw_address) == key);
                assert!(r.get_val(raw_address) == val);
            // Looking through virtual bucket to assert it really is put here
            } else {
                let hit = false;
                let info = r.get_bucket(raw_address).hop_info;
                for i in range(1u,VIRTUAL_BUCKET_CAPACITY-1){
                     if info & 1 == 1{
                            if r.get_key(raw_address+i) == key{
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
        let m:HashMap<int,int> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        let op = m.remove(upper+1);
        assert!(op == None);

    }
    #[test]
    fn test_remove_of_valid_key(){
        let m:HashMap<int,int> = HashMap::with_capacity(200);
        let upper = 180u;
        for i in range(1u,upper){
            m.insert(i,i+2);
        }
        for i in range(1u,upper){
            let op = m.remove(i);
            assert!(op == Some(i+2));
        }
    }
    #[test]
    fn test_remove_with_lookup(){
        let m:HashMap<int,int> = HashMap::with_capacity(200);
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