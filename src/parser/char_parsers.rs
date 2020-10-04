use super::{ParserState, ParserSuccess, ParserFailure, Parser};

use num_traits::{Float, PrimInt};

/// `p_char` takes a single character as the `target` and returns a parser. When the parser is applied to the input string, it will 
/// return the character parsed as a `ParserSuccess` if the next character in the input string matches the `target`. 
/// 
/// # Errors
/// `p_char` will return a `ParserFailure` with a severity of `Error` if the next character in the input string does not match the `target`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Ok(ParserSuccess::new(
///     'a', 
///     Position::new(1, 2, 1)
/// ));
/// 
/// let actual = p_char('a')
///     .run(String::from("abc"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_char(target: char) -> Parser<char> {
    char_return(target, target)
}

/// `skip_char` takes a single character as the `target` and returns a parser. When the parser is applied to the input string, it will 
/// return a `()` as a `ParserSuccess` if the next character in the input string matches the `target`. 
/// 
/// # Errors
/// `skip_char` will return a `ParserFailure` with a severity of `Error` if the next character in the input string does not match the `target`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Ok(ParserSuccess::new(
///     (), 
///     Position::new(1, 2, 1)
/// ));
/// 
/// let actual = skip_char('a')
///     .run(String::from("abc"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_char(target: char) -> Parser<()> {
    char_return(target, ())
}


/// `char_return` takes a single character as the `target` and returns a parser. When the parser is applied to the input string, it will 
/// return the provided `return_value` as a `ParserSuccess` if the next character in the input string matches the `target`. 
/// 
/// # Errors
/// `char_return` will return a `ParserFailure` with a severity of `Error` if the next character in the input string does not match the `target`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Ok(ParserSuccess::new(
///     true, 
///     Position::new(1, 2, 1)
/// ));
/// 
/// let actual = char_return('a', true)
///     .run(String::from("abc"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn char_return<T>(target: char, return_value: T) -> Parser<T> 
where T: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match state.get_remaining_input().chars().next() {
                    Some(c) if c == target => {
                        state.move_state_forward(target.len_utf8());
                        Ok(ParserSuccess::new(return_value, state.get_position()))
                    },
                    Some(c) => {
                        Err(ParserFailure::new_err(
                            target.to_string(),
                            Some(c.to_string()),
                            state.get_position()
                        ))
                    },
                    None => {
                        Err(ParserFailure::new_err(
                            target.to_string(),
                            None,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

/// `satisfy` takes a function (`f`) of type `(char) -> bool` and returns a parser. When the parser is applied to the input string, it will 
/// return the character parsed as a `ParserSuccess` if the next character in the input string returns true when applied to the function `f`.
/// 
/// # Errors
/// `satisfy` will return a `ParserFailure` with a severity of `Error` if the next character in the input string returns false when applied
/// to the function `f`.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Ok(ParserSuccess::new(
///     'c', 
///     Position::new(1, 2, 1)
/// ));
/// 
/// let actual = satisfy(Box::new(|c:char|c.is_ascii_lowercase()))
///     .run("cat".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn satisfy(f: Box<dyn Fn (char) -> bool>) -> Parser<char> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match state.get_remaining_input().chars().next() {
                    Some(c) if f(c) => {
                        state.move_state_forward(c.len_utf8());
                        Ok(ParserSuccess::new(c, state.get_position()))
                    },
                    _ => {
                        Err(ParserFailure::new_err(
                            "char satisfying the condition".to_string(),
                            None,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

/// `many_satisfy` takes a function (`f`) of type `(char) -> bool` and returns a parser. When the parser is applied to the input string, it will 
/// return the character parsed as a `ParserSuccess` if the next character in the input string returns true when applied to the function `f`. Unlike
/// `satsify`, the parser will continue to apply the function `f` on each subsequent character in sequence until the function `f` returns false.
/// All successfully parsed characters are collected into a single string and returned as the value of a `ParserSuccess`.
/// 
/// # Errors
/// `many_satisfy` will never return an error. If the first character consumed returns false when applied to the function `f`, `many_satisfy` will
/// return a `ParserSuccess` with an empty string as the value and the parser state unchanged.
/// 
/// # Examples
/// 
/// ```
/// # use rusty_parsec::*;
/// #
/// let expected = Ok(ParserSuccess::new(
///     String::from("aaa"), 
///     Position::new(1, 4, 3)
/// ));
/// 
/// let actual = many_satisfy(Box::new(|c:char|c == 'a'))
///     .run(String::from("aaabbb"));
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn many_satisfy(f: Box<dyn Fn (char) -> bool>) -> Parser<String> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut count = 0;
                for c in state.get_remaining_input().chars() {
                    if f(c) {
                        count += c.len_utf8();
                    } else {
                        break;
                    }
                }
                let result = state.get_slice(count).unwrap_or(String::new());
                state.move_state_forward(count);
                Ok(ParserSuccess::new(result, state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

/// `p_string` takes a String as an argument and returns a parser success with the expected String value if the next string slice of the input string is a match, otherwise it returns a parser failure.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new("hello".to_string(), Position::new(1, 6, 5)));
/// 
/// let actual = 
///     p_string("hello".to_string())
///         .run("hello, world".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_string(target: String) -> Parser<String> {
    string_return(target.clone(), target)
}

/// `skip_string` takes a String as an argument and returns a parser success of `()` if the next string slice of the input string is a match, otherwise it returns a parser failure.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new((), Position::new(1, 6, 5)));
/// 
/// let actual =  
///     skip_string("hello".to_string())
///         .run("hello, world".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn skip_string(target: String) -> Parser<()> {
    string_return(target, ())
}

/// `string_return` takes a String as an argument and returns a parser success of the value supplied as the second argument of the function if the next string slice of the input string is a match, otherwise it returns a parser failure.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(true, Position::new(1, 6, 5)));
/// 
/// let actual = 
///     string_return("hello".to_string(), true)
///         .run("hello, world".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn string_return<T>(target: String, return_value: T) -> Parser<T> 
where T: 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                match state.get_slice(target.len()) {
                    Some(s) if s == target => {
                        state.move_state_forward(target.len());
                        Ok(ParserSuccess::new(return_value, state.get_position()))
                    },
                    Some(s) => {
                        Err(ParserFailure::new_err(
                            target,
                            Some(s),
                            state.get_position()
                        ))
                    },
                    None => {
                        Err(ParserFailure::new_err(
                            target,
                            None,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

/// `p_i32` tries to parse the input string as an integer and if it succeeds, returns the result as an i32 integer.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));
/// 
/// let actual = 
///     p_u32().run("123abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_i32() -> Parser<i32> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<i32>()))
}

/// `p_i64` tries to parse the input string as an integer and if it succeeds, returns the result as an i64 integer.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));
/// 
/// let actual = 
///     p_i64().run("123abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_i64() -> Parser<i64> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<i64>()))
}

/// `p_u32` tries to parse the input string as an integer and if it succeeds, returns the result as an u32 integer.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));
/// 
/// let actual = 
///     p_u32().run("123abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_u32() -> Parser<u32> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<u32>()))
}

/// `p_u64` tries to parse the input string as an integer and if it succeeds, returns the result as an u64 integer.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));
/// 
/// let actual = 
///     p_u64().run("123abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_u64() -> Parser<u64> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<u64>()))
}

/// `p_isize` tries to parse the input string as an integer and if it succeeds, returns the result as an isize integer.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));
/// 
/// let actual = 
///     p_isize().run("123abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_isize() -> Parser<isize> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<isize>()))
}

/// `p_usize` tries to parse the input string as an integer and if it succeeds, returns the result as an usize integer.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123, Position::new(1, 4, 3)));
/// 
/// let actual = 
///     p_usize().run("123abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_usize() -> Parser<usize> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<usize>()))
}

fn p_int<T>(parse_num: Box<dyn Fn(String) -> Result<T, std::num::ParseIntError>>) -> Parser<T> 
where T: PrimInt + 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut count = 0;

                for c in state.get_remaining_input().chars() {
                    if c.is_numeric() || c == '-' && count == 0 {
                        count += c.len_utf8();
                    } else {
                        break;
                    }
                }

                match state.get_slice(count).map(|s| parse_num(s)) {
                    Some(Ok(int)) => {
                        state.move_state_forward(count);
                        Ok(ParserSuccess::new(int, state.get_position()))
                    },
                    _ =>
                        Err(ParserFailure::new_err(
                            "integral value".to_string(),
                            None,
                            state.get_position())
                        ),
                }
            }
        );

    Parser::new(parser_fn)
}

/// `p_f32` tries to parse the input string as a floating point number and if it succeeds, returns the result as an f32 floating point.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123.35, Position::new(1, 7, 6)));
/// 
/// let actual = 
///     p_f32().run("123.35abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_f32() -> Parser<f32> {
    p_float(Box::new(|maybe_float: String| maybe_float.parse::<f32>()))
}

/// `p_f64` tries to parse the input string as a floating point number and if it succeeds, returns the result as an f64 floating point.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(123.35, Position::new(1, 7, 6)));
/// 
/// let actual = 
///     p_f64().run("123.35abc".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn p_f64() -> Parser<f64> {
    p_float(Box::new(|maybe_float: String| maybe_float.parse::<f64>()))
}

