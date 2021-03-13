use std::io::Cursor;

use rodio::Source;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello%20jordi";
    let tl = "en";
    // let url = "http://httpbin.org/range/26";
    let base_url = "https://translate.google.com/translate_tts?ie=UTF-8";
    let full_url = format!("{}&q={}&tl={}&client=tw-ob", base_url, text, tl);
    // println!("url {:?}", full_url);

    let resp = reqwest::blocking::get(&full_url)?;
    let cursor = Cursor::new(resp.bytes().unwrap());
    let source = rodio::Decoder::new(cursor).unwrap();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    loop {}
    // Ok(())
}
