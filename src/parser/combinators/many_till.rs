use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// ```many_till``` applies the parser returned by ```get_parser``` repeatedly until ```end_parser``` succeeds, returning a Vector of the parsed values.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// pub fn p_true() -> Parser<bool> {
///     p_string("true".to_string())
///         .then_return(true)
/// }
/// 
/// let expected = Ok(ParserSuccess::new(vec![true, true, true], Position::new(1, 16, 15)));
/// 
/// let actual = 
///     many_till(p_true, p_u32)
///         .run("truetruetrue123".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn many_till<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results = apply_parsers(get_parser, end_parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```many_1_till``` applies the parser returned by ```get_parser``` repeatedly until ```end_parser``` succeeds, returning a Vector of the parsed values. ```get_parser``` must succeed at least once or ```many_1_till``` will return a parser failure.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// pub fn p_true() -> Parser<bool> {
///     p_string("true".to_string())
///         .then_return(true)
/// }
/// 
/// let expected = Err(ParserFailure::new_err("true".to_string(), Some("fals".to_string()), Position::new(1, 1, 0)));
/// 
/// let actual = 
///     many_1_till(p_true, p_u32)
///         .run("false".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn many_1_till<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results = apply_parsers_1(get_parser, end_parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```skip_many_till``` applies the parser returned by ```get_parser``` repeatedly until ```end_parser``` succeeds and returns ```()``` as the result.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// pub fn p_true() -> Parser<bool> {
///     p_string("true".to_string())
///         .then_return(true)
/// }
/// 
/// let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));
/// 
/// let actual = 
///     skip_many_till(p_true, p_u32)
///         .run("truetruetrue123".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn skip_many_till<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parsers(get_parser, end_parser, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```skip_many_1_till``` applies the parser returned by ```get_parser``` repeatedly until ```end_parser``` succeeds and returns ```()``` as the result. ```get_parser``` must succeed at least once or ```skip_many_1_till``` will return a parser failure.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// pub fn p_true() -> Parser<bool> {
///     p_string("true".to_string())
///         .then_return(true)
/// }
/// 
/// let expected = Err(ParserFailure::new_err("true".to_string(), Some("fals".to_string()), Position::new(1, 1, 0)));
/// 
/// let actual = 
///     skip_many_1_till(p_true, p_u32)
///         .run("false".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn skip_many_1_till<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parsers_1(get_parser, end_parser, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

fn apply_parsers<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    apply(get_parser, end_parser, state, handle_parser_failure)
}

fn apply_parsers_1<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    apply(get_parser, end_parser, state, handle_parser_failure_1)
}

fn apply<T, U>(get_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>, state: &mut ParserState, failure_handler: fn(ParserFailure, Vec<T>) -> Result<Vec<T>, ParserFailure>) -> Result<Vec<T>, ParserFailure> {
    let mut results: Vec<T> = Vec::new();
    let mut end_parser_succeeds = false;

    while !end_parser_succeeds {
        match get_parser().parse(state) {
            Ok(success) => {
                end_parser_succeeds = apply_end_parser(end_parser, state, success, &mut results)?;
            },
            Err(failure) => {
                return failure_handler(failure, results)
            },
        }
    }

    Ok(results)
}

fn handle_parser_failure<T>(failure: ParserFailure, results: Vec<T>) -> Result<Vec<T>, ParserFailure> {
    if failure.is_fatal() {
        Err(failure)
    } else if results.len() == 0 {
        Ok(results)
    } else {
        Err(failure.to_fatal_err())
    }
}

fn handle_parser_failure_1<T>(failure: ParserFailure, results: Vec<T>) -> Result<Vec<T>, ParserFailure> {
    if failure.is_fatal() {
        Err(failure)
    } else if results.len() == 0 {
        Err(failure)
    } else {
        Err(failure.to_fatal_err())
    }
}

fn apply_end_parser<T, U>(end_parser: fn() -> Parser<U>, state: &mut ParserState, success: ParserSuccess<T>, results: &mut Vec<T>) -> Result<bool, ParserFailure> {
    match end_parser().parse(state) {
        Ok(_) => {
            results.push(success.get_result());

            Ok(true)
        },
        Err(failure) => {
            if failure.is_fatal() {
                return Err(failure)
            }

            results.push(success.get_result());
            Ok(false)
        }
    }
}