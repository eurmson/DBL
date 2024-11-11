use std::path::Path;
use crate::algorithm_hiding::UniqueId;
trait FileLog {
    fn init(&mut self) -> std::result::Result<(), String>;
    fn save_file(&mut self, file_id: &Path, file_contents: &str) -> std::io::Result<UniqueId>;
    fn retrieve_version(&mut self, file_id: &Path, id: UniqueId) -> Option<String>;
}

struct DBLFileLog {
}
use crate::file_system_hiding::file_management;
use crate::file_system_hiding::file_management::write_to_file;

impl FileLog for DBLFileLog {

    fn init(&mut self) -> Result<(), String> {
        write_to_file("file_log_info", "test", "data");

        Ok(())
    }
    fn save_file(&mut self, file_id: &Path, file_contents: &str) -> std::io::Result<UniqueId> {
        todo!()
    }
    fn retrieve_version(&mut self, file_id: &Path, id: UniqueId) -> Option<String> {
        todo!()
    }
}

fn create_file_log() -> impl FileLog {
    todo!();
    DBLFileLog{}
}


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
    fn test_init() {
        let tmpfs = TestFS::new().expect("Failed to create test environment");

        let mut file_log = create_file_log();
        assert!(file_log.init().is_ok());
        assert!(!file_log.init().is_ok());

        drop(tmpfs);
    }
    #[test]
    fn test_init_already_initialized() {
        let mut file_log = create_file_log();
        assert!(!file_log.init().is_ok());
    }

    #[test]
    fn test_save_file() {
        let mut file_log = create_file_log();
        let buf = "TESTING CONTENT";
        let version = file_log.save_file(Path::new("Test_name"), buf);
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version.expect("REASON"));
        assert!(return_buf.is_some());
        assert!(buf.eq(return_buf.unwrap().as_str()));
    }
    #[test]
    fn test_save_multiple_versions() {
        let mut file_log = create_file_log();
        let buf = "TESTING CONTENT";
        let buf2 = "TESTING CONTENT 2";
        let version = file_log.save_file(Path::new("Test_name"), buf);
        let version2 = file_log.save_file(Path::new("Test_name"), buf2);
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version.unwrap());
        let return_buf2 = file_log.retrieve_version(Path::new("Test_name"), version2.unwrap());
        assert!(return_buf.is_some());
        assert!(return_buf2.is_some());
        assert!(buf.eq(return_buf.unwrap().as_str()));
        assert!(buf2.eq(return_buf2.unwrap().as_str()));
    }
    #[test]
    fn test_save_multiple_files() {
        let mut file_log = create_file_log();
        let buf = "TESTING CONTENT";
        let buf2 = "TESTING CONTENT 2";
        let version = file_log.save_file(Path::new("Test_name"), buf);
        let version2 = file_log.save_file(Path::new("Test_name_2"), buf2);
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version.unwrap());
        let return_buf2 = file_log.retrieve_version(Path::new("Test_name2"), version2.unwrap());
        assert!(return_buf.is_some());
        assert!(return_buf2.is_some());
        assert!(buf.eq(return_buf.unwrap().as_str()));
        assert!(buf2.eq(return_buf2.unwrap().as_str()));
    }
    #[test]
    fn test_stored_files() {
        todo!()
        // set_up_mock_fs();
        // let file_log = create_file_log();
        // let version = get_known_version();
        // let buf = known_test_buf();
        // let return_buf = file_log.retrieve_version(Path::new("Test_name"), version);
        // assert!(return_buf.is_some());
        // assert!(buf.eq(return_buf.unwrap().as_ref()));
    }
}

