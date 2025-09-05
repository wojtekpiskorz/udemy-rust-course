use std::{fs, io::Error};

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);
    //
    match divide(5.1, 3.0) {
        Ok(value) => {
            println!("{:#?}", value)
        }
        Err(error) => {
            println!("{:#?}", error)
        }
    }

    match validate_email(String::from("woji@.sd")) {
        Ok(..) => println!("All good baby"),
        Err(error) => println!("{:#}", error),
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Email must have an @"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Cant ddivide by 0"))
    } else {
        Ok(a / b)
    }
}
