extern crate test;
use output_benchmark::write_output;
mod output_benchmark;

#[cfg(test)]
mod tests{
    use std::vec;
    use test::BenchHarness;
    fn vector_sum(size: uint) -> uint{
        let v = vec::from_fn(size,|n| n);
        let mut sum:uint = 0;
        for e in v.iter(){
            sum += *e;
        }
        sum
    }

    #[bench]
    fn sum_1000(b: &mut BenchHarness){
        b.iter(|| vector_sum(1000));
    }
    #[bench]
    fn sum_2000(b: &mut BenchHarness){
        b.iter(|| vector_sum(2000));
    }
    #[bench]
    fn sum_3000(b: &mut BenchHarness){
        b.iter(|| vector_sum(3000));
    }
    #[bench]
    fn sum_4000(b: &mut BenchHarness){
        b.iter(|| vector_sum(4000));
    }
}

fn main(){
    //Need to define output strings
    let filename = ~"vector-tests";
    let title = ~"Vector sum speed";
    let x_axis = ~"Table size";
    let y_axis = ~"speed";
    let data_points = ~[1000,2000,3000,4000];
    write_output(filename,title,x_axis,y_axis,data_points);
}
