use crate::{
    append_to_path::append_to_path,
    randompass::random,
    rpass_dir::rpass_dir,
};
use std::{fs::File, io::Write};

pub fn new(application: String) {
    let filename = &application as &str;
    let filepath = append_to_path(rpass_dir(), filename);
    let password = random();
    let mut file = File::create(filepath).unwrap();
    file.write_all(password.as_bytes()).unwrap();
}