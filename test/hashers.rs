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

pub struct HasherSameBucket{
    table_size:u64,
    bucket_idx:u64,
}

impl HasherSameBucket{
    pub fn new(idx:u64,size:u64)->HasherSameBucket{
        HasherSameBucket{table_size:size,bucket_idx:idx}
    }
}

impl Hasher<HashSameBucket> for HasherSameBucket{
    fn hash<T:Hash<HashSameBucket>>(&self,value:&T)->u64{
        let mut state = HashSameBucket{
            table_size:self.table_size,
            bucket_idx:self.bucket_idx,
            hash:0};
        value.hash(&mut state);
        state.hash
    }
}

struct HashSameBucket{
    table_size:u64,
    bucket_idx:u64,
    hash: u64,
}

impl Writer for HashSameBucket {
    fn write(&mut self, buf: &[u8]) -> IoResult<()>{
        let mut a = 0u64;
        for byte in buf.iter(){
            a += *byte as u64
        }
        let b = self.table_size * a;
        self.hash = self.bucket_idx + b;
        //println!("hash:{}",self.hash);
        //self.hash = 1u64;
        Ok(())
    }
}

pub struct HasherOddDisplace{
    table_size:u64,
    bucket_idx:u64,
}

impl HasherOddDisplace{
    pub fn new(idx:u64,size:u64)->HasherOddDisplace{
        HasherOddDisplace{table_size:size,bucket_idx:idx}
    }
}

impl Hasher<HashOddDisplace> for HasherOddDisplace{
    fn hash<T:Hash<HashOddDisplace>>(&self,value:&T)->u64{
        let mut state = HashOddDisplace{
            table_size:self.table_size,
            bucket_idx:self.bucket_idx,
            hash:0};
        value.hash(&mut state);
        state.hash
    }
}

struct HashOddDisplace{
    table_size:u64,
    bucket_idx:u64,
    hash: u64,
}

impl Writer for HashOddDisplace {
    fn write(&mut self, buf: &[u8]) -> IoResult<()>{
        let mut a = 0u64;
        for byte in buf.iter(){
            a += *byte as u64
        }
        let b = self.table_size * a;
        self.hash = self.bucket_idx + b + (a & 1);
        //println!("hash:{}",self.hash);
        //self.hash = 1u64;
        Ok(())
    }
}
