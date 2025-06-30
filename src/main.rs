mod utils;

use crate::utils::json::IContact;
use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

fn main() {
    loop {
        print!("\x1B[2J\x1B[H");
        const OPTIONS: [&'static str; 5] = [
            "Search Contact",
            "Add Contact",
            "Update Contact",
            "Delete Contact",
            "Exit",
        ];
        let mut option_index: String = String::new();
        let contacts: Vec<IContact> = vec![
            IContact {
                first_name: "Pedro",
                last_name: "Diaz",
                phone_number: 523327853326,
            },
            IContact {
                first_name: "Juan",
                last_name: "Pérez",
                phone_number: 5512345678,
            },
            IContact {
                first_name: "Luis",
                last_name: "Hernández",
                phone_number: 5534567890,
            },
        ];
        let contacts_length: usize = contacts.len();
        const OPTIONS_LENGTH: usize = OPTIONS.len();

        println!("Hello! This is the contact list.");

        utils::welcome::welcome(
            contacts_length,
            (&contacts).to_vec(),
            OPTIONS,
            OPTIONS_LENGTH,
        );

        print!("What do you want to do now? ");
        stdout().flush().unwrap();
        let _ = stdin().read_line(&mut option_index);

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
                let mut search: Vec<IContact> = Vec::new();
                let mut key_to_search = String::new();
                print!("Search => ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut key_to_search);

                for i in 0..contacts_length {
                    let contact = &contacts[i];

                    let include_first_name = contact.first_name.contains(&key_to_search.trim());
                    let include_last_name = contact.last_name.contains(&key_to_search.trim());
                    let include_phone_number = contact
                        .phone_number
                        .to_string()
                        .contains(&key_to_search.trim());

                    if include_first_name == true
                        || include_last_name == true
                        || include_phone_number == true
                    {
                        search.push(contact.clone());
                    }
                    continue;
                }
                utils::show_contacts(search.len(), (&search).to_vec());
                for searched in search {
                    println!("{}", searched.first_name)
                }
                println!();
                thread::sleep(time::Duration::from_secs(10));
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
