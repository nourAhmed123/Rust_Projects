use serde::ser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;
#[derive(Serialize, Deserialize)]


pub struct ServiceInfo{
 pub service : String,
 pub username: String,
 pub password: String,
}
impl ServiceInfo{
    pub fn new(service:String , username:String, password:String) -> Self{
        ServiceInfo{
          service,
          username,
          password,
        }
    }
    pub fn from_json(json_string: &str)->Result<Self,serde_json::Error>{
       serde_json::from_str(json_string)
    }
    #[allow(dead_code)]
    pub fn from_user_input() ->Self{
        println!("Enter your password Entry:");
        let mut service = String::new();
        io::stdin()
        .read_line(&mut service)
        .expect("failed to readline");

    println!("Enter username:");
    let mut username = String::new();
    io::stdin()
    .read_line(&mut username)
    .expect("failed to readline");

    println!("Enter password:");
    let mut password = String::new();
    io::stdin()
    .read_line(&mut password)
    .expect("failed to readline");

ServiceInfo::new(
service.trim().to_string(),
username.trim().to_string(),
password.trim().to_string(),
   ) }
   pub fn to_json(&self)->String{
    serde_json::to_string(&self).expect("failed to serializ json")
 }

pub fn write_to_file(&self){
    let json_output= format!("{} \n",self.to_json());
    match OpenOptions::new().create(true).append(true).open("passwords.json")
    {
        Ok(mut file)=>{
            file.write_all(json_output.as_bytes())
            {

                eprintln!("Error writing to file: {} ", e);

            }
            else{
                println!("sucessfully print passwords to json ")
            }
        }
        Err(e)=>println!("Error opening file:{}",e),
    }
}
}

pub fn read_passwords_from_file()->Result<Vec<ServiceInfo> , io::Error>{
let file = File::open("passwords.json")?;
let reader= io::BufReader::new(file);
let mut services = Vec::new();
for line in reader.lines(){
    if let Ok(json_string)=line{
        if let Ok(service_info)= ServiceInfo::from_json(&json_string){
            services.push(service_info);
        }
    }
}
}
pub fn prompt(prompt :&str)->String{
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = string::new();
    io::read_line(&mut input).unwrap();
    input.trim().to_string()

}