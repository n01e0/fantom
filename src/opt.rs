use fanotify::high_level::{FanEvent, Fanotify, FanotifyMode};
use std::io::ErrorKind;
use std::path;

#[derive(Debug, Clone)]
pub enum TargetPath {
    Path(String),
    Mountpoint(String),
}

#[derive(Debug)]
pub struct FantomOptions {
    pub target_paths: Vec<TargetPath>,
    pub monitor_events: Vec<FanEvent>,
    pub blocking: bool,
}

#[derive(Debug)]
pub enum FanEventParseError {
    ParseError,
    NotSupport,
}

fn parse_fanevent<T: Into<String>>(s: T) -> Result<FanEvent, FanEventParseError> {
    match &s.into()[..] {
        "access" => Ok(FanEvent::Access),
        "attrib" => Ok(FanEvent::Attrib),
        "close" => Ok(FanEvent::Close),
        "close_nowrite" => Ok(FanEvent::CloseNowrite),
        "close_write" => Ok(FanEvent::CloseWrite),
        "create" => Ok(FanEvent::Create),
        "delete" => Ok(FanEvent::Delete),
        "delete_self" => Ok(FanEvent::DeleteSelf),
        "modify" => Ok(FanEvent::Modify),
        "move" => Ok(FanEvent::Move),
        "moved_from" => Ok(FanEvent::MovedFrom),
        "moved_to" => Ok(FanEvent::MovedTo),
        "move_self" => Ok(FanEvent::MoveSelf),
        "open" => Ok(FanEvent::Open),
        "open_exec" => Ok(FanEvent::OpenExec),
        "open_perm" => Err(FanEventParseError::NotSupport),
        "open_exec_perm" => Err(FanEventParseError::NotSupport),
        "access_perm" => Err(FanEventParseError::NotSupport),
        _ => Err(FanEventParseError::ParseError),
    }
}

impl FantomOptions {
    pub fn from(args: &clap::ArgMatches) -> Self {
        let mut paths = args
            .values_of_lossy("target_path")
            .unwrap_or(Vec::new())
            .iter()
            .map(|p| {
                path::Path::new(p).canonicalize().unwrap_or_else(|e| {
                    eprintln!("Error: \"{}\" {}", p, e);
                    std::process::exit(1);
                })
            })
            .map(|p| TargetPath::Path(format!("{}/", p.to_str().unwrap_or(""))))
            .collect::<Vec<TargetPath>>();

        let mut mountpoints = args
            .values_of_lossy("target_mountpoint")
            .unwrap_or(Vec::new())
            .iter()
            .map(|p| {
                path::Path::new(p).canonicalize().unwrap_or_else(|e| {
                    eprintln!("Error: \"{}\" {}", p, e);
                    std::process::exit(1);
                })
            })
            .map(|p| TargetPath::Mountpoint(format!("{}/", p.to_str().unwrap_or(""))))
            .collect::<Vec<TargetPath>>();
        paths.append(&mut mountpoints);

        FantomOptions {
            target_paths: paths,
            monitor_events: args
                .values_of_lossy("monitor_events")
                .unwrap()
                .iter()
                .map(|e| parse_fanevent(e).unwrap())
                .collect::<Vec<FanEvent>>(),
            blocking: args.is_present("blocking"),
        }
    }

    fn get_mode(&self) -> u64 {
        self.monitor_events
            .iter()
            .fold(0, |flag, x| flag | *x as u64)
    }

    fn create_fanotify(&self) -> Fanotify {
        if self.blocking {
            Fanotify::new_with_blocking(FanotifyMode::CONTENT)
        } else {
            Fanotify::new_with_nonblocking(FanotifyMode::CONTENT)
        }
    }

    fn add_target(&self, fan: &Fanotify, mode: u64) -> Result<(), std::io::Error> {
        for path in &self.target_paths {
            match &path {
                TargetPath::Path(path) => {
                    if let Err(e) = fan.add_path(mode, path) {
                        match e.kind() {
                            ErrorKind::NotFound => {
                                println!("{}: {}", e, path);
                                return Err(e.into());
                            }
                            e => return Err(e.into()),
                        }
                    }
                }
                TargetPath::Mountpoint(mountpoint) => {
                    if let Err(e) = fan.add_mountpoint(mode, mountpoint) {
                        match e.kind() {
                            ErrorKind::NotFound => {
                                println!("{}: {}", e, mountpoint);
                                return Err(e.into());
                            }
                            e => return Err(e.into()),
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn apply(&self) -> Result<Fanotify, std::io::Error> {
        let fan = self.create_fanotify();
        let mode = self.get_mode();

        self.add_target(&fan, mode)?;
        Ok(fan)
    }
}
