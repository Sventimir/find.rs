use std::env;
use std::fs;

fn main() {
  let args : Vec<String> = env::args().collect();
  let path = match &args.get(1) {
    None => ".",
    Some(path) => path,
  };
  let paths = fs::read_dir(path).expect("Could not read directory");
  for path in paths {
    println!("{}", path.unwrap().file_name().into_string().unwrap());
  }
}
