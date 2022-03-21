use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Variable {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Variable {
    pub fn add(self, rhs: Self) -> Self {
        match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs + rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 + rhs),
                _ => todo!(),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs + rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs + rhs),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn subtract(self, rhs: Self) -> Self {
        match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs - rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 - rhs),
                _ => todo!(),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs - rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs - rhs),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn multiply(self, rhs: Self) -> Self {
        match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs * rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 * rhs),
                _ => todo!(),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs * rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs * rhs),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn divide(self, rhs: Self) -> Self {
        match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs / rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 / rhs),
                _ => todo!(),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs / rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs / rhs),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }


    pub fn equals(self, rhs: Self) -> Self {
        match self {
            Variable::Boolean(lhs) => match rhs {
                Variable::Boolean(rhs) => Variable::Boolean(lhs == rhs),
                _ => todo!(),
            },
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Boolean(lhs == rhs),
                Variable::Float(rhs) => Variable::Boolean(lhs as f64 == rhs),
                _ => todo!(),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Boolean(lhs == rhs as f64),
                Variable::Float(rhs) => Variable::Boolean(lhs == rhs),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Boolean(b) => b.fmt(fmt),
            Variable::Integer(i) => i.fmt(fmt),
            Variable::Float(f) => f.fmt(fmt),
            Variable::String(s) => s.fmt(fmt),
        }
    }
}
