use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::{self, DerefMut};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Null,
    Return(Box<Object>),
}

impl ops::Add for Object {
    type Output = Result<Object, String>;

    fn add(self, rhs: Self) -> Result<Object, String> {
        return match self {
            Self::Number(num1) => match rhs {
                Self::Number(num2) => Ok(Self::Number(num1 + num2)),
                Self::String(s2) => Ok(Self::String(format!("{}{}", num1, s2))),
                Self::Null | Self::Return(_) => return Err(format!("Invalid value rhs = NULL")),
            },
            Self::String(s1) => match rhs {
                Self::Number(num2) => Ok(Self::String(format!("{}{}", s1, num2))),
                Self::String(s2) => Ok(Self::String(s1 + &s2)),
                Self::Null | Self::Return(_) => return Err(format!("Invalid value rhs = NULL")),
            },
            Self::Null | Self::Return(_) => return Err(format!("Invalud value lhs = NULL")),
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
    pub outer: Option<Rc<RefCell<Environment>>>,
    pub in_function: bool,
}

impl Environment {
    pub fn new(outer_option: Option<Rc<RefCell<Environment>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(match outer_option {
            None => Self {
                variables: HashMap::new(),
                outer: None,
                in_function: false,
            },
            Some(outer_env) => Self {
                variables: HashMap::new(),
                outer: Some(outer_env),
                in_function: false,
            },
        }))
    }
}