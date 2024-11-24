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

    if args.len() < 2 {
        eprintln!("Usage: {} <command> <repository_name> [<file_names>...]", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let repo_name = &args[2];

    // Handle different commands
    match command.as_str() {
        "init" => handle_init_command(&args, repo_name),
        "add" | "remove" | "commit" => {
            if args.len() < 4 {
                eprintln!("Usage: {} {} <repository_name> <file_names> [branch_name]", args[0], command);
                process::exit(1);
            }
            let file_names: Vec<String> = args[3..args.len() - 1].to_vec();
            let branch_name = args[args.len() - 1].clone();
            match repository_hiding::action_handler(command.to_string(), repo_name.to_string(), file_names, branch_name) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("Error in action handler: {}", e);
                    process::exit(1);
                }
            }
        },
        _ => {
            eprintln!("Unknown command: '{}'", command);
            process::exit(1);
        }
    }
}

// Handle 'init' command
fn handle_init_command(args: &[String], repo_name: &str) {
    if args.len() < 3 {
        eprintln!("Usage: {} init <repository_name>", args[0]);
        process::exit(1);
    }

    // Initialize an empty repository
    if let Err(e) = file_management::initialize_repository(repo_name) {
        eprintln!("Failed to initialize repository: {}", e);
        process::exit(1);
    }

    // Handle repository action
    match repository_hiding::action_handler("init".to_string(), repo_name.to_string(), vec![], "".to_string()) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }

    output_success("Repository initialized successfully.");
}

// Output handler
fn output_success(message: &str) {
    println!("{}", message);
}
