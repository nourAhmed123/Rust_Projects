mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;
// Function to clear the terminal screen.
fn clr() {
    print!("{}[2J", 27 as char);
}
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
        // Get user input for menu choice.
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        // Match user choice to corresponding action.
        match choice.trim() {
            "1" => {
                clr();
                // Create a new ServiceInfo entry with user input.
                let entry = ServiceInfo::new(
                    prompt("service :"),
                    prompt("username :"),
                    prompt("password :"),
                );
                println!("Entry added successfully");
                entry.write_to_file(); // Write the entry to the file.
            }
            "2" => {
                clr();
                // Read existing entries from the file.
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading password: {}", err);
                    Vec::new()
                });
                // Print each service entry.
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
                    Vec::new() // Return an empty vector on error.
                });
                // Get the search term from the user.
                let search = prompt("search:");
                // Print the matching service entry if found.
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
            _ => println!("invalid choice."),
        }
        println!("\n\n");
    }
}
