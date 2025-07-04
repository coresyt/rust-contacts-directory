pub mod json;
pub mod welcome;

use crate::utils::json::IContact;
use unicode_normalization::UnicodeNormalization;
use unicode_general_category::{get_general_category, GeneralCategory};

pub fn show_contacts(contacts_length: usize, contacts: Vec<IContact>) {
    for i in 0..contacts_length {
        let contact: &IContact = &contacts[i];
        let first_name: &str = contact.first_name.as_str();
        let last_name: &str = contact.last_name.as_str();
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
}

pub fn normalize_string(s: &str) -> String {
    s.nfd()
        .filter(|c| get_general_category(*c) != GeneralCategory::NonspacingMark)
        .collect::<String>().to_lowercase()
}
