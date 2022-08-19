use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let mut users_file = get_users_file().expect("Failed to open users file");

    println!("Please enter username");
    let mut username = String::new();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let username_exists = check_username_exists(username.trim()).expect("OH SHIT OH SHIT");

    // Handle Invalid Username
    if username_exists == false {
        println!("Username not found, would you like to create a new user? (y/n)");
        let mut user_input = String::new();
        while user_input.trim() != "y" && user_input.trim() != "n" {
            user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            if user_input.trim() == "y" {
                prompt_create_new_user(&mut users_file);
            } else if user_input.trim() == "n" {
                println!("Exiting program");
                return;
            } else {
                println!("Invalid input, please enter y or n");
            }
        }
    }

    // Handle Password Input
    println!("Please enter password");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    // PASSWORD CHECK
}

fn prompt_create_new_user(users_file: &mut File) {
    println!("Enter a new username");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    while check_username_exists(username.trim()).expect("Couldn't check username for validity") {
        println!("Username already exists, please enter a unique username");
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");
    }
    let mut confirm_username = String::new();
    println!("Confirm username");
    io::stdin()
        .read_line(&mut confirm_username)
        .expect("Failed to read line");
    while username.trim() != confirm_username.trim() {
        confirm_username = String::new();
        println!("Usernames do not match, please enter a matching username");
        io::stdin()
            .read_line(&mut confirm_username)
            .expect("Failed to read line");
    }

    // let users_file = get_users_file().expect("Couldn't get users file");
    // fs::write("users.txt", confirm_username.push_str("\n")).expect("Couldn't write to users file");
    // SANITIZE FILE

    // WRITE TO FILE
}

fn check_username_exists(username: &str) -> Result<bool, io::Error> {
    let users_file = get_users_file().expect("Couldn't get users file");
    let reader = BufReader::new(users_file);
    let mut username_found = false;
    for line in reader.lines() {
        if let Ok(line) = line {
            let user_info = line.split(',').collect::<Vec<&str>>();
            if user_info[0] == username {
                username_found = true;
            }
        }
    }
    Ok(username_found)
}

fn get_users_file() -> Result<File, io::Error> {
    const USERS_FILE_NAME: &str = "users.txt";
    let users_file = File::open(USERS_FILE_NAME);
    let users_file = match users_file {
        Ok(file) => file,
        Err(_error) => create_users_file(USERS_FILE_NAME)?, // Might be conditions where the error doesn't stem from the file not existing
    };
    Ok(users_file)
}

fn create_users_file(file_name: &str) -> std::io::Result<File> {
    return File::create(file_name);
}
