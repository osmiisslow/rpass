use crate::{
    append_to_path::append_to_path,
    rpass_dir::rpass_dir,
};

use std::{fs::File, io::Read};

pub fn get(application: String) -> String {
    let filename = &application as &str;
    let filepath = append_to_path(rpass_dir(), filename);
    let mut file = File::open(filepath)
        .expect("Password for desired application does not exist!");
    let mut password = String::new();
    file.read_to_string(&mut password).unwrap();
    password
}