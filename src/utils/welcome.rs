use crate::utils::json::IContact;

pub fn welcome(
    contacts_length: usize,
    contacts: Vec<IContact>,
    options: [&'static str; 5],
    options_length: usize,
) {
    if contacts_length > 0 {
        for i in 0..contacts_length {
            let contact: &IContact<'_> = &contacts[i];
            let first_name: &str = contact.first_name;
            let last_name: &str = contact.last_name;
            let phone_number: String = contact.phone_number.to_string();
            let phone_number_len: usize = phone_number.len();

            println!(
                "{}. {} {}: +...{}",
                i + 1,
                last_name,
                first_name,
                phone_number.split_at(phone_number_len / 2).1
            );
        }
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
