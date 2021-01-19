#[derive(Debug)]
pub enum TargetPath {
    Path(String),
    Mountpoint(String),
}

pub use fanotify::high_level::{FanEvent, FanotifyMode};

#[derive(Debug)]
pub struct FantomOptions {
    pub target_path: Vec<TargetPath>,
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

pub fn parse_options(args: &clap::ArgMatches) -> FantomOptions {
    FantomOptions {
        target_path: if args.is_present("target_path") {
            args.values_of_lossy("target_path")
                .unwrap()
                .iter()
                .map(|p| TargetPath::Path(p.to_string()))
                .collect::<Vec<TargetPath>>()
        } else {
            args.values_of_lossy("target_mountpoint")
                .unwrap()
                .iter()
                .map(|p| TargetPath::Mountpoint(p.to_string()))
                .collect::<Vec<TargetPath>>()
        },
        monitor_events: args
            .values_of_lossy("monitor_events")
            .unwrap()
            .iter()
            .map(|e| parse_fanevent(e).unwrap())
            .collect::<Vec<FanEvent>>(),
        blocking: args.is_present("blocking"),
    }
}
