use crate::{Opt};

pub fn process(settings: &Opt)  -> Vec<String> {
    // load raw data
    let raw = std::fs::read_to_string(&settings.gitlogfile).unwrap();

    // vec of all commit Strings, one element per lime
    let rawvec: Vec<String> = raw.lines().map(|l| l.trim().to_string()).collect();

    // vec of clean commits, one commit per element
    let mut cleanvec: Vec<String> = cleanraw(rawvec);
    cleanvec.sort();
    cleanvec.dedup();
    cleanvec
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
        if temp.to_ascii_lowercase().trim().starts_with("ibid.") {
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

        if temp.len() > 0 {
            returnvec.push(temp.to_string());
        }
        i = i + 1;
    }
    returnvec
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