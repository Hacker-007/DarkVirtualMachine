use lexer::Lexer;
use std::fs;
use vm::VM;

mod code;
mod lexer;
mod utils;
mod values;
mod vm;

fn main() {
    let contents = fs::read_to_string("src\\test.dark").unwrap();
    let contents = contents.replace('\r', "");
    match Lexer::new().lex(contents.as_str()) {
        Ok(tokens) => {
            let mut vm = VM::new(tokens);
            match vm.run() {
                Ok(result) => {
                    if result.is_some() {
                        println!("Result From Final Operation: {:#?}\n", result.unwrap());
                    }

                    println!("{:#?}", vm);
                }
                Err(error) => println!("{}", error.prettify(contents.as_str())),
            }
        }
        Err(error) => println!("{}", error.prettify(contents.as_str())),
    }
}
