mod common;
use common::*;
use rusty_parsec::*;

#[test]
fn tuple_2_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (123, String::from("hello")), 
        Position::new(1, 9, 8)
    ));

    let actual = tuple_2(
        p_u32(), 
        p_hello()
    ).run(String::from("123hello"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_2_run_simple_parsers_fails_with_error_at_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = tuple_2(
        p_u32(), 
        p_hello()
    ).run(String::from("hello123"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_2_run_complex_parsers_fails_with_fatal_error_at_first_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 4, 3)
    ));

    let actual = tuple_2(
        p_abc_123(), 
        p_hello()
    ).run(String::from("abcdefhello"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_2_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("hello"), 
        Some(String::from("world")), 
        Position::new(1, 4, 3)
    ));

    let actual = tuple_2(
        p_u32(), 
        p_hello()
    ).run(String::from("123world"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_2_run_complex_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 7, 6)
    ));

    let actual = tuple_2(
        p_u32(), 
        p_abc_123()
    ).run(String::from("100abcdef"));

    assert_eq!(actual, expected);
}

#[test]
fn tuple_3_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (String::from("hello"), 123 , true), 
        Position::new(1, 13, 12)
    ));

    let actual = tuple_3(
        p_hello(), 
        p_u32(), 
        p_true()
    ).run(String::from("hello123true"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_3_run_simple_parsers_fails_with_error_at_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        Some(String::from("world")), 
        Position::new(1, 1, 0)
    ));

    let actual = tuple_3(
        p_hello(), 
        p_u32(), 
        p_true()
    ).run(String::from("world123true"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_3_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 6, 5)
    ));

    let actual = tuple_3(
        p_hello(), 
        p_u32(), 
        p_true()
    ).run(String::from("helloabctrue"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_3_run_simple_parsers_fails_with_fatal_error_at_third_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("true"), 
        Some(String::from("fals")), 
        Position::new(1, 9, 8)
    ));

    let actual = tuple_3(
        p_hello(), 
        p_u32(), 
        p_true()
    ).run(String::from("hello123false"));

    assert_eq!(actual, expected);
}

#[test]
fn tuple_4_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (String::from("hello"), 123 , true, 1.5), 
        Position::new(1, 16, 15)
    ));

    let actual = tuple_4(
        p_hello(), 
        p_u32(), 
        p_true(),
        p_f32()
    ).run(String::from("hello123true1.5"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_4_run_simple_parsers_fails_with_error_at_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        Some(String::from("world")), 
        Position::new(1, 1, 0)
    ));

    let actual = tuple_4(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32()
    ).run(String::from("world123true1.5"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_4_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 6, 5)
    ));

    let actual = tuple_4(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32()
    ).run(String::from("helloabctrue1.5"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_4_run_simple_parsers_fails_with_fatal_error_at_third_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("true"), 
        Some(String::from("fals")), 
        Position::new(1, 9, 8)
    ));

    let actual = tuple_4(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32()
    ).run(String::from("hello123false1.5"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_4_run_simple_parsers_fails_with_fatal_error_at_fourth_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("floating point value"), 
        None, 
        Position::new(1, 13, 12)
    ));

    let actual = tuple_4(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32()
    ).run(String::from("hello123trueabc"));

    assert_eq!(actual, expected);
}

#[test]
fn tuple_5_run_simple_parserss_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (String::from("hello"), 123 , true, 1.5, 'a'), 
        Position::new(1, 17, 16)
    ));

    let actual = tuple_5(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32(), 
        p_char('a')
    ).run(String::from("hello123true1.5a"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_5_run_simple_parsers_fails_with_error_at_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        Some(String::from("world")), 
        Position::new(1, 1, 0)
    ));

    let actual = tuple_5(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32(), 
        p_char('a')
    ).run(String::from("world123true1.5a"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_5_run_simple_parsers_fails_with_fatal_error_at_second_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 6, 5)
    ));

    let actual = tuple_5(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32(), 
        p_char('a')
    ).run(String::from("helloabctrue1.5a"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_5_run_simple_parsers_fails_with_fatal_error_at_third_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("true"), 
        Some(String::from("fals")), 
        Position::new(1, 9, 8)
    ));

    let actual = tuple_5(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32(), 
        p_char('a')
    ).run(String::from("hello123false1.5a"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_5_run_simple_parsers_fails_with_fatal_error_at_fourth_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("floating point value"), 
        None, 
        Position::new(1, 13, 12)
    ));

    let actual = tuple_5(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32(), 
        p_char('a')
    ).run(String::from("hello123trueabca"));

    assert_eq!(actual, expected);
}

#[test] 
fn tuple_5_run_simple_parsers_fails_with_fatal_error_at_fifth_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("a"), 
        Some(String::from("c")), 
        Position::new(1, 16, 15)
    ));
    
    let actual = tuple_5(
        p_hello(), 
        p_u32(), 
        p_true(), 
        p_f32(), 
        p_char('a')
    ).run(String::from("hello123true1.5c"));

    assert_eq!(actual, expected);
}