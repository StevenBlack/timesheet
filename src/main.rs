use std::env;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
mod utils;
mod git;
mod macjournal;
use crate::utils::common::{file_to_string, commas_and};
mod types;
use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_toml::StructOptToml;
use types::Commitinfo;

use crate::types::{Commit, Commits, Semver, Issue};
// use crate::Semver;

use git::process as fromgit;
use macjournal::process as frommacjournal;

// configuration file
const CONFIG_FILENAME: &str = ".timesheetrc";
#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[structopt(name = "timesheet", about = "Timesheet from git log output and MaJournal export data.")]
#[serde(default)]
pub struct Opt {
    /// The git log input file
    #[structopt(short, parse(from_os_str), default_value = "./commits.txt")]
    gitlogfile: PathBuf,

    /// The MacJournal input file
    #[structopt(short, parse(from_os_str), default_value = "./macjournal.txt")]
    macjournalfile: PathBuf,

    /// Output process information
    #[structopt(short, long)]
    verbose: bool,
}

fn main()  {

    // locate the config file, if any, here or recursively in parent folders
    let mut config_file: Option<PathBuf> = None;
    let path = env::current_dir().unwrap();
    match find_config_file(&path) {
        Some(filepath) => {
            config_file = Some(filepath);
            // println!(".timesheet file is found: {:?}", filepath);
        },
        _ => {
            // println!("No .timesheet file found.");
        },
    };

    let mut fname: String = "".to_string();
    let settings: Opt;
    if config_file.is_some() {
        fname = config_file.unwrap().to_str().unwrap_or("").to_string();
        let toml_str = file_to_string(fname.clone());
        settings = Opt::from_args_with_toml(&toml_str).expect("toml parse failed");
    } else {
        settings = Opt::from_args();
    }

    if settings.verbose {
      println!("Config file: {}", fname);
      println!("Settings {:?}", settings);
    }


    let mut gitvec: Vec<String> = vec![];
    // ckeck if the gitfile exists
    if std::path::Path::new(&settings.gitlogfile).exists() {
        gitvec.extend(fromgit(&settings));
    } else if settings.verbose {
        println!(
            "Git log file {:?} not found.",
            &settings.gitlogfile.to_str()
        );
    }

    let mut macjournalvec: Vec<String> = vec![];
    // ckeck if the gitfile exists
    if std::path::Path::new(&settings.macjournalfile).exists() {
        macjournalvec.extend(frommacjournal(&settings));
    } else if settings.verbose {
        println!(
            "MacJournal file {:?} not found.",
            &settings.macjournalfile.to_str());
    }

    if settings.verbose {
        println!("git lines: {}", gitvec.len());
        println!("MacJournal lines: {}", macjournalvec.len());
    }

    let mut cleanvec: Vec<String> = vec![];
    cleanvec.extend(gitvec);
    cleanvec.extend(macjournalvec);
    cleanvec.sort();
    cleanvec.dedup();

    // the date currently being processed
    let mut curdate: &str = "";

    // vec of all Commits for a given date
    let mut datevec: Commits = vec![];

    // vec of commits for all dates
    let mut datevecs: Vec<Commits> = vec![];

    // load our datevecs
    for (index, commit) in cleanvec.iter().enumerate() {
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
        if index == cleanvec.len() {
        }
    }
    // and finally,
    datevecs.push(datevec);

    // consolidate semver commits within days
    let mut datevecs_temp: Vec<Commits> = vec![];
    for day in datevecs.iter() {
        datevecs_temp.push(semvercommits(day.clone()));
    }
    datevecs = datevecs_temp;

    // consolidate version semver commits within days
    datevecs_temp = vec![];
    for day in datevecs.iter() {
        datevecs_temp.push(versionsemvercommits(day.clone()));
    }
    datevecs = datevecs_temp;

    // consolidate "Issue #nnn: ..."  commits within days
    datevecs_temp = vec![];
    for day in datevecs.iter() {
        datevecs_temp.push(issuecommits(day.clone()));
    }
    datevecs = datevecs_temp;

    // now output:
    for day in datevecs.iter() {
        let mut out = day[0].date.to_owned();
        let xday = day.clone();
        for commit in xday {
            out.push_str(" ");
            out.push_str(commit.msg.as_str());
        }
        println!("{}", out);
    }
}

