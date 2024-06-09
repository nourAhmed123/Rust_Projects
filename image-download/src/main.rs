use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;

error_chain!{
    foreign_links{
       Io(std::io::Error);
       HttpRequest(reqwest::Error);

    }
}

//allows the use of async/await syntax in the main function.

#[tokio::main]

async fn main()->Result<()>{
      // Create a temporary directory with a prefix "example"
    let tmp_dir=Builder::new().prefix("example").tempdir()?;
   // Define the URL of the file to download
    let target="https://www.rust-lang.org/logos/rust-logo-512x512.png";
   // Send a GET request to the target URL
    let response=reqwest::get(target).await?;
 // Determine the filename from the URL path
let mut dest={
    let fname=response
    .url()
    .path_segments() // Get the path segments of the URL
    .and_then(|segments|segments.last()) // Get the last segment
    .and_then(|name|if name.is_empty(){None}else {Some(name)}) // Check if the name is not empty
    .unwrap_or("tmp.bin"); // Default to "tmp.bin" if no name is found
println!("File to download : '{}'",fname);      // Print the filename
  // Create the full path for the temporary file
let fname=tmp_dir.path().join(fname);
println!("File will be Located under: '{:?}' ",fname);

        // Create the file

File::create(fname)?
};
    // Get the content of the response as text
let content =  response.text().await?;
   // Copy the content to the destination file
copy(&mut content.as_bytes(), &mut dest)?;
Ok(())
}
