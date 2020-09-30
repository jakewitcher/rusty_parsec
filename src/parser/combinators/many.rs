use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// `many` applies the parser `many_parser` repeatedly until it fails, returning the parsed values in a Vector as a `ParserSuccess`.
/// If the `many_parser` fails on the first attempt then `many` will return a `ParserSuccess` with an empty Vector.
/// 
/// # Errors
/// `many` will return a `ParserFailure` if the `many_parser` fails with a `FatalError`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_hello() -> Parser<String> {
/// #     p_string(String::from("hello"))
/// # }
/// #
/// let expected = Ok(ParserSuccess::new(
///     vec![ String::from("hello"), String::from("hello"), String::from("hello")], 
///     Position::new(1, 16, 15)
/// ));
/// 
/// let actual = many(p_hello)
///     .run(String::from("hellohellohello"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn many<T>(many_parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results: Vec<T> = apply_parser(many_parser, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// `many_1` applies the parser `many_parser` repeatedly until it fails, returning the parsed values in a Vector as a `ParserSuccess`.
/// 
/// # Errors
/// `many_1` will return a `ParserFailure` if the `many_parser` fails with a `FatalError`. Unlike `many`, if the `many_parser` fails on the first attempt
/// this will also cause `many_1` to return a `ParserFailure`. The `many_parser` must succeed at least once for `many_1` to return a `ParserSuccess`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_hello() -> Parser<String> {
/// #     p_string(String::from("hello"))
/// # }
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("hello"), 
///     Some(String::from("goodb")),
///     Position::new(1, 1, 0)
/// ));
/// 
/// let actual = many_1(p_hello)
///     .run(String::from("goodbye"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn many_1<T>(many_parser: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match many_parser().parse(state) {
                    Ok(success) => {
                        let mut results = apply_parser(many_parser, state)?;
                        results.insert(0, success.get_result());
                        Ok(ParserSuccess::new(results, state.get_position()))
                    },
                    Err(failure) => Err(failure),
                }
            }
        );

    Parser::new(parser_fn)
}

/// `skip_many` applies the parser `many_parser` repeatedly until it fails, returning a `ParserSuccess` of `()`.
/// If the `many_parser` fails on the first attempt then `skip_many` will still return a `ParserSuccess` of `()`.
/// 
/// # Errors
/// `skip_many` will return a `ParserFailure` if the `many_parser` fails with a `FatalError`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_hello() -> Parser<String> {
/// #     p_string(String::from("hello"))
/// # }
/// #
/// let expected = Ok(ParserSuccess::new(
///     (), 
///     Position::new(1, 16, 15)
/// ));
/// 
/// let actual = skip_many(p_hello)
///     .run(String::from("hellohellohello"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_many<T>(many_parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parser(many_parser, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// `skip_many_1` applies the parser `many_parser` repeatedly until it fails, returning a `ParserSuccess` of `()`.
/// 
/// # Errors
/// `skip_many_1` will return a `ParserFailure` if the `many_parser` fails with a `FatalError`. Unlike `skip_many`, if the `many_parser` fails on the first attempt
/// this will also cause `skip_many_1` to return a `ParserFailure`. The `many_parser` must succeed at least once for `skip_many_1` to return a `ParserSuccess`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # fn p_hello() -> Parser<String> {
/// #     p_string(String::from("hello"))
/// # }
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("hello"), 
///     Some(String::from("goodb")),
///     Position::new(1, 1, 0)
/// ));
/// 
/// let actual = skip_many_1(p_hello)
///     .run(String::from("goodbye"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_many_1<T>(many_parser: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match many_parser().parse(state) {
                    Ok(_) => {
                        let _ = apply_parser(many_parser, state)?;
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