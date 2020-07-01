//! The DarkVM is a VM that can be targeted by any language.
//! It is written for the Envious programming language, but may targeted by any language.
//! The VM was created with zero-copy, speed, and simplicity in mind.
//! This means that the implementation of the VM uses reference counted values, resembling Swift.
//! Currently, the VM is highly experimental and may change, therefore writing programs in this language is not recommended.

/// The Tokens module, which contains the Token struct and the TokenKind enum. These describe the various tokens that can be recognized.
pub mod tokens;

/// The Errors module, which contains the Error struct and the ErrorKind enum. These describe the various errors that could occur during the program execution.
pub mod errors;

/// The Utils module, which contains common utilities such as the stack and frames.
pub mod utils;

/// The Lexer module, which creates a vector of all of the tokens in the input. This input may come from either a file or a REPL.
pub mod lexer;

/// The Values module, which contains the Value struct and ValueKind enum. These describe the various values within the program.
pub mod values;

/// The Code module, which maintains the different values generated by the lexer.
pub mod code;

/// The VM module. This maintains most of the code for the behavior of different instructions and the behavior of the VM in general.
pub mod vm;

use lexer::Lexer;
use std::fs;
use vm::VM;

fn main() {
    // Fully reads the contents of the test.dark file.
    // This will change to accept a parameter (the path to the file) from the user.
    let contents = fs::read_to_string("src\\test.dark").unwrap();

    // Run the program by derefencing the String into a &str.
    match run(&contents) {
        Err(error) => println!("{}", error),
        Ok(vm) => println!("{:#?}", vm),
    }
}

/// Runs the VM, and produces either an error, or the final state of the VM after the operations.
/// The errors produced can be found in the utils::error::ErrorKind enum.
fn run(contents: &str) -> Result<VM, String> {
    let tokens = Lexer::default()
        .lex(contents)
        .map_err(|error| error.prettify(contents))?;
    let mut vm = VM::new(tokens).map_err(|error| error.prettify(contents))?;
    let result = vm.run().map_err(|error| error.prettify(contents))?;
    if result.is_some() {
        println!("{:#?}\n", result);
    }

    Ok(vm)
}
