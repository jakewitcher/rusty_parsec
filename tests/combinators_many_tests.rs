mod common;
use common::*;
use rusty_parsec::*;

#[test]
fn many_run_simple_parsers_succeeds_with_three_values() {
    let hello = String::from("hello");

    let expected = Ok(ParserSuccess::new(
        vec![hello.clone(), hello.clone(), hello.clone()],
        Position::new(1, 16, 15)
    ));

    let actual = many(p_hello)
        .run(String::from("hellohellohello"));

    assert_eq!(expected, actual);
}

#[test]
fn many_run_simple_parsers_succeeds_when_no_values_returned_by_parser() {
    let expected = Ok(ParserSuccess::new(
        Vec::new(), 
        Position::new(1, 1, 0)
    ));

    let actual = many(p_hello)
        .run(String::from("worldworldworld"));

    assert_eq!(expected, actual);
}

#[test]
fn many_run_complex_parsers_succeeds_with_three_values() {
    let abc = String::from("abc");

    let expected = Ok(ParserSuccess::new(
        vec![(abc.clone(), 123), (abc.clone(), 456), (abc.clone(), 789)], 
        Position::new(1, 19, 18)
    ));

    let actual = many(p_abc_123)
        .run(String::from("abc123abc456abc789"));

    assert_eq!(expected, actual);
}

#[test]
fn many_run_complex_parsers_fails_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 16, 15)
    ));

    let actual = many(p_abc_123)
        .run(String::from("abc123abc456abcdef"));

    assert_eq!(expected, actual);
}

#[test]
fn many_1_run_simple_parsers_succeeds_with_three_values() {
    let hello = String::from("hello");

    let expected = Ok(ParserSuccess::new(
        vec![hello.clone(), hello.clone(), hello.clone()],
        Position::new(1, 16, 15)
    ));

    let actual = many_1(p_hello)
        .run(String::from("hellohellohello"));

    assert_eq!(expected, actual);
}

#[test]
fn many_1_run_simple_parsers_fails_with_error_when_no_values_returned_by_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        Some(String::from("world")), 
        Position::new(1, 1, 0)
    ));

    let actual = many_1(p_hello)
        .run(String::from("worldworldworld"));

    assert_eq!(expected, actual);
}

#[test]
fn skip_many_run_simple_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 16, 15)
    ));

    let actual = skip_many(p_hello)
        .run(String::from("hellohellohello"));

    assert_eq!(expected, actual);
}

#[test]
fn skip_many_run_complex_parsers_fails_with_fatal_error() {
    let expected = Err(ParserFailure::new_fatal_err(
        String::from("integral value"), 
        None, 
        Position::new(1, 16, 15)
    ));

    let actual = skip_many(p_abc_123)
        .run(String::from("abc123abc456abcdef"));

    assert_eq!(expected, actual);
}

#[test]
fn skip_many_1_run_simple_parsers_succeeds() {
    let expected = Ok(ParserSuccess::new(
        (), 
        Position::new(1, 16, 15)
    ));

    let actual = skip_many_1(p_hello)
        .run(String::from("hellohellohello"));

    assert_eq!(expected, actual);
}

#[test]
fn skip_many_1_run_simple_parsers_fails_with_error_when_no_values_returned_by_parser() {
    let expected = Err(ParserFailure::new_err(
        String::from("hello"), 
        None, 
        Position::new(1, 1, 0)
    ));

    let actual = skip_many_1(p_hello)
        .run(String::from("abc"));

    assert_eq!(expected, actual);
}