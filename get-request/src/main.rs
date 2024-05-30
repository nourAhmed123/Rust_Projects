use error_chain::error_chain; // provides easy error handling. It helps in chaining errors and providing detailed error messages.
use std::io::Read; // trait that provides methods for reading bytes from a source.

//optional
error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main()->Result<()>{ // it either returns Ok(()) on success or an error defined by error_chain on failure.
    let mut res=reqwest::blocking::get("http://httpbin.org/get")?;
let mut body=String::new();
res.read_to_string(&mut body)?; //The ? operator is used to automatically convert any errors into our custom error type and return them from the function if they occur.
println!("status:{}",res.status());
println!("Headers:\n{:#?}",res.headers());
println!("Body: \n{}", body);
Ok(())
}