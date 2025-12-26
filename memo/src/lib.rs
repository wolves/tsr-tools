use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Result},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Memos {
    path: PathBuf,
    pub inner: Vec<Memo>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };
        if fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            for memo in file.lines() {
                memos.inner.push(memo?);
            }
        }
        Ok(memos)
    }
    pub fn sync(&self) -> Result<()> {
        fs::write(&self.path, self.inner.join("\n"))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Pending,
    Done,
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos = Memos::open("tests/data/memos.txt").unwrap();
        assert_eq!(
            memos.inner,
            vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                }
            ],
            "wrong data"
        );
    }

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = Memos::open("bogus.txt").unwrap();
        assert!(memos.inner.is_empty(), "vec not empty");
    }

    #[test]
    fn sync_writes_vec_to_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.txt");
        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ],
        };
        memos.sync().unwrap();

        let memos_2 = Memos::open(&path).unwrap();
        assert_eq!(memos.inner, memos_2.inner, "wrong data");
    }
}
