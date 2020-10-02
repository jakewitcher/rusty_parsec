use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// `sep_by` takes two parsers and applies the first parser (`parser`) followed by the second parser (`separator`) repeatedly until one of them fails.
/// Once either parser fails, all values parsed by the `parser` are returned in a Vector as a `ParserSuccess`.
/// If the `parser` fails on the first attempt, `sep_by` will return a `ParserSuccess` with an empty Vector.
/// 
/// # Errors
/// `sep_by` will return a `ParserFailure` with a `FatalError` if either the `parser` or the `separator` fails having changed the parser state.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_comma() -> Parser<char> {
/// #     p_char(',')
/// # }
/// #
/// let expected = Ok(ParserSuccess::new(
///     vec![1,2,3], 
///     Position::new(1, 6, 5))
/// );
/// 
/// let actual = sep_by(
///     p_u32, 
///     p_comma
/// ).run(String::from("1,2,3"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn sep_by<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<Vec<T>> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results = apply_parser(parser, separator, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );
    
    Parser::new(parser_fn)
}

/// `sep_by_1` takes two parsers and applies the first parser (`parser`) followed by the second parser (`separator`) repeatedly until one of them fails.
/// Once either parser fails, all values parsed by the `parser` are returned in a Vector as a `ParserSuccess`.
/// 
/// # Errors
/// `sep_by_1` will return a `ParserFailure` with a `FatalError` if either the `parser` or the `separator` fails having changed the parser state.
/// Unlinke `sep_by`, if the `parser` fails on the first attempt, `sep_by_1` will return a `ParserFailure`. The `parser` must succeed at least
/// once for `sep_by_1` to return a `ParserSuccess`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_comma() -> Parser<char> {
/// #     p_char(',')
/// # }
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("value satisfying parser at least once"), 
///     None,
///     Position::new(1, 1, 0))
/// );
/// 
/// let actual = sep_by_1(
///     p_u32, 
///     p_comma
/// ).run(String::from("A,B,C"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn sep_by_1<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<Vec<T>> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results = apply_parser(parser, separator, state)?;

                if results.len() == 0 {
                    Err(ParserFailure::new_err(
                        "value satisfying parser at least once".to_string(),
                        None,
                        state.get_position()
                    ))
                } else {
                    Ok(ParserSuccess::new(results, state.get_position()))
                }
            }
        );
    
    Parser::new(parser_fn)
}

/// `skip_sep_by` takes two parsers and applies the first parser (`parser`) followed by the second parser (`separator`) repeatedly until one of them fails.
/// Once either parser fails, `()` is returned as a `ParserSuccess`.
/// If the `parser` fails on the first attempt, `skip_sep_by` will return a `ParserSuccess` of `()`.
/// 
/// # Errors
/// `skip_sep_by` will return a `ParserFailure` with a `FatalError` if either the `parser` or the `separator` fails having changed the parser state.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_comma() -> Parser<char> {
/// #     p_char(',')
/// # }
/// #
/// let expected = Ok(ParserSuccess::new(
///     (), 
///     Position::new(1, 6, 5))
/// );
/// 
/// let actual = skip_sep_by(
///     p_u32, 
///     p_comma
/// ).run(String::from("1,2,3"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_sep_by<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<()> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parser(parser, separator, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );
    
    Parser::new(parser_fn)
}

/// `skip_sep_by_1` takes two parsers and applies the first parser (`parser`) followed by the second parser (`separator`) repeatedly until one of them fails.
/// Once either parser fails, `()` is returned as a `ParserSuccess`.
/// 
/// # Errors
/// `skip_sep_by_1` will return a `ParserFailure` with a `FatalError` if either the `parser` or the `separator` fails having changed the parser state.
/// Unlinke `sep_by`, if the `parser` fails on the first attempt, `skip_sep_by_1` will return a `ParserFailure`. The `parser` must succeed at least
/// once for `skip_sep_by_1` to return a `ParserSuccess`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_comma() -> Parser<char> {
/// #     p_char(',')
/// # }
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("value satisfying parser at least once"), 
///     None,
///     Position::new(1, 1, 0))
/// );
/// 
/// let actual = skip_sep_by_1(
///     p_u32, 
///     p_comma
/// ).run(String::from("A,B,C"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_sep_by_1<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>) -> Parser<()> 
where U: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                if let Ok(_) = parser().parse(state) {
                    match separator().parse(state) {
                        Ok(_) => {
                            let _ = apply_parser(parser, separator, state)?;
                        },
                        Err(failure) => {
                            if failure.is_fatal() {
                                return Err(failure);
                            }
                        }
                    }

                    return Ok(ParserSuccess::new((), state.get_position()))
                }

                Err(ParserFailure::new_err(
                    "value satisfying parser at least once".to_string(),
                    None,
                    state.get_position()
                ))
            }
        );
    
    Parser::new(parser_fn)
}

fn apply_parser<T, U>(parser: fn() -> Parser<T>, separator: fn() -> Parser<U>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    let mut results: Vec<T> = Vec::new();
    let mut parser_succeeds = true;

    while parser_succeeds {
        match parser().parse(state) {
            Ok(success) => {
                results.push(success.get_result());

                if let Err(failure) = separator().parse(state) {
                    if failure.is_fatal() {
                        return Err(failure);
                    }
                    parser_succeeds = false;
                }
            },
            Err(failure) => {
                if failure.is_fatal() {
                    return Err(failure);
                }
                parser_succeeds = false;
            },
        }
    }

    Ok(results)
}