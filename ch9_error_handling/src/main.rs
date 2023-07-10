use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
    error::Error,
};

#[allow(unused)]
fn error_handling_with_match() {
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    let _greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

#[allow(unused)]
fn error_handling_with_closures() {
    let _greeting_file: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _read_username_from_file_verbose() -> Result<String, io::Error> {
    let username_file_result: Result<File, io::Error> = File::open("hello.txt");

    let mut username_file: File = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),
    }
}

fn _read_username_from_file_concise() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    return Ok(username);
}

fn _read_username_from_file_conciser() -> Result<String, io::Error> {
    return fs::read_to_string("hello.txt");
}

fn main() -> Result<(), Box<dyn Error>>{
    // error_handling_with_match();
    // error_handling_with_closures();

    // let _greeting_file: File = File::open("hello.txt").unwrap();
    // let _greeting_file: File =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    let _greeting_file = File::open("hello.txt")?;

    return Ok(());
}
