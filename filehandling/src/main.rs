use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    let file_path = "rai.txt";

    // Write to a file
    let mut file = File::create(file_path)?;
    file.write_all(b"Hello, Rust file handling!\n")?;
    file.write_all(b"i love programming to make a relastic thing by usnig code.\n")?;
    println!("Data written to {}", file_path);

    // Read file 
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Content of {}:\n{}", file_path, contents);

    Ok(())
}
