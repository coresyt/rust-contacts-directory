mod utils;

use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

fn main() {
    print!("\x1B[2J\x1B[H");
    loop {
        // let _ = json::read_file();
        const OPTIONS: [&'static str; 5] = [
            "Search Contact",
            "Add Contact",
            "Update Contact",
            "Delete Contact",
            "Exit",
        ];
        let mut option_index: String = String::new();
        let contacts: Vec<utils::json::IContact> = vec![
            utils::json::IContact {
                first_name: "Pedro",
                last_name: "Diaz",
                phone_number: 523327853326,
            },
            utils::json::IContact {
                first_name: "Juan",
                last_name: "Pérez",
                phone_number: 5512345678,
            },
            utils::json::IContact {
                first_name: "Luis",
                last_name: "Hernández",
                phone_number: 5534567890,
            },
        ];
        let contacts_length: usize = contacts.len();
        const OPTIONS_LENGTH: usize = OPTIONS.len();

        println!("Hello! This is the contact list.");

        utils::welcome::welcome(contacts_length, contacts, OPTIONS, OPTIONS_LENGTH);

        print!("What do you want to do now? ");
        stdout().flush().unwrap();
        let _ = stdin().read_line(&mut option_index);
        println!();

        let option_index_int = match option_index.trim().parse() {
            Ok(c) => c,
            Err(_e) => continue,
        };

        //    Match the option you want to execute
        //     ? 1 -------> Search Contact
        //     ? 2 -------> Add Contact
        //     ? 3 -------> Update Contact
        //     ? 4 -------> Delete Contact
        //     ? 5 -------> Exit
        //     ? _ -------> Retry Question
        match option_index_int {
            1 => {
                println!("Searching...")
            }
            2 => {
                println!("Creating...")
            }
            3 => {
                println!("Updating...")
            }
            4 => {
                println!("Deleting...")
            }
            5 => {
                println!("Exit...");
                print!("1...");
                stdout().flush().unwrap();
                thread::sleep(time::Duration::from_secs(1));
                
                print!("2...");
                stdout().flush().unwrap();
                thread::sleep(time::Duration::from_secs(1));
                
                print!("3...");
                stdout().flush().unwrap();
                thread::sleep(time::Duration::from_secs(1));
                break;
            }
            _ => {
                println!("Retry!!!");
                continue;
            }
        }
    }
}
