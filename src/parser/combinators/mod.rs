pub mod many;
pub mod sep_by;
pub mod many_till;
pub mod pipe;

use super::{ParserState, ParserSuccess, ParserFailure, ParserResult, Parser};

/// `choice` takes a Vector of parsers and applies each one in sequence until one of the parsers returns a `ParserSuccess`. Each parser in the 
/// Vector must return a `ParserSuccess` with the same value type.
/// 
/// # Errors
/// `choice` will return a `ParserFailure` with the `Error` severity if all parsers in the Vector fail, and will return a `FatalError` if any of the 
/// parsers fail after changing the parser state.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Ok(ParserSuccess::new(
///     String::from("nerds"), 
///     Position::new(1, 6, 5)
/// ));
/// 
/// let actual = choice(vec![
///     p_string(String::from("hello")),
///     p_string(String::from("goodbye")),
///     p_string(String::from("nerds"))
/// ]).run(String::from("nerds"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn choice<T>(parsers: Vec<Parser<T>>) -> Parser<T> {
    choice_l(parsers, "value satisfying choice".to_string())
}

/// `choice_l` works exactly like `choice` with one difference, it allows for a custom error message to be attached to the parser. 
/// This custom error message can make it easier to determine where the parser failed.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("custom error message"), 
///     None, 
///     Position::new(1, 1, 0)
/// ));
/// 
/// let actual = choice_l(
///     vec![
///         p_string(String::from("hello")),
///         p_string(String::from("goodbye")),
///         p_string(String::from("nerds"))
///     ],
///     String::from("custom error message")
/// ).run(String::from("world"));
/// 
/// assert_eq!(actual, expected);
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

/// `attempt` applies the the `parser` argument and if fails having changed the parser state, `attempt` reverts the state to point before the `parser`
/// was applied, returning a `ParserFailure` with an `Error` severity instead of a `FatalError`.
/// 
/// # Errors
/// `attempt` will only every return a `ParserFailure` with an `Error` severity. If the `parser` fails having changed the parser state, because it is
/// able to revert the current `Position` as well as the history of the `ParserState` exactly as it was before the `parser` was applied, the parser state
/// is no longer considered changed and therefore `attempt` can safely return an `Error` severity.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// # let p_u32_and_abc = p_u32()
/// #     .and(p_string(String::from("abc")));
/// #
/// let expected = Err(ParserFailure::new_err(
///     String::from("abc"),
///     Some(String::from("def")),
///     Position::new(1, 4, 3)
/// ));
/// 
/// let actual = attempt(p_u32_and_abc)
///     .run(String::from("123def"));
/// 
/// assert_eq!(actual, expected);
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