use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

mod capi;

pub fn lib_function_example_add(num_one: usize, num_two: usize) -> usize {
    num_one + num_two
}

pub fn read_file() -> String {
    let file = File::open("src/simple_test.txt").unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut test = String::new();

    loop {
        if let Some(s) = lines.next() {
            test.push_str(&s.unwrap());
        } else {
            break;
        }
    }
    return test;
}

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod prog_handle;

// Unit tests here
#[cfg(test)]
mod tests {
    use crate::{lib_function_example_add, parser, read_file};

    // Testing larlpop functionality
    #[test]
    fn parser() {
        assert!(parser::TermParser::new().parse("22").is_ok());
        assert!(parser::TermParser::new().parse("(22)").is_ok());
        assert!(parser::TermParser::new().parse("((((22))))").is_ok());
        assert!(parser::TermParser::new().parse("((22)").is_err());
    }

    #[test]
    fn test_add() {
        let result = lib_function_example_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn test_add_wrong() {
        let result = 3;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_open_file() {
        let a = read_file();
        assert_eq!(a.is_empty(), false);
    }
}
