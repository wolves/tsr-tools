use std::path::Path;

use anyhow::Result;

pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos: Vec<_> = open("tests/data/memos.txt").unwrap();
        assert_eq!(memos, vec!["foo", "bar"], "wrong data");
    }
}
