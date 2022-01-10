// Rust program to remove � character from file

use std::fs::File;
use std::io::{self, Read, Write};

use std::env;

fn main() {
    // Handle errors
    run().unwrap();
}

fn run() -> Result<(), io::Error> {
    let word_from = "�";
    let word_to = "";

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut source = File::open(file_path)?;
    let mut data1 = String::new();

    source.read_to_string(&mut data1)?;

    drop(source);

    let data2 = data1.replace(&*word_from, &*word_to);

    let mut dest = File::create(file_path)?;
    dest.write(data2.as_bytes())?;
    Ok(())
}
