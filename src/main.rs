mod data;

use std::collections::VecDeque;
use std::env;
use std::fs;
use data::file::{File, DirQueue};
use data::filter::{Filter, DEFAULT};


fn main() {
  let args : Vec<String> = env::args().collect();
  let path = match &args.get(1) {
    None => ".",
    Some(path) => path,
  };
  let init = File::new(path.into());
  let f: Filter = DEFAULT;
  if init.meta.is_dir() {
    let d = fs::read_dir(path).expect("Could not read directory");
    let q = DirQueue { queue : VecDeque::from(vec![d]) };
    for name in q.filter_map(|p| f.apply(p)) {
      println!("{}", name);
    }
  } else {
    match init.filter_and_format() {
      None => (),
      Some(name) => println!("{}", name)
    }
  }
}
