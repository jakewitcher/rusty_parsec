use rusty_parsec::*;

#[test]
fn p_char_a_char_succeeds() {        
    let expected = Ok(ParserSuccess::new(
        'a', 
        Position::new(1, 2, 1)
    ));

    let actual = p_char('a')
        .run(String::from("abc"));
    
    assert_eq!(actual, expected);
}

#[test]
fn p_char_b_char_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("b"), 
        Some(String::from("a")), 
        Position::new(1, 1, 0)
    ));
    
    let actual = p_char('b')
        .run(String::from("abc"));

    assert_eq!(actual, expected);
}

#[test]
fn satisfy_is_ascii_lowercase_succeeds() {
    let expected = Ok(ParserSuccess::new(
        'c', 
        Position::new(1, 2, 1)
    ));

    let actual = satisfy(Box::new(|c:char|c.is_ascii_lowercase()))
        .run(String::from("cat"));

    assert_eq!(actual, expected);
}

#[test]
fn satisfy_is_ascii_lowercase_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("char satisfying the condition"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = satisfy(Box::new(|c:char|c.is_ascii_lowercase()))
        .run(String::from("Cat"));

    assert_eq!(actual, expected);
}

#[test]
fn many_satisfy_a_char_succeeds() {
    let expected = Ok(ParserSuccess::new(
        String::from("aaa"), 
        Position::new(1, 4, 3)
    ));

    let actual = many_satisfy(Box::new(|c:char|c == 'a'))
        .run(String::from("aaabbb"));

    assert_eq!(actual, expected);
}

#[test]
fn many_satisfy_a_char_succeeds_when_no_values_returned_by_parser() {
    let expected = Ok(ParserSuccess::new(
        String::new(), 
        Position::new(1, 1, 0)
    ));

    let actual = many_satisfy(Box::new(|c:char|c == 'a'))
        .run(String::from("bbbaaa"));

    assert_eq!(actual, expected);
}

#[test]
fn p_string_hello_string_succeeds() {    
    let expected = Ok(ParserSuccess::new(
        String::from("hello"), 
        Position::new(1, 6, 5)
    ));

    let actual = p_string(String::from("hello"))
        .run(String::from("hello, world"));

    assert_eq!(actual, expected);
}

#[test]
fn p_string_hello_string_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        Some(String::from("chell")), 
        Position::new(1, 1, 0)
    ));
        
    let actual = p_string(String::from("hello"))
        .run(String::from("chello, world"));

    assert_eq!(actual, expected);
}

#[test]
fn p_string_hello_string_fails_with_error_when_input_is_too_short() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        None, 
        Position::new(1, 1, 0)
    ));
        
    let actual = p_string(String::from("hello"))
        .run(String::from("hell"));

    assert_eq!(actual, expected);
}

#[test]
fn ws_run_complex_series_of_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        ('a', 'b'), 
        Position::new(3, 2, 10)
    ));

    let actual = ws()
        .take_next(p_char('a'))
        .take_prev(ws())
        .and(p_char('b'))
        .run(String::from("  \na\t  \r\nb"));

    assert_eq!(actual, expected);
}

#[test]
fn ws_run_complex_series_of_parsers_fails_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("b"), 
        Some(String::from("c")), 
        Position::new(3, 1, 9)
    ));

    let actual = ws()
        .take_next(p_char('a'))
        .take_prev(ws())
        .and(p_char('b'))
        .run(String::from("  \na\t  \r\nc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_u32_three_digit_number_succeeds() {
    let expected = Ok(ParserSuccess::new(
        123, 
        Position::new(1, 4, 3)
    ));

    let actual = p_u32()
        .run(String::from("123abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_i32_negative_three_digit_number_succeeds() {
    let expected = Ok(ParserSuccess::new(
        -123, 
        Position::new(1, 5, 4)
    ));

    let actual = p_i32()
        .run(String::from("-123abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_i32_alphabetic_chars_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = p_i32()
        .run(String::from("abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_i32_integer_exceeds_i32_max_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = p_i32()
        .run(String::from("2147483900"));

    assert_eq!(actual, expected);
}

#[test]
fn p_i64_ten_digit_number_succeeds() {
    let expected = Ok(ParserSuccess::new(
        2147483900, 
        Position::new(1, 11, 10)
    ));

    let actual = p_i64()
        .run(String::from("2147483900"));

    assert_eq!(actual, expected);
}

#[test]
fn p_f32_decimal_number_succeeds() {
    let expected = Ok(ParserSuccess::new(
        123.35, 
        Position::new(1, 7, 6)
    ));

    let actual = p_f32()
        .run(String::from("123.35abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_f32_decimal_number_followed_by_period_succeeds() {
    let expected = Ok(ParserSuccess::new(
        123.35, 
        Position::new(1, 7, 6)
    ));

    let actual = p_f32()
        .run(String::from("123.35.abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_f32_negative_decimal_number_succeeds() {
    let expected = Ok(ParserSuccess::new(
        -123.35, 
        Position::new(1, 8, 7)
    ));

    let actual = p_f32()
        .run(String::from("-123.35abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_f32_alphabetic_chars_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("floating point value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = p_f32()
        .run(String::from("abc"));

    assert_eq!(actual, expected);
}

#[test]
fn p_f32_integer_exceeds_f32_max_fails_with_error() {
    let expected = Err(ParserFailure::new_err(
        String::from("floating point value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = p_f32()
        .run(String::from("340282500000000000000000000000000000000"));

    assert_eq!(actual, expected);
}

#[test]
fn p_f64_decimal_number_succeeds() {
    let expected = Ok(ParserSuccess::new(
        340282500000000000000000000000000000000.12, 
        Position::new(1, 43, 42)
    ));

    let actual = p_f64()
        .run(String::from("340282500000000000000000000000000000000.12"));

    assert_eq!(actual, expected);
}