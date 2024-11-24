use std::path::Path;
use std::str::from_utf8;
use serde::__private::de::TagOrContentField::Content;
use crate::algorithm_hiding::{create_unique_id, UniqueId};
pub trait FileLog {
    fn init(&mut self) -> Result<(), String>;
    fn save_file(&self, file_id: &Path) -> std::io::Result<UniqueId>;
    fn retrieve_version(&self, file_id: &Path, id: UniqueId) -> Option<String>;
}

pub struct DBLFileLog<T> where T: Files {
    initialized:bool,
    _t: Option<T>,
}
use crate::file_system_hiding::file_management::Files;

impl<T> FileLog for DBLFileLog<T>  where T: Files{
    fn init(&mut self) -> Result<(), String> {
        let res = T::write_to_file(Path::new("file_log_info"), "Initialized", true);
        match res{
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
    fn save_file(&self, file_id: &Path) -> std::io::Result<UniqueId> {
        let id = create_unique_id();
        let mut path = file_id.to_path_buf();
        path.push(id.into_string());
        let c = match (String::from_utf8(T::read_file(file_id, false)?)) {
            Ok(content) => content,
            Err(e) => {return Err(std::io::Error::new(std::io::ErrorKind::Other, e));}
        };
        T::create_file(path.as_path(),true);
        T::write_to_file(path.as_path(), &c, true);
        Ok(id)
    }
    fn retrieve_version(&self, file_id: &Path, id: UniqueId) -> Option<String> {
        let mut path = file_id.to_path_buf();
        path.push(id.into_string());

        match T::read_file(&path, true) {
            Ok(vec) => {
                match String::from_utf8(vec) {
                    Ok(contents) => Some(contents),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}

pub fn create_file_log<T>() -> impl FileLog where T: Files {
    DBLFileLog::<T>{ initialized: false, _t:None}
}



#[cfg(test)]
mod tests {
    use crate::file_system_hiding::file_log::*;
    use crate::file_system_hiding::file_management::Files;
    use crate::file_system_hiding::file_log::create_file_log;
    use std::io::{ErrorKind, Result};
    use std::path::Path;
    use mockall::{automock, mock};
    use crate::file_system_hiding::file_management;

    struct Directory{}
    #[automock]
    impl Files for Directory{
        fn init(repo_name: &Path) -> Result<()> {unimplemented!()}
        fn create_file(file_id: &Path, is_hidden: bool) -> Result<()> {unimplemented!()}
        fn list_files(file_id: &Path) -> Result<()> { unimplemented!() }
        fn write_to_file(file_id: &Path, addition: &str, hidden: bool) -> Result<()> {unimplemented!()}
        fn read_file(file_id: &Path, hidden: bool) -> Result<Vec<u8>> {unimplemented!()}
        fn remove_file(file_id: &Path, hidden: bool) -> Result<()> {unimplemented!()}
        fn copy_file(source: &Path, destination: &Path, hidden: bool) -> std::result::Result<(), &'static str> { unimplemented!() }
    }


    #[test]
    fn test_init() {
        create_file_log::<file_management::Directory>();
        let wtf_ctx = MockDirectory::write_to_file_context();
        wtf_ctx.expect().once()
            .return_once(|x, x1| {
                println!("{:?}, {:?}", x, x1);
                Ok(())
            });
        wtf_ctx.expect().once()
            .return_once(|x, x1| {
                println!("{:?}, {:?}", x, x1);
                Err(std::io::Error::new(ErrorKind::Other, "test"))
            });

        let mut file_log = create_file_log::<MockDirectory>();

        assert!(file_log.init().is_ok());
        assert!(!file_log.init().is_ok());
    }
    #[test]
    fn test_init_already_initialized() {
        let wtf_ctx = MockDirectory::write_to_file_context();
        wtf_ctx.expect().once()
            .return_once(|x, x1| {
                println!("{:?}, {:?}", x, x1);
                Err(std::io::Error::new(ErrorKind::Other, "test"))
            });
        let mut file_log = create_file_log::<MockDirectory>();
        assert!(!file_log.init().is_ok());
    }

    #[test]
    fn test_save_file() {
        let test_file_name: &Path = Path::new("Test_name");
        let test_file_content: &str = "TESTING CONTENT";

        let wtf_ctx = MockDirectory::write_to_file_context();
        let rf_ctx = MockDirectory::read_file_context();
        let cf_ctx = MockDirectory::create_file_context();

        rf_ctx.expect().once().return_once(move |x| {
            assert_eq!(x, test_file_name);
            Ok(test_file_content.bytes().collect())
        });
        cf_ctx.expect().once().return_once(move |x, x1| {
            assert_eq!(x.parent().unwrap(), test_file_name);
            Ok(())
        });
        wtf_ctx.expect().once().returning(move |x, x1| {
            assert_eq!(x.parent().unwrap(), test_file_name);
            assert_eq!(x1, test_file_content);
            Ok(())
        });
        rf_ctx.expect().once().return_once(move |x| {
            assert_eq!(x.parent().unwrap(), test_file_name);
            Ok(test_file_content.bytes().collect())
        });

        let mut file_log = create_file_log::<MockDirectory>();

        let buf = "TESTING CONTENT";

        let version = file_log.save_file(Path::new("Test_name"));
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version.expect("REASON"));
        assert!(return_buf.is_some());
        assert!(buf.eq(return_buf.unwrap().as_str()));
    }
    #[test]
    fn test_save_multiple_versions() {
        let mut file_log = create_file_log::<MockDirectory>();
        let buf = "TESTING CONTENT";
        let buf2 = "TESTING CONTENT 2";
        let version = file_log.save_file(Path::new("Test_name"));
        let version2 = file_log.save_file(Path::new("Test_name"));
        let return_buf = file_log.retrieve_version(Path::new("Test_name"), version.unwrap());
        let return_buf2 = file_log.retrieve_version(Path::new("Test_name"), version2.unwrap());
        assert!(return_buf.is_some());
        assert!(return_buf2.is_some());
        assert!(buf.eq(return_buf.unwrap().as_str()));
        assert!(buf2.eq(return_buf2.unwrap().as_str()));
    }
    #[test]
    fn test_save_multiple_files() {
        let mut file_log = create_file_log::<MockDirectory>();
        let buf = "TESTING CONTENT";
        let buf2 = "TESTING CONTENT 2";
        let version = file_log.save_file(Path::new("Test_name"));
        let version2 = file_log.save_file(Path::new("Test_name_2"));
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

