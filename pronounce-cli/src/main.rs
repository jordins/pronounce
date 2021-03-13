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

    pronounce::play(text, tl)
}
