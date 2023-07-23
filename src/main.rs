use std::fs;

fn main() {
  let paths = fs::read_dir(".").expect("Could not read directory");
  for path in paths {
    println!("{}", path.unwrap().file_name().into_string().unwrap());
  }
}
