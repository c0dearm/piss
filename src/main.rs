//! **PISS (Picture Secret Steganography)** is a command line tool to encode/decode secrets into/from image files using LSB steganography.
//!
//! Run `piss --help` for usage.
//!
//! ## Installation:
//! ```sh
//! cargo install piss
//! ```
//!
//! ## Usage examples
//! ### Encode a secret into an image
//! ```sh
//! piss encode samples/the-matrix.jpg samples/secret.txt samples/the-matrix-reloaded.png
//! ```
//!
//! Original image:
//!
//! ![alt text](https://github.com/c0dearm/piss/raw/master/samples/the-matrix.jpg "Original image")
//!
//! Image with secret:
//!
//! ![alt text](https://github.com/c0dearm/piss/raw/master/samples/the-matrix-reloaded.png "Image with secret")
//!
//! ### Recover secret from image:
//! ```sh
//! piss decode samples/the-matrix-reloaded.png samples/secret-reloaded.txt
//! ```
//!
//! ### Miscelaneous
//! By default, PISS uses 2 bits per image byte to encode the secret, you can change this value if desired:
//! ```sh
//! piss -b 4 encode samples/the-matrix.jpg samples/secret.txt samples/the-matrix-reloaded.png
//! ```
//!
//! Just remember to decode using the same number of bits, otherwise the output will be garbage:
//! ```sh
//! piss -b 4 decode samples/the-matrix-reloaded.png samples/secret-reloaded.txt
//! ```
//!
//! ## Important note
//! It is not recommended to encode secrets and save the output as `.jpg` as compression is performed and the secret is lost.

mod decoder;
mod encoder;
mod errors;
mod utils;

use std::path::PathBuf;
use structopt::StructOpt;

use decoder::Decoder;
use encoder::Encoder;
use errors::Error;
use utils::ByteMask;

#[derive(StructOpt)]
enum Command {
    Encode {
        #[structopt(parse(from_os_str))]
        image: PathBuf,
        #[structopt(parse(from_os_str))]
        secret: PathBuf,
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
    Decode {
        #[structopt(parse(from_os_str))]
        image: PathBuf,
        #[structopt(parse(from_os_str))]
        output: PathBuf,
    },
}

#[derive(StructOpt)]
#[structopt(
    name = "piss",
    about = "Picture secret steganography encoder/decoder",
    author = "Aitor Ruano <codearm@pm.me>"
)]
struct Opt {
    #[structopt(short = "b", long = "bits", default_value = "2")]
    bits: u8,

    #[structopt(subcommand)]
    cmd: Command,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mask = ByteMask::new(opt.bits)?;

    match opt.cmd {
        Command::Encode {
            image,
            secret,
            output,
        } => encode(image, secret, output, mask)?,
        Command::Decode { image, output } => decode(image, output, mask)?,
    }

    Ok(())
}

fn encode(image: PathBuf, secret: PathBuf, output: PathBuf, mask: ByteMask) -> Result<(), Error> {
    let mut encoder = Encoder::new(image, secret, mask)?;
    encoder.save(output)?;
    Ok(())
}

fn decode(image: PathBuf, output: PathBuf, mask: ByteMask) -> Result<(), Error> {
    let decoder = Decoder::new(image, mask)?;
    decoder.save(output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, ByteMask};
    use std::io::BufRead;
    use std::path::PathBuf;

    fn check_secret() {
        let secret = std::io::BufReader::new(std::fs::File::open("./samples/output.txt").unwrap());
        let result = secret.lines().next().unwrap().unwrap();
        assert_eq!(result, "The Matrix has you.");
    }

    fn remove_tmp_files() {
        std::fs::remove_file("./samples/output.png").unwrap();
        std::fs::remove_file("./samples/output.txt").unwrap();
    }

    #[test]
    fn test_integration() {
        let mask = ByteMask::new(1).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(2).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(3).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(4).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(5).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(6).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(7).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        let mask = ByteMask::new(8).unwrap();
        encode(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            PathBuf::from("./samples/output.png"),
            mask,
        )
        .unwrap();
        decode(
            PathBuf::from("./samples/output.png"),
            PathBuf::from("./samples/output.txt"),
            mask,
        )
        .unwrap();
        check_secret();

        remove_tmp_files();
    }
}
