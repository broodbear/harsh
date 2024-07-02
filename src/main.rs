use std::{error::Error, fs::File, io::{BufRead, BufReader}};
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

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();

        let digest = md5::compute(common_password.as_bytes());
        let hex_digest = format!("{:x}", digest);

        if args.hash == hex_digest {
            println!("Password found: {}", &common_password);
            return Ok(())
        }
    }

    println!("Password not found");

    Ok(())
}
