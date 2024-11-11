use std::path::Path;
use crate::algorithm_hiding::UniqueId;
trait FileLog {
    fn init() -> std::result::Result<(), String>;
    fn save_file(file_id: std::path::Path, file_contents: &str) -> UniqueId;
    fn retrieve_version(file_id: std::path::Path, id: UniqueId) -> Option<String>;
}


fn create_file_log() -> impl FileLog {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let file_log = create_file_log();
        assert!(file_log.init().is_ok());
        assert!(!file_log.init().is_ok());
    }
    #[test]
    fn test_init_already_initialized() {
        let file_log = create_file_log();
        assert!(!file_log.init().is_ok());
    }

    #[test]
    fn test_save_file() {
        let file_log = create_file_log();
        let buf = "TESTING CONTENT";
        let version = file_log.save_file(Path::new("Test_name"), buf);
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version);
        assert!(return_buf.is_some());
        assert!(buf.eq(return_buf.unwrap().as_ref()));
    }
    #[test]
    fn test_save_multiple_versions() {
        let file_log = create_file_log();
        let buf = "TESTING CONTENT";
        let buf2 = "TESTING CONTENT 2";
        let version = file_log.save_file(Path::new("Test_name"), buf);
        let version2 = file_log.save_file(Path::new("Test_name"), buf2);
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version);
        let return_buf2 = file_log.retrieve_version(Path::new("Test_name"), version2);
        assert!(return_buf.is_some());
        assert!(return_buf2.is_some());
        assert!(buf.eq(return_buf.unwrap().as_ref()));
        assert!(buf2.eq(return_buf2.unwrap().as_ref()));
    }
    #[test]
    fn test_save_multiple_files() {
        let file_log = create_file_log();
        let buf = "TESTING CONTENT";
        let buf2 = "TESTING CONTENT 2";
        let version = file_log.save_file(Path::new("Test_name"), buf);
        let version2 = file_log.save_file(Path::new("Test_name_2"), buf2);
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version);
        let return_buf2 = file_log.retrieve_version(Path::new("Test_name2"), version2);
        assert!(return_buf.is_some());
        assert!(return_buf2.is_some());
        assert!(buf.eq(return_buf.unwrap().as_ref()));
        assert!(buf2.eq(return_buf2.unwrap().as_ref()));
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

