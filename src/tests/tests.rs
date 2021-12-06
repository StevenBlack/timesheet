extern crate common;
pub use common::*;

#[cfg(test)]
mod tests {
    #[test]
    fn check_string_to_vec() {
        let myvec = common::string_to_vec("line 1\nLine 2".to_string());
        assert!(myvec.len() == 2);
    }

    #[test]
    fn input_readable_by_file_to_string() {
        let mystring = common::file_to_string("input2.txt".to_string());
        assert!(mystring.len() > 0);
    }

    #[test]
    #[should_panic]
    fn bad_input_file_to_string() {
        let _ = common::file_to_string("badinput.txt".to_string());
    }

    #[test]
    fn convert_strin_to_vec() {
        let my_string = common::file_to_string("input2.txt".to_string());
        let my_vec= common::string_to_vec(my_string);
        assert!(my_vec.len() > 0);
    }

    #[test]
    fn convert_string_split_to_vec() {
        let my_string = common::file_to_string("/Users/steve/Dropbox/commits.txt".to_string());
        let my_vec = common::string_split_to_vec(my_string, 'r'.to_string());
        assert!(my_vec.len() > 0);
    }

    #[test]
    #[should_panic]
    fn convert_strin_to_vec_panic() {
        let my_string = common::file_to_string("badinput.txt".to_string());
        let my_vec = common::string_to_vec(my_string);
        assert!(my_vec.len() > 0);
    }

    #[test]
    fn input_readable_by_file_to_vec() {
        let my_vec = common::file_to_vec("/Users/steve/Dropbox/commits.txt".to_string()).unwrap();
        assert!(my_vec.len() > 0);
    }

    #[test]
    fn input_readable_by_file_to_vec_of_char_vec() {
        let my_vec = common::file_to_vec_of_char_vec("/Users/steve/Dropbox/commits.txt".to_string());
        assert!(my_vec.len() > 0);
    }

    #[test]
    fn input_readable_by_file_to_vec_of_words_vec() {
        let my_vec = common::file_to_vec_of_words_vec("/Users/steve/Dropbox/commits.txt".to_string());
        assert!(my_vec.len() > 0);
    }

    #[test]
    #[should_panic]
    fn bad_input_file_to_vec() {
        let _ = common::file_to_vec("badinput.txt".to_string());
    }

    #[test]
    #[should_panic]
    fn bad_input_file_to_vec_of_char_vec() {
        let _ = common::file_to_vec_of_char_vec("badinput.txt".to_string());
    }
}
