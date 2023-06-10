pub struct BitUtils {}

impl BitUtils {
    // Transforms a decimal represented byte into its bit representation
    pub fn byte_to_bit(byte: u8) -> Vec<u8> {
        (0..8).rev().map(|i| (byte >> i) & 1).collect()
    }

    // Transforms a decimal represented 4 byte into its bits representation
    pub fn byte_u32_to_bit(byte: u32) -> Vec<u8> {
        let mut bits: Vec<u8> = Vec::new();

        for i in (0..32).rev() {
            let bit = (byte >> i) & 1;
            bits.push(bit as u8);
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

    // Transforms 4 bytes in its bit form into its decimal representation
    pub fn byte_u32_to_decimal(byte: Vec<u8>) -> u32 {
        byte.iter()
            .enumerate()
            .filter(|(_, &bit)| bit == 1)
            .fold(0u32, |acc, (i, _)| acc + 2u32.pow(31 - i as u32))
    }

    // Reads the least significant bit (LSB) from a byte array
    pub fn read_lsb(bytes: Vec<u8>) -> Vec<u8> {
        bytes
            .iter()
            .map(|byte| byte % 2)
            .collect()
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
        bytes
            .iter()
            .flat_map(|byte| Self::byte_to_bit(*byte))
            .collect()
    }
}
