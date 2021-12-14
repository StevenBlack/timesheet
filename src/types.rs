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
    static ref RE_ISSUEPREFIX: Regex = Regex::new(r"^Issue.*#.*:").unwrap();
}

pub trait Semver {
    fn issemvertag(&self) -> bool;
}

impl Semver for Commit {
    // Matches digits only
    // 1 - Major
    // 2 - Minor
    // 3 - Patch
    fn issemvertag(&self) -> bool {
        RE_SEMVER.is_match(&self.msg)
    }
}

pub trait Issueprefix {
    fn isissueprefix(&self) -> bool;
}

impl Issueprefix for Commit {
    // "Issue #315: fix — replace the Input control..."
    // "Issue #315: refactor — move fetchdata() to ...."
    // "Issue #495: cosmetic fix — even shorter caption ..."
    fn isissueprefix(&self) -> bool {
        RE_ISSUEPREFIX.is_match(&self.msg)
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

