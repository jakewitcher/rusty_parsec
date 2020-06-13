pub mod result;
pub mod char_parsers;
pub mod combinators;
pub mod state;

pub use state::ParserState;
pub use result::{Position, ParserSuccess, ParserFailure, ParserResult, Severity};

pub type ParserFn <T> = Box<dyn FnOnce(&mut ParserState) -> ParserResult<T>>;

pub struct Parser<T>
where T: 'static
{
    parser_fn: ParserFn<T>
}

impl<T> Parser<T> {
    pub fn new(parser_fn: ParserFn<T>) -> Parser<T> {
        Parser { parser_fn }
    }

    pub fn parse(self, state: &mut ParserState) -> ParserResult<T> {
        let p =self.parser_fn;
        p(state)
    }

    pub fn and<U>(self, other: Parser<U>) -> Parser<(T, U)>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let left = self.parse(state)?;
                    let right = match other.parse(state) {
                        Ok(success) => success,
                        Err(err) => {
                            return Err(err.with_severity(Severity::FatalError))
                        }
                    };

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
                    match self.parse(state) {
                        Ok(success) => Ok(success),
                        Err(err) => {
                            if err.is_fatal() {
                                Err(err)
                            } else {
                                other.parse(state)
                            }
                        },
                    }
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

    pub fn followed_by<U>(self, parser: Parser<U>) -> Parser<T> {
        self.followed_by_l(parser, "following parser to succeed".to_string())
    }

    pub fn followed_by_l<U>(self, parser: Parser<U>, label: String) -> Parser<T> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let result = self.parse(state)?;

                    state.mark();
                    match parser.parse(state) {
                        Ok(_) => {
                            state.revert();
                            Ok(ParserSuccess::new(result.get_result(), state.get_position()))
                        },
                        _ => {
                            state.revert();
                            Err(ParserFailure::new(
                                label,
                                None,
                                Severity::Error,
                                state.get_position()
                            ))
                        }
                    }
                }
            );

        Parser::new(parser_fn)
    }

    pub fn not_followed_by<U>(self, parser: Parser<U>) -> Parser<T> {
        self.not_followed_by_l(parser, "following parser to fail".to_string())
    }

    pub fn not_followed_by_l<U>(self, parser: Parser<U>, label: String) -> Parser<T> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let result = self.parse(state)?;

                    state.mark();
                    match parser.parse(state) {
                        Ok(_) => {
                            state.revert();
                            Err(ParserFailure::new(
                                label,
                                None,
                                Severity::Error,
                                state.get_position()
                            ))
                        },
                        _ => {
                            state.revert();
                            Ok(ParserSuccess::new(result.get_result(), state.get_position()))
                        }
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
