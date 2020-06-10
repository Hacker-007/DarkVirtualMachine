use crate::values::values::ValueKind;

pub struct Error {
    kind: ErrorKind,
    position: usize,
}

impl Error {
    pub fn new(kind: ErrorKind, position: usize) -> Error {
        Error { kind, position }
    }

    pub fn prettify(self, input: &str) -> String {
        let (line_number, column_number) = self.get_line_column_info(input);
        let option_line = input.split_terminator('\n').nth(line_number - 1);
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

    fn get_line_column_info(&self, input: &str) -> (usize, usize) {
        let (mut line_number, mut column_number) = (1, 0);
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

pub enum ErrorKind {
    UnknownCharacter,
    InvalidNumberFormat,
    UnterminatedString,
    ExpectedArgs(usize),
    TypeMismatch(ValueKind, ValueKind),
}

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
