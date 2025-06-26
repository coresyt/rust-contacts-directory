use std::io;

struct Contact<'a> {
    first_name: &'a str,
    last_name: &'a str,
    phone_number: u64
}

fn main() {
    let mut contacts: Vec<Contact> = vec![Contact { first_name: "Pedro", last_name: "Diaz", phone_number: 523327853326 }];

    println!("Hello! This is the contact list.");

    for i in 0..50 {
        print!("-");
        if i == 50 {
            println!()
        }
    }

    // let mut buffer: String = String::new();
    // io::stdin().read_line(&mut buffer)?;
    // Ok(())
}
