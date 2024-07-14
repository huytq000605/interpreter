use std::collections::HashMap;
use std::ops;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Null,
}

impl ops::Add for Object {
    type Output = Result<Object, String>;

    fn add(self, rhs: Self) -> Result<Object, String> {
        return match self {
            Self::Number(num1) => match rhs {
                Self::Number(num2) => Ok(Self::Number(num1 + num2)),
                Self::String(s2) => Ok(Self::String(format!("{}{}", num1, s2))),
                Self::Null => return Err(format!("Invalid value rhs = NULL")),
            },
            Self::String(s1) => match rhs {
                Self::Number(num2) => Ok(Self::String(format!("{}{}", s1, num2))),
                Self::String(s2) => Ok(Self::String(s1 + &s2)),
                Self::Null => return Err(format!("Invalid value rhs = NULL")),
            },
            Self::Null => return Err(format!("Invalud value lhs = NULL")),
        };
    }
}

impl ops::Sub for Object {
    type Output = Result<Object, String>;

    fn sub(self, rhs: Self) -> Result<Object, String> {
        return match self {
            Self::Number(num1) => match rhs {
                Self::Number(num2) => Ok(Self::Number(num1 - num2)),
                _ => return Err(format!("Invalid value rhs = {:?}", self)),
            },
            _ => return Err(format!("Invalud value lhs = {:?}", self)),
        };
    }
}

pub struct Environment {
    pub variables: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            variables: HashMap::new(),
        };
    }
}
