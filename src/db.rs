use std::path::Path;

use crate::subject_desc::{self, SubjectDesc};

trait Repo<T> {
    fn get_all() -> Vec<T>;
}

struct DB<'a> {
    filename: &'a Path
}

impl <'a> DB<'a> {
    pub fn new<P: AsRef<Path>>(filename: &'a P) -> Self {
        Self { filename: filename.as_ref() }
    }
}

impl <'a> Repo<SubjectDesc> for DB<'a> {
    fn get_all() -> Vec<SubjectDesc> {
            
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_db() {
        let db = DB::new(&"/hello.txt");
        assert_eq!(db.filename, Path::new("/hello.txt"));
    }
}