// Everything is tested and works!
// Now you include it by writing "mod output-benchmark;" and call "write_output"
use std::io;
use std::result::Result;

// Helping function turning the lists into a string where elements are 
// separated by spaces
fn stringify_list<T:ToStr>(list: &[T])->~str{
    let mut res:~str = ~"";
    for l in list.iter(){
        res = res + " " + l.to_str();
    }
    res
}

// Main public function outputting benchmark data correctly
pub fn write_output<T:ToStr,K:ToStr>(filename:&str,title:&str,x_axis:&str,
        y_axis:&str,data_points:&[T],results:&[K])->Result<(),std::io::IoError>{
    let mut file = io::File::create(&Path::new(
            "output-files/"+filename+"-output"));
    let output = title + "\n" + x_axis + "\n" + y_axis + "\n" + 
        stringify_list(data_points) + "\n" + stringify_list(results);
    file.write(output.into_bytes())
}