mod RepositoryHiding{
    use std::collections::HashMap;
    use std::path::Path;
    // use serde_json::json;
    // use serde_json::{Value, Error};

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

    impl DummyFileManager{
        fn new(data: String) -> Self{
            DummyFileManager{
                some_data: data,
            }
        }
        fn file_exists(file_name: &str) -> bool{
            true
        }
        fn write_to_file(file_name: &str, data: &str) -> bool{
            true
        }
    }

    trait Serializable{
        fn serialize(&self) -> std::result::Result<(), Error>;
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
        fn serialize(&self) -> std::result::Result<(), Error>{
            // let rg_str = "{}";
            if self.rg.is_empty(){
                // let parsed: Value = serde_json::from_str(rg_str)?;    
                // let expected = json!({});
                // assert_eq!(parsed, expected);    
                // DummyFileManager::write_file(parsed);
                DummyFileManager::write_to_file("./rg_info.json", "{}");
            }
            Ok(())
        }
    }

    fn action_handler(command: &str) -> std::result::Result<String, String>{
        let file_name = "rg_info.json";
        let mut init_res = String::from("initialized");
        let mut rg: RevGraph;
        let rg_pth = Path::new("./rg_info.json");
        let file_manager = DummyFileManager::new(String::from("data"));
        if !DummyFileManager::file_exists(&file_name){
            rg = RevGraph::init();
        }
        else{
            rg = RevGraph::reinit("data");
            // rg = RevGraph::reinit(file_manager.read_file(rg_pth, true));
            init_res = String::from("reinitialized");
        }
        if command == "init"{
            // Serialize is called before exiting after completing command
            RevGraph::serialize(&rg);
            return Ok(init_res)
        }
        Err(String::from("failed"))
    }

    #[cfg(test)]
    mod tests{
        use super::*;
        #[test]
        fn init_test(){
            let mut rg = RevGraph::init();
            assert!(rg.rg.is_empty());
            // assert!(rg.init().is_ok()); 
            assert!(action_handler("init").is_ok());
            assert_eq!(action_handler("init").unwrap(), String::from("reinitialized"));
        }
    }
}