use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// `many_till` takes two parsers and applies the first parser (`many_parser`) repeatedly until the second parser (`end_parser`) succeeds. Once the 
/// `end_parser` succeeds, then all values parsed by the `many_parser` are returned in a Vector as a `ParserSuccess`.
/// If the `many_parser` fails on the first attempt and the `end_parser` succeeds, then `many_till` will return a `ParserSuccess` with an empty Vector.
/// 
/// # Errors
/// `many_till` will return a `ParserFailure` if the `many_parser` fails with a `FatalError` or if the `many_parser` fails and is followed by a failing `end_parser`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_true() -> Parser<bool> {
/// #     p_string(String::from("true"))
/// #         .then_return(true)
/// # }
/// #
/// let expected = Ok(ParserSuccess::new(
///     vec![true, true, true],
///     Position::new(1, 16, 15)
/// ));
/// 
/// let actual = many_till(p_true, p_u32)
///     .run(String::from("truetruetrue123"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn many_till<T, U>(many_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results = apply_parsers(many_parser, end_parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// `many_1_till` takes two parsers and applies the first parser (`many_parser`) repeatedly until the second parser (`end_parser`) succeeds. Once the 
/// `end_parser` succeeds, then all values parsed by the `many_parser` are returned in a Vector as a `ParserSuccess`.
/// 
/// # Errors
/// `many_1_till` will return a `ParserFailure` if the `many_parser` fails with a `FatalError` or if the `many_parser` fails and is followed by a failing `end_parser`.
/// Unlike `many_till`, if the `many_parser` fails on the first attempt and the `end_parser` succeeds, `many_1_till` will return a `ParserFailure`. The `many_parser` must
/// succeed at least once for `many_1_till` to return a `ParserSuccess`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_true() -> Parser<bool> {
/// #     p_string(String::from("true"))
/// #         .then_return(true)
/// # }
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("true"), 
///     Some(String::from("1234")), 
///     Position::new(1, 1, 0)
/// ));
/// 
/// let actual = many_1_till(p_true, p_u32)
///     .run(String::from("1234"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn many_1_till<T, U>(many_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results = apply_parsers_1(many_parser, end_parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// `skip_many_till` takes two parsers and applies the first parser (`many_parser`) repeatedly until the second parser (`end_parser`) succeeds. Once the 
/// `end_parser` succeeds, `()` is returned as a `ParserSuccess`.
/// If the `many_parser` fails on the first attempt and the `end_parser` succeeds, then `skip_many_till` will still return a `ParserSuccess` of `()`.
/// 
/// # Errors
/// `skip_many_till` will return a `ParserFailure` if the `many_parser` fails with a `FatalError` or if the `many_parser` fails and is followed by a failing `end_parser`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_true() -> Parser<bool> {
/// #     p_string(String::from("true"))
/// #         .then_return(true)
/// # }
/// #
/// let expected = Ok(ParserSuccess::new(
///     (), 
///     Position::new(1, 16, 15)
/// ));
/// 
/// let actual = skip_many_till(p_true, p_u32)
///     .run(String::from("truetruetrue123"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_many_till<T, U>(many_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parsers(many_parser, end_parser, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// `skip_many_1_till` takes two parsers and applies the first parser (`many_parser`) repeatedly until the second parser (`end_parser`) succeeds. Once the 
/// `end_parser` succeeds,  `()` is returned as a `ParserSuccess`.
/// 
/// # Errors
/// `skip_many_1_till` will return a `ParserFailure` if the `many_parser` fails with a `FatalError` or if the `many_parser` fails and is followed by a failing `end_parser`.
/// Unlike `skip_many_till`, if the `many_parser` fails on the first attempt and the `end_parser` succeeds, `skip_many_1_till` will return a `ParserFailure`. The `many_parser` must
/// succeed at least once for `skip_many_1_till` to return a `ParserSuccess`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_true() -> Parser<bool> {
/// #     p_string(String::from("true"))
/// #         .then_return(true)
/// # }
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("true"), 
///     Some(String::from("1234")), 
///     Position::new(1, 1, 0)
/// ));
/// 
/// let actual = skip_many_1_till(p_true, p_u32)
///     .run(String::from("1234"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_many_1_till<T, U>(many_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parsers_1(many_parser, end_parser, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

fn apply_parsers<T, U>(many_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    let mut results: Vec<T> = Vec::new();
    let mut end_parser_succeeds = false;

    while !end_parser_succeeds {
        match many_parser().parse(state) {
            Ok(success) => {
                end_parser_succeeds = apply_end_parser(end_parser, state)?;
                results.push(success.get_result())
            },
            Err(failure) => {
                return if results.len() == 0 && !failure.is_fatal() {
                    end_parser().parse(state).and(Ok(results))
                } else {
                    Err(failure.to_fatal_err())
                }
            },
        }
    }

    Ok(results)
}

fn apply_parsers_1<T, U>(many_parser: fn() -> Parser<T>, end_parser: fn() -> Parser<U>, state: &mut ParserState) -> Result<Vec<T>, ParserFailure> {
    let mut results: Vec<T> = Vec::new();
    let mut end_parser_succeeds = false;

    while !end_parser_succeeds {
        match many_parser().parse(state) {
            Ok(success) => {
                end_parser_succeeds = apply_end_parser(end_parser, state)?;
                results.push(success.get_result())
            },
            Err(failure) => {
                return if results.len() == 0 {
                    Err(failure)
                } else {
                    Err(failure.to_fatal_err())
                }
            },
        }
    }

    Ok(results)
}

fn apply_end_parser<T>(end_parser: fn() -> Parser<T>, state: &mut ParserState) -> Result<bool, ParserFailure> {
    match end_parser().parse(state) {
        Ok(_) => {
            Ok(true)
        },
        Err(failure) => {
            if failure.is_fatal() {
                return Err(failure)
            }

            Ok(false)
        }
    }
}