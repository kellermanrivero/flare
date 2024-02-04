use std::io::BufWriter;

use libwebp_sys::WebPEncodeRGBA;
use png::Compression;

pub struct Encoders();

pub trait Encoder {
    fn encode(&self, input_image: &[u8], width: u32, height: u32, stride: u32, bpp: u32, quality: u32) -> Vec<u8>;
}

struct WebPEncoder;

struct PNGEncoder;

struct JPEGEncoder;

impl Encoder for WebPEncoder {
    fn encode(&self, input_image: &[u8], width: u32, height: u32, stride: u32, _: u32, quality: u32) -> Vec<u8> {
        unsafe {
            let mut out_buf = std::ptr::null_mut();
            let len = WebPEncodeRGBA(input_image.as_ptr(), width as i32, height as i32, stride as i32, quality as f32, &mut out_buf);
            std::slice::from_raw_parts(out_buf, len).into()
        }
    }
}

impl Encoder for PNGEncoder {
    fn encode(&self, input_image: &[u8], width: u32, height: u32, _: u32, bpp: u32, _: u32) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let writer = BufWriter::new(&mut buffer);
        let mut encoder = png::Encoder::new(writer, width, height);
        if bpp == 32 {
            encoder.set_color(png::ColorType::Rgba);
        } else if bpp == 8 {
            encoder.set_color(png::ColorType::Grayscale);
        }
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(Compression::Best);

        let mut writer = encoder.write_header().unwrap(); // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        writer.write_image_data(input_image).unwrap();
        writer.finish().expect("Cannot create PNG file");
        buffer
    }
}


impl Encoder for JPEGEncoder {
    fn encode(&self, input_image: &[u8], width: u32, height: u32, _: u32, bpp: u32, quality: u32) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        let writer = BufWriter::new(&mut buffer);
        let mut encoder = jpeg_encoder::Encoder::new(writer, quality as u8);

        let mut color_type = jpeg_encoder::ColorType::Rgba;
        if bpp == 8 {
            color_type = jpeg_encoder::ColorType::Luma;
        }
        encoder.encode(&input_image, width as u16, height as u16, color_type).expect("Cannot encode JPEG image");

        buffer
    }
}

impl Encoders {
    pub fn webp() -> Box<dyn Encoder> {
        Box::new(WebPEncoder {})
    }

    pub fn png() -> Box<dyn Encoder> {
        Box::new(PNGEncoder {})
    }

    pub fn jpeg() -> Box<dyn Encoder> {
        Box::new(JPEGEncoder {})
    }
}




