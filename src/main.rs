use std::{
    fs::{read, File},
    io::{BufWriter, Write},
    path::Path,
};
use png::{Decoder, Encoder};
mod bu;
use bu::BitUtils;

fn decode(src: &str, dest: &str) {
    let decoder = Decoder::new(File::open(src).unwrap());

    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    let mut data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut data).unwrap();

    let (header, remainder) = data.split_at(8);
    let message_len = BitUtils::byte_to_decimal(BitUtils::read_lsb(header.to_vec()));

    let (message_bytes, _) = remainder.split_at(message_len as usize);
    let message_retrived = BitUtils::read_lsb(message_bytes.to_vec());

    let message = BitUtils::bits_to_bytes(message_retrived);

    let mut output_file = File::create(Path::new(dest)).unwrap();
    output_file.write_all(&message).unwrap();
}

fn encode(src: &str, msg_src: &str, dest: &str) {
    let message_bytes = read(msg_src).unwrap();
    let message_bits = BitUtils::make_bits(message_bytes);
    let message_size = BitUtils::byte_to_bit(message_bits.len() as u8);

    let mut complete_message = Vec::new();
    complete_message.extend_from_slice(&message_size);
    complete_message.extend_from_slice(&message_bits);

    let decoder = Decoder::new(File::open(src).unwrap());

    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    if complete_message.len() > reader.output_buffer_size() {
        eprintln!("Image is too small!");
        return;
    }

    let mut data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut data).unwrap();

    let info = reader.info();

    let mut i = 0;
    for bit in complete_message.iter() {
        if *bit == 1 && data[i] % 2 == 0 {
            data[i] += 1;
        } else if *bit == 0 && data[i] % 2 != 0 {
            data[i] -= 1;
        }
        i += 1;
    }

    let encoded_img = File::create(dest).unwrap();

    let mut image_encoder = Encoder::new(BufWriter::new(encoded_img), info.width, info.height);

    image_encoder.set_color(info.color_type);
    image_encoder.set_depth(info.bit_depth);

    image_encoder
        .write_header()
        .unwrap()
        .write_image_data(&data)
        .unwrap();
}

fn main() {
    let _ = encode(
        "test.png",
        "aea.txt",
        "test_out.png",
    );
    let _ = decode("test_out.png", "text.txt");
}
