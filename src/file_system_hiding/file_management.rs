#![allow(unused)]
use std::fs;
use serde_json::json;
use serde_json::{Value, Error};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, Write};

pub fn initialize_repository(repo_name: &str) -> std::io::Result<()> {
    fs::create_dir(repo_name)?;
    println!("Initialized empty repository at '{}'.", repo_name);
    Ok(())
}

pub fn create_hidden_dir(dir_name: &str) -> std::io::Result<()> {
    fs::create_dir(dir_name)?;
    println!("Created new folder for dbl internal info {}.", dir_name);
    Ok(())
}

pub fn write_to_file(file_name: &str, repo_name: &str, data: &str) -> std::io::Result<()>{
    let parsed: Value = serde_json::from_str(data)?;
    let mut file = fs::File::create(format!("{}/.dbl_info/{}", repo_name, file_name).as_str())?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &parsed)?;
    writer.flush()?;
    Ok(())
}
