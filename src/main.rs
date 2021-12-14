use std::env;
use std::path::{Path, PathBuf};
mod utils;
mod git;
mod macjournal;
use crate::utils::common::{file_to_string, commas_and};
mod types;
use serde_derive::Deserialize;
use structopt::StructOpt;
use structopt_toml::StructOptToml;

use crate::types::{Commit, Commits, Semver, Issueprefix};
// use crate::Semver;

use git::process as fromgit;
use macjournal::process as frommacjournal;

// configuration file
const CONFIG_FILENAME: &str = ".timesheetrc";
#[derive(Debug, Deserialize, StructOpt, StructOptToml)]
#[structopt(name = "timesheet", about = "Timesheet from git log output and MaJournal export data.")]
#[serde(default)]
pub struct Opt {
    /// Sets up a dry-run, does not timesheet create output
    #[structopt(short, long)]
    dryrun: bool,

    /// The git log input file
    #[structopt(short, parse(from_os_str), default_value = "./commits.txt")]
    gitlogfile: PathBuf,

    /// The MacJournal input file
    #[structopt(short, parse(from_os_str), default_value = "./macjournal.txt")]
    macjournalfile: PathBuf,

    /// Output file, stdout if not present
    #[structopt(short, parse(from_os_str))]
    outfile: Option<PathBuf>,

    /// Putput process information
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
            // println!("No .timesheet file was found.");
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

    if settings.verbose || settings.dryrun {
        println!("git lines: {}", gitvec.len());
        println!("MacJournal lines: {}", macjournalvec.len());
    }

    let mut cleanvec: Vec<String> = vec![];
    cleanvec.extend(gitvec);
    cleanvec.extend(macjournalvec);
    cleanvec.sort();
    cleanvec.dedup();

    if settings.dryrun {
        return;
    };

    // the date currently being processed
    let mut curdate: &str = "";

    // vec of all Commits for a given date
    let mut datevec: Commits = vec![];

    // vec of commits for all dates
    let mut datevecs: Vec<Commits> = vec![];

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

    // consolidate common repeated elements within days
    // first: semver commits
    let mut datevecs_temp: Vec<Commits> = vec![];
    for day in datevecs.iter() {
        datevecs_temp.push(semvercommits(day.clone()));
    }
    datevecs = datevecs_temp;

    // second: The "Issue #nnn: ..." lines
    datevecs_temp = vec![];
    for day in datevecs.iter() {
        datevecs_temp.push(issueprefixcommits(day.clone()));
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
fn issueprefixcommits(commits: Commits) -> Commits {
    let (issues, mut other):(Vec<Commit>, Vec<Commit>) = commits
        .into_iter()
        .partition(|x|(x.isissueprefix()));

    if issues.len() == 0 {
        return other;
    }
    let mut msgs: Vec<String> = vec![];
    let date = &issues[0].date;
    for c in &issues {
        msgs.push(c.msg.clone());
    }
    let iss: Commit = Commit {
        date: date.to_string(),
        msg: format!("XXXX {:?} XXXX.", msgs),
    };
    other.push(iss);
    return other;
}

/// Squash the semver commits into a single vec element
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
    let semv: Commit = Commit {
        date: date.to_string(),
        msg: format!("Versions {} built, tested, and rolled out.", commas_and(msgs)),
    };
    other.push(semv);
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
