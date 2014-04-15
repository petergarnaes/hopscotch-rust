#[path="../../hashmap/raw_table/mod.rs"]
mod raw_table;

#[cfg(test)]
mod test_raw_table{
    use std::mem::replace;
    use std::num;
    use super::raw_table::Bucket;
    use super::raw_table::RawTable;
    #[test]
    fn test_init(){
        // Init with minimum capacity
        let m:RawTable<int,int> = RawTable::new(52);
        assert!(m.capacity() == 64);
        // Init with larger capacity, ensures rounding up
        let m2:RawTable<int,int> = RawTable::new(100);
        assert!(m2.capacity() == 128)
    }
    #[test]
    fn test_virtual_bucket(){
        let mut m:RawTable<int,int> = RawTable::new(60);
        for i in range(0u,m.capacity()){
            let b = m.get_bucket(i);
            assert!(*b == Bucket{hop_info:0,hash:0});
        }
    }
    #[test]
    fn test_get(){
        let mut m:RawTable<uint,uint> = RawTable::new(60);
        let var = 40;
        for i in range(0u,var){
            m.insert_key(i,i);
            m.insert_val(i,i);
        }
        for i in range(0u,var){
            assert!(*m.get_key(i) == i);
            assert!(*m.get_val(i) == i);
        }
    }
    #[test]
    fn test_insert(){
        // This test just runs and ensures this will not fail
        let mut m:RawTable<uint,uint> = RawTable::new(120);
        for i in range(0u,m.capacity()){
            m.insert_key(i,i);
            m.insert_val(i,i);
        }
    }
    #[test]
    fn test_remove(){
        let mut m:RawTable<uint,uint> = RawTable::new(120);
        for i in range(0u,m.capacity()){
            m.insert_key(i,i);
            m.insert_val(i,i);
            m.remove_key(i);
            m.remove_val(i);
            assert!(*m.get_key(i) == 0);
            assert!(*m.get_val(i) == 0);
        }
    }
    #[test]
    fn test_bucket(){
        let mut m:RawTable<uint,uint> = RawTable::new(110);
        {
            let b = m.get_bucket(10);
            b.hop_info = 1;
            b.hash = 5;
        }
        {
            let b2 = m.get_bucket(10);
            assert!(b2.hop_info == 1);
            assert!(b2.hash == 5);
        }
        // Test replace method on buckets
        replace(m.get_bucket(10),Bucket{hop_info:2,hash:6});
        let b3 = m.get_bucket(10);
        assert!(b3.hop_info == 2);
        assert!(b3.hash == 6);
        // Test how shift actually works
        let mut var = num::next_power_of_two(60u);
        assert!(var == 64);
        var = var << 1;
        assert!(var == 128);
        var = var - 1;
        assert!(56 & var == 56);
        assert!(0 | 15 == 15);
        assert!(128 >> 1 == 64);
    }
    #[test]
    fn test_resize(){
        let mut m:RawTable<uint,uint> = RawTable::new(120);
        for i in range(0u,m.capacity()-1){
            m.insert_key(i,i);
            m.insert_val(i,i);
            let b = m.get_bucket(i);
            b.hop_info = 1;
            b.hash = i as u64;
        }
        for i in range(0u,m.capacity()-1){
            assert!(*m.get_key(i) == i);
            assert!(*m.get_val(i) == i);
            let b = m.get_bucket(i);
            assert!(b.hop_info == 1);
            assert!(b.hash == (i as u64));
        }
        let old_capacity = m.capacity();
        m.resize();
        assert!(m.capacity() == old_capacity << 1);
        for i in range(0u,old_capacity-1){
            //assert!(*m.get_key(i) == i);
            //assert!(*m.get_val(i) == i); 
            let b = m.get_bucket(i);
            assert!(b.hop_info == 1);
            assert!(b.hash == (i as u64));
        }
    }
}
