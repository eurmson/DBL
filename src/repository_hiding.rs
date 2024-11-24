#![allow(unused)]
use super::file_management;
use std::collections::HashMap;
use std::path::Path;
use serde_json::json;
use serde_json::{Value, Error};
use std::rc::Rc;
use std::cell::{Ref, RefCell};

type NodePointer = Option<Rc<RefCell<RGNode>>>;

trait Serializable{
    fn serialize(&self, file_name: String, repo_name: String) -> std::result::Result<(), Error>;
}
#[derive(Debug)]
struct RGData{
    // Min requirements for the RG Data struct:
    // 1. A list of file names tracked for given revision. Addition/removal of files from tracking list affects this
    track_list: Vec<String>,
}

impl RGData{
    fn new(file_names: Vec<String>) -> Self{
        RGData{
            track_list: file_names,
        }
    }
    fn add_files(&mut self, file_names: Vec<String>){
        file_names.iter().for_each(|fname| self.track_list.push(fname.clone()));
    }
    fn remove_files(& mut self, file_names: Vec<String>){
        let temp_list: Vec<&String> = self.track_list.iter().filter(|&fname_1| !file_names.contains(fname_1)).collect();
        self.track_list = temp_list.iter().map(|&fname| fname.clone()).collect();
    }
}

// TODO: Think about whether data should be an Option or when data needs to be empty you should just have an empty vector?
#[derive(Debug)]
struct RGNode{
    // data: Option<RGData>,
    data: RGData,
    // I want the references of the node stored in the rev graph to be refcell
    // but if I change the references to the parents to be refcell then there is an infinite recursion problem
    // Rust won't let me use refcell for the references below
    // So, I'm gonna leave them as Box for now and we'll see how we handle the problems that come because of it
    // p1: Option<Box<RGNode>>,
    // Okay, so I've updated to using Rc<RefCell<RGNode>> everywhere cuz that fixes the infinite recursion problem
    // Plus, it helps maintain consistency
    p1: NodePointer,
    // p2: Option<Box<RGNode>>,
    p2: NodePointer,
}

struct RevGraph{
    // The graph has a hashmap that maps each revision (RGNode) to a unique hash ID (sha hash implemented by Ethan)
    // The graph also has a list of pointers to RGNodes which are the heads of the graph
    // graph: HashMap<DummyUniqueID, Option<RefCell<RGNode>>>,
    graph: HashMap<u32, NodePointer>,
    // Also made heads a hashmap to easily keep track of pointers to heads of different branches
    // Any benefit to using &str over String in this case? other than the added headache of lifetimes?
    // Box because its supposed to point to an RGNode whose size cannot be determined during compile time
    // Option because when the Repo is just initialized the main head will be None
    // heads: HashMap<String, Option<RefCell<RGNode>>>,
    heads: HashMap<String, NodePointer>,
    // Pointer to the current revision node
    // Points to the same node as the one that the pointer for "main" in the heads hashmap points to
    // curr_rev: Option<RefCell<RGNode>>,
    curr_rev: NodePointer,
}

// struct Heads(HashMap<String, Option<Box<RGNode>>>);
// impl Heads{
//     fn add_head(& mut self, branch_name: String, node_pointer: Option<Box<RGNode>>) {
//         self.0.insert(branch_name, node_pointer);
//     }
// }

impl RevGraph{
    fn init() -> Self{
        let rev = RevGraph{
            graph: HashMap::new(),
            heads: HashMap::new(),
            curr_rev: None,
        };
        rev
    }
    // TODO: update to reinitialize rev graph from info stored in hidden dir
    fn reinit(data: &str) -> Self{
        RevGraph{
            graph: HashMap::new(),
            heads: HashMap::new(),
            curr_rev: None
        }
    }

    // TODO: This function should return result
    fn add_files(& mut self, file_names: Vec<String>, branch_name: String){
        match &self.curr_rev{
            Some(rg_node) => {
                // TODO: Handle the case of trying to add files that are already being tracked (error or just a message?)
                // (*rg_node).borrow_mut().data.as_mut().unwrap().add_files(file_names);
                (*rg_node).borrow_mut().data.add_files(file_names);
            }
            _ => {
                let mut rg_node = Rc::new(RefCell::new(RGNode{
                    // data: Some(RGData::new(file_names)),
                    data: RGData::new(file_names),
                    p1: None,
                    p2: None,
                }));
                self.curr_rev = Some(rg_node);
                self.point_head(branch_name);
            }
        }
    }

    // TODO: This function should return result
    fn remove_files(& mut self, file_names: Vec<String>){
        match &self.curr_rev{
            Some(rg_node) =>{
                // TODO: Add the error case of when RGNode is initialized but no files are being tracked
                // (*rg_node).borrow_mut().data.as_mut().unwrap().remove_files(file_names);
                (*rg_node).borrow_mut().data.remove_files(file_names);
            }
            // TODO: Handle the Error case of trying to remove files when RGNode is either uninitialized
            _ => {

            }
        }
    }
    fn add_graph_node(& mut self, branch_name: String){
        match &self.curr_rev{
            // TODO: Handle the error case of committing when no files are being tracked
            Some(rg_node) =>{
                let new_node = Rc::new(RefCell::new(RGNode{
                    data: RGData::new(Vec::new()),
                    p1: Some(Rc::clone(rg_node)),
                    p2: None,
                }));
                let id = generate_unique_id(self.graph.clone());
                self.graph.insert(id, Some(Rc::clone(rg_node)));
                self.curr_rev = Some(new_node);
                self.point_head(branch_name);
            }
            _ => {
                // TODO: Handle the error case of committing before the node even exists
            }
        }
    }
    // adds a new head to the heads hashmap in rev_graph
    fn point_head(& mut self, branch_name: String) {
        match &self.curr_rev{
            Some(rg_node) =>{
                // TODO: Add the error case of when RGNode is initialized but no files are being tracked
                // (*rg_node).borrow_mut().data.as_mut().unwrap().remove_files(file_names);
                self.heads.insert(branch_name, Some(Rc::clone(rg_node)));
            }
            // TODO: Handle the Error case of trying to point head to curr_rev when curr_rev is uninitialized
            // Realistically, this case should not even arise
            _ => {

            }
        }
        // self.heads.insert(branch_name, self.curr_rev.clone());
    } 
}

