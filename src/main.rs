//! The DarkVM is a VM that can be targeted by any language.
//! It is written for the Envious programming language, but may targeted by any language.
//! The VM was created with zero-copy, speed, and simplicity in mind.
//! This means that the implementation of the VM uses reference counted types, resembling Swift.
//! Currently, the VM is highly experimental and may change, therefore writing programs in this language is not recommended.

use lexer::Lexer;
use std::fs;
use vm::VM;

/// The Utils module, which contains common utilities such as errors, tokens, and the stack.
mod utils;

/// The Lexer module, which creates a vector of all of the tokens in the input. This input may come from either a file or a REPL.
mod lexer;

/// The Values module, which contains the Value struct and ValueKind enum. These describe the various types within the program.
mod values;

/// The Code module, which maintains the different values generated by the lexer.
mod code;

/// The VM module. This maintains most of the code for the behavior of different instructions and the behavior of the VM in general.
mod vm;

/// A simple macro to convert a value kind to a reference counted value.
/// This is used in tests to easily create values to populate the VM.
#[macro_export]
macro_rules! value {
    ($kind: expr) => {
        Rc::new(Value::new(0, $kind))
    };
}

fn main() {
    // Fully reads the contents of the test.dark file.
    // This will change to accept a parameter (the path to the file) from the user.
    let contents = fs::read_to_string("src\\test.dark").unwrap();
    
    // Run the program by derefencing the String into a &str.
    match run(&contents) {
        Err(error) => println!("{}", error),
        Ok(_) => {},
    }
}

/// Runs the VM, and produces either an error, or the final state of the VM after the operations.
/// The errors produced can be found in the utils::error::ErrorKind enum.
fn run(contents: &str) -> Result<VM, String> {
    let tokens = Lexer::new().lex(contents).map_err(|error| error.prettify(contents))?;
    let mut vm = VM::new(tokens);
    let result = vm.run().map_err(|error| error.prettify(contents))?;
    if result.is_some() {
        println!("{:#?}\n", result);
    }

    Ok(vm)
}

#[test]
fn test_push_instrution() {
    use std::rc::Rc;
    use crate::values::values::{Value, ValueKind};

    let contents = "push 1";
    let result = run(contents);
    assert!(result.is_ok());
    
    let vm = result.unwrap();
    assert_eq!(vm.stack.len(), 1);
    assert!(vm.stack.contains(value!(ValueKind::Int(1))))
}

#[test]
fn test_pop_instrution() {
    use std::rc::Rc;
    use crate::values::values::{Value, ValueKind};

    let contents = "push 1\npop";
    let result = run(contents);
    assert!(result.is_ok());
    
    let vm = result.unwrap();
    assert_eq!(vm.stack.len(), 0);
    assert!(!vm.stack.contains(value!(ValueKind::Int(1))))
}