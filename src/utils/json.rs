use serde;
use serde_json;
use std::{fs, io::Write};

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct IContact {
    pub first_name: String,
    pub last_name: String,
    pub phone_number: u64,
}

pub fn get_contacts() -> Vec<IContact> {
    let content = match fs::read_to_string("personas.json") {
        Ok(c) => c,
        Err(_e) => String::new()
    };

    let contacts : Vec<IContact> = match serde_json::from_str::<Vec<IContact>>(content.as_str()) {
        Ok(c) => c,
        Err(_e) => vec![]
    };

    return contacts;
}

pub fn add_contact(new_contact: IContact) -> bool {
    let mut contacts = get_contacts().clone();
    let new_contact_formatted = IContact {
        first_name: String::from(new_contact.first_name.trim_end()),
        last_name: String::from(new_contact.last_name.trim_end()),
        phone_number: new_contact.phone_number,
    };
    let first_name_small: bool = new_contact_formatted.first_name.len() >= 4;
    let last_name_small: bool = new_contact_formatted.last_name.len() >= 4;
    let mut file_json = match fs::File::create("personas.json") {
        Ok(f) => f,
        Err(_) => match fs::File::open("personas.json") {
            Ok(f) => f,
            Err(_) => {
                return false;
            }
        },
    };

    if first_name_small == false || last_name_small == false {
        return false;
    }

    for i in 0..contacts.len() {
        let _contact = contacts[i].clone();
        let first_name = String::from(_contact.first_name.trim_end());

        if new_contact_formatted.first_name != first_name {
            continue;
        }

        return false;
    }

    contacts.push(new_contact);

    let json_in_string = match serde_json::to_string_pretty(&contacts) {
        Ok(text) => text,
        Err(_) => {
            return false;
        }
    };

    match file_json.write_all(json_in_string.as_bytes()) {
        Ok(_) => return true,
        Err(_) => {
            return false;
        }
    };
}

pub fn update_contact(index_of_contact: usize, new_info_contact: IContact) -> bool {
    let mut contacts = get_contacts().clone();
    let new_info_contact_formatted = IContact {
        first_name: String::from(new_info_contact.first_name.trim_end()),
        last_name: String::from(new_info_contact.last_name.trim_end()),
        phone_number: new_info_contact.phone_number,
    };
    let first_name_small: bool = new_info_contact_formatted.first_name.len() >= 4;
    let last_name_small: bool = new_info_contact_formatted.last_name.len() >= 4;
    let mut file_json = match fs::File::create("personas.json") {
        Ok(f) => f,
        Err(_) => match fs::File::open("personas.json") {
            Ok(f) => f,
            Err(_) => {
                println!("1");
                return false;
            }
        },
    };

    if first_name_small == false || last_name_small == false {
        println!("2");
        return false;
    }

    contacts[index_of_contact] = new_info_contact_formatted;

    let json_in_string = match serde_json::to_string_pretty(&contacts) {
        Ok(text) => text,
        Err(_e) => {
            println!("3");
            return false;
        }
    };

    match file_json.write_all(json_in_string.as_bytes()) {
        Ok(_) => return true,
        Err(_) => {
            println!("4");
            return false;
        }
    };
}

pub fn delete_contact(index_of_contact: usize) -> bool {
    let mut contacts = get_contacts().clone();
    let mut file_json = match fs::File::create("personas.json") {
        Ok(f) => f,
        Err(_) => match fs::File::open("personas.json") {
            Ok(f) => f,
            Err(_) => {
                println!("1");
                return false;
            }
        },
    };

    if (contacts.len() - 1) < index_of_contact || index_of_contact < 0 { return false; }

    contacts.remove(index_of_contact);

    let json_in_string = match serde_json::to_string_pretty(&contacts) {
        Ok(text) => text,
        Err(_e) => {
            println!("3");
            return false;
        }
    };

    match file_json.write_all(json_in_string.as_bytes()) {
        Ok(_) => return true,
        Err(_) => {
            println!("4");
            return false;
        }
    };
}
