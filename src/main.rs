use std::fs;
use std::io;
use std::path::Path;

fn main() {
    loop {
        println!("File Manager CLI");
        println!("1. Create a file");
        println!("2. Rename a file");
        println!("3. Delete a file");
        println!("4. List files in directory");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => create_file(),
            2 => rename_file(),
            3 => delete_file(),
            4 => list_files(),
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please choose again."),
        }
    }
}

fn create_file() {
    println!("Enter the name of the file to create:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match fs::File::create(filename) {
        Ok(_) => println!("File '{}' created successfully.", filename),
        Err(e) => println!("Error creating file: {}", e),
    }
}

fn rename_file() {
    println!("Enter the name of the file to rename:");
    let mut old_name = String::new();
    io::stdin().read_line(&mut old_name).unwrap();
    let old_name = old_name.trim();

    println!("Enter the new name for the file:");
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).unwrap();
    let new_name = new_name.trim();

    match fs::rename(old_name, new_name) {
        Ok(_) => println!("File renamed from '{}' to '{}'.", old_name, new_name),
        Err(e) => println!("Error renaming file: {}", e),
    }
}

fn delete_file() {
    println!("Enter the name of the file to delete:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match fs::remove_file(filename) {
        Ok(_) => println!("File '{}' deleted successfully.", filename),
        Err(e) => println!("Error deleting file: {}", e),
    }
}

fn list_files() {
    let path = Path::new(".");
    match fs::read_dir(path) {
        Ok(entries) => {
            println!("Files in directory:");
            for entry in entries {
                match entry {
                    Ok(entry) => println!("{}", entry.file_name().to_string_lossy()),
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}
