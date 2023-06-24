use clap::{App, Arg};

fn main() {
       let matches = App::new("echor")
        .version("0.1.1")
        .author("Suhird singh <suhirdsingh92@gmail.com>")
        .about("echo in Rust")
           .arg(
                  Arg::with_name("text")
                      .value_name("TEXT")
                      .help("Input text")
                      .required(true)
                      .min_values(1),
           )
           .arg(
               Arg::with_name("omit_newline")
                   .short("n")
                   .help("Do not print newline")
                   .takes_value(false)
           )
        .get_matches();
    println!("{:#?}", matches);
}
