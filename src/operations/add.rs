use crate::utils;
use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

/// This function returning a Ok or Err.
///
/// Create the file with your inputs
pub fn function() -> Result<(), ()> {
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

    let new_phone_number_int = match new_phone_number_str.trim().parse() {
        Ok(c) => c,
        Err(_e) => {
            eprintln!("Your phone number provided is a invalid number.\nRetry!!!");
            return Err(());
        }
    };

    let new_contact = utils::json::IContact {
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

    match is_sure_str.to_lowercase().trim_end() {
        "yes" | "y" => {
            let is_created = utils::json::add_contact(new_contact);
            if is_created == false {
                println!();
                thread::sleep(time::Duration::from_secs(10));
                print!("\x1B[2J\x1B[H");
                println!("There was an error in the create");
                return Err(());
            }

            print!("\x1B[2J\x1B[H");
            return Ok(());
        }
        "not" | "n" | _ => {
            print!("\x1B[2J\x1B[H");

            eprintln!("Canceled operation!!!");
            return Err(());
        }
    }
}
