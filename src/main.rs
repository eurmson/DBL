#![allow(unused)]

mod repository_hiding;
mod file_system_hiding;
mod algorithm_hiding;

use std::env;
use std::process;
use std::path::{Path, PathBuf};
use std::vec::Drain;
use crate::algorithm_hiding::UniqueId;
use crate::file_system_hiding::file_management;
use crate::file_system_hiding::file_management::{Directory, Files};

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [<file_names>...] [rev_id]", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let repo_name = "&args[2]";

    // Handle different commands
    match command.as_str() {
        "init" => handle_init_command(&args),
        "status" => handle_status_command(&args),
        "cat" => handle_cat_command(&args),
        "diff" => handle_diff_command(&args),
        "merge" => handle_merge_command(&args),
        "log" => handle_log_command(&args, repo_name),
        "checkout" => handle_checkout_command(&args, repo_name),
        "add" | "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: {} {} <file_names>", args[0], command);
                process::exit(1);
            }
            println!("{:?}", &args[3..args.len()]);
            let file_names: Vec<PathBuf> = args[2..args.len()].iter().map(|a|PathBuf::from(a)).collect();
            match repository_hiding::action_handler::<Directory>(command.to_string(), Some(file_names), None, vec![]) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("Error in action handler: {}", e);
                    process::exit(1);
                }
            }
        },
        "commit"|"heads" => {
            match repository_hiding::action_handler::<Directory>(command.to_string(), None, None, vec![]) {
                Ok(result) => println!("{}", result),
                Err(e) => {
                    eprintln!("Error in action handler: {}", e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: '{}'", command);
            process::exit(1);
        }
    }
}

// Handle 'init' command
fn handle_init_command(args: &[String]) {
    let repo_name = match args.len() {
        0..3 => {
            &".".to_string()
        }
        _ => {
            &args[2]
        }
    };

    // Initialize an empty repository
    if let Err(e) = file_management::Directory::init(repo_name.as_ref()) {
        eprintln!("Failed to initialize repository: {}", e);
        process::exit(1);
    }

    env::set_current_dir(repo_name).unwrap();
    // Handle repository action
    match repository_hiding::action_handler::<Directory>("init".to_string(), None, None, vec![]) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }

    output_success("Repository initialized successfully.");
}

// Handle 'status' command
fn handle_status_command(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: {} status", args[0]);
        process::exit(1);
    }
    match repository_hiding::action_handler::<Directory>("status".to_string(), None, None, vec![]) {
        Ok(result) => {
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Handle 'cat' command
fn handle_cat_command(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: {} cat <file_name> <rev_id>", args[0]);
        process::exit(1);
    }

    let file_names: Vec<PathBuf> = args[2..args.len() - 1].iter().map(|a|PathBuf::from(a)).collect();
    let rev_id = if args.len() > 3 { Some(UniqueId::from_string(&args[3])).flatten() } else { None };

    match repository_hiding::action_handler::<Directory>("cat".to_string(), Some(file_names), None, vec![rev_id]) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Handle 'diff' command
fn handle_diff_command(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: {} diff <rev_id1> <rev_id2>", args[0]);
        process::exit(1);
    }

    let rev_id1 = UniqueId::from_string(&args[2]);
    let rev_id2 = UniqueId::from_string(&args[3]);

    match repository_hiding::action_handler::<Directory>("diff".to_string(), None, None, vec![rev_id1.clone(), rev_id2.clone()]) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

// Handle 'merge' command
fn handle_merge_command(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: {} merge <file_name1> <rev_id1> <rev_id2>", args[0]);
        process::exit(1);
    }

    let file_name = PathBuf::from(&args[2]);
    let rev_id1 = UniqueId::from_string(&args[3]);
    let rev_id2 = UniqueId::from_string(&args[4]);

    match repository_hiding::action_handler::<Directory>("merge".to_string(), Some(vec![file_name.clone()]), None, vec![rev_id1.clone(), rev_id2.clone()]) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    }
}

fn handle_log_command(args: &[String], repo_name: &str){
    if args.len() < 3 {
        eprintln!("Usage: {} log <rev_id1>", args[0]);
        process::exit(1);
    }

    let rev_id1 = UniqueId::from_string(&args[2]);
    match repository_hiding::action_handler::<Directory>("log".to_string(), None, None, vec![rev_id1.clone()]) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error in action handler: {}", e);
            process::exit(1);
        }
    } 
}

fn handle_checkout_command(args: &[String], repo_name: &str){
    if args.len() < 3 {
        eprintln!("Usage: {} checkout <rev_id1>", args[0]);
        process::exit(1);
    }
    let rev_id1 = UniqueId::from_string(&args[2]);
    match repository_hiding::action_handler::<Directory>("checkout".to_string(), None, None, vec![rev_id1.clone()]) {
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