/// Squash the issue commits into a single vec element
fn issuecommits(commits: Commits) -> Commits {
    let (takes, mut other):(Vec<Commit>, Vec<Commit>) = commits
        .into_iter()
        .partition(|x|(x.isissue()));

    if takes.len() == 0 {
        return other;
    }
    let mut hashmap: HashMap<&str, Vec<&str>> = HashMap::new();
    let date = &takes[0].date;
    for c in &takes {
        let (take, desc) = c.msg.split_once(':').unwrap();
        let trimmed = desc.trim_end_matches(".");
        hashmap.entry(take).or_insert_with(Vec::new).push(trimmed);
    }
    for (key, value) in hashmap.iter() {
        let commit: Commit = Commit {
            date: date.to_owned(),
            msg: format!("{}:{}.", key.to_string(), value.join(";").to_string()),
        };
        other.push(commit);
    }
    return other;
}

#[test]
fn check_issuecommits() {
    let mut testcommits: Commits = vec![];
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue #3082: exploring ways to make ghostscript optimization happen automatically.".to_string()});
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue #423: fix — limit the height of the picker.".to_string()});
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue #423: fix — Remove the keyExtractor function.".to_string()});
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue #423: fix — rename driver to drvr in this scope.".to_string()});
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue #423: make the pickers a bit smaller.".to_string()});
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue #423: semantics — singular of drivers is driver.".to_string()});
    testcommits.push(Commit{ date: "2021-10-15".to_string(), msg: "Issue curation.".to_string()});
    let output = issuecommits(testcommits);
    assert_eq!(output.len(), 3);
}

/// Squash the semver commits into a single vec element
fn semvercommits(commits: Commits) -> Commits {
    let (takes, mut other):(Vec<Commit>, Vec<Commit>) = commits
        .into_iter()
        .partition(|x|(x.issemvertag()));

    if takes.len() == 0 {
        return other;
    }
    let mut msgs: Vec<String> = vec![];
    let date = &takes[0].date;
    for c in &takes {
        msgs.push(c.msg.clone());
    }
    let v = if msgs.len() < 2 { "version" } else { "versions" };
    let fixed: Commit = Commit {
        date: date.to_string(),
        msg: format!("{} {} built, tested, and rolled out.", v, commas_and(msgs)),
    };
    other.push(fixed);
    return other;
}

/// Squash the version semver commits into a single vec element
fn versionsemvercommits(commits: Commits) -> Commits {
    let (takes, mut other):(Vec<Commit>, Vec<Commit>) = commits
        .into_iter()
        .partition(|x|(x.isversionsemvertag() && x.msg_words() < 5));

    if takes.len() == 0 {
        return other;
    }
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let date = &takes[0].date;
    for c in &takes {
        let (take, rest) = c.msg.split_once(' ').unwrap();
        let (_, desc) = rest.split_once(' ').unwrap();
        let trimmed = desc.trim_end_matches(".");
        hashmap.entry(take.to_string()).or_insert_with(Vec::new).push(trimmed.to_string());
    }
    for (key, msgs) in hashmap.iter() {
        let v = if msgs.len() < 2 { "version" } else { "versions" };
        let commit: Commit = Commit {
            date: date.to_owned(),
            msg: format!("{} {} {} built, tested, and rolled out.", key.to_string(), v, commas_and(msgs.clone())),
        };
        other.push(commit);
    }
    return other;
}

#[test]
fn check_semvercommits() {
    let mut testcommits: Commits = vec![];
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "0.0.1".to_string() });
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "0.0.2".to_string() });
    let output = semvercommits(testcommits);
    assert_eq!(output.len(), 1);
}
#[test]
fn check_versionsemvercommits() {
    let mut testcommits: Commits = vec![];
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Bar version 0.10.0".to_string() });
    let output = versionsemvercommits(testcommits);
    assert_eq!(output.len(), 1);
    assert_eq!(output[0].msg, "Bar version 0.10.0 built, tested, and rolled out.".to_string());
}

#[test]
fn check_versionsemvercommits_2() {
    let mut testcommits: Commits = vec![];
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Foo version 0.0.1".to_string() });
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Foo version 0.0.2".to_string() });
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Foo version 0.0.3".to_string() });
    let output = versionsemvercommits(testcommits);
    assert_eq!(output.len(), 1);
    assert_eq!(output[0].msg, "Foo versions 0.0.1, 0.0.2, and 0.0.3 built, tested, and rolled out.".to_string());
}

#[test]
fn check_versionsemvercommits_3() {
    let mut testcommits: Commits = vec![];
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Bar version 0.10.0".to_string() });
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Foo version 0.0.1".to_string() });
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Foo version 0.0.2".to_string() });
    testcommits.push(Commit{ date: "2021-01-01".to_string(), msg: "Foo version 0.0.3".to_string() });
    let output = versionsemvercommits(testcommits);
    assert_eq!(output.len(), 2);
}

fn find_config_file(starting_directory: &Path) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(CONFIG_FILENAME);

    loop {
        path.push(file);

        if path.is_file() {
            break Some(path);
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break None;
        }
    }
}
