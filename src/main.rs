#[macro_use]
extern crate clap;

mod opt;

use std::io::ErrorKind;

fn main() {
    let yml = load_yaml!("opt.yml");
    let args = clap::App::from_yaml(yml).get_matches();

    let options = opt::FantomOptions::from(&args);
    match options.apply() {
        Ok(fan) => {
            println!("Ok: fan => {}", fan.as_raw_fd());
        }
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => eprintln!("You need CAP_SYS_ADMIN capability"),
            e => eprintln!("Error: {:?}", e),
        },
    }
}
