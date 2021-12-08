mod common;

use std::{fmt, fs::read_to_string};
use regex::Regex;

use common::common::*;

#[derive(Default, Clone, Debug)]
struct Commit {
    date:String,
    msg:String,
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
        let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
        re.is_match(&self.msg)
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.msg)
    }
}

type Commits = Vec<Commit>;

fn main()  {
    let commitsfile= "/Users/steve/Dropbox/commits.sample.txt".to_string();
    let rawdata = file_to_string(commitsfile).trim().to_string();
    process(rawdata);
}

fn process(raw: String) {
    let mut curdate: &str = "";
    let mut datevec: Commits = vec![];
    let mut datevecs: Vec<Commits> = vec![];
    let rawvec: Vec<String> = raw.lines().map(|l| l.trim().to_string()).collect();

    let cleanvec: Vec<String> = cleanraw(rawvec);

    // load our datevecs
    for commit in cleanvec.iter() {
        // split the date from the message
        let (date, msg) = commit.split_once(' ').unwrap();
        let commit = Commit{ date: date.to_string(), msg: msg.to_string() };
        if date != curdate {
            if datevec.len() > 0 {
                datevecs.push(datevec);
            }
            curdate = date;
            datevec = vec![commit];
        } else {
            datevec.push(commit);
        }
    }

    // now output:
    for day in datevecs.iter() {
        let mut out = day[0].date.to_owned();
        let xday = semvercommits(day.clone());
        for commit in xday {
            out.push_str(" ");
            out.push_str(commit.msg.as_str());
        }
        println!("{}", out);
    }
}

fn semvercommits(commits: Commits) -> Commits {
    let (semver, mut other):(Vec<Commit>, Vec<Commit>) = commits
        .into_iter()
        .partition(|x|(x.issemvertag()));

    if semver.len() == 0 {
        return other;
    }
    let mut msgs: Vec<String> = vec![];
    let date = &semver[0].date;
    for c in &semver {
        msgs.push(c.msg.clone());
    }
    let semv: Commit = Commit { date: date.to_string(), msg: format!("Versions {} built, tested, and rolled out.", msgs.join(", ")) };
    other.push(semv);
    return other;
}

fn cleanraw(rawvec: Vec<String>) -> Vec<String> {
    // cleaning the vec
    let mut returnvec: Vec<String> = vec![];
    let mut i = 0;
    for l in rawvec {
        let mut temp = l.trim().replace("  ", " ");

        if temp.len() == 0 {
            continue;
        }

        // ignore "Ibid." lines
        if temp.to_ascii_lowercase().contains("ibid.") {
            continue;
        }

        // a Macjournal export date?
        if temp.len() > 5 && &temp[0..5] == "Date:" {
            temp = cleanrawdate(temp);
        }

        // // a Macjournal export date?
        // if temp.len() > 6 && &temp[0..6] == "Topic:" {
        //     continue;
        // }

        if temp.len() > 0 {
            returnvec.push(temp.to_string());
        }
        i = i + 1;
    }
    returnvec
}

#[test]
fn x() {
    println!("{}", &("Date: 4 December 2021 at 13:32".to_string())[0..5]);
}

fn mj()  {
    let mjfile= "/Users/steve/Dropbox/macjournal.sample.txt".to_string();
    let rawdata = file_to_string(mjfile).trim().to_string();
    processmj(rawdata);
}

fn processmj(raw: String) {
    let mut rawvec: Vec<String> = raw.lines().map(|l| l.trim().to_string()).collect();
    // remove the "Topic: ..." elements
    rawvec.retain(|x| &x.len() < &6 || &x[0..6] != "Topic:");
    let mut cleanvec: Vec<String> = cleanraw(rawvec);
    println!("{:?}", cleanvec);
}

fn cleanrawdate(datestring: String) -> String {
    let strvec:Vec<_> =datestring.split_ascii_whitespace().collect();
    let day = strvec[1];
    let year = strvec[3];
    let monthstr = match strvec[2] {
        "January" => "01",
        "February" => "02",
        "March" => "03",
        "April" => "04",
        "May" => "05",
        "June" => "06",
        "July" => "07",
        "August" => "08",
        "September" => "09",
        "October" => "10",
        "November" => "11",
        "December" => "12",
        _ => "00"
    };
    let month = monthstr.to_string();
    format!("{}-{}-{}", year, month, day).to_string()
}

#[test]
fn check_mj() {
    mj();
}

#[test]
fn check_cleanrawdate() {
    let teststring = "Date: 15 November 2021 at 12:15".to_string();
    assert_eq!(cleanrawdate(teststring), "2021-11-15".to_string())
}

#[test]
fn check_semvertag() {
    let c = Commit {
        date: "2021-10-15".to_string(),
        msg: "0.11.2".to_string()
    };
    assert_eq!(true, c.issemvertag());
}

#[test]
fn check_consolidation() {
    // want this to be three lines
    let day = "
    2021-10-15 Issue #3082: exploring ways to make ghostscript optimization happen automatically.
    2021-10-15 Issue #423: fix — limit the height of the picker.
    2021-10-15 Issue #423: fix — Remove the keyExtractor function.
    2021-10-15 Issue #423: fix — rename driver to drvr in this scope.
    2021-10-15 Issue #423: make the pickers a bit smaller.
    2021-10-15 Issue #423: semantics — singular of drivers is driver.
    2021-10-15 Issue curation.".to_string();

    todo!("Finish this test");
}

#[test]
fn check_cleanraw() {
    let mut t: Vec<String> = vec![];
    t.push(" this is alpha ".to_string());
    t.push("".to_string());
    t.push("Ibid.".to_string());
    t.push("".to_string());
    t.push(" this is beta".to_string());
    let o = cleanraw(t);

    assert_eq!(2, o.len());
    assert_eq!("this is alpha".to_string(), o.first().unwrap().to_string());
}
