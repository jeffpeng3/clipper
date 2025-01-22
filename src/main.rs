use wl_clipboard_rs::copy::{MimeType, Options, Source};


fn main() -> Result<(), anyhow::Error>{
    let opts = Options::new();
    let r = opts.copy(Source::StdIn, MimeType::Autodetect);

    match r {
        Ok(r) => println!("copied {:?}", r),
        Err(e) => println!("error: {:?}", e),
    }

    std::thread::sleep(std::time::Duration::from_millis(10));
    Ok(())
}
