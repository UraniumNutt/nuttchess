use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;
pub struct Comm {
    file: File,
}

impl Comm {
    pub fn create(log_file: &Path) -> Result<Comm, String> {
        match OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .truncate(true)
            .open(log_file)
        {
            Ok(file) => Ok(Comm { file }),
            Err(message) => Err(message.to_string()),
        }
    }
    // Takes in a string, prints it, and logs it
    pub fn engine_out(&mut self, message: String) {
        self.file
            .write(format!("<< {}\n", message.trim()).as_bytes())
            .unwrap();
        self.file.flush().unwrap();
        print!("{}\n", message);
    }

    // Returns a string from input, and logs it
    pub fn engine_in(&mut self) -> String {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        self.file
            .write(format!(">> {}\n", buffer.to_string().trim()).as_bytes())
            .unwrap();
        self.file.flush().unwrap();
        buffer.trim().to_string()
    }

    // Quick and dirty skip past the uci init
    // TODO make the engine properly implement uci
    pub fn prelude(&mut self) -> bool {
        let mut input = self.engine_in();
        while input != "uci" {
            if input == "quit" {
                return false;
            }
            input = self.engine_in();
        }
        while input != "isready" {
            if input == "quit" {
                return false;
            }
            input = self.engine_in();
        }
        self.engine_out("readyok".to_string());

        while input != "ucinewgame" {
            if input == "quit" {
                return false;
            }
            input = self.engine_in();
        }
        true
    }
}
