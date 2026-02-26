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

    let width = image.width as usize;
    let height = image.height as usize;
    println!("CFA: {:?}", image.cfa);

    let mut f = BufWriter::new(File::create(format!("{}.ppm", file)).unwrap());

    if let rawloader::RawImageData::Integer(data) = image.data {
        let max_value = *data.iter().max().unwrap() as f32;

        // 2x2まとめるので解像度半分
        let out_width = width / 2;
        let out_height = height / 2;

        write!(f, "P6 {} {} 255\n", out_width, out_height).unwrap();

        // RGGBの単純なデモザイク
        for y in (0..height - 1).step_by(2) {
            for x in (0..width - 1).step_by(2) {
                let i = y * width + x;

                let r = data[i] as f32;

                let g1 = data[i + 1] as f32;
                let g2 = data[i + width] as f32;
                let g = (g1 + g2) * 0.5;

                let b = data[i + width + 1] as f32;

                // 正規化
                let mut r = r / max_value;
                let mut g = g / max_value;
                let mut b = b / max_value;

                // ガンマ補正
                r = r.powf(1.0 / 2.2);
                g = g.powf(1.0 / 2.2);
                b = b.powf(1.0 / 2.2);

                // 8bit化
                let r8 = (r * 255.0).clamp(0.0, 255.0) as u8;
                let g8 = (g * 255.0).clamp(0.0, 255.0) as u8;
                let b8 = (b * 255.0).clamp(0.0, 255.0) as u8;

                f.write_all(&[r8, g8, b8]).unwrap();
            }
        }
    }
}
