use std::hash::Hash;
use std::io::Read;
use serde_json;
use serde::{Deserialize, Serialize};
use std::time::{UNIX_EPOCH, SystemTime};
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct UniqueId {
    bytes: [u8; 16],
}

pub fn create_unique_id() -> UniqueId {
    let time = SystemTime::now();
    let a = time.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    UniqueId {
        bytes: a.to_be_bytes(),
    }

}
pub fn diff_file_versions(file_contents1: &str, file_contents2: &str) -> Box<String> {
    todo!()
}
pub fn merge_file_versions(file_contents1: &str, file_contents2: &str) -> Result<Box<String>, String> {
    todo!()
}
#[cfg(test)]
mod tests {

    use crate::algorithm_hiding::*;
    #[test]
    fn test_create_unique_id() {
        let UID1= create_unique_id();
        print!("{UID1:?}");
        let UID2= create_unique_id();
        assert_ne!(UID1, UID2);
    }

    #[test]
    fn test_cmp_unique_id() {
        let UID = create_unique_id();
        let copy_UID = UID;
        assert_eq!(copy_UID, UID);

        let a = serde_json::to_string(&UID);
        print!("{a:?}");
        let UID1 = serde_json::from_str::<UniqueId>("{\"bytes\":[0,0,0,0,0,0,0,0,24,6,213,111,37,193,134,212]}").unwrap();
        let UID2 = serde_json::from_str("{\"bytes\":[0,0,0,0,0,0,0,0,24,6,213,111,37,193,134,212]}").unwrap();

        assert_eq!(UID1, UID2);
    }

    #[test]
    fn test_diff_file_versions() {
        let file_contents1 = "TEST CONTENT";
        let file_contents2 = "TEST CONTENT2";
        let diff = diff_file_versions(file_contents1, file_contents2);
        assert_eq!(diff.as_ref(), "2");
    }

    #[test]
    fn test_diff_same_file_versions() {
        let file_contents1 = "TEST CONTENT";
        let file_contents2 = "TEST CONTENT";
        let diff = diff_file_versions(file_contents1, file_contents2);
        assert_eq!(diff.as_ref(), "");
    }

    #[test]
    fn test_merge_file_versions() {
        let file_contents1 = "TEST CONTENT";
        let file_contents2 = "TEST 2 CONTENT";
        let merged = merge_file_versions(file_contents1, file_contents2);
        assert!(merged.is_ok());
        assert_eq!(merged.unwrap().as_ref(), "TEST 2 CONTENT");
    }

    #[test]
    fn test_merge_differing_file_versions() {
        let file_contents1 = "TEST 1 CONTENT";
        let file_contents2 = "TEST 2 CONTENT";
        let merged = merge_file_versions(file_contents1, file_contents2);
        assert!(merged.is_err());
    }

}
