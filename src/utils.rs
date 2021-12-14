/// Steven Black's rust utils
/// This module contains helpful utilities I've collected along the way.
#[allow(dead_code)]

pub mod common {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::io;
    use std::fs;
    use std::io::prelude::*;
    use std::fs::File;

    /// read a file to string.
    pub fn file_to_string(filename: String) -> String {
        let mut file = File::open(filename).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        contents
    }

    /// convert a String to Vec<String>.
    pub fn string_to_vec(param: String) -> Vec<String> {
        param.lines().map(|l| l.to_string()).collect()
    }

    pub fn string_split_to_vec(param: String, splt: String) -> Vec<String> {
        param.split(splt.as_str()).map(|l| l.to_string()).collect()
    }

    /// read a file into Result<Vec<String>>.
    pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
        if fs::metadata(filename.clone()).is_err() {
            panic!("Bad file {}", filename);
        }
        let file_in = fs::File::open(filename)?;
        let file_reader = BufReader::new(file_in);
        Ok(file_reader.lines().filter_map(io::Result::ok).collect())
    }

    /// read a file into Vec<Vec<char>>.
    pub fn file_to_vec_of_char_vec(filename: String) -> Vec<Vec<char>> {
        let readresult = file_to_vec(filename);
        let readvec = match readresult {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        readvec.iter().map(|l| l.chars().collect()).collect()
    }

    /// read a file into Vec<Vec<String>> (words).
    pub fn file_to_vec_of_words_vec(filename: String) -> Vec<Vec<String>> {
        // let v:Vec<&str> = "Hello, world!".split_whitespace().collect();
        // let v:Vec<String> = "Hello, world! I'm having a great day, you?".split_whitespace().map(|w| w.to_string()).collect();
        let readresult = file_to_vec(filename);
        let readvec = match readresult {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let vecvec :Vec<Vec<String>> = readvec.iter().map(|l| l.split_whitespace().map(|l| l.to_string()).collect()).collect();
        vecvec
    }

    /// literate split, using  ", " with a trailing " and " as required.
    pub fn commas_and(mut vec: Vec<String>) -> String {
        let length = vec.len();
        if length > 1 {
            vec[length - 1] = format!("and {}", vec[length - 1]);
            if length == 2 {
                return vec.join(" ").to_string();
            }
        }
        vec.join(", ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::common::*;

    #[test]
    fn test_commas_and() {
        let mut a = vec!();
        a.push("Apples".to_string());
        assert_eq!(commas_and(a), "Apples".to_string());
        a = vec!();
        a.push("Apples".to_string());
        a.push("oranges".to_string());
        assert_eq!(commas_and(a), "Apples and oranges".to_string());
        a = vec!();
        a.push("Apples".to_string());
        a.push("oranges".to_string());
        a.push("bananas".to_string());
        assert_eq!(commas_and(a), "Apples, oranges, and bananas".to_string());
    }
}