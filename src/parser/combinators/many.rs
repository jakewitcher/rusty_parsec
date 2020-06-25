use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// ```many``` applies the parser returned by ```get_parser``` repeatedly until it fails. A Vector of the parsed values is returned once the parser fails. The parser must fail without changing the parser state or ```many``` will return a fatal error.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_hello() -> Parser<String> {
///     p_string("hello".to_string())
/// }
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         vec![ "hello".to_string(), "hello".to_string(), "hello".to_string()], 
///         Position::new(1, 16, 15))
///     );
/// 
/// let actual = many(p_hello).run("hellohellohello".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn many<T>(get_parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results: Vec<T> = apply_parser(get_parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```many_1``` is similar to ```many``` in that it applies the parser returned by ```get_parser``` repeatedly until it fails, however ```many_1``` must succeed at least once. A Vector of the parsed values is returned once the parser fails. The parser must fail without changing the parser state or ```many_1``` will return a fatal error.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_hello() -> Parser<String> {
///     p_string("hello".to_string())
/// }
/// 
/// let expected = 
///     Err(ParserFailure::new_err(
///         "hello".to_string(), 
///         Some("goodb".to_string()),
///         Position::new(1, 1, 0))
///     );
/// 
/// let actual = many_1(p_hello).run("goodbye".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn many_1<T>(get_parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match get_parser().parse(state) {
                    Ok(success) => {
                        let mut results = apply_parser(get_parser, state)?;
                        results.insert(0, success.get_result());
                        Ok(ParserSuccess::new(results, state.get_position()))
                    },
                    Err(failure) => Err(failure),
                }
            }
        );

    Parser::new(parser_fn)
}

/// ```skip_many``` applies the parser returned by ```get_parser``` repeatedly until it fails. The parser must fail without changing the parser state or ```skip_many``` will return a fatal error.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_hello() -> Parser<String> {
///     p_string("hello".to_string())
/// }
/// 
/// let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));
/// 
/// let actual = skip_many(p_hello).run("hellohellohello".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn skip_many<T>(get_parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parser(get_parser, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// ```skip_many_1``` is similar to ```skip_many_1``` in that it applies the parser returned by ```get_parser``` repeatedly,  however ```skip_many_1``` must succeed at least once before returning ```()```. The parser must fail without changing the parser state or ```skip_many_1``` will return a fatal error.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_hello() -> Parser<String> {
///     p_string("hello".to_string())
/// }
/// 
/// let expected = 
///     Err(ParserFailure::new_err(
///         "hello".to_string(), 
///         Some("goodb".to_string()),
///         Position::new(1, 1, 0))
///     );
/// 
/// let actual = skip_many_1(p_hello).run("goodbye".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn skip_many_1<T>(get_parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match get_parser().parse(state) {
                    Ok(_) => {
                        let _ = apply_parser(get_parser, state)?;
                        Ok(ParserSuccess::new((), state.get_position()))
                    },
                    Err(failure) => Err(failure),
                }
            }
        );

    Parser::new(parser_fn)
}

fn apply_parser<T>(p: fn() -> Parser<T>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    let mut results: Vec<T> = Vec::new();
    let mut parser_succeeds = true;

    while parser_succeeds {
        match p().parse(state) {
            Ok(success) => {
                results.push(success.get_result());
            },
            Err(failure) => {
                if failure.is_fatal() {
                    return Err(failure)
                }
                parser_succeeds = false;
            },
        }
    }

    Ok(results)
}