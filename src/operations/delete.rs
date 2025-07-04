use crate::utils;
use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

pub fn function(contacts: Vec<utils::json::IContact>, length: usize) -> Result<(), ()> {
    // * Buffers for the use of inputs
    let mut first_name_to_delete: String = String::new();
    let mut contact_to_delete_is_find: bool = false;
    let mut contact_to_delete_is_find_index: usize = 0;
    let mut is_sure_str = String::new();

    print!("Contact by remove (provide first name) => ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut first_name_to_delete);

    for i in 0..length {
        let contact = contacts.clone()[i].clone();
        let first_name = utils::normalize_string(contact.first_name.as_str());
        let include_with_first_name = first_name.trim_end()
            == utils::normalize_string(first_name_to_delete.as_str()).trim_end();

        if include_with_first_name == false {
            continue;
        } else if contact_to_delete_is_find == true && i == (length - 1) {
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
                return Err(());
            }
        }
        "not" | "n" | _ => {
            print!("\x1B[2J\x1B[H");
            eprintln!("Canceled operation!!!");
            return Err(());
        }
    }

    println!();
    thread::sleep(time::Duration::from_secs(1));
    print!("\x1B[2J\x1B[H");
    println!("Remove successfully!!!");
    Ok(())
}
