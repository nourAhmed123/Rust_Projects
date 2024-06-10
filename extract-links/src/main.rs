use error_chain::error_chain;
use select::document::Document;// For parsing HTML documents
use select::predicate::Name; // For selecting HTML elements by name
error_chain!{
    foreign_links{
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}
#[tokio::main]
async fn main()->Result<()>{
    // Send a GET request to the specified URL and await the response
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await? // Await the response and handle errors
        .text() // Convert the response body to text
        .await?; // Await the text conversion and handle errors

Document::from(res.as_str())
  // Find all <a> elements (hyperlinks) in the document
.find(Name("a"))
   // Filter the elements to only include those with an "href" attribute
.filter_map(|n| n.attr("href"))
      // For each "href" attribute value, print it
.for_each(|x|println!("{}",x));
Ok(())
}