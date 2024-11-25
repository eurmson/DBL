use directories::UserDirs;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

fn adjust_path(path_id: &Path, is_hidden: bool) -> PathBuf {
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
    fn init(repo_name: &Path) -> Result<()>;
    fn create_file(file_id: &Path, is_hidden: bool) -> Result<()>;
    fn list_files(file_id: &Path, is_hidden: bool, file_out: &mut Vec<String>) -> Result<()>;
    fn write_to_file(file_id: &Path, addition: &str, is_hidden: bool) -> Result<()>;
    fn read_file(file_id: &Path, is_hidden: bool) -> Result<Vec<u8>>;
    fn remove_file(file_id: &Path, is_hidden: bool) -> Result<()>;
    fn copy_file(source: &Path, destination: &Path, is_hidden: bool) -> Result<u64>;
}

impl Files for Directory {
    fn init(repo_name: &Path) -> Result<()> {
        //hidden DBL should be inside repo_name
        match fs::create_dir(repo_name) {
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
            Ok(_) => {}
        }
        // let creating_dir = fs::create_dir(repo_name)?;
        let temp = format!("{}{}{}", repo_name.display(), MAIN_SEPARATOR, ".hiddenDBL");
        let new_dir = fs::create_dir(temp);
        Ok(())
    }
    fn list_files(file_id: &Path, is_hidden: bool, file_out: &mut Vec<String>) -> Result<()> {
        Directory::create_file(file_id, is_hidden);
        let new_id = &adjust_path(file_id, is_hidden);
        fs::read_dir(new_id);
        if new_id.is_dir() {
            for entry in fs::read_dir(new_id)? {
                let entry = entry?;
                let mut path = entry.path();
                if path.is_dir() {
                    Directory::list_files(&path, false, file_out);
                } else {
                    file_out.push(new_id.display().to_string());
                    file_out.push(entry.file_name().into_string().unwrap());
                    
                }
            }
        };
        Ok(())
    }
    fn write_to_file(file_id: &Path, addition: &str, is_hidden: bool) -> Result<()> {
        Directory::create_file(file_id, is_hidden);
        let new_id = adjust_path(file_id, is_hidden);
        return fs::write(new_id, addition);
    }
    fn read_file(file_id: &Path, is_hidden: bool) -> Result<Vec<u8>> {
        let new_id = adjust_path(file_id, is_hidden);
        return fs::read(new_id);
    }
    fn remove_file(file_id: &Path, is_hidden: bool) -> Result<()> {
        let new_id = adjust_path(file_id, is_hidden);
        return fs::remove_file(new_id);
    }
    fn create_file(file_id: &Path, is_hidden: bool) -> Result<()> {
        let new_file_id = adjust_path(file_id, is_hidden);
        match new_file_id.parent() {
            Some(v) => {
                fs::create_dir_all(v);
            }
            None => {}
        }
        fs::create_dir_all(new_file_id.parent().unwrap());
        let mut file = fs::File::create(new_file_id)?;
        Ok(())
    }

    fn copy_file(source: &Path, destination: &Path, is_hidden: bool) -> Result<u64> {
        let new_source = adjust_path(source, is_hidden);
        let new_destiantion = adjust_path(destination, is_hidden);
        Directory::create_file(destination, is_hidden);
        return fs::copy(source, destination);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env;
//     use std::path::{Path, PathBuf};
//     use temp_dir::TempDir;
//     struct TestFS {
//         path: PathBuf,
//     }
//     impl TestFS {
//         pub fn new() -> Option<Self> {
//             let path = std::env::current_dir().ok()?;
//             let tmpdir = TempDir::new().ok()?;

//             std::env::set_current_dir(tmpdir.path());
//             Some(TestFS { path })
//         }
//     }
//     impl Drop for TestFS {
//         fn drop(&mut self) {
//             std::env::set_current_dir(self.path.as_path());
//         }
//     }

//     #[test]
//     fn test_create() {
//         Directory::create_file(Path::new("Bye"), true).unwrap();
//     }
//     #[test]
//     fn test_init() {
//         Directory::init(Path::new(".")).unwrap();
//     }
//     #[test]
//     fn test_list() {
//         let mut vec = vec![];
//         println!("Printing...");
//         Directory::list_files(Path::new("test"), true, &mut vec);
//         println!("{:?}", vec.clone());
//     }
//     #[test]
//     fn test_write() {
//         Directory::write_to_file(Path::new("hello/Goodbye.txt"), "Hello there", true).unwrap();
//     }
//     #[test]
//     fn test_read() {
//         println!(
//             "{:?}",
//             String::from_utf8(
//                 Directory::read_file(Path::new("goodbye/Goodbye.txt"), true).unwrap()
//             )
//             .unwrap()
//         );
//     }
//     #[test]
//     fn test_remove() {
//         Directory::remove_file(Path::new("hello"), true).unwrap();
//     }
//     #[test]
//     fn test_copy() {
//         Directory::copy_file(
//             Path::new("hello/Goodbye.txt"),
//             Path::new("Folder2/Goodbye.txt"),
//             true,
//         );
//     }
// }
