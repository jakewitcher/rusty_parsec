use super::Parser;
use super::state::ParserState;
use super::error::ParserError;

use num_traits::{Float, PrimInt};

pub fn p_char(target_char: char) -> Parser<char> {
    Box::new(
        move |parser_state: &mut ParserState| {
            let source_char = 
                parser_state.get_slice(target_char.len_utf8())
                    .and_then(|slice|slice.chars().next());

            match source_char {
                Some(c) if c == target_char => {
                    parser_state.move_input_state_forward(target_char.len_utf8());
                    Ok(c)
                },
                Some(c) => {
                    let err = ParserError::new(
                        parser_state.get_line_number(),
                        parser_state.get_column_number(),
                        target_char.to_string(),
                        Some(c.to_string())
                    );

                    Err(err)
                },
                _ => {
                    let err = ParserError::new(
                        parser_state.get_line_number(),
                        parser_state.get_column_number(),
                        target_char.to_string(),
                        None
                    );

                    Err(err)
                }
            }
        }
    )
}

pub fn p_string(target_string: String) -> Parser<String> {
    Box::new(
        move |parser_state: &mut ParserState| {
            let source_string = parser_state.get_slice(target_string.len());

            match source_string {
                Some(source) => {
                    if target_string == source {
                        parser_state.move_input_state_forward(target_string.len());
                        Ok(String::from(source))
                    } else {
                        let err = ParserError::new(
                            parser_state.get_line_number(),
                            parser_state.get_column_number(),
                            target_string,
                            Some(source)
                        );
        
                        Err(err)
                    }
                },
                None => {
                    let err = ParserError::new(
                        parser_state.get_line_number(),
                        parser_state.get_column_number(),
                        target_string,
                        None
                    );
    
                    Err(err)
                }
            }
        }
    )
}

pub fn p_i32() -> Parser<i32> {
    p_int(Box::new(|slice: String| slice.parse::<i32>()))
}

pub fn p_i64() -> Parser<i64> {
    p_int(Box::new(|slice: String| slice.parse::<i64>()))
}

pub fn p_u32() -> Parser<u32> {
    p_int(Box::new(|slice: String| slice.parse::<u32>()))
}

pub fn p_u64() -> Parser<u64> {
    p_int(Box::new(|slice: String| slice.parse::<u64>()))
}

pub fn p_isize() -> Parser<isize> {
    p_int(Box::new(|slice: String| slice.parse::<isize>()))
}

pub fn p_iusize() -> Parser<usize> {
    p_int(Box::new(|slice: String| slice.parse::<usize>()))
}

fn p_int<T>(parse_num: Box<dyn Fn(String) -> Result<T, std::num::ParseIntError>>) -> Parser<T> 
where T: PrimInt + 'static
{
    Box::new(
        move |parser_state: &mut ParserState| {
            let chars: Vec<char> = 
                parser_state.get_remaining_input()
                    .chars().collect();
            
            let mut int_char_count = 0;

            let err = ParserError::new(
                parser_state.get_line_number(),
                parser_state.get_column_number(),
                "integral value".to_string(),
                None
            );

            for c in chars  {
                if c.is_numeric() || c == '-' && int_char_count == 0 {
                    int_char_count += c.len_utf8();
                } else {
                    break;
                }
            }

            if int_char_count == 0 {
                Err(err)
            } else {
                let int_slice = parser_state.get_slice(int_char_count);

                match int_slice {
                    Some(slice) => {
                        let integer_result = parse_num(slice);
                        match integer_result {
                            Ok(integer) => {
                                parser_state.move_input_state_forward(int_char_count);
                                Ok(integer)
                            },
                            _ => Err(err)
                        }
                    },
                    _ => Err(err)
                }
            }
        }
    )
}

pub fn p_f32() -> Parser<f32> {
    p_float(Box::new(|slice: String| slice.parse::<f32>()))
}

pub fn p_f64() -> Parser<f64> {
    p_float(Box::new(|slice: String| slice.parse::<f64>()))
}

fn p_float<T>(parse_num: Box<dyn Fn(String) -> Result<T, std::num::ParseFloatError>>) -> Parser<T> 
where T: Float + 'static
{
    Box::new(
        move |parser_state: &mut ParserState| {
            let chars: Vec<char> = 
                parser_state.get_remaining_input()
                    .chars().collect();
            
            let mut int_char_count = 0;

            let err = ParserError::new(
                parser_state.get_line_number(),
                parser_state.get_column_number(),
                "floating point value".to_string(),
                None
            );

            let mut has_decimal_point = false;

            for c in chars  {
                if c.is_numeric() || c == '-' && int_char_count == 0 {
                    int_char_count += c.len_utf8();
                } else if c == '.' && has_decimal_point == false {
                    has_decimal_point = true;
                    int_char_count += c.len_utf8();
                } else {
                    break;
                }
            }

            if int_char_count == 0 {
                Err(err)
            } else {
                let int_slice = parser_state.get_slice(int_char_count);

                match int_slice {
                    Some(slice) => {
                        let float_result = parse_num(slice);
                        match float_result {
                            Ok(float) if float.is_infinite() => Err(err),
                            Ok(float) => {
                                parser_state.move_input_state_forward(int_char_count);
                                Ok(float)
                            },
                            _ => Err(err)
                        }
                    },
                    _ => Err(err)
                }
            }
        }
    )
}

pub fn ws() -> Parser<()> {
    Box::new(
        move |parser_state: &mut ParserState| {
            let chars: Vec<char> = 
                parser_state.get_remaining_input()
                    .chars().collect();
            
            let mut ws_char_count = 0;

            for c in chars  {
                if c.is_ascii_whitespace() {
                    ws_char_count += c.len_utf8();
                } else {
                    break;
                }
            }

            parser_state.move_input_state_forward(ws_char_count);
            
            Ok(())
        }
    )
}

