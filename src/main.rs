use std::io::Cursor;
extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Pronounce")
        .version("0.1.0")
        .author("jordins <jordi.nistal@gmail.com>")
        .about("Pronounces the passed text in the desired language")
        .arg(
            Arg::with_name("text")
                .index(1)
                .help("The text you want to pronounce")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LANG")
                .help("Sets the language to pronounce")
                .default_value("en")
                .index(2),
        )
        .get_matches();

    let text = matches.value_of("text").unwrap();
    println!("Value for text: {}", text);
    let tl = matches.value_of("LANG").unwrap();
    println!("Using lang: {}", tl);

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
