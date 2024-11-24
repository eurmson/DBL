use std::fs;

pub fn initialize_repository(repo_name: &str) -> std::io::Result<()> {
    fs::create_dir(repo_name)?;
    println!("Initialized empty repository at '{}'.", repo_name);
    Ok(())
}

mod file_management {
    use directories::UserDirs;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;
    use std::io::Result;
    struct Directory {
        directory: HashMap<String, bool>,
    }
    trait Files {
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

    trait Repo {
        fn init(repo_name: &str) -> Result<()>;
    }

    struct Attributes {
        repo_list: HashMap<i32, directories::UserDirs>,
    }

    impl Repo for Attributes {
        fn init(repo_name: &str) -> Result<()> {
            let repo_name = fs::create_dir(repo_name)?;
            Ok(())
        }

    }
}