mod comm;
use crate::comm::*;
use std::path::Path;

fn main() {
    let path = Path::new("test.txt");
    let com = Comm::new(&path);
    com.engine_out("This is a test".to_string());
}
