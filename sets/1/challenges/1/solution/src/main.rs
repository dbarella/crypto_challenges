// Convert hex strings to base64 strings.

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Hex-to-Base64 Translator")
        .version("1.0")
        .author("Dan B.")
        .about("Converts hex strings to base64 strings.")
        .arg(Arg::with_name("hex")
            .help("Hex string to translate."))
        .get_matches();

    if let Some(hex) = matches.value_of("hex") {
      println!("{}", hex);
    }
}
