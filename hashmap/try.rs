#[feature(default_type_params)];

use std::container::{Map};
use std::hash::{Hash,Hasher};
use  std::hash::sip::{SipHasher};
use std::option::{Option,Some,None};
use std::vec_ng::Vec;

struct Bucket<K,V>{
    hop_info: uint,
    hash: u64,
    key: K,
    val: V,
}

impl<K,V> Bucket<K,V>{
    fn hash(&self)->u64{
        self.hash
    }
    fn val(&self)->V{
        self.val
    }
    fn hop_info(&self)->uint{
        self.hop_info
    }
}

static HOP_RANGE: uint = 32;

struct HashMap<K,V,H=SipHasher>{
    priv hasher: H,
    //Filled elements
    priv size: uint,
    //Max elements there is room for
    priv capacity: u64,
    buckets: Vec<Option<Bucket<K,V>>>,
}

impl<K: Eq + Hash<S>,V,S,H: Hasher<S>> HashMap<K,V,H>{
    // Help functions
    pub fn new() -> HashMap<K,V,SipHasher>{
        let hasher = SipHasher::new_with_keys(12u64,14u64);
        HashMap{
            hasher: hasher,
            size: 0,
            capacity: 8,
            buckets: Vec::from_fn(8,|_| None),
        }
    }
}
impl<K: Eq + Hash<S>, V, S, H: Hasher<S>> Container for HashMap<K,V,H>{
    fn len(&self) -> uint{
        self.size
    }
}
impl<K: Eq + Hash<S>, V, S, H: Hasher<S>> Map<K,V> for HashMap<K,V,H>{
    fn find(&self,k: &K) -> Option<V>{
        // Hash key
        let hash = self.hasher.hash(k);
        // AND it with mask to find start bucket
        let address = (hash & (self.capacity-1)) as uint;
        let hop_range_buckets = self.buckets.slice(address,address+HOP_RANGE);
        // Iterate over hop range, look at hop info to check if we compare
        // key
        let hop_info = match hop_range_buckets[0]{
            Some(b) => b.hop_info(),
            None => fail!("Fuck!")
        };
        for i in range(1u,HOP_RANGE){
            let temp=hop_info;
            temp>>i;
            if(temp&i==1){
                match hop_range_buckets[i]{
                    Some(b) => {
                        if(hash == b.hash()){
                            return Some(b.val());
                        }
                    },
                    None => fail!("Opps!")
                }
            }
        }
        // Else return None
        None
    }
    fn contains_key(&self,k: &K) -> bool {
        true
    }
}

fn main(){
    let b = Bucket{
        hop_info: 10u,
        hash: 12u64,
        key: 10,
        val: 40,
    };
}
