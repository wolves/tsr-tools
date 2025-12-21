use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Result},
    path::Path,
};

pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
    if fs::exists(&path)? {
        let file = BufReader::new(File::open(&path)?);
        file.lines().collect()
    } else {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos: Vec<_> = open("tests/data/memos.txt").unwrap();
        assert_eq!(memos, vec!["foo", "bar"], "wrong data");
    }

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = open("bogus.txt").unwrap();
        assert!(memos.is_empty(), "vec not empty");
    }
}
