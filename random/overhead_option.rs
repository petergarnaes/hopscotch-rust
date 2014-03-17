extern crate test;

#[cfg(test)]
mod tests{
    use test::BenchHarness;
    use std::vec;

    fn option_fun(v: &[Option<uint>]){
        for e in v.iter(){
            match *e {
                Some(b) if b == 0 => {},
                _ => {}
            }
        }
    }
    fn compare_fun(v: &[uint]){
        for e in v.iter(){
            if(*e == 0){}
        }
    }
    #[bench]
    fn bench_option(b: &mut BenchHarness){
        let c: ~[Option<uint>] = vec::from_fn(1000,|n| Some(n+1));
        b.iter(|| option_fun(c));
    }
    
    #[bench]
    fn bench_compare(b: &mut BenchHarness){
        let c: ~[uint] = vec::from_fn(1000,|n| n+1);
        b.iter(|| compare_fun(c));
    }
}
