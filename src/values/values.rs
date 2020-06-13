use crate::utils::{error::{ErrorKind, Error}, token::{Token, TokenKind}};
use std::{fmt, rc::Rc};

/// The Value struct maintains both the position where this value is used and its kind.
/// Maintaining the position is useful because it can be used to produce good error messages.

pub struct Value {
    pub pos: usize,
    pub kind: ValueKind,
}

impl Value {
    /// Constructs a new Value struct with the specified position and kind.
    ///
    /// # Arguments
    /// `pos` - The position where this value is created or called.
    /// `kind` - The type of this value.
    pub fn new(pos: usize, kind: ValueKind) -> Value {
        Value { pos, kind }
    }
}

impl Value {
    /// This function takes the current value and a reference to another value and adds them together.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to add.
    /// `pos` - The position where this operation was called.
    pub fn add(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.add(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.add(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.add(val2, pos),

            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::String(format!("{}{}", val1, val2)))),
            (_, ValueKind::String(val2)) if self.kind != ValueKind::Void => Ok(Value::new(pos, ValueKind::String(format!("{:#?}{}", self, val2)))),
            (ValueKind::String(val1), _) if other.kind != ValueKind::Void => Ok(Value::new(pos, ValueKind::String(format!("{}{:#?}", val1, other)))),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Int(val1 + val2))),
            (ValueKind::Int(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Float(*val1 as f64 + val2))),
            (ValueKind::Float(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Float(val1 + *val2 as f64))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Float(val1 + val2))),
            
            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Add".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and subtracts them.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to subtract.
    /// `pos` - The position where this operation was called.
    pub fn sub(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.add(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.add(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.add(val2, pos),

            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Int(val1 - val2))),
            (ValueKind::Int(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Float(*val1 as f64 - val2))),
            (ValueKind::Float(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Float(val1 - *val2 as f64))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Float(val1 - val2))),
            
            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Sub".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and mutliplies them.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to multiply.
    /// `pos` - The position where this operation was called.
    pub fn mul(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.add(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.add(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.add(val2, pos),

            (ValueKind::String(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::String(val1.repeat(val2.abs() as usize)))),
            (ValueKind::Int(val1), ValueKind::String(val2)) if self.kind != ValueKind::Void => Ok(Value::new(pos, ValueKind::String(val2.repeat(val1.abs() as usize)))),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Int(val1 * val2))),
            (ValueKind::Int(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Float(*val1 as f64 * val2))),
            (ValueKind::Float(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Float(val1 * *val2 as f64))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Float(val1 * val2))),
            
            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Mul".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and divides them.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to divide.
    /// `pos` - The position where this operation was called.
    pub fn div(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.add(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.add(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.add(val2, pos),

            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                if val2 == &0 {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Int(val1 - val2)))
                }
            }
            (ValueKind::Int(val1), ValueKind::Float(val2)) => {
                if val2 - 0.0 < std::f64::EPSILON {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Float(*val1 as f64 - val2)))
                }
            }
            (ValueKind::Float(val1), ValueKind::Int(val2)) => {
                if val1 - 0.0 < std::f64::EPSILON {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Float(val1 - *val2 as f64)))
                }
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                if val2 - 0.0 < std::f64::EPSILON {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Float(val1 / val2)))
                }
            }
            
            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Div".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is less than the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn lt(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.lt(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.lt(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.lt(val2, pos),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 < val2))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 < val2))),
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 < val2))),

            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Lt".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is less than or equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn lte(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.lte(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.lte(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.lte(val2, pos),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 <= val2))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 <= val2))),
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 <= val2))),

            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Lte".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is greater than the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn gt(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.gt(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.gt(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.gt(val2, pos),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 > val2))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 > val2))),
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 > val2))),

            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Gt".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is greater than or equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn gte(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.gte(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.gte(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.gte(val2, pos),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 >= val2))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 >= val2))),
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 >= val2))),

            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Gte".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn equal(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.equal(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.equal(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.equal(val2, pos),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 == val2))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 == val2))),
            (ValueKind::Boolean(val1), ValueKind::Boolean(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 == val2))),
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 == val2))),

            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Eq".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is not equal to the second one. Note that this function does not consume either value. 
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn not_equal(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1.not_equal(val2, pos),
            (ValueKind::Variable(_, val1), _) => val1.not_equal(other, pos),
            (_, ValueKind::Variable(_, val2)) => self.not_equal(val2, pos),
            
            (ValueKind::Int(val1), ValueKind::Int(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 != val2))),
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 != val2))),
            (ValueKind::Boolean(val1), ValueKind::Boolean(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 != val2))),
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(pos, ValueKind::Boolean(val1 != val2))),

            _ => Err(Error::new(ErrorKind::UnsupportedOperation("Neq".to_owned(), format!("The Type '{}' And The Type '{}'.", self.kind.get_type_name(), other.kind.get_type_name())), pos)),
        }
    }
}

/// The ValueKind enum maintains the various types in the language.
/// All of the supported types are in this enum. This makes it easy to expand in the future.

pub enum ValueKind {
    Void,
    Any,
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Variable(String, Rc<Value>),
    Push,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
}

