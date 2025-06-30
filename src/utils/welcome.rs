use crate::utils::{show_contacts, json::IContact};

pub fn welcome(
    contacts_length: usize,
    contacts: Vec<IContact>,
    options: [&'static str; 5],
    options_length: usize,
) {
    if contacts_length > 0 {
        show_contacts(contacts_length, contacts);
    } else {
        println!("Your contact directory is empty!!")
    }

    for _i in 0..50 {
        print!("-");
    }
    println!("\n");

    for i in 0..options_length {
        let opt: &'static str = options[i];
        println!("{}. {}", i + 1, opt);
        {}
    }
}
