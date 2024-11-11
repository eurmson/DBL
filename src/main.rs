#![allow(unused)]

mod repository_hiding;
mod file_system_hiding;
mod algorithm_hiding;

use std::env;
use std::process;
use crate::file_system_hiding::file_management;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 {
        eprintln!("Usage: {} <repository_name>", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    // Handle commands
    match command.as_str() {
        "init" => {
            if args.len() < 3{
                eprintln!("Usage: {} <repository_name>", args[0]);
                process::exit(1);
            }
            let repo_name = &args[2];
            // Initialize an empty repository
            if let Err(e) = file_management::initialize_repository(repo_name) {
                eprintln!("Failed to initialize repository: {}", e);
                process::exit(1);
            }
            if let Err(e) = file_management::create_hidden_dir(format!("{}/.dbl_info", repo_name).as_str()) {
                eprintln!("Failed to create hidden dir: {}", e);
                process::exit(1);
            }
            let repo_res = repository_hiding::action_handler(command, repo_name);
            println!("{}", repo_res.unwrap());
            output_success("Repository initialized successfully.");
        }
        _ => {
            eprintln!("Unknown command: '{}'", command);
            process::exit(1);
        }
    }
}

// Output handler
fn output_success(message: &str) {
    println!("{}", message);
}