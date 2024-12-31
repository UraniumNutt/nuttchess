use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
pub struct Comm<'a> {
    log_file: &'a Path,
}

impl<'a> Comm<'a> {
    pub fn new(log_file: &'a Path) -> Comm {
        Comm { log_file }
    }

    pub fn engine_out(&self, message: String) {
        // File::open(self.log_file).unwrap().write(message.as_bytes());
        let mut file = File::open(self.log_file).unwrap();
        file.write(message.as_bytes()).unwrap();
        file.flush().unwrap();
        print!("{}", message);
    }

    pub fn engine_in(&self) -> String {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        print!("{}", buffer);
        buffer
    }
}
