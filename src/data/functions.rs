use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use crate::errors::Error;

pub fn get_file() -> Result<File, Error> {
    File::options()
        .read(true)
        .write(true)
        .append(true)
        .open("./wichtel.txt")
        .map_err(|_| Error::FileError)
}

pub fn contains_user(user_id: u64) -> Result<bool, Error> {
    let path = Path::new("./wichtel.txt");
    let contents = read_to_string(path).unwrap();

    Ok(contents.contains(&user_id.to_string()))
}

pub fn add_user(user_id: u64) -> bool {
    let file = get_file();
    let mut result = false;
    // TODO check if userid is already in file
    if !contains_user(user_id).unwrap() {
        result = match file {
            Ok(mut file) => writeln!(file, "{}", user_id).is_ok(),
            Err(_) => false,
        };
    };
    result
}

pub fn remove_user(user_id: u64) -> Result<(), Error> {

    let file = get_file();
    if !contains_user(user_id).unwrap() {
        // Here logic lol
    }

    Ok(())
}
