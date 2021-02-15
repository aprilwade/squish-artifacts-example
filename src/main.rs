mod libsquish;

use std::fs::File;

use image::ImageDecoder;
use image::codecs::png::{PngDecoder, PngEncoder};

// fn compress(pixels: &[u8], width: usize, height: usize, compressed_bytes: &mut [u8])
// {
//     for (i, block) in compressed_bytes.chunks_mut(8).enumerate() {

//     }
// }

fn main() {
    let input_file = File::open("./original.png").unwrap();

    let decoder = PngDecoder::new(&input_file).unwrap();
    let (w, h) = decoder.dimensions();
    let (w, h) = (w as usize, h as usize);
    assert!(decoder.color_type() == image::ColorType::Rgba8);
    let mut uncompressed = vec![0u8; 4 * w * h];
    decoder.read_image(&mut uncompressed[..]).unwrap();

    let fmt = squish::Format::Bc1;
    {
        let mut compressed = vec![0u8; w * h / 2];
        unsafe {
            libsquish::compress_dxt1_image(&uncompressed[..], w, h, &mut compressed[..]);
        }

        let mut decompressed = vec![0u8; 4 * w * h];
        fmt.decompress(&compressed[..], w, h, &mut decompressed[..]);
        // unsafe {
        //     libsquish::decompress_dxt1_image(&mut decompressed[..], w, h, &compressed[..]);
        // }

        let output_file = File::create("./libsquish_recompressed.png").unwrap();
        let encoder = PngEncoder::new(output_file);
        encoder.encode(&decompressed[..], w as u32, h as u32, image::ColorType::Rgba8).unwrap();
    }

    {
        let mut compressed = vec![0u8; w * h / 2];
        let params = squish::Params {
            algorithm: squish::Algorithm::IterativeClusterFit,
            ..squish::Params::default()
        };
        fmt.compress(&uncompressed[..], w, h, params, &mut compressed[..]);

        let mut decompressed = vec![0u8; 4 * w * h];
        fmt.decompress(&compressed[..], w, h, &mut decompressed[..]);
        // unsafe {
        //     libsquish::decompress_dxt1_image(&mut decompressed[..], w, h, &compressed[..]);
        // }

        let output_file = File::create("./squish-rs_recompressed.png").unwrap();
        let encoder = PngEncoder::new(output_file);
        encoder.encode(&decompressed[..], w as u32, h as u32, image::ColorType::Rgba8).unwrap();
    }
}
