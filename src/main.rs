extern crate data_encoding;
extern crate ring;

use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read, Error};
use structopt::StructOpt;

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        // reader input var of type? R that implements? trait std::io::Read?
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


// how should the sha256 digest be used?
// what other ways can we present a stream of bytes?

fn main() -> Result<(), Error> {

    let args = Cli::from_args();
    let input = File::open(args.path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("sha256 digest: {}", HEXUPPER.encode(digest.as_ref()));
     

    Ok(())
}
