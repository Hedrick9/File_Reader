use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() {
    println!("Enter the name of the file you would like to read from (including file extension, ex:.txt):");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read line");
    let file_name = file_name.trim().to_owned();
    // Determine File Extension
    let extension = file_extension(&file_name);
    match extension {
        ".txt" => println!("The file extension is .txt"),
        ".csv" => println!("The file extension is .csv"),
        ".xlsx" => println!("The file extension is .xlsx"),
        _ => println!("A valid file was not provided"),
    }
    // Read File Contents
    println!("File Name: {file_name}\n\n");
    read_file(file_name);

}

fn read_file(file_name: String) {

    let mut file = File::open(file_name).expect("Error when attempting to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error when attempting to read file.");

    println!("File Contents:\n {contents}");
}

fn file_extension (file_name: &String) -> &str {
    let bytes = file_name.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'.' {
            return &file_name[i..].trim();
        }
    }
    &file_name
}
