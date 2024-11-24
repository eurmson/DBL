#![allow(unused)]

mod repository_hiding;
mod file_system_hiding;
mod algorithm_hiding;

use std::env;
use std::process;
use std::path::PathBuf;
use crate::file_system_hiding::file_management;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> <repository_name> [<file_names>...] [branch_name] [rev_id]", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let repo_name = &args[2];

    // Handle different commands
    match command.as_str() {
        "init" => handle_init_command(&args, repo_name),
        "status" => handle_status_command(&args, repo_name),
        "cat" => handle_cat_command(&args, repo_name),
        "diff" => handle_diff_command(&args, repo_name),
        "merge" => handle_merge_command(&args, repo_name),
        "add" | "remove" | "commit" => {
            if args.len() < 4 {
                eprintln!("Usage: {} {} <repository_name> <file_names> [branch_name]", args[0], command);
                process::exit(1);
            }
            let file_names: Vec<String> = args[3..args.len() - 1].to_vec();
            let branch_name = args[args.len() - 1].clone();
            match repository_hiding::action_handler(command.to_string(), repo_name.to_string(), file_names, branch_name, String::new()) {
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
    match repository_hiding::action_handler("init".to_string(), repo_name.to_string(), vec![], "".to_string(), String::new()) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }

    output_success("Repository initialized successfully.");
}

// Handle 'status' command
fn handle_status_command(args: &[String], repo_name: &str) {
    if args.len() < 3 {
        eprintln!("Usage: {} status <repository_name> <branch_name> [rev_id]", args[0]);
        process::exit(1);
    }

    let branch_name = args[3].clone();
    let rev_id = if args.len() > 4 { args[4].clone() } else { String::new() };

    match repository_hiding::action_handler("status".to_string(), repo_name.to_string(), vec![], branch_name, rev_id) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Handle 'cat' command
fn handle_cat_command(args: &[String], repo_name: &str) {
    if args.len() < 4 {
        eprintln!("Usage: {} cat <repository_name> <file_names...> [rev_id]", args[0]);
        process::exit(1);
    }

    let file_names: Vec<String> = args[3..args.len() - 1].to_vec();
    let rev_id = if args.len() > 4 { args[args.len() - 1].clone() } else { String::new() };

    match repository_hiding::action_handler("cat".to_string(), repo_name.to_string(), file_names, "".to_string(), rev_id) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Handle 'diff' command
fn handle_diff_command(args: &[String], repo_name: &str) {
    if args.len() < 5 {
        eprintln!("Usage: {} diff <repository_name> <file_name1> <rev_id1> <rev_id2>", args[0]);
        process::exit(1);
    }

    let file_name = &args[3];
    let rev_id1 = &args[4];
    let rev_id2 = &args[5];

    match repository_hiding::action_handler("diff".to_string(), repo_name.to_string(), vec![file_name.clone()], rev_id1.clone(), rev_id2.clone()) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Handle 'merge' command
fn handle_merge_command(args: &[String], repo_name: &str) {
    if args.len() < 5 {
        eprintln!("Usage: {} merge <repository_name> <file_name1> <rev_id1> <rev_id2>", args[0]);
        process::exit(1);
    }

    let file_name = &args[3];
    let rev_id1 = &args[4];
    let rev_id2 = &args[5];

    match repository_hiding::action_handler("merge".to_string(), repo_name.to_string(), vec![file_name.clone()], rev_id1.clone(), rev_id2.clone()) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Output handler
fn output_success(message: &str) {
    println!("{}", message);
}
