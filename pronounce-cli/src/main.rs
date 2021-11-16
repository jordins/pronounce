extern crate clap;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let available_languages = [
        "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "ny",
        "zh-CN", "co", "hr", "cs", "da", "nl", "en", "eo", "et", "tl", "fi", "fr", "fy", "gl",
        "ka", "de", "el", "gu", "ht", "ha", "haw", "iw", "hi", "hmn", "hu", "is", "ig", "id", "ga",
        "it", "ja", "jw", "kn", "kk", "km", "rw", "ko", "ku", "ky", "lo", "la", "lv", "lt", "lb",
        "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn", "my", "ne", "no", "or", "ps", "fa", "pl",
        "pt", "pa", "ro", "ru", "sm", "gd", "sr", "st", "sn", "sd", "si", "sk", "sl", "so", "es",
        "su", "sw", "sv", "tg", "ta", "tt", "te", "th", "tr", "tk", "uk", "ur", "ug", "uz", "vi",
        "cy", "xh", "yi", "yo", "zu",
    ];
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
    let tl = matches.value_of("LANG").unwrap();

    if text.is_empty() {
        println!("Text is empty");
        println!("Using lang: {}", tl);
        Ok(())
    } else if !available_languages.contains(&tl) {
        println!("Value for text: {}", text);
        println!("Using lang: {}", tl);
        println!("Language invalid, please input the right language code");
        Ok(())
    } else {
        println!("Value for text: {}", text);
        println!("Using lang: {}", tl);
        pronounce::play(text, tl)
    }
}
