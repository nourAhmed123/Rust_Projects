extern crate flate2;
use flate2::write::GzEncoder; //is a type of encoder that compresses data into the gzip format.
use flate2::Compression; // It allows you to specify the level of compression when creating a compression encoder.
use std::env::args;// accessing the command-line arguments provided when running the program.
use std::fs::File; // provides methods for reading from and writing to files.
use std::io::copy; //copying data between different IO streams, like files or network connections.
use std::io::BufReader; //a buffered reader that reads data from an underlying reader (like a file) more efficiently by reducing the number of reads and system calls.
use std::time::Instant; //will use  it to  show how much time it take to copy a file or compress  
//cargo run mohamed2021.pdf compressed_file  (command to run )
fn main(){
if args().len()!=3{  
    eprintln!("usage: `source` `target`");
    return;
}
   // Here, we're opening the file specified as the first command-line argument and creating a BufReader to efficiently read from it.
let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
let output = File::create(args().nth(2).unwrap()).unwrap();
let mut encoder= GzEncoder::new(output,Compression::default());
let start = Instant::now();
copy(&mut input , &mut encoder).unwrap();
let output=encoder.finish().unwrap();
println!(
   "Source len : {:?}" , input.get_ref().metadata().unwrap().len()
);
println!("Target len : {:?}",output.metadata().unwrap().len());
println!("Elapsed:{:?}",start.elapsed());
}