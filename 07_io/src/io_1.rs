use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

fn main() -> io::Result<()> {
    let f = File::open("Cargo.toml")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    let mut f = File::open("Cargo.toml")?;
    let mut buffer = [0; 10];

    // skip to the last 10 bytes of the file
    f.seek(SeekFrom::End(-10))?;

    // read up to 10 bytes
    let n = f.read(&mut buffer)?;

    println!("The bytes: {:?}", &buffer[..n]);

    Ok(())
}
