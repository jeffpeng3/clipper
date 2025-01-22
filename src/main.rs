use wl_clipboard_rs::copy::{MimeType, Options, Source};


fn main() -> Result<(), anyhow::Error>{
    let s = "123";
    Options::new().copy(Source::Bytes(s.as_bytes().into()), MimeType::Autodetect)?;
    Ok(())
}
