use super::{ParserState, ParserSuccess, ParserFailure, Parser};

/// ```sep_by``` applies the parser returned by ```get_parser``` followed by the parser returned by ```get_separator``` repeatedly until either parser fails. A Vector of the parsed values is returned once the parser fails. The parsers must fail without changing the parser state or ```sep_by``` will return a fatal error.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_comma() -> Parser<char> {
///     p_char(',')
/// }
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         vec![1,2,3], 
///         Position::new(1, 6, 5))
///     );
/// 
/// let actual = sep_by(p_u32, p_comma).run("1,2,3".to_string());
/// 
/// assert_eq!(expected, actual);
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

/// ```sep_by_1``` is similar to ```sep_by``` however ```sep_by_1``` must succeed at least once. A Vector of the parsed values is returned once the parser fails. The parsers must fail without changing the parser state or ```sep_by_1``` will return a fatal error.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_comma() -> Parser<char> {
///     p_char(',')
/// }
/// 
/// let expected = 
///     Err(ParserFailure::new_err(
///         "value satisfying parser at least once".to_string(), 
///         None,
///         Position::new(1, 1, 0))
///     );
/// 
/// let actual = sep_by_1(p_u32, p_comma).run("A,B,C".to_string());
/// 
/// assert_eq!(expected, actual);
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

/// ```skip_sep_by``` applies the parser returned by ```get_parser``` followed by the parser returned by ```get_separator``` repeatedly until either parser fails and returns ```()``` as the result. The parsers must fail without changing the parser state or ```skip_sep_by``` will return a fatal error.
/// 
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_comma() -> Parser<char> {
///     p_char(',')
/// }
/// 
/// let expected = 
///     Ok(ParserSuccess::new(
///         (), 
///         Position::new(1, 6, 5))
///     );
/// 
/// let actual = skip_sep_by(p_u32, p_comma).run("1,2,3".to_string());
/// 
/// assert_eq!(expected, actual);
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

/// ```skip_sep_by_1``` is similar to ```sep_by``` however ```skip_sep_by_1``` must succeed at least once before returning ```()```. The parsers must fail without changing the parser state or ```sep_by_1``` will return a fatal error.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// fn p_comma() -> Parser<char> {
///     p_char(',')
/// }
/// 
/// let expected = 
///     Err(ParserFailure::new_err(
///         "value satisfying parser at least once".to_string(), 
///         None,
///         Position::new(1, 1, 0))
///     );
/// 
/// let actual = skip_sep_by_1(p_u32, p_comma).run("A,B,C".to_string());
/// 
/// assert_eq!(expected, actual);
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