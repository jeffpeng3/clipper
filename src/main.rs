use anyhow::Result;
use std::io::{self, Read, Cursor};
use arboard::{Clipboard, ImageData};
use image::ImageReader;

fn main()  -> Result<()> {
    let mut buf: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let mut ctx = Clipboard::new()?;
    let img = ImageReader::new(Cursor::new(buf)).with_guessed_format()?.decode()?.into_rgba8();
    let (w,h) = img.dimensions();
    let img_data = ImageData{ width: w as usize, height: h as usize, bytes: img.into_raw().into() };
    ctx.set_image(img_data)?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    Ok(())
}