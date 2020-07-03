pub mod many;
pub mod sep_by;
pub mod many_till;
pub mod pipe;

use super::{ParserState, ParserSuccess, ParserFailure, ParserResult, Parser};

/// ```choice``` takes a Vector of parsers and applies them one at a time until one of the parsers succeeds. If none of the parsers succeed, ```choice``` returns an error. If any of the parsers fail with a fatal error, ```choice``` returns a fatal error.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new("nerds".to_string(), Position::new(1, 6, 5)));
/// 
/// let actual =
///     choice(vec![
///         p_string("hello".to_string()),
///         p_string("goodbye".to_string()),
///         p_string("nerds".to_string())
///     ]).run("nerds".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn choice<T>(parsers: Vec<Parser<T>>) -> Parser<T> {
    choice_l(parsers, "value satisfying choice".to_string())
}

/// ```choice_l``` provides the same functionality as ```choice``` however it allows for a custom error message, making it easier to determine where the parser failed.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Err(ParserFailure::new_err("custom error msg".to_string(), None, Position::new(1, 1, 0)));
/// 
/// let actual =
///     choice_l(
///         vec![
///             p_string("hello".to_string()),
///             p_string("goodbye".to_string()),
///             p_string("nerds".to_string())
///         ],
///         "custom error msg".to_string()).run("world".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn choice_l<T>(parsers: Vec<Parser<T>>, label: String) -> Parser<T> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                for p in parsers.into_iter() {
                    match p.parse(state) {
                        Ok(success) => {
                            return Ok(success)
                        },
                        Err(failure) => {
                            if failure.is_fatal() {
                                return Err(failure)
                            }

                            continue;
                        },
                    } 
                }

                Err(ParserFailure::new_err(
                    label,
                    None,
                    state.get_position()
                ))
            }
        );

    Parser::new(parser_fn)
}

/// ```attempt``` applies the parser provided as an argument and if the parser fails with a fatal error after changing the parser state, ```attempt``` reverts the state back to before the parser was applied and returns an error instead of a fatal error. The position property of the ParserFailure struct is indicative of where the parser failed, not of the current position of the ParserState struct.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let parser = p_u32().and(p_string("abc".to_string()));
/// 
/// let expected = Err(ParserFailure::new_err("abc".to_string(), Some("def".to_string()), Position::new(1, 4, 3)));
/// 
/// let actual = attempt(parser).run("123def".to_string());
/// 
/// assert_eq!(expected, actual);
/// ```
pub fn attempt<T>(parser: Parser<T>) -> Parser<T>
where T: 'static
{
    let parser_fn = 
        Box::new(
            move |state: &mut ParserState| {
                state.mark();
                match parser.parse(state) {
                    Ok(success) => {
                        state.remove_mark();
                        Ok(success)
                    },
                    Err(failure) => {
                        state.revert();
                        Err(failure.to_err())
                    },
                }
            }
        );

    Parser::new(parser_fn)
}