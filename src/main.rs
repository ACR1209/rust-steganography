use std::{
    fs::{read, File},
    io::{BufWriter, Write},
    path::Path,
};
use png::{Decoder, Encoder};
mod bu;
use bu::BitUtils;
use clap::Parser;


macro_rules! info {
    ($($arg:tt)*) => {{
        println!("[INFO] {}", format_args!($($arg)*));
    }};
}
macro_rules! success {
    ($($arg:tt)*) => {{
        println!("[SUCCESS] {}", format_args!($($arg)*));
    }};
}
macro_rules! error {
    ($($arg:tt)*) => {{
        eprintln!("[ERROR] {}", format_args!($($arg)*));
    }};
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// If is to read or to hide
    #[arg(short, long)]
    option: String,

    /// The path to the file to read
    #[arg(short, long)]
    file: Option<String>,

    /// The path to the image to use
    #[arg(short, long)]
    image: String,

    /// The path to output the file
    #[arg(long)]
    output: String,
}

fn decode(src: &str, dest: &str) {
    info!("Getting image data");
    let decoder = Decoder::new(File::open(src).unwrap());

    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    let mut data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut data).unwrap();

    let (message_len, image_data) = data.split_at(32);
    let message_len = BitUtils::byte_u32_to_decimal(BitUtils::read_lsb(message_len.to_vec()));
    info!("Message size of {} bits", &message_len);
    let (bytes_message, _): (&[u8], &[u8]) = image_data.split_at(message_len as usize);
    let message_bits = BitUtils::read_lsb(bytes_message.to_vec());

    let message_retrived = BitUtils::bits_to_bytes(message_bits);

    let mut output_file = File::create(Path::new(dest)).unwrap();
    
    info!("Writing message found to file {}", &dest);
    output_file.write_all(&message_retrived).unwrap();

    success!("Succesfully retrived message to {}", &dest);
}

fn encode(src: &str, msg_src: &str, dest: &str) {
    info!("Transforming message to bytes");
    let message_bytes = read(msg_src).unwrap();
    let message_bits = BitUtils::make_bits(message_bytes);
    let message_size = BitUtils::byte_u32_to_bit(message_bits.len() as u32);
    info!("Message size {} bits", message_bits.len());
    info!("Embedding message size to message header");
    let mut complete_message = Vec::new();
    complete_message.extend_from_slice(&message_size);
    complete_message.extend_from_slice(&message_bits);

    info!("Opening image {}", &src);
    let decoder = Decoder::new(File::open(src).unwrap());

    let mut binding = decoder.read_info();
    let reader = binding.as_mut().unwrap();

    info!("Image capacity: {}", reader.output_buffer_size());
    if complete_message.len() > reader.output_buffer_size() {
        error!("Image is too small: message size is {} and image allows for {}", complete_message.len(), reader.output_buffer_size());
        return;
    }

    let mut data = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut data).unwrap();

    let info = reader.info();

    info!("Saving information in the LSB");
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
    info!("Saving generated image to {}", &dest);
    image_encoder
        .write_header()
        .unwrap()
        .write_image_data(&data)
        .unwrap();
    success!("Succesfully saved message on image {}", &dest);
}

fn main() {
    let args = Cli::parse();

    match args.option.as_str() {
        "read"=>{
            info!("Starting to read file {}", &args.image);
            decode(&args.image, &args.output)
        },
        "write"=>{
            match args.file {
                Some(file)=> {
                    info!("Starting to write file {}", &args.output);
                    encode(&args.image, &file, &args.output)
                },
                None=>eprintln!("ERROR: File not passed!")
            }
        },
        _ => panic!("No valid option given: please try to use --help to see the valid options"),
    }
}
