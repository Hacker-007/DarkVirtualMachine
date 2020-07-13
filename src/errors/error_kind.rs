//! The ErrorKind enum maintains the different errors that can occur during the execution of the program.
//! This allows for uniformity across the various errors because the error messages are the same.
//! This also increases readibility within the code, because the ErrorKind's are more descriptive.

pub enum ErrorKind {
    UnrecognizedArgument(String),

    UnknownCharacter,
    InvalidNumberFormat,
    InvalidLabelName,
    UnterminatedString,

    DuplicateLabel,
    NoMainLabel,
    EndWithoutLabel,

    EmptyStack,
    ExpectedArgs(usize),
    ValueMismatch(String, String),
    UnsupportedOperation(String, String),
    NoEndOfLabel,
    DivisionByZero,
    OutOfBounds(usize, usize),
    UndefinedVariable,
    UndefinedLabel,
}

/// Converts the ErrorKind into a String.
/// This is used in the prettify method to produce the error messages needed.
impl Into<String> for ErrorKind {
    fn into(self) -> String {
        match self {
            ErrorKind::UnrecognizedArgument(arg) => {
                return format!("The Argument '{}' Is Not A Valid Argument.", arg)
            }

            ErrorKind::UnknownCharacter => "Unknown Character Found Here.",
            ErrorKind::InvalidNumberFormat => "Invalid Number Format.",
            ErrorKind::InvalidLabelName => "Invalid Label Name.",
            ErrorKind::UnterminatedString => "Expected The End Of This String.",

            ErrorKind::DuplicateLabel => "Another Label With This Name Was Defined Already.",
            ErrorKind::NoMainLabel => "A Main Label Could Not Be Found.",
            ErrorKind::EndWithoutLabel => "Found An End That Is Not Associated With A Label.",

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
            ErrorKind::ValueMismatch(expected, actual) => {
                return format!(
                    "Expected The Value {:#?}, But Found The Value {:#?}.",
                    expected, actual,
                )
            }
            ErrorKind::UnsupportedOperation(operation, operand) => {
                return format!(
                    "The Operation '{}' Can Not Be Applied To {}",
                    operation, operand
                )
            }
            ErrorKind::NoEndOfLabel => "No 'end' Could Be Found To This Label.",
            ErrorKind::DivisionByZero => "Tried To Divide By 0.",
            ErrorKind::OutOfBounds(beginning, end) => {
                return format!(
                    "An Invalid Index Was Given. The Index Has To Be Between {} And {} Exclusive.",
                    beginning, end
                )
            }
            ErrorKind::UndefinedVariable => "Tried To Use A Variable That Has Not Been Defined.",
            ErrorKind::UndefinedLabel => "Tried To Use A Label That Has Not Been Defined.",
        }
        .to_owned()
    }
}
