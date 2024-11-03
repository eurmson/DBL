use std::fs;

pub fn initialize_repository(repo_name: &str) -> std::io::Result<()> {
    fs::create_dir(repo_name)?;
    println!("Initialized empty repository at '{}'.", repo_name);
    Ok(())
}