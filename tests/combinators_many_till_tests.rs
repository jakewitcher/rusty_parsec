mod common;
use common::*;
use rusty_parsec::*;

#[test]
fn succeeds_parsing_with_many_till() {
    let expected = Ok(ParserSuccess::new(vec![true, true, true], Position::new(1, 16, 15)));

    let actual =
        many_till(p_true, p_u32)
            .run("truetruetrue123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_till_returns_empty_vec() {
    let expected = Ok(ParserSuccess::new(Vec::new(), Position::new(1, 1, 0)));

    let actual =
        many_till(p_true, p_u32)
            .run("false".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_till() {
    let expected = Err(ParserFailure::new_fatal_err("true".to_string(), None, Position::new(1, 13, 12)));

    let actual =
        many_till(p_true, p_u32)
            .run("truetruetrueabc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_till_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("integral value".to_string(), None, Position::new(1, 16, 15)));

    let actual =
        many_till(p_true, p_abc_123)
            .run("truetruetrueabcfalse".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_many_1_till() {
    let expected = Ok(ParserSuccess::new(vec![true, true, true], Position::new(1, 16, 15)));

    let actual =
        many_1_till(p_true, p_u32)
            .run("truetruetrue123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_many_1_till() {
    let expected = Err(ParserFailure::new_err("true".to_string(), Some("fals".to_string()), Position::new(1, 1, 0)));

    let actual =
        many_1_till(p_true, p_u32)
            .run("false".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_many_till() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 16, 15)));

    let actual =
        skip_many_till(p_true, p_u32)
            .run("truetruetrue123".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn succeeds_parsing_with_skip_many_till_nothing_parsed() {
    let expected = Ok(ParserSuccess::new((), Position::new(1, 1, 0)));

    let actual =
        skip_many_till(p_true, p_u32)
            .run("false".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_skip_many_till() {
    let expected = Err(ParserFailure::new_fatal_err("true".to_string(), None, Position::new(1, 13, 12)));

    let actual =
        many_till(p_true, p_u32)
            .run("truetruetrueabc".to_string());

    assert_eq!(expected, actual);
}

#[test]
fn fails_parsing_with_skip_many_till_fatal_err() {
    let expected = Err(ParserFailure::new_fatal_err("integral value".to_string(), None, Position::new(1, 16, 15)));

    let actual =
        many_till(p_true, p_abc_123)
            .run("truetruetrueabcfalse".to_string());

    assert_eq!(expected, actual);
}