

// pub fn initialize_repository(repo_name: &str) -> std::io::Result<()> {
//     fs::create_dir(repo_name)?;
//     println!("Initialized empty repository at '{}'.", repo_name);
//     Ok(())
// }


use directories::UserDirs;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::Result;
pub struct Directory {
    directory: HashMap<String, bool>,
}
pub trait Files {
    fn start_dir(path_id: &Path) -> std::io::Result<()>;
    fn write_to_file(file_id: &Path, addition: &str) -> Result<()>;
    fn read_file(file_id: &Path) -> Result<Vec<u8>>;
    fn remove_file(file_id: &Path) -> Result<()>;
    fn create_file(file_id: &Path, is_hidden: bool) -> Result<()>;
    fn copy_file(source: &Path, destination: &Path) -> std::result::Result<(), &'static str>;
}

impl Files for Directory {
    fn start_dir(path_id: &Path) -> Result<()> {
        return fs::create_dir(path_id);
    }
    fn write_to_file(file_id: &Path, addition: &str) -> std::io::Result<()> {
        return fs::write(file_id, addition);
    }
    fn read_file(file_id: &Path) -> Result<Vec<u8>> {
        return fs::read(file_id);
    }
    fn remove_file(file_id: &Path) -> std::io::Result<()> {
        return fs::remove_file(file_id);
    }
    fn create_file(file_id: &Path, is_hidden: bool) -> Result<()> {
        let mut file = fs::File::create(file_id)?;
        Ok(())
    }
    fn copy_file(source: &Path, destination: &Path) -> std::result::Result<(), &'static str> {
        fs::File::create(destination);
        Ok(())
    }
}

pub trait Repo {
    fn init(repo_name: &str) -> Result<()>;
}

pub struct Attributes {
    repo_list: HashMap<i32, directories::UserDirs>,
}

impl Repo for Attributes {
    fn init(repo_name: &str) -> Result<()> {
        let repo_name = fs::create_dir(repo_name)?;
        Ok(())
    }

    }


// #[cfg(test)]
// mod tests {
//     use std::env;
//     use super::*;
//     use std::path::{Path, PathBuf};
//     use temp_dir::TempDir;
//     struct TestFS{
//         path: PathBuf,
//     }
//     impl TestFS {
//         pub fn new() -> Option<Self> {
//             let path = std::env::current_dir().ok()?;
//             let tmpdir = TempDir::new().ok()?;

//             std::env::set_current_dir(tmpdir.path());
//             Some(TestFS{
//                 path
//             })
//         }
//     }
//     impl Drop for TestFS {
//         fn drop(&mut self) {
//             std::env::set_current_dir(self.path.as_path());
//         }
//     }

//     #[test]
//     fn test_init() {
//      }
     

// }