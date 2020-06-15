//! The Error struct maintains the errors that occur during execution.

pub struct Error {
    kind: ErrorKind,
    position: Option<usize>,
}

impl Error {
    /// Constructs a new error with the error kind and the position.
    ///
    /// # Arguments
    /// `kind` - The type of the error. Maintaining the type allows for the messages to be controlled across execution.
    /// `position` - The position where the error occurred.
    pub fn new(kind: ErrorKind, position: usize) -> Error {
        Error { kind, position: Some(position) }
    }

    /// Constructs a new error with the error kind and no position.
    ///
    /// # Arguments
    /// `kind` - The type of the error. Maintaining the type allows for the messages to be controlled across execution.
    pub fn message_only(kind: ErrorKind) -> Error {
        Error { kind, position: None }
    }

    /// This function generates a pretty version of the error, with arrows pointing to the exact location of the error.
    /// This function also consumes the error, therefore, it should be the last thing called.
    ///
    /// # Arguments
    /// `input` - The input for the program. This is not maintained with every error because the input might be different.
    pub fn prettify(self, input: &str) -> String {
        if self.position.is_some() {
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
                    "An Error Occurred On Line {} And Column {}.\n{}",
                    line_number, column_number, error_message,
                )
            }
        } else {
            // Convert the kind into an error message.
            let error_message: String = self.kind.into();
            format!("An Error Occurred.\n{}", error_message)
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

            if idx == &(self.position.unwrap() - 1) {
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
    InvalidLabelName,
    UnterminatedString,

    DuplicateLabel,
    NoMainLabel,

    EmptyStack,
    ExpectedArgs(usize),
    TypeMismatch(String, String),
    UnsupportedOperation(String, String),
    NoEndOfLabel,
    DivisionByZero,
    OutOfBounds(usize, usize),
}

/// Converts the ErrorKind into a String.
/// This is used in the prettify method to produce the error messages needed.
impl Into<String> for ErrorKind {
    fn into(self) -> String {
        match self {
            ErrorKind::UnknownCharacter => "Unknown Character Found Here.",
            ErrorKind::InvalidNumberFormat => "Invalid Number Format.",
            ErrorKind::InvalidLabelName => "Invalid Label Name.",
            ErrorKind::UnterminatedString => "Expected The End Of This String.",

            ErrorKind::DuplicateLabel => "Another Label With This Name Was Defined Already.",
            ErrorKind::NoMainLabel => "A Main Label Could Not Be Found.",

            ErrorKind::EmptyStack => "Tried To Pop From An Empty Stack.",
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
            ErrorKind::TypeMismatch(expected, actual) => {
                return format!(
                    "Expected The Type {:#?}, But Found The Type {:#?}.",
                    expected,
                    actual,
                )
            },
            ErrorKind::UnsupportedOperation(operation, operand) => return format!("The Operation '{}' Can Not Be Applied To {}", operation, operand),
            ErrorKind::NoEndOfLabel => "No 'end' Could Be Found To This Label.",
            ErrorKind::DivisionByZero => "Tried To Divide By 0.",
            ErrorKind::OutOfBounds(beginning, end) => return format!("An Invalid Index Was Given. The Index Has To Be Between {} And {} Exclusive.", beginning, end),
        }
        .to_owned()
    }
}
