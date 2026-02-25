use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(2);
    }
    let file = &args[1];
    let image = rawloader::decode_file(file).unwrap();

    // 画像をグレースケールのPPMとして書き出す
    let mut f = BufWriter::new(File::create(format!("{}.ppm", file)).unwrap());
    let preamble = format!("P6 {} {} {}\n", image.width, image.height, 65535).into_bytes();
    f.write_all(&preamble).unwrap();
    if let rawloader::RawImageData::Integer(data) = image.data {
        for pix in data {
            // R=G=Bとし簡単なデモザイクを行う
            let pixhigh = (pix >> 8) as u8;
            let pixlow = (pix & 0x0f) as u8;
            f.write_all(&[pixhigh, pixlow, pixhigh, pixlow, pixhigh, pixlow])
                .unwrap()
        }
    } else {
        eprintln!("Don't know how to process non-integer raw files");
    }
}