impl ValueKind {
    /// This function gets the name of the type.
    /// For example, an int with the value of 15, will have the type name 'Int'.
    /// This method is used to provide the right error messages.
    pub fn get_type_name(&self) -> String {
        match self {
            ValueKind::Void => "Void",
            ValueKind::Any => "Any",
            ValueKind::Int(_) => "Int",
            ValueKind::Float(_) => "Float",
            ValueKind::Boolean(_) => "Boolean",
            ValueKind::String(_) => "String",
            ValueKind::Variable(_, value) => return value.kind.get_type_name(),
            ValueKind::Push => "Instruction Push",
            ValueKind::Pop => "Instruction Pop",
            ValueKind::Add => "Instruction Add",
            ValueKind::Sub => "Instruction Sub",
            ValueKind::Mul => "Instruction Mul",
            ValueKind::Div => "Instruction Div",
            ValueKind::LessThan => "Instruction LessThan",
            ValueKind::LessThanEqual => "Instruction LessThanEqual",
            ValueKind::GreaterThan => "Instruction GreaterThan",
            ValueKind::GreaterThanEqual => "Instruction GreaterThanEqual",
            ValueKind::Equal => "Instruction Equal",
            ValueKind::NotEqual => "Instruction NotEqual",
        }.to_owned()
    }
}

/// Converts a token into a value. This is used by the Code struct when generating the vector of values.
impl From<Token> for Value {
    fn from(token: Token) -> Self {
        Value {
            pos: token.pos,
            kind: match token.kind {
                TokenKind::Void => ValueKind::Void,
                TokenKind::Any => ValueKind::Any,
                TokenKind::IntegerLiteral(value) => ValueKind::Int(value),
                TokenKind::FloatLiteral(value) => ValueKind::Float(value),
                TokenKind::BooleanLiteral(value) => ValueKind::Boolean(value),
                TokenKind::StringLiteral(value) => ValueKind::String(value),
                TokenKind::Identifier(name) => ValueKind::Variable(name, Rc::new(Value::new(token.pos, ValueKind::Void))),
                TokenKind::Push => ValueKind::Push,
                TokenKind::Pop => ValueKind::Pop,
                TokenKind::Add => ValueKind::Add,
                TokenKind::Sub => ValueKind::Sub,
                TokenKind::Mul => ValueKind::Mul,
                TokenKind::Div => ValueKind::Div,
                TokenKind::LessThan => ValueKind::LessThan,
                TokenKind::LessThanEqual => ValueKind::LessThanEqual,
                TokenKind::GreaterThan => ValueKind::GreaterThan,
                TokenKind::GreaterThanEqual => ValueKind::GreaterThanEqual,
                TokenKind::Equal => ValueKind::Equal,
                TokenKind::NotEqual => ValueKind::NotEqual,
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialEq for ValueKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueKind::Void, ValueKind::Void) |
            (ValueKind::Any, ValueKind::Any) => true,
            (ValueKind::Int(val1), ValueKind::Int(val2)) => val1 == val2,
            (ValueKind::Float(val1), ValueKind::Float(val2)) => val1 == val2,
            (ValueKind::Boolean(val1), ValueKind::Boolean(val2)) => val1 == val2,
            (ValueKind::String(val1), ValueKind::String(val2)) => val1 == val2,
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1 == val2,
            (ValueKind::Push, ValueKind::Push) |
            (ValueKind::Pop, ValueKind::Pop) |
            (ValueKind::Add, ValueKind::Add) |
            (ValueKind::Sub, ValueKind::Sub) |
            (ValueKind::Mul, ValueKind::Mul) |
            (ValueKind::Div, ValueKind::Div) |
            (ValueKind::LessThan, ValueKind::LessThan) |
            (ValueKind::LessThanEqual, ValueKind::LessThanEqual) |
            (ValueKind::GreaterThan, ValueKind::GreaterThan) |
            (ValueKind::GreaterThanEqual, ValueKind::GreaterThanEqual) |
            (ValueKind::Equal, ValueKind::Equal) |
            (ValueKind::NotEqual, ValueKind::NotEqual) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.kind)
    }
}

impl fmt::Debug for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::Void => write!(f, "Void"),
            ValueKind::Any => write!(f, "Any"),
            ValueKind::Int(value) => write!(f, "{}", value),
            ValueKind::Float(value) => write!(f, "{}", value),
            ValueKind::Boolean(value) => write!(f, "{}", value),
            ValueKind::String(value) => write!(f, "{}", value),
            ValueKind::Variable(name, _) => write!(f, "Variable '{}'", name),
            ValueKind::Push => write!(f, "<instruction push>"),
            ValueKind::Pop => write!(f, "<instruction pop>"),
            ValueKind::Add => write!(f, "<instruction add>"),
            ValueKind::Sub => write!(f, "<instruction sub>"),
            ValueKind::Mul => write!(f, "<instruction mul>"),
            ValueKind::Div => write!(f, "<instruction div>"),
            ValueKind::LessThan => write!(f, "<instruction lt>"),
            ValueKind::LessThanEqual => write!(f, "<instruction lte>"),
            ValueKind::GreaterThan => write!(f, "<instruction gt>"),
            ValueKind::GreaterThanEqual => write!(f, "<instruction gte>"),
            ValueKind::Equal => write!(f, "<instruction eq>"),
            ValueKind::NotEqual => write!(f, "<instruction neq>"),
        }
    }
}
