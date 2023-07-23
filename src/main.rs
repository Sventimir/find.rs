use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;


struct File {
  path : PathBuf,
  meta : fs::Metadata
}

impl File {
  fn new(p : String) -> File {
    let path = PathBuf::from(p);
    let meta = fs::metadata(path.as_path()).unwrap();
    File { path, meta }
  }

  fn filter_and_format(self) -> Option<String> {
    match self.path.file_name() {
      Some(n) => n.to_str().map(|s| s.to_string()),
      None => None
    }
  }
}

struct DirQueue {
  queue : VecDeque<fs::ReadDir>
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

fn main() {
  let args : Vec<String> = env::args().collect();
  let path = match &args.get(1) {
    None => ".",
    Some(path) => path,
  };
  let init = File::new(path.into());
  if init.meta.is_dir() {
    let d = fs::read_dir(path).expect("Could not read directory");
    let q = DirQueue { queue : VecDeque::from(vec![d]) };
    for name in q.filter_map(File::filter_and_format) {
      println!("{}", name);
    }
  } else {
    match init.filter_and_format() {
      None => (),
      Some(name) => println!("{}", name)
    }
  }
}
