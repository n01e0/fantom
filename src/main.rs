#[macro_use]
extern crate clap;

mod opts;

fn main() {
    let yml = load_yaml!("opts.yml");
    let args = clap::App::from_yaml(yml).get_matches();

    let options = opts::parse_options(&args);
    println!("{:#?}", options);
}
