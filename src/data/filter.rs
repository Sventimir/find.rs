use super::file::File;

type Filt = fn(&File) -> bool;
type Format = fn(File) -> Option<String>;

pub struct Filter {
  filter: Filt,
  format: Format,
}

impl Filter {
  pub fn apply(&self, file: File) -> Option<String> {
    if (self.filter)(&file) {
      (self.format)(file)
    } else {
      None
    }
  }
}

pub mod filters {
  use super::File;

  pub fn any(_: &File) -> bool {
    true
  }
}

pub mod formats {
  use super::File;

  pub fn default(f: File) -> Option<String> {
    let p = f.path.to_str().map(|s|
                                if f.meta.is_dir() {
                                  format!("{}{}", s, "/")
                                } else {
                                  s.to_string()
                                });
    if p.is_none() {
      eprintln!("Error: failed to display some paths.");
    }
    p
  }
}

pub const DEFAULT: Filter = Filter {
  filter: filters::any,
  format: formats::default,
};
