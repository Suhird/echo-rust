use clap::App;

fn main() {
       let _match = App::new("echor")
        .version("0.1.1")
        .author("Suhird singh <suhirdsingh92@gmail.com>")
        .about("echo in Rust")
        .get_matches();
}
