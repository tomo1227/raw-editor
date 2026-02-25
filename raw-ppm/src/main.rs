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
    let preamble = format!("P6 {} {} 255\n", image.width, image.height).into_bytes();
    f.write_all(&preamble).unwrap();
    if let rawloader::RawImageData::Integer(data) = image.data {
        let max_val = *data.iter().max().unwrap() as f32;
        for pix in data {
            // 0.0~1.0 に正規化して ガンマ補正
            let mut v = pix as f32 / max_val;
            v = v.powf(1.0 / 2.2);
            let pix8 = (v * 255.0).clamp(0.0, 255.0) as u8;
            f.write_all(&[pix8, pix8, pix8]).unwrap();
        }
    }
}
