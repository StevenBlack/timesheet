mod common;
use std::fmt;

use common::common::*;

#[derive(Default, Debug)]
struct Commitstruc {
    date:String,
    msg:String,
}

impl fmt::Display for Commitstruc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.msg)
    }
}

fn main()  {
    let commitsfile= "/Users/steve/Dropbox/commits.txt".to_string();
    let mut curdate: &str = "";
    let mut datevec: Vec<Commitstruc> = vec![];
    let mut datevecs: Vec<Vec<Commitstruc>> = vec![];
    let commitsvec = file_to_vec(commitsfile).unwrap();
    for commit in commitsvec.iter() {
        let (date, msg) = commit.split_once(' ').unwrap();
        let commitstruc = Commitstruc{ date: date.to_string(), msg: msg.to_string() };
        if msg == "Ibid." {
            continue;
        }
        if date != curdate {
            if datevec.len() > 0 {
                datevecs.push(datevec);
            }
            curdate = date;
            datevec = vec![commitstruc];
        } else {
            datevec.push(commitstruc);
        }
    }

    println!("{:#?}", commitsvec.len());
    println!("{:#?}", datevecs.len());
}
