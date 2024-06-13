mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;

fn main() {
    clr();
    let ascii = r#"
    .__   __    
    ___________    ______ _________  _______   __ __|  |_/  |_  
    \____ \__  \  /  ___//  ___/\  \/ /\__  \ |  |  \  |\   __\ 
    |  |_> > __ \_\___ \ \___ \  \   /  / __ \|  |  /  |_|  |   
    |   __(____  /____  >____  >  \_/  (____  /____/|____/__|   
    |__|       \/     \/     \/             \/                  
    "#;
    println!("{ascii}");
    loop {
        println!("password manager menu");
        println!("1- Add Entry");
        println!("2- list Entries");
        println!("3- search Entry");
        println!("4- quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                clr();
let entry =ServiceInfo::new(
    prompt("service :"),
    prompt("username :"),
    prompt("password :"),
);
println!("Entry added successfully")
entry.write_to_file();


            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading password: {}", err);
                    vec::new()
                });
                for item in &services {
                    println!(
                        "Service = {}
        - Username: {}
        - Password: {}
        ",
                        item.service, item.username, item.password
                    )
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading password: {}", err);
                    vec::new()
                });
                let search = prompt("search:");
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service = {}
            - Username: {}
            - Password: {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
          _=>println!("invalid choice.")  
        }
        println!("\n\n");
    }
}
