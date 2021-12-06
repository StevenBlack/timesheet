mod common;
use common::common::*;

fn main() {
    println!("Hello, world!");
    process();
}

fn process()  {
    let commitsfile= "/Users/steve/Dropbox/commits.txt".to_string();
    let mut curdate: &str = "";
    let mut datevec: Vec<&String> = vec![];
    let mut datevecs: Vec<Vec<&String>> = vec![];
    let commitsvec = file_to_vec(commitsfile).unwrap();
    for commit in commitsvec.iter() {
        let (date, msg) = commit.split_once(' ').unwrap();
        if msg == "Ibid." {
            continue;
        }
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

    for date in datevecs.iter() {
        println!("{:#?}", date.len());
    }
    println!("{:#?}", commitsvec.len());
    println!("{:#?}", datevecs.len());
}
