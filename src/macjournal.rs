use crate::{Opt};
use regex::Regex;

pub fn process(settings: &Opt) -> Vec<String> {
    // load raw data
    let raw = std::fs::read_to_string(&settings.macjournalfile).unwrap();

    // vec of all notes, one element per line.
    let mut rawvec: Vec<String> = raw.lines().map(|l| l.trim().to_string()).collect();

    // remove the "Topic: ..." elements
    rawvec.retain(|x| &x.len() > &2);
    rawvec.retain(|x| &x[0..6] != "Topic:");

    let mut cleanvec: Vec<String> = cleanraw(rawvec);
    // we need to normalize the listing
    cleanvec = dateprefix(cleanvec);
    cleanvec.sort();
    cleanvec.dedup();
    cleanvec
}

fn dateprefix(unvec: Vec<String>) -> Vec<String> {
    let mut retvec: Vec<String> =vec![];
    let mut dt: String = String::from(" ");
    for line in unvec {
        if isyyyymmdddate(line.clone()) {
            dt = line;
            continue;
        }
        retvec.push(format!("{} {}", dt, line));
    }
    retvec
}

fn isyyyymmdddate(lin:String) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(lin.as_str())
}

pub fn cleanraw(rawvec: Vec<String>) -> Vec<String> {
    // cleaning the vec
    let mut returnvec: Vec<String> = vec![];
    let mut i = 0;
    for l in rawvec {
        let mut temp = l.trim().replace("  ", " ");

        if temp.len() == 0 {
            continue;
        }

        // a Macjournal export date?
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

#[test]
fn check_cleanraw_macjournal() {
    let mut t: Vec<String> = vec![];
    t.push(" this is alpha ".to_string());
    t.push("".to_string());
    t.push("".to_string());
    t.push(" this is beta".to_string());
    let o = cleanraw(t);

    assert_eq!(2, o.len());
    assert_eq!("this is alpha".to_string(), o.first().unwrap().to_string());
}

pub fn cleanrawdate(datestring: String) -> String {
    let strvec:Vec<_> =datestring.split_ascii_whitespace().collect();
    let day = format!("{:0>2}", strvec[2].replace(&[','][..], ""));
    let year = strvec[3];
    let monthstr = match strvec[1] {
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
fn check_cleanrawdate() {
    let teststring = "Date: 15 November 2021 at 12:15".to_string();
    assert_eq!(cleanrawdate(teststring), "2021-11-15".to_string())
}
