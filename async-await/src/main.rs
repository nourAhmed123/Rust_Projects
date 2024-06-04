use error_chain::error_chain;

error_chain!{
    /*The foreign_links section within the macro is used to specify errors
      that originate from other libraries (foreign errors).
       These are errors that your code does not directly create, 
       but can occur during IO operations or HTTP requests.*/
    foreign_links{
     Io(std::io::Error); 
     HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main()->Result<()>{
    let res=reqwest::get("http://httpbin.org/get").await?;
println!("status:{}",res.status());
println!("Request");
println!("Headers:\n{:#?}",res.headers());
let body=res.text().await?;
println!("Body:\n{}",body);
Ok(())
}