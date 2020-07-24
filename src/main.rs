/// The Arguments module, which holds all of the arguments to the program.
pub mod arguments;

use dark_vm::run;
use arguments::Arguments;
use std::{fs, time::Instant};

fn main() {
    if let Err(error) = runner() {
        println!("{}", error)
    }
}

fn runner() -> Result<(), String> {
    let args = Arguments::new().map_err(|error| error.prettify(""))?;
    if args.get_path().is_none() {
        generate_error("The REPL Is Not Yet Supported.")
    } else if let Some(path) = args
        .get_path()
        .filter(|path| path.ends_with(".dark"))
    {
        let contents = fs::read_to_string(path)
            .map_err(|_| "An Error Occurred.\nThe Path Provided Is Not Valid.".to_owned())?;
        let start = Instant::now();
        match run(&contents) {
            Ok(vm) if args.show_machine() => println!("{}", vm),
            Ok(_) => {},
            Err(error) => return Err(error)
        }
        
        if args.show_time() {
            println!("Time Taken: {:#?}", start.elapsed())
        }

        Ok(())
    } else {
        generate_error("Expected The File Passed In To Be An Dark File.")
    }
}

fn generate_error(error_message: &str) -> Result<(), String> {
    Err(format!("An Error Occurred.\n{}", error_message))
}