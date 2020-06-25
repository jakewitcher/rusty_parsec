use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// ```many``` applies the parser returned by ```p_factory``` repeatedly until it fails. A Vector of the successfully parsed values is returned. The parser must fail without changing the parser state or ```many``` will return a fatal error.
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
pub fn many<T>(p_factory: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let results: Vec<T> = apply_parser(p_factory, state)?;
                Ok(ParserSuccess::new(results, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn many_1<T>(p_factory: fn() -> Parser<T>) -> Parser<Vec<T>> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match p_factory().parse(state) {
                    Ok(success) => {
                        let mut results = apply_parser(p_factory, state)?;
                        results.insert(0, success.get_result());
                        Ok(ParserSuccess::new(results, state.get_position()))
                    },
                    Err(failure) => Err(failure),
                }
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many<T>(p_factory: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let _ = apply_parser(p_factory, state)?;
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

pub fn skip_many_1<T>(p_factory: fn() -> Parser<T>) -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match p_factory().parse(state) {
                    Ok(_) => {
                        let _ = apply_parser(p_factory, state)?;
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