fn p_float<T>(parse_num: Box<dyn Fn(String) -> Result<T, std::num::ParseFloatError>>) -> Parser<T> 
where T: Float + 'static
{
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut count = 0;
                let mut has_decimal_point = false;

                for c in state.get_remaining_input().chars() {
                    if c.is_numeric() || c == '-' && count == 0 {
                        count += c.len_utf8();
                    } else if c == '.' && has_decimal_point == false {
                        has_decimal_point = true;
                        count += c.len_utf8();
                    } else {
                        break;
                    }
                }

                match state.get_slice(count).map(|s| parse_num(s)) {
                    Some(Ok(float)) if float.is_finite() => {
                        state.move_state_forward(count);
                        Ok(ParserSuccess::new(float, state.get_position()))
                    },
                    _ =>
                        Err(ParserFailure::new_err(
                            "floating point value".to_string(),
                            None,
                            state.get_position())
                        ),
                }
            }
        );

    Parser::new(parser_fn)
}

/// `ws` parses zero or more successive whitespace characters, returning `()` as the parser result.
/// 
/// # Examples
/// 
/// ```
/// use rusty_parsec::*;
/// 
/// let expected = Ok(ParserSuccess::new(('a', 'b'), Position::new(3, 2, 10)));
/// 
/// let actual = 
///     ws().take_next(p_char('a'))
///         .take_prev(ws())
///         .and(p_char('b'))
///         .run("  \na\t  \r\nb".to_string());
/// 
/// assert_eq!(actual, expected);
/// ```
pub fn ws() -> Parser<()> {
    let parser_fn =
        Box::new(
            move |state: &mut ParserState| {
                let mut count = 0;

                for c in state.get_remaining_input().chars()  {
                    if c.is_ascii_whitespace() {
                        count += c.len_utf8();
                    } else {
                        break;
                    }
                }

                state.move_state_forward(count);
                Ok(ParserSuccess::new((), state.get_position()))
            }
        );

    Parser::new(parser_fn)
}

