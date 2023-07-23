use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;


pub struct File {
  pub path : PathBuf,
  pub meta : fs::Metadata
}

impl File {
  pub fn new(p : String) -> File {
    let path = PathBuf::from(p);
    let meta = fs::metadata(path.as_path()).unwrap();
    File { path, meta }
  }

  pub fn filter_and_format(self) -> Option<String> {
    match self.path.file_name() {
      Some(n) => n.to_str().map(|s| s.to_string()),
      None => None
    }
  }
}

pub struct DirQueue {
  pub queue : VecDeque<fs::ReadDir>
}

impl Iterator for DirQueue {
  type Item = File;

  fn next(&mut self) -> Option<Self::Item> {
    match self.queue.pop_front() {
      None => None,
      Some(mut dir) =>
        match dir.next() {
          None => self.next(),
          Some(entry) => {
            self.queue.push_front(dir);
            let file = entry.unwrap();
            let meta = file.metadata().unwrap();
            if meta.is_dir() {
              let path = file.path();
              let dir = fs::read_dir(path).unwrap();
              self.queue.push_front(dir);
            };
            Some(File { path: file.path(), meta })
          }
        }
    }
  }
}
