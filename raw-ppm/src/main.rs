use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let file = &args[1];
    let image = rawloader::decode_file(file).unwrap();

    let mut f = BufWriter::new(File::create(format!("{}.ppm", file)).unwrap());

    let preamble = format!("P6 {} {} 65535\n", image.width, image.height);
    f.write_all(preamble.as_bytes()).unwrap();

    if let rawloader::RawImageData::Integer(data) = image.data {
        for pix in data {
            let high = (pix >> 8) as u8;
            let low = (pix & 0xff) as u8;
            f.write_all(&[high, low, high, low, high, low]).unwrap();
        }
    } else {
        eprintln!("Unsupported raw data type");
    }
}
