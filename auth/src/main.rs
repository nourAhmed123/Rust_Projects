//The blocking Client will block the current thread to execute, instead of returning futures that need to be executed on a runtime.
use reqwest::blocking::Client;
use reqwest::Error;

fn main()->Result<(),Error>{
    //creating new client coming from the reqwest blocking client 
    let client = Client::new();
    //because the link requires user and passwd 
    let user="testuser".to_string();
    let passwd:Option<String>=None;
 let response= client
 .get("http://httpbin.org/get")
 .basic_auth(user,passwd)
 .send();
println!("{:?}",response);
Ok(())
}