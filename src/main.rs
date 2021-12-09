mod common;
mod git;
mod macjournal;
mod types;
mod traits;

use git::process as gitprocess;
use macjournal::process as mjprocess;


pub trait Semver {
    fn issemvertag(&self) -> bool;
}

fn main()  {
    gitprocess();
    mjprocess();
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
}
