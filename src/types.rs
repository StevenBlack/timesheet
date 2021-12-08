// types.rs
use std::{fmt};
use regex::Regex;
use crate::Semver;

#[derive(Default, Clone, Debug)]
pub struct Commit {
    pub date:String,
    pub msg:String,
}

// pub trait Semver {
//     fn issemvertag(&self) -> bool;
// }

impl Semver for Commit {
    // Matches digits only
    // 1 - Major
    // 2 - Minor
    // 3 - Patch
    fn issemvertag(&self) -> bool {
        let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
        re.is_match(&self.msg)
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.msg)
    }
}

pub type Commits = Vec<Commit>;

#[test]
fn check_semvertag() {
    let c = Commit {
        date: "2021-10-15".to_string(),
        msg: "0.11.2".to_string()
    };
    assert_eq!(true, c.issemvertag());
}
