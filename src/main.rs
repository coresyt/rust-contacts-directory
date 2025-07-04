mod utils;
mod operations;

use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

fn main() {
    print!("\x1B[2J\x1B[H");
    'main_loop: loop {
        const OPTIONS: [&'static str; 5] = [
            "Search Contact",
            "Add Contact",
            "Update Contact",
            "Delete Contact",
            "Exit",
        ];
        let mut option_index: String = String::new();
        let contacts: Vec<utils::json::IContact> = utils::json::get_contacts();
        let contacts_length: usize = contacts.len();
        const OPTIONS_LENGTH: usize = OPTIONS.len();
        const OPTIONS_TO_UPDATE: [&str; 3] = ["First Name", "Last Name", "Phone Number"];

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
            Err(_e) => continue 'main_loop,
        };

        //    Match the option you want to execute
        //     ? 1 ------------> Search Contact
        //     ? 2 ------------> Add Contact
        //     ? 3 ------------> Update Contact
        //     ? 4 ------------> Delete Contact
        //     ? 5 or _ -------> Exit
        match option_index_int {
            1 => operations::search::function(contacts.clone(), contacts.len()),
            2 => {
                let _ = operations::add::function();
            },
            3 => {
                let _ = operations::update::function(contacts, contacts_length, OPTIONS_TO_UPDATE);
            },
            4 => {
                let _ = operations::delete::function(contacts, contacts_length);
            }
            5 | _ => {
                print!("Exit...");
                thread::sleep(time::Duration::from_secs(1));
                print!("1...");
                stdout().flush().unwrap();

                thread::sleep(time::Duration::from_secs(1));
                print!("2...");
                stdout().flush().unwrap();

                thread::sleep(time::Duration::from_secs(1));
                print!("3...");
                stdout().flush().unwrap();
                print!("\x1B[2J\x1B[H");

                break 'main_loop;
            }
        }
    }
}
