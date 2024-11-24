use std::io::{Lines, Read};
use std::str::from_utf8;
use serde_json;
use serde::{Deserialize, Serialize};
use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct UniqueId {
    bytes: [u8; 16],
}

impl UniqueId {
    pub fn into_string(self) -> String {
        self.bytes.iter().map(|&b| format!("{:x}", b)).collect::<String>()
    }
}

pub fn create_unique_id() -> UniqueId {
    let time = SystemTime::now();
    let a = time.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    UniqueId {
        bytes: a.to_be_bytes(),
    }

}
pub fn diff_file_versions(file_contents1: &str, file_contents2: &str) -> String {
    for line in file_contents1.lines() {}
    fn lcs(x: &[String], y: &[String]) -> Vec<String> {
        match (x.len(), y.len(), x.last(), y.last()) {
            (0, _, _, _)|(_, 0, _, _) => {
                Vec::new()
            }
            (_, _, Some(a), Some(b)) if (a == b) => {
                let mut tmp = lcs(&x[0..x.len()-1], &y[0..y.len()-1]);
                tmp.append(&mut vec![a.clone()]);
                tmp
            }
            (_, _, Some(_), Some(_)) => {
                let a = lcs(&x[0..x.len()-1], y);
                let b = lcs(x, &y[0..y.len()-1]);
                if a.len() > b.len() {
                    a
                } else {
                    b
                }
            }
            (..) => unreachable!()
        }
    }


    let file1_lines = file_contents1.lines().map(|x|x.into()).collect::<Vec<String>>();
    let file2_lines = file_contents2.lines().map(|x|x.into()).collect::<Vec<String>>();
    let mut diff = String::with_capacity(file_contents1.len() + file_contents2.len());
    let mut iter1 = file1_lines.iter().peekable();
    let mut iter2 = file2_lines.iter().peekable();
    let subset = lcs(&file1_lines, &file2_lines);
    for line in subset{
        while(iter1.peek().is_some() && line.cmp(*iter1.peek().unwrap()).is_ne()) {
            diff.push_str("--");
            diff.push_str(iter1.next().unwrap());
            diff.push_str("\n");
        }
        while(iter2.peek().is_some() && line.cmp(*iter2.peek().unwrap()).is_ne()) {
            diff.push_str("++");
            diff.push_str(iter2.next().unwrap());
            diff.push_str("\n");
        }
        iter1.next();
        iter2.next();
        diff.push_str(&line);
        diff.push_str("\n");
    }
    for line in iter1 {
        diff.push_str("--");
        diff.push_str(line);
        diff.push_str("\n");
    }
    for line in iter2 {
        diff.push_str("++");
        diff.push_str(line);
        diff.push_str("\n");
    }
    diff
}
pub fn merge_file_versions(file_contents1: &str, file_contents2: &str) -> Result<String, String> {
    let diffed = diff_file_versions(file_contents1, file_contents2);
    let mut merged = String::with_capacity(diffed.len());
    let mut add_buff: Vec<String> = Vec::new();
    for line in diffed.lines() {
        match line.chars().take(2).collect::<String>().as_str() {
            "--" => {
                add_buff.push(line[2..].into());
            }
            "++" => {
                if add_buff.len() > 0 {
                    return Err(diffed)
                }
                merged.push_str(&line[2..]);
                merged.push_str("\n");

            }
            _ => {
                if add_buff.len() > 0 {
                    for line in add_buff.iter() {
                        merged.push_str(&line);
                        merged.push_str("\n");
                    }
                    add_buff.clear()
                }
                merged.push_str(line.into());
                merged.push_str("\n");
            }
        }
    }
    for line in add_buff.iter() {
        merged.push_str(line);
        merged.push_str("\n");
    }
    Ok(merged)
}
#[cfg(test)]
mod tests {

    use crate::algorithm_hiding::*;
    #[test]
    fn test_create_unique_id() {
        let uid1 = create_unique_id();
        let uid2 = create_unique_id();
        assert_ne!(uid1, uid2);
    }

    #[test]
    fn test_cmp_unique_id() {
        let uid = create_unique_id();
        let copy_uid = uid;
        assert_eq!(copy_uid, uid);

        let a = serde_json::to_string(&uid);
        let uid1 = serde_json::from_str::<UniqueId>("{\"bytes\":[0,0,0,0,0,0,0,0,24,6,213,111,37,193,134,212]}").unwrap();
        let uid2 = serde_json::from_str("{\"bytes\":[0,0,0,0,0,0,0,0,24,6,213,111,37,193,134,212]}").unwrap();

        assert_eq!(uid1, uid2);
    }

    #[test]
    fn test_diff_file_versions() {
        let file_contents1 = "TEST CONTENT\nTEST CONTENT";
        let file_contents2 = "TEST CONTENT2\nTEST CONTENT";
        let diff = diff_file_versions(file_contents1, file_contents2);
        assert_eq!(diff, "++TEST CONTENT2\nTEST CONTENT\n--TEST CONTENT\n");
    }

    #[test]
    fn test_diff_same_file_versions() {
        let file_contents1 = "TEST CONTENT";
        let file_contents2 = "TEST CONTENT";
        let diff = diff_file_versions(file_contents1, file_contents2);
        assert_eq!(&diff, "TEST CONTENT\n");
    }

    #[test]
    fn test_merge_file_versions() {
        let file_contents1 = "TEST CONTENT\nTEST CONTENT\nLine in 1\nTEST CONTENT";
        let file_contents2 = "TEST CONTENT\nLine in 2\nTEST CONTENT\n";
        let merged = merge_file_versions(file_contents1, file_contents2);
        assert!(merged.is_ok());
        assert_eq!(merged.unwrap(), "TEST CONTENT\nLine in 2\nTEST CONTENT\nLine in 1\nTEST CONTENT\n");
    }

    #[test]
    fn test_merge_differing_file_versions() {
        let file_contents1 = "TEST 1 CONTENT";
        let file_contents2 = "TEST 2 CONTENT";
        let merged = merge_file_versions(file_contents1, file_contents2);
        assert!(merged.is_err());
    }

}
