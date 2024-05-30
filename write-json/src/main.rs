use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize)]
struct Paragraph{
    name:String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article: String,
    author:String,
    paragraph: Vec<Paragraph> //collection of paragraphs
}
fn main(){
    let article: Article=Article{
       article: String::from("how to work with json in rust ") ,
       author:String::from("akhil"),
       paragraph: vec![
        Paragraph{
            name:String::from("first sentence ")
        },
        Paragraph{
            name: String::from("body of the paragraph")
        },
        Paragraph{
            name: String::from("end of the paragraph")
        }
       ]
    };
       // Serializing the Article instance to a JSON string
    let json= serde_json::to_string(&article).unwrap();
    println!("the json is: {} ", json )
}