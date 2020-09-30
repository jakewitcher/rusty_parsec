mod common;
use common::*;
use rusty_parsec::*;

#[test]
fn many_till_run_simple_parsers_succeeds_with_three_values() {
    let expected = Ok(ParserSuccess::new(
        vec![true, true, true], 
        Position::new(1, 16, 15)
    ));

    let actual = many_till(p_true, p_u32)
        .run(String::from("truetruetrue123"));

    assert_eq!(actual, expected);
}

#[test]
fn many_till_run_simple_parsers_succeeds_when_no_values_returned_by_first_parser() {
    let expected = Ok(ParserSuccess::new(
        Vec::new(), 
        Position::new(1, 4, 3)
    ));

    let actual = many_till(p_true, p_u32)
        .run(String::from("123"));

    assert_eq!(actual, expected);
}

#[test]
fn many_till_run_simple_parsers_fails_with_error_at_end_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = many_till(p_true, p_u32)
        .run(String::from("abc"));

    assert_eq!(actual, expected);
}

#[test]
fn many_till_run_simple_parsers_fails_with_fatal_error_at_end_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("true"), 
        None, 
        Position::new(1, 13, 12)
    ));

    let actual = many_till(p_true, p_u32)
        .run(String::from("truetruetrueabc"));

    assert_eq!(actual, expected);
}

#[test]
fn many_till_run_complex_parsers_fails_with_fatal_error_at_many_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 10, 9)
    ));

    let actual = many_till(p_abc_123, p_true)
        .run(String::from("abc123abcdeftrue"));

    assert_eq!(actual, expected);
}

#[test]
fn many_1_till_run_simple_parsers_succeeds_with_three_values() {
    let expected = Ok(ParserSuccess::new(
        vec![true, true, true], 
        Position::new(1, 16, 15)
    ));

    let actual = many_1_till(p_true, p_u32)
        .run(String::from("truetruetrue123"));

    assert_eq!(actual, expected);
}

#[test]
fn many_1_till_run_simple_parsers_fails_with_error_when_no_values_returned_by_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("true"), 
        Some(String::from("1234")), 
        Position::new(1, 1, 0)
    ));

    let actual = many_1_till(p_true, p_u32)
        .run(String::from("1234"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_many_till_run_simple_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 16, 15)
    ));

    let actual = skip_many_till(p_true, p_u32)
        .run(String::from("truetruetrue123"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_many_till_run_simple_parsers_succeeds_with_no_values_parsed_by_first_parser() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 4, 3)
    ));

    let actual = skip_many_till(p_true, p_u32)
        .run(String::from("123"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_many_till_run_simple_parsers_fails_with_error_at_end_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = skip_many_till(p_true, p_u32)
        .run(String::from("abc"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_many_till_run_simple_parsers_fails_with_fatal_error_at_end_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("true"), 
        None, 
        Position::new(1, 13, 12)
    ));

    let actual = many_till(p_true, p_u32)
        .run(String::from("truetruetrueabc"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_many_till_run_complex_parsers_fails_with_fatal_error_at_many_parser() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 10, 9)
    ));

    let actual = many_till(p_abc_123, p_true)
        .run(String::from("abc123abcdeftrue"));

    assert_eq!(actual, expected);
}


#[test]
fn skip_many_1_till_run_simple_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 16, 15)
    ));

    let actual = skip_many_1_till(p_true, p_u32)
        .run(String::from("truetruetrue123"));

    assert_eq!(actual, expected);
}

#[test]
fn skip_many_1_till_run_simple_parsers_fails_with_error_when_no_values_parsed_by_first_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("true"), 
        Some(String::from("1234")), 
        Position::new(1, 1, 0)
    ));

    let actual = skip_many_1_till(p_true, p_u32)
        .run(String::from("1234"));

    assert_eq!(actual, expected);
}