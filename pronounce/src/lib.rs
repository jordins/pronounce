use std::io::Cursor;

pub fn play(text: &str, tl: &str) -> Result<(), Box<dyn std::error::Error>> {    
    // println!("Value for text: {}", text);
    // println!("Using lang: {}", tl);

    let base_url = "https://translate.google.com/translate_tts?ie=UTF-8";
    let full_url = format!("{}&q={}&tl={}&client=tw-ob", base_url, text, tl);

    let resp = reqwest::blocking::get(&full_url)?;
    let cursor = Cursor::new(resp.bytes().unwrap());
    let source = rodio::Decoder::new(cursor).unwrap();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.play();
    sink.sleep_until_end();
    Ok(())
}
