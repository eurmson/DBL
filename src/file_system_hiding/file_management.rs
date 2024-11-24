

// pub fn initialize_repository(repo_name: &str) -> std::io::Result<()> {
//     fs::create_dir(repo_name)?;
//     println!("Initialized empty repository at '{}'.", repo_name);
//     Ok(())
// }


use directories::UserDirs;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::io::Result;

fn adjust_path(path_id: &Path, is_hidden: bool) -> PathBuf{
    match is_hidden {
        true => {
            let mut new_path = String::new();
            new_path.push_str(".");
            new_path.push(std::path::MAIN_SEPARATOR);
            new_path.push_str(".hiddenDBL");
            new_path.push(std::path::MAIN_SEPARATOR);
            new_path.push_str(path_id.to_str().unwrap());
            Path::new(&new_path).to_path_buf()
        }
        _ => path_id.to_path_buf(),
    }

}
pub struct Directory {
    directory: HashMap<String, bool>,
}
pub trait Files {
    // fn start_dir(path_id: &Path, is_hidden: bool) -> std::io::Result<()>;
    fn init(repo_name: &Path) -> Result<()>; //create hidden file 
    fn create_file(file_id: &Path, is_hidden: bool) -> Result<()>;
    fn list_files(file_id: &Path) -> Result<()>;
    fn write_to_file(file_id: &Path, addition: &str, is_hidden:bool) -> Result<()>;
    fn read_file(file_id: &Path, is_hidden:bool) -> Result<Vec<u8>>;
    fn remove_file(file_id: &Path, is_hidden:bool) -> Result<()>;
    fn copy_file(source: &Path, destination: &Path, is_hidden:bool) -> std::result::Result<(), &'static str>;

}

impl Files for Directory {

    fn init(repo_name: &Path) -> Result<()> { //hidden DBL should be inside repo_name
        match fs::create_dir(repo_name) {
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
            Ok(_) => {}
        }
        // let creating_dir = fs::create_dir(repo_name)?;
        let temp = format!("{}{}{}",repo_name.display(), MAIN_SEPARATOR,".hiddenDBL");
        let new_dir=  fs::create_dir(temp);
        Ok(())
    }
    fn list_files(file_id: &Path) -> Result<()> {
        Ok(())
    }
    fn write_to_file(file_id: &Path, addition: &str, is_hidden:bool) -> Result<()> {
        return fs::write(file_id, addition);
    }
    fn read_file(file_id: &Path, is_hidden:bool) -> Result<Vec<u8>> {
        return fs::read(file_id);
    }
    fn remove_file(file_id: &Path, is_hidden:bool) -> Result<()> {
        return fs::remove_file(file_id);
    }
    fn create_file(file_id: &Path, is_hidden: bool) -> Result<()> {
        let new_file_id = adjust_path(file_id, is_hidden);
        match new_file_id.parent() {
            Some(v) => {fs::create_dir_all(v);},
            None => {}

        }
        fs::create_dir_all(new_file_id.parent().unwrap());
        let mut file = fs::File::create(new_file_id)?;
        Ok(())
    }
   
    fn copy_file(source: &Path, destination: &Path, is_hidden:bool) -> std::result::Result<(), &'static str> {
        fs::File::create(destination);
        Ok(())
    }
}

// pub trait Repo {
    
// }

// pub struct Attributes {
//     repo_list: HashMap<i32, directories::UserDirs>,
// }

// impl Repo for Attributes {
    

// }


#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use std::path::{Path, PathBuf};
    use temp_dir::TempDir;
    struct TestFS{
        path: PathBuf,
    }
    impl TestFS {
        pub fn new() -> Option<Self> {
            let path = std::env::current_dir().ok()?;
            let tmpdir = TempDir::new().ok()?;

            std::env::set_current_dir(tmpdir.path());
            Some(TestFS{
                path
            })
        }
    }
    impl Drop for TestFS {
        fn drop(&mut self) {
            std::env::set_current_dir(self.path.as_path());
        }
    }

    #[test]
    fn test_create() {
        Directory::create_file(Path::new("Bye"), true).unwrap();
     }
     #[test]
     fn test_init() {
        Directory::init(Path::new(".")).unwrap();
     } 
     #[test]
     fn test_list() {
        println!("{:?}",Directory::list_files(Path::new("goodbye/.hiddenDBL")).unwrap());
     }   
     #[test] 
     fn test_write() {
        Directory::write_to_file(Path::new("goodbye/Goodbye.txt"), "Hello there", false).unwrap();
     }

}