# fantom
```
@@@@@@@@   @@@@@@   @@@  @@@  @@@@@@@   @@@@@@   @@@@@@@@@@
@@@@@@@@  @@@@@@@@  @@@@ @@@  @@@@@@@  @@@@@@@@  @@@@@@@@@@@
@@!       @@!  @@@  @@!@!@@@    @@!    @@!  @@@  @@! @@! @@!
!@!       !@!  @!@  !@!!@!@!    !@!    !@!  @!@  !@! !@! !@!
@!!!:!    @!@!@!@!  @!@ !!@!    @!!    @!@  !@!  @!! !!@ @!@
!!!!!:    !!!@!!!!  !@!  !!!    !!!    !@!  !!!  !@!   ! !@!
!!:       !!:  !!!  !!:  !!!    !!:    !!:  !!!  !!:     !!:
:!:       :!:  !:!  :!:  !:!    :!:    :!:  !:!  :!:     :!:
 ::       ::   :::   ::   ::     ::    ::::: ::  :::     ::
 :         :   : :  ::    :      :      : :  :    :      :
```

fanotify to monitor.

## Usage
```
    fantom [FLAGS] [OPTIONS] --events <monitor_events>... <--path <target_path>...|--mountpoint <target_mountpoint>...>

FLAGS:
    -b, --blocking    without FAN_NONBLOCK flag
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -e, --events <monitor_events>...           monitor event list [possible values: access, attrib, close,
                                               close_nowrite, close_write, create, delete, delte_self, modify, move,
                                               moved_from, moved_to, move_self, open, open_exec]
    -m, --mountpoint <target_mountpoint>...    target mountpoint
    -p, --path <target_path>...                target path
```
