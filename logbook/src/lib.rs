use std::fs;

use anyhow::Result;

pub fn read(path: &str) -> Result<Option<String>> {
    if fs::exists(path)? {
        let text = fs::read_to_string(path)?;
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    } else {
        Ok(None)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_reads_contents_of_file_as_string() {
        let text = read("tests/data/logbook.txt").unwrap().unwrap();
        assert_eq!(text.trim_end(), "hello world", "wrong text");
    }

    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let text = read("tests/data/bogus.txt").unwrap();
        assert_eq!(text, None, "expected None")
    }

    #[test]
    fn read_returns_none_for_empty_file() {
        let text = read("tests/data/empty.txt").unwrap();
        assert_eq!(text, None, "expected None")
    }
}
