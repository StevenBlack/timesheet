use crate::common::common::{file_to_string};
use crate::types::{Commit, Commits};
use crate::Semver;

pub fn process()  {
    // load raw data
    let commitsfile= "/Users/steve/Dropbox/commits.sample.txt".to_string();
    let raw = file_to_string(commitsfile).trim().to_string();

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

        // ignore commits containing "Ibid."
        if temp.to_ascii_lowercase().contains("ibid.") {
            continue;
        }

        // ignore commits containing "whitespace"
        if temp.to_ascii_lowercase().replace(" ", "").contains("whitespace") {
            continue;
        }

        // ignore commits containing "typo"
        if temp.to_ascii_lowercase().contains("typo") {
            continue;
        }

        // Does this line contain a MacJournal export date?
        if temp.len() > 5 && &temp[0..5] == "Date:" {
            temp = cleanrawdate(temp);
        }

        if temp.len() > 0 {
            returnvec.push(temp.to_string());
        }
        i = i + 1;
    }
    returnvec
}

fn cleanrawdate(datestring: String) -> String {
    let strvec:Vec<_> =datestring.split_ascii_whitespace().collect();
    let day = format!("{:0>2}", strvec[1]);
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
fn check_cleanraw() {
    let mut t: Vec<String> = vec![];
    t.push("Fix typo".to_string());
    t.push("Fix whitespace".to_string());
    t.push("Remove white space".to_string());
    t.push(" this is alpha ".to_string());
    t.push("".to_string());
    t.push("Ibid.".to_string());
    t.push("".to_string());
    t.push(" this is beta".to_string());
    let o = cleanraw(t);

    assert_eq!(2, o.len());
    assert_eq!("this is alpha".to_string(), o.first().unwrap().to_string());
}

#[test]
fn check_process() {
    process();
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
