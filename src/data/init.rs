/*
This file should include the generation of the wichtel file
*/

use std::fs::File;

pub fn init() -> bool {
    let file = File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open("./wichtel.txt");

    file.is_ok()
}
