use std::{error::Error, fs::File, io::{BufRead, BufReader}, str};
use clap::Parser;

/// Simple hash cracker
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Wordlist
    #[arg(long)]
    wordlist: String,

    /// Hash
    #[arg(long)]
    hash: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let wordlist_file = File::open(args.wordlist)?;
    let reader = BufReader::new(&wordlist_file);

    for line_result in reader.split(b'\n') {
        let line = line_result?;

        let digest = md5::compute(line.clone());
        let hex_digest = format!("{:x}", digest);

        if args.hash == hex_digest {
            let s = match str::from_utf8(&line) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("Password found: {}", &s);
            return Ok(())
        }
    }

    println!("Password not found");

    Ok(())
}
