pub mod result;
pub mod char_parsers;
pub mod combinators;
pub mod state;

pub use state::ParserState;
pub use result::{Position, ParserSuccess, ParserFailure, ParserResult, FailureSeverity};

/// `ParserFn` is a type alias for the closure returned by all parser functions and combinators. It takes a mutable reference
/// to a `ParserState` struct and returns a `ParserResult` which can either be a `ParserSuccess` or a `ParserFailure`.
pub type ParserFn<T> = Box<dyn FnOnce(&mut ParserState) -> ParserResult<T>>;

/// `Parser` has a single field contianing a `ParserFn`. This struct is the primary way simple parsing functions are composed into
/// more complex ones. 
pub struct Parser<T>
where T: 'static
{
    parser_fn: ParserFn<T>
}

impl<T> Parser<T> {
    /// `new` creates a new instance of the `Parser` struct.
    pub(in crate::parser) fn new(parser_fn: ParserFn<T>) -> Parser<T> {
        Parser { parser_fn }
    }

    /// `parse` is the method used to apply the parser function to a mutable reference of the `ParserState`.
    pub(in crate::parser) fn parse(self, state: &mut ParserState) -> ParserResult<T> {
        let p =self.parser_fn;
        p(state)
    }

    /// `and` applies the parser contained in the current parser struct, and if it succeeds, it then applies the parser assigned to the `other` parameter.
    /// If both parsers succeed, the results of both are returned in a tuple as the value of a `ParserSuccess` struct. If the first parser fails 
    /// without changing the parser state, a `ParserFailure` will be returned as an `Error`. If the first parser fails after changing the parser state
    /// or if the second parser fails, a `ParserFailure` is returned as a `FatalError`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let p_A = p_char('A');
    /// let p_B = p_char('B');
    ///
    /// let expected = Ok(ParserSuccess::new(
    ///     ('A', 'B'), 
    ///     Position::new(1, 3, 2)
    /// ));
    /// 
    /// let actual = p_A.and(p_B).run(String::from("AB"));
    /// 
    /// assert_eq!(expected, actual);
    /// ```
    pub fn and<U>(self, other: Parser<U>) -> Parser<(T, U)>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    let left = self.parse(state)?;

                    let right = match other.parse(state) {
                        Ok(success) => success,
                        Err(failure) => {
                            return Err(failure.to_fatal_err())
                        },
                    };

                    let result = (left.get_result(), right.get_result());

