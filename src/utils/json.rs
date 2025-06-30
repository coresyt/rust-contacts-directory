use serde;
use serde_json;
use std::fs;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug)]
pub struct IContact<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub phone_number: u64,
}

pub fn read_file() -> Result<(), Box<dyn std::error::Error>> {
    let contenido = match fs::read_to_string("personas.json") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error de I/O: {}", e);
            return Err(Box::new(e));
        }
    };

    match serde_json::from_str::<Vec<IContact>>(&contenido) {
        Ok(c) => {
            println!("{:?}", c);
        }
        Err(e) => {
            eprintln!("Error de I/O: {}", e);
            return Err(Box::new(e));
        }
    };
    Ok(())
}
