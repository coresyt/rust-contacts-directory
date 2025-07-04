use crate::utils;
use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

pub fn function(
    contacts: Vec<utils::json::IContact>,
    length: usize,
    options: [&str; 3],
) -> Result<(), ()> {
    // * Buffers for the use of inputs
    let mut first_name_to_update: String = String::new();
    let mut option_to_update: String = String::new();
    let mut information_to_update: String = String::new();
    // * Temporary variables to perform the update
    let mut contact_to_update: utils::json::IContact = utils::json::IContact {
        first_name: String::new(),
        last_name: String::new(),
        phone_number: 0,
    };
    let mut contact_to_update_is_find: bool = false;
    let mut contact_to_update_is_find_index: usize = 0;

    print!("Contact by update (provide first name) => ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut first_name_to_update);

    for i in 0..length {
        let contact = contacts.clone()[i].clone();
        let first_name = utils::normalize_string(contact.first_name.as_str());
        let include_with_first_name = first_name.trim_end()
            == utils::normalize_string(first_name_to_update.as_str()).trim_end();

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

    for i in 0..(options.len()) {
        let opt = options[i];
        println!("{}. {}", i + 1, opt);
    }

    print!("What do you want to modify? ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut option_to_update);
    let option_to_update_int = match option_to_update.trim().parse::<u32>() {
        Ok(c) => c,
        Err(_e) => return Err(()),
    };

    print!("Enter the new information => ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut information_to_update);

    match option_to_update_int - 1 {
        0 => {
            if information_to_update.len() <= 2 {
                return Err(());
            }
            contact_to_update.first_name = String::from(information_to_update.trim());
        }
        1 => {
            if information_to_update.len() <= 2 {
                return Err(());
            }
            contact_to_update.last_name = String::from(information_to_update.trim());
        }
        2 => {
            if information_to_update.len() <= 2 {
                return Err(());
            }
            match information_to_update.trim().parse::<u64>() {
                Ok(inf) => contact_to_update.phone_number = inf,
                Err(_e) => {
                    print!("\x1B[2J\x1B[H");
                    eprintln!("Your number provided is invalid!");
                    return Err(());
                }
            }
        }
        _ => return Err(()),
    }

    let is_updated =
        utils::json::update_contact(contact_to_update_is_find_index, contact_to_update);

    if is_updated == false {
        println!();
        thread::sleep(time::Duration::from_secs(3));
        print!("\x1B[2J\x1B[H");
        println!("There was an error in the update");
        return Err(());
    }

    println!();
    thread::sleep(time::Duration::from_secs(3));
    print!("\x1B[2J\x1B[H");
    println!("Updating successfully");
    Ok(())
}