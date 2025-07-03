use serde;
use serde_json;
use std::fs;

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
