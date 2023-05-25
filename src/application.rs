extern crate open;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    thread,
    time::Duration,
};

pub fn websites_from_file(filename: impl AsRef<Path>) {
    let file = File::open(filename).expect("Failed trying to open the file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        open::that(&line.expect("Failed reading content of the file"))
            .expect("Failed opening the new tab");
        thread::sleep(Duration::from_secs(2));
    }
}
