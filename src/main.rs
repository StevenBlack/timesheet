use std::env;
use std::path::{Path, PathBuf};
mod utils;
mod git;
mod macjournal;
mod types;
use structopt::StructOpt;

use crate::types::{Commit, Commits, Semver};
// use crate::Semver;

use git::process as fromgit;
use macjournal::process as frommacjournal;

// configuration
const CONFIG_FILENAME: &str = ".timesheet";
#[derive(Debug, StructOpt)]
#[structopt(name = "timesheet", about = "Timesheet input parser.")]
pub struct Opt {
    // Is this a dry run?
    #[structopt(short, long)]
    dryrun: bool,

    // The git log file
    #[structopt(short, parse(from_os_str), default_value = "/Users/steve/Dropbox/commits.sample.txt")]
    gitlogfile: PathBuf,

    // The MacJournal input file
    #[structopt(short, parse(from_os_str), default_value = "/Users/steve/Dropbox/macjournal.sample.txt")]
    macjournalfile: PathBuf,

    // The output file
    #[structopt(short, parse(from_os_str))]
    outfile: Option<PathBuf>,

    #[structopt(short, long)]
    verbose: bool,
}

fn main()  {
    let settings = Opt::from_args();

    if settings.verbose {
      println!("{:?}", settings);
      println!("{:?}", settings.verbose);
    }

    // locate the config file, if any, here or recursively in parent folders
    let path = env::current_dir().unwrap();
    match find_config_file(&path) {
        Some(filepath) => println!(".timesheet file is found: {:?}", filepath),
        None => println!("No .timesheet file was found."),
    };

    let mut gitvec: Vec<String> = vec![];
    // ckeck if the gitfile exists
    if std::path::Path::new(&settings.gitlogfile).exists() {
        gitvec.extend(fromgit(&settings));
    }

    let mut macjournalvec: Vec<String> = vec![];
    // ckeck if the gitfile exists
    if std::path::Path::new(&settings.macjournalfile).exists() {
        macjournalvec.extend(frommacjournal(&settings));
    }

    if settings.verbose {
        println!("git lines: {}", gitvec.len());
        println!("MacJournal lines: {}", macjournalvec.len());
    }

    let mut cleanvec: Vec<String> = vec![];
    cleanvec.extend(gitvec);
    cleanvec.extend(macjournalvec);

    if settings.dryrun {return};

    // the date being processed
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
