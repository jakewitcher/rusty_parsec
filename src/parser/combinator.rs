use super::{ParserFn, ParserState, ParserSuccess, ParserResult};

pub struct Parser<T>
where T: 'static
{
    parser_fn: ParserFn<T>
}

impl<T> Parser<T> {
    pub fn new(parser_fn: ParserFn<T>) -> Parser<T> {
        Parser { parser_fn }
    }

    pub fn get(self) -> ParserFn<T> {
        self.parser_fn
    }

    pub fn parse(self, state: &mut ParserState) -> ParserResult<T> {
        self.get()(state)
    }

    pub fn and<U>(self, other: Parser<U>) -> Parser<(T, U)>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let left = self.parse(state)?;
                    let right = other.parse(state)?;

                    let result = (left.get_result(), right.get_result());

                    Ok(ParserSuccess::new(result, state.get_position()))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn or(self, other: Parser<T>) -> Parser<T>
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| 
                    self.parse(state).or(other.parse(state))
            );

        Parser::new(parser_fn)
    }

    pub fn take_prev<U>(self, other: Parser<U>) -> Parser<T>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let prev = self.parse(state)?;
                    let next = other.parse(state)?;
                    
                    Ok(prev.with_position(next.get_position()))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn take_next<U>(self, other: Parser<U>) -> Parser<U>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState|
                    self.parse(state).and(other.parse(state))
            );

        Parser::new(parser_fn)
    }

    pub fn then_return<U>(self, return_value: U) -> Parser<U>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let result = self.parse(state)?;
                    Ok(ParserSuccess::new(return_value, result.get_position()))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn or_return(self, return_value: T) -> Parser<T> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    self.parse(state).or(Ok(ParserSuccess::new(return_value, state.get_position())))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn between<U, V>(self, p_open: Parser<U>, p_close: Parser<V>) -> Parser<T>
    where U: 'static, V: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    p_open.parse(state)?;
                    let result = self.parse(state)?;
                    let close = p_close.parse(state)?;

                    Ok(result.with_position(close.get_position()))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn opt(self) -> Parser<Option<T>> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    match self.parse(state) {
                        Ok(success) => {
                            Ok(ParserSuccess::new(Some(success.get_result()), state.get_position()))
                        },
                        _ => {
                            Ok(ParserSuccess::new(None, state.get_position()))
                        }
                    }
                }
            );

        Parser::new(parser_fn)
    }

    pub fn optional(self) -> Parser<()> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    match self.parse(state) {
                        _ => Ok(ParserSuccess::new((), state.get_position())),
                    }
                }
            );

        Parser::new(parser_fn)
    }

    pub fn map<U>(self, f: Box<dyn Fn(T) -> U>) -> Parser<U>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let result = self.parse(state)?;

                    Ok(result.map_result(f))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn run(self, input: String) -> ParserResult<T> {
        self.parse(&mut ParserState::new(input))
    }
}
