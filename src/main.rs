mod error;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use error::Error;
use nom::Finish;

#[derive(Debug, Parser)]
struct Args {
    file: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub enum Variable {
    Integer(i64),
    Float(f64),
    String(String),
}

impl Variable {
    pub fn add(self, rhs: Variable) -> Result<Variable, Error> {
        let var = match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs + rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 + rhs),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs + rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs + rhs),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
            Variable::String(ref lhs) => match rhs {
                Variable::String(rhs) => Variable::String(format!("{lhs}{rhs}")),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
        };

        Ok(var)
    }

    pub fn multiply(self, rhs: Variable) -> Result<Variable, Error> {
        let var = match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs * rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 * rhs),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs * rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs * rhs),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
            Variable::String(_) => match rhs {
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
        };

        Ok(var)
    }

    pub fn divide(self, rhs: Variable) -> Result<Variable, Error> {
        let var = match self {
            Variable::Integer(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Integer(lhs / rhs),
                Variable::Float(rhs) => Variable::Float(lhs as f64 / rhs),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
            Variable::Float(lhs) => match rhs {
                Variable::Integer(rhs) => Variable::Float(lhs / rhs as f64),
                Variable::Float(rhs) => Variable::Float(lhs / rhs),
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
            Variable::String(_) => match rhs {
                _ => return Err(Error::TypeMismatch(self, rhs)),
            },
        };

        Ok(var)
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    ///
    Binding {
        identifier: &'a str,
        value: Variable,
    },
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.file {
        Some(path) => {
            let file = fs::read_to_string(path)?;
            let mut cursor = &file[..];
            let mut local = HashMap::new();

            while let Ok((i, statement)) = interpreter::w_statement::<nom::error::Error<&str>>(cursor, &mut local).finish() {
                cursor = i;
                match statement {
                    Statement::Binding { identifier, value } => local.insert(identifier, value),
                };
            }

            println!("{local:?}");
        },
        None => todo!(),
    }

    Ok(())
}

mod interpreter {
    use std::collections::HashMap;

    use nom::IResult;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take_till;
    use nom::bytes::complete::take_until;
    use nom::character::complete::alpha1;
    use nom::character::complete::char;
    use nom::character::complete::digit1;
    use nom::character::complete::multispace1;
    use nom::character::complete::not_line_ending;
    use nom::character::complete::one_of;
    use nom::combinator::fail;
    use nom::combinator::map;
    use nom::combinator::opt;
    use nom::combinator::recognize;
    use nom::error::ContextError;
    use nom::error::ParseError;
    use nom::multi::fold_many1;
    use nom::multi::many0;
    use nom::number::complete::double;
    use nom::sequence::delimited;
    use nom::sequence::pair;
    use nom::sequence::preceded;
    use nom::sequence::terminated;
    use nom::sequence::tuple;

    use crate::Statement;
    use crate::Variable;

    fn w_comment<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        preceded(tag("//"), not_line_ending)(i)
    }

    fn w_ws<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        recognize(many0(alt((multispace1, w_comment))))(i)
    }

    fn w_identifier<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        alpha1(i)
    }

    fn w_integer<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, i64, E> {
        let (i, int) = recognize(pair(opt(one_of("-+")), digit1))(i)?;
        Ok((i, int.parse().unwrap()))
    }

    fn w_float<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, f64, E> {
        double(i)
    }

    fn w_string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        delimited(char('"'), take_until("\""), char('"'))(i)
    }

    fn w_variable<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, Variable, E> {
        let integer: IResult<&'a str, Variable, E> = map(w_integer, |integer| Variable::Integer(integer))(i);
        let float: IResult<&'a str, Variable, E> = map(w_float, |float| Variable::Float(float))(i);
        let string: IResult<&'a str, Variable, E> = map(w_string, |string| Variable::String(string.to_string()))(i);

        let mut var = None;
        if let Ok((i, integer)) = integer {
            var = Some((i, integer));
        }

        if let Ok((i, float)) = float {
            match var {
                Some((k, _)) => if i.len() < k.len() { var = Some((i, float)); },
                None => var = Some((i, float)),
            }
        };

        if let Ok((i, string)) = string {
            match var {
                Some((k, _)) => if i.len() < k.len() { var = Some((i, string)); },
                None => var = Some((i, string)),
            }
        };

        match var {
            Some(var) => Ok(var),
            None => fail(i),
        }
    }

    fn w_factor<'a, 'b, E: ParseError<&'a str> + ContextError<&'a str> + 'b>(
        local: &'b HashMap<&'a str, Variable>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Variable, E> + 'b {
        terminated(
            alt((
                map(w_identifier, |identifier| match local.get(identifier) {
                    Some(variable) => variable.clone(),
                    None => todo!(),
                }),
                w_variable,
            )),
            w_ws,
        )
    }

    fn w_term<'a, 'b, E: ParseError<&'a str> + ContextError<&'a str> + 'b>(
        local: &'b HashMap<&'a str, Variable>,
        init: Variable,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Variable, E> + 'b {
        terminated(
            fold_many1(
                pair(alt((char('*'), char('/'))), w_factor(local)),
                move || init.clone(), // TODO: avoid the clone?
                |acc, (op, var)| match op {
                    '*' => match acc.multiply(var) {
                        Ok(var) => var,
                        Err(_) => todo!(),
                    },
                    '/' => match acc.divide(var) {
                        Ok(var) => var,
                        Err(_) => todo!(),
                    },
                    _ => unreachable!(),
                },
            ),
            w_ws,
        )
    }

    fn w_expression<'a, 'b, E: ParseError<&'a str> + ContextError<&'a str> + 'b>(
        local: &'b HashMap<&'a str, Variable>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, Variable, E> + 'b {
        terminated(
            w_term(local, w_variable),
            tuple((
                take_till(|c| c == ';'),
                char(';'),
                w_ws
            )),
        )
    }

    fn w_binding<'a, 'b, E: ParseError<&'a str> + ContextError<&'a str> + 'b> (
        local: &'b HashMap<&'a str, Variable>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, Variable), E> + 'b {
        pair(
            delimited(
                terminated(tag("let"), w_ws),
                w_identifier,
                delimited(w_ws, char('='), w_ws),
            ),
            w_expression(local)
        )
    }

    pub fn w_statement<'a, 'b, E: ParseError<&'a str> + ContextError<&'a str> + 'b>(
        i: &'a str,
        local: &'b mut HashMap<&'a str, Variable>,
    ) -> IResult<&'a str, Statement<'a>, E> {
        terminated(
            alt((
                map(w_binding(local), |(identifier, value)| Statement::Binding { identifier, value }),
            )),
            w_ws,
        )(i)
    }
}
