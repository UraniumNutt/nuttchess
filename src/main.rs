mod comm;
use crate::comm::*;
use std::path::Path;

fn main() {
    let mut engine_input = String::new();
    let log_path = Path::new("log.txt");
    let mut comm = Comm::create(log_path).unwrap();
    if !comm.prelude() {
        return;
    }
    while engine_input != "quit" {
        engine_input = comm.engine_in();
        comm.engine_out(format!(
            "info string {} was the input after the prelude\n",
            engine_input
        ));
    }
}
