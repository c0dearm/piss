use crate::errors::Error;

/// Represents a number of bits in a byte, its range is limited to [0, 8]
/// Implements Iterator to iterate through non-overlapping masked bits of the byte
#[derive(Copy, Clone)]
pub struct ByteMask {
    pub bits: u8,
    pub mask: u8,
    pub chunks: u8,
    padded: bool,
    byte: u8,
    step: u8,
}

impl ByteMask {
    pub fn new(bits: u8) -> Result<Self, Error> {
        if (bits == 0) || (bits > 8) {
            Err(Error::InvalidNumberOfBits)
        } else {
            let mask = (u16::pow(2, bits as u32) - 1) as u8;
            let chunks = f32::ceil(8.0 / bits as f32) as u8;
            let padded = 8 < (chunks * bits);

            Ok(ByteMask {
                bits,
                mask,
                chunks,
                byte: 0,
                step: 0,
                padded,
            })
        }
    }

    /// Sets the byte for which to iter over
    pub fn set_byte(&mut self, byte: u8) -> Self {
        self.byte = byte;
        self.step = 0;
        *self
    }

    /// Inverse process, given a set of masked bytes, returns the original byte
    pub fn join_chunks<'a, T>(&self, chunks: &'a T) -> u8
    where
        &'a T: IntoIterator<Item = &'a u8>,
    {
        let mut byte = 0;
        let mut shift: u8 = 8;

        for chunk in chunks {
            shift = shift.saturating_sub(self.bits);
            byte |= chunk << shift;
        }

        byte
    }
}

impl Iterator for ByteMask {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step >= self.chunks {
            return None;
        }

        self.step += 1;

        if self.padded && (self.step == self.chunks) {
            let shift = self.bits * self.step - 8;
            Some(self.byte & (self.mask >> shift))
        } else {
            let shift = 8 - self.bits * self.step;
            Some((self.byte >> shift) & self.mask)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ByteMask;

    #[test]
    fn test_invalid_number() {
        let result = ByteMask::new(9);
        assert!(result.is_err());

        let result = ByteMask::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_values() {
        let mask = ByteMask::new(1).unwrap();
        assert_eq!(mask.bits, 1);
        assert_eq!(mask.mask, 0b00000001);
        assert_eq!(mask.chunks, 8);
        assert_eq!(mask.padded, false);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(2).unwrap();
        assert_eq!(mask.bits, 2);
        assert_eq!(mask.mask, 0b00000011);
        assert_eq!(mask.chunks, 4);
        assert_eq!(mask.padded, false);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(3).unwrap();
        assert_eq!(mask.bits, 3);
        assert_eq!(mask.mask, 0b00000111);
        assert_eq!(mask.chunks, 3);
        assert_eq!(mask.padded, true);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(4).unwrap();
        assert_eq!(mask.bits, 4);
        assert_eq!(mask.mask, 0b00001111);
        assert_eq!(mask.chunks, 2);
        assert_eq!(mask.padded, false);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(5).unwrap();
        assert_eq!(mask.bits, 5);
        assert_eq!(mask.mask, 0b00011111);
        assert_eq!(mask.chunks, 2);
        assert_eq!(mask.padded, true);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(6).unwrap();
        assert_eq!(mask.bits, 6);
        assert_eq!(mask.mask, 0b00111111);
        assert_eq!(mask.chunks, 2);
        assert_eq!(mask.padded, true);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(7).unwrap();
        assert_eq!(mask.bits, 7);
        assert_eq!(mask.mask, 0b01111111);
        assert_eq!(mask.chunks, 2);
        assert_eq!(mask.padded, true);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);

        let mask = ByteMask::new(8).unwrap();
        assert_eq!(mask.bits, 8);
        assert_eq!(mask.mask, 0b11111111);
        assert_eq!(mask.chunks, 1);
        assert_eq!(mask.padded, false);
        assert_eq!(mask.byte, 0);
        assert_eq!(mask.step, 0);
    }

    #[test]
    fn test_set_byte() {
        let mut mask = ByteMask::new(2).unwrap();
        mask.step = 3;
        mask.set_byte(5);
        assert_eq!(mask.bits, 2);
        assert_eq!(mask.chunks, 4);
        assert_eq!(mask.padded, false);
        assert_eq!(mask.byte, 5);
        assert_eq!(mask.step, 0);
    }

    #[test]
    fn test_join_bits() {
        let expected = 0b10010011;

        let mask = ByteMask::new(1).unwrap();
        let bytes = mask.join_chunks(&vec![1, 0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(2).unwrap();
        let bytes = mask.join_chunks(&vec![0b10, 0b01, 0b00, 0b11]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(3).unwrap();
        let bytes = mask.join_chunks(&vec![0b100, 0b100, 0b011]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(4).unwrap();
        let bytes = mask.join_chunks(&vec![0b1001, 0b0011]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(5).unwrap();
        let bytes = mask.join_chunks(&vec![0b10010, 0b00011]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(6).unwrap();
        let bytes = mask.join_chunks(&vec![0b100100, 0b000011]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(7).unwrap();
        let bytes = mask.join_chunks(&vec![0b1001001, 0b00000001]);
        assert_eq!(expected, bytes);

        let mask = ByteMask::new(8).unwrap();
        let bytes = mask.join_chunks(&vec![0b10010011]);
        assert_eq!(expected, bytes);
    }

    #[test]
    fn test_iterator() {
        let mut mask = ByteMask::new(1).unwrap();
        let expected = vec![1, 0, 0, 1, 0, 0, 1, 1];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(2).unwrap();
        let expected = vec![0b10, 0b01, 0b00, 0b11];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(3).unwrap();
        let expected = vec![0b100, 0b100, 0b011];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(4).unwrap();
        let expected = vec![0b1001, 0b0011];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(5).unwrap();
        let expected = vec![0b10010, 0b00011];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(6).unwrap();
        let expected = vec![0b100100, 0b000011];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(7).unwrap();
        let expected = vec![0b1001001, 0b0000001];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);

        let mut mask = ByteMask::new(8).unwrap();
        let expected = vec![0b10010011];
        let result: Vec<u8> = mask.set_byte(0b10010011).collect();
        assert_eq!(expected, result);
    }
}
