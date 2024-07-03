use std::{error::Error, fs::File, io::{BufRead, BufReader}, str};
use clap::Parser;
use sha1::{Sha1, Digest};

#[derive(
    clap::ValueEnum, Clone, Default, Debug, PartialEq
)]
enum Algorithm {
    #[default]
    Md5,
    Sha1,
    Sha256,
}

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

    /// Algorithm
    #[arg(long, default_value_t, value_enum)]
    algo: Algorithm,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let wordlist_file = File::open(args.wordlist)?;
    let reader = BufReader::new(&wordlist_file);

    for line_result in reader.split(b'\n') {
        let line = line_result?;

        let mut hex_digest: String = "".to_string();

        if args.algo == Algorithm::Md5 {
            let digest = md5::compute(line.clone());
            hex_digest = format!("{:x}", digest);
        } else if args.algo == Algorithm::Sha1 {
            let mut hasher = Sha1::new();
            hasher.update(line.clone());

            hex_digest = format!("{:x}", hasher.finalize());
        } else if args.algo == Algorithm::Sha256 {
            hex_digest = sha256::digest(line.clone());
        }

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
