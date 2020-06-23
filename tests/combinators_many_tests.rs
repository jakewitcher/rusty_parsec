mod common;
use common::*;
use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_many() {
    let expected = Ok(ParserSuccess::new(vec![ "hello".to_string(), "hello".to_string(), "hello".to_string()], Position::new(1, 16, 15)));

    let actual = many(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_returns_empty_vec() {
    let expected = Ok(ParserSuccess::new(Vec::new(), Position::new(1, 1, 0)));

    let actual = many(p_hello).run("worldworldworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_with_compound_parser() {
    let expected = Ok(ParserSuccess::new(vec![("abc".to_string(), 123), ("abc".to_string(), 456), ("abc".to_string(), 789)], Position::new(1, 19, 18)));

    let actual = many(p_abc_123).run("abc123abc456abc789".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_1() {
    let expected = Ok(ParserSuccess::new(vec![ "hello".to_string(), "hello".to_string(), "hello".to_string()], Position::new(1, 16, 15)));

    let actual = many_1(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_1() {
    let expected = Err(ParserFailure::new_err("value satisfying parser at least once".to_string(), None, Position::new(1, 1, 0)));

    let actual = many_1(p_hello).run("worldworldworld".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_many() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));

    let actual = skip_many(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_many_1() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));

    let actual = skip_many_1(p_hello).run("hellohellohello".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_skip_many_1() {
    let expected = Err(ParserFailure::new_err("value satisfying parser at least once".to_string(), None, Position::new(1, 1, 0)));

    let actual = skip_many_1(p_hello).run("abc".to_string());

    assert_eq!(expected, actual);
}