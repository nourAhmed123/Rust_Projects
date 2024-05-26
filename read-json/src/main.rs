use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,  // Note: field names should match the JSON keys
}

fn main() {
    let json = r#"
 {
    "article": "How to work with json in rust",
    "author": "akhil",
    "paragraph" : [
        {
            "name": "Starting sentence"
        },
        {
            "name" : "body of the paragraph"
        },
        {
            "name" : "end of the paragraph"
        }
    ]
}"#;
 // Parse the JSON string into an Article struct
    let parsed: Article = read_json_typed(json);
    println!(
        "\n\n the name of the first is paragraph is : {} ",
        parsed.paragraph[0].name
    );
}

// Function to read and parse JSON string into an Article struct
fn read_json_typed(raw_json : &str) -> Article{
       // Use serde_json to parse the JSON string

    let parsed: Article =serde_json::from_str(raw_json).unwrap();
     parsed   // Return the parsed Article struct
}