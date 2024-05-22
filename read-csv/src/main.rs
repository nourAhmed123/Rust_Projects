use std::error::Error;
use csv;

// Rust doesnot have exception errors it have panics 
//Error handling use Result 
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>  //* The path we send in the main function 
 {   let mut reader = csv::Reader::from_path(path)?; //  (?)  used instead of using match 
for result in reader.records(){
    let record = result?;
    println!("{:?}",record);
}
Ok(())
}
fn main(){
    if let Err(e)=read_from_file("./customers.csv"){
        eprintln!("{}",e);
    }
}