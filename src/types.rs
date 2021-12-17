// types.rs
use std::{fmt};
use regex::Regex;
use lazy_static::lazy_static;
#[derive(Default, Clone, Debug)]
pub struct Commit {
    pub date:String,
    pub msg:String,
}

lazy_static! {
    static ref RE_SEMVER: Regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    static ref RE_ISSUE: Regex = Regex::new(r"^(?i)issue.*#.*:").unwrap();
    static ref RE_VERSION_SEMVER: Regex = Regex::new(r"(?i)version\s\d+\.\d+\.\d+$").unwrap();
}

pub trait Commitinfo {
    fn msg_words(&self) -> usize;
}

impl Commitinfo for Commit {
    fn msg_words(&self) -> usize {
        let wordvec: Vec<&str> = self.msg.split(" ").collect::<Vec<&str>>();
        wordvec.len()
    }
}

#[test]
fn check_msg_words() {
    let c = Commit {
        date: "2021-10-15".to_string(),
        msg: "One two three.".to_string()
    };
    assert_eq!(3, c.msg_words());
}

pub trait Semver {
    fn issemvertag(&self) -> bool;
    fn isversionsemvertag(&self) -> bool;
}

impl Semver for Commit {
    // Matches digits only, the only thing on the line
    // 1 - Major
    // 2 - Minor
    // 3 - Patch
    fn issemvertag(&self) -> bool {
        RE_SEMVER.is_match(&self.msg)
    }

    // Matches the work "version" followed by a semver tag at end of line.
    fn isversionsemvertag(&self) -> bool {
        RE_VERSION_SEMVER.is_match(&self.msg)
    }
}

pub trait Issue {
    fn isissue(&self) -> bool;
}

impl Issue for Commit {
    // "Issue #315: fix — replace the Input control..."
    // "Issue #315: refactor — move fetchdata() to ...."
    // "Issue #495: cosmetic fix — even shorter caption ..."
    fn isissue(&self) -> bool {
        RE_ISSUE.is_match(&self.msg)
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

#[test]
fn check_not_semvertag() {
    let c = Commit {
        date: "2021-10-15".to_string(),
        msg: "some text".to_string()
    };
    assert_eq!(false, c.issemvertag());
}