                    Ok(ParserSuccess::new(result, state.get_position()))
                }
            );

        Parser::new(parser_fn)
    }

    /// `and_try` applies the parser contained in the current parser struct, and if it succeeds, it then applies the parser assigned to the `other` parameter.
    /// If both parsers succeed, the results of both are returned in a tuple as the value of a `ParserSuccess` struct. Parser failures are handled
    /// the same as `and` except when the first parser succeeds but the second parser fails. `and` will return a `FatalError` when this happens
    /// whereas `and_try` will return a regular `Error` if the second parser fails without changing the `ParserState`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let p_A = p_char('A');
    /// let p_BC = p_char('B').and(p_char('C'));
    ///
    /// let expected = Err(ParserFailure::new_err(
    ///     String::from("B"), 
    ///     Some(String::from("C")),
    ///     Position::new(1, 2, 1)
    /// ));
    /// 
    /// let actual = p_A.and_try(p_BC).run(String::from("ACD"));
    /// 
    /// assert_eq!(expected, actual);
    /// 
    /// 
    /// let p_A = p_char('A');
    /// let p_BC = p_char('B').and(p_char('C'));
    ///
    /// let expected = Err(ParserFailure::new_fatal_err(
    ///     String::from("C"), 
    ///     Some(String::from("D")),
    ///     Position::new(1, 3, 2)
    /// ));
    /// 
    /// let actual = p_A.and_try(p_BC).run(String::from("ABD"));
    /// 
    /// assert_eq!(expected, actual);
    /// ```
    pub fn and_try<U>(self, other: Parser<U>) -> Parser<(T, U)>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    state.mark();

                    let left = match self.parse(state) {
                        Ok(success) => success,
                        Err(failure) => {
                            state.remove_mark();
                            return Err(failure)
                        },
                    };

                    let result = match other.parse(state) {
                        Ok(right) => {
                            let result = (left.get_result(), right.get_result());
                            Ok(ParserSuccess::new(result, state.get_position()))
                        },
                        Err(failure) => {
                            if !failure.is_fatal() {
                                state.revert();
                            }

                            Err(failure)
                        },
                    };
                      
                    state.remove_mark();
                    result
                }
            );

        Parser::new(parser_fn)
    }

    /// `or` applies the parser contained in the current parser struct, and if it succeeds, returns the results of the parser as a `ParserSuccess`.
    /// However if the first parser fails, `or` then tries to apply the parser assigned to the `other` parameter. If the second parser succeeds, the result
    /// value is returned as a `ParserSuccess`. If both parsers fail, `or` returns a `ParserFailure`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusty_parsec::*;
    /// 
    /// let p_A = p_char('A');
    /// let p_B = p_char('B');
    ///
    /// let expected = Ok(ParserSuccess::new(
    ///     'A', 
    ///     Position::new(1, 2, 1)
    /// ));
    /// 
    /// let actual = p_A.or(p_B).run(String::from("A"));
    /// 
    /// assert_eq!(expected, actual);
    /// 
    /// 
    /// let p_A = p_char('A');
    /// let p_B = p_char('B');
    ///
    /// let expected = Ok(ParserSuccess::new(
    ///     'B', 
    ///     Position::new(1, 2, 1)
    /// ));
    /// 
    /// let actual = p_A.or(p_B).run(String::from("B"));
    /// 
    /// assert_eq!(expected, actual);
    /// ```
    pub fn or(self, other: Parser<T>) -> Parser<T>
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState|
                    match self.parse(state) {
                        Ok(success) => Ok(success),
                        Err(failure) => {
                            if failure.is_fatal() {
                                Err(failure)
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

                    let next = match other.parse(state) {
                        Ok(success) => success,
                        Err(failure) => {
                            return Err(failure.to_fatal_err())
                        },
                    };
                    
                    Ok(prev.with_position(next.get_position()))
                }
            );

        Parser::new(parser_fn)
    }

    pub fn try_take_prev<U>(self, other: Parser<U>) -> Parser<T>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    state.mark();

                    let prev = match self.parse(state) {
                        Ok(success) => success,
                        Err(failure) => {
                            state.remove_mark();
                            return Err(failure)
                        },
                    };

                    let result = match other.parse(state) {
                        Ok(success) => {
                            Ok(prev.with_position(success.get_position()))
                        },
                        Err(failure) => {
                            if !failure.is_fatal() {
                                state.revert();
                            }

                            Err(failure)
                        },
                    };
                    
                    state.remove_mark();
                    result
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
                    match self.parse(state) {
                        Ok(_) => {
                            match other.parse(state) {
                                Ok(success) => Ok(success),
                                Err(failure) => Err(failure.to_fatal_err()),
                            }
                        },
                        Err(failure) => Err(failure),
                    }
            );

        Parser::new(parser_fn)
    }

    pub fn try_take_next<U>(self, other: Parser<U>) -> Parser<U>
    where U: 'static
    {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    state.mark();

                    let result = match self.parse(state) {
                        Ok(_) => {
                            match other.parse(state) {
                                Ok(success) => Ok(success),
                                Err(failure) => {
                                    if !failure.is_fatal() {
                                        state.revert();
                                    }

                                    Err(failure)
                                },
                            }
                        },
                        Err(failure) => Err(failure),
                    };

                    state.remove_mark();
                    result
                }
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

    pub fn bind<U>(self, f: Box<dyn Fn (T) -> Parser<U>>) -> Parser<U> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    match self.parse(state) {
                        Ok(success) => {
                            f(success.get_result()).parse(state)
                                .map_err(|failure| failure.to_fatal_err())
                        },
                        Err(failure) => Err(failure),
                    }
                }
            );

        Parser::new(parser_fn)
    }

    pub fn try_bind<U>(self, f: Box<dyn Fn (T) -> Parser<U>>) -> Parser<U> {
        let parser_fn =
            Box::new(
                move |state: &mut ParserState| {
                    state.mark();

                    let result = match self.parse(state) {
                        Ok(success) => {
                            f(success.get_result()).parse(state)
                                .map_err(
                                    |failure| {
                                        if !failure.is_fatal() {
                                            state.revert();                                    
                                        }
                                        failure
                                    }
                                )
                        },
                        Err(failure) => Err(failure),
                    };

                    state.remove_mark();
                    result
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

                    let result = match self.parse(state) {
                        Ok(success) => success,
                        Err(failure) => return Err(failure.to_fatal_err()),
                    };

                    let close = match p_close.parse(state) {
                        Ok(success) => success,
                        Err(failure) => return Err(failure.to_fatal_err()),
                    };

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
                            Err(ParserFailure::new_fatal_err(
                                label,
                                None,
                                state.get_position()
                            ))
                        },
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
                            Err(ParserFailure::new_fatal_err(
                                label,
                                None,
                                state.get_position()
                            ))
                        },
                        _ => {
                            state.revert();
                            Ok(ParserSuccess::new(result.get_result(), state.get_position()))
                        },
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
