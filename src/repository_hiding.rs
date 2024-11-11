#![allow(unused)]
use crate::file_system_hiding::file_management;
use std::collections::HashMap;
use std::path::Path;
use serde_json::json;
use serde_json::{Value, Error};

struct RGData{}

struct RGNode{
    data: RGData,
    p1: Option<Box<RGNode>>,
    p2: Option<Box<RGNode>>,
}

struct DummyUniqueID{}
struct DummyFileManager{
    some_data: String,
}

trait Serializable{
    fn serialize(&self, file_name: &str, repo_name: &str) -> std::result::Result<(), Error>;
}

struct RevGraph{
    rg: HashMap<DummyUniqueID, RGNode>,
}

impl RevGraph{
    fn init() -> Self{
        RevGraph{
            rg: HashMap::new(),
        }
    }
    fn reinit(data: &str) -> Self{
        RevGraph{
            rg: HashMap::new(),
        }
        // Fill in values from stored data when not reinitializing empty rg
    }
}

impl Serializable for RevGraph{
    fn serialize(&self, file_name: &str, repo_name: &str) -> std::result::Result<(), Error>{
        let rg_str = "{}";
        if self.rg.is_empty(){
            if let Err(e) = file_management::write_to_file(file_name, repo_name, rg_str){
                eprintln!("Failed to create/write to file: {}", e);
            }
        }
        Ok(())
    }
}

pub fn action_handler(command: &str, repo_name: &str) -> std::result::Result<String, String>{
    let file_name = "rg_info.json";
    let mut init_res = String::from("initialized");
    let mut rg: RevGraph;

    // Handle the case where the repo has already been initialized

    rg = RevGraph::init();
    if command == "init"{
        // Serialize is called before exiting after completing command
        RevGraph::serialize(&rg, file_name, repo_name);
        return Ok(init_res)
    }
    Err(String::from("failed"))
}