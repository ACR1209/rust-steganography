pub struct BitUtils {}

impl BitUtils {
    // Transforms a decimal represented byte into its bit representation
    pub fn byte_to_bit(byte: u8) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();

        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            bits.push(bit);
        }

        bits
    }

    // Transforms a byte in its bit form into its decimal representation
    pub fn byte_to_decimal(byte: Vec<u8>) -> u8 {
        let mut output: u8 = 0;

        for i in 0..8 {
            if byte[i] == 1 {
                output += 2u8.pow(7 - i as u32) as u8;
            }
        }

        output
    }

    // Reads the least significant bit (LSB) from a byte array
    pub fn read_lsb(bytes: Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        for byte in bytes.iter() {
            output.push(byte % 2);
        }

        output
    }

    // Takes bits and transforms them into bytes
    pub fn bits_to_bytes(bits: Vec<u8>) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();

        for byte in bits.chunks(8) {
            if byte.len() == 8 {
                output.push(Self::byte_to_decimal(byte.to_vec()));
            }
        }

        output
    }

    // Takes bytes and transforms them into a bit array
    pub fn make_bits(bytes: Vec<u8>) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();

        for byte in bytes.iter() {
            bits.append(&mut Self::byte_to_bit(*byte));
        }

        bits
    }
}