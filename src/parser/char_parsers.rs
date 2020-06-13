use super::{ParserState, ParserSuccess, ParserFailure, Parser, Severity};

use num_traits::{Float, PrimInt};

pub fn p_char(target: char) -> Parser<char> {
    char_return(target, target)
}

pub fn skip_char(target: char) -> Parser<()> {
    char_return(target, ())
}

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
                        Err(ParserFailure::new(
                            target.to_string(),
                            Some(c.to_string()),
                            Severity::Error,
                            state.get_position()
                        ))
                    },
                    None => {
                        Err(ParserFailure::new(
                            target.to_string(),
                            None,
                            Severity::Error,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

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
                        Err(ParserFailure::new(
                            "char satisfying the condition".to_string(),
                            None,
                            Severity::Error,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

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

pub fn p_string(target: String) -> Parser<String> {
    string_return(target.clone(), target)
}

pub fn skip_string(target: String) -> Parser<()> {
    string_return(target, ())
}

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
                        Err(ParserFailure::new(
                            target,
                            Some(s),
                            Severity::Error,
                            state.get_position()
                        ))
                    },
                    None => {
                        Err(ParserFailure::new(
                            target,
                            None,
                            Severity::Error,
                            state.get_position()
                        ))
                    },
                }
            }
        );

    Parser::new(parser_fn)
}

pub fn p_i32() -> Parser<i32> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<i32>()))
}

pub fn p_i64() -> Parser<i64> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<i64>()))
}

pub fn p_u32() -> Parser<u32> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<u32>()))
}

pub fn p_u64() -> Parser<u64> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<u64>()))
}

pub fn p_isize() -> Parser<isize> {
    p_int(Box::new(|maybe_int: String| maybe_int.parse::<isize>()))
}

pub fn p_iusize() -> Parser<usize> {
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
                        Err(ParserFailure::new(
                            "integral value".to_string(),
                            None,
                            Severity::Error,
                            state.get_position())
                        ),
                }
            }
        );

    Parser::new(parser_fn)
}

pub fn p_f32() -> Parser<f32> {
    p_float(Box::new(|maybe_float: String| maybe_float.parse::<f32>()))
}

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
                        Err(ParserFailure::new(
                            "floating point value".to_string(),
                            None,
                            Severity::Error,
                            state.get_position())
                        ),
                }
            }
        );

    Parser::new(parser_fn)
}

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

