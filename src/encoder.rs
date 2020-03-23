use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::errors::Error;
use crate::utils::ByteMask;
use image::{ImageBuffer, Rgb};

pub struct Encoder {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    secret: File,
    mask: ByteMask,
    zeroes: usize,
}

impl Encoder {
    pub fn new(image_path: PathBuf, secret_path: PathBuf, mask: ByteMask) -> Result<Self, Error> {
        let image = image::open(image_path)?.to_rgb();
        let secret = File::open(secret_path)?;
        let metadata = secret.metadata()?;

        let image_size = image.len();
        let secret_size = (metadata.len() * mask.chunks as u64) as usize;

        if image_size < secret_size {
            Err(Error::SecretTooLarge)
        } else {
            let zeroes = image_size - secret_size;

            Ok(Encoder {
                image,
                secret,
                mask,
                zeroes,
            })
        }
    }

    pub fn save(&mut self, output: PathBuf) -> Result<(), Error> {
        let mut byte_iter = self.mask;
        let mask = !byte_iter.mask;

        // Iterator over splitted secret bytes
        let secret_bytes = self
            .secret
            .try_clone()?
            .bytes()
            .flat_map(|b| byte_iter.set_byte(b.unwrap()));

        // Fill secret with 0s at the beginning to fit full image and zip it with it
        let image_secret_bytes = self
            .image
            .iter_mut()
            .zip((0..self.zeroes).map(|_| 0).chain(secret_bytes));

        // Write the LSB bytes to the image
        for (p, b) in image_secret_bytes {
            *p = (*p & mask) | b;
        }

        self.image.save(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteMask, Encoder};
    use std::path::PathBuf;

    #[test]
    fn test_new() {
        let mask = ByteMask::new(2).unwrap();
        let encoder = Encoder::new(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            mask,
        )
        .unwrap();
        assert_eq!(encoder.zeroes, 417520);
    }

    #[test]
    fn test_save() {
        let mask = ByteMask::new(2).unwrap();
        let mut encoder = Encoder::new(
            PathBuf::from("./samples/the-matrix.jpg"),
            PathBuf::from("./samples/secret.txt"),
            mask,
        )
        .unwrap();
        encoder.save(PathBuf::from("./samples/tmp.png")).unwrap();
        std::fs::remove_file("./samples/tmp.png").unwrap();
    }
}
