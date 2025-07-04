use crate::utils;
use std::io::stdin;
use std::io::{Write, stdout};
use std::{thread, time};

/// This function asks for 2 arguments returning a Null value.
///
/// Print **contacts** on the terminal based on a query
///
/// `contacts` Its type is **<`Vec<IContact>`>**
///
/// `length` Its type is **<`usize`>**
pub fn function(contacts: Vec<utils::json::IContact>, length: usize) {
    let mut search: Vec<utils::json::IContact> = Vec::new();
    let mut key_to_search = String::new();
    print!("Search => ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut key_to_search);

    for i in 0..length {
        let contact = &contacts[i];
        let first_name = utils::normalize_string(contact.first_name.as_str());
        let last_name = utils::normalize_string(contact.last_name.as_str());

        let include_with_first_name =
            first_name.contains(&utils::normalize_string(key_to_search.trim_end()));
        let include_last_name =
            last_name.contains(&utils::normalize_string(key_to_search.trim_end()));
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
