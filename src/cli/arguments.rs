use crate::errors::{error::Error, error_kind::ErrorKind};
use std::env;

pub struct Arguments {
    path: Option<String>,
    show_time: bool,
    show_machine: bool,
}

impl Arguments {
    pub fn new() -> Result<Arguments, Error> {
        let args = env::args().skip(1);
        let mut arguments = Arguments {
            path: None,
            show_time: false,
            show_machine: false,
        };

        for arg in args {
            match arg.as_str() {
                "-t" |"--show-time" => arguments.show_time = true,
                "-m" | "--show-machine" => arguments.show_machine = true,
                _ if arguments.path.is_none() => arguments.path = Some(arg),
                _ => return Err(Error::message_only(ErrorKind::UnrecognizedArgument(arg))),
            }
        }

        Ok(arguments)
    }

    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn show_machine(&self) -> bool {
        self.show_machine
    }

    pub fn show_time(&self) -> bool {
        self.show_time
    }
}