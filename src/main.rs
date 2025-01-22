use anyhow::Result;
use std::io::{self, Read, Cursor};
use arboard::{Clipboard, ImageData};
use image::ImageReader;

fn main()  -> Result<()> {
    let mut buf: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    println!("Read {} bytes", buf.len());
    let mut ctx = Clipboard::new()?;
    let img = ImageReader::new(Cursor::new(buf)).with_guessed_format()?.decode()?.into_rgba8();
    println!("Image: {}x{}", img.width(), img.height());
    let (w,h) = img.dimensions();
    let img_data = ImageData{ width: w as usize, height: h as usize, bytes: img.into_raw().into() };
    println!("Setting image data");
    ctx.set_image(img_data)?;
    let r = ctx.get_image()?;
    println!("Read image data: {}x{}", r.width, r.height);
    Ok(())
}