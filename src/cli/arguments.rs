use crate::errors::{error::Error, error_kind::ErrorKind};
use std::{env, fs, time::Instant};

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

    pub fn run<F: Fn(&str) -> Result<String, String>>(&self, f: F) -> Result<(), String> {
        if self.path.is_none() {
            generate_error("Expected The Path To The Dark File.")
        } else if self
            .path
            .as_ref()
            .filter(|path| path.ends_with(".dark"))
            .is_some()
        {
            let contents = fs::read_to_string(self.path.as_ref().unwrap())
                .map_err(|_| "An Error Occurred.\nThe Path Provided Is Not Valid.".to_owned())?;
            let start = Instant::now();
            match f(&contents) {
                Ok(vm) if self.show_machine => println!("{}", vm),
                Ok(_) => {},
                Err(error) => return Err(error),
            }
            
            if self.show_time {
                println!("Time Taken: {:#?}", start.elapsed())
            }

            Ok(())
        } else {
            generate_error("Expected The File Passed In To Be An Dark File.")
        }
    }
}

fn generate_error(error_message: &str) -> Result<(), String> {
    Err(format!("An Error Occurred.\n{}", error_message))
}
