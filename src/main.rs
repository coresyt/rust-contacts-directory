mod utils;

use crate::utils::json::IContact;
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
        let contacts: Vec<IContact> = utils::json::get_contacts();
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
            1 => {
                let mut search: Vec<IContact> = Vec::new();
                let mut key_to_search = String::new();
                print!("Search => ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut key_to_search);

                for i in 0..contacts_length {
                    let contact = &contacts[i];
                    let first_name = utils::normalize_string(contact.first_name.as_str());
                    let last_name = utils::normalize_string(contact.last_name.as_str());

                    let include_with_first_name =
                        first_name.contains(&utils::normalize_string(key_to_search.trim_end()));
                    let include_last_name = last_name.contains(&utils::normalize_string(key_to_search.trim_end()));
                    let include_phone_number = contact
                        .phone_number
                        .to_string()
                        .contains(&key_to_search.trim_end());

                    if include_with_first_name == true
                        || include_last_name == true
                        || include_phone_number == true
                    {
                        search.push(contact.clone());
                    }
                    continue;
                }
                utils::show_contacts(search.len(), (&search).to_vec());
                println!();
                thread::sleep(time::Duration::from_secs(3));
                print!("\x1B[2J\x1B[H");
                println!();
            }
            2 => {
                print!("\x1B[2J\x1B[H");
                let mut new_first_name = String::new();
                let mut new_last_name = String::new();
                let mut new_phone_number_str = String::new();
                let mut is_sure_str = String::new();

                print!("What will your first name be? ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut new_first_name);

                print!("What will your last name be? ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut new_last_name);

                print!("What is the phone number? ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut new_phone_number_str);

                let new_phone_number_int: u64 = match new_phone_number_str.trim().parse() {
                    Ok(c) => c,
                    Err(_e) => {
                        eprintln!("Your phone number provided is a invalid number.\nRetry!!!");
                        continue;
                    }
                };

                let new_contact = IContact {
                    first_name: String::from(new_first_name.trim_end()),
                    last_name: String::from(new_last_name.trim_end()),
                    phone_number: new_phone_number_int,
                };

                println!(
                    "Last Name is {}\nFirst Name is {}\nPhone Number is +{}",
                    new_contact.last_name, new_contact.first_name, new_contact.phone_number
                );
                print!("Are you sure you want to continue? (y/n) ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut is_sure_str);

                match (is_sure_str.to_lowercase()).as_str() {
                    "yes" | "y" => {
                        let is_created = utils::json::add_contact(new_contact);
                        if is_created == false {
                            println!();
                            thread::sleep(time::Duration::from_secs(10));
                            print!("\x1B[2J\x1B[H");
                            println!("There was an error in the create");
                            continue 'main_loop;
                        }
                        
                        print!("\x1B[2J\x1B[H");
                        continue;
                    }
                    "not" | "n" | _ => {
                        print!("\x1B[2J\x1B[H");

                        eprintln!("Canceled operation!!!");
                        continue;
                    }
                }
            }
            3 => {
                // * Buffers for the use of inputs
                let mut first_name_to_update: String = String::new();
                let mut option_to_update: String = String::new();
                let mut information_to_update: String = String::new();
                // * Temporary variables to perform the update
                let mut contact_to_update: IContact = IContact {
                    first_name: String::new(),
                    last_name: String::new(),
                    phone_number: 0,
                };
                let mut contact_to_update_is_find: bool = false;
                let mut contact_to_update_is_find_index: usize = 0;

                print!("Contact by update (provide first name) => ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut first_name_to_update);

                for i in 0..contacts_length {
                    let contact = contacts.clone()[i].clone();
                    let first_name = utils::normalize_string(contact.first_name.as_str());
                    let include_with_first_name =
                        first_name.trim_end() == utils::normalize_string(first_name_to_update.as_str()).trim_end();

                    if include_with_first_name == false {
                        continue;
                    }

                    if contact_to_update_is_find == true {
                        break;
                    }

                    contact_to_update = contact;
                    contact_to_update_is_find = true;
                    contact_to_update_is_find_index = i
                }

                for i in 0..(OPTIONS_TO_UPDATE.len()) {
                    let opt = OPTIONS_TO_UPDATE[i];
                    println!("{}. {}", i + 1, opt);
                }

                print!("What do you want to modify? ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut option_to_update);
                let option_to_update_int = match option_to_update.trim().parse::<u32>() {
                    Ok(c) => c,
                    Err(_e) => continue 'main_loop,
                };

                print!("Enter the new information => ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut information_to_update);

                match option_to_update_int - 1 {
                    0 => {
                        if information_to_update.len() <= 2 {
                            continue 'main_loop;
                        }
                        contact_to_update.first_name = String::from(information_to_update.trim());
                    }
                    1 => {
                        if information_to_update.len() <= 2 {
                            continue 'main_loop;
                        }
                        contact_to_update.last_name = String::from(information_to_update.trim());
                    }
                    2 => {
                        if information_to_update.len() <= 2 {
                            continue;
                        }
                        match information_to_update.trim().parse::<u64>() {
                            Ok(inf) => contact_to_update.phone_number = inf,
                            Err(_e) => {
                                print!("\x1B[2J\x1B[H");
                                eprintln!("Your number provided is invalid!");
                                continue 'main_loop;
                            }
                        }
                    }
                    _ => continue 'main_loop,
                }

                let is_updated = utils::json::update_contact(contact_to_update_is_find_index, contact_to_update);

                if is_updated == false {
                    println!();
                    thread::sleep(time::Duration::from_secs(10));
                    print!("\x1B[2J\x1B[H");
                    println!("There was an error in the update");
                    continue 'main_loop;
                }

                println!();
                thread::sleep(time::Duration::from_secs(10));
                print!("\x1B[2J\x1B[H");
                println!("Updating successfully")
            }
            4 => {
                // * Buffers for the use of inputs
                let mut first_name_to_delete: String = String::new();
                let mut contact_to_delete_is_find: bool = false;
                let mut contact_to_delete_is_find_index: usize = 0;
                let mut is_sure_str = String::new();

                print!("Contact by remove (provide first name) => ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut first_name_to_delete);

                for i in 0..contacts_length {
                    let contact = contacts.clone()[i].clone();
                    let first_name = utils::normalize_string(contact.first_name.as_str());
                    let include_with_first_name =
                        first_name.trim_end() == utils::normalize_string(first_name_to_delete.as_str()).trim_end();

                    if include_with_first_name == false {
                        continue;
                    } else if contact_to_delete_is_find == true && i == (contacts_length - 1) {
                        continue;
                    }

                    contact_to_delete_is_find_index = i;
                    contact_to_delete_is_find = true;
                }

                print!("Are you sure you want to continue? (y/n) ");
                stdout().flush().unwrap();
                let _ = stdin().read_line(&mut is_sure_str);

                match (is_sure_str.to_lowercase()).as_str().trim_end() {
                    "yes" | "y" => {
                        print!("\x1B[2J\x1B[H");
                        let is_deleted = utils::json::delete_contact(contact_to_delete_is_find_index);
                        if is_deleted == false {
                            println!();
                            thread::sleep(time::Duration::from_secs(10));
                            print!("\x1B[2J\x1B[H");
                            println!("There was an error in the update");
                            continue 'main_loop;
                        }
                    }
                    "not" | "n" | _ => {
                        print!("\x1B[2J\x1B[H");
                        eprintln!("Canceled operation!!!");
                        continue 'main_loop;
                    }
                }

                println!();
                thread::sleep(time::Duration::from_secs(1));
                print!("\x1B[2J\x1B[H");
                println!("Remove successfully!!!")
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
