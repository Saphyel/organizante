extern crate open;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    thread,
    time::Duration
};


pub fn search_from_file(filename: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        open::that_in_background(&line?);
        thread::sleep(Duration::from_secs(2));
    }
    Ok(())
}
