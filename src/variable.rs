use nom::IResult;
use nom::error::ContextError;
use nom::error::ParseError;


impl Variable {
    fn parse<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        parser::w_variable(i)
    }
}

mod parser {
    use nom::IResult;
    use nom::bytes::complete::take_until;
    use nom::character::streaming::char;
    use nom::error::ContextError;
    use nom::error::ParseError;
    use nom::number::complete::double;
    use nom::sequence::delimited;

    fn w_number<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, f64, E> {
        double(i)
    }

    fn w_string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        delimited(char('"'), take_until("\""), char('"'))(i)
    }

    pub fn w_variable<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
        i: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        delimited(char('"'), take_until("\""), char('"'))(i)
    }
}
