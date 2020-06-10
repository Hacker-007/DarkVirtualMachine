//! The Error struct maintains the errors that occur during execution.

use crate::values::values::ValueKind;

pub struct Error {
    kind: ErrorKind,
    position: usize,
}

impl Error {
    /// Constructs a new error with the error kind and the position.
    ///
    /// # Arguments
    /// `kind` - The type of the error. Maintaining the type allows for the messages to be controlled across execution.
    /// `position` - The position where the error occurred.
    pub fn new(kind: ErrorKind, position: usize) -> Error {
        Error { kind, position }
    }

    /// This function generates a pretty version of the error, with arrows pointing to the exact location of the error.
    /// This function also consumes the error, therefore, it should be the last thing called.
    ///
    /// # Arguments
    /// `input` - The input for the program. This is not maintained with every error because the input might be different.
    pub fn prettify(self, input: &str) -> String {
        // Get the line and column number of where the error occurred.
        let (line_number, column_number) = self.get_line_column_info(input);
        
        // Check if a line is present. If not, the error is printed without the arrows.
        // This should usually produce a line, but it may not.
        let option_line = input.split_terminator('\n').nth(line_number - 1);
        
        // Convert the kind into an error message.
        let error_message: String = self.kind.into();
        if let Some(line) = option_line {
            let len = line_number.to_string().len();
            format!(
                "{} |\n{} | {}\n{} | {}^-- {}\n",
                " ".repeat(len),
                line_number,
                line,
                " ".repeat(len),
                " ".repeat(column_number - 1),
                error_message,
            )
        } else {
            format!(
                "Error Occurred At Line {}, Column Number {}.\n{}",
                line_number, column_number, error_message,
            )
        }
    }

    /// This function gets the line and column number of where the error occurred with respect to the input.
    fn get_line_column_info(&self, input: &str) -> (usize, usize) {
        let (mut line_number, mut column_number) = (1, 0);
        
        // Go through the characters and find the index that matches the position given in the error struct.
        input.chars().enumerate().find(|(idx, ch)| {
            if ch == &'\n' {
                line_number += 1;
                column_number = 0;
            } else {
                column_number += 1;
            }

            if idx == &(self.position - 1) {
                true
            } else {
                false
            }
        });

        (line_number, column_number)
    }
}

/// The ErrorKind enum maintains the different errors that can occur during the execution of the program.
/// This allows for uniformity across the various errors because the error messages are the same.
/// This also increases readibility within the code, because the ErrorKind's are more descriptive.

pub enum ErrorKind {
    UnknownCharacter,
    InvalidNumberFormat,
    UnterminatedString,
    EmptyStack,
    ExpectedArgs(usize),
    TypeMismatch(ValueKind, ValueKind),
}

/// Converts the ErrorKind into a String.
/// This is used in the prettify method to produce the error messages needed.
impl Into<String> for ErrorKind {
    fn into(self) -> String {
        match self {
            ErrorKind::UnknownCharacter => "Unknown Character Found Here.",
            ErrorKind::InvalidNumberFormat => "This Number Has An Invalid Format.",
            ErrorKind::UnterminatedString => "Expected The End Of This String.",
            ErrorKind::ExpectedArgs(arg_amt) => {
                return format!(
                    "Expected {} More {}.",
                    arg_amt,
                    if arg_amt == 1 {
                        "Argument"
                    } else {
                        "Arguments"
                    }
                )
            }
            ErrorKind::EmptyStack => "Tried To Pop From An Empty Stack.",
            ErrorKind::TypeMismatch(expected, actual) => {
                return format!(
                    "Expected The Type {:#?}, But Found The Type {:#?}.",
                    expected, actual
                )
            }
        }
        .to_owned()
    }
}
