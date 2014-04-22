use std::hash::{Hasher,Hash};
use std::io::IoResult;

struct HasherZeroAndUp;

impl Hasher<HashZeroAndUp> for HasherZeroAndUp{
    fn hash<T:Hash<HashZeroAndUp>>(&self,value:&T)->u64{
        let mut state = HashZeroAndUp{hash:0};
        value.hash(&mut state);
        state.hash
    }
}

struct HashZeroAndUp{
    hash: u64,
}

impl Writer for HashZeroAndUp {
    fn write(&mut self, buf: &[u8]) -> IoResult<()>{
        for byte in buf.iter(){
            self.hash += *byte as u64
        }
        //self.hash = 1u64;
        Ok(())
    }
}

pub struct HasherSameBucket;

impl Hasher<HashSameBucket> for HasherSameBucket{
    fn hash<T:Hash<HashSameBucket>>(&self,value:&T)->u64{
        let mut state = HashSameBucket{hash:0};
        value.hash(&mut state);
        state.hash
    }
}

struct HashSameBucket{
    hash: u64,
}

impl Writer for HashSameBucket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()>{
        let mut a = 0u64;
        for byte in buf.iter(){
            a += *byte as u64
        }
        let b = 32u64 * a;
        self.hash = 33u64 + b;
        //println!("hash:{}",self.hash);
        //self.hash = 1u64;
        Ok(())
    }
}
