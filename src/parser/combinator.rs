use super::{Parser, ParserState, ParserSuccess, ParserResult};

pub struct Combinator<T>
where T: 'static
{
    parser: Parser<T>
}

impl<T> Combinator<T> {
    pub fn new(parser: Parser<T>) -> Combinator<T> {
        Combinator { parser }
    }

    pub fn get_parser(self) -> Parser<T> {
        self.parser
    }

    pub fn and<U>(self, other: Parser<U>) -> Combinator<(T, U)>
    where U: 'static
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    let left = p(state)?;
                    let right = other(state)?;

                    let position = right.get_position();
                    let result = (left.get_result(), right.get_result());

                    Ok(ParserSuccess::new(result, position))
                }
            );

        Combinator::new(parser)
    }

    pub fn or(self, other: Parser<T>) -> Combinator<T>
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    p(state).or(other(state))
                }
            );

        Combinator::new(parser)
    }

    pub fn take_prev<U>(self, other: Parser<U>) -> Combinator<T>
    where U: 'static
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    let prev = p(state)?;
                    let next = other(state)?;
                    
                    Ok(prev.with_position(next.get_position()))
                }
            );

        Combinator::new(parser)
    }

    pub fn take_next<U>(self, other: Parser<U>) -> Combinator<U>
    where U: 'static
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    p(state).and(other(state))
                }
            );

        Combinator::new(parser)
    }

    pub fn then_return<U>(self, return_value: U) -> Combinator<U>
    where U: 'static
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    let result = p(state)?;
                    
                    Ok(ParserSuccess::new(return_value, result.get_position()))
                }
            );

        Combinator::new(parser)
    }

    pub fn between<U, V>(self, p_open: Parser<U>, p_close: Parser<V>) -> Combinator<T>
    where U: 'static, V: 'static
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.get_parser();

                    p_open(state)?;

                    let result = p(state)?;
                    let close = p_close(state)?;

                    Ok(result.with_position(close.get_position()))
                }
            );

        Combinator::new(parser)
    }

    pub fn map<U>(self, f: Box<dyn Fn(T) -> U>) -> Combinator<U>
    where U: 'static
    {
        let parser =
            Box::new(
                move |state: &mut ParserState| {
                    let p = self.parser;
                    let result = p(state)?;

                    Ok(result.map_result(f))
                }
            );

        Combinator::new(parser)
    }

    pub fn run(self, input: String) -> ParserResult<T> {
        let parser = self.parser;
        let mut state = ParserState::new(input);

        parser(&mut state)
    }
}
