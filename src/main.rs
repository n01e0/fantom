#[macro_use]
extern crate clap;

mod opt;

use nix::poll::{poll, PollFd, PollFlags};
use std::io::ErrorKind;

fn main() {
    let yml = load_yaml!("opt.yml");
    let args = clap::App::from_yaml(yml).get_matches();

    let options = opt::FantomOptions::from(&args);
    for path in &options.target_paths {
        println!("target: {:?}", path);
    }
    match options.apply() {
        Ok(fan) => {
            let mut fds = [PollFd::new(fan.as_raw_fd(), PollFlags::POLLIN)];
            loop {
                let poll_num = poll(&mut fds, -1).unwrap();
                if poll_num > 0 {
                    for event in fan.read_event() {
                        println!("{:#?}", event);
                    }
                } else {
                    eprintln!("poll_num <= 0");
                    break;
                }
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => eprintln!("You need CAP_SYS_ADMIN capability"),
            e => eprintln!("Error: {:?}", e),
        },
    }
}
