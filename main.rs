mod file_management;

use std::env;
use std::process;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <repository_name>", args[0]);
        process::exit(1);
    }

    let command = &args[1];

    // Handle commands
    match command.as_str() {
        "init" => {
            // Initialize an empty repository
            if let Err(e) = file_management::initialize_repository("my_repo") {
                eprintln!("Failed to initialize repository: {}", e);
                process::exit(1);
            }
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