impl Serializable for RevGraph{
    fn serialize(&self, file_name: String, repo_name: String) -> std::result::Result<(), Error>{
        let rg_str = "{}".to_string();
        if self.graph.is_empty(){
            if let Err(e) = file_management::write_to_file(file_name, repo_name, rg_str){
                eprintln!("Failed to create/write to file: {}", e);
            }
        }
        Ok(())
    }
}

pub fn action_handler(command: String, repo_name: String, file_names: Vec<String>, branch_name: String) -> std::result::Result<String, String>{
    let file_name = "rg_info.json".to_string();
    let mut init_res = String::from("initialized");
    let mut rg: RevGraph;

    // Handle the case where the repo has already been initialized

    rg = RevGraph::init();
    if command == "init"{
        // Serialize is called before exiting after completing command
        RevGraph::serialize(&rg, file_name, repo_name);
        return Ok(init_res)
    }
    // for the add command need the names of the files that are being added to the tracking list for the given revision
    // for now I'm assuming that the file list is sent to the action handler as an additional parameter
    // check if there exists a curr_rev, if not then make a new RGNode and assign it to curr_rev 
    // If making a new curr_rev then add it to the heads hashmap main as well
    // call add_files on RGData in curr_rev and send it the list of file_names 
    if command == "add"{
        // TODO: DECIDE IF THE CURR-REV SHOULD BE INITIALIZED WHEN THE GRAPH IS INITIALIZED OR WHEN ADD IS CALLED FOR THE FIRST TIME
        // if curr_rev exists then add files to its tracking list (in RGData)
        // if let Some(mut curr_rev) = rg.curr_rev {
        //     curr_rev.add_files(file_names);
        // }
        // else{
        //     // If curr_rev doesn't exist then create a new RGNode with the files added to the tracking list
        //     // let mut curr_rev = Some(Rc::new(RGNode { data: RGData::new(file_names), p1: None, p2: None }));
        //     // rg.curr_rev = curr_rev;
        //     // and then add a pointer to it to the heads hashmap for the "main" branch
        //     // rg.add_head("main", RefCell...);
        //     // let mut h = Heads(HashMap::new());
        //     // h.add_head("main", &rg.curr_rev);
        // }
        rg.add_files(file_names.clone(), branch_name.clone());
        println!("{:?}", rg.curr_rev);
        println!("{:?}", rg.heads.get("main"));

        rg.remove_files(vec!["file_2.txt".to_string()]);
        println!("{:?}", rg.curr_rev);
        println!("{:?}", rg.heads.get("main"));

        rg.add_graph_node(branch_name.clone());
        println!("{:?}", rg.curr_rev);
        println!("{:?}", rg.heads.get("main"));

        // RefCell does not implement the clone trait, Rc does
        // So either you make a new refcell that is an exact copy of the rgnode to store in the heads map
        // or you change the definition of curr_rev to be Option<Rc<RefCell<RGNode>>>
        // and change all the other places that need to use a reference to the RGNode as well
        
        // Since the add_head call is right after the add_files call, you can assume that curr_rev 
        // is a pointer to the current rg_node in whichever branch the user is in
        // This branch_name info should not be given to repo hiding from the behvior hiding
        // I should be able to figure out which branch we're currently in based just on the data stored in the serialized rg file 
        // rg.add_head(branch_name.clone());
    }
    if command == "remove"{
        rg.remove_files(file_names.clone());
        // println!("{:?}", rg.curr_rev);
        // println!("{:?}", rg.heads.get("main"));
    }
    if command == "commit"{
        rg.add_graph_node(branch_name.clone());
        // let new_node = Rc::new(RefCell::new(RGNode{
        //     data: RGData::new(Vec::new()),
        //     p1: rg.curr_rev,
        //     p2: None,
        // }));
        // let id = generate_unique_id(rg.graph);
        // let rev = rg.curr_rev.unwrap();
        // rg.graph.insert(id, Some(Rc::clone(&rev)));
        // // TODO: Add current revision node to the graph hashmap with unique ID
        // rg.curr_rev = Some(new_node);
    }

    Err(String::from("failed"))
}

pub fn tests () {
    // Write tests for add command here
    let file_names = vec!["file_1.txt".to_string(), "file_2.txt".into(), "file_3.txt".into()]; 
    action_handler("add".to_string(), "repo".to_string(), file_names, "main".to_string());
    action_handler("remove".to_string(), "repo".to_string(), vec!["file_2.txt".to_string()], "main".to_string());

}

fn generate_unique_id(hm:HashMap<u32, NodePointer>) -> u32{
    let mut i: u32 = 1;
    while hm.contains_key(&i){
        i = i + 1;
    }
    i
}